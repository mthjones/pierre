extern crate pierre;
extern crate rand;
extern crate reqwest;
extern crate slack_api;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate rusoto;

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use rusoto::{default_tls_client, Region, DefaultCredentialsProviderSync};
use rusoto::dynamodb::DynamoDbClient;

mod stash;
mod slack;
mod store;
mod data;

use data::PullRequestData;

use pierre::config::Config;
use slack::SlackPullRequestEventHandler;
use stash::StashPullRequestDataRetriever;
use store::DynamoDataStore;

fn main() {
    let home_dir = std::env::home_dir().expect("Could not find home directory to place config");
    let filepath = format!("{}/.pierre_config", home_dir.to_string_lossy());
    let mut config = Config::load(&filepath).expect(&format!("Could not load config at {}", filepath));

    let aws_credentials_provider = DefaultCredentialsProviderSync::new().unwrap();
    let db = DynamoDbClient::new(default_tls_client().unwrap(), aws_credentials_provider, Region::UsEast1);

    if config.stash.password.is_none() {
        if let Ok(password) = std::env::var("PIERRE_USER_PASSWORD") {
            config.stash.password = Some(password);
        }
    }

    let poll_interval = Duration::from_secs(15 * 60);

    let mut threads = vec![];
    let (tx, rx) = mpsc::channel();

    for project in config.projects.iter() {
        let tx = tx.clone();
        let retriever = StashPullRequestDataRetriever::new(project.clone(), (config.stash.username.clone(), config.stash.password.clone()), config.stash.base_url.clone());
        let t = thread::spawn(move || {
            loop {
                match retriever.get_pull_requests() {
                    Ok(prs) => {
                        for pr in prs {
                            let _ = tx.send(pr);
                        }
                    },
                    Err(e) => println!("{}", e.description())
                }
                
                thread::sleep(poll_interval);
            }
        });
        threads.push(t);
    }

    let post_thread = thread::spawn(move || {
        // Gross type shenanigans because of using PhantomData
        let pr_store: DynamoDataStore<PullRequestData, _, _> = DynamoDataStore::new(&db, "PullRequests");

        let sender = SlackPullRequestEventHandler::new(&pr_store, config.users.clone(), config.slack.channel.clone(), config.slack.token.clone());
        for pr in rx.iter() {
            let _ = sender.on_data(pr);
        }
    });

    for t in threads {
        t.join().unwrap();
    }

    post_thread.join().unwrap();
}
