plain
travis_time:end:10f9ba9b:start=1554997816865960954,finish=1554997819279387399,duration=2413426445
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:06:35] .................................................................................................... 4400/5530
[01:06:39] .................................................................................................... 4500/5530
[01:06:42] .................................................................................................... 4600/5530
[01:06:46] .................................................................................................... 4700/5530
[01:06:53] ...............................................................................F.................... 4800/5530
[01:06:59] .................................................................................................... 5000/5530
[01:07:04] .................................................................................................... 5100/5530
[01:07:07] .................................................................................................... 5200/5530
[01:07:11] .................................................................................................... 5300/5530
[01:07:11] .................................................................................................... 5300/5530
[01:07:13] .................................................................................................... 5400/5530
[01:07:16] ....................................................................i............................... 5500/5530
[01:07:17] ..............................
[01:07:17] failures:
[01:07:17] 
[01:07:17] ---- [ui] ui/rust-2018/uniform-paths/macro-rules.rs stdout ----
[01:07:17] 
[01:07:17] error: Error: expected failure status (Some(1)) but received status Some(101).
[01:07:17] status: exit code: 101
[01:07:17] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rust-2018/uniform-paths/macro-rules.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/uniform-paths/macro-rules/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/uniform-paths/macro-rules/auxiliary" "-A" "unused"
[01:07:17] ------------------------------------------
[01:07:17] 
[01:07:17] ------------------------------------------
[01:07:17] stderr:
[01:07:17] stderr:
[01:07:17] ------------------------------------------
[01:07:17] {"message":"`legacy_macro` is private, and cannot be re-exported","code":{"code":"E0364","explanation":"\nPrivate items cannot be publicly re-exported. This error indicates that you\nattempted to `pub use` a type or value that was not itself public.\n\nErroneous code example:\n\n