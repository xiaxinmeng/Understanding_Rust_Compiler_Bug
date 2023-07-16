
#![feature(once_cell)]
use std::lazy::Lazy;
use std::ops::Deref;
pub fn strint(x: u64) -> impl Deref<Target = String> {
    Lazy::new(move || x.to_string())
}
