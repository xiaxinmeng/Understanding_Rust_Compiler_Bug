Rust
struct Foo;
impl Tr for Foo {
    fn frobnicate<A: Allocator+?Sized>(self: RcWithAllocator<Self, A>) { /* ... */ }
}
