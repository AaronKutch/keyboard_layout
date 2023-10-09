use std::fmt::Debug;

use smallvec::SmallVec;

use crate::{DispChar, Layout, StarRng};

pub struct RampOptimize {
    rng: StarRng,
    pub frozen: Layout<bool>,
    pub beam: Vec<(u64, Layout<DispChar>)>,
}

impl Debug for RampOptimize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (cost, layout) in &self.beam {
            write!(f, "{}\n{}", cost, layout)?;
        }
        Ok(())
    }
}

impl RampOptimize {
    pub fn new<F: FnMut(usize) -> Layout<DispChar>>(
        rng_seed: u64,
        population: usize,
        mut f: F,
    ) -> Option<Self> {
        if population == 0 {
            None
        } else {
            let mut res = Self {
                rng: StarRng::new(rng_seed),
                frozen: Layout::new(|_| false),
                beam: vec![],
            };
            for i in 0..population {
                res.beam.push((u64::MAX, f(i)));
            }
            Some(res)
        }
    }

    pub fn step<F: FnMut(&Layout<DispChar>) -> u64>(&mut self, mut cost_fn: F) {
        let mut inxs_to_move = SmallVec::<[u8; 8]>::new();
        let num_keys = u8::try_from(self.beam[0].1.keys.len()).unwrap();
        // we interpolate from a 0.0 chance to be replaced for the best layout to a ~1.0
        // chance for the worst layout
        let population = self.beam.len();
        for i in 0..population {
            let chance = u32::try_from(
                u64::try_from(i)
                    .unwrap()
                    .checked_shl(32)
                    .unwrap()
                    .checked_div(u64::try_from(population).unwrap())
                    .unwrap(),
            )
            .unwrap();
            let replace = self.rng.next_u32() < chance;
            if replace {
                // choose a random layout
                let mut replacement = self.beam
                    [usize::try_from(self.rng.next_u64()).unwrap() % population]
                    .1
                    .clone();
                // swapping
                let num = (self.rng.next_u8() % 2) + 2;
                inxs_to_move.clear();
                for _ in 0..num {
                    // TODO this is inefficient
                    loop {
                        let inx = self.rng.next_u8() % num_keys;
                        if !self.frozen.keys[usize::from(inx)] {
                            inxs_to_move.push(inx);
                            break
                        }
                    }
                }
                // `inxs_to_move` is already scrambled, move in chain
                for i in 1..usize::from(num) {
                    replacement.keys.swap(
                        usize::from(inxs_to_move[i - 1]),
                        usize::from(inxs_to_move[i]),
                    );
                }
                self.beam[i].1 = replacement;
            }
            // always resample cost
            let cost = cost_fn(&self.beam[i].1);
            self.beam[i].0 = cost;
        }
        self.beam.sort()
    }
}
