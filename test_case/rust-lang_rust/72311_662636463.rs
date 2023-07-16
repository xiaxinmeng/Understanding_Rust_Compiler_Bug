rust
#![feature(once_cell)]
use std::lazy::SyncLazy;
static A: SyncLazy<u32> = SyncLazy::new(|| *A);
fn main() {
    println!("{}", *A);
}
