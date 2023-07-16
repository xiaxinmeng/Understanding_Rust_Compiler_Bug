rust
trait Read {
    fn read (self: &'_ mut Self, buf: &'_ mut [u8]) -> Result<usize>;
    ...
}
