use crate::object::man::Man;

#[derive(Clone)]
pub struct Woman {
    pub name: i32,
    pub preference: Vec<i32>,
    favorite: Option<Man>,
}

impl Woman {
    pub(crate) fn new(name: i32, preference: Vec<i32>, favorite: Option<Man>) -> Woman {
        Woman {
            name,
            preference,
            favorite,
        }
    }

    pub(crate) fn check_favorite(&mut self, pretending: Man) -> Option<Man> {
        if let Some(position_pretending) =
            self.preference.iter().position(|&r| r == pretending.name)
        {
            if let Some(position_favorite) = self.preference.iter().position(|&r| {
                r == match &self.favorite {
                    None => -1,
                    Some(man) => man.name,
                }
            }) {
                return if position_favorite > position_pretending {
                    let former_favorite = self.favorite().to_owned();
                    *self.favorite_mutable() = Some(pretending);
                    former_favorite
                } else {
                    Some(pretending)
                };
            } else {
                *self.favorite_mutable() = Some(pretending);
                None
            }
        } else {
            panic!("This should never ever happen")
        }
    }

    pub(crate) fn favorite(&self) -> &Option<Man> {
        &self.favorite
    }

    pub(crate) fn favorite_mutable(&mut self) -> &mut Option<Man> {
        &mut self.favorite
    }
}
