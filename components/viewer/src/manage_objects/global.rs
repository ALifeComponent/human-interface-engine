use std::ops::Deref;

use once_cell::sync::Lazy;

use crate::types::ThreadSafeVecRw;

pub static REQUEST_LIST: Lazy<RequestList> = Lazy::new(RequestList::new);

#[derive(Debug)]
pub struct RequestList {
    list: ThreadSafeVecRw<super::request::InternalRequest>,
}

impl RequestList {
    pub fn new() -> Self {
        RequestList {
            list: ThreadSafeVecRw::new(),
        }
    }
}

impl Deref for RequestList {
    type Target = ThreadSafeVecRw<super::request::InternalRequest>;

    fn deref(&self) -> &Self::Target {
        &self.list
    }
}
