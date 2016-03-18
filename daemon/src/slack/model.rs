use postgres::{self, Connection};

#[derive(PartialEq,Eq)]
pub struct RepoPrefsDataModel {
    pub audience: String,
    pub project: String,
    pub repo: String
}

impl RepoPrefsDataModel {
    pub fn initialize(conn: &Connection) -> Result<(), postgres::error::Error> {
        try!(conn.execute("CREATE TABLE IF NOT EXISTS repo_prefs (
            audience    VARCHAR(10),
            project     VARCHAR(10),
            repo        VARCHAR(100),
            PRIMARY KEY (audience, project, repo)
        )", &[]));        
        Ok(())
    }

    pub fn all(conn: &Connection) -> Result<Vec<RepoPrefsDataModel>, postgres::error::Error> {
        let statement = try!(conn.prepare("SELECT * FROM repo_prefs"));
        let results = try!(statement.query(&[]));
        Ok(results.iter().map(|r| RepoPrefsDataModel {
            audience: r.get(0),
            project: r.get(1),
            repo: r.get(2)
        }).collect::<Vec<_>>())
    }

    pub fn all_for_scope(conn: &Connection, project: &String, repo: &String) -> Result<Vec<RepoPrefsDataModel>, postgres::error::Error> {
        let statement = try!(conn.prepare("SELECT * FROM repo_prefs WHERE project = $1 AND repo = $2"));
        let results = try!(statement.query(&[project, repo]));
        Ok(results.iter().map(|r| RepoPrefsDataModel {
            audience: r.get(0),
            project: r.get(1),
            repo: r.get(2)
        }).collect::<Vec<_>>())
    }

    pub fn insert(conn: &Connection, audience: &String, project: &String, repo: &String) -> Result<(), postgres::error::Error> {
        try!(conn.execute("INSERT INTO repo_prefs (audience, project, repo) VALUES ($1, $2, $3)", &[audience, project, repo]));
        Ok(())
    }
    
    pub fn delete(conn: &Connection, audience: &String, project: &String, repo: &String) -> Result<(), postgres::error::Error> {
        try!(conn.execute("DELETE FROM repo_prefs WHERE id = $1 AND project = $2 AND repo = $3", &[audience, project, repo]));
        Ok(())
    }
}