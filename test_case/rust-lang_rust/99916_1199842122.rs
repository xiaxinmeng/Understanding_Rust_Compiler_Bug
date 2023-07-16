plain

---- [ui] src/test/ui/rfc-2627-raw-dylib/raw-dylib-windows-only.rs stdout ----
diff of stderr:

1 error[E0455]: link kind `raw-dylib` is only supported on Windows targets
-   --> $DIR/raw-dylib-windows-only.rs:5:29
3    |
3    |
4 LL | #[link(name = "foo", kind = "raw-dylib")]

6 
- error: aborting due to previous error; 1 warning emitted
+ error: aborting due to previous error
---
To only update this specific test, also pass `--test-args rfc-2627-raw-dylib/raw-dylib-windows-only.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2627-raw-dylib/raw-dylib-windows-only.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2627-raw-dylib/raw-dylib-windows-only" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2627-raw-dylib/raw-dylib-windows-only/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0455]: link kind `raw-dylib` is only supported on Windows targets
   |
   |
LL | #[link(name = "foo", kind = "raw-dylib")]

error: aborting due to previous error

For more information about this error, try `rustc --explain E0455`.
