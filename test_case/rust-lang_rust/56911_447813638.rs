plain
######################################################################## 100.0%
[00:02:02] extracting /checkout/obj/build/cache/2018-10-25/cargo-0.31.0-x86_64-unknown-linux-gnu.tar.gz
[00:02:02]     Updating crates.io index
[00:02:12]     Updating git repository `https://github.com/rust-lang-nursery/rust-clippy`
[00:02:15] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:15] Build completed unsuccessfully in 0:00:24
[00:02:15] make: *** [prepare] Error 1
[00:02:15] Makefile:81: recipe for target 'prepare' failed
[00:02:16] Command failed. Attempt 2/5:
[00:02:16] Command failed. Attempt 2/5:
[00:02:16]     Updating crates.io index
[00:02:16]     Updating git repository `https://github.com/rust-lang-nursery/rust-clippy`
[00:02:16] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:16] Build completed unsuccessfully in 0:00:00
[00:02:16] make: *** [prepare] Error 1
[00:02:16] Makefile:81: recipe for target 'prepare' failed
[00:02:18] Command failed. Attempt 3/5:
[00:02:18] Command failed. Attempt 3/5:
[00:02:19]     Updating crates.io index
[00:02:19]     Updating git repository `https://github.com/rust-lang-nursery/rust-clippy`
[00:02:19] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:19] Build completed unsuccessfully in 0:00:00
[00:02:19] make: *** [prepare] Error 1
[00:02:19] Makefile:81: recipe for target 'prepare' failed
[00:02:22] Command failed. Attempt 4/5:
[00:02:22] Command failed. Attempt 4/5:
[00:02:22]     Updating crates.io index
[00:02:23]     Updating git repository `https://github.com/rust-lang-nursery/rust-clippy`
[00:02:23] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:23] Build completed unsuccessfully in 0:00:00
[00:02:23] Makefile:81: recipe for target 'prepare' failed
[00:02:23] make: *** [prepare] Error 1
[00:02:27] Command failed. Attempt 5/5:
[00:02:27] Command failed. Attempt 5/5:
[00:02:27]     Updating crates.io index
[00:02:28]     Updating git repository `https://github.com/rust-lang-nursery/rust-clippy`
[00:02:28] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:28] Build completed unsuccessfully in 0:00:00
[00:02:28] make: *** [prepare] Error 1
[00:02:28] Makefile:81: recipe for target 'prepare' failed
[00:02:28] The command has failed after 5 attempts.
---
travis_time:end:05443d7e:start=1545046135709020191,finish=1545046135717910181,duration=8889990
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1efe6723
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:31a87586
travis_time:start:31a87586
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0b051a9c
$ dmesg | grep -i kill
