mod manager;

use slack_api::{self, Event, Message};
use pierre::{self, config};

pub struct EventHandler {
    user: String,
}

impl EventHandler {
    pub fn new(user: String) -> Self {
        EventHandler {
            user: user,
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
                        let re = Regex::new(format!(r"<@{}> watch (\w)\\(\w) --notify (\w)", self.user)).unwrap();
                        let cap = re.captures(text).unwrap();
                        let proj = cap.at(0);
                        let repo = cap.at(1);
                        let scope = cap.at(2);
                        manager::configure_repos(proj, repo, scope);
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