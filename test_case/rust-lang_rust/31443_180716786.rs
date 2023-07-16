
struct Foo(i32, i32, i32);

fn main() {
    match Foo(1, 2, 3) {
        Foo(a, ..) => { }
    }
}
