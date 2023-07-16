 Rust
extern mod foo;
use foo::Greeter;

fn main() {
    let g = foo::NumberGreeter { a: 3 };
    g.greet();
}
