rust
#[derive(MyDerive)]
struct Foo {
    #[cfg(FALSE)]
    a: u8,
    #[cfg(not(FALSE)]
    a: u16
}
