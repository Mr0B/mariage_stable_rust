mod object;

use std::collections::HashMap;
use rand::prelude::SliceRandom;
use crate::object::{deck, man, woman};
use crate::object::deck::*;
use crate::object::woman::*;

fn main() {
    let mut test_deck: Deck = deck::Deck::new(vec![]);
    init_men(&mut test_deck, 5);
    let mut test_women: HashMap<i32, Woman> = init_woman(5);
    while !(test_deck.empty()) {
        let man_proposing=test_deck.give_first();
        match man_proposing {
            None => {println!("Something went wrong")}
            Some(man) => {
                let index:usize= *man.proposing_to() as usize;
                let target_list:Vec<i32> = man.preference().clone();
                let target: Option<&i32> = target_list.get(index);
                match target {
                    None => {println!("Something went wrong")}
                    Some(target) => {
                        let woman_being_proposed_to: Option<&mut Woman> =test_women.get_mut(target);
                        match woman_being_proposed_to {
                            None => {println!("Something went wrong")}
                            Some(woman) => {
                                let mut dropped_man = woman.check_favorite(man);
                                *dropped_man.proposing_to_mutable()=*dropped_man.proposing_to()+1;
                                if dropped_man.name !=-1 {
                                    test_deck.put_at_the_end(dropped_man);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    for i in 0..test_women.len() {
        let temporary = test_women.get(&(i as i32));
        match temporary {
            None => {}
            Some(woman) => {
                println!("Man");
                println!("{},{:?},{}", woman.favorite().name, woman.favorite().preference(), woman.favorite().proposing_to());
                println!("Woman");
                println!("{},{:?},{}", woman.name, woman.preference, woman.favorite().name);
            }
        }
    }
}

fn init_men(deck: &mut Deck, number:i32) {
    for i in 0..number {
        let man_holder = man::Man::new(i, generate_preference(number), 0);
        deck.put_at_the_end(man_holder);
    }
}

fn init_woman(number: i32 ) -> HashMap<i32, Woman> {
    let mut women = HashMap::new();
    for i in 0..number {
        let woman_holder=woman::Woman::new(i,generate_preference(number), man::Man::new(-1, generate_preference(0), -1));
        women.insert(i, woman_holder);
    }
    return women;
}

fn generate_preference(size: i32) -> Vec<i32> {
    let mut my_vector : Vec<i32> = (0..size).map(|x| x).collect();
    let mut rng = rand::thread_rng();
    my_vector.shuffle(&mut rng);
    my_vector
}