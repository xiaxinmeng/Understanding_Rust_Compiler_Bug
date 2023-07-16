 rust
struct Foo;
struct g;
impl Foo { fn T<A>(&self); {} }

fn main() {
    let f = Foo;
    f.T::<g>();
}
