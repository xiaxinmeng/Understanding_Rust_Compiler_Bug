rust
  pub trait Read {
    // ...
    
    fn read_exact_vectored(&mut self, mut bufs: &mut [IoSliceMut]) -> std::io::Result<()> {
      // Guarantee that bufs is empty if it contains no data,
      // to avoid calling write_vectored if there is no data to be written.
      bufs = IoSliceMut::advance(bufs, 0);
      while !bufs.is_empty() {
        match self.read_vectored(bufs) {
          Ok(0) => {
            return Err(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "failed to fill whole buffer"));
          }
          Ok(n) => bufs = IoSliceMut::advance(bufs, n),
          Err(ref e) if e.kind() == std::io::ErrorKind::Interrupted => {}
          Err(e) => return Err(e),
        }
      }
      Ok(())
    }
  }
  