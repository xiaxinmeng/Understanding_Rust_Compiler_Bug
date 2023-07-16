plain
travis_time:end:05fb5810:start=1548796732319083221,finish=1548796818901711168,duration=86582627947
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[01:03:18] ................................................................................................i... 3800/5358
[01:03:20] .................................................................................................... 3900/5358
[01:03:22] .....................................................i.............................................. 4000/5358
[01:03:25] .................................................................................................... 4100/5358
[01:03:33] .........................................F.......................................................... 4200/5358
[01:03:41] .................................................................................................... 4400/5358
[01:03:45] .................................................................................................... 4500/5358
[01:03:50] .........i.......................................................................................... 4600/5358
[01:03:54] .................................................................................................... 4700/5358
---
[01:04:14] .................................................................................................i.. 5300/5358
[01:04:15] ..........................................................
[01:04:15] failures:
[01:04:15] 
[01:04:15] ---- [ui] ui/privacy/pub-priv-dep/std-pub.rs stdout ----
[01:04:15] 
[01:04:15] error: test compilation failed although it shouldn't!
[01:04:15] status: exit code: 1
[01:04:15] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/privacy/pub-priv-dep/std-pub.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/pub-priv-dep/std-pub/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/pub-priv-dep/std-pub/auxiliary" "-A" "unused"
[01:04:15] ------------------------------------------
[01:04:15] 
[01:04:15] ------------------------------------------
[01:04:15] stderr:
[01:04:15] stderr:
[01:04:15] ------------------------------------------
[01:04:15] {"message":"unknown feature `public_private_dependencies`","code":{"code":"E0635","explanation":"\nThe `#![feature]` attribute specified an unknown feature.\n\nErroneous code example:\n\n