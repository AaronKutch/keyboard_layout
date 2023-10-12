use std::{fs, path::PathBuf};

use common::std_primary_map;

const FILE: &str = "./text.txt";
const OUT: &str = "./primary_layer_text.txt";

fn main() {
    let char_map = std_primary_map();

    let text = fs::read_to_string(PathBuf::from(FILE.to_owned())).unwrap();
    let mut text = text.as_bytes().to_owned();

    // get all primary layer chars, remove ones that don't fit
    for c in &mut text {
        *c = char_map[usize::from(*c)];
    }
    text.retain(|c| *c != 0);

    // replace groups of 4 spaces with tabs
    let mut text1 = vec![];
    let mut space_count = 0;
    for c in &text {
        text1.push(*c);
        if *c == b' ' {
            space_count += 1;
        } else {
            space_count = 0;
        }
        if space_count == 4 {
            text1.pop();
            text1.pop();
            text1.pop();
            text1.pop();
            text1.push(b'\t');
            space_count = 0;
        }
    }

    fs::write(PathBuf::from(OUT.to_owned()), text1).unwrap();
}
