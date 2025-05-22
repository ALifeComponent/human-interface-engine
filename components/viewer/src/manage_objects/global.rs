use std::ops::{Deref, DerefMut};

use once_cell::sync::Lazy;

use crate::types::ThreadSafeVecRw;

use super::SpawnObjectRequest;

pub static SPAWN_OBJECT_REQUEST_LIST: Lazy<SpawnObjectRequestList> =
    Lazy::new(SpawnObjectRequestList::new);

#[derive(Debug)]
pub struct SpawnObjectRequestList {
    queue: ThreadSafeVecRw<SpawnObjectRequest>,
}

impl SpawnObjectRequestList {
    pub fn new() -> Self {
        SpawnObjectRequestList {
            queue: ThreadSafeVecRw::new(),
        }
    }
}

impl Deref for SpawnObjectRequestList {
    type Target = ThreadSafeVecRw<SpawnObjectRequest>;

    fn deref(&self) -> &Self::Target {
        &self.queue
    }
}

impl DerefMut for SpawnObjectRequestList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.queue
    }
}
