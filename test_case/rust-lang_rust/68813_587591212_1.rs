rust
use tokio::net::TcpStream;
use tokio::io::{self, AsyncWriteExt};

use std::sync::{Arc, Mutex};

pub struct Connection {
	writer: Option<io::WriteHalf<TcpStream>>,
}
impl Connection {
	pub async fn setup_outbound() {
		let us: Arc<Mutex<Self>> = unimplemented!();
		us.lock().unwrap().writer.as_mut().unwrap().write_all(b"hi").await;
	}
}
