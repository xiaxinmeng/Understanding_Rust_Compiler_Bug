plain
[02:20:14] warning: spurious network error (2 tries remaining): curl error: Could not resolve host: github.com
[02:20:14] ; class=Net (12)
[02:20:35] warning: spurious network error (1 tries remaining): curl error: Could not resolve host: github.com
[02:20:35] ; class=Net (12)
[02:20:55] error: failed to load source for a dependency on `rand`
[02:20:55] Caused by:
[02:20:55]   Unable to update registry `https://github.com/rust-lang/crates.io-index`
[02:20:55] 
[02:20:55] Caused by:
[02:20:55] Caused by:
[02:20:55]   failed to fetch `https://github.com/rust-lang/crates.io-index`
[02:20:55] 
[02:20:55] Caused by:
[02:20:55]   curl error: Could not resolve host: github.com
[02:20:55] ; class=Net (12)
[02:20:55] 
[02:20:55] 
[02:20:55] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "generate-lockfile" "--manifest-path" "/checkout/obj/build/tmp/distcheck-src/rust-src/lib/rustlib/src/rust/src/libstd/Cargo.toml"
[02:20:55] 
[02:20:55] 
[02:20:55] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test distcheck
[02:20:55] Build completed unsuccessfully in 2:17:46
---
travis_time:end:06aca06c:start=1539413165398035422,finish=1539413165404781177,duration=6745755
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:003a863f
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:04a7c2d0
travis_time:start:04a7c2d0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:23062fa2
$ dmesg | grep -i kill
