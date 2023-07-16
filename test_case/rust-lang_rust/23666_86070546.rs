 Rust
fn mktime(&self, secs: u64, nsecs: u64) -> Error<u64> {
    let d_nsecs = nsecs / 1000000;

    if (secs + d_nsecs) < u64::max / 1000 {
         Ok(secs * 1000 + d_nsecs)
    } else {
        Err(some_error_code)
    }
}
