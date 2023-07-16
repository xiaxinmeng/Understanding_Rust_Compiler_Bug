rust
#[may_ignore]
fn bar() -> Result<usize, usize> { todo!() }

fn foo() -> Result<usize, usize> { bar() } // not may_ignore (so, must_use)
