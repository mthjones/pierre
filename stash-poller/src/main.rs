extern crate pierre;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate rusoto;

use pierre::config::Config;
use pierre::store::{Store, Keyed};
use pierre::store::dynamodb::DynamoDataStore;
use pierre::data::PullRequestData;

use rusoto::{default_tls_client, Region, DefaultCredentialsProviderSync};
use rusoto::dynamodb::DynamoDbClient;

use std::sync::Arc;
use std::thread;
use std::time::Duration;

mod api;
mod notifier;
mod retriever;

use notifier::{Notifier, HttpNotifier};
use retriever::{PullRequestRetriever, StashPullRequestRetriever};

impl From<api::PullRequest> for PullRequestData {
    fn from(pr: api::PullRequest) -> PullRequestData {
        PullRequestData {
            id: pr.id,
            project: pr.to_ref.repository.project.name.clone(),
            repo: pr.to_ref.repository.name.clone(),
        }
    }
}

fn main() {
    let config = Config::load_default().expect("Could not load config at default location");

    let poll_interval = Duration::from_secs(15 * 60);

    let aws_credentials_provider = DefaultCredentialsProviderSync::new().unwrap();
    let db = DynamoDbClient::new(default_tls_client().unwrap(), aws_credentials_provider, Region::UsEast1);

    let pr_store: Arc<DynamoDataStore<PullRequestData, _, _>> = Arc::new(DynamoDataStore::new(Arc::new(db), "PullRequests"));

    let http_client = Arc::new(reqwest::Client::new().expect("Unable to create HTTP client"));
    let notifier = Arc::new(HttpNotifier::<api::PullRequest>::new(http_client.clone(), "http://localhost:9000"));
    
    let auth = (config.stash.username.clone(), config.stash.password.or_else(|| std::env::var("PIERRE_USER_PASSWORD").ok()).clone());
    let retriever = StashPullRequestRetriever::new(http_client.clone(), auth, config.stash.base_url.clone());

    let projects = config.projects.iter().map(|p| (p.id.clone(), p.repo.clone()));

    let mut threads = vec![];
    for (project, repo) in projects {
        let notifier = notifier.clone();
        let pr_store = pr_store.clone();
        let retriever = retriever.clone();

        let t = thread::spawn(move || {
            loop {
                if let Ok(prs) = retriever.retrieve(&project, &repo) {
                    if let Ok(processed_prs) = pr_store.list() {
                        for pr in prs {
                            let pr_data: PullRequestData = pr.clone().into();
                            if processed_prs.iter().find(|ppr| ppr.key() == pr_data.key()).is_some() {
                                continue;
                            }
                            if pr_store.create(pr_data).is_ok() {
                                let _ = notifier.notify(&pr);
                            }
                        }
                    }
                }
                
                thread::sleep(poll_interval);
            }
        });

        threads.push(t);
    }

    for t in threads {
        t.join().unwrap();
    }
}