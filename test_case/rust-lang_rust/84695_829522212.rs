rust
fn f(x: Option<u32>, y: Option<u32>) -> Option<u32> {
    x.into_iter().chain(y.into_iter()).min()
}

fn main() {
    assert_eq!(f(None, None), None);
    assert_eq!(f(Some(10), None), Some(10));
    assert_eq!(f(None, Some(5)), Some(5));
    assert_eq!(f(Some(10), Some(5)), Some(5));
}
