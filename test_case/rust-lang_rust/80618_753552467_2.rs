rust
// These compiled with `mixed` defined as above.  The first version fires the lint as a warning.
let _ = mixed::<'static, 'static>;
let _: for<'a> fn(&'a Foo<'static, 'static>) -> Foo<'static, 'static> = mixed;

// Apply the suggested hint:
fn mixed<'a: 'a, 'b: 'b, 'c: 'c>(_: &'c Foo<'a, 'b>) -> Foo<'a, 'b> { todo!() }

// This now fails: expected 3 lifetime arguments
let _ = mixed::<'static, 'static>;

// This now fails: one type is more general than the other
let _: for<'a> fn(&'a Foo<'static, 'static>) -> Foo<'static, 'static> = mixed;
