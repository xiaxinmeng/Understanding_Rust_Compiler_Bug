
pub trait BufferMe<'a> {
     fn from_buffer<'a>(buffer: &'a[u8]) -> Self;
}
