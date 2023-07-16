plain
test [ui] src/test/ui/allocator/allocator-args.rs ... ok
test [ui] src/test/ui/alloc-error/alloc-error-handler-bad-signature-2.rs ... ok
test [ui] src/test/ui/allocator/two-allocators.rs ... ok
test [ui] src/test/ui/allocator/not-an-allocator.rs ... ok
test [ui] src/test/ui/abi/issues/issue-97463-broken-abi-leaked-uninit-data.rs ... FAILED
test [ui] src/test/ui/abi/x86stdcall2.rs ... ok
test [ui] src/test/ui/aligned_enum_cast.rs ... ok
test [ui] src/test/ui/align-with-extern-c-fn.rs ... ok
test [ui] src/test/ui/abi/abi-sysv64-arg-passing.rs ... ok
---
test [ui] src/test/ui/wrong-hashset-issue-42918.rs ... ok

failures:

---- [ui] src/test/ui/abi/issues/issue-97463-broken-abi-leaked-uninit-data.rs stdout ----
error: test compilation failed although it shouldn't!
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/abi/issues/issue-97463-broken-abi-leaked-uninit-data.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/issues/issue-97463-broken-abi-leaked-uninit-data/a.wasm" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/issues/issue-97463-broken-abi-leaked-uninit-data/auxiliary"
stdout: none
--- stderr -------------------------------
error: linking with `rust-lld` failed: exit status: 1
   |
   = note: "rust-lld" "-flavor" "wasm" "--rsp-quoting=posix" "--export" "main" "--export=__heap_base" "--export=__data_end" "-z" "stack-size=1048576" "--stack-first" "--allow-undefined" "--fatal-warnings" "--no-demangle" "--no-entry" "--export-dynamic" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/issues/issue-97463-broken-abi-leaked-uninit-data/a.issue_97463_broken_abi_leaked_uninit_data.305a54f7-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/issues/issue-97463-broken-abi-leaked-uninit-data/a.junm93kb3x49uh4.rcgu.o" "-L" "/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/issues/issue-97463-broken-abi-leaked-uninit-data/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib" "-l" "rust_test_helpers" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib/libstd-522387fa7c52b7c4.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib/libpanic_abort-181955fd339dfa00.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib/libdlmalloc-f94f9803490a8303.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib/librustc_demangle-983dcdf7e24affff.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib/libstd_detect-bf0cb248adf3b6fa.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib/libhashbrown-5934778f3092a6d5.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib/libminiz_oxide-2409eee3494f5458.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib/libadler-62cdd07647a4d9e4.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib/librustc_std_workspace_alloc-112e5f81ab90ae0e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib/libunwind-5e1d5d17da3115db.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib/libcfg_if-7a5406a8308fb165.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib/liblibc-1646056521a1f0e2.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib/liballoc-09603881c584693a.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib/librustc_std_workspace_core-36b06538a2da70c9.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib/libcore-b523e6a4b101d4a1.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib/libcompiler_builtins-8ce0e0cb86e194b0.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib/self-contained" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/issues/issue-97463-broken-abi-leaked-uninit-data/a.wasm" "--gc-sections" "-O2" "--strip-debug"
   = note: rust-lld: error: unable to find library -lrust_test_helpers

error: aborting due to previous error
------------------------------------------




failures:
    [ui] src/test/ui/abi/issues/issue-97463-broken-abi-leaked-uninit-data.rs
test result: FAILED. 12659 passed; 1 failed; 665 ignored; 0 measured; 0 filtered out; finished in 89.70s

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=wasm32-unknown-unknown
 finished in 90.235 seconds
