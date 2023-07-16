plain
travis_time:end:19f0e510:start=1541693632881082181,finish=1541693687703607503,duration=54822525322
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:21:10]    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
[00:21:17] error[E0277]: the size for values of type `T` cannot be known at compilation time
[00:21:17]    --> libcore/mem.rs:144:14
[00:21:17]     |
[00:21:17] 144 |     unsafe { intrinsics::forget(t) }
[00:21:17]     |
[00:21:17]     = help: the trait `marker::Sized` is not implemented for `T`
[00:21:17]     = note: to learn more, visit <https://doc.rust-lang.org/book/second-edition/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:21:17]     = note: to learn more, visit <https://doc.rust-lang.org/book/second-edition/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
[00:21:17]     = help: consider adding a `where T: marker::Sized` bound
[00:21:17] note: required by `intrinsics::::forget`
[00:21:17]     |
[00:21:17]     |
[00:21:17] 719 |     pub fn forget<T>(_: T);
[00:21:17] 
[00:21:25] error: aborting due to previous error
[00:21:25] 
[00:21:25] For more information about this error, try `rustc --explain E0277`.
[00:21:25] For more information about this error, try `rustc --explain E0277`.
[00:21:25] error: Could not compile `core`.
[00:21:25] 
[00:21:25] To learn more, run the command again with --verbose.
[00:21:25] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:21:25] expected success, got: exit code: 101
[00:21:25] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1101:9
[00:21:25] travis_fold:end:stage1-std

[00:21:25] travis_time:end:stage1-std:start=1541694955363872284,finish=1541694983860242104,duration=28496369820


[00:21:25] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:21:25] Build completed unsuccessfully in 0:17:37
[00:21:25] Makefile:28: recipe for target 'all' failed
[00:21:25] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:11efb3f3
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:19b1f982:start=1541694984580977248,finish=1541694984585804604,duration=4827356
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:053b9d2b
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:107f2ba0
travis_time:start:107f2ba0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0c4923e4
$ dmesg | grep -i kill
