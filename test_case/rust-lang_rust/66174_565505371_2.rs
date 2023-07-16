rust
trait Read {
    fn read (self:&'_ mut Self, buf: &'_ out [u8]) -> Result<usize>;
    ...
}
