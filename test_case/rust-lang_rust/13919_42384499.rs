
let mut listener = TcpListener::listen("0.0.0.0", 8080);
let mut acceptor = listener.listen(); // listen *again*?
// ...
