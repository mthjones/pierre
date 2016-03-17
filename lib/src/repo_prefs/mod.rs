use postgres::{Connection, SslMode};
mod RepoPrefsDataModel;

pub struct RepoPrefsManager {
    conn: Connection,
}

impl RepoPrefsManager {
    pub fn new(db: String) -> Self {
        RepoPrefsManager {
            conn: try!(Connection::connect(&db[..], &SslMode::None)),
        }
        RepoPrefsDataModel::initialize(&conn).expect("DB Error!");
    }
    pub fn fetch_all(&self) {
        try!(RepoPrefsDataModel::all(&self.conn));
    }
    pub fn update(&self, audience: String, project: String, repo: String) {
        try!(RepoPrefsDataModel::insert(&self.conn, audience, project, repo));
    }
}