Rust
fn sigmask(how: libc::c_int, set: Option<sigset_t>) -> sigset_t {
  let mut oldset = unsafe { mem::zeroed::<sigset_t>() };
  if unsafe {
    libc::pthread_sigmask(how,
                          match set {
                            Some(z) => &z,
                            None => panic!(),
                          } as &_, //~ ERROR does not live long enough
                          &mut oldset)
  } < 0 {
    panic!();
  }
  oldset
}
