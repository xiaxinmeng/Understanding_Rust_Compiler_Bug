plain
...........................................................................

failures:

---- [ui] tests/ui/async-await/return-type-notation/ty-or-ct-params.rs stdout ----

- warning: the feature `async_fn_in_trait` is incomplete and may not be safe to use and/or cause compiler crashes
- warning: the feature `async_fn_in_trait` is incomplete and may not be safe to use and/or cause compiler crashes
-   --> $DIR/ty-or-ct-params.rs:3:12
- LL | #![feature(async_fn_in_trait, return_type_notation)]
-    |            ^^^^^^^^^^^^^^^^^
-    |
-    = note: see issue #91611 <https://github.com/rust-lang/rust/issues/91611> for more information
-    = note: see issue #91611 <https://github.com/rust-lang/rust/issues/91611> for more information
-    = note: `#[warn(incomplete_features)]` on by default
- 
10 warning: the feature `return_type_notation` is incomplete and may not be safe to use and/or cause compiler crashes
11   --> $DIR/ty-or-ct-params.rs:3:31

14    |                               ^^^^^^^^^^^^^^^^^^^^
15    |
16    = note: see issue #109417 <https://github.com/rust-lang/rust/issues/109417> for more information
16    = note: see issue #109417 <https://github.com/rust-lang/rust/issues/109417> for more information
+    = note: `#[warn(incomplete_features)]` on by default
17 
18 error: return type notation is not allowed for functions that have type parameters
19   --> $DIR/ty-or-ct-params.rs:15:12

33 LL |     T: Foo<bar(): Send, baz(): Send>,
35 
- error: aborting due to 2 previous errors; 2 warnings emitted
+ error: aborting due to 2 previous errors; 1 warning emitted
37 
37 
38 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/return-type-notation/ty-or-ct-params/ty-or-ct-params.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args async-await/return-type-notation/ty-or-ct-params.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/async-await/return-type-notation/ty-or-ct-params.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/return-type-notation/ty-or-ct-params" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/return-type-notation/ty-or-ct-params/auxiliary" "--edition=2021"
stdout: none
warning: the feature `return_type_notation` is incomplete and may not be safe to use and/or cause compiler crashes
warning: the feature `return_type_notation` is incomplete and may not be safe to use and/or cause compiler crashes
  --> fake-test-src-base/async-await/return-type-notation/ty-or-ct-params.rs:3:31
LL | #![feature(async_fn_in_trait, return_type_notation)]
   |                               ^^^^^^^^^^^^^^^^^^^^
   |
   = note: see issue #109417 <https://github.com/rust-lang/rust/issues/109417> for more information
   = note: see issue #109417 <https://github.com/rust-lang/rust/issues/109417> for more information
   = note: `#[warn(incomplete_features)]` on by default

error: return type notation is not allowed for functions that have type parameters
  --> fake-test-src-base/async-await/return-type-notation/ty-or-ct-params.rs:15:12
   |
LL |     async fn bar<T>() {}
...
...
LL |     T: Foo<bar(): Send, baz(): Send>,

error: return type notation is not allowed for functions that have const parameters
error: return type notation is not allowed for functions that have const parameters
  --> fake-test-src-base/async-await/return-type-notation/ty-or-ct-params.rs:15:25
   |
LL |     async fn baz<const N: usize>() {}
Build completed unsuccessfully in 0:12:53
...
...
LL |     T: Foo<bar(): Send, baz(): Send>,

error: aborting due to 2 previous errors; 1 warning emitted
------------------------------------------




failures:
    [ui] tests/ui/async-await/return-type-notation/ty-or-ct-params.rs
test result: FAILED. 14812 passed; 1 failed; 134 ignored; 0 measured; 0 filtered out; finished in 158.30s

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
