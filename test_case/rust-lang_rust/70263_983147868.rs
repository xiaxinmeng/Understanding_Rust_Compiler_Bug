rust
use std::future::Future;
use std::ops::FnOnce;

trait Foo<'a, Arg: 'a, Ret>: FnOnce(&'a mut Arg) -> Self::Fut {
    type Fut: Future<Output = Ret> + 'a;
}

impl<'a, Arg: 'a, Ret, Fut: Future<Output = Ret> + 'a, Fn: FnOnce(&'a mut Arg) -> Fut>
    Foo<'a, Arg, Ret> for Fn
{
    type Fut = Fut;
}

struct A;

impl A {
    async fn test(&self) {}
}

async fn run_with_ctx<Ret, Fn>(a: Fn) -> Ret
where
    Fn: for<'a> Foo<'a, A, Ret>,
{
    a(&mut A).await
}

fn main() {
    // Closures don't work, however async functions do work.
    run_with_ctx(|arg: &mut A| async move {
        arg.test().await;
    });
}

