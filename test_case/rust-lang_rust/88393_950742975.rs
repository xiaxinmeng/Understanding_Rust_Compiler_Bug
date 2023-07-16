rust
#![allow(deprecated, invalid_value)]

enum Void {}

fn main() {
    let b = false;
    // should warn:
    // any code following this `match` expression is unreachable, as all arms diverge
    match b {
        false => unsafe { std::mem::uninitialized::<Void>(); }
        _ => unreachable!(),
    }
    println!();
}
