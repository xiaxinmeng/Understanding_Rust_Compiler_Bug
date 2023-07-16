rs
#![feature(generic_associated_types)]
use std::{future::Future, marker::PhantomData};

trait Trait {
    type Associated<'a>: Send
    where
        Self: 'a;
}

fn future<'a, S: Trait + 'a, F>(f: F) -> F
where
    F: Future<Output = ()> + Send,
{
    f
}

async fn f<'a, S: Trait + 'a>() {
    let result: PhantomData<S::Associated<'a>> = PhantomData;
    async {}.await;
} 

fn foo<'a, S: Trait + 'a>() {
    future::<'a, S, _>(f::<'a, S>());
}
