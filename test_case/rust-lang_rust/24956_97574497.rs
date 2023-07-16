 Rust
struct Foo(bool);
const NEW_FALSE: bool = false;
const STATIC_FOO: Foo = Foo(NEW_FALSE);

pub fn main() {
    match (Foo(false)) {
        STATIC_FOO => 3,
        _ => 11
    };
}
