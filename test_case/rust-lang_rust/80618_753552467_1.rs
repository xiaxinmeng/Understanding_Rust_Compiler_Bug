rust
struct Foo<'a, 'b> { a: &'a(), b: &'b () }
// There is an implicit late bound parameter due to &Foo; 'a and 'b are early bound
fn mixed<'a: 'a, 'b: 'b>(_: &Foo<'a, 'b>) -> Foo<'a, 'b> { todo!() }

// This happens to work but no lifetimes can be specified without lint or error
let _ = mixed;

// For example, this triggers the lint as a warning; specifying 1 or 3+ lifetimes triggers an error:
// let _ = mixed::<'static, 'static>;

// type ascription allows specifying the lifetimes without warning or error
let _: for<'a> fn(&'a Foo<'static, 'static>) -> Foo<'static, 'static> = mixed;
