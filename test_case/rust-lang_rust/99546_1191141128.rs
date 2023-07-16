plain
    
    --- stdout
    
    running 17 tests
    test /tmp/mdbook-bfYDjo/macros-by-example.md - Macros_By_Example::Dollar_dollar_____ (line 233) ... ignored
    test /tmp/mdbook-bfYDjo/macros-by-example.md - Macros_By_Example::Hygiene (line 417) ... ignored
    test /tmp/mdbook-bfYDjo/macros-by-example.md - Macros_By_Example::Scoping__Exporting__and_Importing (line 258) ... ignored
    test /tmp/mdbook-bfYDjo/macros-by-example.md - Macros_By_Example::Scoping__Exporting__and_Importing::Textual_Scope (line 280) ... ignored
    test /tmp/mdbook-bfYDjo/macros-by-example.md - Macros_By_Example::Scoping__Exporting__and_Importing::The_ (line 371) ... ignored
    test /tmp/mdbook-bfYDjo/macros-by-example.md - Macros_By_Example::Dollar_dollar_____ (line 219) ... FAILED
    test /tmp/mdbook-bfYDjo/macros-by-example.md - Macros_By_Example::Dollar_dollar_____ (line 204) - compile fail ... ok
    test /tmp/mdbook-bfYDjo/macros-by-example.md - Macros_By_Example::Transcribing (line 63) - compile fail ... ok
    test /tmp/mdbook-bfYDjo/macros-by-example.md - Macros_By_Example::Transcribing (line 86) - compile fail ... ok
    test /tmp/mdbook-bfYDjo/macros-by-example.md - Macros_By_Example::Transcribing (line 102) ... ok
    test /tmp/mdbook-bfYDjo/macros-by-example.md - Macros_By_Example::Scoping__Exporting__and_Importing::Textual_Scope (line 303) ... ok
    test /tmp/mdbook-bfYDjo/macros-by-example.md - Macros_By_Example::Hygiene (line 482) ... ok
    test /tmp/mdbook-bfYDjo/macros-by-example.md - Macros_By_Example::Hygiene (line 442) ... ok
    test /tmp/mdbook-bfYDjo/macros-by-example.md - Macros_By_Example::Scoping__Exporting__and_Importing::Textual_Scope (line 331) ... ok
    test /tmp/mdbook-bfYDjo/macros-by-example.md - Macros_By_Example::Hygiene (line 459) ... ok
    test /tmp/mdbook-bfYDjo/macros-by-example.md - Macros_By_Example::Scoping__Exporting__and_Importing::The_ (line 350) ... ok
    test /tmp/mdbook-bfYDjo/macros-by-example.md - Macros_By_Example::Scoping__Exporting__and_Importing::Path_Based_Scope (line 388) ... ok
    failures:
    
    
    ---- /tmp/mdbook-bfYDjo/macros-by-example.md - Macros_By_Example::Dollar_dollar_____ (line 219) stdout ----
    error[E0658]: meta-variable expressions are unstable
     --> /tmp/mdbook-bfYDjo/macros-by-example.md:223:16
    6 |             ( $$( $any:tt )* ) => { $$( $any )* };
      |                ^
      |
      = note: see issue #83527 <https://github.com/rust-lang/rust/issues/83527> for more information
      = note: see issue #83527 <https://github.com/rust-lang/rust/issues/83527> for more information
      = help: add `#![feature(macro_metavar_expr)]` to the crate attributes to enable
    
    error[E0658]: meta-variable expressions are unstable
     --> /tmp/mdbook-bfYDjo/macros-by-example.md:223:38
    6 |             ( $$( $any:tt )* ) => { $$( $any )* };
      |                                      ^
      |
      = note: see issue #83527 <https://github.com/rust-lang/rust/issues/83527> for more information
---
    For more information about this error, try `rustc --explain E0658`.
    Couldn't compile the test.
    
    failures:
        /tmp/mdbook-bfYDjo/macros-by-example.md - Macros_By_Example::Dollar_dollar_____ (line 219)
    test result: FAILED. 11 passed; 1 failed; 5 ignored; 0 measured; 0 filtered out; finished in 0.24s
    
    
    --- stderr
---

 error[E0412]: cannot find type `PhantomData` in this scope
   --> $DIR/ice-6252.rs:8:9
    |
 LL |     _n: PhantomData,
    |
 help: consider importing one of these items
    |
 LL | use core::marker::PhantomData;
 LL | use core::marker::PhantomData;
    |
 LL | use serde::__private::PhantomData;
 LL | use std::marker::PhantomData;
    |
 
 error[E0412]: cannot find type `VAL` in this scope
 error[E0412]: cannot find type `VAL` in this scope
   --> $DIR/ice-6252.rs:10:63
    |
 LL | impl<N, M> TypeVal<usize> for Multiply<N, M> where N: TypeVal<VAL> {}
    |          -                                                    ^^^ not found in this scope
    |          |
    |          help: you might be missing a type parameter: `, VAL`
 error[E0046]: not all trait items implemented, missing: `VAL`
   --> $DIR/ice-6252.rs:10:1
    |
 LL |     const VAL: T;
 LL |     const VAL: T;
    |     ------------ `VAL` from trait
 ...
 LL | impl<N, M> TypeVal<usize> for Multiply<N, M> where N: TypeVal<VAL> {}
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `VAL` in implementation
-error: constant expression depends on a generic parameter
-  --> $DIR/ice-6252.rs:13:9
-   |
-   |
-LL |     [1; <Multiply<Five, Five>>::VAL];
-   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^
-   |
-   = note: this may fail depending on what value the parameter takes
-error: aborting due to 4 previous errors
+error: aborting due to 3 previous errors
 
 Some errors have detailed explanations: E0046, E0412.
---
To only update this specific test, also pass `--test-args crashes/ice-6252.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/crashes/ice-6252.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/crashes/ice-6252.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-71205fa4273edf27.so" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-7d13fa063f867ef0.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-0c795f7a8756f15a.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-dcc59fbd39a40970.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-46430df947b7dad0.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-ff530e80824c3a36.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-b92911696ae4394a.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-61ecef333190a996.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-cdd893c121eb00e4.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-021aec868151835c.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-aceff80e643e9fe7.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-5411643be5ff72c2.so" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/crashes/ice-6252.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: test failed, to rerun pass '--test compile-test'
{"message":"cannot find type `PhantomData` in this scope","code":{"code":"E0412","explanation":"A used type name is not in scope.\n\nErroneous code examples:\n\n