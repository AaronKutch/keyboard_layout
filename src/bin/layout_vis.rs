use common::Layout;

fn main() {
    let layout = Layout::<u64>::new(|i| Layout::<()>::base_cost(i));
    println!("{layout}");
}
