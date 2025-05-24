use std::ops::Deref;

use once_cell::sync::Lazy;

use crate::types::ThreadSafeVecRw;

pub static INTERNAL_REQUEST_LIST: Lazy<InternalRequestList> = Lazy::new(InternalRequestList::new);

#[derive(Debug)]
pub struct InternalRequestList {
    list: ThreadSafeVecRw<super::request::InternalRequest>,
}

impl InternalRequestList {
    /// Creates a new internal request list backed by a thread-safe vector.
    pub fn new() -> Self {
        InternalRequestList {
            list: ThreadSafeVecRw::new(),
        }
    }
}

impl Deref for InternalRequestList {
    type Target = ThreadSafeVecRw<super::request::InternalRequest>;

    /// Returns a reference to the underlying thread-safe request vector.
    fn deref(&self) -> &Self::Target {
        &self.list
    }
}
