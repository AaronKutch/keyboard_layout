use common::{base_cost, colemak_dh_reference, qwerty_reference, Layout};

fn main() {
    let layout = Layout::<u64>::new(base_cost);
    println!("{layout}");

    println!("qwerty:\n{}", qwerty_reference());
    println!("colemak:\n{}", colemak_dh_reference());
}
