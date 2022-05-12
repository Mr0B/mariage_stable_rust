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
        match &self.favorite {
            None => {
                self.favorite = Some(pretending);
                None
            }
            Some(favorite) => {
                if self.prefer_man(&pretending, &favorite) {
                    let former_favorite = self.favorite.take();
                    self.favorite = Some(pretending);
                    former_favorite
                } else {
                    Some(pretending)
                }
            }
        }
    }

    pub(crate) fn favorite(&self) -> &Option<Man> {
        &self.favorite
    }

    pub(crate) fn favorite_mutable(&mut self) -> &mut Option<Man> {
        &mut self.favorite
    }

    pub(crate) fn prefer_man(&self, man1: &Man, man2: &Man) -> bool {
        self.preference.iter().position(|&r| r == man1.name)
            < self.preference.iter().position(|&r| r == man2.name)
    }

    pub(crate) fn position_woman_preference(&self, man: &Man) -> usize {
        for i in 0..self.preference.len() {
            if self.preference[i] == man.name {
                return i;
            }
        }
        return usize::MAX;
    }
}
