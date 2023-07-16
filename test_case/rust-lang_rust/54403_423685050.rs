plain
[00:02:15] extracting /checkout/obj/build/cache/2018-09-11/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:02:15] error: failed to resolve patches for `https://github.com/rust-lang/cargo`
[00:02:15] 
[00:02:15] Caused by:
[00:02:15]   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:02:15] Build completed unsuccessfully in 0:00:30
[00:02:15] Makefile:81: recipe for target 'prepare' failed
[00:02:15] make: *** [prepare] Error 1
[00:02:16] Command failed. Attempt 2/5:
[00:02:16] Command failed. Attempt 2/5:
[00:02:17] error: failed to resolve patches for `https://github.com/rust-lang/cargo`
[00:02:17] 
[00:02:17] Caused by:
[00:02:17]   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:02:17] Build completed unsuccessfully in 0:00:00
[00:02:17] Makefile:81: recipe for target 'prepare' failed
[00:02:17] make: *** [prepare] Error 1
[00:02:19] Command failed. Attempt 3/5:
[00:02:19] Command failed. Attempt 3/5:
[00:02:19] error: failed to resolve patches for `https://github.com/rust-lang/cargo`
[00:02:19] 
[00:02:19] Caused by:
[00:02:19]   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:02:19] Build completed unsuccessfully in 0:00:00
[00:02:19] Makefile:81: recipe for target 'prepare' failed
[00:02:19] make: *** [prepare] Error 1
[00:02:22] Command failed. Attempt 4/5:
[00:02:22] Command failed. Attempt 4/5:
[00:02:22] error: failed to resolve patches for `https://github.com/rust-lang/cargo`
[00:02:22] 
[00:02:22] Caused by:
[00:02:22]   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:02:22] Build completed unsuccessfully in 0:00:00
[00:02:22] make: *** [prepare] Error 1
[00:02:22] Makefile:81: recipe for target 'prepare' failed
[00:02:26] Command failed. Attempt 5/5:
[00:02:26] Command failed. Attempt 5/5:
[00:02:26] error: failed to resolve patches for `https://github.com/rust-lang/cargo`
[00:02:26] 
[00:02:26] Caused by:
[00:02:26]   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:02:26] Build completed unsuccessfully in 0:00:00
[00:02:26] Makefile:81: recipe for target 'prepare' failed
[00:02:26] make: *** [prepare] Error 1
[00:02:26] The command has failed after 5 attempts.
---
travis_time:end:08c17716:start=1537568032744130537,finish=1537568032750181895,duration=6051358
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:162064c3
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:20da0f62
travis_time:start:20da0f62
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0fb04600
$ dmesg | grep -i kill
