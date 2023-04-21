use rand::prelude::SmallRng;
use rand::{Rng, SeedableRng};

#[derive(Copy, Clone, Debug)]
pub struct RandomKill {
    kill_rate: f64,
    partners: usize,
    seed: u64,
}

impl RandomKill {
    pub fn new(partners: usize, kill_rate: f64) -> Self {
        RandomKill {
            kill_rate,
            partners,
            seed: rand::random(),
        }
    }
    pub fn set_seed(&mut self, seed: u64) {
        self.seed = seed;
    }
    pub fn with_seed(mut self, seed: u64) -> Self {
        self.set_seed(seed);
        self
    }
}

pub struct RandomKillState {
    rng: SmallRng,
    kill_rate: f64,
    partners: Vec<usize>,
    killer_index: usize,
    killer_steps: usize,
}

impl IntoIterator for RandomKill {
    type Item = usize;
    type IntoIter = RandomKillState;

    fn into_iter(self) -> Self::IntoIter {
        let mut rng = SmallRng::seed_from_u64(self.seed);
        let mut partners = Vec::with_capacity(self.partners);
        for i in 0..self.partners {
            partners.push(i);
        }
        RandomKillState {
            rng,
            kill_rate: self.kill_rate,
            partners,
            killer_index: 0,
            killer_steps: 0,
        }
    }
}

impl RandomKillState {
    pub fn survivor(&self) -> usize {
        assert_eq!(self.partners.len(), 1, "survivor() called before iteration finished");
        unsafe {
            *self.partners.get_unchecked(0)
        }
    }
    pub fn killer_steps(&self) -> usize {
        self.killer_steps
    }
}

impl Iterator for RandomKillState {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.partners.len() == 1 {
            return None;
        }
        self.killer_steps += 1;
        if self.rng.gen_bool(self.kill_rate) {
            let victim_index = (self.killer_index + 1) % self.partners.len();
            let victim = self.partners.remove(victim_index);
            if victim_index < self.killer_index {
                self.killer_index -= 1;
            }
            self.killer_steps = 0;
            Some(victim)
        } else {
            self.killer_index = (self.killer_index + 1) % self.partners.len();
            self.next()
        }
    }
}