use std::sync::{Arc, RwLock, RwLockReadGuard};

#[derive(Clone, Debug)]
pub struct ThreadSafeVecRw<T> {
    inner: Arc<RwLock<Vec<T>>>,
}

impl<T> ThreadSafeVecRw<T> {
    pub fn new() -> Self {
        ThreadSafeVecRw {
            inner: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub fn push(&self, value: T) {
        let mut vec = self.inner.write().unwrap();
        vec.push(value);
    }

    pub fn get(&self, index: usize) -> Option<T>
    where
        T: Clone,
    {
        let vec = self.inner.read().unwrap();
        vec.get(index).cloned()
    }

    pub fn len(&self) -> usize {
        let vec = self.inner.read().unwrap();
        vec.len()
    }

    pub fn get_reader(
        &self,
    ) -> Result<RwLockReadGuard<Vec<T>>, std::sync::PoisonError<RwLockReadGuard<Vec<T>>>> {
        Ok(self.inner.read()?)
    }
}
