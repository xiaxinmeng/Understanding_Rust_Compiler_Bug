plain
Building stage2 tool cargo (i686-unknown-freebsd)
[01:07:57]  Downloading crates ...
[01:08:17] warning: spurious network error (2 tries remaining): [6] Couldn't resolve host name (Could not resolve host: crates.io)
[01:08:37] warning: spurious network error (1 tries remaining): [6] Couldn't resolve host name (Could not resolve host: crates.io)
[01:08:57] error: failed to download from `https://crates.io/api/v1/crates/openssl-src/111.0.1+1.1.1/download`
[01:08:57] Caused by:
[01:08:57]   [6] Couldn't resolve host name (Could not resolve host: crates.io)
[01:08:57] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "i686-unknown-freebsd" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/cargo/Cargo.toml" "--features" "rustc-workspace-hack/all-static" "--message-format" "json"
[01:08:57] expected success, got: exit code: 101
---
travis_time:end:066a0967:start=1542610726837447021,finish=1542610726846315660,duration=8868639
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1ba59dce
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:05278b39
travis_time:start:05278b39
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:3065098e
$ dmesg | grep -i kill
