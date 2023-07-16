 rust
use std::default::Default;
type new_progress = [u8; 5];

fn main() { }

fn combine_progress(a : [u8; 5], b : [u8; 5]) -> [u8; 5] {
    let mut new_progress = Default::default();

    for i in range(new_progress.len()) {
        new_progress[i] = std::cmp::max(a[i], b[i]);
    }

    new_progress
}
