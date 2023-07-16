rust
fn main() {
    let a = vec![vec![42], vec![13], vec![99]];
    let b: Vec<_> = a.into_iter().flatten().collect();
    println!("{:?}", b);
}
