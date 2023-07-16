plain
travis_time:end:0a93d2c8:start=1542806290134257400,finish=1542806343566019549,duration=53431762149
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:47:52]    Compiling serde_derive v1.0.75
[00:48:20]    Compiling failure v0.1.2
[00:48:21]    Compiling rustfix v0.4.2
[00:48:22]    Compiling compiletest v0.0.0 (/checkout/src/tools/compiletest)
[00:48:22] error: expected one of `.`, `;`, `?`, or an operator, found `let`
[00:48:22]     |
[00:48:22]     |
[00:48:22] 122 |     let start = line.find(tag)?
[00:48:22]     |                                - expected one of `.`, `;`, `?`, or an operator here
[00:48:22] 123 |     let (follow, adjusts) = if line[start + tag.len()..].chars().next().unwrap() == '|' {
[00:48:22]     |     ^^^ unexpected token
[00:48:23] error: aborting due to previous error
[00:48:23] 
[00:48:23] error: Could not compile `compiletest`.
[00:48:23] 
[00:48:23] 
[00:48:23] To learn more, run the command again with --verbose.
[00:48:23] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/compiletest/Cargo.toml" "--message-format" "json"
[00:48:23] expected success, got: exit code: 101
[00:48:23] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:48:23] Build completed unsuccessfully in 0:00:56
[00:48:23] make: *** [check] Error 1
[00:48:23] Makefile:58: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:34144c65
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Nov 21 14:07:35 UTC 2018
---
travis_time:end:2b56f100:start=1542809256737507380,finish=1542809256742340066,duration=4832686
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:18059242
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch
