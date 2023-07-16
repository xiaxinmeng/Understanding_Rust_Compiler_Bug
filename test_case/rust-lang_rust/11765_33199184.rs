 rs
let (send, receive) = channel();
send(x);
let y = receive();
