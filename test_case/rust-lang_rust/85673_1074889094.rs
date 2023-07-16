plain
test [run-make] run-make/rustdoc-scrape-examples-ordering ... ok
test [run-make] run-make/rustdoc-scrape-examples-multiple ... ok
test [run-make] run-make/emit-shared-files ... ok
test [run-make] run-make/thumb-none-cortex-m ... ok
Some tests failed in compiletest suite=run-make mode=run-make host=x86_64-unknown-linux-gnu target=thumbv6m-none-eabi

failures:


---- [run-make] run-make/export-executable-symbols stdout ----
error: make failed
status: exit status: 2
command: "make"
--- stdout -------------------------------
--- stdout -------------------------------
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/export-executable-symbols/export-executable-symbols:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/export-executable-symbols/export-executable-symbols -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/export-executable-symbols/export-executable-symbols  -Clinker='arm-none-eabi-gcc' --edition=2018 --crate-type=cdylib foo.rs -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/export-executable-symbols/export-executable-symbols/libfoo.so
--- stderr -------------------------------
warning: ignoring --out-dir flag due to -o flag


error: linking with `arm-none-eabi-gcc` failed: exit status: 1
  |
  = note: "arm-none-eabi-gcc" "-Wl,--version-script=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/export-executable-symbols/export-executable-symbols/rustcDcYGyu/list" "-m64" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/export-executable-symbols/export-executable-symbols/libfoo.foo.91a024fd-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/export-executable-symbols/export-executable-symbols/libfoo.4ya8s8n1o2es9g07.rcgu.o" "-Wl,--as-needed" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/export-executable-symbols/export-executable-symbols" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,--start-group" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-fcd8278774b9c6fc.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-e7f86684b9679284.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libobject-10a87791239bd676.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libmemchr-0e6cad6dd623d38e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libaddr2line-9d05dd05a4d51b33.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libgimli-a60ff138fe1adfad.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_demangle-0e9344458c09713e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd_detect-ef3b808d96f639bf.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libhashbrown-e3ce420901586b53.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libminiz_oxide-546bece8dd1bea42.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libadler-9b358af16d7d7aa1.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_alloc-554cd499fe1a3b52.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-d105877dbe329cd1.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcfg_if-8fec8a74c706e4b4.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-e869a247063ed69a.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-4896c4057dc9553a.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-2a6a2797f7a73818.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-0e3656b1fda5fd7b.rlib" "-Wl,--end-group" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-1f103368fa522bc0.rlib" "-Wl,-Bdynamic" "-lgcc_s" "-lutil" "-lrt" "-lpthread" "-lm" "-ldl" "-lc" "-Wl,--eh-frame-hdr" "-Wl,-znoexecstack" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/export-executable-symbols/export-executable-symbols/libfoo.so" "-Wl,--gc-sections" "-shared" "-Wl,-zrelro,-znow" "-nodefaultlibs"
  = note: arm-none-eabi-gcc: error: unrecognized command line option '-m64'

error: aborting due to previous error; 1 warning emitted


make: *** [Makefile:4: all] Error 1



failures:
