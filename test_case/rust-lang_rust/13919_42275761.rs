
let mut listener1 = TcpListener::bind("0.0.0.0", 8080);
// vs.
let mut listener2 = TcpListener::bind_addr(SocketAddr{...})
