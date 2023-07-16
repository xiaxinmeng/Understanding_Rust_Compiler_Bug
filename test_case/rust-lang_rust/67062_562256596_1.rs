rust
macro_rules! mbe {
    ($e: expr) => ( add_mul!($e) )
}

fn main() {
    let x = mbe!(2 + 2);
    assert_eq!(x, 8); // FAIL: the result is 6 != 8
}
