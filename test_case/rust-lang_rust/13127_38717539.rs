 rust
fn read(&mut self, buf: &mut [u8]) -> IoResult<uint>;
fn read_at_least(&mut self, buf: &mut [u8], amt: uint) -> IoResult<uint>;
fn read_exact(&mut self, buf: &mut [u8]) -> IoResult<()>;
