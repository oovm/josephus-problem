use rand::{prelude::SmallRng, Rng, SeedableRng};

#[derive(Copy, Clone, Debug)]
pub struct RandomKill {
    kill_rate: f32,
    partners: usize,
    seed: u64,
}

#[derive(Copy, Clone, Debug)]
pub struct KillMan {
    pub id: usize,
    pub steps: usize,
}

impl RandomKill {
    pub fn new(partners: usize, kill_rate: f32) -> Self {
        RandomKill { kill_rate, partners, seed: rand::random() }
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
    living: Vec<usize>,
    kill_rate: f32,
    killer_index: usize,
    killer_steps: usize,
}

impl IntoIterator for RandomKill {
    type Item = KillMan;
    type IntoIter = RandomKillState;

    fn into_iter(self) -> Self::IntoIter {
        let rng = SmallRng::seed_from_u64(self.seed);
        let mut partners = Vec::with_capacity(self.partners);
        for i in 0..self.partners {
            partners.push(i);
        }
        RandomKillState { rng, kill_rate: self.kill_rate, living: partners, killer_index: 0, killer_steps: 0 }
    }
}

impl Iterator for RandomKillState {
    type Item = KillMan;

    fn next(&mut self) -> Option<Self::Item> {
        if self.living.is_empty() {
            return None;
        }
        self.killer_steps += 1;
        if self.rng.gen_bool(self.kill_rate as f64) {
            let victim = self.living.remove(self.killer_index);
            match self.living.len() {
                // next round return None
                0 => {}
                _ => {
                    self.killer_index %= self.living.len();
                }
            }
            Some(KillMan { id: victim, steps: self.killer_steps.saturating_sub(1) })
        }
        else {
            self.killer_index = (self.killer_index + 1) % self.living.len();
            self.next()
        }
    }
}
