extern crate slack as slack_api;

mod slack;

use slack::EventHandler;

fn main() {
    
    let token = "xoxb-13051168534-3gCvViEkEMhoEmNlI7057rFC";
    
    let mut handler = EventHandler;
    let mut cli = slack_api::RtmClient::new(&token);
    let r = cli.login_and_run::<EventHandler>(&mut handler);
    match r {
        Ok(_) => {}
        Err(err) => panic!("Error: {}", err),
    }
}
