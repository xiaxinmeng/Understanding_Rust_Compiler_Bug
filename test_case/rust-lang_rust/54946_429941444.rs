plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:00ed7614
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[00:08:58]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:09:21] error[E0061]: this function takes 1 parameter but 0 parameters were supplied
[00:09:21]    --> librustc/traits/error_reporting.rs:428:28
[00:09:21]     |
[00:09:21] 428 |                     scalar.to_usize().ok()
[00:09:21]     | 
[00:09:21]    ::: librustc/mir/interpret/value.rs:312:5
[00:09:21]     |
[00:09:21]     |
[00:09:21] 312 |     pub fn to_usize(self, cx: impl HasDataLayout) -> EvalResult<'static, u64> {
[00:09:21]     |     ------------------------------------------------------------------------- defined here
[00:09:26] error: aborting due to previous error
[00:09:26] 
[00:09:26] For more information about this error, try `rustc --explain E0061`.
[00:09:26] error: Could not compile `rustc`.
---
[00:09:26] travis_time:end:stage0-rustc:start=1539624114042681152,finish=1539624270879608834,duration=156836927682

[00:09:26] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:09:26] expected success, got: exit code: 101
[00:09:26] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1112:9
[00:09:26] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
[00:09:26] Build completed unsuccessfully in 0:03:41
travis_time:end:3ab44230:start=1539623704224758169,finish=1539624271135938987,duration=566911180818

---
travis_time:end:28aef0f8:start=1539624271549451395,finish=1539624271555836751,duration=6385356
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:03e59b8f
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:04be1b6a
travis_time:start:04be1b6a
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0863338a
$ dmesg | grep -i kill
