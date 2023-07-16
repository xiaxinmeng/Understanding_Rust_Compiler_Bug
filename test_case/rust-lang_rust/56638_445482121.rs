plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:148cd42c
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[00:15:21]    Compiling rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
[00:15:23] error[E0424]: expected value, found module `self`
[00:15:23]    --> src/librustc_mir/borrow_check/nll/mod.rs:249:36
[00:15:23]     |
[00:15:23] 249 |                     for borrow in &self.borrows {
[00:15:23]     |                                    ^^^^ `self` value is a keyword only available in methods with `self` parameter
[00:15:38] error: aborting due to previous error
[00:15:38] 
[00:15:38] For more information about this error, try `rustc --explain E0424`.
[00:15:38] error: Could not compile `rustc_mir`.
---
travis_time:end:1d4e5abe:start=1544295813562666230,finish=1544295813572004369,duration=9338139
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:05659930
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:18de4938
travis_time:start:18de4938
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:00c27d3e
$ dmesg | grep -i kill
