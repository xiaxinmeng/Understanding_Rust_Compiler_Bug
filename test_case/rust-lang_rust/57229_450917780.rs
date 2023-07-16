plain
travis_time:end:046d1c72:start=1546444275453806109,finish=1546444335251201728,duration=59797395619
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:57:45] .................................................................................................... 2500/5224
[00:57:49] .................................................................................................... 2600/5224
[00:57:53] .................................................................................................... 2700/5224
[00:57:57] .................................................................................................... 2800/5224
[00:58:00] ...................F................................................................................ 2900/5224
[00:58:07] .................................................................................................... 3100/5224
[00:58:10] .............i...................................................................................... 3200/5224
[00:58:13] ............................................................................ii...i..ii.............. 3300/5224
[00:58:17] .................................................................................................... 3400/5224
---
[00:59:19] 
[00:59:19] ---- [ui] ui/issues/issue-56806.rs stdout ----
[00:59:19] diff of stderr:
[00:59:19] 
[00:59:19] 1 error[E0307]: invalid method receiver type: std::boxed::Box<(dyn Trait + 'static)>
[00:59:19] +   --> $DIR/issue-56806.rs:2:34
[00:59:19] 3    |
[00:59:19] 3    |
[00:59:19] 4 LL |     fn dyn_instead_of_self(self: Box<dyn Trait>);
[00:59:19] 
[00:59:19] 
[00:59:19] The actual stderr differed from the expected stderr.
[00:59:19] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-56806/issue-56806.stderr
[00:59:19] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-56806/issue-56806.stderr
[00:59:19] To update references, rerun the tests and pass the `--bless` flag
[00:59:19] To only update this specific test, also pass `--test-args issues/issue-56806.rs`
[00:59:19] error: 1 errors occurred comparing output.
[00:59:19] status: exit code: 1
[00:59:19] status: exit code: 1
[00:59:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-56806.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-56806/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknorting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:59:19] {"message":"For more information about this error, try `rustc --explain E0307`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0307`.\n"}
[00:59:19] ------------------------------------------
[00:59:19] 
[00:59:19] thread '[ui] ui/issues/issue-56806.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[00:59:19] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:59:19] 
[00:59:19] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:495:22
[00:59:19] 
[00:59:19] 
[00:59:19] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/ltravis_time:end:0ca3a460:start=1546444343601187202,finish=1546447903148233843,duration=3559547046641
travis_time:start:0de4a138
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Jan  2 16:51:43 UTC 2019
Wed, 02 Jan 2019 16:51:43 GMT
