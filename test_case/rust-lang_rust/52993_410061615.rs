plain
travis_time:start:test_run-pass
Check compiletest suite=run-pass mode=run-pass (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:50:19] 
[00:50:19] running 3056 tests
[00:50:28] ..........F..F.F....................................................................................
[00:50:51] ....................................................................................................
[00:51:01] ....................................................................................................
[00:51:10] ....................................................................................................
[00:51:25] ....................................................................................................
---
[00:54:29] ....................................................................................................
[00:54:43] ....................................................................................................
[00:54:59] .................................................ii.................................................
[00:55:16] ...........i....i.....................................................i.............................
[00:55:44] .................................................................................F..................
[00:56:02] ....................................................................................................
[00:56:13] ....................................................................................................
[00:56:13] ....................................................................................................
pass/allocator/custom/a" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/allocator/custom/a.crate.allocator.rcgu.o" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro" "-Wl,-znow" "-Wl,-O1" "-nodefaultlibs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/allocator/custom/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/allocator/custom/auxiliary/libhelper.rlib" "-Wl,--start-group" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-e054c7a28f8831a7.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-17efed325058ddbe.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-a2697584dddf62e0.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc_system-ebc4ac8fd3c426d7.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-a1822c0b755de650.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-0f56ef064ad577ca.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-aae624166adf9237.rlib" "-Wl,--end-group" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-2866e3dedc0t/obj/build/x86_64-unknown-linux-gnu/test/run-pass/allocator/xcrate-use/a.xcrate_use5-317d481089b8c8fe83113de504472633.rs.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/allocator/xcrate-use/a.xcrate_use6-317d481089b8c8fe83113de504472633.rs.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/allocator/xcrate-use/a.xcrate_use7-317d481089b8c8fe83113de504472633.rs.rcgu.o" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/allocator/xcrate-use/a" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/allocator/xcrate-use/a.crate.allocator.rcgu.o" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro" "-Wl,-znow" "-Wl,-O1" "-nodefaultlibs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/allocator/xcrate-use/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/allocator/xcrate-use/auxiliary/libhelper.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/allocator/xcrate-use/auxiliary/libcustom.rlib" "-Wl,--start-group" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-e054c7a28f8831a7.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-17efed325058ddbe.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-a2697584dddf62e0.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustl[00:56:20]           collect2: error: ld returned 1 exit status
[00:56:20] 
[00:56:20] error: aborting due to previous error
[00:56:20] 
[00:56:20] 
---
[00:56:20] ---- [run-pass] run-pass/allocator/xcrate-use2.rs stdout ----
[00:56:20] 
[00:56:20] error: compilation failed!
[00:56:20] status: exit code: 1
[00:56:20] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/allocator/xcrate-use2.rs" "--target=x86_64-unknown-linux-gnu" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/allocator/xcrate-use2/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/allocator/xcrate-use2/auxiliary"
[00:56:20] ------------------------------------------
[00:56:20] 
[00:56:20] ------------------------------------------
[00:56:20] stderr:
[00:56:20] stderr:
[00:56:20] ------------------------------------------
[00:56:20] error: linking with `cc` failed: exit code: 1
[00:56:20]   |
[00:56:20]   = note: "cc" "-Wl,--as-needed" "-Wl,-z,noexecstack" "-m64" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/allocator/xcrate-use2/a.xcrate_use20-317d481089b8c8fe83113de504472633.rs.rcgu.o" "/checkout/obj/86_64-unknown-linux-gnu/lib/libunwind-a2697584dddf62e0.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc_system-ebc4ac8fd3c426d7.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-a1822c0b755de650.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-0f56ef064ad577ca.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-aae624166adf9237.rlib" "-Wl,--end-group" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-2866e3dedc0a56e0.rlib" "-Wl,-Bdynamic" "-ldl" "-lrt" "-lpthread" "-lpthread" "-lgcc_s" "-lc" "-lm" "-lrt" "-lpthread" "-lutil" "-lutil" "-Wl,-rpath,$ORIGIN/../../../../stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-rpath,/checkout/obj/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,--enable-new-dtags"
[00:56:20]   = note: /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/allocator/xcrate-use2/a.crate.allocator.rcgu.o: In function `__rust_alloc':
[00:56:20]           allocator:(.text.__rust_alloc+0x1): undefined reference to `__rg_alloc'
[00:56:20]           /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/allocator/xcrate-use2/a.crate.allocator.rcgu.o: In function `__rust_dealloc':
[00:56:20]           allocator:(.text.__rust_dealloc+0x1): undefined reference to `__rg_dealloc'
[00:56:20]           /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/allocator/xcrate-use2/a.crate.allocator.rcgu.o: In function `__rust_realloc':
[00:56:20]           allocator:(.text.__rust_realloc+0x1): undefined reference to `__rg_realloc'
[00:56:20]           
[00:56:20] 
[00:56:20] error: aborting due to previous error
[00:56:20] 
---
[00:56:20] ---- [run-pass] run-pass/thin-lto-global-allocator.rs stdout ----
[00:56:20] 
[00:56:20] error: compilation failed!
[00:56:20] status: exit code: 1
[00:56:20] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/thin-lto-global-allocator.rs" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/thin-lto-global-allocator/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "thinlto" "-C" "codegen-units=2" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/thin-lto-global-allocator/auxiliary"
[00:56:20] ------------------------------------------
[00:56:20] 
[00:56:20] ------------------------------------------
[00:56:20] stderr:
[00:56:20] stderr:
[00:56:20] ------------------------------------------
[00:56:20] error: linking with `cc` failed: exit code: 1
[00:56:20]   |
[00:56:20]   = note: "cc" "-Wl,--as-needed" "-Wl,-z,noexecstack" "-m64" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/thin-lto-global-allocator/a.thin_lto_global_allocator0-317d481089b8c8fe83113de504472633.rs.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/thin-lto-global-allocator/a.thin_lto_global_allocator1-317d481089b8c8fe83113de504472633.rs.rcgu.o" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/thin-lto-global-allocator/a" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/thin-lto-global-allocator/a.crate.allocator.rcgu.o" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro" "-Wl,-znow" "-Wl,-O1" "-nodefaultlibs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/thin-lto-global-allocator/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,--start-group" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-lstd-e054c7a28f8831a7" "-Wl,--end-group" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-2866e3dedc0a56e0.rlib" "-Wl,-Bdynamic" "-ldl" "-lrt" "-lpthread" "-lpthread" "-lgcc_s" "-lc" "-lm" "-lrt" "-lpthread" "-lutil" "-lutil" "-Wl,-rpath,$ORIGIN/../../../stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-rpath,/checkout/obj/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,--enable-new-dtags"
[00:56:20]   = note: /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/thin-lto-global-allocator/a.crate.allocatoD. 3042 passed; 4 failed; 10 ignored; 0 measured; 0 filtered out
[00:56:20] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:56:20] 
[00:56:20] 
[00:56:20] 
[00:56:20] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:56:20] 
[00:56:20] 
[00:56:20] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:56:20] Build completed unsuccessfully in 0:08:52
