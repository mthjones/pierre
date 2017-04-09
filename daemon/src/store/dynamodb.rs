use std::marker::PhantomData;

use rusoto;
use rusoto::dynamodb::{self, DynamoDbClient, BatchGetItemInput, BatchGetRequestMap, DeleteItemInput, GetItemInput, KeysAndAttributes, PutItemInput};

use super::{Store, Keyed};

pub struct DynamoDataStore<'a, T, P, D>
  where T: Keyed + From<dynamodb::AttributeMap> + Into<dynamodb::AttributeMap>,
        T::Key: Into<dynamodb::Key>,
        P: 'a + rusoto::ProvideAwsCredentials,
        D: 'a + rusoto::DispatchSignedRequest
{
    client: &'a DynamoDbClient<P, D>,
    table_name: String,
    item_ty: PhantomData<T>
}

impl<'a, T, P:, D: rusoto::DispatchSignedRequest> DynamoDataStore<'a, T, P, D>
  where T: Keyed + From<dynamodb::AttributeMap> + Into<dynamodb::AttributeMap>,
        T::Key: Into<dynamodb::Key>,
        P: 'a + rusoto::ProvideAwsCredentials,
        D: 'a + rusoto::DispatchSignedRequest
{
    pub fn new<S: Into<String>>(client: &'a DynamoDbClient<P, D>, table_name: S) -> Self {
        DynamoDataStore {
            client: client,
            table_name: table_name.into(),
            item_ty: PhantomData
        }
    }
}

impl<'a, T, P, D> Store for DynamoDataStore<'a, T, P, D>
  where T: Keyed + From<dynamodb::AttributeMap> + Into<dynamodb::AttributeMap>,
        T::Key: Into<dynamodb::Key>,
        P: 'a + rusoto::ProvideAwsCredentials,
        D: 'a + rusoto::DispatchSignedRequest
{
    type Item = T;

    fn list(&self) -> Result<Vec<Self::Item>, ()> {
        let request_map: BatchGetRequestMap = vec![
            (self.table_name.clone(), KeysAndAttributes::default())
        ].into_iter().collect();

        let response = self.client.batch_get_item(&BatchGetItemInput {
            request_items: request_map,
            ..BatchGetItemInput::default()
        }).map_err(|_| ())?;

        match response.responses {
            Some(map) => {
                match map.get(&self.table_name) {
                    Some(items) => {
                        Ok(items.iter().map(|x| Self::Item::from(x.clone())).collect())
                    },
                    None => Err(())
                }
            },
            None => Err(())
        }
    }

    fn find(&self, key: <Self::Item as Keyed>::Key) -> Result<Option<Self::Item>, ()> {
        let response = self.client.get_item(&GetItemInput {
            table_name: self.table_name.clone(),
            key: key.into(),
            ..GetItemInput::default()
        }).map_err(|_| ())?;

        if let Some(item) = response.item {
            Ok(Some(Self::Item::from(item)))
        } else {
            Ok(None)
        }
    }

    fn create(&self, item: Self::Item) -> Result<(), ()> {
        self.client.put_item(&PutItemInput {
            table_name: self.table_name.clone(),
            item: item.into(),
            ..PutItemInput::default()
        }).map(|_| ()).map_err(|_| ())
    }

    fn delete(&self, key: <Self::Item as Keyed>::Key) -> Result<(), ()> {
        self.client.delete_item(&DeleteItemInput {
            table_name: self.table_name.clone(),
            key: key.into(),
            ..DeleteItemInput::default()
        }).map(|_| ()).map_err(|_| ())
    }
}
