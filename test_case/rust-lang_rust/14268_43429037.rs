 rust
#[deriving(Show)]
struct Foo {
    x: int,
    #[deriving(skip)]
    nocopy: ::std::kinds::marker::NoCopy
}
