rust
// heterogeneous PartialEq
impl PartialEq<SocketAddr> for SocketAddrV4 {...}
impl PartialEq<SocketAddr> for SocketAddrV6 {...}
impl PartialEq<SocketAddrV4> for SocketAddr {...}
impl PartialEq<SocketAddrV6> for SocketAddr {...}

// Ord
impl Ord for SocketAddr {...}
impl Ord for SocketAddrV4 {...}
impl Ord for SocketAddrV6 {...}

// homogeneous PartialOrd
impl PartialOrd<SocketAddr> for SocketAddr {...}
impl PartialOrd<SocketAddrV4> for SocketAddrV4 {...}
impl PartialOrd<SocketAddrV6> for SocketAddrV6 {...}

// heterogeneous PartialOrd
impl PartialOrd<SocketAddr> for SocketAddrV4 {...}
impl PartialOrd<SocketAddr> for SocketAddrV6 {...}
impl PartialOrd<SocketAddrV4> for SocketAddr {...}
impl PartialOrd<SocketAddrV6> for SocketAddr {...}
