use postgres::{Connection, SslMode};
mod model;
use self::model::*;

pub struct RepoPrefsManager {
    conn: Connection,
}

impl RepoPrefsManager {
    pub fn new(db: &String) -> Self {
        let conn = Connection::connect(&db[..], &SslMode::None).unwrap();
        RepoPrefsDataModel::initialize(&conn).expect("DB Error!");
        RepoPrefsManager {
            conn: conn,
        }
    }
    pub fn fetch_all(&self) -> Vec<RepoPrefsDataModel> {
        RepoPrefsDataModel::all(&self.conn).unwrap()
    }
    pub fn update(&self, audience: String, project: String, repo: String) {
        RepoPrefsDataModel::insert(&self.conn, &audience, &project, &repo).unwrap();
    }
}