plain
[00:06:46]     Checking rustc_allocator v0.0.0 (/checkout/src/librustc_allocator)
[00:06:46]     Checking rustc_traits v0.0.0 (/checkout/src/librustc_traits)
[00:06:50]     Checking rustc_plugin v0.0.0 (/checkout/src/librustc_plugin)
[00:06:50]     Checking rustc_resolve v0.0.0 (/checkout/src/librustc_resolve)
[00:06:52] error[E0609]: no field `extern_prelude` on type `&mut resolve_imports::ImportResolver<'a, 'b, 'c>`
[00:06:52]    --> librustc_resolve/error_reporting.rs:135:41
[00:06:52]     |
[00:06:52] 135 |         let external_crate_names = self.extern_prelude.clone();
[00:06:52] 
[00:06:52] error: aborting due to previous error
[00:06:52] 
[00:06:52] For more information about this error, try `rustc --explain E0609`.
[00:06:52] For more information about this error, try `rustc --explain E0609`.
[00:06:52] error: Could not compile `rustc_resolve`.
[00:06:52] warning: build failed, waiting for other jobs to finish...
[00:07:20] error: build failed
[00:07:20] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:07:20] expected success, got: exit code: 101
[00:07:20] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1112:9
[00:07:20] travis_fold:end:stage0-rustc

[00:07:20] travis_time:end:stage0-rustc:start=1538541735174863493,finish=1538541896027504234,duration=160852640741

---
travis_time:end:26a4ef48:start=1538541896747656856,finish=1538541896756495733,duration=8838877
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0178d06b
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1bf7e0cc
travis_time:start:1bf7e0cc
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:08c0b5d8
$ dmesg | grep -i kill
