
pub trait BufferMe {
     fn from_buffer<'a>(buffer: &'a[u8]) -> Self;
}
pub struct Buffer<'a> {
    buffer: &'a[u8]
}

impl<'c> BufferMe for Buffer<'c> {
    // Lifetime 'a at the function is now shadowing
    fn from_buffer<'b>(buffer: &'b[u8]) -> Buffer<'b> {
        Buffer {
            buffer: buffer
        }
    }
}
fn main() {}
