rust
fn scope<'env>(f: impl for<'scope where 'env: 'scope> FnOnce(&'scope Scope<'scope>)
