#[derive(Clone)]
pub struct Man {
    pub name: i32,
    preference: Vec<i32>,
    proposing_to: i32,
}

impl Man {
    pub(crate) fn new(name: i32, preference: Vec<i32>, proposing_to: i32) -> Man {
        Man {
            name,
            preference,
            proposing_to,
        }
    }

    pub(crate) fn find_next_woman(&mut self) -> Option<&i32> {
        *self.proposing_to_mutable() = self.proposing_to() + 1;
        if let Some(target) = self.preference().get(*self.proposing_to() as usize) {
            return Some(target);
        }
        return None;
    }

    pub(crate) fn proposing_to(&self) -> &i32 {
        &self.proposing_to
    }

    fn proposing_to_mutable(&mut self) -> &mut i32 {
        &mut self.proposing_to
    }

    fn preference(&self) -> &Vec<i32> {
        &self.preference
    }
}
