
[01:04:25] ---- [run-make] run-make/sanitizer-memory stdout ----
[01:04:25] 	
[01:04:25] error: make failed
[01:04:25] status: exit code: 2
[01:04:25] command: "make"
[01:04:25] stdout:
[01:04:25] ------------------------------------------
[01:04:25] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/sanitizer-memory.stage2-x86_64-unknown-linux-gnu:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib:" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/sanitizer-memory.stage2-x86_64-unknown-linux-gnu -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/sanitizer-memory.stage2-x86_64-unknown-linux-gnu  -g -Z sanitizer=memory -Z print-link-args uninit.rs | "/checkout/src/etc/cat-and-grep.sh" librustc_msan
[01:04:25] [[[ begin stdout ]]]
[01:04:25] [90m"cc" "-Wl,--as-needed" "-Wl,-z,noexecstack" "-m64" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/sanitizer-memory.stage2-x86_64-unknown-linux-gnu/uninit.uninit0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/sanitizer-memory.stage2-x86_64-unknown-linux-gnu/uninit.uninit1.rcgu.o" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/sanitizer-memory.stage2-x86_64-unknown-linux-gnu/uninit" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/sanitizer-memory.stage2-x86_64-unknown-linux-gnu/uninit.crate.allocator.rcgu.o" "-Wl,--gc-sections" "-pie" "-Wl,-z,relro,-z,now" "-nodefaultlibs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/sanitizer-memory.stage2-x86_64-unknown-linux-gnu" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-8a57f85a0ae8b781.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libpanic_unwind-d795c34fd7eb72c9.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libunwind-0f4b95d88e53e455.rlib" "-Wl,--whole-archive" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/sanitizer-memory.stage2-x86_64-unknown-linux-gnu/rustc.CIJw1hdYBiRz/librustc_msan-fbd8d4c3523df812.rlib" "-Wl,--no-whole-archive" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc_system-6d9370e956155877.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-f874e85b0fc1f5b8.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liballoc-c7758873e5bedec1.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd_unicode-c9b8ae6d904bd621.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-c80c8268f013635d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-6389cf691b0a8db6.rlib" "-Wl,-Bdynamic" "-l" "dl" "-l" "rt" "-l" "pthread" "-l" "pthread" "-l" "gcc_s" "-l" "c" "-l" "m" "-l" "rt" "-l" "pthread" "-l" "util" "-l" "util"
[01:04:25] [0m
[01:04:25] [[[ end stdout ]]]
[01:04:25] /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/sanitizer-memory.stage2-x86_64-unknown-linux-gnu/uninit 2>&1 | "/checkout/src/etc/cat-and-grep.sh" use-of-uninitialized-value
[01:04:25] [[[ begin stdout ]]]
[01:04:25] [90mFATAL: Code 0x0100da5aad10 is out of application range. Non-PIE build?
[01:04:25] FATAL: MemorySanitizer can not mmap the shadow memory.
[01:04:25] FATAL: Make sure to compile with -fPIE and to link with -pie.
[01:04:25] FATAL: Disabling ASLR is known to cause this error.
[01:04:25] FATAL: If running under GDB, try 'set disable-randomization off'.
[01:04:25] ==29253==Process memory map follows:
[01:04:25] 	0x0100da584000-0x0100da655000	/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/sanitizer-memory.stage2-x86_64-unknown-linux-gnu/uninit
[01:04:25] 	0x0100da855000-0x0100da85a000	/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/sanitizer-memory.stage2-x86_64-unknown-linux-gnu/uninit
[01:04:25] 	0x0100da85a000-0x0100da85d000	/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/sanitizer-memory.stage2-x86_64-unknown-linux-gnu/uninit
[01:04:25] 	0x0100da85d000-0x0100dccc3000	
[01:04:25] 	0x7ffabc640000-0x7ffabc992000	
[01:04:25] 	0x7ffabc992000-0x7ffabca9a000	/lib/x86_64-linux-gnu/libm-2.23.so
[01:04:25] 	0x7ffabca9a000-0x7ffabcc99000	/lib/x86_64-linux-gnu/libm-2.23.so
[01:04:25] 	0x7ffabcc99000-0x7ffabcc9a000	/lib/x86_64-linux-gnu/libm-2.23.so
[01:04:25] 	0x7ffabcc9a000-0x7ffabcc9b000	/lib/x86_64-linux-gnu/libm-2.23.so
[01:04:25] 	0x7ffabcc9b000-0x7ffabce5b000	/lib/x86_64-linux-gnu/libc-2.23.so
[01:04:25] 	0x7ffabce5b000-0x7ffabd05b000	/lib/x86_64-linux-gnu/libc-2.23.so
[01:04:25] 	0x7ffabd05b000-0x7ffabd05f000	/lib/x86_64-linux-gnu/libc-2.23.so
[01:04:25] 	0x7ffabd05f000-0x7ffabd061000	/lib/x86_64-linux-gnu/libc-2.23.so
[01:04:25] 	0x7ffabd061000-0x7ffabd065000	
[01:04:25] 	0x7ffabd065000-0x7ffabd07b000	/lib/x86_64-linux-gnu/libgcc_s.so.1
[01:04:25] 	0x7ffabd07b000-0x7ffabd27a000	/lib/x86_64-linux-gnu/libgcc_s.so.1
[01:04:25] 	0x7ffabd27a000-0x7ffabd27b000	/lib/x86_64-linux-gnu/libgcc_s.so.1
[01:04:25] 	0x7ffabd27b000-0x7ffabd293000	/lib/x86_64-linux-gnu/libpthread-2.23.so
[01:04:25] 	0x7ffabd293000-0x7ffabd492000	/lib/x86_64-linux-gnu/libpthread-2.23.so
[01:04:25] 	0x7ffabd492000-0x7ffabd493000	/lib/x86_64-linux-gnu/libpthread-2.23.so
[01:04:25] 	0x7ffabd493000-0x7ffabd494000	/lib/x86_64-linux-gnu/libpthread-2.23.so
[01:04:25] 	0x7ffabd494000-0x7ffabd498000	
[01:04:25] 	0x7ffabd498000-0x7ffabd49f000	/lib/x86_64-linux-gnu/librt-2.23.so
[01:04:25] 	0x7ffabd49f000-0x7ffabd69e000	/lib/x86_64-linux-gnu/librt-2.23.so
[01:04:25] 	0x7ffabd69e000-0x7ffabd69f000	/lib/x86_64-linux-gnu/librt-2.23.so
[01:04:25] 	0x7ffabd69f000-0x7ffabd6a0000	/lib/x86_64-linux-gnu/librt-2.23.so
[01:04:25] 	0x7ffabd6a0000-0x7ffabd6a3000	/lib/x86_64-linux-gnu/libdl-2.23.so
[01:04:25] 	0x7ffabd6a3000-0x7ffabd8a2000	/lib/x86_64-linux-gnu/libdl-2.23.so
[01:04:25] 	0x7ffabd8a2000-0x7ffabd8a3000	/lib/x86_64-linux-gnu/libdl-2.23.so
[01:04:25] 	0x7ffabd8a3000-0x7ffabd8a4000	/lib/x86_64-linux-gnu/libdl-2.23.so
[01:04:25] 	0x7ffabd8a4000-0x7ffabd8ca000	/lib/x86_64-linux-gnu/ld-2.23.so
[01:04:25] 	0x7ffabdab6000-0x7ffabdac7000	
[01:04:25] 	0x7ffabdac7000-0x7ffabdac9000	
[01:04:25] 	0x7ffabdac9000-0x7ffabdaca000	/lib/x86_64-linux-gnu/ld-2.23.so
[01:04:25] 	0x7ffabdaca000-0x7ffabdacb000	/lib/x86_64-linux-gnu/ld-2.23.so
[01:04:25] 	0x7ffabdacb000-0x7ffabdacc000	
[01:04:25] 	0x7fffbed67000-0x7fffbed89000	[stack]
[01:04:25] 	0x7fffbeda7000-0x7fffbeda9000	[vvar]
[01:04:25] 	0x7fffbeda9000-0x7fffbedab000	[vdso]
[01:04:25] 	0xffffffffff600000-0xffffffffff601000	[vsyscall]
[01:04:25] ==29253==End of process memory map.
[01:04:25] [0m
[01:04:25] [[[ end stdout ]]]
[01:04:25] [1;31mError: cannot match: use-of-uninitialized-value[0m
[01:04:25] Makefile:6: recipe for target 'all' failed
[01:04:25] 
[01:04:25] ------------------------------------------
[01:04:25] stderr:
[01:04:25] ------------------------------------------
[01:04:25] warning: unused variable: `y`
[01:04:25]   --> uninit.rs:15:9
[01:04:25]    |
[01:04:25] 15 |     let y = xs[0] + xs[1];
[01:04:25]    |         ^
[01:04:25]    |
[01:04:25]    = note: #[warn(unused_variables)] on by default
[01:04:25]    = note: to avoid this warning, consider using `_y` instead
[01:04:25] 
[01:04:25] make: *** [all] Error 1
[01:04:25] 
[01:04:25] ------------------------------------------
[01:04:25] 
[01:04:25] thread '[run-make] run-make/sanitizer-memory' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:2776:8
[01:04:25] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:04:25] 
[01:04:25] 
[01:04:25] failures:
[01:04:25]     [run-make] run-make/sanitizer-memory
[01:04:25] 
[01:04:25] test result: [31mFAILED(B[m. 167 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
