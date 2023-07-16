rust
impl AsInner<c::in6_addr> for Ipv6Addr {
    const fn as_inner(&self) -> &c::in6_addr {
        &self.inner
    }
}
