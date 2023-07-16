 rust
// ok
enum X {
    Y(())
}
fn main() {
    let x = X::Y(());
    match &x {
        &X::Y(()) if true => {},
        &X::Y(ref _x)  => {},
    }
}
