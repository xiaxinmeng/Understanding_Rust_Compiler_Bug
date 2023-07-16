 rust
#![feature(collections)]
fn main() {
    let mut line = [0u8; 512];
    {
        let mut buf = &mut line[..];

        /// reslice is essentially equivalent to `*buf = &mut buf[len..];`.
        fn reslice(buf: &mut &mut [u8], len: usize) {
            let tmp = std::mem::replace(buf, &mut []);
            std::mem::replace(buf, &mut tmp[len..]);
        }
        fn append(buf: &mut &mut [u8], v: &[u8]) {
            let len = buf.clone_from_slice(v);
            reslice(buf, len);
        }

        append(&mut buf, b"test");
        append(&mut buf, b"foo");
    }
    assert_eq!(&line[..7], b"testfoo");
}
