plain
travis_time:end:07f1bc18:start=1543316636517541716,finish=1543316638760867822,duration=2243326106
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:21:51]    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:21:56]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:23:28]    Compiling proc_macro v0.0.0 (/checkout/src/libproc_macro)
[00:23:37]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:25:17] error: variable does not need to be mutable
[00:25:17]   --> src/librustc/infer/canonical/query_response.rs:70:14
[00:25:17]    |
[00:25:17] 70 |             |ref infcx, key, canonical_inference_vars| {
[00:25:17]    |              |
[00:25:17]    |              help: remove this `mut`
[00:25:17]    |
[00:25:17]    = note: `-D unused-mut` implied by `-D warnings`
[00:25:17]    = note: `-D unused-mut` implied by `-D warnings`
[00:25:17] 
[00:25:24] error: variable does not need to be mutable
[00:25:24]    --> src/librustc/traits/error_reporting.rs:500:79
[00:25:24]     |
[00:25:24] 500 |         let normalize = |candidate| self.tcx.global_tcx().infer_ctxt().enter(|ref infcx| {
[00:25:24]     |                                                                               |
[00:25:24]     |                                                                               help: remove this `mut`
[00:25:24] 
[00:25:24] error: variable does not need to be mutable
[00:25:24] error: variable does not need to be mutable
[00:25:24]    --> src/librustc/traits/object_safety.rs:560:34
[00:25:24]     |
[00:25:24] 560 |         self.infer_ctxt().enter(|ref infcx| {
[00:25:24]     |                                  |
[00:25:24]     |                                  help: remove this `mut`
[00:25:24] 
[00:25:33] error: aborting due to 3 previous errors
---
[00:25:33] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:25:33] expected success, got: exit code: 101
[00:25:33] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:25:33] Build completed unsuccessfully in 0:22:17
[00:25:33] make: *** [all] Error 1
[00:25:33] Makefile:28: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:192529ae
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Nov 27 11:29:41 UTC 2018
---
travis_time:end:1d336d6c:start=1543318181619762227,finish=1543318181626161724,duration=6399497
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1cb23ad8
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fo
