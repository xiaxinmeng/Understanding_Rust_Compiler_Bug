rust
fn foo() -> Option<impl Future> { ... }

fn bar() {
   foo();
}
