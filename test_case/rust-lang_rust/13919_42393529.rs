
iotest!(fn listen_ip4_localhost() {
    let socket_addr = next_test_ip4();
    let ip_str = socket_addr.ip.to_str();
    let port = socket_addr.port;
    let listener = TcpListener::from_host_port(ip_str, port);                                                      
    let mut acceptor = listener.listen();

    spawn(proc() {
        let mut stream = TcpStream::from_host_port("localhost", port);
        stream.write([144]).unwrap();
    });

    let mut stream = acceptor.accept();
    let mut buf = [0];
    stream.read(buf).unwrap();
    assert!(buf[0] == 144);
})

iotest!(fn open_localhost() {
    let addr = next_test_ip4();
    let mut acceptor = TcpListener::from_socket_addr(addr).listen();

    spawn(proc() {
        let mut stream = TcpStream::from_host_port("localhost", addr.port);
        stream.write([64]).unwrap();
    });

    let mut stream = acceptor.accept();
    let mut buf = [0];
    stream.read(buf).unwrap();
    assert!(buf[0] == 64);
})
