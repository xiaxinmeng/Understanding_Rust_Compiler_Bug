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

fn foo<'a, S: Trait + 'a>() {
    future::<'a, S, _>(async move {
        let result: PhantomData<S::Associated<'a>> = PhantomData;
        async {}.await;
    });
}
