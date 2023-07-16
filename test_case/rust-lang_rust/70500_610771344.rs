
fn main() {
    let xs: Vec<u64> = vec![1, 2, 3];
    assert_eq!(6u64, xs.iter().sum()); // This works
    // assert_eq!(xs.iter().sum(), 6u64); // This does not work.
}
