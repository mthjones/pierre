extern crate slack as slack_api;
extern crate pierre;

mod event_handler;

use pierre::config::Config;
use event_handler::EventHandler;

fn main() {
    
    let home_dir = std::env::home_dir().expect("Could not find home directory to place config");
    let filepath = format!("{}/.pierre_config", home_dir.to_string_lossy());
    let mut config = Config::load(&filepath).expect(&format!("Could not load config at {}", filepath));

    let mut handler = EventHandler::new(config.slack.user);
    let mut cli = slack_api::RtmClient::new(config.slack.token.as_str());
    let r = cli.login_and_run::<EventHandler>(&mut handler);
    r.expect("Error!");
}
