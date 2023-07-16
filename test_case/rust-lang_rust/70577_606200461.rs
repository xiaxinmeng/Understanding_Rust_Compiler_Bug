rust
impl<R: Seek> Seek for BufReader<R> {
   fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
      // ...
   }

   fn stream_positon(&mut self) -> io::Result<u64> {
       Ok(self.pos as u64)
   }
}
