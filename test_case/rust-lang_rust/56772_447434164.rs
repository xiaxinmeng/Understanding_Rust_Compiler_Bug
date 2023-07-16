
[01:00:08] ---- [run-pass] run-pass/issues/issue-18804/main.rs stdout ----
[01:00:08] 
[01:00:08] error: test compilation failed although it shouldn't!
[01:00:08] status: signal: 6
[01:00:08] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/issues/issue-18804/main.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-18804/main/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "no-prepopulate-passes" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/issues/issue-18804/main/auxiliary"
[01:00:08] ------------------------------------------
[01:00:08] 
[01:00:08] ------------------------------------------
[01:00:08] stderr:
[01:00:08] stderr:
[01:00:08] ------------------------------------------
[01:00:08] rustc: /checkout/src/llvm/include/llvm/ADT/StringRef.h:239: char llvm::StringRef::operator[](size_t) const: Assertion `Index < Length && "Invalid index!"' failed.
[01:00:08] ------------------------------------------
[01:00:08] 
[01:00:08] thread '[run-pass] run-pass/issues/issue-18804/main.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3252:9
[01:00:08] note: Run with `RUST_BACKTRACE=1` for a backtrace.
