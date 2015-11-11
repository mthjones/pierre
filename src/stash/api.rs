use std::collections::HashMap;

#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct PagedData<T> {
    pub size: u32,
    pub limit: u32,
    #[serde(rename="isLastPage")]
    pub is_last_page: bool,
    pub values: Vec<T>,
    pub start: u32,
    #[serde(rename="nextPageStart")]
    pub next_page_start: Option<u32>,
}

#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct Project {
    pub id: u32,
    pub key: String,
    pub name: String,
    pub public: Option<bool>,
    pub description: Option<String>,
    #[serde(rename="type")]
    pub _type: String,
    pub owner: Option<User>,
    pub link: Link,
    pub links: HashMap<String, Vec<LinkHref>>,
}

#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct Ref {
    pub id: String,
    pub repository: Repository,
    #[serde(rename="displayId")]
    pub display_id: String,
    #[serde(rename="latestChangeset")]
    pub latest_changeset: String,
    #[serde(rename="latestCommit")]
    pub latest_commit: String,
}

#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct Repository {
    pub id: u32,
    pub slug: String,
    pub name: String,
    #[serde(rename="scmId")]
    pub scm_id: String,
    pub state: String,
    #[serde(rename="statusMessage")]
    pub status_message: String,
    pub forkable: bool,
    pub origin: Option<Box<Repository>>,
    pub project: Project,
    pub public: bool,
    pub link: Link,
    #[serde(rename="cloneUrl")]
    pub clone_url: String,
    pub links: HashMap<String, Vec<LinkHref>>,
}

#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct User {
    pub name: String,
    #[serde(rename="emailAddress")]
    pub email_address: Option<String>,
    pub id: u32,
    #[serde(rename="displayName")]
    pub display_name: String,
    pub active: bool,
    pub slug: String,
    #[serde(rename="type")]
    pub _type: String,
    pub link: Link,
    pub links: HashMap<String, Vec<LinkHref>>,
}

#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct Author {
    pub user: User,
    pub role: String,
    pub approved: bool,
}

#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct Link {
    pub url: String,
    pub rel: String,
}

#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct LinkHref {
    pub href: String,
    pub name: Option<String>,
}

#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct PullRequest {
    pub id: u32,
    pub version: u32,
    pub title: String,
    pub description: Option<String>,
    pub state: String,
    pub open: bool,
    pub closed: bool,
    #[serde(rename="createdDate")]
    pub created_date: u64,
    #[serde(rename="updatedDate")]
    pub updated_date: u64,
    #[serde(rename="fromRef")]
    pub from_ref: Ref,
    #[serde(rename="toRef")]
    pub to_ref: Ref,
    pub author: Author,
    pub locked: bool,
    pub reviewers: Vec<Author>,
    pub participants: Vec<Author>,
    pub attributes: HashMap<String, Vec<String>>,
    pub link: Link,
    pub links: HashMap<String, Vec<LinkHref>>,
}
