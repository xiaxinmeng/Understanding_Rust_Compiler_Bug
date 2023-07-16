rust
pub trait HasItem<'a> {
    type Item;
}

// Does not compile:
fn example1<'a, T: HasItem<'a>>(item: T::Item) -> impl IntoIterator<Item = T::Item> {
    Some(item)
}

// Compiles:
fn example2<'a, T: HasItem<'a>>(item: T::Item) -> impl IntoIterator<Item = <T as HasItem<'a>>::Item> {
    Some(item)
}

fn main() { }
