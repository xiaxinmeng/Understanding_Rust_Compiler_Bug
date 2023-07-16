plain
[01:03:48] travis_fold:start:stage2-cargo
travis_time:start:stage2-cargo
Building stage2 tool cargo (aarch64-unknown-linux-gnu)
[01:03:48]  Downloading crates ...
[01:03:59] error: failed to download from `https://crates.io/api/v1/crates/openssl-src/111.1.0+1.1.1a/download`
[01:03:59] Caused by:
[01:03:59]   [35] SSL connect error (OpenSSL SSL_connect: SSL_ERROR_SYSCALL in connection to static.crates.io:443 )
[01:03:59] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "aarch64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/cargo/Cargo.toml" "--features" "rustc-workspace-hack/all-static" "--message-format" "json"
[01:03:59] expected success, got: exit code: 101
---
travis_time:end:126e4c88:start=1544847764213454945,finish=1544847764226934085,duration=13479140
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:02eecc97
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:020f1e24
travis_time:start:020f1e24
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:006ec426
$ dmesg | grep -i kill
