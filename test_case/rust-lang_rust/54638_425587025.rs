plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:006b0d00
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---

[00:03:42] travis_time:end:246c965c:start=1538166780312412115,finish=1538166939468855200,duration=159156443085
[CI_JOB_NAME=dist-x86_64-linux]
[00:03:42] [CI_JOB_NAME=dist-x86_64-linux]
[00:03:42] /checkout/src/ci/run.sh: line 79: [: too many arguments
[00:03:43] travis_fold:start:configure
travis_time:start:0bc89d20
configure: processing command line
[00:03:43] configure: 
---
[02:03:08] travis_fold:end:stage2-rls

[02:03:08] travis_time:end:stage2-rls:start=1538173907967003722,finish=1538174105289868642,duration=197322864920

[02:03:08] thread 'main' panicked at 'Unable to build RLS', bootstrap/dist.rs:74:9
[02:03:08] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
[02:03:08] Build completed unsuccessfully in 1:57:24
travis_time:end:05bc1bd7:start=1538166716895898519,finish=1538174105623685225,duration=7388727786706

---
travis_time:end:15c5a4b0:start=1538174109734973110,finish=1538174109744409559,duration=9436449
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1a1903d0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:119a803a
travis_time:start:119a803a
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:246ba385
$ dmesg | grep -i kill
