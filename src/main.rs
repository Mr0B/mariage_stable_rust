mod object;

use rand::prelude::SliceRandom;
use crate::object::{deck, man, woman};
use crate::object::deck::*;
use crate::object::woman::*;

fn main() {
    let mut test_deck: Deck = deck::Deck::new(vec![]);
    let mut test_women: Vec<Woman> = Vec::new();

    init_men(&mut test_deck, 5);
    init_woman(&mut test_women, 5);
    for i in 0..5 {
        let holder_man =test_deck.give_first();
        match holder_man{
            Some(man) => println!("{}{:?}{}",man.0, man.1, man.2),
            None => println!("didn't work"),
        }
        let holder_woman=test_women.get(i);
        match holder_woman{
            Some(woman) => println!("{}{:?}",woman.0, woman.1),
            None => println!("didn't work"),
        }
    }
}

fn init_men(deck: &mut Deck, number:i32) {
    for i in 0..number {
        let man_holder = man::Man::new(i, generate_preference(number), -1);
        deck.put_at_the_end(man_holder);
    }
}

fn init_woman(women: &mut Vec<Woman>, number: i32 ){
    for i in 0..number {
        let woman_holder=woman::Woman::new(i,generate_preference(number), -1);
        women.push(woman_holder);
    }
}

fn generate_preference(size: i32) -> Vec<i32> {
    let mut my_vector : Vec<i32> = (0..size+1).map(|x| x).collect();
    let mut rng = rand::thread_rng();
    my_vector.shuffle(&mut rng);
    my_vector
}