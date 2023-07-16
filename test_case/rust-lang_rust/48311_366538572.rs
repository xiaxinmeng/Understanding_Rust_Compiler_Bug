rust
let listener = TcpListener::bind("127.0.0.1:0").unwrap();
let addr = listener.local_addr().unwrap();

thread::spawn(move || {
    listener.accept().unwrap();
});

let socket = TcpStream::connect(addr).unwrap();
match socket.set_read_timeout(Some(Duration::from_secs(0))) {
    Err(ref e) if e.kind() == io::ErrorKind::InvalidInput => {}
    _ => panic!("expected error"),
}
