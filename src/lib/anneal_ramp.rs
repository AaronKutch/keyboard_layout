use std::fmt::Debug;

use smallvec::SmallVec;

use crate::{DispChar, Layout, StarRng};

pub struct AnnealRamp {
    rng: StarRng,
    pub beam: Vec<(u64, Layout<DispChar>)>,
}

impl Debug for AnnealRamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (cost, layout) in &self.beam {
            write!(f, "{}\n{}", cost, layout)?;
        }
        Ok(())
    }
}

impl AnnealRamp {
    pub fn new(size: usize, init: Layout<DispChar>) -> Option<Self> {
        if size == 0 {
            None
        } else {
            Some(Self {
                rng: StarRng::new(0),
                beam: vec![(u64::MAX, init); size],
            })
        }
    }

    pub fn step<F: FnMut(&Layout<DispChar>) -> u64>(&mut self, mut cost_fn: F) {
        let mut inxs_to_move = SmallVec::<[u8; 8]>::new();
        let num_keys = u8::try_from(self.beam[0].1.keys.len()).unwrap();
        // we interpolate from a 0.0 chance to be replaced for the best layout to a ~1.0
        // chance for the worst layout
        let size = self.beam.len();
        for i in 0..size {
            let chance = u32::try_from(
                u64::try_from(i)
                    .unwrap()
                    .checked_shl(32)
                    .unwrap()
                    .checked_div(u64::try_from(size).unwrap())
                    .unwrap(),
            )
            .unwrap();
            let replace = self.rng.next_u32() < chance;
            if replace {
                // choose a random layout
                let mut replacement = self.beam
                    [usize::try_from(self.rng.next_u64()).unwrap() % size]
                    .1
                    .clone();
                // probably no reason to swap more than 8 at a time
                let num = (self.rng.next_u8() % 7) + 2;
                inxs_to_move.clear();
                for _ in 0..num {
                    inxs_to_move.push(self.rng.next_u8() % num_keys);
                }
                // `inxs_to_move` is already scrambled, move in chain
                for i in 1..usize::from(num) {
                    replacement.keys.swap(
                        usize::from(inxs_to_move[i - 1]),
                        usize::from(inxs_to_move[i]),
                    );
                }
                let cost = cost_fn(&replacement);
                self.beam[i] = (cost, replacement);
            }
        }
        self.beam.sort()
    }
}
