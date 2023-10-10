use std::{fs, iter, path::PathBuf};

use common::{char_to_byte, std_primary_map, StarRng};

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

    // add in backspaces and escapes, I think they should be this common considering
    // writing code and escaping from shortcuts
    let mut rng = StarRng::new(0);
    let mut text2 = vec![];
    let mut i = 0;
    let mut next_backspace = 0;
    //let mut next_escape = 0;
    //let escape = char_to_byte('\u{1b}').unwrap();
    let backspace = char_to_byte('\u{8}').unwrap();
    loop {
        if i >= text1.len() {
            break
        }
        /*text2.push(text1[i]);
        if i >= next_escape {
            text2.push(escape);
            next_escape = i + ((rng.next_u16() % 100) as usize);
        }*/
        if i >= next_backspace {
            text2.extend(iter::repeat(backspace).take((rng.next_u16() % 10) as usize));
            next_backspace = i + ((rng.next_u16() % 100) as usize);
        }
        i += 1;
    }

    fs::write(PathBuf::from(OUT.to_owned()), text2).unwrap();
}
