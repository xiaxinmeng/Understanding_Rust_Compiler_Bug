rust
let stream = TcpStream::connect("104.198.14.52:80")?;
stream.set_keepalive(Some(Duration::from_secs(1)))?;
println!("{:?}", stream.keepalive()?);
