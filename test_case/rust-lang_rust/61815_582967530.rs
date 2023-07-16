rust
trait T {
    const N: usize;
    fn fun(x: [u8; Self::N]);
}
