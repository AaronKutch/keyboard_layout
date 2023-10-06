use common::{base_cost, Layout};

fn main() {
    let layout = Layout::<u64>::new(base_cost);
    println!("{layout}");
}
