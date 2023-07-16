
   Compiling test v0.0.0 (file:///build/src/libtest)
error[E0281]: type mismatch: the type `[closure@src/libtest/lib.rs:1499:40: 1499:51]` implements the trait `std::ops::FnOnce<()>`, but the trait `std::ops::FnOnce<((),)>` is required (expected tuple, found ())
    --> src/libtest/lib.rs:1499:31
     |
1499 |             testfn: DynTestFn(Box::new(move || f())),
     |                               ^^^^^^^^^^^^^^^^^^^^^
     |
     = note: required because of the requirements on the impl of `FnBox<()>` for `[closure@src/libtest/lib.rs:1499:40: 1499:51]`
     = note: required for the cast to the object type `FnBox<()> + 'static`
error[E0281]: type mismatch: the type `[closure@src/libtest/lib.rs:1516:40: 1516:51]` implements the trait `std::ops::FnOnce<()>`, but the trait `std::ops::FnOnce<((),)>` is required (expected tuple, found ())
    --> src/libtest/lib.rs:1516:31
     |
1516 |             testfn: DynTestFn(Box::new(move || f())),
     |                               ^^^^^^^^^^^^^^^^^^^^^
     |
     = note: required because of the requirements on the impl of `FnBox<()>` for `[closure@src/libtest/lib.rs:1516:40: 1516:51]`
     = note: required for the cast to the object type `FnBox<()> + 'static`
error[E0281]: type mismatch: the type `[closure@src/libtest/lib.rs:1535:40: 1535:51]` implements the trait `std::ops::FnOnce<()>`, but the trait `std::ops::FnOnce<((),)>` is required (expected tuple, found ())
    --> src/libtest/lib.rs:1535:31
     |
1535 |             testfn: DynTestFn(Box::new(move || f())),
     |                               ^^^^^^^^^^^^^^^^^^^^^
     |
     = note: required because of the requirements on the impl of `FnBox<()>` for `[closure@src/libtest/lib.rs:1535:40: 1535:51]`
     = note: required for the cast to the object type `FnBox<()> + 'static`
