 rust
extern {
    fn bar(a: u8);
}
pub fn sum(v: Vec<u8>) {
    for e in v.into_iter() {
        unsafe { bar(e); }
    }
}
