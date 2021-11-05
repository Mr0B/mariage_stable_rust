use crate::man::Man;

pub struct Deck {
    pub men: Vec<Man>,
}

impl Deck{
    pub(crate) fn new(men: Vec<Man>) -> Deck {
        Deck { men }
    }

    pub(crate) fn give_first(&mut self) -> Option<Man> {
        self.men.pop()
    }

    pub(crate) fn put_at_the_end(&mut self, newly_single: Man) {
        self.men.push(newly_single);
    }

    pub(crate) fn empty(&mut self) -> bool {
        return self.men.is_empty();
    }
}