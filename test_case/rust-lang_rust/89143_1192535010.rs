rs
use mio::net::TcpStream;
pub trait IoStream: Read + Write + Evented + Send + 'static {}

impl IoStream for TcpStream {}
