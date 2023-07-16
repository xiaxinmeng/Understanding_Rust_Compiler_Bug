rust
fn main() {
        let socket = create_bindless_socket().expect("Failed to create socket");
        socket.set_broadcast(true).expect("Failed to enable broadcast");
        socket.send_to(b"TEST", (Ipv4Addr::BROADCAST, 9998)).expect("Failed to send broadcast packet");
}
