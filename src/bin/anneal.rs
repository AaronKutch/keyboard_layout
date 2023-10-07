use std::{fs, path::PathBuf};

use common::{base_cost, colemak_dh_reference, movement_cost, qwerty_reference, AnnealRamp};

fn main() {
    let text = fs::read_to_string(PathBuf::from("./text.txt".to_owned())).unwrap();
    let mut text = text.as_bytes().to_owned();
    common::remove_other_layer_keys(&mut text);

    //let layout = qwerty_reference();
    let layout = qwerty_reference();

    let mut anneal = AnnealRamp::new(32, layout).unwrap();

    for step in 0..50 {
        dbg!(anneal.beam[0].0);
        anneal.step(|layout| {
            let mut char_to_layout_inx: [Option<u8>; 256] = [None; 256];
            for (i, c) in layout.keys.iter().enumerate() {
                //assert!(char_to_layout_inx[c.0 as usize].is_none());
                char_to_layout_inx[c.0 as usize] = Some(i as u8);
            }

            let mut mapped_text = vec![];
            for c in &text {
                mapped_text.push(
                    char_to_layout_inx[usize::from(*c)]
                        .unwrap_or_else(|| panic!("{:?}", char::from(*c))),
                );
            }
            let mut cost = 0;
            let len = mapped_text.len();
            for i in 0..len {
                cost += base_cost(mapped_text[i]);
                if i > 0 {
                    cost += movement_cost(&[mapped_text[i], mapped_text[i - 1]]);
                }
            }
            //dbg!(cost);
            cost
        });
    }
    //dbg!(&anneal);

    dbg!(anneal.beam[0].0);
    println!("annealed:\n{}", anneal.beam[0].1);
    println!("colemak:\n{}", colemak_dh_reference());
}
/*
annealed:
'\t'  'f'  'w'  'j'  '_'  'b'    '/'  'p'  'u'  'l'  'g'  '='
 'q' '\n'  'i'  's'  't'  'm'    'a'  ' '  'e'  'd'  'o'  ';'
 'x'  'n'  'r'  'h'  '.'  'y'    'c'  ':'  ','  'v'  'z'  'k'

colemak:
'\t'  'q'  'w'  'f'  'p'  'b'    'j'  'l'  'u'  'y'  ';'  '_'
 ':'  'a'  'r'  's'  't'  'g'    'm'  'n'  'e'  'i'  'o' '\n'
 '='  'x'  'c'  'd'  'v'  'z'    'k'  'h'  ','  '.'  '/'  ' '
*/
