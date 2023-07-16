Rust
trait Bug {
    type Item;
    
    const FUN: Self::Item;
}

impl Bug for () {
    existential type Item: Fn() -> Self::Item;
    
    const FUN: fn() -> Self::Item = || { || {} }; // <---- double closure!
}
