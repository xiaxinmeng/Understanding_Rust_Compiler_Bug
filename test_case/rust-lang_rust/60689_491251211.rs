rust
pub fn send_file<R: AsRawFd, W: AsRawFd>(r: &mut R, w: &mut W) -> io::Result<u64> {...}
