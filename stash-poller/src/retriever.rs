use reqwest;

use std::error;
use std::sync::{Arc, Mutex};

pub trait PullRequestRetriever {
    type Error;

    fn retrieve(&self, project: &str, repo: &str) -> Result<Vec<::api::PullRequest>, Self::Error>;
}

#[derive(Clone)]
pub struct StashPullRequestRetriever {
    base_url: String,
    username: String,
    password: Option<String>,
    client: Arc<reqwest::Client>
}

impl StashPullRequestRetriever {
    pub fn new(client: Arc<reqwest::Client>, auth: (String, Option<String>), base_url: String) -> Self {
        StashPullRequestRetriever {
            base_url: base_url,
            username: auth.0,
            password: auth.1,
            client: client
        }
    }
}

impl PullRequestRetriever for StashPullRequestRetriever {
    type Error = Box<error::Error>;

    fn retrieve(&self, project: &str, repo: &str) -> Result<Vec<::api::PullRequest>, Self::Error> {
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
            project.to_uppercase(),
            repo.to_lowercase());
            
        let mut response = self.client.get(&url).headers(headers).send()?;

        let prs: ::api::PagedData<::api::PullRequest> = response.json()?;
        Ok(prs.values)
    }
}

#[allow(dead_code)]
pub struct DummyRetriever {
    count: Mutex<u32>
}

#[allow(dead_code)]
impl DummyRetriever {
    pub fn new() -> Self {
        DummyRetriever {
            count: Mutex::new(1)
        }
    }
}

fn create_dummy_pr(id: u32, project: &str, repo: &str) -> ::api::PullRequest {
    ::api::PullRequest {
        id: id,
        to_ref: ::api::Ref {
            repository: ::api::Repository {
                name: repo.to_owned(),
                project: ::api::Project {
                    name: project.to_owned(),
                    ..::api::Project::default()
                },
                ..::api::Repository::default()
            },
            ..::api::Ref::default()
        },
        ..::api::PullRequest::default()
    }
}

impl PullRequestRetriever for DummyRetriever {
    type Error = ();

    fn retrieve(&self, project: &str, repo: &str) -> Result<Vec<::api::PullRequest>, Self::Error> {
        let mut count = self.count.lock().unwrap();
        let prs = vec![
            create_dummy_pr(*count, project, repo),
            create_dummy_pr(*count + 1, project, repo),
            create_dummy_pr(*count + 2, project, repo)
        ];
        *count += 3;
        Ok(prs)
    }
}