rust
struct Session {
    client: websocket::sync::Client<impl websocket::sync::Stream>
}
