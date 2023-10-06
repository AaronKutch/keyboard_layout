mod layout;

use std::fmt::Display;

pub use layout::*;
use rand_xoshiro::{rand_core::RngCore, Xoshiro128StarStar};

#[derive(Debug, Clone, Copy)]
pub struct DispChar(pub u8);

impl Display for DispChar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", char::from(self.0))
    }
}

pub fn intersperse(rng: &mut Xoshiro128StarStar, s: &mut Vec<u8>, c: u8) {
    if s.is_empty() {
        return
    }
    let i = rng.next_u64() % u64::try_from(s.len()).unwrap();
    s.insert(usize::try_from(i).unwrap(), c);
}

// TODO: special keys we want:
//
// first, do not have enter or the other special ones on a pinky
//
// space, enter, backspace, dot, comma, colon, semicolon
pub fn remove_other_layer_keys(s: &mut Vec<u8>) {
    let to_remove = [
        '(', ')', '{', '}', '[', ']', '<', '>', '"', '\'', '`', '|', '/', '\\', '~', '@', '%', '^',
        '+', '-', '=', '\u{1b}', '&', '$', '*', '#', '?', '!', '0', '1', '2', '3', '4', '5', '6',
        '7', '8', '9',
    ]
    .map(|c| u8::try_from(c).unwrap());
    s.retain(|c| !to_remove.contains(c));
}

pub fn qwerty_reference() -> Layout<DispChar> {
    let mut res = Layout {
        keys: [DispChar(0); 36],
    };
    let s = "\tqwertyuiop_:asdfghjkl;\n=zxcvbnm,./ ";
    assert_eq!(s.len(), res.keys.len());
    for (i, c) in s.chars().enumerate() {
        res.keys[i] = DispChar(u8::try_from(c).unwrap());
    }
    res
}

pub fn colemak_dh_reference() -> Layout<DispChar> {
    let mut res = Layout {
        keys: [DispChar(0); 36],
    };
    let s = "\tqwfpbjluy;_:arstgmneio\n=xcdvzkh,./ ";
    assert_eq!(s.len(), res.keys.len());
    for (i, c) in s.chars().enumerate() {
        res.keys[i] = DispChar(u8::try_from(c).unwrap());
    }
    res
}
