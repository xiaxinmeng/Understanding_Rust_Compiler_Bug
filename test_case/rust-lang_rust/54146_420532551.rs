plain
[02:10:42] warning: spurious network error (2 tries remaining): curl error: Could not resolve host: github.com
[02:10:42] ; class=Net (12)
[02:11:03] warning: spurious network error (1 tries remaining): curl error: Could not resolve host: github.com
[02:11:03] ; class=Net (12)
[02:11:24] error: failed to load source for a dependency on `rand`
[02:11:24] Caused by:
[02:11:24]   Unable to update registry `https://github.com/rust-lang/crates.io-index`
[02:11:24] 
[02:11:24] Caused by:
[02:11:24] Caused by:
[02:11:24]   failed to fetch `https://github.com/rust-lang/crates.io-index`
[02:11:24] 
[02:11:24] Caused by:
[02:11:24]   curl error: Could not resolve host: github.com
[02:11:24] ; class=Net (12)
[02:11:24] 
[02:11:24] 
[02:11:24] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "generate-lockfile" "--manifest-path" "/checkout/obj/build/tmp/distcheck-src/rust-src/lib/rustlib/src/rust/src/libstd/Cargo.toml"
[02:11:24] 
[02:11:24] 
[02:11:24] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test distcheck
[02:11:24] Build completed unsuccessfully in 2:07:52
---
travis_time:end:2bb8d99c:start=1536734979241358481,finish=1536734979246840248,duration=5481767
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0885ccfc
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:391b7e78
travis_time:start:391b7e78
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:02c8dbce
$ dmesg | grep -i kill
