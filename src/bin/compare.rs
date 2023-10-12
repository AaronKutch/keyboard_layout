use std::{fs, path::PathBuf};

use common::*;

//const FILE: &str = "./test_english.txt";
const FILE: &str = "./text.txt";

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
    // for them
    text1.retain(|c| *c != b'\n');
    text1.retain(|c| *c != b'\t');
    text1.retain(|c| *c != b'.');
    text1.retain(|c| *c != b'/');
    text1.retain(|c| *c != b',');
    text1.retain(|c| *c != b';');
    text1.retain(|c| *c != b'_');
    text1.retain(|c| *c != b'(');
    text1.retain(|c| *c != b')');

    let cost_fn = |sample: &[u8], layout: &Layout<DispChar>| {
        let mut char_to_layout_inx: [u8; 256] = [0; 256];
        for (i, c) in layout.keys.iter().enumerate() {
            char_to_layout_inx[c.0 as usize] = i as u8;
        }

        let mut cost = 0;
        let mut v = vec![];
        for i in 0..sample.len() {
            if text[i] == b' ' {
                v.clear();
                continue
            }
            let mapped = char_to_layout_inx[usize::from(text[i])];
            v.push(mapped);
            if v.len() > 3 {
                v.remove(0);
            }
            cost += movement_cost(&v);
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
    println!("improvement over qwerty: {}\n", qwerty_cost / (cost as f64));

    let layout = dvorak_reference();
    let cost = cost_fn(&text1, &layout);
    println!("dvorak: {}\n{}", cost, layout);
    println!("improvement over qwerty: {}\n", qwerty_cost / (cost as f64));

    let layout = isrt_reference();
    let cost = cost_fn(&text1, &layout);
    println!("ISRT: {}\n{}", cost, layout);
    println!("improvement over qwerty: {}\n", qwerty_cost / (cost as f64));

    /*let layout = tlrs_reference();
    let cost = cost_fn(&text1, &layout);
    println!("TLRS: {}\n{}", cost, layout);
    println!("improvement over qwerty: {}\n", qwerty_cost / (cost as f64));*/

    let layout = v9_reference();
    let cost = cost_fn(&text1, &layout);
    println!("V9: {}\n{}", cost, layout);
    println!("improvement over qwerty: {}\n", qwerty_cost / (cost as f64));

    let layout = v10_reference();
    let cost = cost_fn(&text1, &layout);
    println!("V10: {}\n{}", cost, layout);
    println!("improvement over qwerty: {}\n", qwerty_cost / (cost as f64));

    let layout = uciea_reference();
    let cost = cost_fn(&text1, &layout);
    println!("Uciea: {}\n{}", cost, layout);
    println!("improvement over qwerty: {}\n", qwerty_cost / (cost as f64));

    let layout = v11_reference();
    let cost = cost_fn(&text1, &layout);
    println!("V11: {}\n{}", cost, layout);
    println!("improvement over qwerty: {}\n", qwerty_cost / (cost as f64));

    let layout = map!("zmclvg.;/uh~qnsrtp_ieao~jbwkdf(,y)x~");
    let cost = cost_fn(&text1, &layout);
    println!("nsrt: {}\n{}", cost, layout);
    println!("improvement over qwerty: {}\n", qwerty_cost / (cost as f64));
}
