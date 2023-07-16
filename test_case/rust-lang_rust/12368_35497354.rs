 rust
struct ErrorHandleInternals { priv error: Option<IoError> }

pub struct ErrorHandle { priv inner: *mut ErrorHandleInternals }
impl ErrorHandle {
    /// Retrieve any IO error that has occur before this point on the
    /// corresponding `ChompingReader`.
    /// This resets the error state and so an immediate second 
    /// call will return `Ok`.
    pub fn error(&mut self) -> IoResult<()> {
        Err(unsafe {(*self.inner).error.take_unwrap()})
    }
}

pub struct ChompingReader<R> {
    priv inner: R,
    priv error_handle: ~ErrorHandleInternals,

    // can't have concurrent access to `error_handle`
    priv marker: std::kinds::marker::NoSend
}

impl ChompingReader<R> {
    pub fn new(reader: R) -> (ChompingReader<R>, ErrorHandle) { ... }
}

impl<R: Reader> Reader for ChompingReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> IoResult<uint> {
        match self.inner.read(buf) {
            Ok(x) => Ok(x)
            Err(e) => {
                // set an indication that an error happened
                self.error_handle.error = Some(e)
                Err(/* substitute an EOF error */)
            }
        }
    }
}
