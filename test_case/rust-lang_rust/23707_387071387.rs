rust
extern crate tokio_core;
extern crate tokio_io;

use std::{borrow::Borrow, rc::Rc};
use tokio_core::net::TcpStream;
use tokio_io::io::read_exact;

fn read_one(conn: Rc<TcpStream>) {
    read_exact(conn.borrow(), [0u8]);
}
