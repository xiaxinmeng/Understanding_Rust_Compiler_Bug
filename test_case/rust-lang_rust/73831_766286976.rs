rust
#![feature(partition_point)]
use std::iter::Iterator;
fn main() {
    let mut s = [2, 7, 1, 8, 2, 8, 1, 8, 2, 8, 4];
    let a = itertools::partition(&mut s, |x| *x > 5);

    for (i, x) in s.iter().enumerate() {
        print!("{}{}", if i == a { "|" } else { " " }, x);
    }
    //  8 7 8 8 8|2 1 1 2 2 4

    let b = s.partition_point(|x| *x > 5);
    assert_eq!(a, b); // pass
}
