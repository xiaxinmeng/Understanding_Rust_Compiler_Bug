rust
fn f() {
    g();
}
fn g() {
    f();
}
fn main() {
    f();
}
