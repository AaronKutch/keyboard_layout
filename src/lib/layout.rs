use std::{array, cmp::max, fmt::Display};

#[derive(Debug, Clone)]
pub struct Layout<T> {
    // 6 rows of 6
    pub keys: [T; 36],
}

impl<T: Display> Display for Layout<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut x: Vec<String> = self.keys.iter().map(|t| format!("{t}")).collect();
        let max_len = x.iter().fold(0, |acc, e| max(acc, e.len()));
        for s in &mut x {
            for _ in s.len()..max_len {
                s.insert(0, ' ');
            }
        }
        for i in [0, 12, 24] {
            writeln!(
                f,
                "{} {} {} {} {} {}   {} {} {} {} {} {}",
                x[i + 0],
                x[i + 1],
                x[i + 2],
                x[i + 3],
                x[i + 4],
                x[i + 5],
                x[i + 6],
                x[i + 7],
                x[i + 8],
                x[i + 9],
                x[i + 10],
                x[i + 11]
            )?;
        }
        Ok(())
    }
}

impl<T> Layout<T> {
    pub fn new<F: FnMut(usize) -> T>(f: F) -> Self {
        Self {
            keys: array::from_fn(f),
        }
    }

    pub fn base_cost(i: usize) -> u64 {
        match i {
            // 8 main keys
            13..=16 | 19..=22 => 100,
            // two below index finger
            28 | 31 => 150,
            // other keys in index and middle orthogonal neighborhood
            3 | 4 | 7 | 8 | 17 | 18 | 27 | 32 => 200,
            // other orthogonal keys in main group
            1 | 2 | 9 | 10 | 25 | 26 | 33 | 34 => 250,
            // diagonal to index
            5 | 6 | 29 | 30 => 300,
            // pinky extension
            _ => 400,
        }
    }
}
