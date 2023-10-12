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

impl<T> Layout<T> {
    pub fn new<F: FnMut(u8) -> T>(mut f: F) -> Self {
        Self {
            keys: array::from_fn(|i| f(i as u8)),
        }
    }
}

pub fn column(i: u8) -> u8 {
    i % 12
}

pub fn row(i: u8) -> u8 {
    i / 12
}

pub fn right_side(i: u8) -> bool {
    column(i) >= 6
}

pub fn same_side(i: u8, j: u8) -> bool {
    right_side(i) == right_side(j)
}

pub fn neighboring_columns(i: u8, j: u8) -> bool {
    (column(i) + 1 == column(j)) || (column(i) == column(j) + 1)
}

pub fn between_groups(group0: &[u8], group1: &[u8], i: u8, j: u8) -> bool {
    (group0.contains(&column(i)) && group1.contains(&column(j)))
        || (group1.contains(&column(i)) && group0.contains(&column(j)))
}

pub fn neighboring_between_column_groups(group0: &[u8], group1: &[u8], i: u8, j: u8) -> bool {
    neighboring_columns(i, j) && between_groups(group0, group1, i, j)
}

pub fn diagonal_penalty(i: u8, j: u8) -> u64 {
    let mut c = 0;
    let group0 = [2, 3, 8, 9];
    let group1 = [1, 4, 7, 10];
    if same_side(i, j) {
        // penalize rolling outwards slightly
        if right_side(i) {
            if column(i) > column(j) {
                c += 50;
            }
        } else if column(i) < column(j) {
            c += 50;
        }
        if neighboring_columns(i, j) {
            // things like 'e'<->'f' on QWERTY is easy but 'c'<->'f' is hard. However, this
            // is on a staggered keyboard, I plan on using a vertically staggered ortho
            // keyboard that would make 'e'<->'f' harder and 'c'<->'f' easier. The vertical
            // stagger differs between boards. I will take the middle ground by penalizing
            // all immediate diagonals, and penalize movements between the group of columns
            // 2,3,8,9 and the group 1,4,7,10 more strongly.
            let bad_coincidence = between_groups(&group0, &group1, i, j);
            if bad_coincidence {
                c += 100;
            }
            if row(i).abs_diff(row(j)) == 1 {
                c += 100;
            } else if row(i).abs_diff(row(j)) == 2 {
                // always very bad to splay your fingers
                c += 200;
            }
        }
    }
    c
}

/// A record of the last few keys, in order from older to newer keys
pub fn movement_cost(older_to_newer: &[u8]) -> u64 {
    let len = older_to_newer.len();
    let mut c = 0;
    let group1 = [1, 4, 7, 10];
    let group2 = [0, 5, 6, 11];
    if len >= 1 {
        let i = older_to_newer[len - 1];
        // base cost of pressing key
        c += match i {
            // 8 main keys
            13..=16 | 19..=22 => 100,
            // two below index finger, 4 above middle and ring fingers
            28 | 31 | 2 | 3 | 8 | 9 => 150,
            // get stuff away from the pinkies
            0 | 11 | 12 | 23 | 24 | 35 => 400,
            // not too high, other rules are more important
            _ => 200,
        };
    }
    if len >= 2 {
        let i = older_to_newer[len - 1];
        let j = older_to_newer[len - 2];
        if column(i) == column(j) {
            if row(i).abs_diff(row(j)) == 1 {
                // immediate vertical redirects
                c += 300;
            }
            if row(i).abs_diff(row(j)) == 2 {
                // heavily penalize vertical redirects from top to bottom row or vice versa
                c += 500;
            }
        }
        // also need to count redirects on pinky and index finger
        if neighboring_between_column_groups(&group1, &group2, i, j) {
            // diagonal penalty will add the needed extra penalty for cases like 'f' to 'b'
            // on QWERTY
            c += 200;
        }
        c += diagonal_penalty(i, j);
    }
    if len >= 3 {
        let i = older_to_newer[len - 1];
        // I think that maybe we actually don't incorporate `j`, because there will be
        // two bad i,j costs in subsequent steps if they were actually a problem on the
        // same hand let j = older_to_newer[len - 2];
        let k = older_to_newer[len - 3];
        if column(i) == column(k) {
            // redirects on recent keys
            if row(i).abs_diff(row(k)) == 1 {
                c += 100;
            }
            if row(i).abs_diff(row(k)) == 2 {
                c += 200;
            }
        }
        if neighboring_between_column_groups(&group1, &group2, i, k) {
            c += 50;
        }
        c += diagonal_penalty(i, k) / 3;
    }
    c
}
