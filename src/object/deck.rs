use std::sync::Mutex;
use crate::man::Man;

pub(crate) struct Storage {men: Vec<Man>}

pub(crate) struct MutexStorage {men_mutex: Mutex<Vec<Man>>}

pub (crate) trait Deck {
    fn new() -> Self;

    fn add(&mut self, newly_single: Man);

    fn pop(&mut self) -> Option<Man>;
}

impl Deck for Storage {
    fn new() -> Storage {
        Storage {men : vec![]}
    }

    fn add(&mut self, newly_single: Man) {
        self.men.push(newly_single);
    }

    fn pop(&mut self) -> Option<Man> {
        self.men.pop()
    }
}

impl Deck for MutexStorage {
    fn new() -> MutexStorage {
        MutexStorage {men_mutex : Mutex::new(vec![])}
    }

    fn add(&mut self, newly_single: Man) {
        self.men_mutex.lock().unwrap().push(newly_single);
    }

    fn pop(&mut self) -> Option<Man> {
        self.men_mutex.lock().unwrap().pop()
    }
}