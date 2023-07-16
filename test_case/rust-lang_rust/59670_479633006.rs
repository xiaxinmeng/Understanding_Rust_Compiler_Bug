plain
[02:01:07] test util::lev_distance::test_lev_distance ... ok
[02:01:07] 
[02:01:07] failures:
[02:01:07] 
[02:01:07] ---- core::package_id::tests::debug stdout ----
[02:01:07] thread 'core::package_id::tests::debug' panicked at 'assertion failed: `(left == right)`
[02:01:07]   left: `"PackageId {\n    name: \"foo\",\n    version: \"1.0.0\",\n    source: \"registry `https://github.com/rust-lang/crates.io-index`\"\n}"`,
[02:01:07]  right: `"PackageId {\n    name: \"foo\",\n    version: \"1.0.0\",\n    source: \"registry `https://github.com/rust-lang/crates.io-index`\",\n}"`', src/tools/cargo/src/cargo/core/package_id.rs:235:9
[02:01:07] 
[02:01:07] 
[02:01:07] failures:
[02:01:07]     core::package_id::tests::debug
---
[02:01:07] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/cargo/Cargo.toml" "--features" "rustc-workspace-hack/all-static"
[02:01:07] expected success, got: exit code: 101
[02:01:07] 
[02:01:07] 
[02:01:07] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/test/run-pass/pretty src/test/run-fail/pretty src/test/run-pass-valgrind/pretty src/test/run-pass-fulldeps/pretty src/tools/cargo src/tools/cargotest
[02:01:07] Build completed unsuccessfully in 0:27:28
[02:01:07] Makefile:50: recipe for target 'check-aux' failed
[02:01:07] make: *** [check-aux] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:286d2b76
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Apr  3 19:46:07 UTC 2019
---
travis_time:end:00deeae0:start=1554320768839431572,finish=1554320768860153865,duration=20722293
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:014d6fa8
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:25b5edd4
travis_time:start:25b5edd4
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0f9848f6
$ dmesg | grep -i kill
