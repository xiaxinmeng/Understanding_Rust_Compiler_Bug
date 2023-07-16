plain
Building stage2 tool cargo (i686-unknown-freebsd)
[01:15:00]  Downloading crates ...
[01:15:20] warning: spurious network error (2 tries remaining): [6] Couldn't resolve host name (Could not resolve host: crates.io)
[01:15:40] warning: spurious network error (1 tries remaining): [6] Couldn't resolve host name (Could not resolve host: crates.io)
[01:16:00] error: failed to download from `https://crates.io/api/v1/crates/openssl-src/111.1.0+1.1.1a/download`
[01:16:00] Caused by:
[01:16:00]   [6] Couldn't resolve host name (Could not resolve host: crates.io)
[01:16:00] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "i686-unknown-freebsd" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/cargo/Cargo.toml" "--features" "rustc-workspace-hack/all-static" "--message-format" "json"
[01:16:00] expected success, got: exit code: 101
---
travis_time:end:0d066e10:start=1546153859804048951,finish=1546153859812849868,duration=8800917
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1484957d
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:094f3490
travis_time:start:094f3490
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:07407219
$ dmesg | grep -i kill
