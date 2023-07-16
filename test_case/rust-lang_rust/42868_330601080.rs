rust
// Full form
fn foo<'early: 'early, 'late>(arg: &'late u8) { ... }
// Short form
fn foo<'early: 'early>(arg: &u8) { ... }
