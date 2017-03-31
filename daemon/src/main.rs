extern crate pierre;
extern crate rand;
extern crate reqwest;
extern crate postgres;
extern crate slack_api;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate rusoto;
extern crate chrono;

use std::time::Duration;
use postgres::{Connection, SslMode};
use rusoto::{ProvideAwsCredentials, AwsCredentials, CredentialsError, default_tls_client, Region};
use rusoto::dynamodb::DynamoDbClient;
use chrono::{DateTime, UTC};

mod stash;
mod watcher;
mod slack;
mod models;

use watcher::Watcher;
use pierre::config::Config;
use slack::SlackPullRequestEventHandler;
use stash::StashPullRequestDataRetriever;
use models::PullRequestDataModel;

struct PierreConfigAwsCredentialsProvider<'a> {
    config: &'a Config
}

impl<'a> ProvideAwsCredentials for PierreConfigAwsCredentialsProvider<'a> {
    fn credentials(&self) -> Result<AwsCredentials, CredentialsError> {
        let in_ten_minutes = UTC::now() + Duration::from_secs(10 * 60);
        Ok(AwsCredentials::new(&self.config.aws_access_key, &self.config.aws_secret, None, in_ten_minutes))
    }
}

fn main() {
    let home_dir = std::env::home_dir().expect("Could not find home directory to place config");
    let filepath = format!("{}/.pierre_config", home_dir.to_string_lossy());
    let mut config = Config::load(&filepath).expect(&format!("Could not load config at {}", filepath));

    let aws_credentials_provider = PierreConfigAwsCredentialsProvider { config: &config };
    let db = rusoto::dynamodb::DynamoDbClient(default_tls_client.unwrap(), &aws_credentials_provider, Region::UsEast1);

    if config.stash.password.is_none() {
        if let Ok(password) = std::env::var("PIERRE_USER_PASSWORD") {
            config.stash.password = Some(password);
        }
    }

    let watchers = config.projects.iter().map(|p| {
        let config = config.clone();
        let retriever = StashPullRequestDataRetriever::new(p.clone(), (config.stash.username, config.stash.password), config.stash.base_url);
        let handler = SlackPullRequestEventHandler::new(config.db, config.users, config.slack.channel, config.slack.token);
        let watcher = Watcher::new(Box::new(retriever), Box::new(handler), Duration::from_secs(15 * 60));
        watcher.watch()
    }).collect::<Vec<_>>();

    for watcher in watchers {
        watcher.join().unwrap();
    }
}
