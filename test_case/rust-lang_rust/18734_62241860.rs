 rust
#[deriving(Show, PartialEq)]
struct Struct(uint);

#[deriving(Show, PartialEq)]
enum Enum {
    Variant(uint)
}

fn main() {
    let f: fn(uint) -> Struct = Struct;
    let g: fn(uint) -> Enum = Variant;

    assert_eq!(f(42), Struct(42));
    assert_eq!(g(42), Variant(42));
}
