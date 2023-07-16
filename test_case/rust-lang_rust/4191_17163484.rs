
fn test_foo() {
  let iotask1 = std::uv_iotask::spawn_iotask(task::task());
  let iotask2 = iotask1.clone();
  let (port, chan) = comm::stream();

  do task::spawn() {
    loop {
      chan.send(());
      std::timer::sleep(&iotask2, 0);
    }
  }

  port.recv();
  std::uv_iotask::exit(&iotask1);
}
