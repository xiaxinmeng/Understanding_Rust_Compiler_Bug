plain
travis_time:end:072f6028:start=1549775163029528478,finish=1549775173444295840,duration=10414767362
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:18:29]    Compiling rustc_plugin v0.0.0 (/checkout/src/librustc_plugin)
[00:18:30] error[E0432]: unresolved import `crate::errors`
[00:18:30]   --> src/librustc_resolve/error_reporting.rs:12:12
[00:18:30]    |
[00:18:30] 12 | use crate::errors::{Applicability, DiagnosticBuilder, DiagnosticId};
[00:18:30]    |            ^^^^^^ did you mean `syntax::errors`?
[00:18:32] error: aborting due to previous error
[00:18:32] 
[00:18:32] For more information about this error, try `rustc --explain E0432`.
[00:18:32] error: Could not compile `rustc_resolve`.
[00:18:32] error: Could not compile `rustc_resolve`.
[00:18:32] warning: build failed, waiting for other jobs to finish...
[00:19:12] error: build failed
[00:19:12] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:19:12] expected success, got: exit code: 101
[00:19:12] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:19:12] Build completed unsuccessfully in 0:15:10
[00:19:12] Makefile:18: recipe for target 'all' failed
[00:19:12] make: *** [all] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:13bc4d5f
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Feb 10 05:25:35 UTC 2019
