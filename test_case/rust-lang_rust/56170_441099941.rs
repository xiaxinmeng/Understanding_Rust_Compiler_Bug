plain
travis_time:end:06181f6d:start=1542910651068563798,finish=1542910652219717647,duration=1151153849
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:05:47]    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:05:51]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:07:14]    Compiling proc_macro v0.0.0 (/checkout/src/libproc_macro)
[00:07:24]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:07:24] error: expected one of `.`, `;`, `?`, or an operator, found `self`
[00:07:24]    --> librustc/util/profiling.rs:215:9
[00:07:24] 214 |             }
[00:07:24] 214 |             }
[00:07:24]     |              - expected one of `.`, `;`, `?`, or an operator here
[00:07:24] 215 |         self.current_timer.elapsed();
[00:07:24]     |         ^^^^ unexpected token
[00:07:29] error: unused import: `Duration`
[00:07:29]   --> librustc/util/profiling.rs:15:17
[00:07:29]    |
[00:07:29]    |
[00:07:29] 15 | use std::time::{Duration, Instant};
[00:07:29]    |
[00:07:29]    = note: `-D unused-imports` implied by `-D warnings`
[00:07:29] 
[00:07:56] error: aborting due to 2 previous errors
---
[00:07:56] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:07:56] expected success, got: exit code: 101
[00:07:56] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:07:56] Build completed unsuccessfully in 0:03:59
[00:07:56] Makefile:28: recipe for target 'all' failed
[00:07:56] make: *** [all] Error 1
72532 ./src/llvm/lib
69912 ./src/llvm-emscripten/lib
67936 ./src/test
56436 ./src/llvm/test/MC
