plain
[02:29:34] warning: spurious network error (2 tries remaining): curl error: Could not resolve host: github.com
[02:29:34] ; class=Net (12)
[02:29:54] warning: spurious network error (1 tries remaining): curl error: Could not resolve host: github.com
[02:29:54] ; class=Net (12)
[02:30:15] error: failed to load source for a dependency on `rand`
[02:30:15] Caused by:
[02:30:15]   Unable to update registry `https://github.com/rust-lang/crates.io-index`
[02:30:15] 
[02:30:15] Caused by:
[02:30:15] Caused by:
[02:30:15]   failed to fetch `https://github.com/rust-lang/crates.io-index`
[02:30:15] 
[02:30:15] Caused by:
[02:30:15]   curl error: Could not resolve host: github.com
[02:30:15] ; class=Net (12)
[02:30:15] 
[02:30:15] 
[02:30:15] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "generate-lockfile" "--manifest-path" "/checkout/obj/build/tmp/distcheck-src/rust-src/lib/rustlib/src/rust/src/libstd/Cargo.toml"
[02:30:15] 
[02:30:15] 
[02:30:15] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test distcheck
[02:30:15] Build completed unsuccessfully in 2:27:11
---
travis_time:end:19180a7c:start=1540624280304249995,finish=1540624280314062198,duration=9812203
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:04ad5080
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:031b17fa
travis_time:start:031b17fa
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:202dada1
$ dmesg | grep -i kill
