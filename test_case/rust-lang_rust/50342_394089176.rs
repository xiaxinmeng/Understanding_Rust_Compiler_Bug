rust
#![feature(euclidean_division)]

use std::f32::INFINITY;

#[test]
fn mod_euc() {
    let a = 42.0;
    assert!(INFINITY.mod_euc(a).is_nan());
}

#[test]
fn div_euc() {
    let a = 42.0;
    assert_eq!(a.div_euc(INFINITY), 0.0);
}
