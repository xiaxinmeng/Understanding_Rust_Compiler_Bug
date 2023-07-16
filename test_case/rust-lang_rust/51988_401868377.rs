plain
############################################                              62.1%
###############################################################           88.3%
######################################################################## 100.0%
[00:01:09] extracting /checkout/obj/build/cache/2018-06-30/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:10] error: failed to resolve patches for `https://github.com/rust-lang/cargo`
[00:01:10] Caused by:
[00:01:10] Caused by:
[00:01:10]   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:01:10] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:10] make: *** [prepare] Error 1
[00:01:10] make: *** [prepare] Error 1
[00:01:10] Makefile:81: recipe for target 'prepare' failed
[00:01:11] Command failed. Attempt 2/5:
[00:01:11] error: failed to resolve patches for `https://github.com/rust-lang/cargo`
[00:01:11] Caused by:
[00:01:11] Caused by:
[00:01:11]   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:01:11] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:11] make: *** [prepare] Error 1
[00:01:11] make: *** [prepare] Error 1
[00:01:11] Makefile:81: recipe for target 'prepare' failed
[00:01:13] Command failed. Attempt 3/5:
[00:01:13] error: failed to resolve patches for `https://github.com/rust-lang/cargo`
[00:01:13] Caused by:
[00:01:13] Caused by:
[00:01:13]   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:01:13] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:13] make: *** [prepare] Error 1
[00:01:13] make: *** [prepare] Error 1
[00:01:13] Makefile:81: recipe for target 'prepare' failed
[00:01:16] Command failed. Attempt 4/5:
[00:01:16] error: failed to resolve patches for `https://github.com/rust-lang/cargo`
[00:01:16] Caused by:
[00:01:16] Caused by:
[00:01:16]   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:01:16] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:16] Build completed unsuccessfully in 0:00:00
[00:01:16] Makefile:81: recipe for target 'prepare' failed
[00:01:16] make: *** [prepare] Error 1
[00:01:20] Command failed. Attempt 5/5:
[00:01:20] error: failed to resolve patches for `https://github.com/rust-lang/cargo`
[00:01:20] Caused by:
[00:01:20] Caused by:
[00:01:20]   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:01:20] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:20] Build completed unsuccessfully in 0:00:00
[00:01:20] Makefile:81: recipe for target 'prepare' failed
[00:01:20] make: *** [prepare] Error 1
[00:01:20] The command has failed after 5 attempts.

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
travis_time:start:04be8300
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:2c503daf:start=1530550592067600760,finish=1530550592075907592,duration=8306832
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0c7272ac
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0af89990
$ dmesg | grep -i kill
