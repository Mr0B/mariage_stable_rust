use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;
use std::time::Instant;
use crate::{Resultant, Storage, Man, Woman, Deck, Sequential, init_instance, mutex_women_to_women, Parallel};

pub(crate) enum Algo {
    Sequential,
    Parallel(i32),
}

pub(crate) struct SequentialAlgorithm {

}

pub(crate) struct ParallelAlgorithm {
    number_thread: i32
}

pub(crate) trait Algorithm {

    fn new() -> Box<Algorithm>;

    fn resolve(&self, _: Storage<Man>, _: Vec<Woman>) -> Resultant;
}

impl Algorithm for SequentialAlgorithm {
    fn new() -> Box<Algorithm> {
        SequentialAlgorithm {}
    }

    fn resolve(&self, mut deck: Storage<Man>, mut women: Vec<Woman>) -> Resultant {
        let now = Instant::now();
        while let Some(mut man_proposing) = deck.pop() {
            if let Some(target) = man_proposing.find_next_woman() {
                let woman_being_proposed_to: &mut Woman = &mut women[(*target) - 1];
                if let Some(dropped_man) = woman_being_proposed_to.check_favorite(man_proposing) {
                    deck.add(dropped_man);
                }
            }
        }
        let elapsed_time=now.elapsed();
        Resultant::new(women, Sequential, elapsed_time.as_micros())
    }
}

impl Algorithm for ParallelAlgorithm {
    fn resolve(&self, deck: Storage<Man>, women: Vec<Woman>) -> Resultant {
        let instance = init_instance(deck, women);
        let mut handles: Vec<JoinHandle<()>> = vec![];
        let now = Instant::now();
        for _ in 0..self.number_thread {
            let instance = Arc::clone(&instance);
            let handle = thread::spawn(move || {
                loop {
                    let variable = instance.list_man.lock().unwrap().pop();
                    match variable {
                        None => {
                            break;
                        }
                        Some(mut man) => {
                            if let Some(target) = man.find_next_woman() {
                                let mut woman_proposed_to =
                                    instance.list_woman[(*target) - 1].lock().unwrap();
                                if let Some(dropped_man) = woman_proposed_to.check_favorite(man) {
                                    instance.list_man.lock().unwrap().add(dropped_man);
                                }
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
        let elapsed_time=now.elapsed();
        let instance = Arc::try_unwrap(instance).ok().unwrap();
        Resultant::new(mutex_women_to_women(instance.list_woman), Parallel, elapsed_time.as_micros())
    }
}