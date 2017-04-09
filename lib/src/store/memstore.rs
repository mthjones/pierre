use std::cell::RefCell;

use super::{Store, Keyed};

pub struct MemStore<T> {
    items: RefCell<Vec<T>>
}

impl<T> Store for MemStore<T>
  where T: Keyed + Clone
{
    type Item = T;

    fn list(&self) -> Result<Vec<Self::Item>, ()> {
        Ok(self.items.borrow().clone())
    }

    fn find(&self, key: <Self::Item as Keyed>::Key) -> Result<Option<Self::Item>, ()> {
        Ok(self.items.borrow().iter().find(|i| i.key() == key).cloned())
    }

    fn create(&self, item: Self::Item) -> Result<(), ()> {
        self.items.borrow_mut().push(item);
        Ok(())
    }

    fn delete(&self, key: <Self::Item as Keyed>::Key) -> Result<(), ()> {
        if let Some(pos) = self.items.borrow().iter().position(|i| i.key() == key) {
            let _ = self.items.borrow_mut().swap_remove(pos);
        }
        Ok(())
    }
}