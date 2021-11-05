#[derive(Clone)]
pub struct Man {
    pub name: i32,
    preference: Vec<i32>,
    proposing_to: i32,
}

impl Man{
    pub(crate) fn new(name: i32, preference: Vec<i32>, proposing_to: i32) -> Man {
        Man { name, preference, proposing_to }
    }

    pub(crate) fn proposing_to(&self) -> &i32 {
        &self.proposing_to
    }

    pub(crate) fn proposing_to_mutable(&mut self) -> &mut i32 {
        &mut self.proposing_to
    }

    pub(crate) fn preference(&self) -> &Vec<i32> {
        &self.preference
    }
}