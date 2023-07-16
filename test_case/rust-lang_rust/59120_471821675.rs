plain
travis_time:end:2294a8e6:start=1552351866253667459,finish=1552351867343357056,duration=1089689597
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:02:06] Successfully built b2d081f21945
[00:02:06] Successfully tagged rust-ci:latest
[00:02:06] Built container sha256:b2d081f21945dba0e311f4d10a763e4c0522073c932b998807ded93b5bfb6863
[00:02:06] Uploading finished image to s3://rust-lang-ci-sccache2/docker/62c93bf697156b017a22966f8ca731737ea7d0a974527596f0b565ab0f2afc82bfdb4433a508c739a9c55c3a8b92cc9175c86be6278091d791c98549912f4d89
[00:02:56] upload failed: - to s3://rust-lang-ci-sccache2/docker/62c93bf697156b017a22966f8ca731737ea7d0a974527596f0b565ab0f2afc82bfdb4433a508c739a9c55c3a8b92cc9175c86be6278091d791c98549912f4d89 Unable to locate credentials

[00:02:56] travis_time:end:09a2baf6:start=1552351903990574833,finish=1552352054171351882,duration=150180777049
[CI_JOB_NAME=x86_64-gnu-llvm-6.0]
[00:02:56] [CI_JOB_NAME=x86_64-gnu-llvm-6.0]
---
[01:18:20] .................................................................................................... 700/5448
[01:18:25] .................................................................................................... 800/5448
[01:18:30] ................................................................................................i... 900/5448
[01:18:36] ............i....................................................................................... 1000/5448
[01:18:39] ERROR 2019-03-12T02:09:57Z: compiletest::runtest: fatal error, panic: "aux-build `/checkout/src/test/ui/e0119/auxiliary/issue_23563_a.rs` source not found"
[01:18:40] .........................iiiii..........................................................F........... 1100/5448
[01:18:46] .................................................................................................... 1300/5448
[01:18:49] .................................................................................................... 1400/5448
[01:18:52] .................................................................................................... 1500/5448
[01:18:55] .................................................................................................... 1600/5448
[01:18:55] .................................................................................................... 1600/5448
[01:18:59] ..............................i..................................................................... 1700/5448
[01:19:03] .................................................................................................... 1800/5448
[01:19:08] .................................................................................................... 1900/5448
[01:19:11] .................................................................................................... 2000/5448
[01:19:15] .........................................................i.......................................... 2100/5448
[01:19:17] ERROR 2019-03-12T02:10:35Z: compiletest::runtest: fatal error, panic: "aux-build `/checkout/src/test/ui/issues/auxiliary/use_from_trait_xc.rs` source not found"
[01:19:20] .......................................F............................................................ 2200/5448
[01:19:29] .................................................................................................... 2400/5448
[01:19:34] .................................................................................................... 2500/5448
[01:19:37] .................................................................................................... 2600/5448
[01:19:42] .................................................................................................... 2700/5448
[01:19:42] .................................................................................................... 2700/5448
[01:19:47] .................................................................................................... 2800/5448
[01:19:51] .................................................................................................... 2900/5448
[01:19:56] ...................................F................................................................ 3000/5448
[01:20:05] .................................................................................................... 3200/5448
[01:20:09] ..........................................................................i......................... 3300/5448
[01:20:13] .................................................................................................... 3400/5448
[01:20:17] ................................................ii...i..ii.......................................... 3500/5448
[01:20:17] ................................................ii...i..ii.......................................... 3500/5448
[01:20:22] .................................................................................................... 3600/5448
[01:20:26] .................................................................................................... 3700/5448
[01:20:30] ..........................................................ii........................................ 3800/5448
[01:20:32] ............................................................................i....................... 3900/5448
[01:20:35] .................................................................................................... 4000/5448
[01:20:37] ..................................i................................................................. 4100/5448
[01:20:41] .................................................................................................... 4200/5448
[01:20:47] ERROR 2019-03-12T02:12:05Z: compiletest::runtest: fatal error, panic: "aux-build `/checkout/src/test/ui/proc-macro/auxiliary/issue_38586.rs` source not found"
[01:20:48] ERROR 2019-03-12T02:12:06Z: compiletest::runtest: fatal error, panic: "aux-build `/checkout/src/test/ui/proc-macro/auxiliary/issue_50493.rs` source not found"
[01:20:54] ...............................................................F.F.................................. 4300/5448
[01:21:01] .................................................................................................... 4500/5448
[01:21:01] .................................................................................................... 4500/5448
[01:21:02] ERROR 2019-03-12T02:12:20Z: compiletest::runtest: fatal error, panic: "aux-build `/checkout/src/test/ui/resolve/auxiliary/issue_19452_aux.rs` source not found"
[01:21:02] ERROR 2019-03-12T02:12:20Z: compiletest::runtest: fatal error, panic: "aux-build `/checkout/src/test/ui/resolve/auxiliary/issue_3907.rs` source not found"
[01:21:02] ERROR 2019-03-12T02:12:20Z: compiletest::runtest: fatal error, panic: "aux-build `/checkout/src/test/ui/resolve/auxiliary/issue_3907.rs` source not found"
[01:21:05] ........F..........FF........................................................................i...... 4600/5448
[01:21:17] .................................................................................................... 4800/5448
[01:21:20] .................................................................................................... 4900/5448
[01:21:25] .................................................................................................... 5000/5448
[01:21:29] .................................................................................................... 5100/5448
---
[01:21:41] failures:
[01:21:41] 
[01:21:41] ---- [ui] ui/e0119/issue-23563.rs stdout ----
[01:21:41] 
[01:21:41] error: aux-build `/checkout/src/test/ui/e0119/auxiliary/issue_23563_a.rs` source not found
[01:21:41] thread '[ui] ui/e0119/issue-23563.rs' panicked at 'fatal error', src/tools/compiletest/src/runtest.rs:2040:9
[01:21:41] 
[01:21:41] ---- [ui] ui/issues/issue-18986.rs stdout ----
[01:21:41] 
[01:21:41] 
[01:21:41] error: aux-build `/checkout/src/test/ui/issues/auxiliary/use_from_trait_xc.rs` source not found
[01:21:41] thread '[ui] ui/issues/issue-18986.rs' panicked at 'fatal error', src/tools/compiletest/src/runtest.rs:2040:9
[01:21:41] ---- [ui] ui/issues/issue-56411.rs stdout ----
[01:21:41] diff of stderr:
[01:21:41] 
[01:21:41] 
[01:21:41] - error[E0255]: the name `issue_56411_aux` is defined multiple times
[01:21:41] -   --> $DIR/issue-56411.rs:5:21
[01:21:41] + error[E0583]: file not found for module `issue_56411_aux`
[01:21:41] 3    |
[01:21:41] 3    |
[01:21:41] - LL |             mod $name;
[01:21:41] -    |             ---------- previous definition of the module `issue_56411_aux` here
[01:21:41] - LL |             pub use self::$name;
[01:21:41] -    |                     |
[01:21:41] -    |                     |
[01:21:41] -    |                     `issue_56411_aux` reimported here
[01:21:41] - ...
[01:21:41] - ...
[01:21:41] 12 LL | import!(issue_56411_aux);
[01:21:41] -    | ------------------------- in this macro invocation
[01:21:41] 14    |
[01:21:41] 14    |
[01:21:41] -    = note: `issue_56411_aux` must be defined only once in the type namespace of this module
[01:21:41] +    = help: name the file either issue_56411_aux.rs or issue_56411_aux/mod.rs inside the directory "$DIR"
[01:21:41] 16 
[01:21:41] - error[E0365]: `issue_56411_aux` is private, and cannot be re-exported
[01:21:41] -    |
[01:21:41] -    |
[01:21:41] - LL |             pub use self::$name;
[01:21:41] -    |                     ^^^^^^^^^^^ re-export of private `issue_56411_aux`
[01:21:41] - ...
[01:21:41] - LL | import!(issue_56411_aux);
[01:21:41] -    | ------------------------- in this macro invocation
[01:21:41] -    |
[01:21:41] -    = note: consider declaring type or module `issue_56411_aux` with `pub`
[01:21:41] + error: aborting due to previous error
[01:21:41] - error: aborting due to 2 previous errors
[01:21:41] - 
[01:21:41] - Some errors occurred: E0255, E0365.
[01:21:41] - For more information about an error, try `rustc --explain E0255`.
[01:21:41] - For more information about an error, try `rustc --explain E0255`.
[01:21:41] + For more information about this error, try `rustc --explain E0583`.
[01:21:41] 32 
[01:21:41] 
[01:21:41] 
[01:21:41] The actual stderr differed from the expected stderr.
[01:21:41] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-56411/issue-56411.stderr
[01:21:41] To update references, rerun the tests and pass the `--bless` flag
[01:21:41] To only update this specific test, also pass `--test-args issues/issue-56411.rs`
[01:21:41] error: 1 errors occurred comparing output.
[01:21:41] status: exit code: 1
[01:21:41] status: exit code: 1
[01:21:41] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-56411.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-56411/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-56411/auxiliary" "-A" "unused"
[01:21:41] ------------------------------------------
[01:21:41] 
[01:21:41] ------------------------------------------
[01:21:41] stderr:
[01:21:41] stderr:
[01:21:41] ------------------------------------------
[01:21:41] {"message":"file not found for module `issue_56411_aux`","code":{"code":"E0583","explanation":"\nA file wasn't found for an out-of-line module.\n\nErroneous code example:\n\n