#![allow(unused_imports)]
mod object;
mod test;
mod test_thread;

use crate::object::deck::*;
use crate::object::man::*;
use crate::object::result::*;
use crate::object::test_instances::*;
use crate::object::woman::*;
use crate::object::{man, result, test_instances, woman};
use rand::prelude::SliceRandom;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

fn main() {
    let size_of_list: i32 = 5;
    let mut deck: Storage<Man> = Deck::new();
    init_men(&mut deck, size_of_list);
    let women: Vec<Woman> = init_woman(size_of_list);
    let deck_parallel = deck.clone();
    let women_parallel = women.clone();
    let result_sequential = marriage_stable_sequential(deck, women);
    let result_parallel = marriage_stable_parallel(deck_parallel, women_parallel);
    print_couples(result_sequential.paired_women());
    print_couples(result_parallel.paired_women());
}

fn marriage_stable_sequential(mut deck: Storage<Man>, mut women: Vec<Woman>) -> Resultant {
    while let Some(mut man_proposing) = deck.pop() {
        if let Some(target) = man_proposing.find_next_woman() {
            let woman_being_proposed_to: &mut Woman = &mut women[(*target) as usize];
            if let Some(dropped_man) = woman_being_proposed_to.check_favorite(man_proposing) {
                deck.add(dropped_man);
            }
        }
    }
    result::Resultant::new(women)
}

fn marriage_stable_parallel(deck: Storage<Man>, women: Vec<Woman>) -> Resultant {
    let instance = init_instance(deck, women);
    let mut handles: Vec<JoinHandle<()>> = vec![];
    for _ in 0..2 {
        let instance = Arc::clone(&instance);
        let handle = thread::spawn(move || loop {
            let variable = instance.list_man.lock().unwrap().pop();
            match variable {
                None => break,
                Some(mut man) => {
                    if let Some(target) = man.find_next_woman() {
                        let mut woman_proposed_to =
                            instance.list_woman[(*target as usize)].lock().unwrap();
                        if let Some(dropped_man) = woman_proposed_to.check_favorite(man) {
                            instance.list_man.lock().unwrap().add(dropped_man);
                        }
                    }
                }
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    let instance = Arc::try_unwrap(instance).ok().unwrap();
    result::Resultant::new(mutex_women_to_women(instance.list_woman))
}

fn init_men(deck: &mut Storage<Man>, number: i32) {
    for i in 0..number {
        let man_holder = man::Man::new(i, generate_preference(number), -1);
        deck.add(man_holder);
    }
}

fn init_woman(number: i32) -> Vec<Woman> {
    let mut women = vec![];
    for i in 0..number {
        let woman_holder = woman::Woman::new(i, generate_preference(number), None);
        women.push(woman_holder);
    }
    return women;
}

fn init_instance(deck: Storage<Man>, women: Vec<Woman>) -> Arc<TestInstances> {
    let mut mutex_women: Vec<Mutex<Woman>> = vec![];
    for woman in women {
        mutex_women.push(Mutex::new(woman))
    }
    Arc::new(TestInstances::new(Mutex::new(deck), mutex_women))
}

fn mutex_women_to_women(mutex_women: Vec<Mutex<Woman>>) -> Vec<Woman> {
    let mut women = vec![];
    for mutex_woman in mutex_women {
        women.push(mutex_woman.into_inner().unwrap());
    }
    women
}

fn generate_preference(size: i32) -> Vec<i32> {
    let mut my_vector: Vec<i32> = (0..size).map(|x| x).collect();
    let mut rng = rand::thread_rng();
    my_vector.shuffle(&mut rng);
    my_vector
}

fn print_couples(women: &Vec<Woman>) {
    let mut i = 0;
    println!("|          | Woman | preference_woman | Man | preference_man  |");
    for woman in women {
        i += 1;
        let man_of_woman = woman.favorite();
        match man_of_woman {
            None => {
                println!("dafuq is happening")
            }
            Some(man) => {
                let woman_list = format!("{:?}", woman.preference);
                let man_list = format!("{:?}", man.preference);
                println!(
                    "{}",
                    format!(
                        "| couple {iterator} |   {woman}   | {preference_woman}  |  {man}  | {preference_man} |",
                        iterator = i,
                        woman = woman.name,
                        preference_woman = woman_list,
                        man = man.name,
                        preference_man = man_list
                    )
                );
                println!("---------------------------------------------------------------");
            }
        }
    }
}
