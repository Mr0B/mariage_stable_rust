use std::sync::Mutex;
use crate::{Deck, MutexStorage, Storage, Woman};

pub(crate) struct  TestInstances {
    pub(crate) list_man: MutexStorage,
    pub(crate) list_woman: Vec<Mutex<Woman>>,
}

impl TestInstances {
    pub(crate) fn new(list_man: MutexStorage, list_woman: Vec<Mutex<Woman>>) -> TestInstances {
        TestInstances {
            list_man,
            list_woman,
        }
    }

    pub(crate) fn list_man_mutable(&mut self) -> &mut MutexStorage {
        &mut self.list_man
    }

    pub(crate) fn list_woman_mutable(&mut self) -> &mut Vec<Mutex<Woman>> {
        &mut self.list_woman
    }
}
