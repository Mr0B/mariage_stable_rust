use crate::man::Man;

pub struct Deck(pub Vec<Man>);

impl Deck{
    pub(crate) fn new(men: Vec<Man>) -> Deck {
        Deck { 0: men}
    }

    pub(crate) fn give_first(&mut self) -> Option<Man> {
        self.0.pop()
    }

    pub(crate) fn put_at_the_end(&mut self, newly_single: Man) {
        self.0.push(newly_single);
    }

    pub(crate) fn empty(&mut self) -> bool {
        return self.0.is_empty();
    }
}