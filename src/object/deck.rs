use crate::man::Man;

pub(crate) struct Storage {men: Vec<Man>}

pub (crate) trait Deck {
    fn new(men: Vec<Man>) -> Self;

    fn add(&mut self, newly_single: Man);

    fn pop(&mut self) -> Option<Man>;
}

impl Deck for Storage {
    fn new(men: Vec<Man>) -> Storage {
        Storage {men}
    }

    fn add(&mut self, newly_single: Man) {
        self.men.push(newly_single);
    }

    fn pop(&mut self) -> Option<Man> {
        self.men.pop()
    }
}