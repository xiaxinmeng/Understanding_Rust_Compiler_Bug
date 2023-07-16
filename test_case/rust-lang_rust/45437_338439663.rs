rust
#![feature(conservative_impl_trait)] 
extern crate futures;

use futures::Future;
use futures::future::ok;

fn future1<'a>(data: &'a i32) -> impl Future<Item=&'a i32, Error=()> + 'a {
    ok(data)
}

fn main() {
    future1(&1).wait().unwrap();
}
