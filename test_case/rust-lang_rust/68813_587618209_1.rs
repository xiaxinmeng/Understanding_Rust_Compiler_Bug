rust
use tokio::io::{WriteHalf, AsyncWriteExt, Sink};

struct Connection {
    writer: WriteHalf<Sink>,
}

pub async fn setup_outbound() {
    use std::sync::Mutex;

    let us: Mutex<Connection> = unimplemented!();
    us.lock().unwrap().writer.write_all(b"").await;
}
