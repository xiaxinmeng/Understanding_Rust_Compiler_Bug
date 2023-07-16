rust
fn from_abstract_name<N>(name: &N) -> Result<SocketAddr>
where
    N: AsRef<[u8]>;
