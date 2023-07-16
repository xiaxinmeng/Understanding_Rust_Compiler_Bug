 rust
extern mod issue10028;
use issue10028::ZeroLengthThingWithDestructor;

struct Foo {
    zero_length_thing: ZeroLengthThingWithDestructor
}

fn make_foo() -> Foo {
    Foo { zero_length_thing: ZeroLengthThingWithDestructor::new() }
}

fn main() {
    let _f:Foo = make_foo();
}
