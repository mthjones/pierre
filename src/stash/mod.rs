use std::error;
use std::io::Read;

use serde_json;
use hyper;
use hyper::header::{Headers, Authorization, Basic};

use ::watcher::Retriever;

mod api;
pub use self::api::*;

pub struct StashPullRequestDataRetriever {
    base_url: String,
    username: String,
    password: Option<String>,
    client: hyper::Client,
    project: ::config::Project
}

impl StashPullRequestDataRetriever {
    pub fn new(project: ::config::Project, auth: (String, Option<String>), base_url: String) -> Self {
        let client = hyper::Client::new();
        StashPullRequestDataRetriever {
            base_url: base_url,
            username: auth.0,
            password: auth.1,
            client: client,
            project: project
        }
    }
}

impl Retriever<PullRequest> for StashPullRequestDataRetriever {
    fn retrieve(&self) -> Result<Vec<PullRequest>, Box<error::Error>> {
        let mut headers = Headers::new();
        headers.set(Authorization(Basic { username: self.username.clone(), password: self.password.clone() }));
        headers.set(hyper::header::Connection::close());
        headers.set(hyper::header::UserAgent("pierre/1.0".to_owned()));

        let url = format!("{}/rest/api/1.0/projects/{}/repos/{}/pull-requests",
            self.base_url,
            self.project.id.to_uppercase(),
            self.project.repo.to_lowercase());

        let mut response = try!(self.client.get(&url).headers(headers.clone()).send());

        let mut body = String::new();
        try!(response.read_to_string(&mut body));

        let prs: PagedData<PullRequest> = try!(serde_json::from_str(&body));
        Ok(prs.values)
    }
}
