use reqwest;
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
