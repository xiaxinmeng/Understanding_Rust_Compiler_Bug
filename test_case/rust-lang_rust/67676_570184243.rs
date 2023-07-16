rust
enum Signed {
    Bar = -42,
    Baz,
    Quux = 100,
}
fn signed() -> [i8; 3] {
    let baz = Signed::Baz; // let-expansion changes the MIR significantly
    [Signed::Bar as i8, baz as i8, Signed::Quux as i8]
}
