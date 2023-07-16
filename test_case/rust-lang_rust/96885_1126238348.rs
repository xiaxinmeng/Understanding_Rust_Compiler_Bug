plain

7    = note: `#[warn(incomplete_features)]` on by default
8    = note: see issue #58713 <https://github.com/rust-lang/rust/issues/58713> for more information
9 
- error: link kind `raw-dylib` is only supported on Windows targets
+ error[E0455]: link kind `raw-dylib` is only supported on Windows targets
12    |
12    |
13 LL | #[link(name = "foo", kind = "raw-dylib")]
15 
16 error: aborting due to previous error; 1 warning emitted
17 
+ For more information about this error, try `rustc --explain E0455`.
---
To only update this specific test, also pass `--test-args rfc-2627-raw-dylib/raw-dylib-windows-only.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2627-raw-dylib/raw-dylib-windows-only.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2627-raw-dylib/raw-dylib-windows-only" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2627-raw-dylib/raw-dylib-windows-only/auxiliary"
stdout: none
--- stderr -------------------------------
warning: the feature `raw_dylib` is incomplete and may not be safe to use and/or cause compiler crashes
   |
   |
LL | #![feature(raw_dylib)]
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #58713 <https://github.com/rust-lang/rust/issues/58713> for more information


error[E0455]: link kind `raw-dylib` is only supported on Windows targets
   |
   |
LL | #[link(name = "foo", kind = "raw-dylib")]

error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0455`.
