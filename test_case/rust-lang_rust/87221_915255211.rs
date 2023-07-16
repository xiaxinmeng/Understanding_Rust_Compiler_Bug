rust
#[cfg(foo)] // expanded first
#[bar] // expanded second
#[cfg(baz)] // expanded third
struct S {
    #[xyz] // expanded fourth
    #[cfg(abc)] // expanded fifth
    field: u8,
}
