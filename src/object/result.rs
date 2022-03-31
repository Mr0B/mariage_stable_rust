use crate::Woman;

pub(crate) struct Resultant {
    paired_women: Vec<Woman>,
}

impl Resultant {
    pub(crate) fn new(women: Vec<Woman>) -> Resultant {
        Resultant {
            paired_women: women,
        }
    }

    pub(crate) fn paired_women(&self) -> &Vec<Woman> {
        &self.paired_women
    }
}
