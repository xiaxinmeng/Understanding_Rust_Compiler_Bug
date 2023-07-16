rust
  // --target x86_64-unknown-linux-gnu
  use sendfd::SendWithFd;
  use std::os::unix::net::UnixDatagram;
  let socket = UnixDatagram::unbound().unwrap();
  let fds = vec![0; u32::MAX as usize / 4 - 4];
  socket.send_with_fd(&[], &fds).unwrap();
  // calls alloc::alloc() with size 0
  // at sendfd::construct_msghdr_for()
  