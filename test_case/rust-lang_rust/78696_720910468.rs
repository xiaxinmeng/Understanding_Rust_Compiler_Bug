rust
// Uncomment this and the warning will go away
// use crate::io::Read;

pub mod io {
    pub trait Read {
        fn read(&mut self);
    }
}

pub mod bufreader {
    //! [`crate::TcpStream::read`]
    use crate::io::Read;
}

pub struct TcpStream;

impl crate::io::Read for TcpStream {
    fn read(&mut self) {
    }
}
