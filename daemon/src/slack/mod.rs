use std::collections::HashMap;
use std::error;

use rand;
use hyper;
use slack_api;
use postgres::{Connection,SslMode};

mod attachment_builder;
use self::attachment_builder::*;

use ::models::PullRequestDataModel;
use ::watcher::EventHandler;

pub struct SlackPullRequestEventHandler {
    conn_str: String,
    user_map: HashMap<String, String>,
    client: hyper::Client,
    channel: String,
    token: String
}

impl SlackPullRequestEventHandler {
    pub fn new(conn_str: String, user_map: HashMap<String, String>, channel: String, token: String) -> Self {
        let client = hyper::Client::new();
        SlackPullRequestEventHandler {
            conn_str: conn_str,
            user_map: user_map,
            client: client,
            channel: channel,
            token: token
        }
    }

    fn build_attachment_from_pr(&self, pr: &::stash::PullRequest) -> Result<slack_api::Attachment, String> {
        let mut rng = rand::thread_rng();
        let pr = pr.clone();
        let reviewers = pr.reviewers.iter()
            .filter_map(|r| self.user_map.get(&r.user.name).cloned())
            .collect::<Vec<_>>();
        if reviewers.is_empty() {
            return Err("No reviewers.".to_owned());
        }
        let reviewers_field = slack_api::api::AttachmentField {
            title: "Reviewers".to_owned(),
            value: reviewers.join(", "),
            short: false
        };
        let text = pr.description.unwrap_or("".to_owned());
        let fallback = format!("*New Pull Request!\n*{}*\nAssigned to: {}", pr.title, reviewers.join(", "));
        let author = AttachmentAuthorBuilder::with_name(pr.author.user.name)
            .set_link(pr.author.user.links.get("self").unwrap()[0].href.clone())
            .build();

        let mut builder = AttachmentBuilder::with_text_and_fallback(text, fallback)
            .set_color("#00CC99")
            .set_title(pr.title, pr.links.get("self").map(|links| links[0].href.clone()))
            .set_author(author)
            .add_field(reviewers_field);

        if !reviewers.is_empty() {
            let assigned_demoer_field = slack_api::api::AttachmentField {
                title: "Assigned Demo Reviewer".to_owned(),
                value: rand::sample(&mut rng, reviewers.clone().into_iter(), 1)[0].clone(),
                short: true
            };
            builder = builder.add_field(assigned_demoer_field);
        }

        Ok(builder.build())
    }
}

impl EventHandler<::stash::PullRequest> for SlackPullRequestEventHandler {
    fn on_data(&mut self, prs: Vec<::stash::PullRequest>) -> Result<(), Box<error::Error>> {
        let conn = try!(Connection::connect(&self.conn_str[..], &SslMode::None));
        let processed_prs = try!(PullRequestDataModel::all(&conn));

        for pr in prs.into_iter().filter(|pr| !processed_prs.contains(&pr.clone().into())) {
            match self.build_attachment_from_pr(&pr) {
                Ok(attachment) => {
                    try!(PullRequestDataModel::create(&conn, pr.id, &pr.to_ref.repository.project.key, &pr.to_ref.repository.slug));
                    if let Err(e) = slack_api::api::chat::post_message(&self.client, &self.token, &self.channel, "*New Pull Request!*", Some("pierre"), Some(true), None, None, Some(vec![attachment]), None, None, None, None) {
                        PullRequestDataModel::delete(&conn, pr.id, &pr.to_ref.repository.project.key, &pr.to_ref.repository.slug).unwrap();
                        return Err(Box::new(e));
                    }
                },
                Err(_) => {
                    PullRequestDataModel::create(&conn, pr.id, &pr.to_ref.repository.project.key, &pr.to_ref.repository.slug).unwrap();
                }
            }
        }
        Ok(())
    }
}

pub struct RepoConfigEventHandler;

#[allow(unused_variables)]
impl slack_api::EventHandler for RepoConfigEventHandler {
    fn on_event(&mut self, cli: &mut slack_api::RtmClient, event: Result<&slack_api::Event, slack_api::Error>, raw_json: &str) {
        println!("on_event(event: {:?}, raw_json: {:?})", event, raw_json);
    }

    fn on_ping(&mut self, cli: &mut slack_api::RtmClient) {
        println!("on_ping");
    }

    fn on_close(&mut self, cli: &mut slack_api::RtmClient) {
        println!("on_close");
    }

    fn on_connect(&mut self, cli: &mut slack_api::RtmClient) {
        println!("on_connect");
        // Do a few things using the api:
        // send a message over the real time api websocket
        let _ = cli.send_message("#general", "Hello world! (rtm)");
        // post a message as a user to the web api
        let _ = cli.post_message("#general", "hello world! (postMessage)", None);
        // set a channel topic via the web api
        // let _ = cli.set_topic("#general", "bots rule!");
    }
}
