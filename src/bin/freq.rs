use std::{fs, path::PathBuf};

use triple_arena::{ptr_struct, OrdArena};

ptr_struct!(P0);

fn main() {
    let stats = fs::read_to_string(PathBuf::from("./stats.txt".to_owned())).unwrap();
    let mut stats = stats.as_bytes().to_owned();
    common::remove_other_layer_keys(&mut stats);
    let mut ord: OrdArena<P0, u8, u64> = OrdArena::new();
    for c in stats {
        if let Some(p) = ord.find_key(&c) {
            *ord.get_val_mut(p).unwrap() += 1;
        } else {
            let _ = ord.insert(c, 1);
        }
    }
    let mut ord1: OrdArena<P0, u64, u8> = OrdArena::new();
    for (_, c, freq) in ord.iter() {
        let _ = ord1.insert_nonhereditary(*freq, *c);
    }
    for (_, freq, c) in ord1.iter() {
        println!("{:?} {}", char::from(*c), *freq);
    }
    dbg!(ord1.len());
}
