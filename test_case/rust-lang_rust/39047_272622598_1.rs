Rust
fn sigmask(how: libc::c_int, set: Option<sigset_t>) -> sigset_t {
  let mut oldset = unsafe { mem::zeroed::<sigset_t>() };
  if unsafe {
    let sigmask = match set {
        Some(z) => z,
        None => panic!(),
    };
    libc::pthread_sigmask(how,
                          &sigmask, // works
                          &mut oldset)
  } < 0 {
    panic!();
  }
  oldset
}
