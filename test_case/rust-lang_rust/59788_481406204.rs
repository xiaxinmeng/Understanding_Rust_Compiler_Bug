
#[repr(u8)]
enum Foo {
    Bar = 1,
    Baz = 2,
    #[repr(reserved=Option::None)]
    _ = 0,
    #[repr(reserved)]
    _ = 42..100,
}
