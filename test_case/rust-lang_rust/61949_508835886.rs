rust
#![feature(async_await)]

pub trait HasItem<'a> {
    type Item;
}

// Does not compile:
async fn example1<'a, T: HasItem<'a>>(item: T::Item) -> T::Item {
    item
}


fn main() { }
