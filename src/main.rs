#![allow(unused_imports)]
mod object;
mod test;
mod test_thread;

use crate::object::algo::Algo;
use crate::object::algo::Algo::{Parallel, Sequential};
use crate::object::algorithm::{Algorithm, ParallelAlgorithm, SequentialAlgorithm};
use crate::object::deck::*;
use crate::object::man::*;
use crate::object::preference_generator::PreferenceGenerator;
use crate::object::result::*;
use crate::object::test_instances::*;
use crate::object::woman::*;
use crate::object::{algo, man, result, test_instances, woman};
use rand::prelude::SliceRandom;
use rand::rngs::StdRng;
use rand::SeedableRng;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::convert::TryInto;
use std::ops::Deref;
use std::ptr::null;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::time::Instant;

fn main() {
    let result_sequential = marriage_stable(Sequential);
    let result_parallel = marriage_stable(Parallel(16));
    println!(
        "{}",
        check_result(
            result_sequential.paired_women(),
            result_parallel.paired_women()
        )
    );
    print_algo_type_and_duration(&result_sequential);
    print_couples(result_sequential.paired_women());
    print_algo_type_and_duration(&result_parallel);
    print_couples(result_parallel.paired_women());
}

fn marriage_stable(algo: Algo) -> Resultant {
    let size_of_list: usize = 3;
    let mut random_generator = PreferenceGenerator::new(27);
    let mut deck: Storage<Man> = Deck::new();
    init_men(&mut deck, size_of_list, &mut random_generator);
    let women: Vec<Woman> = init_woman(size_of_list, &mut random_generator);
    solve(algo, deck, women)
}

fn solve(algo: Algo, men: Storage<Man>, women: Vec<Woman>) -> Resultant {
    match algo {
        Sequential => SequentialAlgorithm::new().resolve(men, women),
        Parallel(number_thread) => ParallelAlgorithm::new(number_thread).resolve(men, women),
    }
}

fn init_men(deck: &mut Storage<Man>, number: usize, random_generator: &mut PreferenceGenerator) {
    for i in 0..number {
        let man_holder = man::Man::new(i, random_generator.generate_preference(number), 0);
        deck.add(man_holder);
    }
}

fn init_woman(number: usize, random_generator: &mut PreferenceGenerator) -> Vec<Woman> {
    let mut women = vec![];
    for i in 0..number {
        let woman_holder = woman::Woman::new(i, random_generator.generate_preference(number), None);
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

fn print_algo_type_and_duration(result: &Resultant) {
    println!("The {:?} algorithm took {}", result.algo(), result.time());
}

fn print_couples(women: &Vec<Woman>) {
    let mut i = 0;
    println!("|          | Woman | preference_woman | Man | preference_man  |");
    for woman in women {
        i += 1;
        let man_of_woman = woman.favorite();
        match man_of_woman {
            None => {
                println!("{:?}", woman.preference);
                println!("daf is happening")
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

fn check_result(women_sequential: &Vec<Woman>, women_parallel: &Vec<Woman>) -> bool {
    for i in 0..women_sequential.len() {
        if women_sequential[i].name != women_parallel[i].name {
            return false;
        }
    }
    true
}
