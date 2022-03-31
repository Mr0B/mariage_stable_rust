#[cfg(test)]
mod tests {
    use crate::object::deck::{Deck, Storage};
    use crate::object::woman::Woman;
    use crate::object::{man, woman};
    use crate::{generate_preference, marriage_stable_parallel, marriage_stable_sequential, Man};
    use std::collections::HashMap;

    #[test]
    fn test_marriage_stable() {
        let mut test_deck: Storage<Man> = Deck::new();
        test_deck.add(man::Man::new(0, vec![2, 1, 3, 0], -1));
        test_deck.add(man::Man::new(1, vec![1, 0, 2, 3], -1));
        test_deck.add(man::Man::new(2, vec![1, 3, 0, 2], -1));
        test_deck.add(man::Man::new(3, vec![2, 0, 3, 1], -1));
        let mut test_women: Vec<Woman> = vec![];
        test_women.push(woman::Woman::new(0, vec![0, 1, 3, 2], None));
        test_women.push(woman::Woman::new(1, vec![2, 0, 3, 1], None));
        test_women.push(woman::Woman::new(2, vec![2, 1, 3, 0], None));
        test_women.push(woman::Woman::new(3, vec![1, 0, 2, 3], None));
        let test_deck_2 = test_deck.clone();
        let test_women_2 = test_women.clone();
        let result_sequential = marriage_stable_sequential(test_deck, test_women);
        let result_parallel = marriage_stable_parallel(test_deck_2, test_women_2);
        match result_sequential.paired_women()[0].favorite() {
            None => {}
            Some(man) => {
                assert_eq!(man.name, 1)
            }
        }
        match result_parallel.paired_women()[0].favorite() {
            None => {}
            Some(man) => {
                assert_eq!(man.name, 1)
            }
        }
        match result_sequential.paired_women()[1].favorite() {
            None => {}
            Some(man) => {
                assert_eq!(man.name, 2)
            }
        }
        match result_parallel.paired_women()[1].favorite() {
            None => {}
            Some(man) => {
                assert_eq!(man.name, 2)
            }
        }
        match result_sequential.paired_women()[2].favorite() {
            None => {}
            Some(man) => {
                assert_eq!(man.name, 3)
            }
        }
        match result_parallel.paired_women()[2].favorite() {
            None => {}
            Some(man) => {
                assert_eq!(man.name, 3)
            }
        }
        match result_sequential.paired_women()[3].favorite() {
            None => {}
            Some(man) => {
                assert_eq!(man.name, 0)
            }
        }
        match result_parallel.paired_women()[3].favorite() {
            None => {}
            Some(man) => {
                assert_eq!(man.name, 0)
            }
        }
    }
}
