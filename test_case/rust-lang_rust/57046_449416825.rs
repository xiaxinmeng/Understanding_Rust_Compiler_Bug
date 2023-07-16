plain
travis_time:end:011a5620:start=1545405416917645196,finish=1545405417911755641,duration=994110445
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:05:38]    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:05:43]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:07:04]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:07:57] error: unnecessary `unsafe` block
[00:07:57]   --> src/librustc/dep_graph/graph.rs:81:9
[00:07:57] 81 |         unsafe {
[00:07:57]    |         ^^^^^^ unnecessary `unsafe` block
[00:07:57]    |
[00:07:57]    = note: `-D unused-unsafe` implied by `-D warnings`
[00:07:57]    = note: `-D unused-unsafe` implied by `-D warnings`
[00:07:57] 
[00:07:57] error: unnecessary `unsafe` block
[00:07:57]   --> src/librustc/dep_graph/graph.rs:88:9
[00:07:57]    |
[00:07:57] 88 |         unsafe { DepNodeIndex { idx: value as usize } }
[00:07:57] 
[00:07:58] error: aborting due to 2 previous errors
[00:07:58] 
[00:07:58] error: Could not compile `rustc`.
[00:07:58] error: Could not compile `rustc`.
[00:07:58] 
[00:07:58] To learn more, run the command again with --verbose.
[00:07:58] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:07:58] expected success, got: exit code: 101
[00:07:58] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:07:58] Build completed unsuccessfully in 0:04:29
[00:07:58] Makefile:28: recipe for target 'all' failed
[00:07:58] make: *** [all] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1d8c527e
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Dec 21 15:25:05 UTC 2018
