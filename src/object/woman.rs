use crate::object::man::Man;

//favorite is public only for testing purposes
pub struct Woman {
    pub name: i32,
    pub preference: Vec<i32>,
    favorite: Man,
}

impl Woman{
    pub(crate) fn new(name: i32, preference: Vec<i32>, favorite: Man) -> Woman {
        Woman { name, preference, favorite }
    }
    pub(crate) fn check_favorite(&mut self, pretending: Man) -> Man {
        let position_pretending =self.preference.iter().position(|&r| r == pretending.name).unwrap();
        let position_favorite=self.preference.iter().position(|&r| r == self.favorite.name).unwrap_or(999999999);
        return if position_favorite > position_pretending {
            let former_favorite = self.favorite().clone();
            *self.favorite_mutable() = pretending;
            former_favorite
        } else {
            pretending
        }
    }

    pub(crate) fn favorite(&self) -> &Man {
        &self.favorite
    }

    pub(crate) fn favorite_mutable(&mut self) -> &mut Man {
        &mut self.favorite
    }
}