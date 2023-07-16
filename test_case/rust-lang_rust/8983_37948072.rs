 rust
#[feature(managed_boxes)];
fn main() {
    fn f(_: proc:()) {}
    fn eat<T>(_: T) {}

    let x = @1;
    f(proc() { eat(x) });
}
