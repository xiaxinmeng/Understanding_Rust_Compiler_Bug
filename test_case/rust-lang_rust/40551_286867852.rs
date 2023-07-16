Rust
    fn read_buf<B: BufMut>(&mut self, buf: &mut B) -> Poll<usize, io::Error> {
        if let Async::NotReady = <TcpStream>::poll_read(self) {
            return Err(::would_block())
        }
        let mut bufs: [_; 16] = Default::default();
        unsafe {
            let n = buf.bytes_vec_mut(&mut bufs); // `buf` is borrowed here
            match
                self.io.get_ref().read_bufs(&mut bufs[..n]) // `buf` needs to stay valid until here
            {
                Ok(n) => {
                    buf.advance_mut(n); // ... but because regions are lexical, is also borrowed here
                    Ok(Async::Ready(n))
                }
                Err(e) => {
                    if e.kind() == io::ErrorKind::WouldBlock {
                        self.io.need_write();
                    }
                    Err(e)
                }
            }
        }
    }
