rust
fn main() {
    let b: &[u8] = b"\xC2";
    let s: &std::ffi::OsStr = unsafe { std::mem::transmute(b) };
    dbg!(s);
}
