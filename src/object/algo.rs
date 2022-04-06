#[derive(Debug)]
pub(crate) enum Algo {
    Sequential,
    Parallel(i32),
}