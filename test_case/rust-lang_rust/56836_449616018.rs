plain
[00:49:14] [RUSTC-TIMING] libc test:false 0.104
[00:49:18] [RUSTC-TIMING] compiler_builtins test:false 3.385
[00:49:18]    Compiling alloc v0.0.0 (/checkout/src/liballoc)
[00:49:18]    Compiling rustc-demangle v0.1.10
[00:49:18] error[E0432]: unresolved imports `libc::c_void`, `libc::c_char`, `libc::c_int`, `libc::uintptr_t`
[00:49:18]   |
[00:49:18]   |
[00:49:18] 6 | use libc::{c_void, c_char, c_int, uintptr_t};
[00:49:18]   |            ^^^^^^  ^^^^^^  ^^^^^  ^^^^^^^^^ no `uintptr_t` in the root
[00:49:18]   |            |       |       no `c_int` in the root
[00:49:18]   |            |       no `c_char` in the root
[00:49:18]   |            no `c_void` in the root
[00:49:18] 
---
travis_time:end:1729dfa3:start=1545543838428127534,finish=1545543838436516227,duration=8388693
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:10a1fc24
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:01deeb10
travis_time:start:01deeb10
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1ed2b2a1
$ dmesg | grep -i kill
