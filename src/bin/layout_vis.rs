use common::{colemak_dh_reference, movement_cost, qwerty_reference, Layout};

fn main() {
    for i in 0..36 {
        let layout = Layout::<String>::new(|j| {
            let cost = movement_cost(&[i, j]);
            if i == j {
                "X".to_owned()
            } else {
                format!("{cost}")
            }
        });
        println!("{layout}");
    }

    println!(
        "base costs:\n{}",
        Layout::<u64>::new(|i| movement_cost(&[i]))
    );

    println!(
        "example triple:\n{}",
        Layout::<u64>::new(|i| movement_cost(&[27, 9, i]))
    );

    println!("qwerty:\n{}", qwerty_reference());
    println!("colemak:\n{}", colemak_dh_reference());
}
