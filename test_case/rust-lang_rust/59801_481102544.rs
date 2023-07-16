plain
[00:57:33]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[00:57:33] [RUSTC-TIMING] panic_unwind test:false 0.244
[00:57:52] error: linking with `sparcv9-sun-solaris2.10-gcc` failed: exit code: 1
[00:57:52]   |
[00:57:52]   = note: "sparcv9-sun-solaris2.10-gcc" "-m64" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/sparcv9-sun-solaris/lib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/sparcv9-sun-solaris/release/deps/std-a13bd72181596505.std.dn7eah83-cgu.0.rcgu.o" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/sparcv9-sun-solaris/release/deps/libstd-a13bd72181596505.so" "-Wl,-M,/tmp/rustc6CHczt/list" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/sparcv9-sun-solaris/release/deps/std-a13bd72181596505.4s1vtwpn7z85dmqf.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/sparcv9-sun-solaris/release/deps/std-a13bd72181596505.1mglhhcuhvh8teko.rcgu.o" "-Wl,-zignore" "-nodefaultlibs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/sparcv9-sun-solaris/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/sparcv9-sun-solaris/release/build/compiler_builtins-8378b22d6a548e04/out" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/sparcv9-sun-solaris/release/build/backtrace-sys-cbe5a45b39d44720/out" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/sparcv9-sun-solaris/lib" "-lsocket" "-lposix4" "-lpthread" "-lresolv" "-Wl,-Bstatic" "-Wl,--whole-archive" "/tmp/rustc6CHczt/libpanic_unwind-c2a16410f228c714.rlib" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "/tmp/rustc6CHczt/libbacktrace_sys-5ba735a64c8951c3.rlib" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "/tmp/rustc6CHczt/librustc_demangle-8fd284197e46ab70.rlib" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "/tmp/rustc6CHczt/libunwind-eebc1ec39336b17a.rlib" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "/tmp/rustc6CHczt/liblibc-233cfda2f4c7d8e1.rlib" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "/tmp/rustc6CHczt/liballoc-e75db5a2a9e140cf.rlib" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "/tmp/rustc6CHczt/librustc_std_workspace_core-f74bc27f7dfbf352.rlib" "-Wl,--no-whole-archive" "-Wl,--whole-archive" "/tmp/rustc6CHczt/libcore-e2f4bb5fa52d60ee.rlib" "-Wl,--no-whole-archive" "/tmp/rustc6CHczt/libcompiler_builtins-4ab94bdb9690f25a.rlib" "-Wl,-Bdynamic" "-lgcc_s" "-lc" "-lm" "-lrt" "-lpthread" "-shared" "-Wl,-rpath,$ORIGIN/../lib"
[00:57:52]   = note: /usr/local/lib/gcc/sparcv9-sun-solaris2.10/6.4.0/../../../../sparcv9-sun-solaris2.10/bin/ld: warning: -z ignore ignored.
[00:57:52]           /usr/local/lib/gcc/sparcv9-sun-solaris2.10/6.4.0/../../../../sparcv9-sun-solaris2.10/bin/ld:/tmp/rustc6CHczt/list: file format not recognized; treating as linker script
[00:57:52]           /usr/local/lib/gcc/sparcv9-sun-solaris2.10/6.4.0/../../../../sparcv9-sun-solaris2.10/bin/ld:/tmp/rustc6CHczt/list:1: syntax error
[00:57:52]           
[00:57:52] 
[00:57:52] error: aborting due to previous error
[00:57:52] 
---
travis_time:end:36261d68:start=1554785991343313982,finish=1554785991351070568,duration=7756586
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0dd804f8
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:3371e2de
travis_time:start:3371e2de
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:03e74954
$ dmesg | grep -i kill
