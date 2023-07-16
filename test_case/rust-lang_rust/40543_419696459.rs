rust
struct Foo<T>(T);
const X: Foo<i32> = Foo(0);
fn main() {
    X.0 += 1;
}
