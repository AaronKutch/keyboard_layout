mod anneal_ramp;
mod layout;
mod rng;

use std::fmt::Display;

pub use anneal_ramp::*;
pub use layout::*;
use rand_xoshiro::{rand_core::RngCore, Xoshiro128StarStar};
pub use rng::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DispChar(pub u8);

impl Display for DispChar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", char::from(self.0))
    }
}

pub fn char_to_byte(c: char) -> Option<u8> {
    // as_ascii isn't stable yet
    let mut s = String::new();
    s.push(c);
    if !s.is_ascii() {
        None
    } else {
        Some(s.as_bytes()[0])
    }
}

pub fn intersperse(rng: &mut Xoshiro128StarStar, s: &mut Vec<u8>, c: u8) {
    if s.is_empty() {
        return
    }
    let i = rng.next_u64() % u64::try_from(s.len()).unwrap();
    s.insert(usize::try_from(i).unwrap(), c);
}

// All the "ordinary" chars we could care about, including the special
// escape, backspace, tab, space, and newline chars
pub fn ordinary_chars() -> Vec<char> {
    let mut v = [
        '\u{1b}', '\u{8}', '\t', '\n', ' ', '~', '%', 'J', 'Z', '@', 'Q', '^', 'Y', 'X', '+', 'j',
        '?', '9', '\\', '7', 'G', '$', '5', 'W', 'H', 'K', 'q', 'V', 'U', '*', '8', '4', 'z', '6',
        'B', '3', '|', 'M', 'L', 'F', '!', 'D', 'P', '2', '#', 'N', 'O', '1', '\'', '0', ']', '[',
        'A', 'C', '<', 'R', '&', 'I', 'E', 'T', '-', 'S', 'k', ';', '>', 'w', '`', 'v', '}', '{',
        '=', 'x', 'y', '"', 'b', 'g', ',', 'h', '.', '(', ')', '/', 'm', ':', 'f', 'p', 'u', '_',
        'd', 'c', 'l', 'o', 'r', 'n', 's', 'a', 'i', 't', 'e',
    ]
    .to_vec();
    v.sort();
    assert_eq!(v.len(), 99);
    v
}

pub fn lowercase_ordinary_chars() -> Vec<char> {
    let mut v: Vec<char> = ordinary_chars()
        .into_iter()
        .map(|c| c.to_ascii_lowercase())
        .collect();
    v.sort();
    v.dedup();
    assert_eq!(v.len(), 99 - 26);
    v
}

pub fn lowercase_alpha_chars() -> Vec<char> {
    [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ]
    .to_vec()
}

pub fn primary_layer_chars() -> Vec<char> {
    let mut v = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '_', ' ', '\u{8}', '\u{1b}', '\t', '\n', ':', '/',
        '.', ',', ';',
    ]
    .to_vec();
    v.sort();
    v
}

// there is some modifier to convert to capitals

// layout should probably be
/*
  ~ ` '    " & ^
< > ( )    [ ] { }
      \    |
*/
pub fn delim_layer_chars() -> Vec<char> {
    let mut v = [
        '\\', '|', '\'', '"', '`', '~', '<', '>', '(', ')', '[', ']', '{', '}',
    ]
    .to_vec();
    v.sort();
    v
}

// dual layer (arrow keys are shown in "v < ^ >")
/*

  @ # ? ! % 0 7 8 9 0
  v < ^ > = 0 4 5 6 0
^ / * - + $ 0 1 2 3 0

*/

// maps all bytes to the primary layer chars, also lowercasing alphabetical
// chars
pub fn std_primary_map() -> [u8; 256] {
    let mut char_map: [u8; 256] = [0; 256];
    for c in primary_layer_chars() {
        let c = char_to_byte(c).unwrap();
        char_map[usize::from(c)] = c;
    }
    for c in lowercase_alpha_chars() {
        let c_lo = char_to_byte(c).unwrap();
        let c_hi = char_to_byte(c.to_ascii_uppercase()).unwrap();
        char_map[usize::from(c_hi)] = c_lo;
    }
    char_map
}

pub fn qwerty_reference() -> Layout<DispChar> {
    let mut res = Layout {
        keys: [DispChar(0); 36],
    };
    let s = "\tqwertyuiop_\u{1b}asdfghjkl;\n:zxcvbnm,./ ";
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
    let s = "\tqwfpbjluy;_\u{1b}arstgmneio\n:xcdvzkh,./ ";
    assert_eq!(s.len(), res.keys.len());
    for (i, c) in s.chars().enumerate() {
        res.keys[i] = DispChar(u8::try_from(c).unwrap());
    }
    res
}
