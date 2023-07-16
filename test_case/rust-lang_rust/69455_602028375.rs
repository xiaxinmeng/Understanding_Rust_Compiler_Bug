rust
fn main() {
    let xs: Vec<u64> = vec![1, 2, 3];
    let ys: Vec<u64> = xs.iter().map(|i: &u64| i*2).collect();
    let _: u64 = 23u64 + xs.iter().sum();
}
