
~/Downloads $ rustc test.rs
test.rs:4:5: 4:15 error: attempted access of field `a` on type `Foo`, but no field with that name was found
test.rs:4     Foo::Bar.a;
              ^~~~~~~~~~
error: aborting due to previous error
