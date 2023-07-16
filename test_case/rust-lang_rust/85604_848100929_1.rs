rust 
enum Ipv6UnicastScope {
    LinkLocal,
    Global
}

impl Ipv6Addr {
    fn is_unicast(&self) -> bool;
    fn unicast_scope(&self) -> Option<Ipv6UnicastScope>
}
