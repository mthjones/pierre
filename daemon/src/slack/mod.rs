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
       let processed_prs_clone = processed_prs.clone();
       let prs_clone = prs.clone();
       let mut prs_to_save = Vec::<PullRequestDataModel>::new();
       
       //get all prs in the db as PullRequestDataModels that are also in the stash repository
       for pr_to_save in prs_clone.into_iter().filter(|pr_to_save| processed_prs.contains(&pr_to_save.clone().into())){
           prs_to_save.push(PullRequestDataModel::from(pr_to_save));
       }
       
       //remove any PR in database but not in stash repo
       for pr_to_remove in processed_prs_clone.into_iter().filter(|pr_to_remove| !prs_to_save.contains(&pr_to_remove.clone().into())){
            if let Err(e) = PullRequestDataModel::delete(&conn, pr_to_remove.id, &pr_to_remove.project, &pr_to_remove.repo){
                 return Err(Box::new(e));
            }
       }
       //For any received PR not in the DB and converts it into a PullRequestDataModel
       for pr in prs.into_iter().filter(|pr| !processed_prs.contains(&pr.clone().into())) {
           match self.build_attachment_from_pr(&pr) {
               Ok(attachment) => {
                   try!(PullRequestDataModel::create(&conn, pr.id, &pr.to_ref.repository.project.key, &pr.to_ref.repository.slug));
                   if let Err(e) = slack_api::api::chat::post_message(&self.client, &self.token, &self.channel, "*New Pull Request!*", Some("pierre"), Some(true), None, None, Some(vec![attachment]), None, None, None, None) {
                       PullRequestDataModel::delete(&conn, pr.id, &pr.to_ref.repository.project.key, &pr.to_ref.repository.slug).unwrap();
                       return Err(Box::new(e));
                   }
               },
               //get matt to expain this
               Err(_) => {
                   PullRequestDataModel::create(&conn, pr.id, &pr.to_ref.repository.project.key, &pr.to_ref.repository.slug).unwrap();
               }
           }
       }
       Ok(())
    }
}
