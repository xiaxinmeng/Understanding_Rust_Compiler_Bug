rust
fn main() {
    let v: Vec<usize> = vec![1,2,3,4];
    assert_eq!(10_usize, v.iter().sum());
}
