use crate::man::Man;

pub struct Deck(pub Vec<Man>);

impl Deck{
    pub(crate) fn new(men: Vec<Man>) -> Deck {
        Deck { 0: men}
    }

    pub(crate) unsafe fn give(&self) -> &Man {
        let first: &Man = self.0.get_unchecked(0);
        return first
    }

    pub(crate) fn put_at_the_end(&mut self, newly_single: Man) {
        &self.0.push(newly_single);
    }

    pub(crate) fn is_empty() -> bool {
        let check: bool = self::deck.0.is_empty();
        return check
    }
}