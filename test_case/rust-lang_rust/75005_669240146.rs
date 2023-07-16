rust
#[cfg(target_os = "redox")]
fn max_iov() -> usize { 16 }
