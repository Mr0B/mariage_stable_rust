#[derive(Clone)]
pub struct Man(pub i32, pub Vec<i32>, pub i32,);

impl Man{
    pub(crate) fn new(name: i32, preference: Vec<i32>, proposing_to: i32) -> Man {
        Man { 0: name, 1: preference, 2: proposing_to }
    }

    pub(crate) fn proposing_to(&self) -> &i32 {
        &self.2
    }

    pub(crate) fn proposing_to_mutable(&mut self) -> &mut i32 {
        &mut self.2
    }

    pub(crate) fn preference(&self) -> &Vec<i32> {
        &self.1
    }
}