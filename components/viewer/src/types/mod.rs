use crossbeam_channel::{Receiver, Sender, unbounded};

#[derive(Debug)]
pub struct MultiThreadQueue<T> {
    sender: Sender<T>,
    receiver: Receiver<T>,
}

impl<T> MultiThreadQueue<T> {
    pub fn new() -> Self {
        let (sender, receiver) = unbounded();
        MultiThreadQueue { sender, receiver }
    }

    /// ノンブロッキングなenqueue
    pub fn enqueue(&self, item: T) {
        self.sender.send(item).expect("send failed");
    }

    /// ノンブロッキングなdequeue
    /// アイテムがなければ`None`
    pub fn try_dequeue(&self) -> Option<T> {
        self.receiver.try_recv().ok()
    }

    /// ブロッキングなdequeue
    /// アイテムが来るまで待機
    pub fn blocking_dequeue(&self) -> T {
        self.receiver.recv().expect("receive failed")
    }

    /// Receiverをクローンして複数スレッドから読み出し可能に
    pub fn clone_receiver(&self) -> Receiver<T> {
        self.receiver.clone()
    }
}

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
