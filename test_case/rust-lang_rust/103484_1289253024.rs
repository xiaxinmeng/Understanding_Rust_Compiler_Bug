plain
   |
note: the lint level is defined here
  --> src/let_underscore.rs:22:11
   |
10 |    #[warn(let_underscore_drop)]
help: consider binding to an unused variable to avoid immediately dropping the value
   |
15 |     let _unused = SomeStruct;
   |         ~~~~~~~
   |         ~~~~~~~
help: consider immediately dropping the value
   |
15 |     drop(SomeStruct);

warning: 1 warning emitted

Test compiled successfully, but it's marked `compile_fail`.
Test compiled successfully, but it's marked `compile_fail`.

failures:
    src/let_underscore.rs - let_underscore::LET_UNDERSCORE_DROP (line 13)

test result: FAILED. 59 passed; 1 failed; 2 ignored; 0 measured; 0 filtered out; finished in 1.38s

error: doctest failed, to rerun pass `-p rustc_lint --doc`
