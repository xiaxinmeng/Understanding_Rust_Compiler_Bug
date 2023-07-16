rust
pub trait UnixStreamExt {
    fn connect_abstract(name: &[u8]) -> io::Result<UnixStream>;
}

pub trait SocketAddrExt {
    fn as_abstract(&self) -> Option<&[u8]>;
}
