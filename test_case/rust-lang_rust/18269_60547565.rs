 rust
pub enum AddrType {
    HostNameAddress(String),
    IpAddress(IpAddr),
}

pub struct SocketAddr {
    pub addr: AddrType,
    pub port: Port,
}
