#[cfg(test)]
mod tests {
    use crate::object::deck::{Deck, Storage};
    use crate::object::woman::Woman;
    use crate::object::{man, woman};
    use crate::{generate_preference, Man, marriage_stable_sequential};
    use std::collections::HashMap;

    #[test]
    fn test_marriage_stable() {
        let mut test_deck: Storage<Man> = Deck::new();
        test_deck.add(man::Man::new(0, vec![2, 1, 3, 0], -1));
        test_deck.add(man::Man::new(1, vec![1, 0, 2, 3], -1));
        test_deck.add(man::Man::new(2, vec![1, 3, 0, 2], -1));
        test_deck.add(man::Man::new(3, vec![2, 0, 3, 1], -1));
        let mut test_women: Vec<Woman> = vec![];
        test_women.push(woman::Woman::new(
            0,
            vec![0, 1, 3, 2],
            None,
        ));
        test_women.push(woman::Woman::new(
            1,
            vec![2, 0, 3, 1],
            None,
        ));
        test_women.push(woman::Woman::new(
            2,
            vec![2, 1, 3, 0],
            None,
        ));
        test_women.push(woman::Woman::new(
            3,
            vec![1, 0, 2, 3],
            None,
        ));
        marriage_stable_sequential(&mut test_deck, &mut test_women);
        let clone_women = test_women;
        let woman_a = &clone_women[0];
        match woman_a.favorite() {
            None => {println!("Errare Humanum Est")}
            Some(man_a) => {assert_eq!(man_a.name, 1)}
        }
        let woman_b = &clone_women[1];
        match woman_b.favorite() {
            None => {println!("Errare Humanum Est")}
            Some(man_b) => {assert_eq!(man_b.name, 2)}
        }
        let woman_c = &clone_women[2];
        match woman_c.favorite() {
            None => {println!("Errare Humanum Est")}
            Some(man_c) => {assert_eq!(man_c.name, 3)}
        }
        let woman_d = &clone_women[3];
        match woman_d.favorite() {
            None => {println!("Errare Humanum Est")}
            Some(man_d) => {assert_eq!(man_d.name, 0)}
        }
    }
}
