plain
[00:59:22]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[00:59:23] [RUSTC-TIMING] panic_unwind test:false 0.240
[00:59:42] error: linking with `sparcv9-sun-solaris2.10-gcc` failed: exit code: 1
[00:59:42]   |
[00:59:42]   = note: "sparcv9-sun-solaris2.10-gcc" "-m64" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/sparcv9-sun-solaris/lib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/sparcv9-sun-solaris/release/deps/std-d78d1728fe345ee5.std.ajv6mi5u-cgu.0.rcgu.o" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/sparcv9-sun-solaris/release/deps/libstd-d78d1728fe345ee5.so" "-Wl,-M,/tmp/rustcyyN6QC/list" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/sparcv9-sun-solaris/release/deps/std-d78d1728fe345ee5.1idtlln1s2yand06.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/sparcv9-sun-solaris/release/deps/std-d78d1728fe345ee5.4z2gy4dz46fr542x.rcgu.o" "-Wl,-zignore" "-nodefaultlibs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/sparcv9-sun-solaris/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/sparcv9-sun-solaris/release/build/compiler_builtins-f1b47d285171b807/out" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/sparcv9-sun-solaris/release/build/backtrace-sys-437db0d3ccefcba2/out" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/sparcv9-sun-solaris/lib" "-lsocket" "-lposix4" "-lpthread" "-lresolv" "-Wl,-Bstatic" "-Wl,--whole-archive" "/tmp/rustcyyN6QC/libpanic_unwind-5fcf5f96e35ebf7b.rlib" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "/tmp/rustcyyN6QC/libbacktrace_sys-c2dccf1657ebd17f.rlib" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "/tmp/rustcyyN6QC/librustc_demangle-441c25bcbcf7d921.rlib" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "/tmp/rustcyyN6QC/libunwind-4dd905694bb0512c.rlib" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "/tmp/rustcyyN6QC/liblibc-c52303f164d8be4a.rlib" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "/tmp/rustcyyN6QC/liballoc-ac0aed9b01c11413.rlib" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "/tmp/rustcyyN6QC/librustc_std_workspace_core-20f0b28d2561573f.rlib" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "/tmp/rustcyyN6QC/libcore-dcc0e87728e57bb3.rlib" "-Wl,--no-whole-archive" "/tmp/rustcyyN6QC/libcompiler_builtins-c5375e1750e708b8.rlib" "-Wl,-Bdynamic" "-lgcc_s" "-lc" "-lm" "-lrt" "-lpthread" "-shared" "-Wl,-rpath,$ORIGIN/../lib"
[00:59:42]   = note: /usr/local/lib/gcc/sparcv9-sun-solaris2.10/6.4.0/../../../../sparcv9-sun-solaris2.10/bin/ld: warning: -z ignore ignored.
[00:59:42]           /usr/local/lib/gcc/sparcv9-sun-solaris2.10/6.4.0/../../../../sparcv9-sun-solaris2.10/bin/ld:/tmp/rustcyyN6QC/list: file format not recognized; treating as linker script
[00:59:42]           /usr/local/lib/gcc/sparcv9-sun-solaris2.10/6.4.0/../../../../sparcv9-sun-solaris2.10/bin/ld:/tmp/rustcyyN6QC/list:1: syntax error
[00:59:42]           
[00:59:42] 
[00:59:42] error: aborting due to previous error
[00:59:42] 
---
travis_time:end:2c2c7384:start=1555103261739527076,finish=1555103261744905868,duration=5378792
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:034e2d66
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0740dea0
travis_time:start:0740dea0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:01d29bf0
$ dmesg | grep -i kill
