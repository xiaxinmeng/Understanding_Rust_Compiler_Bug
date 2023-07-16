plain
travis_time:end:2eedb140:start=1555379039076918081,finish=1555379041119706030,duration=2042787949
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:03:57] .................................................................................................... 500/5542
[01:04:01] ................................................i................................................... 600/5542
[01:04:04] .................................................................................................... 700/5542
[01:04:09] .................................................................................................... 800/5542
[01:04:13] .....................................................................F.............................. 900/5542
[01:04:21] ..............................................iiiii................................................. 1100/5542
[01:04:25] .................................................................................................... 1200/5542
[01:04:27] .................................................................................................... 1300/5542
[01:04:30] .................................................................................................... 1400/5542
[01:04:30] .................................................................................................... 1400/5542
[01:04:33] .................................................................................................... 1500/5542
[01:04:35] ...........................FF....................................................................... 1600/5542
[01:04:42] .................................................................................................... 1800/5542
[01:04:46] .................................................................................................... 1900/5542
[01:04:49] .................................................................................................... 2000/5542
[01:04:53] .................................................................................................... 2100/5542
---
[01:07:03] 
[01:07:03] ---- [ui] ui/consts/min_const_fn/allow_const_fn_ptr_feature_gate.rs stdout ----
[01:07:03] diff of stderr:
[01:07:03] 
[01:07:03] - error[E0658]: unless otherwise specified, attributes with the prefix `rustc_` are reserved for internal compiler diagnostics (see issue #29642)
[01:07:03] + error[E0658]: unless otherwise specified, attributes with the prefix `rustc_` are reserved for internal compiler diagnostics
[01:07:03] 2   --> $DIR/allow_const_fn_ptr_feature_gate.rs:7:3
[01:07:03] 4 LL | #[rustc_allow_const_fn_ptr]
[01:07:03] 
[01:07:03] 5    |   ^^^^^^^^^^^^^^^^^^^^^^^^
[01:07:03] 6    |
[01:07:03] 6    |
[01:07:03] +    = note: for more information, see https://github.com/rust-lang/rust/issues/29642
[01:07:03] 7    = help: add #![feature(rustc_attrs)] to the crate attributes to enable
[01:07:03] 8 
[01:07:03] 9 error: aborting due to previous error
[01:07:03] 
[01:07:03] 
[01:07:03] The actual stderr differed from the expected stderr.
[01:07:03] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/allow_const_fn_ptr_feature_gate/allow_const_fn_ptr_feature_gate.stderr
[01:07:03] To update references, rerun the tests and pass the `--bless` flag
[01:07:03] To only update this specific test, also pass `--test-args consts/min_const_fn/allow_const_fn_ptr_feature_gate.rs`
[01:07:03] error: 1 errors occurred comparing output.
[01:07:03] status: exit code: 1
[01:07:03] status: exit code: 1
[01:07:03] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/min_const_fn/allow_const_fn_ptr_feature_gate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/allow_const_fn_ptr_feature_gate/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/allow_const_fn_ptr_feature_gate/auxiliary" "-A" "unused"
[01:07:03] ------------------------------------------
[01:07:03] 
[01:07:03] ------------------------------------------
[01:07:03] stderr:
[01:07:03] stderr:
[01:07:03] ------------------------------------------
[01:07:03] {"message":"unless otherwise specified, attributes with the prefix `rustc_` are reserved for internal compiler diagnostics","code":{"code":"E0658","explanation":"\nAn unstable feature was used.\n\nErroneous code example:\n\n