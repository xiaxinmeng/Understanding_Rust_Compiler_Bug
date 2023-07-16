 rust
fn test(x: &[u8]) {
}

macro_rules! impl_buffer( ($name:ident, $size:expr) => (
    impl $name {
        fn run(&mut self, in: &[u8]) {
            let size = $size;
            let mut i = 0;
            while in.len() - i >= size {
                test(in.slice(i, i + size));
                i += $size;
            }
        }
    }
))

pub struct Buffer;
impl_buffer!(Buffer, 64)
