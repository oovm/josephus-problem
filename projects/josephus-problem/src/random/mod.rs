use rand::prelude::SmallRng;
use rand::Rng;

pub struct RandomKill {
    kill_rate: f64,
    partners: usize,
    seed: u64,
}

struct State {
    rng: SmallRng,
    kill_rate: f64,
    partners: Vec<usize>,
    killer_index: usize,
    killer_steps: usize,
}

impl IntoIterator for RandomKill {
    type Item = usize;
    type IntoIter = State;

    fn into_iter(self) -> Self::IntoIter {
        todo!()
    }
}

impl Iterator for State {
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