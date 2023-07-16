Rust
    fn read_buf<B: BufMut>(&mut self, buf: &mut B) -> Poll<usize, io::Error> {
        if let Async::NotReady = <TcpStream>::poll_read(self) {
            return Err(::would_block())
        }
        let mut bufs: [_; 16] = Default::default();
        unsafe {
            let result /* : io::Result<usize> - no borrowed content */ = {
                let n = buf.bytes_vec_mut(&mut bufs);
                self.io.get_ref().read_bufs(&mut bufs[..n])
                // `buf` is only borrowed within ^
            };
            match result {
                Ok(n) => {
                    buf.advance_mut(n);
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
