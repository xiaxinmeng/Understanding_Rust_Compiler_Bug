rust
#[may_ignore]
fn bar() -> Result<usize, usize> { todo!() }

// is this must_use or may_ignore?
fn foo() -> Result<usize, usize> { bar() }
