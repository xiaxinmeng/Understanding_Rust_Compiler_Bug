
error[E0281]: type mismatch: the type `[closure@src/tools/compiletest/src/main.rs:467:30: 469:6 config:_, testpaths:_]` implements the trait `std::ops::FnOnce<()>`, but the trait `std::ops::FnOnce<((),)>` is required (expected tuple, found ())
   --> src/tools/compiletest/src/main.rs:467:21
    |
467 |     test::DynTestFn(Box::new(move || {
    |                     ^
    |
    = note: required because of the requirements on the impl of `test::FnBox<()>` for `[closure@src/tools/compiletest/src/main.rs:467:30: 469:6 config:_, testpaths:_]`
    = note: required for the cast to the object type `test::FnBox<()> + 'static`

error: aborting due to previous error
