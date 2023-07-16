 rust
fn foo<T: Into<u32>>(t: T) { /* ... */ }

foo(1usize); // not always portable
