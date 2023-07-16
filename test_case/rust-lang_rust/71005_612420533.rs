rust
// Something large.
struct Buf([u8; 128]);
fn fill(buf: &mut Buf) {...}
fn make() -> Buf {
    let mut buf = Buf::new();
    fill(&mut buf);
    buf
}
