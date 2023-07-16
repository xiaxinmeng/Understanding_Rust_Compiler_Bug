rust
#![allow(unreachable_code)]
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq)]
pub struct CustomSet<T>(HashSet<T>);

fn main() {
    let set1: CustomSet<i32> = todo!();
    let set2: CustomSet<i32> = todo!();
    assert_eq!(set1, set2);
}
