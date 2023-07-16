 rust
// FAILS
enum X {
    Y(())
}
fn main() {
    let x = X::Y(());
    match (true, &x) {
        (true, &X::Y(ref _x))  => {},
        (_, &X::Y(())) => {},
    }
}
