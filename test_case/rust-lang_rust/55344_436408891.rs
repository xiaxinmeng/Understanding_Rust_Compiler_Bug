rust
#![deny(warnings)]  

#[allow(unreachable_code)]
pub fn sum_nan() {
    let mut v;
    v = 0;
    assert_eq!(v, 0);
    return;
    v = 1;
    assert_eq!(v, 1);
}

fn main() {}
