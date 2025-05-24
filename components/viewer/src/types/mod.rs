use std::sync::{Arc, RwLock, RwLockReadGuard};

#[derive(Clone, Debug)]
pub struct ThreadSafeVecRw<T> {
    inner: Arc<RwLock<Vec<T>>>,
}

impl<T> ThreadSafeVecRw<T> {
    /// Creates an empty, thread-safe vector.
    pub fn new() -> Self {
        ThreadSafeVecRw {
            inner: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Appends a value to the inner vector with write-lock protection.
    pub fn push(&self, value: T) {
        let mut vec = self.inner.write().unwrap();
        vec.push(value);
    }

    /// Retrieves and clones the element at the given index, if any.
    pub fn get(&self, index: usize) -> Option<T>
    where
        T: Clone,
    {
        let vec = self.inner.read().unwrap();
        vec.get(index).cloned()
    }

    /// Returns the current number of elements in the vector.
    pub fn len(&self) -> usize {
        let vec = self.inner.read().unwrap();
        vec.len()
    }

    /// Returns a read guard for the internal vector or an error if poisoned.
    pub fn get_reader(
        &self,
    ) -> Result<RwLockReadGuard<Vec<T>>, std::sync::PoisonError<RwLockReadGuard<Vec<T>>>> {
        Ok(self.inner.read()?)
    }
}
