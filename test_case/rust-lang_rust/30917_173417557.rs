 Rust
#![crate_type="lib"]

use std::cmp::Ordering::*;

pub fn binary_search_by_split(a: &[i32]) -> Result<usize, usize> {
    let mut s = a;
    let mut base = 0usize;

    loop {
        let (head, tail) = s.split_at(s.len() >> 1);
        if tail.is_empty() {
            return Err(base)
        }
        match tail[0].cmp(&0) {
            Equal => return Ok(base),
            Less => {
                base += head.len() + 1;
                s = &tail[1..];
            }
            Greater => s = head,
        }
    }
}
