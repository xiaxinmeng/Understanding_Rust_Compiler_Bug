 rust
#[test]
fn foo(){
    let socket = UdpSocket::bind("127.0.0.1:5001").ok().expect("fail bind");;//.expect("fail bind");

}
