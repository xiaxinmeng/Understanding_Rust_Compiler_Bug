plain
travis_time:end:0305bf38:start=1561023431806014071,finish=1561023432781729081,duration=975715010
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:05:17] 
[01:05:17] running 9 tests
[01:05:17] iiiiiiiii
[01:05:17] 
[01:05:17]  finished in 0.149
[01:05:17] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:05:33] 
[01:05:33] running 122 tests
[01:05:58] .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
[01:06:03] .i.i......iii.i.....ii
[01:06:03] 
[01:06:03]  finished in 30.047
[01:06:03] travis_fold:end:test_debuginfo

---
travis_time:start:test_run-pass-fulldeps
Check compiletest suite=run-pass-fulldeps mode=run-pass (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:06:11] 
[01:06:11] running 44 tests
[01:06:51] .......................F....................
[01:06:51] failures:
[01:06:51] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:06:51] 
[01:06:51] ---- [run-pass] run-pass-fulldeps/issue-40001.rs stdout ----
[01:06:51] ---- [run-pass] run-pass-fulldeps/issue-40001.rs stdout ----
[01:06:51] 
[01:06:51] error: auxiliary build of "/checkout/src/test/run-pass-fulldeps/auxiliary/issue-40001-plugin.rs" failed to compile: 
[01:06:51] status: exit code: 1
[01:06:51] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/auxiliary/issue-40001-plugin.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/issue-40001/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/issue-40001/auxiliary"
[01:06:51] ------------------------------------------
[01:06:51] 
[01:06:51] ------------------------------------------
[01:06:51] stderr:
[01:06:51] stderr:
[01:06:51] ------------------------------------------
[01:06:51] warning: unused import: `syntax::ext::base::*`
[01:06:51]   --> /checkout/src/test/run-pass-fulldeps/auxiliary/issue-40001-plugin.rs:11:5
[01:06:51] LL | use syntax::ext::base::*;
[01:06:51]    |     ^^^^^^^^^^^^^^^^^^^^
[01:06:51]    |
[01:06:51]    = note: #[warn(unused_imports)] on by default
[01:06:51]    = note: #[warn(unused_imports)] on by default
[01:06:51] 
[01:06:51] warning: unused import: `rustc::hir::map as hir_map`
[01:06:51]   --> /checkout/src/test/run-pass-fulldeps/auxiliary/issue-40001-plugin.rs:17:5
[01:06:51]    |
[01:06:51] LL | use rustc::hir::map as hir_map;
[01:06:51] 
[01:06:51] warning: unused import: `rustc::ty`
[01:06:51] warning: unused import: `rustc::ty`
[01:06:51]   --> /checkout/src/test/run-pass-fulldeps/auxiliary/issue-40001-plugin.rs:20:5
[01:06:51] LL | use rustc::ty;
[01:06:51]    |     ^^^^^^^^^
[01:06:51] 
[01:06:51] warning: unused import: `ast`
[01:06:51] warning: unused import: `ast`
[01:06:51]   --> /checkout/src/test/run-pass-fulldeps/auxiliary/issue-40001-plugin.rs:21:14
[01:06:51] LL | use syntax::{ast, source_map};
[01:06:51]    |              ^^^
[01:06:51] 
[01:06:51] error[E0599]: no method named `get_by_hir_id` found for type `&rustc::hir::map::Map<'_>` in the current scope
[01:06:51] error[E0599]: no method named `get_by_hir_id` found for type `&rustc::hir::map::Map<'_>` in the current scope
[01:06:51]   --> /checkout/src/test/run-pass-fulldeps/auxiliary/issue-40001-plugin.rs:46:39
[01:06:51] LL |         let item = match cx.tcx.hir().get_by_hir_id(id) {
[01:06:51]    |                                       ^^^^^^^^^^^^^ help: there is a method with a similar name: `find_by_hir_id`
[01:06:51] 
[01:06:51] error: aborting due to previous error
---
[01:06:51] test result: FAILED. 43 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:06:51] 
[01:06:51] 
[01:06:51] 
[01:06:51] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:06:51] 
[01:06:51] 
[01:06:51] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:06:51] Build completed unsuccessfully in 1:02:13
---
travis_time:end:07b02928:start=1561027457146565924,finish=1561027457151151765,duration=4585841
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0b36c8fa
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:047381ac
travis_time:start:047381ac
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:031e6198
$ dmesg | grep -i kill
