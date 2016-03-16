extern crate slack as slack_api;
extern crate pierre;

mod slack;

//use std::collections::String;
use pierre::config::Config;
use slack::EventHandler;

fn main() {
    
    let home_dir = std::env::home_dir().expect("Could not find home directory to place config");
    let filepath = format!("{}/.pierre_config", home_dir.to_string_lossy());
    let mut config = Config::load(&filepath).expect(&format!("Could not load config at {}", filepath));

    let mut handler = EventHandler::new(config.slack.user);
    let mut cli = slack_api::RtmClient::new(config.slack.token.as_str());
    let r = cli.login_and_run::<EventHandler>(&mut handler);
    match r {
        Ok(_) => {}
        Err(err) => panic!("Error: {}", err),
    }
}
