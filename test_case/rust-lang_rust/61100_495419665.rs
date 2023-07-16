plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:075fb8c9
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[01:03:05]    --> src/librustc_lint/lib.rs:23:9
[01:03:05]     |
[01:03:05] 23  | #![deny(internal)]
[01:03:05]     |         ^^^^^^^^
[01:03:05]     = note: #[deny(usage_of_qualified_ty)] implied by #[deny(internal)]
[01:03:05] error: aborting due to previous error
[01:03:05] 
[01:03:05] error: Could not compile `rustc_lint`.
[01:03:05] warning: build failed, waiting for other jobs to finish...
---
travis_time:end:0472b7d0:start=1558653787403704513,finish=1558653787422000560,duration=18296047
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0eea0c46
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:055e3e1c
travis_time:start:055e3e1c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0438be74
$ dmesg | grep -i kill
