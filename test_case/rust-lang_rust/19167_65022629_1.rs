 rust
eq.rs:22:5: 22:15 error: overflow evaluating the trait `Sized` for the type `_`
eq.rs:22     Foo == Bar;
             ^~~~~~~~~~
eq.rs:22:5: 22:15 error: overflow evaluating the trait `Sized` for the type `Foo`
eq.rs:22     Foo == Bar;
             ^~~~~~~~~~
eq.rs:22:5: 22:15 error: overflow evaluating the trait `PartialEq<_>` for the type `Foo`
eq.rs:22     Foo == Bar;
             ^~~~~~~~~~
error: aborting due to 3 previous errors
