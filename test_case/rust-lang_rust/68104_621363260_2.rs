rust
// bin/crash.rs
use mycrate::Num;

pub fn main() {
    let n = Num::<4>;
    n.four();
}
