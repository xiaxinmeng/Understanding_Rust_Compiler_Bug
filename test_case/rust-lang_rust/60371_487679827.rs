Rust
#![feature(existential_type)]

trait Bug {
    type Item: Bug;
    
    const FUN: fn() -> Self::Item;
}

impl Bug for () {
    existential type Item: Bug;
    
    const FUN: fn() -> Self::Item = || {};
}
