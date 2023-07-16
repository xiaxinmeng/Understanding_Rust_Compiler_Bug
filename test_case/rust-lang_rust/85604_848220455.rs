rust
impl Ipv6Addr {
    fn is_unicast(&self) -> bool;
    fn is_unicast_global(&self) -> bool;
    fn is_unicast_link_local(&self) -> bool;
}
