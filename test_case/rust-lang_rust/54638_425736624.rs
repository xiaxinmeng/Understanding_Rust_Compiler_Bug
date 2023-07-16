plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:1af47c41
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
travis_time:end:0f079fe6:start=1538328210530022753,finish=1538328210536593551,duration=6570798
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0ed74682
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:17d2739c
travis_time:start:17d2739c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:02f4be3b
$ dmesg | grep -i kill
