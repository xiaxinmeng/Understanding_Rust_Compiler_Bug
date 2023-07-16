plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:06590740
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
travis_time:end:0333937b:start=1537168631867322898,finish=1537168631877421956,duration=10099058
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:02e48c14
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0128a784
travis_time:start:0128a784
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:00440559
$ dmesg | grep -i kill
