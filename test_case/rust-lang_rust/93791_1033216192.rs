plain
    Finished release [optimized] target(s) in 12.78s
Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 229 tests
..........i.ii....ii......................F......................................................... 100/229
.................i...................iiiiiii..F...i...................iii........................... 200/229
failures:
Some tests failed in compiletest suite=run-make-fulldeps mode=run-make host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

---- [run-make] run-make-fulldeps/extern-fn-generic stdout ----
---- [run-make] run-make-fulldeps/extern-fn-generic stdout ----

error: make failed
status: exit status: 2
command: "make"
stdout:
------------------------------------------
cc -ffunction-sections -fdata-sections -fPIC -m64 -c -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-fn-generic/extern-fn-generic/libtest.o test.c
ar crus /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-fn-generic/extern-fn-generic/libtest.a /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-fn-generic/extern-fn-generic/libtest.o
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-fn-generic/extern-fn-generic:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-fn-generic/extern-fn-generic -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-fn-generic/extern-fn-generic  testcrate.rs
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-fn-generic/extern-fn-generic:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-fn-generic/extern-fn-generic -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-fn-generic/extern-fn-generic  test.rs
rm /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-fn-generic/extern-fn-generic/libtest.o
------------------------------------------
stderr:
------------------------------------------
ar: `u' modifier ignored since `D' is the default (see `U')
ar: `u' modifier ignored since `D' is the default (see `U')
error: linking with `cc` failed: exit status: 1
  |
  = note: "cc" "-m64" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-fn-generic/extern-fn-generic/test.test.b2d137c1-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-fn-generic/extern-fn-generic/test.test.b2d137c1-cgu.1.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-fn-generic/extern-fn-generic/test.test.b2d137c1-cgu.10.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-fn-generic/extern-fn-generic/test.test.b2d137c1-cgu.2.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-fn-generic/extern-fn-generic/test.test.b2d137c1-cgu.3.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-fn-generic/extern-fn-generic/test.test.b2d137c1-cgu.4.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-fn-generic/extern-fn-generic/test.test.b2d137c1-cgu.5.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-fn-generic/extern-fn-generic/test.test.b2d137c1-cgu.6.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-fn-generic/extern-fn-generic/test.test.b2d137c1-cgu.7.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-fn-generic/extern-fn-generic/test.test.b2d137c1-cgu.8.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-fn-generic/extern-fn-generic/test.test.b2d137c1-cgu.9.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-fn-generic/extern-fn-generic/test.2k7bs2fd4dv6n3lm.rcgu.o" "-Wl,--as-needed" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-fn-generic/extern-fn-generic" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-Bstatic" "-Wl,--whole-archive" "-ltest" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-fn-generic/extern-fn-generic/libtestcrate.rlib" "-Wl,--no-whole-archive" "-Wl,--start-group" "-Wl,--whole-archive" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-bd4d981d8a81bf84.rlib" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-f663e0e2fd3514d9.rlib" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libminiz_oxide-1ef813edcf034814.rlib" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libadler-31cc24455faf05ae.rlib" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libobject-37b6e6f45f0b8568.rlib" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libmemchr-b69201f842c667b3.rlib" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libaddr2line-4b7ad2ffd31eee24.rlib" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgimli-8e2fa439c7cd8422.rlib" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_demangle-144ff24f90f4403f.rlib" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd_detect-36a2845ff32ef6cb.rlib" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libhashbrown-15e079c738929d4a.rlib" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_alloc-e2a2a36bc40170a2.rlib" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-c3ea56044af7b95f.rlib" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcfg_if-af8cb4557e2888e2.rlib" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-e998257ebeaf724f.rlib" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-64320117df9543a5.rlib" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-2deeba5fb4de62b0.rlib" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-a2a7040fb9f918eb.rlib" "-Wl,--no-whole-archive" "-Wl,--end-group" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-89964d8ba43e75fe.rlib" "-Wl,-Bdynamic" "-lgcc_s" "-lutil" "-lrt" "-lpthread" "-lm" "-ldl" "-lc" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-fn-generic/extern-fn-generic/test" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro,-znow" "-nodefaultlibs"
  = note: /usr/bin/ld: /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-fn-generic/extern-fn-generic/libtestcrate.rlib(libtest.o): in function `call':
          test.c:(.text.call+0x0): multiple definition of `call'; /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-fn-generic/extern-fn-generic/libtest.a(libtest.o):test.c:(.text.call+0x0): first defined here
          collect2: error: ld returned 1 exit status

error: aborting due to previous error


make: *** [Makefile:5: all] Error 1
------------------------------------------


---- [run-make] run-make-fulldeps/lto-no-link-whole-rlib stdout ----
---- [run-make] run-make-fulldeps/lto-no-link-whole-rlib stdout ----

error: make failed
status: exit status: 2
command: "make"
stdout:
------------------------------------------
cc -ffunction-sections -fdata-sections -fPIC -m64 -c -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/lto-no-link-whole-rlib/lto-no-link-whole-rlib/libfoo.o foo.c
ar crus /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/lto-no-link-whole-rlib/lto-no-link-whole-rlib/libfoo.a /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/lto-no-link-whole-rlib/lto-no-link-whole-rlib/libfoo.o
cc -ffunction-sections -fdata-sections -fPIC -m64 -c -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/lto-no-link-whole-rlib/lto-no-link-whole-rlib/libbar.o bar.c
ar crus /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/lto-no-link-whole-rlib/lto-no-link-whole-rlib/libbar.a /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/lto-no-link-whole-rlib/lto-no-link-whole-rlib/libbar.o
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/lto-no-link-whole-rlib/lto-no-link-whole-rlib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/lto-no-link-whole-rlib/lto-no-link-whole-rlib -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/lto-no-link-whole-rlib/lto-no-link-whole-rlib  lib1.rs
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/lto-no-link-whole-rlib/lto-no-link-whole-rlib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/lto-no-link-whole-rlib/lto-no-link-whole-rlib -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/lto-no-link-whole-rlib/lto-no-link-whole-rlib  lib2.rs
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/lto-no-link-whole-rlib/lto-no-link-whole-rlib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/lto-no-link-whole-rlib/lto-no-link-whole-rlib -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/lto-no-link-whole-rlib/lto-no-link-whole-rlib  main.rs -Clto
rm /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/lto-no-link-whole-rlib/lto-no-link-whole-rlib/libfoo.o /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/lto-no-link-whole-rlib/lto-no-link-whole-rlib/libbar.o
------------------------------------------
stderr:
------------------------------------------
ar: `u' modifier ignored since `D' is the default (see `U')
ar: `u' modifier ignored since `D' is the default (see `U')
ar: `u' modifier ignored since `D' is the default (see `U')
error: linking with `cc` failed: exit status: 1
  |
  = note: "cc" "-m64" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/lto-no-link-whole-rlib/lto-no-link-whole-rlib/main.main.cbcd4161-cgu.8.rcgu.o" "-Wl,--as-needed" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/lto-no-link-whole-rlib/lto-no-link-whole-rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-Bstatic" "-Wl,--whole-archive" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/lto-no-link-whole-rlib/lto-no-link-whole-rlib/rustcmQCOBd/liblib2.rlib" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/lto-no-link-whole-rlib/lto-no-link-whole-rlib/rustcmQCOBd/liblib1.rlib" "-Wl,--no-whole-archive" "-Wl,--start-group" "-Wl,--end-group" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-89964d8ba43e75fe.rlib" "-Wl,-Bdynamic" "-lgcc_s" "-lutil" "-lrt" "-lpthread" "-lm" "-ldl" "-lc" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/lto-no-link-whole-rlib/lto-no-link-whole-rlib/main" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro,-znow" "-nodefaultlibs"
  = note: /usr/bin/ld: /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/lto-no-link-whole-rlib/lto-no-link-whole-rlib/rustcmQCOBd/liblib1.rlib(libfoo.o): in function `foo':
          foo.c:(.text.foo+0x0): multiple definition of `foo'; /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/lto-no-link-whole-rlib/lto-no-link-whole-rlib/rustcmQCOBd/liblib2.rlib(libbar.o):bar.c:(.text.foo+0x0): first defined here
          collect2: error: ld returned 1 exit status

error: aborting due to previous error


make: *** [Makefile:6: all] Error 1
------------------------------------------



