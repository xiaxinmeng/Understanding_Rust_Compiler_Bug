rust
#![feature(
    await_macro,
    async_await,
    futures_api,
    optin_builtin_traits
)]
use std::future::Future;

struct Foo;
impl Foo {
    fn foo(&self) -> Option<()> { Some(()) }
}
impl !Sync for Foo{}

async fn bar() {
    let f = Foo;
    if let Some(v) = f.foo() {
        await!(async{})
    }
}

async fn buz(f: impl Future<Output=()> + Send) {
    await!(f)
}

fn main(){
    buz(bar());
}
