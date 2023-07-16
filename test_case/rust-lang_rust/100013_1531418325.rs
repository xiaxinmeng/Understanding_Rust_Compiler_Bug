rust
#![feature(return_position_impl_trait_in_trait)]

use std::future::Future;

pub trait Foo: Sync {
    fn run<'a, 'b: 'a, T: Sync>(&'a self, _: &'b T) -> impl Future<Output = ()> + 'a + Send; // doesn't compile
    // fn run<'a, 'b, T: Sync>(&'a self, _: &'b T) -> impl Future<Output = ()> + 'a + Send; // compiles
}

pub trait FooExt: Foo {
    fn run_via<'a, 'b: 'a, T: Sync>(&'a self, t: &'b T) -> impl Future<Output = ()> + 'a + Send {
        async move {
            // asks for an unspecified lifetime to outlive itself? weird diagnostics
            self.run(t).await;
        }
    }
}
