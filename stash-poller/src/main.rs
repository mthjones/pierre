extern crate pierre;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate rusoto;

use pierre::config::{self, Config};
use pierre::store::Store;
use pierre::store::dynamodb::DynamoDataStore;
use pierre::data::PullRequestData;

use rusoto::{default_tls_client, Region, DefaultCredentialsProviderSync};
use rusoto::dynamodb::DynamoDbClient;

use serde::Serialize;

use std::error;
use std::marker::PhantomData;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

mod api;

impl From<api::PullRequest> for PullRequestData {
    fn from(pr: api::PullRequest) -> PullRequestData {
        PullRequestData {
            id: pr.id,
            project: pr.to_ref.repository.project.name.clone(),
            repo: pr.to_ref.repository.name.clone(),
        }
    }
}

trait Notifier {
    type Item;
    type Error;

    fn notify(&self, item: Self::Item) -> Result<(), Self::Error>;
}

#[derive(Clone)]
struct Sink<'a, T: 'a>(PhantomData<&'a T>);

impl<'a, T: 'a> Notifier for Sink<'a, T> {
    type Item = &'a T;
    type Error = ();

    fn notify(&self, _: Self::Item) -> Result<(), Self::Error> {
        Ok(())
    }
}

struct HttpNotifier<'a, T: 'a> {
    client: Arc<reqwest::Client>,
    endpoint: String,
    _ty: PhantomData<&'a T>
}

impl<'a, T: 'a> HttpNotifier<'a, T> {
    fn new<S: Into<String>>(client: Arc<reqwest::Client>, endpoint: S) -> Self {
        HttpNotifier {
            client: client,
            endpoint: endpoint.into(),
            _ty: PhantomData
        }
    }
}

impl<'a, T: 'a + Serialize> Notifier for HttpNotifier<'a, T> {
    type Item = &'a T;
    type Error = reqwest::Error;

    fn notify(&self, item: Self::Item) -> Result<(), Self::Error> {
        self.client.post(&self.endpoint).json(item).send().map(|_| ())
    }
}

#[derive(Clone)]
pub struct StashPullRequestDataRetriever {
    base_url: String,
    username: String,
    password: Option<String>,
    client: Arc<reqwest::Client>
}

impl StashPullRequestDataRetriever {
    pub fn new(client: Arc<reqwest::Client>, auth: (String, Option<String>), base_url: String) -> Self {
        StashPullRequestDataRetriever {
            base_url: base_url,
            username: auth.0,
            password: auth.1,
            client: client
        }
    }

    pub fn get_pull_requests(&self, project: &config::Project) -> Result<Vec<api::PullRequest>, Box<error::Error>> {
        let mut headers = reqwest::header::Headers::new();
        headers.set(reqwest::header::Authorization(
            reqwest::header::Basic {
                username: self.username.clone(),
                password: self.password.clone()
            }
        ));
        headers.set(reqwest::header::Connection::close());
        headers.set(reqwest::header::UserAgent("pierre/1.0".to_owned()));

        let url = format!("{}/rest/api/1.0/projects/{}/repos/{}/pull-requests",
            self.base_url,
            project.id.to_uppercase(),
            project.repo.to_lowercase());
            
        println!("{}", url);

        let mut response = self.client.get(&url).headers(headers).send()?;

        let prs: api::PagedData<api::PullRequest> = response.json()?;
        Ok(prs.values)
    }
}

fn main() {
    let mut config = Config::load_default().expect("Could not load config at default location");

    let aws_credentials_provider = DefaultCredentialsProviderSync::new().unwrap();
    let db = DynamoDbClient::new(default_tls_client().unwrap(), aws_credentials_provider, Region::UsEast1);

    if config.stash.password.is_none() {
        if let Ok(password) = std::env::var("PIERRE_USER_PASSWORD") {
            config.stash.password = Some(password);
        }
    }

    let pr_store: Arc<DynamoDataStore<PullRequestData, _, _>> = Arc::new(DynamoDataStore::new(Arc::new(db), "PullRequests"));

    let http_client = Arc::new(reqwest::Client::new().expect("Unable to create HTTP client"));
    let notifier = Arc::new(HttpNotifier::<api::PullRequest>::new(http_client.clone(), "http://localhost:9000"));
    
    let poll_interval = Duration::from_secs(15 * 60);

    let retriever = StashPullRequestDataRetriever::new(http_client.clone(), (config.stash.username.clone(), config.stash.password.clone()), config.stash.base_url.clone());

    let mut threads = vec![];
    for project in config.projects {
        let notifier = notifier.clone();
        let pr_store = pr_store.clone();
        let retriever = retriever.clone();

        let t = thread::spawn(move || {
            loop {
                if let Ok(prs) = retriever.get_pull_requests(&project) {
                    for pr in prs {
                        let pr_data: PullRequestData = pr.clone().into();
                        if pr_store.create(pr_data).is_ok() {
                            let _ = notifier.notify(&pr);
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