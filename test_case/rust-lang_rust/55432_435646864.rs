plain
[02:29:01] warning: spurious network error (2 tries remaining): curl error: Could not resolve host: github.com
[02:29:01] ; class=Net (12)
[02:29:21] warning: spurious network error (1 tries remaining): curl error: Could not resolve host: github.com
[02:29:21] ; class=Net (12)
[02:29:41] error: failed to load source for a dependency on `rand`
[02:29:41] Caused by:
[02:29:41]   Unable to update registry `https://github.com/rust-lang/crates.io-index`
[02:29:41] 
[02:29:41] Caused by:
[02:29:41] Caused by:
[02:29:41]   failed to fetch `https://github.com/rust-lang/crates.io-index`
[02:29:41] 
[02:29:41] Caused by:
[02:29:41]   curl error: Could not resolve host: github.com
[02:29:41] ; class=Net (12)
[02:29:41] 
[02:29:41] 
[02:29:41] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "generate-lockfile" "--manifest-path" "/checkout/obj/build/tmp/distcheck-src/rust-src/lib/rustlib/src/rust/src/libstd/Cargo.toml"
[02:29:41] 
[02:29:41] 
[02:29:41] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test distcheck
[02:29:41] Build completed unsuccessfully in 2:26:38
---
travis_time:end:05e1b992:start=1541314526985736513,finish=1541314526993865921,duration=8129408
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:26c9542a
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:00d50ecb
travis_time:start:00d50ecb
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0ccc1db6
$ dmesg | grep -i kill
