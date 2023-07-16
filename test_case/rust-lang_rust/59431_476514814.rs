plain
[01:05:31]    Compiling compiler_builtins v0.1.5
[01:05:31]    Compiling unwind v0.0.0 (/checkout/src/libunwind)
[01:05:31]    Compiling backtrace-sys v0.1.27
[01:05:31]    Compiling std v0.0.0 (/checkout/src/libstd)
[01:05:32] error[E0472]: asm! is unsupported on this target
[01:05:32]    --> src/libcore/hint.rs:105:18
[01:05:32]     |
[01:05:32] 105 |         unsafe { asm!("" : : "r"(&dummy)) }
[01:05:32] 
[01:05:42] error: aborting due to previous error
[01:05:42] 
[01:05:42] For more information about this error, try `rustc --explain E0472`.
---
travis_time:end:11afb722:start=1553587279331196703,finish=1553587279338421821,duration=7225118
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0b6466af
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0a21fcd8
travis_time:start:0a21fcd8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0828239c
$ dmesg | grep -i kill
