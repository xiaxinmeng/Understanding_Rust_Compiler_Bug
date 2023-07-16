plain
[01:34:18] failures:
[01:34:18] 
[01:34:18] ---- [run-pass] run-pass/debuginfo-lto.rs stdout ----
[01:34:18] 
[01:34:18] error: test compilation failed although it shouldn't!
[01:34:18] status: exit code: 1
[01:34:18] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/debuginfo-lto.rs" "-Zthreads=1" "--target=arm-unknown-linux-gnueabihf" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/debuginfo-lto/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/arm-unknown-linux-gnueabihf/native/rust-test-helpers" "-Clinker=arm-linux-gnueabihf-gcc" "-C" "lto" "-g" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/debuginfo-lto/auxiliary"
[01:34:18] ------------------------------------------
[01:34:18] 
[01:34:18] ------------------------------------------
[01:34:18] stderr:
[01:34:18] stderr:
[01:34:18] ------------------------------------------
[01:34:18] error: linking with `arm-linux-gnueabihf-gcc` failed: exit code: 1
[01:34:18]    |
[01:34:18]    = note: "arm-linux-gnueabihf-gcc" "-Wl,--as-needed" "-Wl,-z,noexecstack" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-unknown-linux-gnueabihf/lib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/debuginfo-lto/a.debuginfo_lto.7rcbfp3g-cgu.1.rcgu.o" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/debuginfo-lto/a" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro" "-Wl,-znow" "-Wl,-O1" "-nodefaultlibs" "-L" "/checkout/obj/build/arm-unknown-linux-gnueabihf/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/debuginfo-lto/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-unknown-linux-gnueabihf/lib" "-Wl,--start-group" "-Wl,-Bstatic" "/tmp/rustcjmb7YR/libbacktrace_sys-69828ba5b177f6e2.rlib" "-Wl,--end-group" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-unknown-linux-gnueabihf/lib/libcompiler_builtins-34ad96caca8cd606.rlib" "-Wl,-Bdynamic" "-ldl" "-lrt" "-lpthread" "-lgcc_s" "-lc" "-lm" "-lrt" "-lpthread" "-lutil" "-lutil" "-Wl,-rpath,$ORIGIN/auxiliary" "-Wl,-rpath,$ORIGIN/../../../stage2/lib/rustlib/arm-unknown-linux-gnueabihf/lib" "-Wl,-rpath,/checkout/obj/lib/rustlib/arm-unknown-linux-gnueabihf/lib" "-Wl,--enable-new-dtags"
[01:34:18]    = note: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-unknown-linux-gnueabihf/lib/libcompiler_builtins-34ad96caca8cd606.rlib(compiler_builtins-34ad96caca8cd606.compiler_builtins.7arsbjs9-cgu.0.rcgu.o): In function `__aeabi_memcpy8':
[01:34:18]            compiler_builtins.7arsbjs9-cgu.0:(.text.__aeabi_memcpy4+0x6c): undefined reference to `core::panicking::panic::h747b6b606f8c4f43'
[01:34:18]            compiler_builtins.7arsbjs9-cgu.0:(.text.__aeabi_memcpy4+0x7c): undefined reference to `core::panicking::panic::h747b6b606f8c4f43'
[01:34:18]            
[01:34:18] 
[01:34:18] error: aborting due to previous error
[01:34:18] 
[01:34:18] 
[01:34:18] 
[01:34:18] ------------------------------------------
[01:34:18] 
[01:34:18] 
[01:34:18] ---- [run-pass] run-pass/fat-lto.rs stdout ----
[01:34:18] 
[01:34:18] error: test compilation failed although it shouldn't!
[01:34:18] status: exit code: 1
[01:34:18] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/fat-lto.rs" "-Zthreads=1" "--target=arm-unknown-linux-gnueabihf" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/fat-lto/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/arm-unknown-linux-gnueabihf/native/rust-test-helpers" "-Clinker=arm-linux-gnueabihf-gcc" "-Clto=fat" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/fat-lto/auxiliary"
[01:34:18] ------------------------------------------
[01:34:18] 
[01:34:18] ------------------------------------------
[01:34:18] stderr:
[01:34:18] stderr:
[01:34:18] ------------------------------------------
[01:34:18] error: linking with `arm-linux-gnueabihf-gcc` failed: exit code: 1
[01:34:18]    |
[01:34:18]    = note: "arm-linux-gnueabihf-gcc" "-Wl,--as-needed" "-Wl,-z,noexecstack" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-unknown-linux-gnueabihf/lib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/fat-lto/a.fat_lto.7rcbfp3g-cgu.0.rcgu.o" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/fat-lto/a" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro" "-Wl,-znow" "-Wl,-O1" "-nodefaultlibs" "-L" "/checkout/obj/build/arm-unknown-linux-gnueabihf/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/fat-lto/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-unknown-linux-gnueabihf/lib" "-Wl,--start-group" "-Wl,-Bstatic" "/tmp/rustc7plUzh/libbacktrace_sys-69828ba5b177f6e2.rlib" "-Wl,--end-group" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-unknown-linux-gnueabihf/lib/libcompiler_builtins-34ad96caca8cd606.rlib" "-Wl,-Bdynamic" "-ldl" "-lrt" "-lpthread" "-lgcc_s" "-lc" "-lm" "-lrt" "-lpthread" "-lutil" "-lutil" "-Wl,-rpath,$ORIGIN/../../../stage2/lib/rustlib/arm-unknown-linux-gnueabihf/lib" "-Wl,-rpath,/checkout/obj/lib/rustlib/arm-unknown-linux-gnueabihf/lib" "-Wl,--enable-new-dtags"
[01:34:18]    = note: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-unknown-linux-gnueabihf/lib/libcompiler_builtins-34ad96caca8cd606.rlib(compiler_builtins-34ad96caca8cd606.compiler_builtins.7arsbjs9-cgu.0.rcgu.o): In function `__aeabi_memcpy8':
[01:34:18]            compiler_builtins.7arsbjs9-cgu.0:(.text.__aeabi_memcpy4+0x6c): undefined reference to `core::panicking::panic::h747b6b606f8c4f43'
[01:34:18]            compiler_builtins.7arsbjs9-cgu.0:(.text.__aeabi_memcpy4+0x7c): undefined reference to `core::panicking::panic::h747b6b606f8c4f43'
[01:34:18]            
[01:34:18] 
[01:34:18] error: aborting due to previous error
[01:34:18] 
[01:34:18] 
[01:34:18] 
[01:34:18] ------------------------------------------
[01:34:18] 
[01:34:18] 
[01:34:18] ---- [run-pass] run-pass/lto-many-codegen-units.rs stdout ----
[01:34:18] 
[01:34:18] error: test compilation failed although it shouldn't!
[01:34:18] status: exit code: 1
[01:34:18] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/lto-many-codegen-units.rs" "-Zthreads=1" "--target=arm-unknown-linux-gnueabihf" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/lto-many-codegen-units/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/arm-unknown-linux-gnueabihf/native/rust-test-helpers" "-Clinker=arm-linux-gnueabihf-gcc" "-C" "lto" "-C" "codegen-units=8" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/lto-many-codegen-units/auxiliary"
[01:34:18] ------------------------------------------
[01:34:18] 
[01:34:18] ------------------------------------------
[01:34:18] stderr:
[01:34:18] stderr:
[01:34:18] ------------------------------------------
[01:34:18] error: linking with `arm-linux-gnueabihf-gcc` failed: exit code: 1
[01:34:18]    |
[01:34:18]    = note: "arm-linux-gnueabihf-gcc" "-Wl,--as-needed" "-Wl,-z,noexecstack" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-unknown-linux-gnueabihf/lib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/lto-many-codegen-units/a.lto_many_codegen_units.7rcbfp3g-cgu.0.rcgu.o" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/lto-many-codegen-units/a" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro" "-Wl,-znow" "-Wl,-O1" "-nodefaultlibs" "-L" "/checkout/obj/build/arm-unknown-linux-gnueabihf/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/lto-many-codegen-units/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-unknown-linux-gnueabihf/lib" "-Wl,--start-group" "-Wl,-Bstatic" "/tmp/rustc5lCwpB/libbacktrace_sys-69828ba5b177f6e2.rlib" "-Wl,--end-group" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-unknown-linux-gnueabihf/lib/libcompiler_builtins-34ad96caca8cd606.rlib" "-Wl,-Bdynamic" "-ldl" "-lrt" "-lpthread" "-lgcc_s" "-lc" "-lm" "-lrt" "-lpthread" "-lutil" "-lutil" "-Wl,-rpath,$ORIGIN/../../../stage2/lib/rustlib/arm-unknown-linux-gnueabihf/lib" "-Wl,-rpath,/checkout/obj/lib/rustlib/arm-unknown-linux-gnueabihf/lib" "-Wl,--enable-new-dtags"
[01:34:18]    = note: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-unknown-linux-gnueabihf/lib/libcompiler_builtins-34ad96caca8cd606.rlib(compiler_builtins-34ad96caca8cd606.compiler_builtins.7arsbjs9-cgu.0.rcgu.o): In function `__aeabi_memcpy8':
[01:34:18]            compiler_builtins.7arsbjs9-cgu.0:(.text.__aeabi_memcpy4+0x6c): undefined reference to `core::panicking::panic::h747b6b606f8c4f43'
[01:34:18]            compiler_builtins.7arsbjs9-cgu.0:(.text.__aeabi_memcpy4+0x7c): undefined reference to `core::panicking::panic::h747b6b606f8c4f43'
[01:34:18]            
[01:34:18] 
[01:34:18] error: aborting due to previous error
[01:34:18] 
[01:34:18] 
[01:34:18] 
[01:34:18] ------------------------------------------
[01:34:18] 
[01:34:18] 
[01:34:18] ---- [run-pass] run-pass/lto-still-runs-thread-dtors.rs stdout ----
[01:34:18] 
[01:34:18] error: test compilation failed although it shouldn't!
[01:34:18] status: exit code: 1
[01:34:18] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/lto-still-runs-thread-dtors.rs" "-Zthreads=1" "--target=arm-unknown-linux-gnueabihf" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/lto-still-runs-thread-dtors/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/arm-unknown-linux-gnueabihf/native/rust-test-helpers" "-Clinker=arm-linux-gnueabihf-gcc" "-C" "lto" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/lto-still-runs-thread-dtors/auxiliary"
[01:34:18] ------------------------------------------
[01:34:18] 
[01:34:18] ------------------------------------------
[01:34:18] stderr:
[01:34:18] stderr:
[01:34:18] ------------------------------------------
[01:34:18] error: linking with `arm-linux-gnueabihf-gcc` failed: exit code: 1
[01:34:18]    |
[01:34:18]    = note: "arm-linux-gnueabihf-gcc" "-Wl,--as-needed" "-Wl,-z,noexecstack" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-unknown-linux-gnueabihf/lib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/lto-still-runs-thread-dtors/a.lto_still_runs_thread_dtors.7rcbfp3g-cgu.3.rcgu.o" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/lto-still-runs-thread-dtors/a" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro" "-Wl,-znow" "-Wl,-O1" "-nodefaultlibs" "-L" "/checkout/obj/build/arm-unknown-linux-gnueabihf/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/lto-still-runs-thread-dtors/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-unknown-linux-gnueabihf/lib" "-Wl,--start-group" "-Wl,-Bstatic" "/tmp/rustcG8vxCI/libbacktrace_sys-69828ba5b177f6e2.rlib" "-Wl,--end-group" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-unknown-linux-gnueabihf/lib/libcompiler_builtins-34ad96caca8cd606.rlib" "-Wl,-Bdynamic" "-ldl" "-lrt" "-lpthread" "-lgcc_s" "-lc" "-lm" "-lrt" "-lpthread" "-lutil" "-lutil" "-Wl,-rpath,$ORIGIN/../../../stage2/lib/rustlib/arm-unknown-linux-gnueabihf/lib" "-Wl,-rpath,/checkout/obj/lib/rustlib/arm-unknown-linux-gnueabihf/lib" "-Wl,--enable-new-dtags"
[01:34:18]    = note: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-unknown-linux-gnueabihf/lib/libcompiler_builtins-34ad96caca8cd606.rlib(compiler_builtins-34ad96caca8cd606.compiler_builtins.7arsbjs9-cgu.0.rcgu.o): In function `__aeabi_memcpy8':
[01:34:18]            compiler_builtins.7arsbjs9-cgu.0:(.text.__aeabi_memcpy4+0x6c): undefined reference to `core::panicking::panic::h747b6b606f8c4f43'
[01:34:18]            compiler_builtins.7arsbjs9-cgu.0:(.text.__aeabi_memcpy4+0x7c): undefined reference to `core::panicking::panic::h747b6b606f8c4f43'
[01:34:18]            
[01:34:18] 
[01:34:18] error: aborting due to previous error
[01:34:18] 
[01:34:18] 
[01:34:18] 
[01:34:18] ------------------------------------------
[01:34:18] 
[01:34:18] 
[01:34:18] ---- [run-pass] run-pass/panic-runtime/lto-abort.rs stdout ----
[01:34:18] 
[01:34:18] error: test compilation failed although it shouldn't!
[01:34:18] status: exit code: 1
[01:34:18] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/panic-runtime/lto-abort.rs" "-Zthreads=1" "--target=arm-unknown-linux-gnueabihf" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/panic-runtime/lto-abort/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/arm-unknown-linux-gnueabihf/native/rust-test-helpers" "-Clinker=arm-linux-gnueabihf-gcc" "-C" "lto" "-C" "panic=abort" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/panic-runtime/lto-abort/auxiliary"
[01:34:18] ------------------------------------------
[01:34:18] 
[01:34:18] ------------------------------------------
[01:34:18] stderr:
[01:34:18] stderr:
[01:34:18] ------------------------------------------
[01:34:18] error: linking with `arm-linux-gnueabihf-gcc` failed: exit code: 1
[01:34:18]    |
[01:34:18]    = note: "arm-linux-gnueabihf-gcc" "-Wl,--as-needed" "-Wl,-z,noexecstack" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-unknown-linux-gnueabihf/lib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/panic-runtime/lto-abort/a.lto_abort.7rcbfp3g-cgu.9.rcgu.o" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/panic-runtime/lto-abort/a" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro" "-Wl,-znow" "-Wl,-O1" "-nodefaultlibs" "-L" "/checkout/obj/build/arm-unknown-linux-gnueabihf/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/panic-runtime/lto-abort/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-unknown-linux-gnueabihf/lib" "-Wl,--start-group" "-Wl,-Bstatic" "/tmp/rustcFy1AqJ/libbacktrace_sys-69828ba5b177f6e2.rlib" "-Wl,--end-group" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-unknown-linux-gnueabihf/lib/libcompiler_builtins-34ad96caca8cd606.rlib" "-Wl,-Bdynamic" "-ldl" "-lrt" "-lpthread" "-lgcc_s" "-lc" "-lm" "-lrt" "-lpthread" "-lutil" "-lutil" "-Wl,-rpath,$ORIGIN/../../../../stage2/lib/rustlib/arm-unknown-linux-gnueabihf/lib" "-Wl,-rpath,/checkout/obj/lib/rustlib/arm-unknown-linux-gnueabihf/lib" "-Wl,--enable-new-dtags"
[01:34:18]    = note: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-unknown-linux-gnueabihf/lib/libcompiler_builtins-34ad96caca8cd606.rlib(compiler_builtins-34ad96caca8cd606.compiler_builtins.7arsbjs9-cgu.0.rcgu.o): In function `__aeabi_memcpy8':
[01:34:18]            compiler_builtins.7arsbjs9-cgu.0:(.text.__aeabi_memcpy4+0x6c): undefined reference to `core::panicking::panic::h747b6b606f8c4f43'
[01:34:18]            compiler_builtins.7arsbjs9-cgu.0:(.text.__aeabi_memcpy4+0x7c): undefined reference to `core::panicking::panic::h747b6b606f8c4f43'
[01:34:18]            
[01:34:18] 
[01:34:18] error: aborting due to previous error
[01:34:18] 
[01:34:18] 
[01:34:18] 
[01:34:18] ------------------------------------------
[01:34:18] 
[01:34:18] 
[01:34:18] ---- [run-pass] run-pass/panic-runtime/lto-unwind.rs stdout ----
[01:34:18] 
[01:34:18] error: test compilation failed although it shouldn't!
[01:34:18] status: exit code: 1
[01:34:18] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/panic-runtime/lto-unwind.rs" "-Zthreads=1" "--target=arm-unknown-linux-gnueabihf" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/panic-runtime/lto-unwind/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/arm-unknown-linux-gnueabihf/native/rust-test-helpers" "-Clinker=arm-linux-gnueabihf-gcc" "-C" "lto" "-C" "panic=unwind" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/panic-runtime/lto-unwind/auxiliary"
[01:34:18] ------------------------------------------
[01:34:18] 
[01:34:18] ------------------------------------------
[01:34:18] stderr:
[01:34:18] stderr:
[01:34:18] ------------------------------------------
[01:34:18] error: linking with `arm-linux-gnueabihf-gcc` failed: exit code: 1
[01:34:18]    |
[01:34:18]    = note: "arm-linux-gnueabihf-gcc" "-Wl,--as-needed" "-Wl,-z,noexecstack" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-unknown-linux-gnueabihf/lib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/panic-runtime/lto-unwind/a.lto_unwind.7rcbfp3g-cgu.5.rcgu.o" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/panic-runtime/lto-unwind/a" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro" "-Wl,-znow" "-Wl,-O1" "-nodefaultlibs" "-L" "/checkout/obj/build/arm-unknown-linux-gnueabihf/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/panic-runtime/lto-unwind/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-unknown-linux-gnueabihf/lib" "-Wl,--start-group" "-Wl,-Bstatic" "/tmp/rustcsuc2xm/libbacktrace_sys-69828ba5b177f6e2.rlib" "-Wl,--end-group" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-unknown-linux-gnueabihf/lib/libcompiler_builtins-34ad96caca8cd606.rlib" "-Wl,-Bdynamic" "-ldl" "-lrt" "-lpthread" "-lgcc_s" "-lc" "-lm" "-lrt" "-lpthread" "-lutil" "-lutil" "-Wl,-rpath,$ORIGIN/../../../../stage2/lib/rustlib/arm-unknown-linux-gnueabihf/lib" "-Wl,-rpath,/checkout/obj/lib/rustlib/arm-unknown-linux-gnueabihf/lib" "-Wl,--enable-new-dtags"
[01:34:18]    = note: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-unknown-linux-gnueabihf/lib/libcompiler_builtins-34ad96caca8cd606.rlib(compiler_builtins-34ad96caca8cd606.compiler_builtins.7arsbjs9-cgu.0.rcgu.o): In function `__aeabi_memcpy8':
[01:34:18]            compiler_builtins.7arsbjs9-cgu.0:(.text.__aeabi_memcpy4+0x6c): undefined reference to `core::panicking::panic::h747b6b606f8c4f43'
[01:34:18]            compiler_builtins.7arsbjs9-cgu.0:(.text.__aeabi_memcpy4+0x7c): undefined reference to `core::panicking::panic::h747b6b606f8c4f43'
[01:34:18]            
[01:34:18] 
[01:34:18] error: aborting due to previous error
[01:34:18] 
[01:34:18] 
[01:34:18] 
[01:34:18] ------------------------------------------
[01:34:18] 
[01:34:18] 
[01:34:18] ---- [run-pass] run-pass/sepcomp/sepcomp-lib-lto.rs stdout ----
[01:34:18] 
[01:34:18] error: test compilation failed although it shouldn't!
[01:34:18] status: exit code: 1
[01:34:18] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/sepcomp/sepcomp-lib-lto.rs" "-Zthreads=1" "--target=arm-unknown-linux-gnueabihf" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/sepcomp/sepcomp-lib-lto/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/arm-unknown-linux-gnueabihf/native/rust-test-helpers" "-Clinker=arm-linux-gnueabihf-gcc" "-C" "lto" "-g" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/sepcomp/sepcomp-lib-lto/auxiliary"
[01:34:18] ------------------------------------------
[01:34:18] 
[01:34:18] ------------------------------------------
[01:34:18] stderr:
[01:34:18] stderr:
[01:34:18] ------------------------------------------
[01:34:18] error: linking with `arm-linux-gnueabihf-gcc` failed: exit code: 1
[01:34:18]    |
[01:34:18]    = note: "arm-linux-gnueabihf-gcc" "-Wl,--as-needed" "-Wl,-z,noexecstack" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-unknown-linux-gnueabihf/lib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/sepcomp/sepcomp-lib-lto/a.sepcomp_lib_lto.7rcbfp3g-cgu.0.rcgu.o" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/sepcomp/sepcomp-lib-lto/a" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro" "-Wl,-znow" "-Wl,-O1" "-nodefaultlibs" "-L" "/checkout/obj/build/arm-unknown-linux-gnueabihf/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/sepcomp/sepcomp-lib-lto/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-unknown-linux-gnueabihf/lib" "-Wl,--start-group" "-Wl,-Bstatic" "/tmp/rustcjL9xI6/libbacktrace_sys-69828ba5b177f6e2.rlib" "-Wl,--end-group" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-unknown-linux-gnueabihf/lib/libcompiler_builtins-34ad96caca8cd606.rlib" "-Wl,-Bdynamic" "-ldl" "-lrt" "-lpthread" "-lgcc_s" "-lc" "-lm" "-lrt" "-lpthread" "-lutil" "-lutil" "-Wl,-rpath,$ORIGIN/auxiliary" "-Wl,-rpath,$ORIGIN/../../../../stage2/lib/rustlib/arm-unknown-linux-gnueabihf/lib" "-Wl,-rpath,/checkout/obj/lib/rustlib/arm-unknown-linux-gnueabihf/lib" "-Wl,--enable-new-dtags"
[01:34:18]    = note: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-unknown-linux-gnueabihf/lib/libcompiler_builtins-34ad96caca8cd606.rlib(compiler_builtins-34ad96caca8cd606.compiler_builtins.7arsbjs9-cgu.0.rcgu.o): In function `__aeabi_memcpy8':
[01:34:18]            compiler_builtins.7arsbjs9-cgu.0:(.text.__aeabi_memcpy4+0x6c): undefined reference to `core::panicking::panic::h747b6b606f8c4f43'
[01:34:18]            compiler_builtins.7arsbjs9-cgu.0:(.text.__aeabi_memcpy4+0x7c): undefined reference to `core::panicking::panic::h747b6b606f8c4f43'
[01:34:18]            
[01:34:18] 
[01:34:18] error: aborting due to previous error
[01:34:18] 
---
[01:34:18] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:521:22
[01:34:18] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:34:18] 
[01:34:18] 
[01:34:18] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-unknown-linux-gnueabihf/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-arm-unknown-linux-gnueabihf" "--mode" "run-pass" "--target" "arm-unknown-linux-gnueabihf" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "arm-linux-gnueabihf-gcc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/arm-unknown-linux-gnueabihf/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--llvm-version" "8.0.0-rust-1.37.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--remote-test-client" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/remote-test-client" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:34:18] 
[01:34:18] 
[01:34:18] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target arm-unknown-linux-gnueabihf
[01:34:18] Build completed unsuccessfully in 1:31:17
---
travis_time:end:068c72d0:start=1561645746476621532,finish=1561645746483687575,duration=7066043
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:081167a3
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:173f296c
travis_time:start:173f296c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:033fd850
$ dmesg | grep -i kill
