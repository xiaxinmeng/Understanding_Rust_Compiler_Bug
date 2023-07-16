
[00:57:08] ---- [codegen] codegen/target-cpu-on-functions.rs stdout ----
[00:57:08] 
[00:57:08] error: compilation failed!
[00:57:08] status: exit code: 1
[00:57:08] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/codegen/target-cpu-on-functions.rs" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/target-cpu-on-functions" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "no-prepopulate-passes" "-C" "panic=abort" "-Z" "cross-lang-lto" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/target-cpu-on-functions/auxiliary" "--emit=llvm-ir"
[00:57:08] stdout:
[00:57:08] ------------------------------------------
[00:57:08] 
[00:57:08] ------------------------------------------
[00:57:08] stderr:
[00:57:08] ------------------------------------------
[00:57:08] error: The current compilation is going to use thin LTO buffers without running LLVM's NameAnonGlobals pass. This will likely cause errors in LLVM. Consider adding -C passes=name-anon-globals to the compiler command line.
[00:57:08] 
[00:57:08] error: aborting due to previous error
[00:57:08] 
[00:57:08] 
[00:57:08] ------------------------------------------
[00:57:08] 
[00:57:08] thread '[codegen] codegen/target-cpu-on-functions.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3149:9
[00:57:08] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:57:08] 
[00:57:08] 
[00:57:08] failures:
[00:57:08]     [codegen] codegen/target-cpu-on-functions.rs
[00:57:08] 
