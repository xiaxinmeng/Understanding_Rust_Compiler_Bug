rust
fn foo() -> impl Debug { None }

fn bar() {
    let mut x = foo();
    x = Some(0);
}
