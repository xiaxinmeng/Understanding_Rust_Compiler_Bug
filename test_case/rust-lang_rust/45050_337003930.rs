rust
struct TypeError;
fn foo() -> Result<(), TypeError> { .. }

fn bar() {
    let TypeError = foo().unwrap_err(); // artificial but wev
}
