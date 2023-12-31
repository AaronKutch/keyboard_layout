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

    pub fn freeze_key(&mut self, key: char, inx: usize) {
        self.frozen.keys[inx] = true;
        for layout in &mut self.beam {
            let layout = &mut layout.1;
            for i in 0..layout.keys.len() {
                if layout.keys[i].0 == crate::char_to_byte(key).unwrap() {
                    layout.keys.swap(i, inx);
                }
            }
        }
    }

    pub fn step<F: FnMut(&Layout<DispChar>) -> u64>(&mut self, mut cost_fn: F) {
        let mut inxs_to_move = SmallVec::<[u8; 8]>::new();

        let mut count = 0;
        for b in self.frozen.keys {
            if b {
                count += 1;
            }
        }
        let num_unfrozen_keys = u8::try_from(self.beam[0].1.keys.len() - count).unwrap();
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
                // swapping. it seems that single swaps work the best
                let num = 2u8;
                /* + (self.rng.next_u8() % 2) */
                inxs_to_move.clear();
                // this is ugly but the sampling step is far more expensive
                for _ in 0..num {
                    // we skip over frozen keys as if they were not there
                    let mut inx = self.rng.next_u8() % num_unfrozen_keys;
                    let mut j = 0u8;
                    loop {
                        if self.frozen.keys[usize::from(j)] {
                            inx += 1;
                            j += 1;
                        } else {
                            if j >= inx {
                                break
                            }
                            j += 1;
                        }
                    }
                    inxs_to_move.push(inx);
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
