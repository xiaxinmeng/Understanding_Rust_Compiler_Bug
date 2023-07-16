
fn main() {
    assert_eq!((vec![1u8,2,3]).iter().max_by(|n|**n).map(|&e|e), Some(3))
}
