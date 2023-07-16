plain
travis_time:end:23a44ba0:start=1545332293688084218,finish=1545332296187319194,duration=2499234976
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:56:34] .................................................................................................... 400/5192
[00:56:38] .................................................................................................... 500/5192
[00:56:41] ...............................i.................................................................... 600/5192
[00:56:44] .................................................................................................... 700/5192
[00:56:49] .....................................................F.............................................. 800/5192
[00:56:56] ...............................iiiii................................................................ 1000/5192
[00:56:59] .................................................................................................... 1100/5192
[00:57:02] .................................................................................................... 1200/5192
[00:57:04] .................................................................................................... 1300/5192
---
[00:57:35] .................................................................................................... 2300/5192
[00:57:39] .................................................................................................... 2400/5192
[00:57:42] .................................................................................................... 2500/5192
[00:57:46] .................................................................................................... 2600/5192
[00:57:50] ................F................................................................................... 2700/5192
[00:57:56] .................................................................................................... 2900/5192
[00:58:00] .................................................................................................... 3000/5192
[00:58:03] ..............................................................................................i..... 3100/5192
[00:58:06] .................................................................................................... 3200/5192
---
[00:59:01] .................................................................................................... 4800/5192
[00:59:04] .................................................................................................... 4900/5192
[00:59:07] .................................................................................................... 5000/5192
[00:59:10] .................................................................................................... 5100/5192
code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0391`.\n"}
[00:59:12] ------------------------------------------
[00:59:12] 
[00:59:12] thread '[ui] ui/consts/const-size_of-cycle.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3255:9
[00:59:12] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:59:12] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:59:12] 
[00:59:12] ---- [ui] ui/issues/issue-44415.rs stdout ----
[00:59:12] diff of stderr:
[00:59:12] 
[00:59:12] 10 LL |     bytes: [u8; unsafe { intrinsics::size_of::<Foo>() }],
[00:59:12] 11    |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:59:12] 12 note: ...which requires computing layout of `Foo`...
[00:59:12] - note: ...which requires normalizing `ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: All }, value: [u8; _] }`...
[00:59:12] + note: ...which requires normalizing `ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: All, def_id: None }, value: [u8; _] }`...
[00:59:12] 14 note: ...which requires const-evaluating + checking `Foo::bytes::{{constant}}`...
[00:59:12] 16    |
[00:59:12] 
[00:59:12] 
[00:59:12] The actual stderr differed from the expected stderr.
[00:59:12] The actual stderr differed from the expected stderr.
[00:59:12] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-44415/issue-44415.stderr
[00:59:12] To update references, rerun the tests and pass the `--bless` flag
[00:59:12] To only update this specific test, also pass `--test-args issues/issue-44415.rs`
[00:59:12] 
[00:59:12] error: 1 errors occurred comparing ou const-evaluating + checking `Foo::bytes::{{constant}}`...","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-44415.rs","byte_start":548,"byte_end":554,"line_start":16,"line_end":16,"column_start":17,"column_end":23,"is_primary":true,"text":[{"text":"    bytes: [u8; unsafe { intrinsics::size_of::<Foo>() }],","highlight_start":17,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"...which again requires const-evaluating + checking `Foo::bytes::{{constant}}`, completing the cycle","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"cycle used when processing `Foo`","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-44415.rs","byte_start":519,"byte_end":529,"line_start":15,"line_end":15,"column_start":1,"column_end":11,"is_primary":true,"text":[{"text":"struct Foo {","highlight_start":1,"highlight_end":11}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0391]: cycle detected when const-evaluating + checking `Foo::bytes::{{constant}}`\n  --> /checkout/src/test/ui/issues/issue-44415.rs:16:17\n   |\nLL |     bytes: [u8; unsafe { intrinsics::size_of::<Foo>() }],\n   |                 ^^^^^^\n   |\nnote: ...which requires const-evaluating `Foo::bytes::{{constant}}`...\n  --> /checkout/src/test/ui/issues/issue-44415.rs:16:26\n   |\nLL |     bytes: [u8; unsafe { intrinsics::size_of::<Foo>() }],\n   |                          ^^^^^^^^^^^^^^^travis_time:end:212f30f2:start=1545332304680113240,finish=1545335857401528064,duration=3552721414824
travis_time:start:050346d2
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Dec 20 19:57:37 UTC 2018
Thu, 20 Dec 2018 19:57:37 GMT
