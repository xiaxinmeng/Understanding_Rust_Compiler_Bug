plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:02cb88ee
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[01:56:37]    Compiling thread_local v0.3.6
[01:56:37]    Compiling idna v0.1.5
[01:56:37]    Compiling clippy_lints v0.0.212 (/checkout/src/tools/clippy/clippy_lints)
[01:56:39]    Compiling error-chain v0.12.0
[01:56:41] error: cannot find derive macro `Deserialize` in this scope
[01:56:41]     |
[01:56:41]     |
[01:56:41] 79  |               #[derive(Deserialize)]
[01:56:41] ...
[01:56:41] ...
[01:56:41] 118 | / define_Conf! {
[01:56:41] 119 | |     /// Lint: BLACKLISTED_NAME. The list of blacklisted names to lint about
[01:56:41] 120 | |     (blacklisted_names, "blacklisted_names", ["foo", "bar", "baz", "quux"] => Vec<String>),
[01:56:41] 121 | |     /// Lint: CYCLOMATIC_COMPLEXITY. The maximum cyclomatic complexity a function can have
[01:56:41] ...   |
[01:56:41] 157 | |     (trivial_copy_size_limit, "trivial_copy_size_limit", None => Option<u64>),
[01:56:41]     | |_- in this macro invocation
[01:56:41] 
[01:56:41] error: aborting due to previous error
[01:56:41] 
---
[01:57:17]    Compiling jsonrpc-core v8.0.1
[01:57:28]    Compiling rustc-rayon-core v0.1.1
[01:57:28]    Compiling clippy_lints v0.0.212 (/checkout/src/tools/clippy/clippy_lints)
[01:57:28]    Compiling languageserver-types v0.45.0
[01:57:31] error: cannot find derive macro `Deserialize` in this scope
[01:57:31]     |
[01:57:31]     |
[01:57:31] 79  |               #[derive(Deserialize)]
[01:57:31] ...
[01:57:31] ...
[01:57:31] 118 | / define_Conf! {
[01:57:31] 119 | |     /// Lint: BLACKLISTED_NAME. The list of blacklisted names to lint about
[01:57:31] 120 | |     (blacklisted_names, "blacklisted_names", ["foo", "bar", "baz", "quux"] => Vec<String>),
[01:57:31] 121 | |     /// Lint: CYCLOMATIC_COMPLEXITY. The maximum cyclomatic complexity a function can have
[01:57:31] ...   |
[01:57:31] 157 | |     (trivial_copy_size_limit, "trivial_copy_size_limit", None => Option<u64>),
[01:57:31]     | |_- in this macro invocation
[01:57:31] 
[01:57:31] error: aborting due to previous error
[01:57:31] 
---
[01:57:38] travis_fold:end:stage2-rls

[01:57:38] travis_time:end:stage2-rls:start=1542079108958249039,finish=1542079151790007330,duration=42831758291

[01:57:38] thread 'main' panicked at 'Unable to build RLS', bootstrap/dist.rs:73:9
[01:57:38] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
[01:57:38] Build completed unsuccessfully in 1:51:35
travis_time:end:0295727c:start=1542072093704888703,finish=1542079152117122963,duration=7058412234260
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
---
travis_time:end:0c76a0c8:start=1542079155915457733,finish=1542079155927756353,duration=12298620
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:19cd26aa
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:033d2776
travis_time:start:033d2776
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:270b43ff
$ dmesg | grep -i kill
