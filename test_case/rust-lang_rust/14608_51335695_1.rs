 rust
extern {
    fn f(x: u8);
}

fn main() {
    unsafe { f(2); }
}
