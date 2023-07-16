rust
#[derive(Trait)] // (1) `Trait` sees `struct S {}` as input
#[my_macro] // (2) `my_macro` sees `struct S { #[cfg(FALSE)] field: u8, }` as input
struct S {
    #[cfg(FALSE)]
    field: u8,
}
