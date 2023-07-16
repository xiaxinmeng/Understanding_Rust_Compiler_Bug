plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:04d6adec
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---

[01:43:18] travis_time:end:stage2-cargo-miri:start=1554942822949774906,finish=1554942823373097308,duration=423322402

[01:43:20] Dist docs (x86_64-unknown-linux-gnu)
[01:43:20] thread 'main' panicked at 'fs::read_dir(src) failed with No such file or directory (os error 2)', src/bootstrap/lib.rs:1228:18
[01:43:20] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
[01:43:20] Build completed unsuccessfully in 1:37:05
travis_time:end:2203964a:start=1554936624654012503,finish=1554942825878909557,duration=6201224897054
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
---
travis_time:end:0b80916a:start=1554942827723779838,finish=1554942827732339278,duration=8559440
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:01eb708a
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:12da5878
travis_time:start:12da5878
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:102e9435
$ dmesg | grep -i kill
