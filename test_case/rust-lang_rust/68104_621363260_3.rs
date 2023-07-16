rust
// bin/crash2.rs
use mycrate::Num;

pub fn main() {
    let n = Num::<5>;
    n.five();
}
