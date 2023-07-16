rust
#[derive(Trait)]
#[my_macro]
struct S {
    #[cfg(FALSE)]
    field: u8,
}
