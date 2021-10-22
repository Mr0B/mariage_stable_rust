mod object;

use rand::prelude::SliceRandom;
use crate::object::{deck, man, woman};
use crate::object::deck::*;
use crate::object::man::*;
use crate::object::woman::*;
use rand::Rng;

fn main() {
    let mut test_deck: Deck = deck::Deck::new(vec![]);
    let mut test_woman: Woman = woman::Woman::new(2, vec![1, 0, 2], -1);

    init_men(&mut test_deck, 5);
    for i in 0..5 {
        let holder =test_deck.give_first();
        match holder{
            Some(man) => println!("{}{:?}{}",man.0, man.1, man.2),
            None => println!("didn't work"),
        }
    }
}

fn init_men(deck: &mut Deck, number:i32) {
    for i in 0..number {
        let man_holder = man::Man::new(i, generate_preference(number, i), -1);
        deck.put_at_the_end(man_holder);
    }
}

fn generate_preference(size: i32, preference_of: i32) -> Vec<i32> {
    let mut my_vector : Vec<i32> = (0..size+1).map(|x| x).collect();
    let mut rng = rand::thread_rng();
    my_vector.shuffle(&mut rng);
    my_vector
}