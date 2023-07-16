rust
fn foo() {}
const fn bar() {}

fn main() {
    let _ = foo();
    const A: () = bar();
    let _ = A;
}
