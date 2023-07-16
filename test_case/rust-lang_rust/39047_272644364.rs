
fn sigmask(how: libc::c_int, set: Option<sigset_t>) -> sigset_t {
  let mut oldset = unsafe { mem::zeroed::<sigset_t>() };
  let z = match set {
            Some(z) => &z,
            None => panic!(),
          };
  if unsafe {
    libc::pthread_sigmask(how,
                          z,
                          &mut oldset)
  } < 0 {
    panic!();
  }
  oldset
}
