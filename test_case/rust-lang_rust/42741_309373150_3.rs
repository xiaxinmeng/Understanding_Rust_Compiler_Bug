rust
extern crate a;
extern crate b;

struct C();

impl b::B for C {
    fn a(a: a::A) { }
    fn sub(sub: a::Sub) { }
}
