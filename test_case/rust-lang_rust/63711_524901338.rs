rust
use std::net::ToSocketAddrs;
pub struct Config<A>
where
    A: ToSocketAddrs,
{
    addr: A,
}

impl<A> Default for Config<A>
where
    A: ToSocketAddrs,
{
    fn default() -> Self {
        Self {
            addr: "127.0.0.1:1883",
        }
    }
}

fn start<A: ToSocketAddrs>(addr: A) {}
