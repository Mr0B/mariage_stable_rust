use crate::test_object::{Man, Woman};

mod test_object;

fn main() {
    let test_man: Man = test_object::Man::new(0, vec![2, 0, 1],-1);
    let test_man2: Man = test_object::Man::new(1, vec![0, 2, 1],-1);
    let mut test_woman: Woman = test_object::Woman::new(2, vec![1, 0, 2], -1);
    println!("{}", test_woman.2);
    let _dumped: i32 = test_woman.check_favorite(test_man);
    println!("{}", test_woman.2);
    let _dumped2: i32 = test_woman.check_favorite(test_man2);
    println!("{}", test_woman.2);
}
