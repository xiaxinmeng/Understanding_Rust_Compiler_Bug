rust
let socket = UdpSocket::bind("0.0.0.0:0")?;
socket.send_to(data, "127.0.0.1:456")?;
