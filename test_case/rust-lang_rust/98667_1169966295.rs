plain
test [ui] src/test/ui/wrong-hashset-issue-42918.rs ... ok

failures:

---- [ui] src/test/ui/unwind-abis/ffi-unwind-calls-lint.rs stdout ----

- warning: call to foreign function with FFI-unwind ABI
-   --> $DIR/ffi-unwind-calls-lint.rs:19:14
-    |
-    |
- LL |     unsafe { foo(); }
-    |              ^^^^^ call to foreign function with FFI-unwind ABI
- note: the lint level is defined here
-   --> $DIR/ffi-unwind-calls-lint.rs:4:9
-    |
-    |
- LL | #![warn(ffi_unwind_calls)]
- 
- warning: call to function pointer with FFI-unwind ABI
-   --> $DIR/ffi-unwind-calls-lint.rs:23:5
-    |
-    |
- LL |     ptr();
-    |     ^^^^^ call to function pointer with FFI-unwind ABI
- warning: 2 warnings emitted
- 
- 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unwind-abis/ffi-unwind-calls-lint/ffi-unwind-calls-lint.stderr
To only update this specific test, also pass `--test-args unwind-abis/ffi-unwind-calls-lint.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unwind-abis/ffi-unwind-calls-lint.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unwind-abis/ffi-unwind-calls-lint" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unwind-abis/ffi-unwind-calls-lint/auxiliary"
stdout: none
stderr: none


failures:
    [ui] src/test/ui/unwind-abis/ffi-unwind-calls-lint.rs
