
error[E0277]: can't compare `Val` with `&Val`
   --> /home/duddino/.cargo/registry/src/github.com-1ecc6299db9ec823/datafrog-2.0.1/src/treefrog.rs:379:50
    |
379 |                 slice = gallop(slice, |kv| &kv.1 < v);
    |                                                  ^ no implementation for `Val < &Val` and `Val > &Val`
    |
    = help: the trait `std::cmp::PartialOrd<&Val>` is not implemented for `Val`
    = help: consider adding a `where Val: std::cmp::PartialOrd<&Val>` bound
    = note: required because of the requirements on the impl of `std::cmp::PartialOrd<&&Val>` for `&Val`

    Checking rustc_lexer v0.1.0 (/home/duddino/Desktop/rust/src/librustc_lexer)
    Checking chalk-macros v0.1.0
error[E0277]: can't compare `Val` with `&Val`
   --> /home/duddino/.cargo/registry/src/github.com-1ecc6299db9ec823/datafrog-2.0.1/src/treefrog.rs:464:54
    |
464 |                     slice = gallop(slice, |kv| &kv.1 < v);
    |                                                      ^ no implementation for `Val < &Val` and `Val > &Val`
    |
    = help: the trait `std::cmp::PartialOrd<&Val>` is not implemented for `Val`
    = help: consider adding a `where Val: std::cmp::PartialOrd<&Val>` bound
    = note: required because of the requirements on the impl of `std::cmp::PartialOrd<&&Val>` for `&Val`

    Checking libc v0.2.62
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0277`.
error: could not compile `datafrog`.
warning: build failed, waiting for other jobs to finish...
error: build failed
