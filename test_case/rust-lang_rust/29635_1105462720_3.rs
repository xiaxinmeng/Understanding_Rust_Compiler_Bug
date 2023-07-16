rust
impl<#[downstream] T : ?Sized, A: Allocator> Trait1 for Box<T, A>;

impl<T : ?Sized, #[downstream] A: Allocator> Trait2 for Box<T, A>;
