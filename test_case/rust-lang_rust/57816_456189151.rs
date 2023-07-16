plain
travis_time:end:126a393c:start=1548101546472067657,finish=1548101618263621152,duration=71791553495
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:05:37]    Compiling tempfile v3.0.5
[00:05:39]    Compiling arena v0.0.0 (/checkout/src/libarena)
[00:05:40]    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:05:44]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:05:58] error[E0412]: cannot find type `ParenthesisedArgs` in this scope
[00:05:58]    --> src/libsyntax/ast.rs:195:6
[00:05:58]     |
[00:05:58] 195 | impl ParenthesisedArgs {
[00:05:58]     |      ^^^^^^^^^^^^^^^^^ did you mean `ParenthesizedArgs`?
[00:06:04] error: aborting due to previous error
[00:06:04] 
[00:06:04] For more information about this error, try `rustc --explain E0412`.
[00:06:04] error: Could not compile `syntax`.
[00:06:04] error: Could not compile `syntax`.
[00:06:04] 
[00:06:04] To learn more, run the command again with --verbose.
[00:06:04] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:06:04] expected success, got: exit code: 101
[00:06:04] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:06:04] Build completed unsuccessfully in 0:02:25
[00:06:04] Makefile:18: recipe for target 'all' failed
[00:06:04] make: *** [all] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:002183cf
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Jan 21 20:19:51 UTC 2019
