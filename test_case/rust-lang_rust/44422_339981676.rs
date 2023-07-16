rust
fn send_all(&mut self) -> Result<usize, io::Error> {
    let null = unsafe { mem::uninitialized() };
    let mut size = 0;
    let mut bufs = [null; IOVEC_MAX];
    for (idx, message) in self.pending.iter().enumerate().take(IOVEC_MAX / 2) {
        size += 2;
        bufs[idx * 2] = &message.mbuf.head.as_ref()[..];
        bufs[idx * 2 + 1] = &message.mbuf.data.as_ref()[..];
    }

    self.sock.send_all(&bufs[..size])
}
