 rust
#[deriving(ToStr)]
struct Foo<A>(A);

struct Bar;
impl ToStr for Bar {
    fn to_str(&self) -> ~str {
        ~"some string"
    }
}

fn main() {
    println(Bar.to_str());
    println(Foo(Bar).to_str());
    println(fmt!("%?", Foo(Bar)));
}
