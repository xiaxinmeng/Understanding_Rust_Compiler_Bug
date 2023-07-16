 rust
extern mod extra;

fn test_foo() {
  let iotask1 = extra::uv_iotask::spawn_iotask(std::task::task());
  let iotask2 = iotask1.clone();
  let (port, chan) = std::comm::stream();

  do std::task::spawn() {
    loop {
      chan.send(());
      extra::timer::sleep(&iotask2, 0);
    }
  }

  port.recv();
  extra::uv_iotask::exit(&iotask1);
}

fn main() {
    test_foo();
}
