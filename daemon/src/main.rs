extern crate pierre;
extern crate rand;
extern crate reqwest;
extern crate postgres;
extern crate slack_api;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::sync::mpsc;
use std::time::Duration;
use std::thread;
use postgres::{Connection, SslMode};

mod stash;
mod slack;
mod models;

use pierre::config::Config;
use slack::SlackPullRequestEventHandler;
use stash::StashPullRequestDataRetriever;
use models::PullRequestDataModel;

fn main() {
    let home_dir = std::env::home_dir().expect("Could not find home directory to place config");
    let filepath = format!("{}/.pierre_config", home_dir.to_string_lossy());
    let mut config = Config::load(&filepath).expect(&format!("Could not load config at {}", filepath));

    {
        let conn = Connection::connect(&config.db[..], &SslMode::None).unwrap();
        PullRequestDataModel::initialize(&conn).unwrap();
    }

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
        let sender = SlackPullRequestEventHandler::new(config.db.clone(), config.users.clone(), config.slack.channel.clone(), config.slack.token.clone());
        for pr in rx.iter() {
            let _ = sender.on_data(pr);
        }
    });

    for t in threads {
        t.join().unwrap();
    }

    post_thread.join().unwrap();
}
