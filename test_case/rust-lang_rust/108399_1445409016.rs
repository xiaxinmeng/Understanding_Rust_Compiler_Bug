rust
#![feature(type_alias_impl_trait)]

use std::future::Future;

type Opaque = impl Future;

async fn get_rpit() {}

fn query(_: impl FnOnce() -> Opaque) {}

fn test() {
    query(get_rpit);
}
