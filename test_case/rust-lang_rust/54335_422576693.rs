plain
[00:03:19] extracting /checkout/obj/build/cache/2018-09-13/cargo-0.30.0-x86_64-unknown-linux-gnu.tar.gz
[00:03:19] error: failed to resolve patches for `https://github.com/rust-lang/cargo`
[00:03:19] 
[00:03:19] Caused by:
[00:03:19]   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:03:19] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:03:19] make: *** [prepare] Error 1
[00:03:19] Makefile:81: recipe for target 'prepare' failed
[00:03:20] Command failed. Attempt 2/5:
[00:03:20] error: failed to resolve patches for `https://github.com/rust-lang/cargo`
[00:03:20] error: failed to resolve patches for `https://github.com/rust-lang/cargo`
[00:03:20] 
[00:03:20] Caused by:
[00:03:20]   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:03:20] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:03:20] make: *** [prepare] Error 1
[00:03:20] Makefile:81: recipe for target 'prepare' failed
[00:03:22] Command failed. Attempt 3/5:
[00:03:22] error: failed to resolve patches for `https://github.com/rust-lang/cargo`
[00:03:22] error: failed to resolve patches for `https://github.com/rust-lang/cargo`
[00:03:22] 
[00:03:22] Caused by:
[00:03:22]   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:03:22] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:03:22] Makefile:81: recipe for target 'prepare' failed
[00:03:22] make: *** [prepare] Error 1
[00:03:25] Command failed. Attempt 4/5:
[00:03:25] error: failed to resolve patches for `https://github.com/rust-lang/cargo`
[00:03:25] error: failed to resolve patches for `https://github.com/rust-lang/cargo`
[00:03:25] 
[00:03:25] Caused by:
[00:03:25]   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:03:25] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:03:25] Makefile:81: recipe for target 'prepare' failed
[00:03:25] make: *** [prepare] Error 1
[00:03:29] Command failed. Attempt 5/5:
[00:03:29] error: failed to resolve patches for `https://github.com/rust-lang/cargo`
[00:03:29] error: failed to resolve patches for `https://github.com/rust-lang/cargo`
[00:03:29] 
[00:03:29] Caused by:
[00:03:29]   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
[00:03:29] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:03:29] Makefile:81: recipe for target 'prepare' failed
[00:03:29] make: *** [prepare] Error 1
[00:03:29] The command has failed after 5 attempts.
travis_time:end:1bef0cdc:start=1537307366723867817,finish=1537307576427682068,duration=209703814251
---
travis_time:end:24cbe56e:start=1537307576827715140,finish=1537307576833393414,duration=5678274
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:342d5ad8
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0437e14c
travis_time:start:0437e14c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:07946230
$ dmesg | grep -i kill
