rust
let stream : tokio::net::TcpStream =  listener.accpt().await?;
let stream: std::net::TcpStream = stream.into_std().unwrap();
let socket: socket2::Socket = socket2::Socket::from(stream);
let keepalive = TcpKeepalive::new()
                .with_time(Duration::from_secs(4))
                .with_interval(Duration::from_secs(1))
                .with_retries(4);
socket.set_tcp_keepalive(&keepalive)?;
let stream: std::net::TcpStream = socket.into();
let stream: tokio::net::TcpStream = tokio::net::TcpStream::from_std(stream).unwrap();
