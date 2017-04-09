use std::collections::HashMap;

use rand;
use reqwest;
use slack_api;
use serde_json;

mod attachment_builder;
use self::attachment_builder::*;

pub struct SlackPullRequestEventHandler {
    user_map: HashMap<String, String>,
    client: reqwest::Client,
    channel: String,
    token: String
}

impl SlackPullRequestEventHandler {
    pub fn new(user_map: HashMap<String, String>, channel: String, token: String) -> Self {
        let client = reqwest::Client::new().unwrap();
        SlackPullRequestEventHandler {
            user_map: user_map,
            client: client,
            channel: channel,
            token: token
        }
    }

    fn build_attachment_from_pr(&self, pr: &::stash::PullRequest) -> Result<Attachment, String> {
        let mut rng = rand::thread_rng();
        let pr = pr.clone();
        let reviewers = pr.reviewers.iter()
            .filter_map(|r| self.user_map.get(&r.user.name).cloned())
            .collect::<Vec<_>>();
        if reviewers.is_empty() {
            return Err("No reviewers.".to_owned());
        }
        let reviewers_field = AttachmentField {
            title: Some("Reviewers".to_owned()),
            value: Some(reviewers.join(", ")),
            short: Some(false)
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
            let assigned_demoer_field = AttachmentField {
                title: Some("Assigned Demo Reviewer".to_owned()),
                value: Some(rand::sample(&mut rng, reviewers.clone().into_iter(), 1)[0].clone()),
                short: Some(true)
            };
            builder = builder.add_field(assigned_demoer_field);
        }

        Ok(builder.build())
    }

    pub fn on_data(&self, pr: ::stash::PullRequest) -> Result<(), ()> {
        if let Ok(attachment) = self.build_attachment_from_pr(&pr) {
            let serialized_attachments = serde_json::to_string(&[attachment]).unwrap();
            let message = slack_api::chat::PostMessageRequest {
                channel: &self.channel,
                text: "*New Pull Request!*",
                username: Some("pierre"),
                as_user: Some(true),
                attachments: Some(&serialized_attachments),
                ..slack_api::chat::PostMessageRequest::default()
            };

            if let Err(_) = slack_api::chat::post_message(&self.client, &self.token, &message) {
                return Err(());
            }
        }
        Ok(())
    }
}
