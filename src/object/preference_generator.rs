use rand::prelude::SliceRandom;
use rand::rngs::StdRng;
use rand::SeedableRng;

pub(crate) struct PreferenceGenerator {
    rng: StdRng,
}

impl PreferenceGenerator {
    pub(crate) fn new(number: u64) -> PreferenceGenerator {
        PreferenceGenerator {
            rng: StdRng::seed_from_u64(number),
        }
    }
}

impl PreferenceGenerator {
    pub(crate) fn generate_preference(&mut self, size: usize) -> Vec<usize> {
        let mut my_vector: Vec<usize> = (0..size).collect();
        my_vector.shuffle(&mut self.rng);
        my_vector
    }
}
