
use async_trait::async_trait;
use std::sync::{Arc, Weak};

fn main() {
    println!("Hello, world!");
}

pub type MyTraitPtr = Arc<dyn MyTrait>;

#[async_trait]
pub trait MyTrait: Sync {
    async fn foo(self: Arc<Self>) {}
}

pub struct Parent {
    child: MyTraitPtr,
}
