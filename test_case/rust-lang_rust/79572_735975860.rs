rust
#[must_use]
fn bar() -> usize { todo!() }

fn foo() -> usize { bar() } // not must_use
