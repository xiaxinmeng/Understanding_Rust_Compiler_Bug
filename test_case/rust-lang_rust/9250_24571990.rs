 rust
#[deriving(FromPrimitive)]
struct Foo { x: int }

#[deriving(FromPrimitive)]
enum Bar { A(int), B(uint) }

#[deriving(FromPrimitive)]
enum Baz { S { x: int } }
