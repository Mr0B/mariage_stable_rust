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
    #[clap(long)]
    instance_size_start: usize,
    #[clap(long)]
    instance_size_end: usize,
    #[clap(short, long)]
    pas: usize,
    #[clap(short, long, default_value_t = 2)]
    thread_number: i32,
    #[clap(short, long)]
    nombre_repetition: u128,
}

fn main() {
    let args = Args::parse();
    let mut seed = args.seed;
    let size_instance_start = args.instance_size_start;
    let size_instance_end = args.instance_size_end;
    let pas = args.pas;
    let thread_number = args.thread_number;
    let nombre_repetition = args.nombre_repetition;
    for i in (size_instance_start..(size_instance_end + pas)).step_by(pas) {
        for _ in 0..nombre_repetition {
            if seed == 0 {
                let mut rng = rand::thread_rng();
                seed = rng.gen()
            }
            let result_sequential = marriage_stable(Sequential, i, seed);
            let result_parallel = marriage_stable(Parallel(thread_number), i, seed);
            log_result(&result_sequential).expect("");
            log_result(&result_parallel).expect("");
        }
    }
}

fn marriage_stable(algo: Algo, size_instance: usize, seed: u64) -> Resultant {
    let mut random_generator = PreferenceGenerator::new(seed);
    let mut deck: Storage<Man> = Deck::new();
    init_men(&mut deck, size_instance, &mut random_generator);
    let women: Vec<Woman> = init_woman(size_instance, &mut random_generator);
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

fn check_result(women_sequential: &Vec<Woman>, women_parallel: &Vec<Woman>) -> bool {
    for i in 0..women_sequential.len() {
        if women_sequential[i].name != women_parallel[i].name {
            return false;
        }
    }
    true
}
