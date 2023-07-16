rust
fn foo() -> Result<Result<(), impl Error>, impl Error> { ... }

fn foobar() -> anyhow::Result<()> {
  foo().flatten_into()?
}
