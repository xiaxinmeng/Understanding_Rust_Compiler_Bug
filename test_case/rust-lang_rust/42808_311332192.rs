rust
// vecel.rs

use std::env;

fn main() {
    let mut args = env::args();
    args.next();
    let arg1 = args.next().unwrap();
    let num: usize = arg1.parse().unwrap();
    let mut v = Vec::new();
    for i in 0..num {
        v.push(i);
    }
    assert_eq!(v.len(), num);
}
