plain
[00:45:48] ....................................................................................................
[00:45:53] ....................................................................................................
[00:45:59] ....................................................................................................
[00:46:04] ....................................................................................................
[00:46:10] ...................F...i............................................................................
[00:46:20] ....................................................................................................
[00:46:26] ....................................................................................................
[00:46:32] ...........................................i........................................................
[00:46:34] ............................
[00:46:34] ............................
[00:46:34] failures:
[00:46:34] 
[00:46:34] ---- [ui] ui/missing-allocator.rs stdout ----
[00:46:34] diff of stderr:
[00:46:34] 
[00:46:34] 1 error: no global memory allocator found but one is required; link to std or add #[global_allocator] to a static item that implements the GlobalAlloc trait.
[00:46:34] 2 
[00:46:34] - error: aborting due to previous error
[00:46:34] + error[E0522]: definition of an unknown language item: `oom`
[00:46:34] +   --> $DIR/missing-allocator.rs:23:1
[00:46:34] +    |
[00:46:34] + LL | #[lang = "oom"]
[00:46:34] +    | ^^^^^^^^^^^^^^^ definition of unknown language item `oom`
[00:46:34] + error: aborting due to 2 previous errors
[00:46:34] + 
[00:46:34] + For more information about this error, try `rustc --explain E0522`.
[00:46:34] 5 
[00:46:34] 5 
[00:46:34] 
[00:46:34] 
[00:46:34] The actual stderr differed from the expected stderr.
[00:46:34] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/missing-allocator/missing-allocator.stderr
[00:46:34] To update references, rerun the tests and pass the `--bless` flag
[00:46:34] To only update this specific test, also pass `--test-args missing-allocator.rs`
[00:46:34] error: 1 errors occurred comparing output.
[00:46:34] status: exit code: 101
[00:46:34] status: exit code: 101
[00:46:34] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/missing-allocator.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/missing-allocator/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "panic=abort" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/missing-allocator/auxiliary" "-A" "unused"
[00:46:34] ------------------------------------------
[00:46:34] 
[00:46:34] ------------------------------------------
[00:46:34] stderr:
[00:46:34] stderr:
[00:46:34] ------------------------------------------
[00:46:34] {"message":"no global memory allocator found but one is required; link to std or add #[global_allocator] to a static item that implements the GlobalAlloc trait.","code":null,"level":"error","spans":[],"children":[],"rendered":"error: no global memory allocator found but one is required; link to std or add #[global_allocator] to a static item that implements the GlobalAlloc trait.\n\n"}
[00:46:34] {"message":"definition of an unknown language item: `oom`","code":{"code":"E0522","explanation":"\nThe lang attribute is intended for marking special items that are built-in to\nRust itself. This includes special traits (like `Copy` and `Sized`) that affect\nhow the compiler behaves, as well as special functions that may be automatically\ninvoked (such as the handler for out-of-bounds accesses when indexing a slice).\nErroneous code example:\n\n