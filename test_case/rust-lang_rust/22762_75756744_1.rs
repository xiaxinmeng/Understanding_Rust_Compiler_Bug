
pub fn recv(&mut self) -> Result<T, Failure> {
    // This code is essentially the exact same as that found in the stream
    // case (see stream.rs)
    match self.try_recv() {
        Err(Empty) => {}
        data => return data,
    }

    let (wait_token, signal_token) = blocking::tokens();
