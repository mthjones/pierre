use slack_api::{self, Event, Message};
use pierre::{self, config};
use regex::Regex;
use repo_prefs;

pub struct EventHandler {
    user: String,
    db: String,
    channel: String,
}

impl EventHandler {
    pub fn new(user: String, db: String, channel: String) -> Self {
        EventHandler {
            user: user,
            db: db,
            channel: channel,
        }
    }
}

#[allow(unused_variables)]
impl slack_api::EventHandler for EventHandler {
    fn on_event(&mut self, cli: &mut slack_api::RtmClient, event: Result<&slack_api::Event, slack_api::Error>, raw_json: &str) {
        println!("on_event(event: {:?}, raw_json: {:?})", event, raw_json);
        
        match event {
            Ok(&Event::Message(Message::Standard { text: Some(ref t), channel: Some(ref c), .. } )) => {
                match t.find(&format!("@{}", self.user)) {
                    Some(_) => {
                        println!("You talkin' to me?");
                        let re1 = Regex::new(&format!(r"<@{}> watch (\w+)/(\w+)$", self.user)).unwrap();
                        let re2 = Regex::new(&format!(r"<@{}> h[ae]lp$", self.user)).unwrap();
                        let re3 = Regex::new(&format!(r"<@{}> h[eau]llo|hi$", self.user)).unwrap();
                        let re4 = Regex::new(&format!(r"<@{}> where is Matt$", self.user)).unwrap();
                        if re1.is_match(t) {
                            let cap = re1.captures(t).unwrap();
                            let proj = cap.at(1);
                            let repo = cap.at(2);
                            let scope = cap.at(3);
                            let manager = repo_prefs::RepoPrefsManager::new(&self.db);
                            manager.update(c, &String::from(proj.unwrap()), &String::from(repo.unwrap()));
                            let _ = cli.post_message(self.channel.as_str(), "Updated repo preferences!", None);
                        } else if re2.is_match(t) {
                            let _ = cli.post_message(self.channel.as_str(), "No.", None);
                        } else if re3.is_match(t) {
                            let _ = cli.post_message(self.channel.as_str(), "You talkin' to me?", None);
                        } else if re4.is_match(t) {
                            let _ = cli.post_message(self.channel.as_str(), "¯\\_(ツ)_/¯", None);
                        }
                    }
                    _ => println!("Not my concern"),
                }
            },
            _ => println!("Not a message"),
        }
    }

    fn on_ping(&mut self, cli: &mut slack_api::RtmClient) {
        println!("on_ping");
    }

    fn on_close(&mut self, cli: &mut slack_api::RtmClient) {
        println!("on_close");
    }

    fn on_connect(&mut self, cli: &mut slack_api::RtmClient) {
        println!("on_connect"); 
    }
}