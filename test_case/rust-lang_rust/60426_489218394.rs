
[00:59:29] ---- [run-make] run-make/override-aliased-flags stdout ----
[00:59:29] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[00:59:29] 
[00:59:29] error: make failed
[00:59:29] status: exit code: 2
[00:59:29] command: "make"
[00:59:29] stdout:
[00:59:29] ------------------------------------------
[00:59:29] # Test that `-O` and `-C opt-level` can be specified multiple times.
[00:59:29] # The rightmost flag will be used over any previous flags.
[00:59:29] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/override-aliased-flags/override-aliased-flags:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/override-aliased-flags/override-aliased-flags -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/override-aliased-flags/override-aliased-flags  -Clinker=arm-none-eabi-gcc -O -O main.rs
[00:59:29] Makefile:8: recipe for target 'all' failed
[00:59:29] 
[00:59:29] ------------------------------------------
[00:59:29] stderr:
[00:59:29] ------------------------------------------
[00:59:29] error: linking with `arm-none-eabi-gcc` failed: exit code: 1
[00:59:29]   |
[00:59:29]   = note: "arm-none-eabi-gcc" "-Wl,--as-needed" "-Wl,-z,noexecstack" "-m64" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/override-aliased-flags/override-aliased-flags/main.main.7rcbfp3g-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/override-aliased-flags/override-aliased-flags/main.main.7rcbfp3g-cgu.1.rcgu.o" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/override-aliased-flags/override-aliased-flags/main" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/override-aliased-flags/override-aliased-flags/main.4s37gsrti678ik8u.rcgu.o" "-Wl,--gc-sections" "-pie" "-Wl,-zrelro" "-Wl,-znow" "-Wl,-O1" "-nodefaultlibs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/override-aliased-flags/override-aliased-flags" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,--start-group" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-36963fe6a2961b22.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-bad302bb9c2be352.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libbacktrace_sys-068d4c647d225b00.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_demangle-dd618d3a61d2285d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libhashbrown-08bd494bdfbe6ec5.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_alloc-f434e5c003b087ed.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-4ad3589c043e8a63.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-b93458722dc1ee7d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-cbc983130e35415c.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librustc_std_workspace_core-b986af5e424776e6.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-08dafd64384d2a0a.rlib" "-Wl,--end-group" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-bc40ea2e2336c57a.rlib" "-Wl,-Bdynamic" "-ldl" "-lrt" "-lpthread" "-lgcc_s" "-lc" "-lm" "-lrt" "-lpthread" "-lutil" "-lutil"
[00:59:29]   = note: arm-none-eabi-gcc: error: unrecognized command line option '-m64'
[00:59:29]           
[00:59:29] 
[00:59:29] error: aborting due to previous error
[00:59:29] 
[00:59:29] make: *** [all] Error 1
[00:59:29] 
[00:59:29] ------------------------------------------
