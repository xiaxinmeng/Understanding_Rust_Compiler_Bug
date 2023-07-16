 rust
fn main() {
    let s = unsafe { String::from_utf8_unchecked(vec![b'a'; 1 << 32]) };
    panic!("{}", s);
}
