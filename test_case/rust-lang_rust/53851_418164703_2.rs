\n\nSee [RFC 911] for more details on the design of `const fn`s.\n\n[RFC 911]: https://github.com/rust-lang/rfcs/blob/master/text/0911-const-fn.md\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-39559-2.rs","byte_start":686,"byte_end":697,"line_start":27,"line_end":27,"column_start":15,"column_end":26,"is_primary":true,"text":[{"text":"        = [0; Dim3::dim()];","highlight_start":15,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants\n  --> /checkout/src/test/ui/issues/issue-39559-2.rs:27:15\n   |\nLL |         = [0; Dim3::dim()];\n   |               ^^^^^^^^^^^\n\n"}
[00:48:16] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:48:16] {"message":"For more information about this error, try `rustc --explain E0015`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0015`.\n"}
[00:48:16] ------------------------------------------
[00:48:16] 
[00:48:16] thread '[ui] ui/issues/issue-39559-2.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3196:9
[00:48:16] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:48:16] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:48:16] 
[00:48:16] ---- [ui] ui/issues/issue-43105.rs stdout ----
[00:48:16] diff of stderr:
[00:48:16] 
[00:48:16] 4 LL | const NUM: u8 = xyz();
[00:48:16] 6 
[00:48:16] - error[E0080]: could not evaluate constant pattern
[00:48:16] -   --> $DIR/issue-43105.rs:18:9
[00:48:16] -    |
[00:48:16] -    |
[00:48:16] - LL | const NUM: u8 = xyz();
[00:48:16] -    |                 ----- calling non-const fn `xyz`
[00:48:16] - ...
[00:48:16] - LL |         NUM => unimplemented!(),
[00:48:16] + error: aborting due to previous error
[00:48:16] 15 
[00:48:16] - error: aborting due to 2 previous errors
[00:48:16] - 
---
[00:48:16] 
[00:48:16] 
[00:48:16] The actual stderr differed from the expected stderr.
[00:48:16] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-43105/issue-43105.stderr
[00:48:16] To update references, rerun the tests and pass the `--bless` flag
[00:48:16] To only update this specific test, also pass `--test-args issues/issue-43105.rs`
[00:48:16] error: 1 errors occurred comparing output.
[00:48:16] status: exit code: 1
[00:48:16] status: exit code: 1
[00:48:16] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-43105.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-43105/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-43105/auxiliary" "-A" "unused"
[00:48:16] ------------------------------------------
[00:48:16] 
[00:48:16] ------------------------------------------
[00:48:16] stderr:
[00:48:16] stderr:
[00:48:16] ------------------------------------------
[00:48:16] {"message":"calls in constants are limited to constant functions, tuple structs and tuple variants","code":{"code":"E0015","explanation":"\nThe only functions that can be called in static or constant expressions are\n`const` functions, and struct/enum constructors. `const` functions are only\navailable on a nightly compiler. Rust currently does not support more general\ncompile-time function execution.\n\n