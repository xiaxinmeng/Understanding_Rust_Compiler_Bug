rust
use std::io::Result;

trait ReadFill
{
    fn read_fill(&mut self, buf: &mut [u8]) -> Result<usize>;
}

impl<T: std::io::Read> ReadFill for T
{
    fn read_fill(&mut self, buf: &mut [u8]) -> Result<usize>
    {
        let mut bytes = 0;
        let mut len = self.read(buf)?;
        bytes += len;
        while len > 0 && buf.len() - len > 0 {
            len = self.read(&mut buf[len..])?;
            bytes += len;
        }
        return Ok(bytes);
    }
}
