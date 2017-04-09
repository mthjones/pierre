pub mod dynamodb;
pub mod memstore;

pub trait Keyed {
    type Key: PartialEq<Self::Key>;

    fn key(&self) -> Self::Key;
}

pub trait Store {
    type Item: Keyed;

    fn list(&self) -> Result<Vec<Self::Item>, ()>;
    fn find(&self, key: <Self::Item as Keyed>::Key) -> Result<Option<Self::Item>, ()>;

    fn create(&self, item: Self::Item) -> Result<(), ()>;
    fn delete(&self, key: <Self::Item as Keyed>::Key) -> Result<(), ()>;
}
