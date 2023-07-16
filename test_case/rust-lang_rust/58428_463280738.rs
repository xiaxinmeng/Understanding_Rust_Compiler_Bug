plain
[00:57:35]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[00:57:35] error: unused extern crate
[00:57:35]   --> src/libpanic_unwind/lib.rs:38:1
[00:57:35]    |
[00:57:35] 37 | / #[cfg(not(any(target_env = "msvc", all(windows, target_arch = "x86_64", target_env = "gnu"))))]
[00:57:35] 38 | | extern crate unwind;
[00:57:35]    | | ^^^^^^^^^^^^^^^^^^^-
[00:57:35]    | |____________________|
[00:57:35]    |
[00:57:35] note: lint level defined here
[00:57:35]   --> src/libpanic_unwind/lib.rs:20:9
[00:57:35]    |
[00:57:35]    |
[00:57:35] 20 | #![deny(rust_2018_idioms)]
[00:57:35]    |         ^^^^^^^^^^^^^^^^
[00:57:35]    = note: #[deny(unused_extern_crates)] implied by #[deny(rust_2018_idioms)]
[00:57:35] error: aborting due to previous error
[00:57:35] 
[00:57:35] [RUSTC-TIMING] panic_unwind test:false 0.121
[00:57:35] error: Could not compile `panic_unwind`.
---
travis_time:end:15a17c60:start=1550077320708551824,finish=1550077320721321697,duration=12769873
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0a2caf57
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:14b99a1b
travis_time:start:14b99a1b
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0f33de15
$ dmesg | grep -i kill
