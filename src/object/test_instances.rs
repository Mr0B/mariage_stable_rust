use crate::{Deck, Man, Storage, Woman};
use std::sync::Mutex;
pub(crate) struct TestInstances {
    pub(crate) list_man: Mutex<Storage<Man>>,
    pub(crate) list_woman: Vec<Mutex<Woman>>,
}

impl TestInstances {
    pub(crate) fn new(list_man: Mutex<Storage<Man>>, list_woman: Vec<Mutex<Woman>>) -> TestInstances {
        TestInstances {
            list_man,
            list_woman,
        }
    }
}
