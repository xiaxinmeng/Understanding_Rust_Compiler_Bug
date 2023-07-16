 rust
fn main() {
    type Foo = (i32, i32);
    static ON: Foo = (1, 1);
    static mut OFF: Foo = (0, 0);

    match (1, 1) {
        OFF => unreachable!(),
        ON => (),    // error: unreachable pattern [E0001]
        _ => unreachable!()    // error: unreachable pattern [E0001]
    }
}
