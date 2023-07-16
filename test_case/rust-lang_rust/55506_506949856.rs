rust
pub struct Box<#[rustc_propagate_must_use] T: ?Sized>(Unique<T>);
