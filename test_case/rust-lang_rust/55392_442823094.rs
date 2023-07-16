plain
travis_time:end:23bf3efe:start=1543493064279087024,finish=1543493119398660625,duration=55119573601
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:46:49] .................................................................................................... 2300/5063
[00:46:53] .................................................................................................... 2400/5063
[00:46:57] .................................................................................................... 2500/5063
[00:47:00] .................................................................................................... 2600/5063
[00:47:04] .........................................................F.......................................... 2700/5063
[00:47:08] .......................................F............................................................ 2800/5063
[00:47:14] .................................................................................................... 3000/5063
[00:47:17] .....................................................................i.............................. 3100/5063
[00:47:17] .....................................................................i.............................. 3100/5063
[00:47:20] .....................................F.F............................................................ 3200/5063
[00:47:27] .................................................................................................... 3400/5063
[00:47:30] .................................................................................................... 3500/5063
[00:47:33] .............ii..................................................................................... 3600/5063
[00:47:35] ................................i................................................................... 3700/5063
---
[00:48:15] failures:
[00:48:15] 
[00:48:15] ---- [ui] ui/issues/issue-46964.rs stdout ----
[00:48:15] 
[00:48:15] error: test compilation failed although it shouldn't!
[00:48:15] status: exit code: 101
[00:48:15] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-46964.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-46964/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-46964/auxiliary" "-A" "unused"
[00:48:15] ------------------------------------------
[00:48:15] 
[00:48:15] ------------------------------------------
[00:48:15] ------------------------------------------
[00:           0,\n                0\n            ],\n            relocations: Relocations(\n                SortedMap {\n                    data: [\n                        (\n                            Size {\n                                raw: 0\n                            },\n                            (\n                                (),\n                                AllocId(\n                                    0\n                                )\n                            )\n                        )\n                    ]\n                }\n            ),\n            undef_mask: UndefMask {\n                blocks: [\n                    65535\n                ],\n                len: Size {\n                    raw: 16\n                }\n            },\n            align: Align {\n                pow2: 3\n            },\n            mutability: Mutable,\n            extra: ()\n        },\n        Size {\n            raw: 0\n        }\n    )\n}\n\n"}
[00:48:15] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:48:15] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:48:15] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:48:15] note: the compiler unexpectedly panicked. this is a bug.
[00:48:15] 
[00:48:15] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:48:15] 
[00:48:15] 
[00:48:15] note: rustc 1.32.0-dev running on x86_64-unknown-linux-gnu
[00:48:15] 
[00:48:15] note: compiler flags: -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[00:48:15] 
[00:48:15] ------------------------------------------
[00:48:15] 
[00:48:15] thread '[ui] ui/issues/issue-46964.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:48:15] thread '[ui] ui/issues/issue-46964.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[00:48:15] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:48:15] 
[00:48:15] ---- [ui] ui/issues/issue-51655.rs stdout ----
[00:48:15] 
[00:48:15] error: test compilation failed although it shouldn't!
[00:48:15] status: exit code: 101
[00:48:15] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-51655.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-51655/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-51655/auxiliary" "-A" "unused"
[00:48:15] ------------------------------------------
[00:48:15] 
[00:48:15] ------------------------------------------
[00:48:15] stderr:
[00:48:15] stderr:
[00:48:15] ------------------------------------------
[00:48:15] {"message":"src/librustc_mir/hair/pattern/_match.rs:1395: unexpected ConstValue: Const {\n    ty: &[u8],\n    val: ByRef(\n        AllocId(\n            4\n        ),\n        Allocation {\n            bytes: [\n                0,\n                0,\n                0,\n                0,\n                0,\n                0,\n                0,\n                0              0,\n                0,\n                0,\n                0,\n                0\n            ],\n            relocations: Relocations(\n                SortedMap {\n                    data: [\n                        (\n                            Size {\n                                raw: 0\n                            },\n                            (\n                                (),\n                                AllocId(\n                                    2\n                                )\n                            )\n                        )\n                    ]\n                }\n            ),\n            undef_mask: UndefMask {\n                blocks: [\n                    65535\n                ],\n                len: Size {\n                    raw: 16\n                }\n            },\n            align: Align {\n                pow2: 3\n            },\n            mutability: Mutable,\n            extra: ()\n        },\n        Size {\n            raw: 0\n        }\n    )\n}\n\n"}
[00:48:15] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:48:15] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:48:15] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:48:15] note: the compiler unexpectedly panicked. this is a bug.
[00:48:15] 
[00:48:15] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:48:15] 
[00:48:15] 
[00:48:15] note: rustc 1.32.0-dev running on x86_64-unknowr types with a small number of variants\n(like enums) you should probably cover all cases explicitly. Alternatively, the\nunderscore `_` wildcard pattern can be added after all other patterns to match\n\"anything else\". Example:\n\n