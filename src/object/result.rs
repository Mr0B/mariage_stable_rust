use crate::object::algo::Algo;
use crate::Woman;

pub(crate) struct Resultant {
    paired_women: Vec<Woman>,
    algo: Algo,
    time_elapsed: u128,
}

impl Resultant {
    pub(crate) fn new(paired_women: Vec<Woman>, algo: Algo, time_elapsed: u128) -> Resultant {
        Resultant {
            paired_women,
            algo,
            time_elapsed,
        }
    }

    pub(crate) fn paired_women(&self) -> &Vec<Woman> {
        &self.paired_women
    }

    pub(crate) fn algo(&self) -> &Algo {
        &self.algo
    }

    pub(crate) fn time(&self) -> &u128 {
        &self.time_elapsed
    }
}
