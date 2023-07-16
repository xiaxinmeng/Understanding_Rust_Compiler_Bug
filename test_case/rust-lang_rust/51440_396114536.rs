plain
[01:09:13] ---- [run-pass] run-pass/extern-prelude-core.rs stdout ----
[01:09:13] 
[01:09:13] error: compilation failed!
[01:09:13] status: exit code: 101
[01:09:13] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/extern-prelude-core.rs" "--target=i686-unknown-linux-musl" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/extern-prelude-core/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/i686-unknown-linux-musl/native/rust-test-helpers" "-Clinker=/musl-i686/bin/musl-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/extern-prelude-core/auxiliary"
[01:09:13] ------------------------------------------
[01:09:13] 
[01:09:13] ------------------------------------------
[01:09:13] stderr:
[01:09:13] stderr:
[01:09:13] ------------------------------------------
[01:09:13] error: linking with `/musl-i686/bin/musl-gcc` failed: exit code: 1
[01:09:13]   |
[01:09:13]   = note: "/musl-i686/bin/musl-gcc" "-Wl,--as-needed" "-Wl,-z,noexecstack" "-Wl,--eh-frame-hdr" "-Wl,-(" "-m32" "-Wl,-melf_i386" "-nostdlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-musl/lib/crt1.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-musl/lib/crti.o" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-musl/lib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/extern-prelude-core/a.extern_prelude_core0-317d481089b8c8fe83113de504472633.rs.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/extern-prelude-core/a.extern_prelude_core1-317d481089b8c8fe83113de504472633.rs.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/extern-prelude-core/a.extern_prelude_core2-317d481089b8c8fe83113de504472633.rs.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/extern-prelude-core/a.extern_prelude_core3-317d481089b8c8fe83113de504472633.rs.rcgu.o" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/extern-prelude-core/a" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/extern-prelude-core/a.crate.allocator.rcgu.o" "-Wl,--gc-sections" "-no-pie" "-Wl,-z,relro,-z,now" "-Wl,-O1" "-nodefaultlibs" "-L" "/checkout/obj/build/i686-unknown-linux-musl/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/extern-prelude-core/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-musl/lib" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-musl/lib/liballoc_jemalloc-bc383e0f057203ff.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-musl/lib/libstd-6a80ec8be7cd108e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-musl/lib/libpanic_unwind-9dc4a36381d3e96f.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-musl/lib/libunwind-cb8f79e68e7bccd2.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-musl/lib/liballoc_system-5280629c3db400fb.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-musl/lib/liblibc-e341d15b13410457.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-musl/lib/liballoc-5d430cabf0a1afd6.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-musl/lib/libcore-28a7cc3bf4217758.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-musl/lib/libcompiler_builtins-e2fcc1d0db99a66e.rlib" "-static" "-Wl,-rpath,/checkout/obj/lib/rustlib/i686-unknown-linux-musl/lib" "-Wl,--enable-new-dtags" "-Wl,-Bdynamic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-musl/lib/crtn.o" "-Wl,-)"
[01:09:13]   = note: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-musl/lib/libcompiler_builtins-e2fcc1d0db99a66e.rlib(compiler_builtins-e2fcc1d0db99a66e.compiler_builtins12-8730ddb7c7ab62a37c92724f5ac1c1d7.rs.rcgu.o): In function `memcmp':
[01:09:13]           /checkout/src/rustc/compiler_builtins_shim/../../libcompiler_builtins/src/mem.rs:55: multiple definition of `memcmp'
[01:09:13]           /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-musl/lib/liblibc-e341d15b13410457.rlib(memcmp.lo):/build/musl-1.1.18/src/string/memcmp.c:4: first defined here
[01:09:13]           /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-musl/lib/libcompiler_builtins-e2fcc1d0db99a66e.rlib(compiler_builtins-e2fcc1d0db99a66e.compiler_builtins12-8730ddb7c7ab62a37c92724f5ac1c1d7.rs.rcgu.o): In function `memcpy':
[01:09:13]           /checkout/src/rustc/compiler_builtins_shim/../../libcompiler_builtins/src/mem.rs:9: multiple definition of `memcpy'
[01:09:13]           /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-musl/lib/liblibc-e341d15b13410457.rlib(memcpy.lo):/build/musl-1.1.18/src/string/i386/memcpy.s:7: first defined here
[01:09:13]           /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-musl/lib/libcompiler_builtins-e2fcc1d0db99a66e.rlib(compiler_builtins-e2fcc1d0db99a66e.compiler_builtins12-8730ddb7c7ab62a37c92724f5ac1c1d7.rs.rcgu.o): In function `memcpy':
[01:09:13]           /checkout/src/rustc/compiler_builtins_shim/../../libcompiler_builtins/src/mem.rs:9: multiple definition of `memmove'
[01:09:13]           /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-musl/lib/liblibc-e341d15b13410457.rlib(memmove.lo):/build/musl-1.1.18/src/string/i386/memmove.s:4: first defined here
[01:09:13]           collect2: error: ld returned 1 exit status
[01:09:13] 
[01:09:13] error: aborting due to previous error
[01:09:13] 
[01:09:13] 
---
[01:09:13] 
[01:09:13] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[01:09:13] 
[01:09:13] 
[01:09:13] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-musl/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-i686-unknown-linux-musl" "--mode" "run-pass" "--target" "i686-unknown-linux-musl" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "/musl-i686/bin/musl-gcc" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/i686-unknown-linux-musl/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "6.0.1\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:09:13] 
[01:09:13] 
[01:09:13] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target i586-unknown-linux-gnu,i686-unknown-linux-musl
[01:09:13] Build completed unsuccessfully in 1:06:19
---
travis_time:end:11a949fe:start=1528688566535580383,finish=1528688566542637180,duration=7056797
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:33d1d173
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:03182a24
$ dmesg | grep -i kill
