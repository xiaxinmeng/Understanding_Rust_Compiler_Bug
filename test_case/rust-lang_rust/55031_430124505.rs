plain
[00:01:21] configure: 
[00:01:21] configure: rust.debug-assertions := True
[00:01:21] configure: rust.ignore-git      := False
[00:01:21] configure: build.print-step-timings := True
[00:01:21] configure: rust.verify-llvm-ir  := True
[00:01:21] configure: llvm.assertions      := True
[00:01:21] configure: build.locked-deps    := True
[00:01:21] configure: llvm.ccache          := sccache
[00:01:21] configure: build.openssl-static := True
---
[00:14:55] configure: 
[00:14:55] configure: rust.debug-assertions := True
[00:14:55] configure: rust.ignore-git      := False
[00:14:55] configure: build.print-step-timings := True
[00:14:55] configure: rust.verify-llvm-ir  := True
[00:14:55] configure: build.submodules     := False
[00:14:55] configure: llvm.assertions      := True
[00:14:55] configure: build.locked-deps    := True
[00:14:55] configure: llvm.ccache          := sccache
---
[02:22:10] warning: spurious network error (2 tries remaining): curl error: Could not resolve host: github.com
[02:22:10] ; class=Net (12)
[02:22:31] warning: spurious network error (1 tries remaining): curl error: Could not resolve host: github.com
[02:22:31] ; class=Net (12)
[02:22:51] error: failed to load source for a dependency on `rand`
[02:22:51] Caused by:
[02:22:51]   Unable to update registry `https://github.com/rust-lang/crates.io-index`
[02:22:51] 
[02:22:51] Caused by:
[02:22:51] Caused by:
[02:22:51]   failed to fetch `https://github.com/rust-lang/crates.io-index`
[02:22:51] 
[02:22:51] Caused by:
[02:22:51]   curl error: Could not resolve host: github.com
[02:22:51] ; class=Net (12)
[02:22:51] 
[02:22:51] 
[02:22:51] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "generate-lockfile" "--manifest-path" "/checkout/obj/build/tmp/distcheck-src/rust-src/lib/rustlib/src/rust/src/libstd/Cargo.toml"
[02:22:51] 
[02:22:51] 
[02:22:51] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test distcheck
[02:22:51] Build completed unsuccessfully in 2:19:42
---
travis_time:end:0108fa8e:start=1539673356022073514,finish=1539673356030718510,duration=8644996
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:18881eac
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0d5eff60
travis_time:start:0d5eff60
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0ba58860
$ dmesg | grep -i kill
