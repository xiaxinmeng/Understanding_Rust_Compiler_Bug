
// This returns the order we want because s_addr is stored in big-endian.
self.inner.s_addr.to_ne_bytes()
