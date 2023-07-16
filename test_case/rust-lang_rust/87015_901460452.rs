rust
use std::collections::HashSet;

#[derive(Default, Eq, PartialEq, Clone)] // no Hash
struct Data;

fn main() {
    let mut set = HashSet::<Data>::new();
    set.extend(std::iter::once(Data));
}
