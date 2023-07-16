 rust
struct Foo {
    a: int,
    b: ~str
}
impl_debug_show!(Foo)

fn foos() -> ~[Foo] {
    ~[Foo { a: 3, b: ~"test" }, Foo { a: 4, b: ~"hello" }]
}

fn main() {
    let x = foos();
    println!("{}", x);
}
