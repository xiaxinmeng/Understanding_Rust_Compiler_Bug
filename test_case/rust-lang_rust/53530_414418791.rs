plain
[00:02:09] Successfully tagged rust-ci:latest
[00:02:09] Built container sha256:62a414b53655fac5e0afb30fcd2057090b2dc68ffc6cb2cc2732b30823cf2999
[00:02:09] Uploading finished image to s3://rust-lang-ci-sccache2/docker/a08f738800c5d4bf390acb73ec0118b6ff1146863e6648bef20b9a0326bcba9bd46d442bc30b69fac421c987dce13ae565bdfc6c2fccdb7c32851f08d351168d
[00:02:09] 
[00:02:09] Partial credentials found in env, missing: AWS_SECRET_ACCESS_KEY
[00:02:14] xargs: docker: terminated by signal 13

[00:02:14] travis_time:end:219eb154:start=1534787385211093754,finish=1534787453156524923,duration=67945431169
[CI_JOB_NAME=x86_64-gnu-llvm-5.0]
[00:02:14] [CI_JOB_NAME=x86_64-gnu-llvm-5.0]
---
[00:45:08] ....................................................................................................
[00:45:11] ....................................................................................................
[00:45:14] ....................................................................................................
[00:45:17] .............i......................................................................................
[00:45:21] .......................................................................................F...........i
[00:45:27] .............ii.iii.................................................................................
[00:45:30] ....................................................................................................
[00:45:32] ....................................................................................................
[00:45:34] ....................................................................................................
---
[00:46:58] ---- [ui] ui/consts/const-size_of-cycle.rs stdout ----
[00:46:58] diff of stderr:
[00:46:58] 
[00:46:58] 2    |
[00:46:58] 3 note: ...which requires normalizing `ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: All }, value: [u8; _] }`...
[00:46:58] 4 note: ...which requires const-evaluating `Foo::bytes::{{constant}}`...
[00:46:58] -   --> $SRC_DIR/libcore/mem.rs:323:14
[00:46:58] +   --> $SRC_DIR/libcore/mem.rs:291:14
[00:46:58] 6    |
[00:46:58] 7 LL |     unsafe { intrinsics::size_of::<T>() }
[00:46:58] 
[00:46:58] 
[00:46:58] 9    = note: ...which again requires computing layout of `Foo`, completing the cycle
[00:46:58] 10 note: cycle used when const-evaluating `Foo::bytes::{{constant}}`
[00:46:58] -   --> $SRC_DIR/libcore/mem.rs:323:14
[00:46:58] +   --> $SRC_DIR/libcore/mem.rs:291:14
[00:46:58] 12    |
[00:46:58] 13 LL |     unsafe { intrinsics::size_of::<T>() }
[00:46:58] 
[00:46:58] 
[00:46:58] The actual stderr differed from the expected stderr.
[00:46:58] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-size_of-cycle/const-size_of-cycle.stderr
[00:46:58] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-size_of-cycle/const-size_of-cycle.stderr
[00:46:58] To update references, rerun the tests and pass the `--bless` flag
[00:46:58] To only update this specific test, also pass `--test-args consts/const-size_of-cycle.rs`
[00:46:58] error: 1 errors occurred comparing output.
[00:46:58] status: exit code: 1
[00:46:58] status: exit code: 1
[00:46:58] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-size_of-cycle.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-size_of-cycle/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-size_of-cycle/auxiliary" "-A" "unused"
[00:46:58] ------------------------------------------
[00:46:58] 
[00:46:58] ------------------------------------------
[00:46:58] stderr:
[00:46:58] stderr:
[00:46:58] ------------------------------------------
[00:46:58] {"message":"cycle detected when computing layout of `Foo`","code":{"code":"E0391","explanation":"\nThis error indicates that some types or traits depend on each other\nand therefore cannot be constructed.\n\nThe following example contains a circular dependency between two traits:\n\n