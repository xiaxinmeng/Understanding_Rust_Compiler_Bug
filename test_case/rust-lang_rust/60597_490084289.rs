plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:11528a76
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[00:59:30]    --> src/librustc_mir/lib.rs:31:9
[00:59:30]     |
[00:59:30] 31  | #![deny(internal)]
[00:59:30]     |         ^^^^^^^^
[00:59:30]     = note: #[deny(usage_of_qualified_ty)] implied by #[deny(internal)]
[00:59:30] error: aborting due to previous error
[00:59:30] 
[00:59:31] error: Could not compile `rustc_mir`.
[00:59:31] warning: build failed, waiting for other jobs to finish...
---
travis_time:end:15336b20:start=1557236433577772568,finish=1557236433605290258,duration=27517690
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:04401b3c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:065e1e56
travis_time:start:065e1e56
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:31277e29
$ dmesg | grep -i kill
