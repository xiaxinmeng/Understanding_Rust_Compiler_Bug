rust
pub fn read(&self, offset: usize) -> Result<[u8; 4], std::io::Error> {
  self.file.seek(SeekFrom::Start(offset as u64))?;
  let mut buffer = [0; 4];
  self.file.read_exact(&mut buffer)?;
  Ok(buffer)
}
