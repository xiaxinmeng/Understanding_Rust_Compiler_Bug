 rust
extern mod other;

fn main() {
    // other::baz(); // fails with a link-time error, *not* a compile time one
    println!("{}", other::foo::mk_foo().x);
}
