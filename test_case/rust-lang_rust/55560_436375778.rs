plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:00ddc430
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[01:47:52]    Compiling quote v0.6.8
[01:47:53]    Compiling idna v0.1.5
[01:48:11]    Compiling thread_local v0.3.6
[01:48:16]    Compiling syn v0.14.9
[01:48:16] error: couldn't read /cargo/registry/src/github.com-1ecc6299db9ec823/syn-0.14.9/src/../gen_helper.rs: No such file or directory (os error 2)
[01:48:16]     |
[01:48:16] 566 |     mod helper;
[01:48:16]     |         ^^^^^^
[01:48:16] 
---
travis_time:end:37532890:start=1541531926250872066,finish=1541531926268339518,duration=17467452
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:02c05ff4
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0299e6a1
travis_time:start:0299e6a1
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0864ae90
$ dmesg | grep -i kill
