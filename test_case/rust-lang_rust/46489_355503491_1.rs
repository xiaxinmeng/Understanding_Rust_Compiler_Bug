rust
#![feature(proc_macro)]
extern crate add;
use add::add;

#[test]
fn test_add() {
    let a = 1;
    let b = 2;
    let c = add!(a, b);
    assert_eq!(3, c);
}
