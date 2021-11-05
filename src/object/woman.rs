use crate::object::man::Man;

//favorite is public only for testing purposes
pub struct Woman(pub i32, pub Vec<i32>, pub Man);

impl Woman{
    pub(crate) fn new(name: i32, preference: Vec<i32>, favorite: Man) -> Woman {
        Woman { 0: name, 1: preference, 2: favorite }
    }
    pub(crate) fn check_favorite(&mut self, pretending: Man) -> Man {
        let position_pretending =self.1.iter().position(|&r| r == pretending.name).unwrap();
        let position_favorite=self.1.iter().position(|&r| r == self.2.name).unwrap_or(999999999);
        return if position_favorite > position_pretending {
            let former_favorite = self.favorite().clone();
            *self.favorite_mutable() = pretending;
            former_favorite
        } else {
            pretending
        }
    }

    pub(crate) fn favorite(&self) -> &Man {
        &self.2
    }

    pub(crate) fn favorite_mutable(&mut self) -> &mut Man {
        &mut self.2
    }
}