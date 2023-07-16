rust
struct A {}
struct B {}

fn takes_cb(f: fn(A)) {}

fn main() {
    fn callback(x: B) {}
    takes_cb(callback)
}
