plain
[01:10:31] ---- [run-pass] run-pass-fulldeps/macro-crate-multi-decorator-literals.rs stdout ----
[01:10:31]  
[01:10:31] error: compilation failed!
[01:10:31] status: exit code: 101
[01:10:31] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/macro-crate-multi-decorator-literals.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/macro-crate-multi-decorator-literals.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/macro-crate-multi-decorator-literals.stage2-x86_64-unknown-linux-gnu.aux"
[01:10:31] ------------------------------------------
[01:10:31] 
[01:10:31] ------------------------------------------
[01:10:31] stderr:
---
[01:10:31]    | ^^^^^^^^^^^^
[01:10:31]    |
[01:10:31]    = note: #[warn(unused_imports)] on by default
[01:10:31] 
[01:10:31] warning: compiler plugin used as an ordinary library
[01:10:31]    |
[01:10:31] 19 | extern crate macro_crate_test;
[01:10:31]    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:10:31]    |
[01:10:31]    |
[01:10:31]    = note: #[warn(plugin_as_library)] on by default
[01:10:31] 
[01:10:31] error: linking with `cc` failed: exit code: 1
[01:10:31]   |
[01:10:31]   = note: "cc" "-Wl,--as-needed" "-Wl,-z,noexecstack" "-m64" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/macro-crate-multi-decorator-literals.macro_crate_multi_decorator_literals0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/macro-crate-multi-decorator-literals.macro_crate_multi_decorator_literals1.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/macro-crate-multi-decorator-literals.macro_crate_multi_decorator_literals10.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/macro-crate-multi-decorator-literals.macro_crate_multi_decorator_literals11.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/macro-crate-multi-decorator-literals.macro_crate_multi_decorator_literals2.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/macro-crate-multi-decorator-literals.macro_crate_multi_decorator_literals3.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/macro-crate-multi-decorator-literals.macro_crate_multi_decorator_literals4.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/macro-crate-multi-decorator-literals.macro_crate_multi_decorator_literals5.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/macro-crate-multi-decorator-literals.macro_crate_multi_decorator_literals6.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/macro-crate-multi-decorator-literals.macro_crate_multi_decorator_literals7.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/macro-crate-multi-decorator-literals.macro_crate_multi_decorator_literals8.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/macro-crate-multi-decorator-literals.macro_crate_multi_decorator_literals9.rcgu.o" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/macro-crate-multi-decorator-literals.stage2-x86_64-unknown-linux-gnu" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/macro-crate-multi-decorator-literals.crate.allocator.rcgu.o" "-Wl,--gc-sections" "-pie" "-Wl,-z,relro,-z,now" "-nodefaultlibs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/macro-crate-multi-decorator-literals.stage2-x86_64-unknown-linux-gnu.aux" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-l" "rustc_plugin-197ac5dec2e5592f" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-l" "rustc_metadata-73a60d28dcc8d3e0" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-l" "syntax_ext-3dacfb5b3c39e1ee" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-l" "rustc-c6263d330d2b9f31" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-l" "test-902559d4a0cc83d5" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-l" "term-36880752958d2c6f" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-l" "proc_macro-c7d45a460cb3a738" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-l" "rustc_const_math-e212b3283c198d45" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-l" "graphviz-06bb7008d45cdff0" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-l" "fmt_macros-7fa0522df5af949a" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-l" "arena-16294da79648b864" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-l" "syntax-e97e6d4ccc983706" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-l" "rustc_target-aaec33fa4a23ec51" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-l" "rustc_errors-dafdecc781917b18" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-l" "syntax_pos-ccbedf94f6bd555f" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-l" "rustc_data_structures-22e458fc7e0d421f" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-l" "rustc_cratesio_shim-7bdcd669c7579602" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-l" "serialize-b63c42e100577192" "-Wl,--start-group" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-l" "std-b63203ee1146648a" "-Wl,--end-group" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-bbf524080a1524b2.rlib" "-Wl,-Bdynamic" "-l" "util" "-l" "util" "-l" "dl" "-l" "rt" "-l" "pthread" "-l" "pthread" "-l" "gcc_s" "-l" "c" "-l" "m" "-l" "rt" "-l" "pthread" "-l" "util" "-l" "util" "-Wl,-rpath,$ORIGIN/../../stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-rpath,/checkout/obj/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,--enable-new-dtags"
[01:10:31]   = note: /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/macro-crate-multi-decorator-literals.macro_crate_multi_decorator_literals0.rcgu.o: In function `core::str::traits::_$LT$impl$u20$core..cmp..PartialEq$u20$for$u20$str$GT$::eq::h87d16125d269f67a':
[01:10:31]           macro_crate_multi_decorator_literals0-317d481089b8c8fe83113de504472633.rs:(.text._ZN4core3str6traits54_$LT$impl$u20$core..cmp..PartialEq$u20$for$u20$str$GT$2eq17h87d16125d269f67aE+0x41): undefined reference to `core::cmp::impls::_$LT$impl$u20$core..cmp..PartialEq$LT$$RF$$u27$b$u20$B$GT$$u20$for$u20$$RF$$u27$a$u20$A$GT$::eq::ha31f7d414bd1aa29'
[01:10:31]           collect2: error: ld returned 1 exit status
[01:10:31] 
[01:10:31] error: aborting due to previous error
[01:10:31] 
[01:10:31] 
---
[01:10:31] 
[01:10:31] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:488:22
[01:10:31] 
[01:10:31] 
[01:10:31] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Zunstable-options " "--target-rustcflags" "-Crpath -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "6.0.0\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:10:31] 
[01:10:31] 
[01:10:31] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:10:31] Build completed unsuccessfully in 0:13:30
[01:10:31] Build completed unsuccessfully in 0:13:30
[01:10:31] Makefile:58: recipe for target 'check' failed
[01:10:31] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:026bd193
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
