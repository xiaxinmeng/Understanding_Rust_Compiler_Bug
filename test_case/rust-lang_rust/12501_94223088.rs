 rust
#![feature(negate_unsigned)]

fn main() {
    const MAX: usize = -1;
    match 0 {
        1usize...MAX => {},
        _ => {}
    }
}
