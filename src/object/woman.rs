use crate::object::man::Man;

#[derive(Clone)]
pub struct Woman {
    pub name: usize,
    pub preference: Vec<usize>,
    favorite: Option<Man>,
}

impl Woman {
    pub(crate) fn new(name: usize, preference: Vec<usize>, favorite: Option<Man>) -> Woman {
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
            if let Some(position_favorite) = self
                .preference
                .iter()
                .position(|&r| r == match &self.favorite {
                    None => {usize::MAX}
                    Some(man) => {man.name}
                })
            {
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
