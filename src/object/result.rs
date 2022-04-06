use crate::object::algo::Algo;
use crate::Woman;

pub(crate) struct Resultant {
    paired_women: Vec<Woman>,
    algo: Algo,
    time: u128
}

impl Resultant {
    pub(crate) fn new(women: Vec<Woman>, algo: Algo , time_elapsed: u128) -> Resultant {
        Resultant {
            paired_women: women,
            algo,
            time: time_elapsed,
        }
    }

    pub(crate) fn paired_women(&self) -> &Vec<Woman> {
        &self.paired_women
    }
}
