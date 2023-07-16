rust
async fn example1<'a, T: HasItem<'a>>(item: T::Item) -> <T as HasItem<'a>>::Item {
    item
}
