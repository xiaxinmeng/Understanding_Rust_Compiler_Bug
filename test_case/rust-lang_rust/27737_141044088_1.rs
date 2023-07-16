 rust
fn eq<I>(self, other: I) -> bool where Self: Sized, Self::Item: Eq<I::Item>, I: Iterator { ... }
