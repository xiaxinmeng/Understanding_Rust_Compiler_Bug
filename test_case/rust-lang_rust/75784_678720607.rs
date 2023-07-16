rust
use std::iter::{once, repeat};

pub fn t1() -> String {
    let mut iter = ["hello", "world"].iter().copied();
    let first = iter.next();
    let rest = repeat(" ")
        .zip(iter)
        .flat_map(|(x, y)| once(x).chain(once(y)));
    first.iter().copied().chain(rest).collect()
}
