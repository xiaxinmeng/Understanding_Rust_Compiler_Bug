plain
travis_time:end:04637f34:start=1545502198099955332,finish=1545502254132288765,duration=56032333433
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:56:27] .................................................................................................... 400/5192
[00:56:31] .................................................................................................... 500/5192
[00:56:34] ..............................i..................................................................... 600/5192
[00:56:37] .................................................................................................... 700/5192
[00:56:43] ....................................................F............................................... 800/5192
[00:56:50] ..............................iiiii................................................................. 1000/5192
[00:56:53] .................................................................................................... 1100/5192
[00:56:55] .................................................................................................... 1200/5192
[00:56:57] .................................................................................................... 1300/5192
---
[00:59:07] diff of stderr:
[00:59:07] 
[00:59:07] 5    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:59:07] 6    |
[00:59:07] 7 note: ...which requires const-evaluating `Foo::bytes::{{constant}}`...
[00:59:07] +   --> $SRC_DIR/libcore/mem.rs:LL:COL
[00:59:07] +    |
[00:59:07] + LL |     intrinsics::size_of::<T>()
[00:59:07] +    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:59:07] 8 note: ...which requires computing layout of `Foo`...
[00:59:07] 9 note: ...which requires normalizing `ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: All }, value: [u8; _] }`...
[00:59:07] 10 note: ...which requires const-evaluating + checking `Foo::bytes::{{constant}}`...
[00:59:07] 
[00:59:07] The actual stderr differed from the expected stderr.
[00:59:07] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-size_of-cycle/const-size_of-cycle.stderr
[00:59:07] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-size_of-cycle/const-size_of-cycle.stderr
[00:59:07] To update references, rerun the tests and pass the `--bless` flag
[00:59:07] To only update this specific test, also pass `--test-args consts/const-size_of-cycle.rs`
[00:59:07] error: 1 errors occurred comparing output.
[00:59:07] status: exit code: 1
[00:59:07] status: exit code: 1
[00:59:07] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-size_of-cycle.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-size_of-cycle/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-size_of-cycle/auxiliary" "-A" "unused"
[00:59:07] ------------------------------------------
[00:59:07] 
[00:59:07] ------------------------------------------
[00:59:07] stderr:
[00:59:07] stderr:
[00:59:07] ------------------------------------------
[00:59:07] {"message":"cycle detected when const-evaluating + checking `Foo::bytes::{{constant}}`","code":{"code":"E0391","explanation":"\nThis error indicates that some types or traits depend on each other\nand therefore cannot be constructed.\n\nThe following example contains a circular dependency between two traits:\n\n