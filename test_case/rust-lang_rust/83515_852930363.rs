rust
#![feature(pattern)]

use std::str::pattern::{Searcher, Pattern};

fn main() {
    let s = "aaaaa_aaaaa_aaaa";
    let mut searcher = '_'.into_searcher(s);
    while let Some((a, b)) = searcher.next_reject() {
        println!("reject: {:?}: {:?}", a..b, &s[a..b]);
    }
}
