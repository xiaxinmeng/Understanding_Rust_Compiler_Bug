rust
// Short form
fn foo<'early: 'early>(arg: &u8) { ... } // `foo::<'static>` is okay
// Full desugared form
for<'late> fn foo<'early: 'early>(arg: &'late u8) { ... } // `foo::<'static>` is still okay
