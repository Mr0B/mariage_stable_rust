use std::sync::{Arc, Mutex};
use std::sync::mpsc::channel;
use std::thread;

#[allow(dead_code)]
pub(crate) fn parallel_increment(number_thread: i32, number_increment: i32) {
    let increment = Arc::new(Mutex::new(0));
    let (tx, rx) = channel();
    for _i in 0..number_thread {
        let (increment, tx) = (Arc::clone(&increment), tx.clone());
        let handle= thread::spawn(move || {
            let mut increment = increment.lock().unwrap();
            *increment += number_increment;
            if *increment == number_thread*number_increment {
                tx.send(*increment).unwrap();
            }
        });
        handle.join().unwrap();
    }
    let received = rx.recv().unwrap();
    println!("{:?}", received);
}