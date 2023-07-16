rust
enum Ipv6MulticastScope { ... }

impl Ipv6Addr {
    fn is_unicast_link_local(&self) -> bool { ... }

    fn is_unicast_site_local(&self) -> bool { ... }

    fn is_unicast_global(&self) -> bool { ... }

    // other methods...

    fn multicast_scope(&self) -> Ipv6MulticastScope { ... }
}
