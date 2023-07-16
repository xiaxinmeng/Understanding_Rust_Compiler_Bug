plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:071909ec
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
travis_time:end:17a4b9f6:start=1561404357760883048,finish=1561404357778672921,duration=17789873
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:05197400
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0e2fef50
travis_time:start:0e2fef50
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2e6474f8
$ dmesg | grep -i kill
