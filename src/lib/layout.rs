use std::{array, cmp::max, fmt::Display};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
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
                x[i],
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

pub fn middle_column(i: u8) -> bool {
    matches!(i, 5 | 6 | 17 | 18 | 29 | 30)
}

pub fn index_column(i: u8) -> bool {
    matches!(i, 4 | 7 | 16 | 19 | 28 | 29)
}

pub fn ext_pinky(i: u8) -> bool {
    matches!(i, 0 | 11 | 12 | 23 | 24 | 35)
}

pub fn left_side(i: u8) -> bool {
    matches!(i, 0..=5 | 12..=17 | 24..=35)
}

pub fn middle_row(i: u8) -> bool {
    matches!(i, 12..=23)
}

pub fn upper_row(i: u8) -> bool {
    matches!(i, 0..=11)
}

pub fn lower_row(i: u8) -> bool {
    matches!(i, 24..=35)
}

pub fn column(i: u8) -> u8 {
    i % 12
}

/// Base cost of pressing a key
pub fn base_cost(i: u8) -> u64 {
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

/// A record of up to the last few keys, with the zeroeth being the most recent
/// press
pub fn movement_cost(x: &[u8]) -> u64 {
    let mut c = 0;
    if x.len() > 1 {
        if left_side(x[0]) == left_side(x[1]) {
            // same side

            // mid to upper or lower row changes on same side
            if !middle_row(x[0]) && middle_row(x[1]) {
                c += 100;
            }

            // these kinds of changes are always ugly
            if (upper_row(x[0]) && lower_row(x[1])) || (lower_row(x[0]) && upper_row(x[1])) {
                c += 200;
            }

            // penalize rolling outwards a little
            if left_side(x[0]) {
                if column(x[0]) > column(x[1]) {
                    c += 30;
                }
            } else {
                if column(x[0]) < column(x[1]) {
                    c += 30;
                }
            }
        }

        // a key in the index or middle column was pressed and then a far pinky is
        // pressed, heavily penalize
        if (index_column(x[1]) || middle_column(x[1])) && ext_pinky(x[0]) {
            c += 400;
        }
    }
    //if x.len() > 2 {}
    c
}

/// The costs associated with a sequence of two key presses
pub fn cost_matrix() -> [[u64; 36]; 36] {
    let mut res = [[0; 36]; 36];
    for x1 in 0..36u8 {
        for x0 in 0..36u8 {
            res[usize::from(x1)][usize::from(x0)] = movement_cost(&[x0, x1]);
        }
    }
    res
}

impl<T> Layout<T> {
    pub fn new<F: FnMut(u8) -> T>(mut f: F) -> Self {
        Self {
            keys: array::from_fn(|i| f(i as u8)),
        }
    }
}
