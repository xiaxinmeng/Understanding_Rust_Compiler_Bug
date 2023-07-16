plain
travis_time:end:29052192:start=1546082696878615507,finish=1546082754606746020,duration=57728130513
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[01:01:26] .................................................................................................... 300/2935
[01:01:36] .................................................................................................... 400/2935
[01:01:45] .................................................................................................... 500/2935
[01:01:56] .................................................................................................... 600/2935
[01:02:12] ...............................F.F.................................................................. 700/2935
[01:02:23] .................................................................................................... 800/2935
[01:02:32] ........................................F..F........................................................ 900/2935
[01:02:59] .................................................................................................... 1100/2935
[01:03:08] .................................................................................................... 1200/2935
[01:03:18] .................................................................................................... 1300/2935
[01:03:31] .................................................................................................... 1400/2935
---
[01:07:14] failures:
[01:07:14] 
[01:07:14] ---- [run-pass] run-pass/drop/dynamic-drop.rs#lexical stdout ----
[01:07:14] normalized stderr:
[01:07:14] warning: the feature `pin` has been stable since 1.33.0 and no longer requires an attribute to enable
[01:07:14]    |
[01:07:14]    |
[01:07:14] LL | #![feature(generators, generator_trait, untagged_unions, pin)]
[01:07:14]    |
[01:07:14]    = note: #[warn(stable_features)] on by default
[01:07:14] 
[01:07:14] 
[01:07:14] 
[01:07:14] 
[01:07:14] 
[01:07:14] The actual stderr differed from the expected stderr.
[01:07:14] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/drop/dynamic-drop.lexical/dynamic-drop.lexical.stderr
[01:07:14] To update references, rerun the tests and pass the `--bless` flag
[01:07:14] To only update this specific test, also pass `--test-args drop/dynamic-drop.rs`
[01:07:14] 
[01:07:14] error in revision `lexical`: 1 errors occurred comparing output.
[01:07:14] status: exit code: 0
[01:07:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ru|                                                          ^^^\n   |\n   = note: #[warn(stable_features)] on by default\n\n"}
[01:07:14] ------------------------------------------
[01:07:14] 
[01:07:14] thread '[run-pass] run-pass/drop/dynamic-drop.rs#lexical' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:07:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:07:14] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:07:14] 
[01:07:14] ---- [run-pass] run-pass/drop/dynamic-drop.rs#nll stdout ----
[01:07:14] normalized stderr:
[01:07:14] warning: the feature `pin` has been stable since 1.33.0 and no longer requires an attribute to enable
[01:07:14]    |
[01:07:14]    |
[01:07:14] LL | #![feature(generators, generator_trait, untagged_unions, pin)]
[01:07:14]    |
[01:07:14]    = note: #[warn(stable_features)] on by default
[01:07:14] 
[01:07:14] 
[01:07:14] 
[01:07:14] 
[01:07:14] 
[01:07:14] The actual stderr differed from the expected stderr.
[01:07:14] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/drop/dynamic-drop.nll/dynamic-drop.nll.stderr
[01:07:14] To update references, rerun the tests and pass the `--bless` flag
[01:07:14] To only update this specific test, also pass `--test-args drop/dynamic-drop.rs`
[01:07:14] 
[01:07:14] error in revision `nll`: 1 errors occurred comparing output.
[01:07:14] status: exit code: 0
[01:07:14] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/drop/dynamic-drop.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "nll" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/drop/dynamic-drop.nll/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/drop/dynamic-drop.nll/auxiliary"
[01:07:14] ------------------------------------------
[01:07:14] 
[01:07:14] ------------------------------------------
[01:07:14] stderr:
[01:07:14] stderr:
[01:07:14] ------------------------------------------
[01:07:14] {"message":"the feature `pin` has been stable since 1.33.0 and no longer requires an attribute to enable","code":{"code":"stable_features","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/run-pass/drop/dynamic-drop.rs","byte_start":245,"byte_end":248,"line_start":9,"line_end":9,"column_start":58,"column_end":61,"is_primary":true,"text":[{"text":"#![feature(generators, generator_trait, untagged_unions, pin)]","highlight_start":58,"highlight_end":61}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"#[warn(stable_features)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: the feature `pin` has been stable since 1.33.0 and no longer requires an attribute to enable\n  --> /checkout/src/test/run-pass/drop/dynamic-drop.rs:9:58\n   |\nLL | #![feature(generators, generator_trait, untagged_unions, pin)]\n   |                                                          ^^^\n   |linux-gnu/stage2-tools/x86_64-unknown-linux-gnu
57976 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu
57972 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release
56896 ./src/llvm/test/MC
56092 ./obj/build/x86_64-unknown-linux-gnu/test/run-pass/proc-macro
