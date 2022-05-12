#![allow(unused_imports)]
#![allow(dead_code)]
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
use clap::Parser;
use clap::*;
use rand::prelude::SliceRandom;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::borrow::Borrow;
use std::collections::HashMap;
use std::convert::TryInto;
use std::env::args;
use std::fs::{File, OpenOptions};
use std::io::{Error, Write};
use std::ops::Deref;
use std::ptr::{null, write};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::time::Instant;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub(crate) struct Args {
    #[clap(short, long, default_value_t = 0)]
    seed: u64,
    #[clap(long, default_value_t = 5)]
    instance_size_start: usize,
    #[clap(long, default_value_t = 5)]
    instance_size_end: usize,
    #[clap(short, long, default_value_t = 1)]
    pas: usize,
    #[clap(short, long, default_value_t = 4)]
    thread_number: i32,
    #[clap(short, long, default_value_t = 1)]
    number_repetition: u128,
    #[clap(short, long)]
    worst_case: bool,
}

fn main() {
    let args = Args::parse();
    let mut seed = args.seed;
    let size_instance_start = args.instance_size_start;
    let size_instance_end = args.instance_size_end;
    let pas = args.pas;
    let thread_number = args.thread_number;
    let number_repetition = args.number_repetition;
    let worst_case = args.worst_case;
    for i in (size_instance_start..=size_instance_end).step_by(pas) {
        for _ in 0..number_repetition {
            if seed == 0 {
                let mut rng = rand::thread_rng();
                seed = rng.gen()
            }
            let instance = if worst_case {
                generate_worst_case(i)
            } else {
                marriage_stable(i, seed)
            };
            let (men2, women2) = instance.clone();
            let result_sequential = solve(Sequential, instance.0, instance.1);
            let result_parallel = solve(Parallel(thread_number), men2, women2);
            if check_stability(&result_sequential.paired_women())
                && check_result(
                    &result_sequential.paired_women(),
                    &result_parallel.paired_women(),
                )
            {
                //log_result(&result_sequential).expect("");
                //log_result(&result_parallel).expect("");
                print_couples(result_sequential.paired_women());
                print_couples(result_parallel.paired_women());
            }
        }
    }
}

fn marriage_stable(size_instance: usize, seed: u64) -> (Storage<Man>, Vec<Woman>) {
    let mut random_generator = PreferenceGenerator::new(seed);
    let mut deck: Storage<Man> = Deck::new();
    init_men(&mut deck, size_instance, &mut random_generator);
    let women: Vec<Woman> = init_woman(size_instance, &mut random_generator);
    return (deck, women);
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

fn generate_worst_case(size: usize) -> (Storage<Man>, Vec<Woman>) {
    let mut deck: Storage<Man> = Deck::new();
    let pref_men: Vec<usize> = (0..size).collect();
    (0..size)
        .map(|i| Man::new(i, pref_men.clone(), 0))
        .for_each(|m| deck.add(m));
    let pref_women: Vec<usize> = (0..size).rev().collect();
    let women: Vec<Woman> = (0..size)
        .map(|i| Woman::new(i, pref_women.clone(), None))
        .collect();
    (deck, women)
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

fn log_result(result: &Resultant) -> std::io::Result<()> {
    let buffer = format!(
        "{}/{:?}/{}\n",
        result.paired_women().len(),
        result.algo(),
        result.time()
    );
    let path = "graph_generation/log.txt";
    if std::path::Path::new(path).exists() {
        let mut output = OpenOptions::new().append(true).open(path)?;
        output.write_all(buffer.as_bytes())
    } else {
        let mut output = File::create(path)?;
        output.write_all(buffer.as_bytes())
    }
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

fn check_stability(women: &Vec<Woman>) -> bool {
    for woman1 in women {
        if let Some(man1) = woman1.favorite() {
            let position1 = woman1.position_woman_preference(man1);
            for woman2 in women {
                if let Some(man2) = woman2.favorite() {
                    let position2 = woman1.position_woman_preference(man2);
                    if (position1 > position2)
                        && (man2.position_man_preference(woman2)
                            > man2.position_man_preference(woman1))
                    {
                        return false;
                    }
                }
            }
        }
    }
    return true;
}

fn check_result(women_sequential: &Vec<Woman>, women_parallel: &Vec<Woman>) -> bool {
    for i in 0..women_sequential.len() {
        if women_sequential[i].name != women_parallel[i].name {
            return false;
        }
    }
    true
}
