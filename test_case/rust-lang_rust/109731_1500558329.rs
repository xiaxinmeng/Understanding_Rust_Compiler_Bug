rust
use std::cmp::Ordering::*;
fn main() {
    let max = u64::MAX as usize;
    match (usize::MAX as u64).cmp(&u64::MAX) {
        Greater => {
            dbg!(max + 1);
        }
        Equal | Less => {}
    }
}
