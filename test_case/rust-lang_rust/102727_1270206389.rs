rust
use std::cmp::min;
use std::io;

const TOTAL_BYTES: usize = 16 * 1024 * 1024;
// runtime increases as the difference between these values increases
const BYTES_PER_READ: usize = 8 * 1024;
const BUFREADER_CAPACITY: usize = 256 * 1024;

fn main() {
    for _ in 0..1_000 {
        let mut reader = io::BufReader::with_capacity(BUFREADER_CAPACITY, Reader(TOTAL_BYTES));
        let count = io::copy(&mut reader, &mut io::sink()).unwrap();
        assert_eq!(count, 16777216)
    }
}

struct Reader(usize);

impl io::Read for Reader {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let len = min(min(buf.len(), self.0), BYTES_PER_READ);
        if len > 0 {
            buf[..len].copy_from_slice(&vec![0; len]);
            self.0 -= len;
        }
        Ok(len)
    }
}
