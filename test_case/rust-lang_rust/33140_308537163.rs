rust
fn foo(x: Box<Send+Sync>) -> Box<Sync+Send> { x }
