plain
travis_time:end:046cf26b:start=1550049436063082300,finish=1550049436987767363,duration=924685063
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:05:57]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:07:06]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:12:38]    Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
[00:12:38]    Compiling rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
[00:12:53] error[E0599]: no method named `binary_op_imm` found for type `&mut interpret::eval_context::EvalContext<'a, 'mir, 'tcx, M>` in the current scope
[00:12:53]    --> src/librustc_mir/interpret/intrinsics.rs:129:46
[00:12:53]     |
[00:12:53] 129 |                 let (val, overflowed) = self.binary_op_imm(if is_add {
[00:12:53]     |
[00:12:53]     |
[00:12:53]     = help: did you mean `binary_op`?
[00:12:54] error: aborting due to previous error
[00:12:54] 
[00:12:54] For more information about this error, try `rustc --explain E0599`.
[00:12:54] error: Could not compile `rustc_mir`.
[00:12:54] error: Could not compile `rustc_mir`.
[00:12:54] warning: build failed, waiting for other jobs to finish...
[00:15:45] error: build failed
[00:15:45] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:15:45] expected success, got: exit code: 101
[00:15:45] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:15:45] Build completed unsuccessfully in 0:11:57
[00:15:45] make: *** [all] Error 1
[00:15:45] Makefile:18: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1d076d7a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Feb 13 09:33:12 UTC 2019
---
travis_time:end:146153e1:start=1550050393251264169,finish=1550050393257523245,duration=6259076
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2cb8ae64
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1b4b1cd9
travis_time:start:1b4b1cd9
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0dfaec06
$ dmesg | grep -i kill
