rust
// where f: Fn(Self::Item, Self::Item) -> Self::Item
iter.next().map(|a| iter.fold(a, f));
