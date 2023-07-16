
---- [run-pass] run-pass/debuginfo-lto.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/debuginfo-lto.rs" "-Zthreads=1" "--target=arm-unknown-linux-gnueabihf" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/debuginfo-lto" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/arm-unknown-linux-gnueabihf/native/rust-test-helpers" "-Clinker=arm-linux-gnueabihf-gcc" "-C" "lto" "-g" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/debuginfo-lto/auxiliary"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
error: linking with `arm-linux-gnueabihf-gcc` failed: exit code: 1
   |
   = note: "arm-linux-gnueabihf-gcc" "-Wl,--as-needed" "-Wl,-z,noexecstack" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-unknown-linux-gnueabihf/lib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/debuginfo-lto/debuginfo-lto.debuginfo_lto.7rcbfp3g-cgu.1.rcgu.o" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/debuginfo-lto/debuginfo-lto" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro" "-Wl,-znow" "-Wl,-O1" "-nodefaultlibs" "-L" "/checkout/obj/build/arm-unknown-linux-gnueabihf/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/debuginfo-lto/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-unknown-linux-gnueabihf/lib" "-Wl,--start-group" "-Wl,-Bstatic" "/tmp/rustcwZ9vAg/libbacktrace_sys-69828ba5b177f6e2.rlib" "-Wl,--end-group" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-unknown-linux-gnueabihf/lib/libcompiler_builtins-34ad96caca8cd606.rlib" "-Wl,-Bdynamic" "-ldl" "-lrt" "-lpthread" "-lgcc_s" "-lc" "-lm" "-lrt" "-lpthread" "-lutil" "-lutil" "-Wl,-rpath,$ORIGIN/auxiliary" "-Wl,-rpath,$ORIGIN/../../../stage2/lib/rustlib/arm-unknown-linux-gnueabihf/lib" "-Wl,-rpath,/checkout/obj/lib/rustlib/arm-unknown-linux-gnueabihf/lib" "-Wl,--enable-new-dtags"
   = note: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/arm-unknown-linux-gnueabihf/lib/libcompiler_builtins-34ad96caca8cd606.rlib(compiler_builtins-34ad96caca8cd606.compiler_builtins.7arsbjs9-cgu.0.rcgu.o): In function `__aeabi_memcpy8':
           compiler_builtins.7arsbjs9-cgu.0:(.text.__aeabi_memcpy4+0x6c): undefined reference to `core::panicking::panic::h747b6b606f8c4f43'
           compiler_builtins.7arsbjs9-cgu.0:(.text.__aeabi_memcpy4+0x7c): undefined reference to `core::panicking::panic::h747b6b606f8c4f43'
           collect2: error: ld returned 1 exit status
           

error: aborting due to previous error
