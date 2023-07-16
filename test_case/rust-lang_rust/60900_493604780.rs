plain
travis_time:end:0fa70be6:start=1558126492135971338,finish=1558126591477599402,duration=99341628064
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:31:08]    Compiling synstructure v0.10.1
[00:31:35]    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
[00:31:44]    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:31:49]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:31:49] error[E0283]: type annotations required: cannot resolve `dyn emitter::Emitter + rustc_data_structures::sync::Send: rustc_data_structures::sync::Send`
[00:31:49]     |
[00:31:49] 391 |             e,
[00:31:49]     |             ^
[00:31:49]     |
[00:31:49]     |
[00:31:49]     = note: required for the cast to the object type `dyn emitter::Emitter + rustc_data_structures::sync::Send`
[00:31:50] error: aborting due to previous error
[00:31:50] 
[00:31:50] For more information about this error, try `rustc --explain E0283`.
[00:31:50] error: Could not compile `rustc_errors`.
---
156496 ./src/llvm-project/clang
145168 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu
145164 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release
142472 ./obj/build/bootstrap/debug/incremental/bootstrap-gm2kk8y15os9
142468 ./obj/build/bootstrap/debug/incremental/bootstrap-gm2kk8y15os9/s-fcb5jjwfjh-1bn7ic2-34hadj2q5a3fu
123644 ./src/llvm-project/llvm/test/CodeGen
108532 ./src/llvm-project/lldb
97584 ./src/llvm-project/clang/test
89964 ./src/llvm-emscripten/test/CodeGen
---
travis_time:end:1b13fafe:start=1558128511368520562,finish=1558128511373232229,duration=4711667
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0a8d6488
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
