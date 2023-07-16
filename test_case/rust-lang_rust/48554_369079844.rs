rust
struct A;
struct B;
fn a(_: A) {}
fn main() {
    a({B});
}
