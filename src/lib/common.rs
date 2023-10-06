mod layout;

use rand_xoshiro::{rand_core::RngCore, Xoshiro128StarStar};

pub fn intersperse(rng: &mut Xoshiro128StarStar, s: &mut Vec<u8>, c: u8) {
    if s.is_empty() {
        return
    }
    let i = rng.next_u64() % u64::try_from(s.len()).unwrap();
    s.insert(usize::try_from(i).unwrap(), c);
}

pub fn remove_other_layer_keys(s: &mut Vec<u8>) {
    let to_remove = [
        '(', ')', '{', '}', '[', ']', '<', '>', '"', '\'', '|', '/', '\\', '~', '@', '%', '^', '+',
        '-', '$', '*', '#', '?', '!', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
    ]
    .map(|c| u8::try_from(c).unwrap());
    s.retain(|c| !to_remove.contains(c));
}
