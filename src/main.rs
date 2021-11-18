mod object;
mod test;

use std::collections::HashMap;
use rand::prelude::SliceRandom;
use crate::object::{man, woman};
use crate::object::deck::*;
use crate::object::woman::*;

fn main() {
    let mut test_deck: Storage = Deck::new();
    init_men(&mut test_deck, 5);
    let mut test_women: HashMap<i32, Woman> = init_woman(5);
    marriage_stable(&mut test_deck, &mut test_women);
}

fn marriage_stable(deck: &mut Storage, women: &mut HashMap<i32, Woman>) {
    while let Some(mut man_proposing) = deck.pop() {
        if let Some(target) = man_proposing.find_next_woman() {
            let woman_being_proposed_to: Option<&mut Woman> = women.get_mut(target);
            match woman_being_proposed_to {
                None => { panic!("Something went wrong!") }
                Some(woman) => {
                    if let Some(dropped_man) = woman.check_favorite(man_proposing) {
                        if dropped_man.name != -1 {
                            deck.add(dropped_man);
                        }
                    }
                }
            }
        }
    }
}

fn init_men(deck: &mut Storage, number: i32) {
    for i in 0..number {
        let man_holder = man::Man::new(i, generate_preference(number), -1);
        deck.add(man_holder);
    }
}

fn init_woman(number: i32) -> HashMap<i32, Woman> {
    let mut women = HashMap::new();
    for i in 0..number {
        let woman_holder = woman::Woman::new(i, generate_preference(number), man::Man::new(-1, generate_preference(0), -1));
        women.insert(i, woman_holder);
    }
    return women;
}

fn generate_preference(size: i32) -> Vec<i32> {
    let mut my_vector: Vec<i32> = (0..size).map(|x| x).collect();
    let mut rng = rand::thread_rng();
    my_vector.shuffle(&mut rng);
    my_vector
}