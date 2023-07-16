plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:27b478ea
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[00:09:01]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:09:05] error[E0425]: cannot find value `values` in this scope
[00:09:05]   --> src/librustc/ty/erase_regions.rs:25:13
[00:09:05]    |
[00:09:05] 25 |         if !values.has_erasable_regions() {
[00:09:05] 
[00:09:33] error: aborting due to previous error
[00:09:33] 
[00:09:33] For more information about this error, try `rustc --explain E0425`.
---
travis_time:end:02b8a31e:start=1547665189670058274,finish=1547665189694319926,duration=24261652
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:15407ff7
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:02425368
travis_time:start:02425368
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0f007cf6
$ dmesg | grep -i kill
