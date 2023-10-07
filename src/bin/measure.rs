use std::{fs, path::PathBuf};

use common::{base_cost, colemak_dh_reference, movement_cost};

fn main() {
    let text = fs::read_to_string(PathBuf::from("./text.txt".to_owned())).unwrap();
    let mut text = text.as_bytes().to_owned();
    common::remove_other_layer_keys(&mut text);

    let mut char_to_layout_inx: [Option<u8>; 256] = [None; 256];

    //let layout = qwerty_reference();
    let layout = colemak_dh_reference();
    for (i, c) in layout.keys.iter().enumerate() {
        //assert!(char_to_layout_inx[c.0 as usize].is_none());
        char_to_layout_inx[c.0 as usize] = Some(i as u8);
    }

    let mut mapped_text = vec![];
    for c in text {
        mapped_text
            .push(char_to_layout_inx[c as usize].unwrap_or_else(|| panic!("{:?}", char::from(c))));
    }

    let mut cost = 0;
    let len = mapped_text.len();
    for i in 0..len {
        cost += base_cost(mapped_text[i]);
        if i > 0 {
            cost += movement_cost(&[mapped_text[i], mapped_text[i - 1]]);
        }
    }
    dbg!(cost);
}
