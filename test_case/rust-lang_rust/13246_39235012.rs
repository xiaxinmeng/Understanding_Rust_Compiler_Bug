 rust
struct Connection {
    conn: BufferedStream<TcpStream>
}

struct Statement<'conn> {
    conn: &'conn Connection
}

struct Rows<'stmt> {
    stmt: &'stmt Statement<'stmt>
}

impl<'stmt> Drop for Rows<'stmt> {
    fn drop(&mut self) {
        self.stmt.conn.conn.write("drop my server side state please".as_bytes());
        self.stmt.conn.conn.flush();
    }
}
