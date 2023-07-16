plain
[02:03:43] travis_fold:start:stage2-cargo
travis_time:start:stage2-cargo
Building stage2 tool cargo (i686-unknown-linux-gnu)
[02:03:44]  Downloading openssl-src v111.0.1+1.1.1
[02:04:24] warning: spurious network error (2 tries remaining): failed to download from `https://crates.io/api/v1/crates/openssl-src/111.0.1+1.1.1/download`
[02:05:04] warning: spurious network error (1 tries remaining): failed to download from `https://crates.io/api/v1/crates/openssl-src/111.0.1+1.1.1/download`
[02:05:44] 
[02:05:44] Caused by:
[02:05:44] Caused by:
[02:05:44]   failed to download from `https://crates.io/api/v1/crates/openssl-src/111.0.1+1.1.1/download`
[02:05:44] Caused by:
[02:05:44]   [28] Timeout was reached (Resolving timed out after 30000 milliseconds)
[02:05:44] command did not execute successfully: "/checkout/obj/build/i686-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "i686-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/cargo/Cargo.toml" "--features" "rustc-workspace-hack/all-static" "--message-format" "json"
[02:05:44] expected success, got: exit code: 101
---
travis_time:end:09103844:start=1540369312181158612,finish=1540369312189016063,duration=7857451
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2030f3d3
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:025eb855
travis_time:start:025eb855
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:076d7bd0
$ dmesg | grep -i kill
