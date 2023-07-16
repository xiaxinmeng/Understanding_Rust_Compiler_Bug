rust
#![feature(trait_alias)]

trait Alias<T: Clone> = where T: Default;

// okay, equivalent to `where T: Clone + Default`
fn clone<T: Alias<T> + Clone>() {}

// ERROR, trait bound `T: Clone` not satisfied
fn default<T: Alias<T> + Default>() {}
