use rusoto::dynamodb::{self, AttributeValue, AttributeMap};

use ::store::Keyed;

#[derive(Clone)]
pub struct PullRequestData {
    pub id: u32,
    pub project: String,
    pub repo: String,
}

impl Keyed for PullRequestData {
    type Key = PullRequestKey;

    fn key(&self) -> Self::Key {
        PullRequestKey {
            id: self.id,
            project: self.project.clone(),
            repo: self.repo.clone()
        }
    }
}

impl From<dynamodb::AttributeMap> for PullRequestData {
    fn from(map: AttributeMap) -> Self {
        PullRequestData {
            id: map.get("id").and_then(|val| val.n.as_ref()).and_then(|n| n.parse::<u32>().ok()).unwrap(),
            project: map.get("project").and_then(|val| val.s.clone()).unwrap(),
            repo: map.get("repo").and_then(|val| val.s.clone()).unwrap(),
        }
    }
}

impl Into<dynamodb::AttributeMap> for PullRequestData {
    fn into(self) -> AttributeMap {
        vec![
            ("id".to_owned(), AttributeValue { n: self.id.to_string().into(), ..AttributeValue::default() }),
            ("project".to_owned(), AttributeValue { s: self.project.clone().into(), ..AttributeValue::default() }),
            ("repo".to_owned(), AttributeValue { s: self.repo.clone().into(), ..AttributeValue::default() }),
        ].into_iter().collect()
    }
}

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct PullRequestKey {
    id: u32,
    project: String,
    repo: String
}

impl Into<dynamodb::Key> for PullRequestKey {
    fn into(self) -> dynamodb::Key {
        dynamodb::Key::from(vec![
            ("id".to_owned(), AttributeValue { n: self.id.to_string().into(), ..AttributeValue::default() }),
            ("project".to_owned(), AttributeValue { s: self.project.into(), ..AttributeValue::default() }),
            ("repo".to_owned(), AttributeValue { s: self.repo.into(), ..AttributeValue::default() }),
        ].into_iter().collect())
    }
}