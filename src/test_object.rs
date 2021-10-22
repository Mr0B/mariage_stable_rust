pub struct Man(pub i32, pub Vec<i32>, pub i32,);

impl Man{
    pub(crate) fn new(name: i32, preference: Vec<i32>, proposing_to: i32) -> Man {
        Man { 0: name, 1: preference, 2: proposing_to }
    }
}

//favorite is public only for testing purposes
pub struct Woman(pub i32, pub Vec<i32>, pub i32,);

impl Woman{
    pub(crate) fn new(name: i32, preference: Vec<i32>, favorite: i32) -> Woman {
        Woman { 0: name, 1: preference, 2: favorite }
    }
    pub(crate) fn check_favorite(&mut self, pretending: Man) -> i32 {
        let position_pretending =self.1.iter().position(|&r| r == pretending.0).unwrap();
        let position_favorite=self.1.iter().position(|&r| r == self.2).unwrap_or(999999999);
        return if position_favorite > position_pretending {
            let former_favorite = self.2;
            self.2 = pretending.0;
            former_favorite
        } else {
            pretending.0
        }
    }
}