rust
impl<T> ToSocketAddrs for T
where T: Iterator<Item=SocketAddr> + Clone,
{
    // implementation
}
