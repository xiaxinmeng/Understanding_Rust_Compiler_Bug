 sh
> rustc foo.rs
foo.rs:8:5: 8:6 error: unable to infer enough type information about `_`; type annotations required [E0282]
foo.rs:8     f(Foo(PhantomData));
             ^
error: aborting due to previous error
