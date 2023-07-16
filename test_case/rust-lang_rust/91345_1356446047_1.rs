rust
fn foo() -> Result<(), SomeError> { /* ... */ }

fn bar() {
  foo().inspect_err(|e| warn!("encountered error {} while performing task xyz", e));
}
