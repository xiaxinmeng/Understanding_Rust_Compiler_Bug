Rust
#![feature(nll)]

fn main() {
    let mut x = 2;
    match x {
        ref mut t if {
            println!("In Guard: {:?}", (&t) as *const &mut i32);
            true
        } => {
            // with current play.rust-lang.org debug, this prints a different address
            // than in the guard.
            println!("In   Arm: {:?}", (&t) as *const &mut i32);
        }
        _ => {}
    }
}
