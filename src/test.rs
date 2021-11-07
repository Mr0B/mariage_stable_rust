#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::{generate_preference, mariage_stable};
    use crate::object::{deck, man, woman};
    use crate::object::deck::Deck;
    use crate::object::woman::Woman;

    #[test]
    fn test_mariage_stable(){
        let mut test_deck: Deck = deck::Deck::new(vec![]);
        test_deck.put_at_the_end(man::Man::new(0, vec![2,1,3,0], 0));
        test_deck.put_at_the_end(man::Man::new(1, vec![1,0,2,3], 0));
        test_deck.put_at_the_end(man::Man::new(2, vec![1,3,0,2], 0));
        test_deck.put_at_the_end(man::Man::new(3, vec![2,0,3,1], 0));
        let mut test_women: HashMap<i32, Woman> = HashMap::new();
        test_women.insert(0, woman::Woman::new(0,vec![0,1,3,2], man::Man::new(-1, generate_preference(0), -1)));
        test_women.insert(1, woman::Woman::new(1,vec![2,0,3,1], man::Man::new(-1, generate_preference(0), -1)));
        test_women.insert(2, woman::Woman::new(2,vec![2,1,3,0], man::Man::new(-1, generate_preference(0), -1)));
        test_women.insert(3, woman::Woman::new(3,vec![1,0,2,3], man::Man::new(-1, generate_preference(0), -1)));
        mariage_stable(&mut test_deck, &mut test_women);
        let clone_women = test_women;
        let woman_a = clone_women.get(&0);
        match woman_a {
            None => {panic!()}
            Some(woman) => {assert_eq!(woman.favorite().name, 1)}
        }
        let woman_b = clone_women.get(&1);
        match woman_b {
            None => {panic!()}
            Some(woman) => {assert_eq!(woman.favorite().name, 2)}
        }
        let woman_c = clone_women.get(&2);
        match woman_c {
            None => {panic!()}
            Some(woman) => {assert_eq!(woman.favorite().name, 3)}
        }
        let woman_d = clone_women.get(&3);
        match woman_d {
            None => {panic!()}
            Some(woman) => {assert_eq!(woman.favorite().name, 0)}
        }
    }
}