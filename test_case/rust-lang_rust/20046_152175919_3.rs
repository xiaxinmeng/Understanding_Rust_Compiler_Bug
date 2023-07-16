 rust
// FAILS
enum X {
    Y(())
}
fn main() {
    let x = X::Y(());
    match &x {
        &X::Y(ref _x) if true => {},
        &X::Y(()) if true => {},
        _ => {}
    }
}
