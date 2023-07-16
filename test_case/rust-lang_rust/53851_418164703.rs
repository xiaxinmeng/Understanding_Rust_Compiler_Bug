plain
[00:47:02] ...........................................................i........................................
[00:47:04] ....................................................................................................
[00:47:08] ....................................................................................................
[00:47:11] ....................................................................................................
[00:47:14] ............................................................................................F.......
[00:47:17] ......................................F.............................................................
[00:47:23] ....................................................................................................
[00:47:27] ....................................................................................................
[00:47:29] ....................................................i...............................................
[00:47:33] ....................................................................................................
---
[00:48:16] 
[00:48:16] ---- [ui] ui/issues/issue-39559-2.rs stdout ----
[00:48:16] diff of stderr:
[00:48:16] 
[00:48:16] 10 LL |         = [0; Dim3::dim()];
[00:48:16] 12 
[00:48:16] - error[E0080]: could not evaluate repeat length
[00:48:16] -   --> $DIR/issue-39559-2.rs:27:15
[00:48:16] -    |
[00:48:16] -    |
[00:48:16] - LL |         = [0; Dim3::dim()];
[00:48:16] -    |               ^^^^^^^^^^^ calling non-const fn `<Dim3 as Dim>::dim`
[00:48:16] + error: aborting due to 2 previous errors
[00:48:16] - error[E0080]: could not evaluate constant expression
[00:48:16] -   --> $DIR/issue-39559-2.rs:24:16
[00:48:16] -    |
[00:48:16] -    |
[00:48:16] - LL |     let array: [usize; Dim3::dim()]
[00:48:16] -    |                        |
[00:48:16] -    |                        |
[00:48:16] -    |                        calling non-const fn `<Dim3 as Dim>::dim`
[00:48:16] - error: aborting due to 4 previous errors
[00:48:16] - 
[00:48:16] - Some errors occurred: E0015, E0080.
[00:48:16] - For more information about an error, try `rustc --explain E0015`.
[00:48:16] - For more information about an error, try `rustc --explain E0015`.
[00:48:16] + For more information about this error, try `rustc --explain E0015`.
[00:48:16] 31 
[00:48:16] 
[00:48:16] 
[00:48:16] The actual stderr differed from the expected stderr.
[00:48:16] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-39559-2/issue-39559-2.stderr
[00:48:16] To update references, rerun the tests and pass the `--bless` flag
[00:48:16] To only update this specific test, also pass `--test-args issues/issue-39559-2.rs`
[00:48:16] error: 1 errors occurred comparing output.
[00:48:16] status: exit code: 1
[00:48:16] status: exit code: 1
[00:48:16] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-39559-2.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-39559-2/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-39559-2/auxiliary" "-A" "unused"
[00:48:16] ------------------------------------------
[00:48:16] 
[00:48:16] ------------------------------------------
[00:48:16] stderr:
[00:48:16] stderr:
[00:48:16] ------------------------------------------
[00:48:16] {"message":"calls in constants are limited to constant functions, tuple structs and tuple variants","code":{"code":"E0015","explanation":"\nThe only functions that can be called in static or constant expressions are\n`const` functions, and struct/enum constructors. `const` functions are only\navailable on a nightly compiler. Rust currently does not support more general\ncompile-time function execution.\n\n