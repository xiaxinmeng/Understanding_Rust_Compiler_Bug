rust
pub const fn is_something(&self) -> bool {
    // IPv6 checking
    self.octets()[0] == IPv6::SOMETHING.octets()[0] ||
    // IPv4 checking
    if let Some(ipv4_addr) = self.to_ipv4() { ipv4_addr.is_something() } else { false }
}
