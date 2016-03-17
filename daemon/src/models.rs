use postgres::{self, Connection};

use ::stash::PullRequest;

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
