use std::sync::mpsc;
use std::thread;
use std::time::Duration;

#[allow(dead_code)]
pub(crate) fn canal() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![0, 2, 5, 9];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
