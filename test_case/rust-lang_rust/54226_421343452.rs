plain
[00:02:10] 
[00:02:10] Caused by:
[00:02:10]   feature `edition` is required
[00:02:10] 
[00:02:10] consider adding `cargo-features = ["edition"]` to the manifest
[00:02:10] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:10] make: *** [prepare] Error 1
[00:02:10] Makefile:81: recipe for target 'prepare' failed
[00:02:11] Command failed. Attempt 2/5:
[00:02:11] error: failed to parse manifest at `/checkout/src/tools/clippy/Cargo.toml`
[00:02:11] error: failed to parse manifest at `/checkout/src/tools/clippy/Cargo.toml`
[00:02:11] 
[00:02:11] Caused by:
[00:02:11]   editions are unstable
[00:02:11] 
[00:02:11] Caused by:
[00:02:11]   feature `edition` is required
[00:02:11] 
[00:02:11] consider adding `cargo-features = ["edition"]` to the manifest
[00:02:11] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:11] make: *** [prepare] Error 1
[00:02:11] Makefile:81: recipe for target 'prepare' failed
[00:02:13] Command failed. Attempt 3/5:
[00:02:14] error: failed to parse manifest at `/checkout/src/tools/clippy/Cargo.toml`
[00:02:14] error: failed to parse manifest at `/checkout/src/tools/clippy/Cargo.toml`
[00:02:14] 
[00:02:14] Caused by:
[00:02:14]   editions are unstable
[00:02:14] 
[00:02:14] Caused by:
[00:02:14]   feature `edition` is required
[00:02:14] 
[00:02:14] consider adding `cargo-features = ["edition"]` to the manifest
[00:02:14] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:14] Makefile:81: recipe for target 'prepare' failed
[00:02:14] make: *** [prepare] Error 1
[00:02:17] Command failed. Attempt 4/5:
[00:02:17] error: failed to parse manifest at `/checkout/src/tools/clippy/Cargo.toml`
[00:02:17] error: failed to parse manifest at `/checkout/src/tools/clippy/Cargo.toml`
[00:02:17] 
[00:02:17] Caused by:
[00:02:17]   editions are unstable
[00:02:17] 
[00:02:17] Caused by:
[00:02:17]   feature `edition` is required
[00:02:17] 
[00:02:17] consider adding `cargo-features = ["edition"]` to the manifest
[00:02:17] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:17] Makefile:81: recipe for target 'prepare' failed
[00:02:17] make: *** [prepare] Error 1
[00:02:21] Command failed. Attempt 5/5:
[00:02:21] error: failed to parse manifest at `/checkout/src/tools/clippy/Cargo.toml`
[00:02:21] error: failed to parse manifest at `/checkout/src/tools/clippy/Cargo.toml`
[00:02:21] 
[00:02:21] Caused by:
[00:02:21]   editions are unstable
[00:02:21] 
[00:02:21] Caused by:
[00:02:21]   feature `edition` is required
[00:02:21] 
[00:02:21] consider adding `cargo-features = ["edition"]` to the manifest
[00:02:21] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:21] make: *** [prepare] Error 1
[00:02:21] Makefile:81: recipe for target 'prepare' failed
[00:02:21] The command has failed after 5 attempts.
travis_time:end:1a2b4512:start=1536927974324039910,finish=1536928125242490879,duration=150918450969
---
travis_time:end:28038e00:start=1536928125682894937,finish=1536928125690402659,duration=7507722
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0968f040
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:3a0f09c0
travis_time:start:3a0f09c0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0496cec9
$ dmesg | grep -i kill
