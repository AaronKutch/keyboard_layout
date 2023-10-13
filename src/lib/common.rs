mod layout;
mod ramp_optimize;
mod rng;

use std::fmt::Display;

pub use layout::*;
pub use ramp_optimize::*;
use rand_xoshiro::{rand_core::RngCore, Xoshiro128StarStar};
pub use rng::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DispChar(pub u8);

impl Display for DispChar {
    /// note: converts to lowercase and maps some special cases to uppercase
    /// letters
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self.0;
        let c = if s == 0 {
            'Z'
        } else if s == char_to_byte(' ').unwrap() {
            'S'
        } else if s == char_to_byte('\n').unwrap() {
            'N'
        } else if s == char_to_byte('\t').unwrap() {
            'T'
        } else if s == char_to_byte('\u{8}').unwrap() {
            'B'
        } else if s == char_to_byte('\u{1b}').unwrap() {
            'E'
        } else {
            char::from(self.0)
        };
        write!(f, "{}", c)
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

// space, tab, enter, escape need to be handled by thumbs and special keys
// '(', ')' have a high enough frequency, and I will remap intentation changing
// to shift plus them
pub fn primary_layer_chars() -> Vec<char> {
    let mut v = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '_', '/', '.', ',', ';', '(', ')',
    ]
    .to_vec();
    v.sort();
    assert_eq!(v.len(), 33);
    v
}

/// Returns a randomly sorted layout
pub fn rand_layout(rng: &mut StarRng) -> Layout<DispChar> {
    let chars = primary_layer_chars();
    let mut layout = Layout::new(|i| {
        DispChar(if let Some(c) = chars.get(usize::from(i)) {
            char_to_byte(*c).unwrap()
        } else {
            0
        })
    });
    let len = layout.keys.len();
    for i in 0..len {
        layout.keys.swap(
            i,
            usize::try_from(rng.next_u32() % u32::try_from(len).unwrap()).unwrap(),
        );
    }
    layout
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

#[macro_export]
macro_rules! map {
    ($s:expr) => {{
        let mut res = Layout {
            keys: [DispChar(0); 36],
        };
        assert_eq!($s.len(), res.keys.len());
        for (i, c) in $s.chars().enumerate() {
            res.keys[i] = DispChar(u8::try_from(c).unwrap());
        }
        res
    }};
}

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
    map!("~qwertyuiop_~asdfghjkl;~~zxcvbnm,./~")
}

pub fn colemak_dh_reference() -> Layout<DispChar> {
    map!("~qwfpbjluy;_~arstgmneio~~xcdvzkh,./~")
}

pub fn dvorak_reference() -> Layout<DispChar> {
    map!("~q,.pyfgcrl/~aoeuidhtns~~_qjkxbmwvz~")
}

pub fn isrt_reference() -> Layout<DispChar> {
    map!("~yclmkzfu_,~~isrtgpneao~~qvwdjbh/.~~")
}

pub fn tlrs_reference() -> Layout<DispChar> {
    /*
    E w b / m v   y u o _ . j
    T t l r s p   B S i e a N
    q , g d n k   ; h c x f z
    */
    //"\u{1b}wb/mvyuo_.j\ttlrsp\u{8} iea\nq,gdnk;hcxfz"
    map!("~wb/mvyuo_.j~tlrsp~~iea~q,gdnk;hcxfz")
}

pub fn v9_reference() -> Layout<DispChar> {
    /*
        j u o / ; w   m k l _ v q
        ( i e a t d   p n r s c )
        y , . f g   b h x   z

        l u o . q w   v k l m _
        ( i e a t g   p n r s c )
        y z / d f   b h x , j
    */
    map!("juo/lwmkl_vq(ieatdpnrsc)~y,.fgbhx~z~")
}

pub fn v10_reference() -> Layout<DispChar> {
    // note: the program would say to swap 'd' and 'm', but
    // the important 'd' would be in the worst place on a horizontally
    // staggered keyboard
    /*
        j b f l w g   . ; / u h ~
        z n t r s c   _ i e a o ~
        q p v k d m   ( , y ) x ~
    */
    map!("jbflwg.;/uh~zntrsc_ieao~qpvkdm(.y)x~")
}

pub fn uciea_reference() -> Layout<DispChar> {
    map!("~pyuo_kdhfxq~ciea/gtnsrv~z(,.;wmlbj)")
}

pub fn v11_reference() -> Layout<DispChar> {
    // gets rid of annoying placement of top 10 chars,
    // only adds the SP bigram and makes other things more elegant
    /*
        j b l d v w   . ; / u h ~
        q n r t s p   _ i e a o ~
        z m k g f c   ( , y ) x ~
    */
    map!("jbldvw.;/uh~qnrtsp_ieao~zmkgfc(,y)x~")
}

pub fn v12_reference() -> Layout<DispChar> {
    // based on manual feedback, the cost function did not punish sfbs quite enough,
    // I froze the h key, then mirroring the result matches
    // https://github.com/rdavison/graphite-layout and makes the top sfbs SP with
    // only 0.158% frequency, at the cost of only 1% vs my cost function
    /*
        j b l d v w   . ; o u y ~
        q n r t s p   ( h a e i ~
        z m k g f c   _ , x ) / ~
    */
    map!("jbldvw.;ouy~qnrtsp(haei~zmkgfc_,x)/~")
}
