 rust
fn main() {
    type Foo = (i32, i32);
    static mut OFF: Foo = (0, 0);

    match (1i32, 1i32) {
        OFF => unreachable!(),
    }
}
