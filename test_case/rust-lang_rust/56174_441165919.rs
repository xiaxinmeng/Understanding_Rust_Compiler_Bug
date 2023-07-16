plain
[01:11:05] travis_fold:start:stage2-cargo
travis_time:start:stage2-cargo
Building stage2 tool cargo (i686-unknown-freebsd)
[01:11:06]  Downloading openssl-src v111.0.1+1.1.1
[01:11:26] warning: spurious network error (2 tries remaining): failed to download from `https://crates.io/api/v1/crates/openssl-src/111.0.1+1.1.1/download`
[01:11:46] warning: spurious network error (1 tries remaining): failed to download from `https://crates.io/api/v1/crates/openssl-src/111.0.1+1.1.1/download`
[01:12:06] 
[01:12:06] Caused by:
[01:12:06] Caused by:
[01:12:06]   failed to download from `https://crates.io/api/v1/crates/openssl-src/111.0.1+1.1.1/download`
[01:12:06] Caused by:
[01:12:06]   [6] Couldn't resolve host name (Could not resolve host: crates.io)
[01:12:06] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "i686-unknown-freebsd" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/cargo/Cargo.toml" "--features" "rustc-workspace-hack/all-static" "--message-format" "json"
[01:12:06] expected success, got: exit code: 101
---
travis_time:end:1c4fb64c:start=1542955786734286083,finish=1542955786742264422,duration=7978339
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0028a39c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1701e8fe
travis_time:start:1701e8fe
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:01f17867
$ dmesg | grep -i kill
