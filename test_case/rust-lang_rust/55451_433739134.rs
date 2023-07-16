plain
travis_time:end:08bd543e:start=1540753143185751711,finish=1540753204311295646,duration=61125543935
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
---
[00:53:25] .................................................................................................... 2200/4978
[00:53:30] .................................................................................................... 2300/4978
[00:53:34] .................................................................................................... 2400/4978
[00:53:38] .................................................................................................... 2500/4978
[00:53:42] .............................................................iiiiiiiii.............................. 2600/4978
[00:53:49] ............ii...................................................................................... 2800/4978
[00:53:52] .................................................................................................... 2900/4978
[00:53:56] .................................................................................................... 3000/4978
[00:53:59] .......i............................................................................................ 3100/4978
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:07:49] 
[01:07:49] running 112 tests
[01:07:52] i..ii...iii.......i...i.........i..iii...........i.....i.....ii...i..i.ii..............i...ii..ii.i. 100/112
[01:07:53] test result: ok. 82 passed; 0 failed; 30 ignored; 0 measured; 0 filtered out
[01:07:53] 
[01:07:53]  finished in 3.619
[01:07:53] travis_fold:end:test_codegen
---
[01:35:36]      |           ^^^^^^^^^ expected 1 parameter
[01:35:36]      | 
[01:35:36]     ::: libsyntax/parse/parser.rs:4099:5
[01:35:36]      |
[01:35:36] 4099 |     pub fn parse_pat(&mut self, expected: Option<&'static str>) -> PResult<'a, P<Pat>> {
[01:35:36]      |     ---------------------------------------------------------------------------------- defined here
[01:35:42] error: aborting due to previous error
[01:35:42] 
[01:35:42] For more information about this error, try `rustc --explain E0061`.
[01:35:42] error: Could not compile `syntax`.
[01:35:42] error: Could not compile `syntax`.
[01:35:42] 
[01:35:42] To learn more, run the command again with --verbose.
[01:35:42] 
[01:35:42] 
[01:35:42] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "syntax" "--" "--quiet"
[01:35:42] 
[01:35:42] 
[01:35:42] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:35:42] Build completed unsuccessfully in 0:47:40
[01:35:42] Build completed unsuccessfully in 0:47:40
[01:35:42] make: *** [check] Error 1
[01:35:42] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:08e64934
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:31eaf80a:start=1540758964843220029,finish=1540758964848714257,duration=5494228
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:000f5034
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then print
