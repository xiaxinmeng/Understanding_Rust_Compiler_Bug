 rust
> fn eq<E, I>(self, other: I) -> bool where Self: Sized, Self::Item: Eq<E>, I: Iterator<Item = E> { ... }
> 