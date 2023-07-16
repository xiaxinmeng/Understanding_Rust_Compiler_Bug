rust
#![feature(async_await)]

trait FirstTrait {}
trait SecondTrait {
    type Item;
}

async fn foo(x: &str) -> impl SecondTrait<Item = dyn FirstTrait> {}
