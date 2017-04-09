use std::sync::RwLock;

use super::{Store, Keyed};

#[allow(dead_code)]
pub struct MemStore<T> {
    items: RwLock<Vec<T>>
}

impl<T> MemStore<T> {
    pub fn new() -> Self {
        MemStore {
            items: RwLock::new(Vec::new())
        }
    }
}

impl<T> Store for MemStore<T>
  where T: Keyed + Clone
{
    type Item = T;

    fn list(&self) -> Result<Vec<Self::Item>, ()> {
        let items = self.items.read().unwrap();
        Ok((*items).clone())
    }

    fn find(&self, key: <Self::Item as Keyed>::Key) -> Result<Option<Self::Item>, ()> {
        let items = self.items.read().unwrap();
        Ok((*items).iter().find(|i| i.key() == key).cloned())
    }

    fn create(&self, item: Self::Item) -> Result<(), ()> {
        let mut items = self.items.write().unwrap();
        (*items).push(item);
        Ok(())
    }

    fn delete(&self, key: <Self::Item as Keyed>::Key) -> Result<(), ()> {
        let mut items = self.items.write().unwrap();
        if let Some(pos) = (*items).iter().position(|i| i.key() == key) {
            let _ = (*items).swap_remove(pos);
        }
        Ok(())
    }
}