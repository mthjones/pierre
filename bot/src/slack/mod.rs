use slack_api::{self, Event, Message};
use pierre::{self, config};
use regex::Regex;
use repo_prefs;

pub struct EventHandler {
    user: String,
    db: String,
}

impl EventHandler {
    pub fn new(user: String, db: String) -> Self {
        EventHandler {
            user: user,
            db: db,
        }
    }
}

#[allow(unused_variables)]
impl slack_api::EventHandler for EventHandler {
    fn on_event(&mut self, cli: &mut slack_api::RtmClient, event: Result<&slack_api::Event, slack_api::Error>, raw_json: &str) {
        println!("on_event(event: {:?}, raw_json: {:?})", event, raw_json);
        
        match event {
            Ok(&Event::Message(Message::Standard { text: Some(ref t), .. } )) => {
                match t.find(&format!("@{}", self.user)) {
                    Some(_) => {
                        println!("You talkin' to me?");
                        let re = Regex::new(&format!(r"<@{}> watch (\w+)/(\w+) --notify (\w+)$", self.user)).unwrap();
                        if re.is_match(t) {
                            let cap = re.captures(t).unwrap();
                            let proj = cap.at(1);
                            let repo = cap.at(2);
                            let scope = cap.at(3);
                            let manager = repo_prefs::RepoPrefsManager::new(&self.db);
                            manager.update(String::from(proj.unwrap()), String::from(repo.unwrap()), String::from(scope.unwrap()));
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