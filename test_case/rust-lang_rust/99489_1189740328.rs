plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 9a7b7d5e50ab0b59c6d349bbf005680a7c880e98 and fb225b2105b323b30571415186fb71d04ff344f5
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
    
    --- stdout
    
    running 17 tests
    test /tmp/mdbook-COe6q9/macros-by-example.md - Macros_By_Example::Dollar_dollar_____ (line 233) ... ignored
    test /tmp/mdbook-COe6q9/macros-by-example.md - Macros_By_Example::Hygiene (line 417) ... ignored
    test /tmp/mdbook-COe6q9/macros-by-example.md - Macros_By_Example::Scoping__Exporting__and_Importing (line 258) ... ignored
    test /tmp/mdbook-COe6q9/macros-by-example.md - Macros_By_Example::Scoping__Exporting__and_Importing::Textual_Scope (line 280) ... ignored
    test /tmp/mdbook-COe6q9/macros-by-example.md - Macros_By_Example::Scoping__Exporting__and_Importing::The_ (line 371) ... ignored
    test /tmp/mdbook-COe6q9/macros-by-example.md - Macros_By_Example::Dollar_dollar_____ (line 204) - compile fail ... ok
    test /tmp/mdbook-COe6q9/macros-by-example.md - Macros_By_Example::Dollar_dollar_____ (line 219) ... FAILED
    test /tmp/mdbook-COe6q9/macros-by-example.md - Macros_By_Example::Transcribing (line 86) - compile fail ... ok
    test /tmp/mdbook-COe6q9/macros-by-example.md - Macros_By_Example::Transcribing (line 63) - compile fail ... ok
    test /tmp/mdbook-COe6q9/macros-by-example.md - Macros_By_Example::Scoping__Exporting__and_Importing::Path_Based_Scope (line 388) ... ok
    test /tmp/mdbook-COe6q9/macros-by-example.md - Macros_By_Example::Transcribing (line 102) ... ok
    test /tmp/mdbook-COe6q9/macros-by-example.md - Macros_By_Example::Scoping__Exporting__and_Importing::The_ (line 350) ... ok
    test /tmp/mdbook-COe6q9/macros-by-example.md - Macros_By_Example::Scoping__Exporting__and_Importing::Textual_Scope (line 331) ... ok
    test /tmp/mdbook-COe6q9/macros-by-example.md - Macros_By_Example::Hygiene (line 482) ... ok
    test /tmp/mdbook-COe6q9/macros-by-example.md - Macros_By_Example::Hygiene (line 459) ... ok
    test /tmp/mdbook-COe6q9/macros-by-example.md - Macros_By_Example::Scoping__Exporting__and_Importing::Textual_Scope (line 303) ... ok
    test /tmp/mdbook-COe6q9/macros-by-example.md - Macros_By_Example::Hygiene (line 442) ... ok
    failures:
    
    
    ---- /tmp/mdbook-COe6q9/macros-by-example.md - Macros_By_Example::Dollar_dollar_____ (line 219) stdout ----
    error[E0658]: meta-variable expressions are unstable
     --> /tmp/mdbook-COe6q9/macros-by-example.md:223:16
    6 |             ( $$( $any:tt )* ) => { $$( $any )* };
      |                ^
      |
      = note: see issue #83527 <https://github.com/rust-lang/rust/issues/83527> for more information
      = note: see issue #83527 <https://github.com/rust-lang/rust/issues/83527> for more information
      = help: add `#![feature(macro_metavar_expr)]` to the crate attributes to enable
    
    error[E0658]: meta-variable expressions are unstable
     --> /tmp/mdbook-COe6q9/macros-by-example.md:223:38
    6 |             ( $$( $any:tt )* ) => { $$( $any )* };
      |                                      ^
      |
      = note: see issue #83527 <https://github.com/rust-lang/rust/issues/83527> for more information
---
    For more information about this error, try `rustc --explain E0658`.
    Couldn't compile the test.
    
    failures:
        /tmp/mdbook-COe6q9/macros-by-example.md - Macros_By_Example::Dollar_dollar_____ (line 219)
    test result: FAILED. 11 passed; 1 failed; 5 ignored; 0 measured; 0 filtered out; finished in 0.29s
    
    
    --- stderr
---
.......... (60/67)
.......    (67/67)


/checkout/src/test/rustdoc-gui/search-input.goml search-input... FAILED
[ERROR] (line 6) Error: Evaluation failed: The following errors happened (for selector `.search-input`): [expected `rgb(224, 224, 224)` for key `border-color`, found `rgba(224, 224, 224, 0.804)`]: for command `assert-css: (".search-input", {"border-color": "rgb(224, 224, 224)"})`
[ERROR] (line 8) Error: Evaluation failed: The following errors happened (for selector `.search-input:focus`): [expected `rgb(75, 169, 243)` for key `border-color`, found `rgba(110, 182, 239, 0.92)`]: for command `assert-css: (".search-input:focus", {"border-color": "rgb(75, 169, 243)"})`
Build completed unsuccessfully in 0:00:25
