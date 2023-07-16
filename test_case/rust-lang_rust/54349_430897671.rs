plain
[02:20:17] warning: spurious network error (2 tries remaining): curl error: Could not resolve host: github.com
[02:20:17] ; class=Net (12)
[02:20:37] warning: spurious network error (1 tries remaining): curl error: Could not resolve host: github.com
[02:20:37] ; class=Net (12)
[02:20:57] error: failed to load source for a dependency on `rand`
[02:20:57] Caused by:
[02:20:57]   Unable to update registry `https://github.com/rust-lang/crates.io-index`
[02:20:57] 
[02:20:57] Caused by:
[02:20:57] Caused by:
[02:20:57]   failed to fetch `https://github.com/rust-lang/crates.io-index`
[02:20:57] 
[02:20:57] Caused by:
[02:20:57]   curl error: Could not resolve host: github.com
[02:20:57] ; class=Net (12)
[02:20:57] 
[02:20:57] 
[02:20:57] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "generate-lockfile" "--manifest-path" "/checkout/obj/build/tmp/distcheck-src/rust-src/lib/rustlib/src/rust/src/libstd/Cargo.toml"
[02:20:57] 
[02:20:57] 
[02:20:57] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test distcheck
[02:20:57] Build completed unsuccessfully in 2:17:47
---
travis_time:end:12fa98cf:start=1539846086410640742,finish=1539846086421668352,duration=11027610
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:148cb46e
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:12eb7986
travis_time:start:12eb7986
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:241d94f9
$ dmesg | grep -i kill
