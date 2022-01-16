mod object;
mod test;
mod test_thread;

use crate::object::deck::*;
use crate::object::woman::*;
use crate::object::{man, woman};
use rand::prelude::SliceRandom;
use std::collections::HashMap;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

fn main() {
    let mut test_deck: Storage = Deck::new();
    init_men(&mut test_deck, 5);
    let mut test_women: Vec<Woman> = init_woman(5);
    marriage_stable(&mut test_deck, &mut test_women);
    print_couples(test_women);
}

fn marriage_stable(deck: &mut Storage, women: &mut Vec<Woman>) {
    while let Some(mut man_proposing) = deck.pop() {
        if let Some(target) = man_proposing.find_next_woman() {
            let woman_being_proposed_to: &mut Woman = &mut women[(*target) as usize];
            if let Some(dropped_man) = woman_being_proposed_to.check_favorite(man_proposing) {
                if dropped_man.name != -1 {
                    deck.add(dropped_man);
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

fn init_woman(number: i32) -> Vec<Woman> {
    let mut women = vec![];
    for i in 0..number {
        let woman_holder = woman::Woman::new(
            i,
            generate_preference(number),
            man::Man::new(-1, generate_preference(0), -1),
        );
        women.push(woman_holder);
    }
    return women;
}

fn generate_preference(size: i32) -> Vec<i32> {
    let mut my_vector: Vec<i32> = (0..size).map(|x| x).collect();
    let mut rng = rand::thread_rng();
    my_vector.shuffle(&mut rng);
    my_vector
}

//fn find_rank_in_preference_list_of_couple(woman: Woman) -> (i32, i32) {
//    let x: i32 = *woman.favorite().proposing_to();
//    let mut y: i32 = 0;
//    for element in woman.preference {
//        y+=1;
//        if element==woman.favorite().name {
//            break
//        }
//    }
//    return (x,y)
//}

fn print_couples(women: Vec<Woman>) {
    for woman in women {
        println!(
            "{}",
            format!(
                "woman {woman} is paired with man {man}",
                woman = woman.name,
                man = woman.favorite().name
            )
        )
    }
}
