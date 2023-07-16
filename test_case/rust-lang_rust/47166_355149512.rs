rust
pub trait Seek {
    fn tell(&self) -> IoResult<u64>;
    fn seek(&mut self, pos: i64, style: SeekStyle) -> IoResult<()>;
}
