 rust
struct Foo(i32, RangeFull);

fn main() {
    match Foo(i32, ..) {
        Foo(a, ..) => { }
    }
}
