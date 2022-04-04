#[derive(Clone)]
pub struct Man {
    pub name: usize,
    pub preference: Vec<usize>,
    proposing_to: usize,
}

impl Man {
    pub(crate) fn new(name: usize, preference: Vec<usize>, proposing_to: usize) -> Man {
        Man {
            name,
            preference,
            proposing_to,
        }
    }

    pub(crate) fn find_next_woman(&mut self) -> Option<&usize> {
        *self.proposing_to_mutable() = self.proposing_to() + 1;
        if let Some(target) = self.preference().get(*self.proposing_to()) {
            return Some(target);
        }
        return None;
    }

    pub(crate) fn proposing_to(&self) -> &usize {
        &self.proposing_to
    }

    fn proposing_to_mutable(&mut self) -> &mut usize {
        &mut self.proposing_to
    }

    fn preference(&self) -> &Vec<usize> {
        &self.preference
    }
}
