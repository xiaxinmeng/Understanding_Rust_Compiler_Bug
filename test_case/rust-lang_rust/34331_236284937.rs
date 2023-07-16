 rust
pub const ALPHA: char = 'a';
pub const BETA: char = ALPHA;

pub fn foo(c: char) {
    match c {
        BETA => (),
        _ => ()
    }
}
