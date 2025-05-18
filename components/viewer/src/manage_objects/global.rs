use once_cell::sync::Lazy;

use crate::types::ThreadSafeVecRw;

use super::SpawnObjectRequest;

pub static SPAWN_OBJECT_REQUEST_LIST: Lazy<SpawnObjectRequestList> =
    Lazy::new(SpawnObjectRequestList::new);

#[derive(Debug)]
pub struct SpawnObjectRequestList {
    pub queue: ThreadSafeVecRw<SpawnObjectRequest>,
}

impl SpawnObjectRequestList {
    pub fn new() -> Self {
        SpawnObjectRequestList {
            queue: ThreadSafeVecRw::new(),
        }
    }
}
