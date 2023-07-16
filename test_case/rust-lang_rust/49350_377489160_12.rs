\n\nIf the trait `Foo` was deriving from something like `Super<String>` or\n`Super<T>` (where `Foo` itself is `Foo<T>`), this is okay, because given a type\n`get_a()` will definitely return an object of that type.\n\nHowever, if it derives from `Super<Self>`, even though `Super` is object safe,\nthe method `get_a()` would return an object of unknown type when called on the\nfunction. `Self` type parameters let us make object safe traits no longer safe,\nso they are forbidden when specifying supertraits.\n\nThere's no easy fix for this, generally code will need to be refactored so that\nyou no longer need to derive from `Super<Self>`.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/uil. (see issue #49476)
[00:36:07] -   --> $DIR/feature-gate-macros_in_extern.rs:31:5
[00:36:07] +   --> $DIR/feature-gate-macros_in_extern.rs:29:5
[00:36:07] 3    |
[00:36:07] 4 LL |     returns_isize!(rust_get_test_int);
[00:36:07] 5    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:36:07]
[00:36:07] 7    = help: add #![feature(macros_in_extern)] to the crate attributes to enable
[00:36:07] 8
[00:36:07] 9 error[E0658]: Macro invocations in `extern {}` blocks are experimental. (see issue #49476)
[00:36:07] -   --> $DIR/feature-gate-macros_in_extern.rs:33:5
[00:36:07] +   --> $DIR/feature-gate-macros_in_extern.rs:31:5
[00:36:07] 11    |
[00:36:07] 12 LL |     takes_u32_returns_u32!(rust_dbg_extern_identity_u32);
[00:36:07] 13    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:36:07]
[00:36:07] 15    = help: add #![feature(macros_in_extern)] to the crate attributes to enable
[00:36:07] 16
[00:36:07] 17 error[E0658]: Macro invocations in `extern {}` blocks are experimental. (see issue #49476)
[00:36:07] -   --> $DIR/feature-gate-macros_in_extern.rs:35:5
[00:36:07] +   --> $DIR/feature-gate-macros_in_extern.rs:33:5
[00:36:07] 19    |
[00:36:07] 20 LL |     emits_nothing!();
---
[00:36:07] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'feature-gate-macros_in_extern.rs'
[00:36:07]
[00:36:07] error: 1 errors occurred comparing output.
[00:36:07] status: exit code: 101
[00:36:07] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gate-macros_in_extern.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gate-macros_in_extern.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gate-macros_in_extern.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
---
[00:36:07] {"message":"Macro invocations in `extern {}` blocks are experimental. (see issue #49476)","code":{"code":"E0658","explanation":"\nAn unstable feature was used.\n\nErroneous code example:\n\n