rust
#![feature(async_fn_in_trait)]

trait A {
    async fn uwu(x: ());
}

impl A for () {
    async fn uwu(x: u8) {}
}
