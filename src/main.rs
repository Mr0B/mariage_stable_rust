use crate::test_object::{Man, Woman};

mod test_object;

fn main() {
    let mut test_man = Man{
        0: 0,
        1: vec![2, 0, 1],
        2: -1
    };
    let mut _test_woman = Woman{
        0: 2,
        1: vec![1, 0, 2],
        2: -1
    };
    println!("{}", test_man.0);
    println!("{}", test_man.2);
    test_man.2=2;
    println!("{}", test_man.2);
}
