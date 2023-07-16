 rust
struct Foo { x: [Foo, .. 0] } // possibly ok, size is 0.
