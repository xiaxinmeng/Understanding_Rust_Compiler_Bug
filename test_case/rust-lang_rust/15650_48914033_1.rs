 rust
fn main() {
    type Foo = (i32, i32);
    static ON: Foo = (1, 1);
    static mut OFF: Foo = (0, 0);

    // error: cannot determine a type for this expression: cannot determine the type of this integer
    match (1, 1) {
        OFF => unreachable!(),
    }
}
