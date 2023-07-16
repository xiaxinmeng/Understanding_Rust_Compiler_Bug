plain
travis_time:end:1a989a40:start=1543159432545645597,finish=1543159435126349343,duration=2580703746
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:52:48] .................................................................................................... 100/5055
[00:52:51] .................................................................................................... 200/5055
[00:52:53] .............................ii............................................ii...................ii.. 300/5055
[00:52:56] ..............................................................................................iii... 400/5055
[00:52:59] .....iiiiiiii.iii............................iii...........................................i........ 500/5055
[00:53:06] .................................................................................................... 700/5055
[00:53:13] .....................................................................................i...........i.. 800/5055
[00:53:16] .................................................................................................... 900/5055
[00:53:20] ....iiiii..................ii.iiii.................................................................. 1000/5055
---
[01:02:38] .................................................................................................... 300/2886
[01:02:49] .................................................................................................... 400/2886
[01:02:58] .................................................................................................... 500/2886
[01:03:10] .................................................................................................... 600/2886
[01:03:26] ................F.FF................................................................................ 700/2886
[01:03:47] .................................................................................................... 900/2886
[01:04:02] .................................................................................................... 1000/2886
[01:04:16] .................................................................................................... 1100/2886
[01:04:24] .................................................................................................... 1200/2886
---
[01:08:40] failures:
[01:08:40] 
[01:08:40] ---- [run-pass (nll)] run-pass/drop/dropck-eyepatch-extern-crate.rs stdout ----
[01:08:40] 
[01:08:40] error: test compilation failed although it shouldn't!
[01:08:40] status: exit code: 101
[01:08:40] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/drop/dropck-eyepatch-extern-crate.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/drop/dropck-eyepatch-extern-crate.nll/a" "-Zborrowck=migrate" "-Ztwo-phase-borrows" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/drop/dropck-eyepatch-extern-crate.nll/auxiliary"
[01:08:40] ------------------------------------------
[01:08:40] 
[01:08:40] ------------------------------------------
[01:08:40] stderr:
[01:08:40] stderr:
[01:08:40] ------------------------------------------
[01:08:40] {"message":"unresolved inference variable in outlives: _#1t","code":null,"level":"error: internal compiler error","spans":[{"file_name":"/checkout/src/test/run-pass/drop/dropck-eyepatch-extern-crate.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"// Copyright 2016 The Rust Project Developers. See the COPYRIGHT","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: internal compiler error: unresolved inference variable in outlives: _#1t\n\n"}
[01:08:40] {"message":"unresolved inference variable in outlives: _#4t","code":null,"level":"error: internal compiler error","spans":[{"file_name":"/checkout/src/test/run-pass/drop/dropck-eyepatch-extern-crate.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"// Copyright 2016 The Rust Project Developers. See the COPYRIGHT","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: internal compiler error: unresolved inference variable in outlives: _#4t\n\n"}
[01:08:40] {"message":"unresolved inference variable in outlives: _#6t","code":null,"level":"error: internal compiler error","spans":[{"file_name":"/checkout/src/test/run-pass/drop/dropck-eyepatch-extern-crate.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"// Copyright 2016 The Rust Project Developers. See the COPYRIGHT","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: internal compiler error: unresolved inference variable in outlives: _#6t\n\n"}
[01:08:40] {"message":"unresolved inference variable in outlives: _#8t","code":null,"level":"error: internal compiler error","spans":[{"file_name":"/checkout/src/test/run-pass/drop/dropck-eyepatch-extern-crate.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"// Copyright 2016 The Rust Project Developers. See the COPYRIGHT","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: internal compiler error: unresolved inference variable in outlives: _#8t\n\n"}
[01:08:40] {"message":"unresolved inference variable in outlives: _#11t","code":null,"level":"error: internal compiler error","spans":[{"file_name":"/checkout/src/test/run-pass/drop/dropck-eyepatch-extern-crate.rs","byte_start":0,mpiler error: unresolved inference variable in outlives: _#18t\n\n"}
[01:08:40] {"message":"unresolved inference variable in outlives: _#20t","code":null,"level":"error: internal compiler error","spans":[{"file_name":"/checkout/src/test/run-pass/drop/dropck-eyepatch-extern-crate.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"// Copyright 2016 The Rust Project Developers. See the COPYRIGHT","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: internal compiler error: unresolved inference variable in outlives: _#20t\n\n"}
[01:08:40] {"message":"unresolved inference variable in outlives: _#22t","code":null,"level":"error: internal compiler error","spans":[{"file_name":"/checkout/src/test/run-pass/drop/dropck-eyepatch-extern-crate.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"// Copyright 2016 The Rust Project Developers. See the COPYRIGHT","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: internal compiler error: unresolved inference variable in outlives: _#22t\n\n"}
[01:08:40] {"message":"unresolved inference variable in outlives: _#25t","code":null,"level":"error: internal compiler error","spans":[{"file_name":"/checkout/src/test/run-pass/drop/dropck-eyepatch-extern-crate.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_stext":"// Copyright 2016 The Rust Project Developers. See the COPYRIGHT","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: internal compiler error: unresolved inference variable in outlives: _#39t\n\n"}
[01:08:40] {"message":"unresolved inference variable in outlives: _#41t","code":null,"level":"error: internal compiler error","spans":[{"file_name":"/checkout/src/test/run-pass/drop/dropck-eyepatch-extern-crate.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"// Copyright 2016 The Rust Project Developers. See the COPYRIGHT","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: internal compiler error: unresolved inference variable in outlives: _#41t\n\n"}
[01:08:40] {"message":"unresolved inference variable in outlives: _#43t","code":null,"level":"error: internal compiler error","spans":[{"file_name":"/checkout/src/test/run-pass/drop/dropck-eyepatch-extern-crate.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"// Copyright 2016 The Rust Project Developers. See the COPYRIGHT","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: internal compiler error: unresolved inference variable in outlives: _#43t\n\n"}
[01:08:40] {"message":"unresolved inference variable in outlives: _#46t","code":null,"level":"error: internal compiler error","spans":[{"file_name":"/checkout/src/test/run-pass/drop/dropck-eyepatch-extern-crate.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"// Copyright 2016 The Rust Project Developers. See the COPYRIGHT","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: internal compiler error: unresolved inference variable in outlives: _#46t\n\n"}
[01:08:40] thread 'main' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:334:17
[01:08:40] 
[01:08:40] error: internal compiler error: unexpected panic
[01:08:40] 
[01:08:40] note: the compiler unexpectedly panicked. this is a bug.
[01:08:40] note: the compiler unexpectedly panicked. this is a bug.
[01:08:40] 
[01:08:40] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:08:40] 
[01:08:40] note: rustc 1.32.0-dev running on x86_64-unknown-linux-gnu
[01:08:40] 
[01:08:40] note: compiler flags: -Z ui-testing -Z borrowck=migrate -Z two-phase-borrows -Z unstable-options -C prefer-dynamic -C rpath
[01:08:40] 
[01:08:40] ------------------------------------------
[01:08:40] 
[01:08:40] thread '[run-pass (nll)] run-pass/drop/dropck-eyepatch-extern-crate.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[01:08:40] thread '[run-pass (nll)] run-pass/drop/dropck-eyepatch-extern-crate.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[01:08:40] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:08:40] 
[01:08:40] ---- [run-pass (nll)] run-pass/drop/dropck-eyepatch-reorder.rs stdout ----
[01:08:40] 
[01:08:40] error: test compilation failed although it shouldn't!
[01:08:40] status: exit code: 101
[01:08:40] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/drop/dropck-eyepatch-reorder.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/drop/dropck-eyepatch-reorder.nll/a" "-Zborrowck=migrate" "-Ztwo-phase-borrows" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/drop/dropck-eyepatch-reorder.nll/auxiliary"
[01:08:40] ------------------------------------------
[01:08:40] 
[01:08:40] ------------------------------------------
[01:08:40] stderr:
[01:08:40] stderr:
[01:08:40] ------------------------------------------
[01:08:40] {"message":"unresolved inference variable in outlives: _#1t","code":null,"level":"error: internal compiler error","spans":[{"file_name":"/checkout/src/test/run-pass/drop/dropck-eyepatch-reorder.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"// Copyright 2016 The Rust Project Developers. See the COPYRIGHT","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: internal compiler error: unresolved inighlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: internal compiler error: unresolved inference variable in outlives: _#22t\n\n"}
[01:08:40] {"message":"unresolved inference variable in outlives: _#25t","code":null,"level":"error: internal compiler error","spans":[{"file_name":"/checkout/src/test/run-pass/drop/dropck-eyepatch-reorder.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"// Copyright 2016 The Rust Project Developers. See the COPYRIGHT","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: internal compiler error: unresolved inference variable in outlives: _#25t\n\n"}
[01:08:40] {"message":"unresolved inference variable in outlives: _#27t","code":null,"level":"error: internal compiler error","spans":[{"file_name":"/checkout/src/test/run-pass/drop/dropck-eyepatch-reorder.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"// Copyright 2016 The Rust Project Developers. See the COPYRIGHT","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: internal compiler error: unresolved inference variable in outlives: _#27t\n\n"}
[01:08:40] {"message":"unresolved inference variable in outlives: _#29t","code":null,"level":"error: internal compilull,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: internal compiler error: unresolved inference variable in outlives: _#36t\n\n"}
[01:08:40] {"message":"unresolved inference variable in outlives: _#39t","code":null,"level":"error: internal compiler error","spans":[{"file_name":"/checkout/src/test/run-pass/drop/dropck-eyepatch-reorder.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"// Copyright 2016 The Rust Project Developers. See the COPYRIGHT","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: internal compiler error: unresolved inference variable in outlives: _#39t\n\n"}
[01:08:40] {"message":"unresolved inference variable in outlives: _#41t","code":null,"level":"error: internal compiler error","spans":[{"file_name":"/checkout/src/test/run-pass/drop/dropck-eyepatch-reorder.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"// Copyright 2016 The Rust Project Developers. See the COPYRIGHT","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: internal compiler error: unresolved inference variable in outlives: _#41t\n\n"}
[01:08:40] {"message":"unresolved inference variable in outlives: _#43t","code":null,"level":"error: internal compiler error","spans":[{"file_name":"/checkout/src/test/run-pass/drop/dropck-eye64-unknown-linux-gnu
[01:08:40] 
[01:08:40] note: compiler flags: -Z ui-testing -Z borrowck=migrate -Z two-phase-borrows -Z unstable-options -C prefer-dynamic -C rpath
[01:08:40] 
[01:08:40] ------------------------------------------
[01:08:40] 
[01:08:40] thread '[run-pass (nll)] run-pass/drop/dropck-eyepatch-reorder.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[01:08:40] thread '[run-pass (nll)] run-pass/drop/dropck-eyepatch-reorder.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[01:08:40] 
[01:08:40] ---- [run-pass (nll)] run-pass/drop/dropck-eyepatch.rs stdout ----
[01:08:40] 
[01:08:40] error: test compilation failed although it shouldn't!
[01:08:40] status: exit code: 101
[01:08:40] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/drop/dropck-eyepatch.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/drop/dropck-eyepatch.nll/a" "-Zborrowck=migrate" "-Ztwo-phase-borrows" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/drop/dropck-eyepatch.nll/auxiliary"
[01:08:40] ------------------------------------------
[01:08:40] 
[01:08:40] ------------------------------------------
[01:08:40] stderr:
[01:08:40] stderr:
[01:08:40] ------------------------------------------
[01:08:40] {"message":"unresolved inference variable in outlives: _#1t","code":null,"level":"error: internal compiler error","spans":[{"file_name":"/checkout/src/test/run-pass/drop/dropck-eyepatch.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"// Copyright 2016 The Rust Project Developers. See the COPYRIGHT","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: internal compiler error: unresolved inference variable in outlives: _#1t\n\n"}
[01:08:40] {"message":"unresolved inference variable in outlives: _#4t","code":null,"level":"error: internal compiler error","spans":[{"file_name":"/checkout/src/test/run-pass/drop/dropck-eyepatch.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"// Copyright 2016 The Rust Project Developers. See the COPYRIGHT","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: internal compiler error: unresolved inference variable in outlives: _#4t\n\n"}
[01:08:40] {"message":"unresolved inference variable in outlives: _#6t","code":null,"level":"error: internal compiler error","spans":[{"file_name":"/checkout/src/test/run-pass/drop/dropck-eyepatch.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"// Copyright 2016 The Rust Project Developers. See the COPYRIGHT","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: internal compiler error: unresolved inference variable in outlives: _#6t\n\n"}
[01:08:40] {"message":"unresolved inference variable in outlives: _#8t","code":null,"level":"error: internal compiler error","spans":[{"file_name":"/checkout/src/test/run-pass/drop/dropck-eyepatch.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"// Copyright 2016 The Rust Project Developers. See the COPYRIGHT","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: internal compiler error: unresolved inference variable in outlives: _#8t\n\n"}
[01:08:40] {"message":"unresolved inference variable in outlives: _#11t","code":null,"level":"error: internal compiler error","spans":[{"file_name":"/checkout/src/test/run-pass/drop/dropck-eyepatch.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"// Copyright 2016 The Rust Project Developers. See the COPYRIGHT","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: internal compiler error: unresolved inference variable in outlives: _#11t\n\n"}
[01:08:40] {"message":"unresolved inference variable in outlives: _#13t","code":null,"level":"error: internal compiler error","spans":[{"file_name":"/checkout/src/test/run-pass/drop/dropck-eyepatch.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"// Copyright 2016 Thement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: internal compiler error: unresolved inference variable in outlives: _#27t\n\n"}
[01:08:40] {"message":"unresolved inference variable in outlives: _#29t","code":null,"level":"error: internal compiler error","spans":[{"file_name":"/checkout/src/test/run-pass/drop/dropck-eyepatch.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"// Copyright 2016 The Rust Project Developers. See the COPYRIGHT","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: internal compiler error: unresolved inference variable in outlives: _#29t\n\n"}
[01:08:40] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:503:22
[01:08:40] {"message":"unresolved inference variable in outlives: _#34t","code":null,"level":"error: internal compiler error","spans":[{"file_name":"/checkout/src/test/run-pass/drop/dropck-eyepatch.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"// Copyright 2016 The Rust Project Developers. See the COPYRIGHT","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: internal compiler error: unresolved inference variable in outlives: _#34t\n\n"}
[01:08:40] {"message":"unresolved inference variable in outlives: _#36t","code":null,"level":"error:gestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: internal compiler error: unresolved inference variable in outlives: _#41t\n\n"}
[01:08:40] {"message":"unresolved inference variable in outlives: _#43t","code":null,"level":"error: internal compiler error","spans":[{"file_name":"/checkout/src/test/run-pass/drop/dropck-eyepatch.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"// Copyright 2016 The Rust Project Developers. See the COPYRIGHT","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: internal compiler error: unresolved inference variable in outlives: _#43t\n\n"}
[01:08:40] {"message":"unresolved inference variable in outlives: _#46t","code":null,"level":"error: internal compiler error","spans":[{"file_name":"/checkout/src/test/run-pass/drop/dropck-eyepatch.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"// Copyright 2016 The Rust Project Developers. See the COPYRIGHT","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: internal compiler error: unresolved inference variable in outlives: _#46t\n\n"}
[01:08:40] thread 'main' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:334:17
[01:08:40] 
[01:08:3824152 .
2532492 ./obj
2507008 ./obj/build
---
140912 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu
140908 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release
137964 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps
134660 ./obj/build/bootstrap/debug/incremental/bootstrap-1plb86h2refwc
134656 ./obj/build/bootstrap/debug/incremental/bootstrap-1plb86h2refwc/s-f705ploqqp-6pswaw-3bcgkskzxpex9
130764 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release
127444 ./.git/modules
127440 ./.git/modules/src
123692 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps
