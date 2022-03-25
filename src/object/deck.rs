use crate::man::Man;
use std::sync::Mutex;

pub(crate) struct Storage<T> {
    men: Vec<T>,
}

pub(crate) struct MutexStorage<T> {
    men_mutex: Mutex<Vec<T>>,
}

pub(crate) trait Deck<T> {
    fn new() -> Self;

    fn add(&mut self, newly_single: T);

    fn pop(&mut self) -> Option<T>;
}

impl<T> Deck<T> for Storage<T> {
    fn new() -> Storage<T> {
        Storage { men: vec![] }
    }

    fn add(&mut self, newly_single: T) {
        self.men.push(newly_single);
    }

    fn pop(&mut self) -> Option<T> {
        self.men.pop()
    }
}

impl<T> Deck<T> for MutexStorage<T> {
    fn new() -> MutexStorage<T> {
        MutexStorage {
            men_mutex: Mutex::new(vec![]),
        }
    }

    fn add(&mut self, newly_single: T) {
        self.men_mutex.lock().unwrap().push(newly_single);
    }

    fn pop(&mut self) -> Option<T> {
        self.men_mutex.lock().unwrap().pop()
    }
}
