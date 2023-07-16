plain
   Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
error[E0277]: the trait bound `test::Crate: Copy` is not satisfied
   --> src/bootstrap/builder/tests.rs:508:33
    |
508 |             first(builder.cache.all::<test::Crate>()),
    |                                 ^^^ the trait `Copy` is not implemented for `test::Crate`

error[E0560]: struct `test::Crate` has no field named `krate`
   --> src/bootstrap/builder/tests.rs:514:17
    |
514 |                 krate: INTERNER.intern_str("std"),
    |                 ^^^^^ `test::Crate` does not have this field
    |
    = note: available fields are: `compiler`, `target`, `mode`, `test_kind`, `krate_name`, `krate_path`
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0277, E0560.
For more information about an error, try `rustc --explain E0277`.
---
expected success, got: exit status: 101


note: failed while building bootstrap::test::Bootstrap
help: to replicate this failure, run `./x.py test src/bootstrap --stage 2`
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
