plain
travis_time:end:177fae9c:start=1548830098382928281,finish=1548830099255908321,duration=872980040
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:14:03]    Compiling rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
[00:14:14] error[E0507]: cannot move out of borrowed content
[00:14:14]     --> src/librustc_typeck/collect.rs:1306:44
[00:14:14]      |
[00:14:14] 1306 |         Node::GenericParam(param) => match param.kind {
[00:14:14]      |                                            |
[00:14:14]      |                                            cannot move out of borrowed content
[00:14:14]      |                                            cannot move out of borrowed content
[00:14:14]      |                                            help: consider borrowing here: `&param.kind`
[00:14:14] ...
[00:14:14] 1311 |             x => bug!("unexpected non-type Node::GenericParam: {:?}", x),
[00:14:14]      |             - data moved here
[00:14:14]      |
[00:14:14] note: move occurs because `x` has type `rustc::hir::GenericParamKind`, which does not implement the `Copy` trait
[00:14:14]     --> src/librustc_typeck/collect.rs:1311:13
[00:14:14]      |
[00:14:14] 1311 |             x => bug!("unexpected non-type Node::GenericParam: {:?}", x),
[00:14:14] 
[00:14:14] error: aborting due to previous error
[00:14:14] 
[00:14:14] For more information about this error, try `rustc --explain E0507`.
---
156148 ./src/llvm-project/clang
154512 ./obj/build/bootstrap/debug/incremental
141396 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc
139748 ./obj/build/bootstrap/debug/incremental/bootstrap-2ahv8almm435e
139744 ./obj/build/bootstrap/debug/incremental/bootstrap-2ahv8almm435e/s-f90isrud82-167y8w1-211f56mf0fscc
108528 ./src/llvm-project/lldb
104288 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu
104284 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release
101784 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps
---
travis_time:end:032694e0:start=1548831189545321365,finish=1548831189549414861,duration=4093496
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1560be22
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:232b8b2c
travis_time:start:232b8b2c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/cla
