use crate::{
    init_instance, mutex_women_to_women, Deck, Man, Parallel, Resultant, Sequential, Storage, Woman,
};
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;
use std::time::Instant;

pub(crate) struct SequentialAlgorithm {}

pub(crate) struct ParallelAlgorithm {
    number_thread: i32,
}

pub(crate) trait Algorithm {
    fn resolve(&self, _: Storage<Man>, _: Vec<Woman>) -> Resultant;
}

impl SequentialAlgorithm {
    pub(crate) fn new() -> Self {
        SequentialAlgorithm {}
    }
}

impl Algorithm for SequentialAlgorithm {
    fn resolve(&self, mut deck: Storage<Man>, mut women: Vec<Woman>) -> Resultant {
        let now = Instant::now();
        while let Some(mut man_proposing) = deck.pop() {
            if let Some(target) = man_proposing.find_next_woman() {
                let woman_being_proposed_to: &mut Woman = &mut women[(*target)];
                if let Some(dropped_man) = woman_being_proposed_to.check_favorite(man_proposing) {
                    deck.add(dropped_man);
                }
            }
        }
        let elapsed_time = now.elapsed();
        Resultant::new(women, Sequential, elapsed_time.as_micros())
    }
}

impl ParallelAlgorithm {
    pub(crate) fn new(number_thread: i32) -> Self {
        ParallelAlgorithm { number_thread }
    }
}

impl Algorithm for ParallelAlgorithm {
    fn resolve(&self, deck: Storage<Man>, women: Vec<Woman>) -> Resultant {
        let instance = init_instance(deck, women);
        let mut handles: Vec<JoinHandle<()>> = vec![];
        let now = Instant::now();
        for _ in 0..self.number_thread {
            let instance = Arc::clone(&instance);
            let handle = thread::spawn(move || loop {
                let variable = instance.list_man.lock().unwrap().pop();
                match variable {
                    None => {
                        break;
                    }
                    Some(mut man) => {
                        if let Some(target) = man.find_next_woman() {
                            let mut woman_proposed_to =
                                instance.list_woman[(*target)].lock().unwrap();
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
        let elapsed_time = now.elapsed();
        let instance = Arc::try_unwrap(instance).ok().unwrap();
        Resultant::new(
            mutex_women_to_women(instance.list_woman),
            Parallel(self.number_thread),
            elapsed_time.as_micros(),
        )
    }
}
