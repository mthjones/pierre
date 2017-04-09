extern crate pierre;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use pierre::config;

use std::error;

mod api;
pub use self::api::*;

pub struct StashPullRequestDataRetriever {
    base_url: String,
    username: String,
    password: Option<String>,
    client: reqwest::Client,
    project: config::Project
}

impl StashPullRequestDataRetriever {
    pub fn new(project: config::Project, auth: (String, Option<String>), base_url: String) -> Self {
        let client = reqwest::Client::new().unwrap();
        StashPullRequestDataRetriever {
            base_url: base_url,
            username: auth.0,
            password: auth.1,
            client: client,
            project: project
        }
    }

    pub fn get_pull_requests(&self) -> Result<Vec<PullRequest>, Box<error::Error>> {
        let mut headers = reqwest::header::Headers::new();
        headers.set(reqwest::header::Authorization(
            reqwest::header::Basic {
                username: self.username.clone(),
                password: self.password.clone()
            }
        ));
        headers.set(reqwest::header::Connection::close());
        headers.set(reqwest::header::UserAgent("pierre/1.0".to_owned()));

        let url = format!("{}/rest/api/1.0/projects/{}/repos/{}/pull-requests",
            self.base_url,
            self.project.id.to_uppercase(),
            self.project.repo.to_lowercase());
            
        println!("{}", url);

        let mut response = self.client.get(&url).headers(headers).send()?;

        let prs: PagedData<PullRequest> = try!(response.json());
        Ok(prs.values)
    }
}

fn main() {
    let config = Config::load_default().expect("Could not load config at default location");

    let http_client = reqwest::Client::new();
    
    let poll_interval = Duration::from_secs(15 * 60);

    let mut threads = vec![];
    for project in config.projects.iter() {
        let tx = tx.clone();
        let retriever = StashPullRequestDataRetriever::new(project.clone(), (config.stash.username.clone(), config.stash.password.clone()), config.stash.base_url.clone());
        let t = thread::spawn(move || {
            loop {
                match retriever.get_pull_requests() {
                    Ok(prs) => {
                        for pr in prs {
                            http_client.post("http://localhost:9000/")
                        }
                    },
                    Err(e) => println!("{}", e.description())
                }
                
                thread::sleep(poll_interval);
            }
        });
        threads.push(t);
    }

    for t in threads {
        t.join().unwrap();
    }
}