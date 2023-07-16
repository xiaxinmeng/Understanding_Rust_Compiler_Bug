rust
fn main() {
    let mut v1 = vec![(); 1];
    let mut v2 = vec![(); 1];
    assert_ne!(v1.as_mut_ptr(), v2.as_mut_ptr());
}
