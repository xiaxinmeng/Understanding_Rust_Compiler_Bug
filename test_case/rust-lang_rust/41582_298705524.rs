rust
#![feature(test)]
extern crate libc;
extern crate test;

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    use std::net::ToSocketAddrs;

    #[bench]
    fn bench_plain(b: &mut Bencher) {
        let addr = ("google.com", 80);
        b.iter(|| addr.to_socket_addrs().map(|a| a.count()).unwrap_or(0));
    }

    #[bench]
    fn bench_reinit(b: &mut Bencher) {
        let addr = ("google.com", 80);
        b.iter(|| {
                   addr.to_socket_addrs().map(|a| a.count()).unwrap_or(0);
                   unsafe { libc::res_init() };
               });
    }
}
