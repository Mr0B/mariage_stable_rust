mod object;

use crate::object::{deck, man, woman};
use crate::object::deck::*;
use crate::object::man::*;
use crate::object::woman::*;

fn main() {
    let mut _test_deck: Deck = deck::Deck::new(vec![]);
    let mut test_woman: Woman = woman::Woman::new(2, vec![1, 0, 2], -1);

    println!("{}", test_woman.0)
}

fn init_men(mut deck:Deck, number:i32) {
    for i in 0..number {
        let man_holder = man::Man::new(i, generate_man_preference(number, i), -1);
        deck.put_at_the_end(man_holder);
    }
}

fn generate_preference(size: i32, preference_of: i32) -> Vec<i32> {
    let mut my_vector : Vec<i32> = (1..size+1).map(|x| x).collect();
    my_vector.retain(|x| *x != preference_of);
    rand::thread_rng().shuffle(my_vector)
}