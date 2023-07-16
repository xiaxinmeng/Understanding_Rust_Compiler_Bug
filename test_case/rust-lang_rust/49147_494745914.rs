rust
// Doesn't implement `Copy`.
struct Foo;
const FOO: Foo = Foo;
fn main() {
    let _five_foos = [FOO; 5];
}
