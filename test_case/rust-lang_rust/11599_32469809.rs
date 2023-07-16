 rust
fn f(_: &int) {}
fn g(x: int) -> int { x }
fn main() {
    let v = ~[0];
    f(~g(0));
    f(~v[0]);
}
