
trait Thing<T: Thing> {
    fn make_vanilla_thing() -> T;
}

struct A;
struct B;

impl Thing<B> for A {
    fn make_vanilla_thing() -> B { B }
}

impl Thing<B> for B {
    fn make_vanilla_thing() -> B { B }
}
