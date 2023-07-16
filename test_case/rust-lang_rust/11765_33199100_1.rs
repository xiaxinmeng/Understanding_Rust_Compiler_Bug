 rust
let (sender, receiver) = channel();
sender.send(x);
let y = receiver.recv();
