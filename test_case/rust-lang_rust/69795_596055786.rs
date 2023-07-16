plain
2020-03-07T06:22:49.6263984Z [TIMING] BookTest { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, path: "src/doc/book", name: "book", is_ext_doc: true } -- 61.935
2020-03-07T06:22:49.6268501Z Testing rustbook src/doc/nomicon
2020-03-07T06:22:56.3445310Z Error: Rustdoc returned an error: 
2020-03-07T06:22:56.3445590Z running 6 tests
2020-03-07T06:22:56.3446452Z test /tmp/mdbook-y11Qm8/destructors.md - Destructors (line 128) ... FAILED
2020-03-07T06:22:56.3447003Z test /tmp/mdbook-y11Qm8/destructors.md - Destructors (line 28) ... FAILED
2020-03-07T06:22:56.3447826Z test /tmp/mdbook-y11Qm8/destructors.md - Destructors (line 110) ... ok
2020-03-07T06:22:56.3448733Z test /tmp/mdbook-y11Qm8/destructors.md - Destructors (line 6) ... ignored
2020-03-07T06:22:56.3449316Z test /tmp/mdbook-y11Qm8/destructors.md - Destructors (line 55) ... FAILED
2020-03-07T06:22:56.3449873Z test /tmp/mdbook-y11Qm8/destructors.md - Destructors (line 96) ... ok
2020-03-07T06:22:56.3450257Z failures:
2020-03-07T06:22:56.3450374Z 
2020-03-07T06:22:56.3450374Z 
2020-03-07T06:22:56.3450866Z ---- /tmp/mdbook-y11Qm8/destructors.md - Destructors (line 128) stdout ----
2020-03-07T06:22:56.3451238Z error[E0432]: unresolved import `std::alloc::Alloc`
2020-03-07T06:22:56.3452212Z  --> /tmp/mdbook-y11Qm8/destructors.md:131:18
2020-03-07T06:22:56.3452614Z 4 | use std::alloc::{Alloc, GlobalAlloc, Global, Layout};
2020-03-07T06:22:56.3452860Z   |                  ^^^^^
2020-03-07T06:22:56.3453033Z   |                  |
2020-03-07T06:22:56.3453243Z   |                  no `Alloc` in `alloc`
2020-03-07T06:22:56.3453243Z   |                  no `Alloc` in `alloc`
2020-03-07T06:22:56.3453528Z   |                  help: a similar name exists in the module: `alloc`
2020-03-07T06:22:56.3453933Z 
2020-03-07T06:22:56.3454212Z error[E0599]: no method named `dealloc` found for struct `std::alloc::Global` in the current scope
2020-03-07T06:22:56.3454763Z   --> /tmp/mdbook-y11Qm8/destructors.md:142:20
2020-03-07T06:22:56.3455390Z 15 |             Global.dealloc(c.cast(), Layout::new::<T>());
2020-03-07T06:22:56.3455763Z    |                    ^^^^^^^ method not found in `std::alloc::Global`
2020-03-07T06:22:56.3456035Z    |
2020-03-07T06:22:56.3456282Z    = help: items from traits can only be used if the trait is in scope
2020-03-07T06:22:56.3456282Z    = help: items from traits can only be used if the trait is in scope
2020-03-07T06:22:56.3456644Z help: the following trait is implemented but not in scope; perhaps add a `use` for it:
2020-03-07T06:22:56.3456913Z    |
2020-03-07T06:22:56.3457085Z 4  | use std::alloc::AllocRef;
2020-03-07T06:22:56.3457278Z    |
2020-03-07T06:22:56.3457375Z 
2020-03-07T06:22:56.3457862Z error[E0599]: no method named `dealloc` found for struct `std::alloc::Global` in the current scope
2020-03-07T06:22:56.3459129Z   --> /tmp/mdbook-y11Qm8/destructors.md:157:20
2020-03-07T06:22:56.3459828Z 30 |             Global.dealloc(c.cast(), Layout::new::<T>());
2020-03-07T06:22:56.3460444Z    |                    ^^^^^^^ method not found in `std::alloc::Global`
2020-03-07T06:22:56.3460745Z    |
2020-03-07T06:22:56.3461033Z    = help: items from traits can only be used if the trait is in scope
---
2020-03-07T06:22:56.3463404Z 
2020-03-07T06:22:56.3463625Z Some errors have detailed explanations: E0432, E0599.
2020-03-07T06:22:56.3464140Z For more information about an error, try `rustc --explain E0432`.
2020-03-07T06:22:56.3464544Z Couldn't compile the test.
2020-03-07T06:22:56.3464994Z ---- /tmp/mdbook-y11Qm8/destructors.md - Destructors (line 28) stdout ----
2020-03-07T06:22:56.3465318Z error[E0432]: unresolved import `std::alloc::Alloc`
2020-03-07T06:22:56.3465767Z  --> /tmp/mdbook-y11Qm8/destructors.md:31:18
2020-03-07T06:22:56.3467202Z 4 | use std::alloc::{Alloc, Global, GlobalAlloc, Layout};
2020-03-07T06:22:56.3467466Z   |                  ^^^^^
2020-03-07T06:22:56.3467848Z   |                  |
2020-03-07T06:22:56.3468059Z   |                  no `Alloc` in `alloc`
2020-03-07T06:22:56.3468059Z   |                  no `Alloc` in `alloc`
2020-03-07T06:22:56.3468786Z   |                  help: a similar name exists in the module: `alloc`
2020-03-07T06:22:56.3469033Z 
2020-03-07T06:22:56.3469368Z error[E0599]: no method named `dealloc` found for struct `std::alloc::Global` in the current scope
2020-03-07T06:22:56.3470036Z   --> /tmp/mdbook-y11Qm8/destructors.md:42:20
2020-03-07T06:22:56.3470538Z 15 |             Global.dealloc(c.cast(), Layout::new::<T>())
2020-03-07T06:22:56.3471101Z    |                    ^^^^^^^ method not found in `std::alloc::Global`
2020-03-07T06:22:56.3471388Z    |
2020-03-07T06:22:56.3471998Z    = help: items from traits can only be used if the trait is in scope
---
2020-03-07T06:22:56.3473581Z 
2020-03-07T06:22:56.3473780Z Some errors have detailed explanations: E0432, E0599.
2020-03-07T06:22:56.3474283Z For more information about an error, try `rustc --explain E0432`.
2020-03-07T06:22:56.3474660Z Couldn't compile the test.
2020-03-07T06:22:56.3475100Z ---- /tmp/mdbook-y11Qm8/destructors.md - Destructors (line 55) stdout ----
2020-03-07T06:22:56.3475412Z error[E0432]: unresolved import `std::alloc::Alloc`
2020-03-07T06:22:56.3476201Z  --> /tmp/mdbook-y11Qm8/destructors.md:58:18
2020-03-07T06:22:56.3476615Z 4 | use std::alloc::{Alloc, Global, GlobalAlloc, Layout};
2020-03-07T06:22:56.3476867Z   |                  ^^^^^
2020-03-07T06:22:56.3477043Z   |                  |
2020-03-07T06:22:56.3477262Z   |                  no `Alloc` in `alloc`
2020-03-07T06:22:56.3477262Z   |                  no `Alloc` in `alloc`
2020-03-07T06:22:56.3477561Z   |                  help: a similar name exists in the module: `alloc`
2020-03-07T06:22:56.3477974Z 
2020-03-07T06:22:56.3478445Z error[E0599]: no method named `dealloc` found for struct `std::alloc::Global` in the current scope
2020-03-07T06:22:56.3479043Z   --> /tmp/mdbook-y11Qm8/destructors.md:69:20
2020-03-07T06:22:56.3479529Z 15 |             Global.dealloc(c.cast(), Layout::new::<T>());
2020-03-07T06:22:56.3479918Z    |                    ^^^^^^^ method not found in `std::alloc::Global`
2020-03-07T06:22:56.3480208Z    |
2020-03-07T06:22:56.3480468Z    = help: items from traits can only be used if the trait is in scope
2020-03-07T06:22:56.3480468Z    = help: items from traits can only be used if the trait is in scope
2020-03-07T06:22:56.3480849Z help: the following trait is implemented but not in scope; perhaps add a `use` for it:
2020-03-07T06:22:56.3481135Z    |
2020-03-07T06:22:56.3481643Z 4  | use std::alloc::AllocRef;
2020-03-07T06:22:56.3481827Z    |
2020-03-07T06:22:56.3481917Z 
2020-03-07T06:22:56.3482303Z error[E0599]: no method named `dealloc` found for struct `std::alloc::Global` in the current scope
2020-03-07T06:22:56.3482840Z   --> /tmp/mdbook-y11Qm8/destructors.md:82:20
2020-03-07T06:22:56.3483345Z 28 |             Global.dealloc(c.cast::<u8>(), Layout::new::<T>());
2020-03-07T06:22:56.3483707Z    |                    ^^^^^^^ method not found in `std::alloc::Global`
2020-03-07T06:22:56.3483941Z    |
2020-03-07T06:22:56.3484170Z    = help: items from traits can only be used if the trait is in scope
---
2020-03-07T06:22:56.3486372Z For more information about an error, try `rustc --explain E0432`.
2020-03-07T06:22:56.3486782Z Couldn't compile the test.
2020-03-07T06:22:56.3486916Z 
2020-03-07T06:22:56.3487057Z failures:
2020-03-07T06:22:56.3487448Z     /tmp/mdbook-y11Qm8/destructors.md - Destructors (line 128)
2020-03-07T06:22:56.3488114Z     /tmp/mdbook-y11Qm8/destructors.md - Destructors (line 28)
2020-03-07T06:22:56.3488826Z     /tmp/mdbook-y11Qm8/destructors.md - Destructors (line 55)
2020-03-07T06:22:56.3489464Z test result: FAILED. 2 passed; 3 failed; 1 ignored; 0 measured; 0 filtered out
2020-03-07T06:22:56.3489714Z 
2020-03-07T06:22:56.3489811Z 
2020-03-07T06:22:56.3489966Z 
---
2020-03-07T06:22:56.3492801Z [TIMING] BookTest { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, path: "src/doc/nomicon", name: "nomicon", is_ext_doc: true } -- 6.718
2020-03-07T06:22:56.3493254Z Testing rustbook src/doc/reference
2020-03-07T06:23:12.3486892Z Error: Rustdoc returned an error: 
2020-03-07T06:23:12.3487208Z running 2 tests
2020-03-07T06:23:12.3488416Z test /tmp/mdbook-nsUu3Y/expressions/array-expr.md - Array_and_array_index_expressions::Array_and_slice_indexing_expressions (line 63) ... FAILED
2020-03-07T06:23:12.3489688Z test /tmp/mdbook-nsUu3Y/expressions/array-expr.md - Array_and_array_index_expressions::Array_expressions (line 27) ... ok
2020-03-07T06:23:12.3490356Z failures:
2020-03-07T06:23:12.3490470Z 
2020-03-07T06:23:12.3490470Z 
2020-03-07T06:23:12.3491095Z ---- /tmp/mdbook-nsUu3Y/expressions/array-expr.md - Array_and_array_index_expressions::Array_and_slice_indexing_expressions (line 63) stdout ----
2020-03-07T06:23:12.3491567Z error: this operation will panic at runtime
2020-03-07T06:23:12.3492272Z   --> /tmp/mdbook-nsUu3Y/expressions/array-expr.md:72:9
2020-03-07T06:23:12.3492959Z 11 | let x = (["a", "b"])[10]; // warning: index out of bounds
2020-03-07T06:23:12.3493352Z    |         ^^^^^^^^^^^^^^^^ index out of bounds: the len is 2 but the index is 10
2020-03-07T06:23:12.3493656Z    |
2020-03-07T06:23:12.3493878Z    = note: `#[deny(unconditional_panic)]` on by default
2020-03-07T06:23:12.3493878Z    = note: `#[deny(unconditional_panic)]` on by default
2020-03-07T06:23:12.3494088Z 
2020-03-07T06:23:12.3494281Z error: this operation will panic at runtime
2020-03-07T06:23:12.3494778Z   --> /tmp/mdbook-nsUu3Y/expressions/array-expr.md:78:1
2020-03-07T06:23:12.3495763Z 17 | arr[10];                  // warning: index out of bounds
2020-03-07T06:23:12.3496642Z    | ^^^^^^^ index out of bounds: the len is 2 but the index is 10
2020-03-07T06:23:12.3496914Z 
2020-03-07T06:23:12.3497133Z error: aborting due to 2 previous errors
2020-03-07T06:23:12.3497133Z error: aborting due to 2 previous errors
2020-03-07T06:23:12.3497341Z 
2020-03-07T06:23:12.3497704Z Couldn't compile the test.
2020-03-07T06:23:12.3498052Z 
2020-03-07T06:23:12.3498243Z failures:
2020-03-07T06:23:12.3499001Z     /tmp/mdbook-nsUu3Y/expressions/array-expr.md - Array_and_array_index_expressions::Array_and_slice_indexing_expressions (line 63)
2020-03-07T06:23:12.3500252Z test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
2020-03-07T06:23:12.3500488Z 
2020-03-07T06:23:12.3500579Z 
2020-03-07T06:23:12.3500732Z 
---
2020-03-07T06:25:48.4902574Z error: Unable to retrieve "http://www.ps.uni-sb.de/courses/typen-ws99/class.ps.gz": http://www.ps.uni-sb.de/courses/typen-ws99/class.ps.gz: timed out
2020-03-07T06:25:48.4903964Z 
2020-03-07T06:25:48.4904564Z     ┌── appendix/bibliography.md:11:3 ───
2020-03-07T06:25:48.4904941Z     │
2020-03-07T06:25:48.4905528Z  11 │ * [Typeclasses: making ad-hoc polymorphism less ad hoc](http://www.ps.uni-sb.de/courses/typen-ws99/class.ps.gz)
2020-03-07T06:25:48.4906547Z     │   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ http://www.ps.uni-sb.de/courses/typen-ws99/class.ps.gz: timed out
2020-03-07T06:25:48.4907700Z 
2020-03-07T06:25:48.5017159Z Building stage2 tool clippy-driver (x86_64-unknown-linux-gnu)
2020-03-07T06:25:48.7860902Z    Compiling proc-macro2 v0.4.30
2020-03-07T06:25:48.7861481Z    Compiling unicode-xid v0.1.0
---
2020-03-07T07:09:34.7548788Z Verifying status of rustc-guide...
2020-03-07T07:09:34.7601649Z Cloning into 'rust-toolstate'...
2020-03-07T07:09:35.3813440Z warning: Tool `nomicon` is not test-pass (is `test-fail`), this should be fixed before beta is branched.
2020-03-07T07:09:35.3814713Z warning: Tool `reference` is not test-pass (is `test-fail`), this should be fixed before beta is branched.
2020-03-07T07:09:35.3815622Z error: Tool `clippy-driver` has regressed from test-pass to build-fail during beta week.
2020-03-07T07:09:35.3823986Z Build completed unsuccessfully in 0:00:02
2020-03-07T07:09:35.3872848Z == clock drift check ==
2020-03-07T07:09:35.3883304Z   local time: Sat Mar  7 07:09:35 UTC 2020
2020-03-07T07:09:35.4596288Z   network time: Sat, 07 Mar 2020 07:09:35 GMT
2020-03-07T07:09:35.4596288Z   network time: Sat, 07 Mar 2020 07:09:35 GMT
2020-03-07T07:09:35.4598741Z == end clock drift check ==
2020-03-07T07:09:35.8007804Z 
2020-03-07T07:09:35.8089924Z ##[error]Bash exited with code '1'.
2020-03-07T07:09:35.8175192Z ##[section]Starting: Checkout rust-lang/rust@auto to s
2020-03-07T07:09:35.8180390Z ==============================================================================
2020-03-07T07:09:35.8180794Z Task         : Get sources
2020-03-07T07:09:35.8181219Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
