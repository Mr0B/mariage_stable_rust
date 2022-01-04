#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::{generate_preference, marriage_stable};
    use crate::object::{man, woman};
    use crate::object::deck::{Deck, Storage};
    use crate::object::woman::Woman;

    #[test]
    fn test_marriage_stable(){
        let mut test_deck: Storage = Deck::new();
        test_deck.add(man::Man::new(0, vec![2,1,3,0], -1));
        test_deck.add(man::Man::new(1, vec![1,0,2,3], -1));
        test_deck.add(man::Man::new(2, vec![1,3,0,2], -1));
        test_deck.add(man::Man::new(3, vec![2,0,3,1], -1));
        let mut test_women: Vec<Woman> = vec![];
        test_women.push(woman::Woman::new(0,vec![0,1,3,2], man::Man::new(-1, generate_preference(0), -1)));
        test_women.push(woman::Woman::new(1,vec![2,0,3,1], man::Man::new(-1, generate_preference(0), -1)));
        test_women.push(woman::Woman::new(2,vec![2,1,3,0], man::Man::new(-1, generate_preference(0), -1)));
        test_women.push(woman::Woman::new(3,vec![1,0,2,3], man::Man::new(-1, generate_preference(0), -1)));
        marriage_stable(&mut test_deck, &mut test_women);
        let clone_women = test_women;
        let woman_a = clone_women[&0 as usize];
        match woman_a {
            None => {panic!()}
            Some(woman) => {assert_eq!(woman.favorite().name, 1)}
        }
        let woman_b = clone_women[&1];
        match woman_b {
            None => {panic!()}
            Some(woman) => {assert_eq!(woman.favorite().name, 2)}
        }
        let woman_c = clone_women[&2];
        match woman_c {
            None => {panic!()}
            Some(woman) => {assert_eq!(woman.favorite().name, 3)}
        }
        let woman_d = clone_women[&3];
        match woman_d {
            None => {panic!()}
            Some(woman) => {assert_eq!(woman.favorite().name, 0)}
        }
    }
}