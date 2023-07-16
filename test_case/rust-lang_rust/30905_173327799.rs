 rust
mod foo {
    struct Buf {
        inner: Vec<u8>
    }

    mod bar {
        use super::Buf;

        pub struct Encoder;

        impl Encoder {
            pub fn encode(&mut self, buf: &mut Buf) {
                buf.inner.push(b'!');
            }
        }
    }
}
