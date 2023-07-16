
#[unsafe_destructor]
struct A<'a>;

struct B;

impl <'a> B {
    fn f(x: Vec<A<'a>>) { }
}
fn main() {}
