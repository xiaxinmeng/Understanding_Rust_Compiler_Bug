plain
test [run-make] src/test/run-make/pass-linker-flags-from-dep ... ok
test [run-make] src/test/run-make/emit-path-unhashed ... ok
test [run-make] src/test/run-make/emit-named-files ... ok
test [run-make] src/test/run-make/static-pie ... ignored
test [run-make] src/test/run-make/rlib-format-packed-bundled-libs-2 ... FAILED
test [run-make] src/test/run-make/remap-path-prefix-dwarf ... ok
test [run-make] src/test/run-make/env-dep-info ... ok
test [run-make] src/test/run-make/unstable-flag-required ... ok
test [run-make] src/test/run-make/wasm-abi ... ignored
---
test [run-make] src/test/run-make/emit-shared-files ... ok
test [run-make] src/test/run-make/rustdoc-scrape-examples-multiple ... ok
test [run-make] src/test/run-make/rustdoc-scrape-examples-ordering ... ok
test [run-make] src/test/run-make/thumb-none-cortex-m ... ok
Some tests failed in compiletest suite=run-make mode=run-make host=x86_64-unknown-linux-gnu target=thumbv6m-none-eabi

failures:


---- [run-make] src/test/run-make/rlib-format-packed-bundled-libs-2 stdout ----
error: make failed
status: exit status: 2
command: "make"
--- stdout -------------------------------
--- stdout -------------------------------
# Build strange-named dep
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs-2/rlib-format-packed-bundled-libs-2:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs-2/rlib-format-packed-bundled-libs-2 -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs-2/rlib-format-packed-bundled-libs-2  -Clinker='arm-none-eabi-gcc' native_dep.rs --crate-type=staticlib -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs-2/rlib-format-packed-bundled-libs-2/native_dep.ext
# Build new rlibs
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs-2/rlib-format-packed-bundled-libs-2:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs-2/rlib-format-packed-bundled-libs-2 -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs-2/rlib-format-packed-bundled-libs-2  -Clinker='arm-none-eabi-gcc' rust_dep.rs --crate-type=rlib -Zpacked_bundled_libs
"/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin"/llvm-nm /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs-2/rlib-format-packed-bundled-libs-2/librust_dep.rlib | "/checkout/src/etc/cat-and-grep.sh" "U native_f1"
[[[ begin stdout ]]]
lib.rmeta:


rust_dep.rust_dep.f8908835-cgu.0.rcgu.o:
                 U _ZN4core9panicking5panic17h5c795e070b17f101E
---------------- T _ZN8rust_dep8rust_dep17h46da1ffdf057d344E
                 U native_f1

[[[ end stdout ]]]
arm-none-eabi-ar t /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs-2/rlib-format-packed-bundled-libs-2/librust_dep.rlib | "/checkout/src/etc/cat-and-grep.sh" "native_dep.ext"
[[[ begin stdout ]]]
lib.rmeta
rust_dep.rust_dep.f8908835-cgu.0.rcgu.o
native_dep.ext

[[[ end stdout ]]]
# Check extract
rm /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs-2/rlib-format-packed-bundled-libs-2/native_dep.ext
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs-2/rlib-format-packed-bundled-libs-2:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs-2/rlib-format-packed-bundled-libs-2 -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs-2/rlib-format-packed-bundled-libs-2  -Clinker='arm-none-eabi-gcc' main.rs --extern rust_dep=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs-2/rlib-format-packed-bundled-libs-2/librust_dep.rlib -Zpacked_bundled_libs
--- stderr -------------------------------
warning: ignoring --out-dir flag due to -o flag

warning: 1 warning emitted
warning: 1 warning emitted

/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs-2/rlib-format-packed-bundled-libs-2/librust_dep.rlib:lib.rmeta: no symbols
error: linking with `arm-none-eabi-gcc` failed: exit status: 1
  |
  = note: "arm-none-eabi-gcc" "-m64" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs-2/rlib-format-packed-bundled-libs-2/rustcAGu7GZ/symbols.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs-2/rlib-format-packed-bundled-libs-2/main.main.cbcd4161-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs-2/rlib-format-packed-bundled-libs-2/main.main.cbcd4161-cgu.1.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs-2/rlib-format-packed-bundled-libs-2/main.main.cbcd4161-cgu.2.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs-2/rlib-format-packed-bundled-libs-2/main.main.cbcd4161-cgu.3.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs-2/rlib-format-packed-bundled-libs-2/main.main.cbcd4161-cgu.4.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs-2/rlib-format-packed-bundled-libs-2/main.4g86d6gt786od2ny.rcgu.o" "-Wl,--as-needed" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs-2/rlib-format-packed-bundled-libs-2" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs-2/rlib-format-packed-bundled-libs-2/librust_dep.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs-2/rlib-format-packed-bundled-libs-2/rustcAGu7GZ/native_dep.ext" "-Wl,--start-group" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-bdaf92b78f291e95.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-e13cbb326bcd01a4.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libobject-13ac6af5403a52c8.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libmemchr-a7b8febdd2acb289.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libaddr2line-343513f0726f71ed.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgimli-8f833d900bfb98aa.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_demangle-e97a7960ca6216c8.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd_detect-683fb35093a61fcc.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libhashbrown-f943c2d34bd4b56d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libminiz_oxide-72ce2aaa649404e0.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libadler-ac5d08ad5339e92e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_alloc-dacfda262d5656fb.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-e2056a834ba0712c.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcfg_if-a60649c148c6e2db.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-3e961d059b9bcde7.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-20f26f875d0170e2.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-522518611024dce5.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-05898138a596088a.rlib" "-Wl,--end-group" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-5b83a1df856cf582.rlib" "-Wl,-Bdynamic" "-lgcc_s" "-lutil" "-lrt" "-lpthread" "-lm" "-ldl" "-lc" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs-2/rlib-format-packed-bundled-libs-2/main" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro,-znow" "-nodefaultlibs"
  = note: arm-none-eabi-gcc: error: unrecognized command line option '-m64'

error: aborting due to previous error


make: *** [Makefile:18: all] Error 1



---- [run-make] src/test/run-make/rlib-format-packed-bundled-libs stdout ----
error: make failed
status: exit status: 2
command: "make"
--- stdout -------------------------------
--- stdout -------------------------------
# Build native dependencies
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs  -Clinker='arm-none-eabi-gcc' native_dep_1.rs --crate-type=staticlib
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs  -Clinker='arm-none-eabi-gcc' native_dep_2.rs --crate-type=staticlib
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs  -Clinker='arm-none-eabi-gcc' native_dep_3.rs --crate-type=staticlib
# Build new rlibs
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs  -Clinker='arm-none-eabi-gcc' rust_dep_up.rs --crate-type=rlib -Zpacked_bundled_libs
"/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin"/llvm-nm /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/librust_dep_up.rlib | "/checkout/src/etc/cat-and-grep.sh" "U native_f2"
[[[ begin stdout ]]]
lib.rmeta:


rust_dep_up.rust_dep_up.9cf8be89-cgu.0.rcgu.o:
---------------- T _ZN11rust_dep_up11rust_dep_up17h591fcb77681d961dE
                 U _ZN4core9panicking5panic17h5c795e070b17f101E
                 U native_f2
                 U native_f3

[[[ end stdout ]]]
"/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin"/llvm-nm /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/librust_dep_up.rlib | "/checkout/src/etc/cat-and-grep.sh" "U native_f3"
[[[ begin stdout ]]]
lib.rmeta:


rust_dep_up.rust_dep_up.9cf8be89-cgu.0.rcgu.o:
---------------- T _ZN11rust_dep_up11rust_dep_up17h591fcb77681d961dE
                 U _ZN4core9panicking5panic17h5c795e070b17f101E
                 U native_f2
                 U native_f3

[[[ end stdout ]]]
"/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin"/llvm-nm /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/librust_dep_up.rlib | "/checkout/src/etc/cat-and-grep.sh" -e "T.*rust_dep_up"
[[[ begin stdout ]]]
lib.rmeta:


rust_dep_up.rust_dep_up.9cf8be89-cgu.0.rcgu.o:
---------------- T _ZN11rust_dep_up11rust_dep_up17h591fcb77681d961dE
                 U _ZN4core9panicking5panic17h5c795e070b17f101E
                 U native_f2
                 U native_f3

[[[ end stdout ]]]
arm-none-eabi-ar t /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/librust_dep_up.rlib | "/checkout/src/etc/cat-and-grep.sh" "libnative_dep_2.a"
[[[ begin stdout ]]]
lib.rmeta
rust_dep_up.rust_dep_up.9cf8be89-cgu.0.rcgu.o
libnative_dep_2.a
libnative_dep_3.a

[[[ end stdout ]]]
arm-none-eabi-ar t /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/librust_dep_up.rlib | "/checkout/src/etc/cat-and-grep.sh" "libnative_dep_3.a"
[[[ begin stdout ]]]
lib.rmeta
rust_dep_up.rust_dep_up.9cf8be89-cgu.0.rcgu.o
libnative_dep_2.a
libnative_dep_3.a

[[[ end stdout ]]]
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs  -Clinker='arm-none-eabi-gcc' rust_dep_local.rs --extern rlib=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/librust_dep_up.rlib -Zpacked_bundled_libs --crate-type=rlib
"/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin"/llvm-nm /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/librust_dep_local.rlib | "/checkout/src/etc/cat-and-grep.sh" "U native_f1"
[[[ begin stdout ]]]
lib.rmeta:


rust_dep_local.rust_dep_local.17a839ca-cgu.0.rcgu.o:
                 U _ZN11rust_dep_up11rust_dep_up17h591fcb77681d961dE
---------------- T _ZN14rust_dep_local14rust_dep_local17h9562439c45fb1788E
                 U _ZN4core9panicking5panic17h5c795e070b17f101E
                 U native_f1

[[[ end stdout ]]]
"/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin"/llvm-nm /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/librust_dep_local.rlib | "/checkout/src/etc/cat-and-grep.sh" -e "T.*rust_dep_local"
[[[ begin stdout ]]]
lib.rmeta:


rust_dep_local.rust_dep_local.17a839ca-cgu.0.rcgu.o:
                 U _ZN11rust_dep_up11rust_dep_up17h591fcb77681d961dE
---------------- T _ZN14rust_dep_local14rust_dep_local17h9562439c45fb1788E
                 U _ZN4core9panicking5panic17h5c795e070b17f101E
                 U native_f1

[[[ end stdout ]]]
arm-none-eabi-ar t /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/librust_dep_local.rlib | "/checkout/src/etc/cat-and-grep.sh" "libnative_dep_1.a"
[[[ begin stdout ]]]
lib.rmeta
rust_dep_local.rust_dep_local.17a839ca-cgu.0.rcgu.o
libnative_dep_1.a

[[[ end stdout ]]]
rm /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/libnative_dep_1.a
rm /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/libnative_dep_2.a
rm /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/libnative_dep_3.a
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs  -Clinker='arm-none-eabi-gcc' main.rs --extern lib=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/librust_dep_local.rlib --crate-type=bin -Zpacked_bundled_libs --print link-args | "/checkout/src/etc/cat-and-grep.sh" -e "libnative_dep_1.a.*libnative_dep_2.a.*libnative_dep_3.a"
[[[ begin stdout ]]]
"arm-none-eabi-gcc" "-m64" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/rustcRgSd3z/symbols.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/main.main.cbcd4161-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/main.main.cbcd4161-cgu.1.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/main.main.cbcd4161-cgu.2.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/main.main.cbcd4161-cgu.3.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/main.main.cbcd4161-cgu.4.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/main.4g86d6gt786od2ny.rcgu.o" "-Wl,--as-needed" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/librust_dep_local.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/rustcRgSd3z/libnative_dep_1.a" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/librust_dep_up.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/rustcRgSd3z/libnative_dep_2.a" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/rustcRgSd3z/libnative_dep_3.a" "-Wl,--start-group" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-bdaf92b78f291e95.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-e13cbb326bcd01a4.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libobject-13ac6af5403a52c8.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libmemchr-a7b8febdd2acb289.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libaddr2line-343513f0726f71ed.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgimli-8f833d900bfb98aa.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_demangle-e97a7960ca6216c8.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd_detect-683fb35093a61fcc.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libhashbrown-f943c2d34bd4b56d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libminiz_oxide-72ce2aaa649404e0.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libadler-ac5d08ad5339e92e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_alloc-dacfda262d5656fb.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-e2056a834ba0712c.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcfg_if-a60649c148c6e2db.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-3e961d059b9bcde7.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-20f26f875d0170e2.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-522518611024dce5.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-05898138a596088a.rlib" "-Wl,--end-group" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-5b83a1df856cf582.rlib" "-Wl,-Bdynamic" "-lgcc_s" "-lutil" "-lrt" "-lpthread" "-lm" "-ldl" "-lc" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/main" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro,-znow" "-nodefaultlibs"

[[[ end stdout ]]]
# Build bin
"/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin"/llvm-nm /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/main | "/checkout/src/etc/cat-and-grep.sh" "T native_f1"
[[[ begin stdout ]]]

[[[ end stdout ]]]
Error: cannot match: T native_f1
--- stderr -------------------------------
--- stderr -------------------------------
/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/librust_dep_up.rlib:lib.rmeta: no symbols
/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/librust_dep_up.rlib:lib.rmeta: no symbols
/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/librust_dep_up.rlib:lib.rmeta: no symbols
/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/librust_dep_local.rlib:lib.rmeta: no symbols
/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/librust_dep_local.rlib:lib.rmeta: no symbols
error: linking with `arm-none-eabi-gcc` failed: exit status: 1
  |
  = note: "arm-none-eabi-gcc" "-m64" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/rustcRgSd3z/symbols.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/main.main.cbcd4161-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/main.main.cbcd4161-cgu.1.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/main.main.cbcd4161-cgu.2.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/main.main.cbcd4161-cgu.3.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/main.main.cbcd4161-cgu.4.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/main.4g86d6gt786od2ny.rcgu.o" "-Wl,--as-needed" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/librust_dep_local.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/rustcRgSd3z/libnative_dep_1.a" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/librust_dep_up.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/rustcRgSd3z/libnative_dep_2.a" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/rustcRgSd3z/libnative_dep_3.a" "-Wl,--start-group" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-bdaf92b78f291e95.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-e13cbb326bcd01a4.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libobject-13ac6af5403a52c8.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libmemchr-a7b8febdd2acb289.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libaddr2line-343513f0726f71ed.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgimli-8f833d900bfb98aa.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_demangle-e97a7960ca6216c8.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd_detect-683fb35093a61fcc.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libhashbrown-f943c2d34bd4b56d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libminiz_oxide-72ce2aaa649404e0.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libadler-ac5d08ad5339e92e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_alloc-dacfda262d5656fb.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-e2056a834ba0712c.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcfg_if-a60649c148c6e2db.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-3e961d059b9bcde7.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-20f26f875d0170e2.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-522518611024dce5.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-05898138a596088a.rlib" "-Wl,--end-group" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-5b83a1df856cf582.rlib" "-Wl,-Bdynamic" "-lgcc_s" "-lutil" "-lrt" "-lpthread" "-lm" "-ldl" "-lc" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/main" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro,-znow" "-nodefaultlibs"
  = note: arm-none-eabi-gcc: error: unrecognized command line option '-m64'

error: aborting due to previous error


/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/llvm-nm: error: /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/main: No such file or directory
make: *** [Makefile:30: all] Error 1



failures:
failures:
    [run-make] src/test/run-make/rlib-format-packed-bundled-libs
    [run-make] src/test/run-make/rlib-format-packed-bundled-libs-2
test result: FAILED. 31 passed; 2 failed; 32 ignored; 0 measured; 0 filtered out; finished in 20.77s

Build completed unsuccessfully in 0:14:32
