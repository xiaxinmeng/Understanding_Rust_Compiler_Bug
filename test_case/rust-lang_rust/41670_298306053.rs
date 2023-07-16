rust
#![feature(splice)]
fn main() {
    let mut v = (0..10).collect::<Vec<_>>();
    v.splice(7..7, 100..104);
    assert_eq!(&v, &[0, 1, 2, 3, 4, 5, 6, 100, 101, 102, 103, 7, 8, 9]);
}
