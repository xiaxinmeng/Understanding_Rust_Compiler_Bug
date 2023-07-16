
~/Downloads $ rustc test.rs
test.rs:8:5: 8:10 error: attempted access of field `a` on type `Foo`, but no field with that name was found
test.rs:8     foo.a += 1;
              ^~~~~
error: aborting due to previous error
