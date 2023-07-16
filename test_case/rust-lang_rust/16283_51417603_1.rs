 rust
fn main() {
    let c: c_char = -1;
    unsafe { f(c as c_int); }
}
