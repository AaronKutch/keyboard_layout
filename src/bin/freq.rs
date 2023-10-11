use std::{fs, path::PathBuf};

const FILE: &str = "./text.txt";

fn main() {
    let text = fs::read_to_string(PathBuf::from(FILE.to_owned())).unwrap();
    let text = text.as_bytes().to_owned();

    let mut counts: Vec<(u64, u8)> = vec![(0, 0); 256];
    for (i, count) in counts.iter_mut().enumerate() {
        count.1 = i as u8;
    }
    for c in &text {
        counts[usize::from(*c)].0 += 1;
    }
    counts.sort_by(|a, b| a.0.cmp(&b.0));

    for (count, c) in &counts {
        println!(
            "{} {:?} {}",
            char::try_from(*c).unwrap_or('?'),
            char::try_from(*c),
            count
        );
    }
}
