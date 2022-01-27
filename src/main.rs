#![allow(unused_imports)]
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
    let size_of_list: i32 = 5;
    let mut test_deck: Storage = Deck::new();
    init_men(&mut test_deck, size_of_list);
    let mut test_women: Vec<Woman> = init_woman(size_of_list);
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

fn print_couples(women: Vec<Woman>) {
    let mut i = 0;
    println!("|          | Woman | preference_woman | Man | preference_man  |");
    for woman in women {
        i += 1;
        let woman_list = format!("{:?}", woman.preference);
        let man_list = format!("{:?}", woman.favorite().preference);
        println!(
            "{}",
            format!(
                "| couple {iterator} |   {woman}   | {preference_woman}  |  {man}  | {preference_man} |",
                iterator = i,
                woman = woman.name,
                preference_woman = woman_list,
                man = woman.favorite().name,
                preference_man = man_list
            )
        );
        println!("---------------------------------------------------------------");
    }
}
