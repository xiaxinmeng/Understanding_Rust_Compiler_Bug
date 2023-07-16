rust
pub struct CopyTimeout<R, W>
where
    R: AsyncRead,
    W: AsyncWrite,
{
    r: Option<R>,
    w: Option<W>,
    timeout: Duration,
    amt: u64,
    timer: Option<Timeout>,
    buf: [u8; BUFFER_SIZE],   // <----
    pos: usize,
    cap: usize,
}
