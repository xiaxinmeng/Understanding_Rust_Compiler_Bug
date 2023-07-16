rust
#![feature(async_fn_in_trait)]
#![feature(return_position_impl_trait_in_trait)]
use core::future::Future;

pub trait Trait {
    fn foo(&self) -> impl Future<Output = ()>;
}

pub struct S;

impl Trait for S {
    async fn foo(&self) -> () {}
}
