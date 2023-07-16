rust
#[derive(Trait)]
struct S {
    #[cfg(FALSE)]
    field: u8,
}
