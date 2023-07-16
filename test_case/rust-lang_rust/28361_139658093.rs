 rust
fn f(i: i32) -> i32 { i }
fn main() {
    let mut b: bool;
    let mut i: i32;
    1 == {f}(1); // error: expected one of `.`, `;`, `}`, or an operator, found `(`
    i = {f}(1); // OK
    b = 1 == {f}(1); // OK
}
