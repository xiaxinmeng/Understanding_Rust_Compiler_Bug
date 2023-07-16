 rust
fn main() {
    let mut a = 3i;
    let b = box &mut a;
    let c = &**b;
    **b = 4i;
    println!("{}", c);
}
