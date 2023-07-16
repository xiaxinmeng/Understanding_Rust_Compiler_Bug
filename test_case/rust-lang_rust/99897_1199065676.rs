plain
    
    --- stdout
    
    running 17 tests
    test /tmp/mdbook-nGoOus/macros-by-example.md - Macros_By_Example::Dollar_dollar_____ (line 233) ... ignored
    test /tmp/mdbook-nGoOus/macros-by-example.md - Macros_By_Example::Hygiene (line 417) ... ignored
    test /tmp/mdbook-nGoOus/macros-by-example.md - Macros_By_Example::Scoping__Exporting__and_Importing (line 258) ... ignored
    test /tmp/mdbook-nGoOus/macros-by-example.md - Macros_By_Example::Scoping__Exporting__and_Importing::Textual_Scope (line 280) ... ignored
    test /tmp/mdbook-nGoOus/macros-by-example.md - Macros_By_Example::Scoping__Exporting__and_Importing::The_ (line 371) ... ignored
    test /tmp/mdbook-nGoOus/macros-by-example.md - Macros_By_Example::Dollar_dollar_____ (line 219) ... FAILED
    test /tmp/mdbook-nGoOus/macros-by-example.md - Macros_By_Example::Dollar_dollar_____ (line 204) - compile fail ... ok
    test /tmp/mdbook-nGoOus/macros-by-example.md - Macros_By_Example::Transcribing (line 63) - compile fail ... ok
    test /tmp/mdbook-nGoOus/macros-by-example.md - Macros_By_Example::Transcribing (line 86) - compile fail ... ok
    test /tmp/mdbook-nGoOus/macros-by-example.md - Macros_By_Example::Hygiene (line 442) ... ok
    test /tmp/mdbook-nGoOus/macros-by-example.md - Macros_By_Example::Hygiene (line 482) ... ok
    test /tmp/mdbook-nGoOus/macros-by-example.md - Macros_By_Example::Scoping__Exporting__and_Importing::The_ (line 350) ... ok
    test /tmp/mdbook-nGoOus/macros-by-example.md - Macros_By_Example::Scoping__Exporting__and_Importing::Textual_Scope (line 303) ... ok
    test /tmp/mdbook-nGoOus/macros-by-example.md - Macros_By_Example::Hygiene (line 459) ... ok
    test /tmp/mdbook-nGoOus/macros-by-example.md - Macros_By_Example::Scoping__Exporting__and_Importing::Path_Based_Scope (line 388) ... ok
    test /tmp/mdbook-nGoOus/macros-by-example.md - Macros_By_Example::Scoping__Exporting__and_Importing::Textual_Scope (line 331) ... ok
    test /tmp/mdbook-nGoOus/macros-by-example.md - Macros_By_Example::Transcribing (line 102) ... ok
    failures:
    
    
    ---- /tmp/mdbook-nGoOus/macros-by-example.md - Macros_By_Example::Dollar_dollar_____ (line 219) stdout ----
    error[E0658]: meta-variable expressions are unstable
     --> /tmp/mdbook-nGoOus/macros-by-example.md:223:16
    6 |             ( $$( $any:tt )* ) => { $$( $any )* };
      |                ^
      |
      = note: see issue #83527 <https://github.com/rust-lang/rust/issues/83527> for more information
      = note: see issue #83527 <https://github.com/rust-lang/rust/issues/83527> for more information
      = help: add `#![feature(macro_metavar_expr)]` to the crate attributes to enable
    
    error[E0658]: meta-variable expressions are unstable
     --> /tmp/mdbook-nGoOus/macros-by-example.md:223:38
    6 |             ( $$( $any:tt )* ) => { $$( $any )* };
      |                                      ^
      |
      = note: see issue #83527 <https://github.com/rust-lang/rust/issues/83527> for more information
---
    For more information about this error, try `rustc --explain E0658`.
    Couldn't compile the test.
    
    failures:
        /tmp/mdbook-nGoOus/macros-by-example.md - Macros_By_Example::Dollar_dollar_____ (line 219)
    test result: FAILED. 11 passed; 1 failed; 5 ignored; 0 measured; 0 filtered out; finished in 0.23s
    
    
    --- stderr
---
This PR updated 'src/doc/reference', verifying if status is 'test-pass'...

We detected that this PR updated 'reference', but its tests failed.

If you do intend to update 'reference', please check the error messages above and
commit another update.

If you do NOT intend to update 'reference', please ensure you did not accidentally
change the submodule at 'src/doc/reference'. You may ask your reviewer for the
Build completed unsuccessfully in 0:00:00
