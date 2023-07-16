 rust
fn main() {
    let v = vec![1_u8, 2, 3, 4, 5, 6, 7, 8, 9];
    println!("{}", test(&v[]));
}

fn test(n: &[u8]) -> u8 {
    1 << n[0]
}
