#![feature(mutex_unlock)]

use std::sync::{Arc, Mutex};

pub(crate) struct Deque<T: ?Sized> {
    mutex: Arc<Mutex<Vec<Box<T>>>>
}


impl<T> Deque<T> {
    pub fn new() -> Deque<T> {
        Self {
            mutex: Arc::new(Mutex::new(vec![]))
        }
    }

    pub fn schedule(&mut self, item: Box<T>) {
        let data = self.mutex.get_mut().unwrap();
        data.push(item);
    }

    pub fn accept_one(&mut self) -> &T {
        let data = self.mutex.get_mut().unwrap();
        let res: Box<T> = data.drain(0..1)[0];
        return &res;
    }
}