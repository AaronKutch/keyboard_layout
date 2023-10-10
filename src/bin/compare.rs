use std::{fs, path::PathBuf};

use common::{
    colemak_dh_reference, qwerty_reference, std_primary_map, tlrs_reference, DispChar, Layout,
};

const FILE: &str = "./test_english.txt";
//const FILE: &str = "./text.txt";

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

    // remove all the special characters since the other layouts weren't designed
    // for them and the cost function heavily penalizes edge places
    text1.retain(|c| *c != b'\n');
    text1.retain(|c| *c != b'\t');
    text1.retain(|c| *c != b'.');
    text1.retain(|c| *c != b'/');
    text1.retain(|c| *c != b',');
    text1.retain(|c| *c != b';');
    text1.retain(|c| *c != b'_');

    let cost_fn = |sample: &[u8], layout: &Layout<DispChar>| {
        let mut char_to_layout_inx: [DispChar; 256] = [DispChar(0); 256];
        for (i, c) in layout.keys.iter().enumerate() {
            char_to_layout_inx[c.0 as usize] = DispChar(i as u8);
        }

        let mut cost = 0;
        for j in 1..sample.len() {
            let c0 = char_to_layout_inx[usize::from(sample[j])];
            let c1 = char_to_layout_inx[usize::from(sample[j - 1])];
            // don't include costs of spaces
            if c0.0 != b' ' {
                cost += layout.unigram_cost(c0);
                if c1.0 != b' ' {
                    cost += layout.bigram_cost(c1, c0);
                }
            }
        }
        cost
    };

    let layout = qwerty_reference();
    let cost = cost_fn(&text1, &layout);
    println!("qwerty: {}\n{}", cost, layout);
    let qwerty_cost = cost as f64;

    let layout = colemak_dh_reference();
    let cost = cost_fn(&text1, &layout);
    println!("colemak: {}\n{}", cost, layout);
    println!("improvement over qwerty: {}", qwerty_cost / (cost as f64));

    let layout = tlrs_reference();
    let cost = cost_fn(&text1, &layout);
    println!("TLRS ,/b; _.: {}\n{}", cost, layout);
    println!("improvement over qwerty: {}", qwerty_cost / (cost as f64));
}
