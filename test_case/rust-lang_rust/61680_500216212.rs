rust
fn main() {
    let a = vec![Some(42), None, Some(99)];
    let b: Vec<_> = a.into_iter().flatten().collect();
    println!("{:?}", b);
}
