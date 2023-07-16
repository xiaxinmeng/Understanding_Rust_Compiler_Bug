rust
fn write_all_vectored(file: &mut File, mut bufs: &mut [IoSlice<'_>]) -> std::io::Result<()> {
    while bufs.len() > 0 {
        match file.write_vectored(bufs) {
            Ok(0) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::WriteZero,
                    "failed to write whole buffer",
                ));
            }
            Ok(n) => bufs = IoSlice::advance(bufs, n),
            Err(ref e) if e.kind() == std::io::ErrorKind::Interrupted => {}
            Err(e) => return Err(e),
        }
    }
    Ok(())
}
