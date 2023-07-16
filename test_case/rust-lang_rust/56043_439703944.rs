plain
travis_time:end:03fcf380:start=1542554063353629564,finish=1542554118398878364,duration=55045248800
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:50:11] .................................................................................................... 3000/5023
[00:50:14] ..........................................i......................................................... 3100/5023
[00:50:18] .................................................................................................... 3200/5023
[00:50:21] .....i.i..ii........................................................................................ 3300/5023
[00:50:25] ...............................................................................................F.... 3400/5023
[00:50:28] ...................................................................................i.ii............. 3500/5023
[00:50:32] ...i................................................................................................ 3700/5023
[00:50:33] ...........................................................i........................................ 3800/5023
[00:50:35] .................................................................................................... 3900/5023
[00:50:39] .................................................................................................... 4000/5023
---
[00:51:13] ---- [ui] ui/nll/ty-outlives/issue-55756.rs stdout ----
[00:51:13] 
[00:51:13] error: ui test compiled successfully!
[00:51:13] status: exit code: 0
[00:51:13] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/ty-outlives/issue-55756.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/ty-outlives/issue-55756/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/ty-outlives/issue-55756/auxiliary" "-A" "unused"
[00:51:13] ------------------------------------------
[00:51:13] 
[00:51:13] ------------------------------------------
[00:51:13] stderr:
---
[00:51:13] failures:
[00:51:13]     [ui] ui/nll/ty-outlives/issue-55756.rs
[00:51:13] 
[
[00:51:13] Makefile:58: recipe for target 'check' failed
[00:51:13] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:25e9e218
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Nov 18 16:06:41 UTC 2018
---
travis_time:end:0a797714:start=1542557202309422864,finish=1542557202315345732,duration=5922868
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0768f479
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_fai
