plain
travis_time:end:0561cccf:start=1553522842118343460,finish=1553522844362975229,duration=2244631769
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[01:08:23] .................................................................................................... 1400/5488
[01:08:26] .................................................................................................... 1500/5488
[01:08:29] .................................................................................................... 1600/5488
[01:08:32] .........................................i.......................................................... 1700/5488
[01:08:35] .....................................................................................F.............. 1800/5488
[01:08:42] .................................................................................................... 2000/5488
[01:08:46] ...........................................................................i........................ 2100/5488
[01:08:49] .................................................................................................... 2200/5488
[01:08:53] .................................................................................................... 2300/5488
---
[01:10:51] failures:
[01:10:51] 
[01:10:51] ---- [ui] ui/impl-trait/bindings.rs stdout ----
[01:10:51] 
[01:10:51] error: Error: expected failure status (Some(1)) but received status Some(101).
[01:10:51] status: exit code: 101
[01:10:51] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/bindings.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/bindings/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/bindings/auxiliary" "-A" "unused"
[01:10:51] ------------------------------------------
[01:10:51] 
[01:10:51] ------------------------------------------
[01:10:51] stderr:
[01:10:51] stderr:
[01:10:51] ------------------------------------------
[01:10:51] {"message":"attempt to use a non-constant value in a constant","code":{"code":"E0435","explanation":"\nA non-constant value was used in a constant expression.\n\nErroneous code example:\n\n