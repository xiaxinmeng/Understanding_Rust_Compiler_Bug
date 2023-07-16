
type T = impl Sized;
fn take(_: fn() -> T) {}
fn main() {
    take(|| {});
    take(|| {});
}
