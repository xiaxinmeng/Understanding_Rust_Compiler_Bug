 rust
struct Foo(i32);

type Bar = Foo;

fn main() {
    let a = Bar(42); //~ ERROR  unresolved name `Bar`
}
