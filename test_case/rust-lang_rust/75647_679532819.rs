rust
enum Void { }
fn void_as_usize(x: Void) -> usize {
    match x {}
}
