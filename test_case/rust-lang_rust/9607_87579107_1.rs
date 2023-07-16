
test.rs:15:5: 15:14 error: the trait `Bar` is not implemented for the type `T` [E0277]
test.rs:15     x: Foo<T>
               ^~~~~~~~~
test.rs:13:10: 13:15 note: in expansion of #[derive_Clone]
test.rs:13:10: 13:15 note: expansion site
error: aborting due to previous error
