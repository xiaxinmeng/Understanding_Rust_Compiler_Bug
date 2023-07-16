rust
let ports = 1000..1005;
let sock_iter = ports.map(|port| sa4(Ipv4Addr::new(77, 88, 21, 11), port));
let to_sock_addrs = SocketAddrsIter::new(sock_iter); // <- extra expression
UdpSocket::bind(to_sock_addrs);
