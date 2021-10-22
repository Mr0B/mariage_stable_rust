pub struct Man(pub i32, pub Vec<i32>, pub i32,);

impl Man{
    pub(crate) fn new(name: i32, preference: Vec<i32>, proposing_to: i32) -> Man {
        Man { 0: name, 1: preference, 2: proposing_to }
    }
}