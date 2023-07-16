rust
fn foo() -> impl Iterator<Item = impl core::fmt::Debug> { 0..10 }
