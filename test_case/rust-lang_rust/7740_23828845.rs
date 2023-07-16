 rust
pub fn f() {
    static A_backing: char = 'A';   // <---- this is the important thing.
    static A: &'static char = &A_backing;
}

fn main() {
    f();
}
