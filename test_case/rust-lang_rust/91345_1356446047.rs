rust
fn foo() -> Result<Foo, SomeError> { /* ... */ }

fn bar() -> Result<Foo, SomeError> {
  foo().inspect_err(|e| warn!("encountered error {} while performing task xyz", e))
}
