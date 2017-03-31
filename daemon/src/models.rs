use postgres::{self, Connection};
use rusoto::dynamodb::{DynamoDbClient, BatchGetItemInput, BatchGetRequestMap, KeysAndAttributes, KeyList, AttributeMap};

use ::stash::PullRequest;

struct Stored {
    
}

impl Stored {
    fn save(&self) -> Result<(), ()>;
    fn delete(&self) -> Result<(), ()>;
}

trait Store {
    type Item;
    type Key;

    fn list() -> Result<Vec<Self::Item>, ()>;
    fn find(key: &Self::Key) -> Result<Self::Item, ()>;
}

impl From<AttributeMap> for PullRequest {
    fn from(map: AttributeMap) -> Self {
        PullRequest {
            ...
        }
    }
}

struct PullRequestDataStore<'a> {
    dynamo: &'a DynamoDbClient
}

impl<'a> Store for PullRequestDataStore<'a> {
    type Item = PullRequest;
    type Key = PullRequestKey;

    fn list() -> Result<Vec<Self::Item>, ()> {
        let request_map: BatchGetRequestMap = [
            ["PullRequests", KeysAndAttributes {
                keys: KeyList::new(),
                ..KeysAndAttributes::default()
            }]
        ].iter().collect();

        let response = self.dynamo.batch_get_item(BatchGetItemInput {
            request_items: request_map,
            return_consumed_capacity: None
        }).map_err(|_|, ())?;

        if let Some(pull_request_data) = response.responses.map(|r| r.get("PullRequests")) {
            Ok(pull_request_data.iter().map(PullRequest::from).collect())
        } else {
            Err(())
        }
    }

    fn find(key: &Self::Key) -> Result<Self::Item, ()> {

    }
}

#[derive(PartialEq,Eq)]
pub struct PullRequestKey {
    pub id: u32,
    pub project: String,
    pub repo: String
}

impl Stored<PullRequestKey> for PullRequest {
    fn save(&self) -> Result<(), ()>;
    fn delete(&self) -> Result<(), ()>;
}

#[derive(PartialEq,Eq)]
pub struct PullRequestDataModel {
    pub id: u32,
    pub project: String,
    pub repo: String
}

impl PullRequestDataModel {
    pub fn initialize(conn: &Connection) -> Result<(), postgres::error::Error> {
        try!(conn.execute("CREATE TABLE IF NOT EXISTS processed_prs (
            id          OID,
            project     VARCHAR(10),
            repo        VARCHAR(100),
            PRIMARY KEY (id, project, repo)
        )", &[]));
        Ok(())
    }

    pub fn all(conn: &Connection) -> Result<Vec<PullRequestDataModel>, postgres::error::Error> {
        let statement = try!(conn.prepare("SELECT id, project, repo FROM processed_prs"));
        let results = try!(statement.query(&[]));
        Ok(results.iter().map(|r| PullRequestDataModel {
            id: r.get(0),
            project: r.get(1),
            repo: r.get(2)
        }).collect::<Vec<_>>())
    }

    pub fn create(conn: &Connection, id: u32, project: &String, repo: &String) -> Result<(), postgres::error::Error> {
        try!(conn.execute("INSERT INTO processed_prs (id, project, repo) VALUES ($1, $2, $3)", &[&id, project, repo]));
        Ok(())
    }

    pub fn delete(conn: &Connection, id: u32, project: &String, repo: &String) -> Result<(), postgres::error::Error> {
        try!(conn.execute("DELETE FROM processed_prs WHERE id = $1 AND project = $2 AND repo = $3", &[&id, project, repo]));
        Ok(())
    }
}

impl From<PullRequest> for PullRequestDataModel {
    fn from(pr: PullRequest) -> PullRequestDataModel {
        PullRequestDataModel {
            id: pr.id,
            project: pr.to_ref.repository.project.key.clone(),
            repo: pr.to_ref.repository.slug.clone()
        }
    }
}
