 rust
#![feature(conservative_impl_trait)]
extern crate tokio_core;
extern crate futures;
use tokio_core::reactor::{Handle, Core, Timeout};
use futures::{finished, Future};

fn f<'a>(l: Handle, s: &'a str) -> impl Future<Item = (), Error = std::io::Error> {
    Timeout::new(std::time::Duration::from_secs(1), &l).unwrap().and_then(move |_| {
        println!("{}", s);
        finished(())
    })
}

fn main() {
    let s = "foo".to_string();
    let mut l = Core::new().unwrap();
    let h = l.handle();
    struct S<F> {
        s: String,
        f: F
    }
    let s = S {
        s: s,
        f: f(h, &s)
    };
}
