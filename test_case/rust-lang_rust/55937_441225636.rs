plain
travis_time:end:06947127:start=1542970668331512763,finish=1542970731003464517,duration=62671951754
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:54:50] .................................................................................................... 100/5050
[00:54:54] .................................................................................................... 200/5050
[00:54:56] .............................ii............................................ii...................ii.. 300/5050
[00:54:59] ..............................................................................................iii... 400/5050
[00:55:02] .....iiiiiiii.iii............................iii...........................................i........ 500/5050
[00:55:09] .................................................................................................... 700/5050
[00:55:16] ...................................................................................i...........i.... 800/5050
[00:55:19] .................................................................................................... 900/5050
[00:55:23] ..iiiii..................ii.iiii.................................................................... 1000/5050
---
[00:55:59] .................................................................................................... 2200/5050
[00:56:03] .................................................................................................... 2300/5050
[00:56:07] .................................................................................................... 2400/5050
[00:56:11] .................................................................................................... 2500/5050
[00:56:15] ............................................................................................iiiiiiii 2600/5050
[00:56:22] ..........................................................ii........................................ 2800/5050
[00:56:25] .................................................................................................... 2900/5050
[00:56:29] .................................................................................................... 3000/5050
[00:56:32] .......................................................i............................................ 3100/5050
---
[01:04:37] .................................................................................................... 300/2886
[01:04:48] .................................................................................................... 400/2886
[01:04:57] .................................................................................................... 500/2886
[01:05:09] .................................................................................................... 600/2886
[01:05:26] ................F.FF................................................................................ 700/2886
[01:05:47] .................................................................................................... 900/2886
[01:06:03] .................................................................................................... 1000/2886
[01:06:16] .................................................................................................... 1100/2886
[01:06:25] .................................................................................................... 1200/2886
---
[01:10:38] failures:
[01:10:38] 
[01:10:38] ---- [run-pass (nll)] run-pass/drop/dropck-eyepatch-extern-crate.rs stdout ----
[01:10:38] 
[01:10:38] error: test compilation failed although it shouldn't!
[01:10:38] status: exit code: 101
[01:10:38] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/drop/dropck-eyepatch-extern-crate.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/drop/dropck-eyepatch-extern-crate.nll/a" "-Zborrowck=mion":null}],"children":[],"rendered":"error: internal compiler error: unresolved inference variable in outlives: _#4t\n\n"}
[01:10:38] {"message":"unresolved inference variable in outlives: _#6t","code":null,"level":"error: internal compiler error","spans":[{"file_name":"/checkout/src/test/run-pass/drop/dropck-eyepatch-extern-crate.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"// Copyright 2016 The Rust Project Developers. See the COPYRIGHT","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: internal compiler error: unresolved inference variable in outlives: _#6t\n\n"}
[01:10:38] {"message":"unresolved inference variable in outlives: _#8t","code":null,"level":"error: internal compiler error","spans":[{"file_name":"/checkout/src/test/run-pass/drop/dropck-eyepatch-extern-crate.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"// Copyright 2016 The Rust Project Developers. See the COPYRIGHT","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: internal compiler error: unresolved inference variable in outlives: _#8t\n\n"}
[01:10:38] {"message":"unresolved inference variable in outlives: _#11t","code":null,"level":"error: internal compiler error","spans":[{"file_name":"/checkout/src/test/run-pass/drop/dropck-eyepatch-extern-crate.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"// Copyright 2016 The Rust Project Developers. See the COPYRIGHT","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: internal compiler error: unresolved inference variable in outlives: _#11t\n\n"}
[01:10:38] {"message":"unresolved inference variable in outlives: _#13t","code":null,"level":"error: internal compiler error","spans":[{"file_name":"/checkout/src/test/run-pass/drop/dropck-eyepatch-extern-crate.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"// Copyright 2016 The Rust Project Developers. See the COPYRIGHT","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: internal compiler error: unresolved inference variable in outlives: _#13t\n\n"}
[01:10:38] {"message":"unresolved inference variable in outlives: _#18t","code":null,"level":"error: internal compiler error","spans":[{"file_name":"/checkout/src/test/run-pass/drop/dropck-eyepatch-extern-crate.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"// Copyright 2016 The Rust Project Developers. See the COPYRIGHT","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: internal coives: _#29t\n\n"}
[01:10:38] {"message":"unresolved inference variable in outlives: _#34t","code":null,"level":"error: internal compiler error","spans":[{"file_name":"/checkout/src/test/run-pass/drop/dropck-eyepatch-extern-crate.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"// Copyright 2016 The Rust Project Developers. See the COPYRIGHT","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: internal compiler error: unresolved inference variable in outlives: _#34t\n\n"}
[01:10:38] {"message":"unresolved inference variable in outlives: _#36t","code":null,"level":"error: internal compiler error","spans":[{"file_name":"/checkout/src/test/run-pass/drop/dropck-eyepatch-extern-crate.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"// Copyright 2016 The Rust Project Developers. See the COPYRIGHT","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: internal compiler error: unresolved inference variable in outlives: _#36t\n\n"}
[01:10:38] {"message":"unresolved inference variable in outlives: _#39t","code":null,"level":"error: internal compiler error","spans":[{"file_name":"/checkout/src/test/run-pass/drop/dropck-eyepatch-extern-crate.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"td inference variable in outlives: _#46t","code":null,"level":"error: internal compiler error","spans":[{"file_name":"/checkout/src/test/run-pass/drop/dropck-eyepatch-extern-crate.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"// Copyright 2016 The Rust Project Developers. See the COPYRIGHT","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: internal compiler error: unresolved inference variable in outlives: _#46t\n\n"}
[01:10:38] thread 'main' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:334:17
[01:10:38] 
[01:10:38] error: internal compiler error: unexpected panic
[01:10:38] 
[01:10:38] note: the compiler unexpectedly panicked. this is a bug.
[01:10:38] note: the compiler unexpectedly panicked. this is a bug.
[01:10:38] 
[01:10:38] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:10:38] 
[01:10:38] note: rustc 1.32.0-dev running on x86_64-unknown-linux-gnu
[01:10:38] 
[01:10:38] note: compiler flags: -Z ui-testing -Z borrowck=migrate -Z two-phase-borrows -Z unstable-options -C prefer-dynamic -C rpath
[01:10:38] 
[01:10:38] ------------------------------------------
[01:10:38] 
[01:10:38] thread '[run-pass (nll)] run-pass/drop/dropck-eyepatch-extern-crate.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[01:10:38] thread '[run-pass (nll)] run-pass/drop/dropck-eyepatch-extern-crate.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[01:10:38] note: Run with `RUST_BACKTRACE=1` for a backtraference variable in outlives: _#1t\n\n"}
[01:10:38] {"message":"unresolved inference variable in outlives: _#4t","code":null,"level":"error: internal compiler error","spans":[{"file_name":"/checkout/src/test/run-pass/drop/dropck-eyepatch-reorder.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"// Copyright 2016 The Rust Project Developers. See the COPYRIGHT","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: internal compiler error: unresolved inference variable in outlives: _#4t\n\n"}
[01:10:38] {"message":"unresolved inference variable in outlives: _#6t","code":null,"level":"error: internal compiler error","spans":[{"file_name":"/checkout/src/test/run-pass/drop/dropck-eyepatch-reorder.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"// Copyright 2016 The Rust Project Developers. See the COPYRIGHT","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: internal compiler error: unresolved inference variable in outlives: _#6t\n\n"}
[01:10:38] {"message":"unresolved inference variable in outlives: _#8t","code":null,"level":"error: internal compiler error","spans":[{"file_name":"/checkout/src/test/run-pass/drop/dropck-eyepatch-reorder.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"// Copyright 2016 The Rust Project Developers. See the COPYRIGHT","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: internal compiler error: unresolved inference variable in outlives: _#8t\n\n"}
[01:10:38] {"message":"unresolved inference variable in outlives: _#11t","code":null,"level":"error: internal compiler error","spans":[{"file_name":"/checkout/src/test/run-pass/drop/dropck-eyepatch-reorder.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"// Copyright 2016 The Rust Project Developers. See the COPYRIGHT","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: internal compiler error: unresolved inference variable in outlives: _#11t\n\n"}
[01:10:38] {"message":"unresolved inference variable in outlives: _#13t","code":null,"level":"error: internal compiler error","spans":[{"file_name":"/checkout/src/test/run-pass/drop/dropck-eyepatch-reorder.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"// Copyright 2016 The Rust Project Developers. See the COPYRIGHT","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: internal compiler error: unresolved inference variable in outlives: _#13t\n\n"}
[01:10:38] {"message":"unresolved infereer error","spans":[{"file_name":"/checkout/src/test/run-pass/drop/dropck-eyepatch-reorder.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"// Copyright 2016 The Rust Project Developers. See the COPYRIGHT","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: internal compiler error: unresolved inference variable in outlives: _#29t\n\n"}
[01:10:38] {"message":"unresolved inference variable in outlives: _#34t","code":null,"level":"error: internal compiler error","spans":[{"file_name":"/checkout/src/test/run-pass/drop/dropck-eyepatch-reorder.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"// Copyright 2016 The Rust Project Developers. See the COPYRIGHT","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: internal compiler error: unresolved inference variable in outlives: _#34t\n\n"}
[01:10:38] {"message":"unresolved inference variable in outlives: _#36t","code":null,"level":"error: internal compiler error","spans":[{"file_name":"/checkout/src/test/run-pass/drop/dropck-eyepatch-reorder.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"// Copyright 2016 The Rust Project Developers. See the COPYRIGHT","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":n64-unknown-linux-gnu
[01:10:38] 
[01:10:38] note: compiler flags: -Z ui-testing -Z borrowck=migrate -Z two-phase-borrows -Z unstable-options -C prefer-dynamic -C rpath
[01:10:38] 
[01:10:38] ------------------------------------------
[01:10:38] 
[01:10:38] thread '[run-pass (nll)] run-pass/drop/dropck-eyepatch-reorder.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[01:10:38] thread '[run-pass (nll)] run-pass/drop/dropck-eyepatch-reorder.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3282:9
[01:10:38] 
[01:10:38] ---- [run-pass (nll)] run-pass/drop/dropck-eyepatch.rs stdout ----
[01:10:38] 
[01:10:38] error: test compilation failed although it shouldn't!
[01:10:38] status: exit code: 101
[01:10:38] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/drop/dropck-eyepatch.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/drop/dropck-eyepatch.nll/a" "-Zborrowck=migrate" "-Ztwo-phase-borrows" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/drop/dropck-eyepatch.nll/auxiliary"
[01:10:38] ------------------------------------------
[01:10:38] 
[01:10:38] ------------------------------------------
[01:10:38] stderr:
[01:10:38] stderr:
[01:10:38] ------------------------------------------
[01:10:38] {"message":"unresolved inference variable in outlives: _#1t","code":null,"level":"error: internal compiler error","spans":[{"file_name":"/checkout/src/test/run-pass/drop/dropck-eyepatch.rs","byte_start":0,"bytevariable in outlives: _#6t\n\n"}
[01:10:38] {"message":"unresolved inference variable in outlives: _#8t","code":null,"level":"error: internal compiler error","spans":[{"file_name":"/checkout/src/test/run-pass/drop/dropck-eyepatch.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"// Copyright 2016 The Rust Project Developers. See the COPYRIGHT","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: internal compiler error: unresolved inference variable in outlives: _#8t\n\n"}
[01:10:38] {"message":"unresolved inference variable in outlives: _#11t","code":null,"level":"error: internal compiler error","spans":[{"file_name":"/checkout/src/test/run-pass/drop/dropck-eyepatch.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"// Copyright 2016 The Rust Project Developers. See the COPYRIGHT","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: internal compiler error: unresolved inference variable in outlives: _#11t\n\n"}
[01:10:38] {"message":"unresolved inference variable in outlives: _#13t","code":null,"level":"error: internal compiler error","spans":[{"file_name":"/checkout/src/test/run-pass/drop/dropck-eyepatch.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"// Copyright 2016 The Rust Project Developers. See the COPYRIGHT","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: internal compiler error: unresolved inference variable in outlives: _#13t\n\n"}
[01:10:38] {"message":"unresolved inference variable in outlives: _#18t","code":null,"level":"error: internal compiler error","spans":[{"file_name":"/checkout/src/test/run-pass/drop/dropck-eyepatch.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"// Copyright 2016 The Rust Project Developers. See the COPYRIGHT","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: internal compiler error: unresolved inference variable in outlives: _#18t\n\n"}
[01:10:38] {"message":"unresolved inference variable in outlives: _#20t","code":null,"level":"error: internal compiler error","spans":[{"file_name":"/checkout/src/test/run-pass/drop/dropck-eyepatch.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"// Copyright 2016 The Rust Project Developers. See the COPYRIGHT","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: internal compiler error: unresolved inference variable in outlives: _#20t\n\n"}
[01:10:38] {"message":"unresolved inference variable in outlives: _#22t","code":null,"level":"error: internal compiler error","spans":[{"file_name":"/checkout/src/test/run-pass/drop/dropck-eyepatch.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"// Copyright 2016 The Rust Project Developers. See the COPYRIGHT","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: internal compiler error: unresolved inference variable in outlives: _#22t\n\n"}
[01:10:38] {"message":"unresolved inference variable in outlives: _#25t","code":null,"level":"error: internal compiler error","spans":[{"file_name":"/checkout/src/test/run-pass/drop/dropck-eyepatch.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"// Copyright 2016 The Rust Project Developers. See the COPYRIGHT","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: internal compiler error: unresolved inference variable in outlives: _#25t\n\n"}
[01:10:38] {"message":"unresolved inference variable in outlives: _#27t","code":null,"level":"error: internal compiler error","spans":[{"file_name":"/checkout/src/test/run-pass/drop/dropck-eyepatch.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"// Copyright 2016 The Rust Project Developers. See the COPYRIGHT","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: internal compiler error: unresolved inference variable in outlives: _#27t\n\n"}
[01:10:38] {"message":"unresolved inference variable in outlives: _#29t","code":null,"level":"error: internal compiler error","spans":[{"file_name":"/checkout/src/test/run-pass/drop/dropck-eyepatch.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"// Copyright 2016 The Rust Project Developers. See the COPYRIGHT","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: internal compiler error: unresolved inference variable in outlives: _#29t\n\n"}
[01:10:38] {"message":"unresolved inference variable in outlives: _#34t","code":null,"level":"error: internal compiler error","spans":[{"file_name":"/checkout/src/test/run-pass/drop/dropck-eyepatch.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"// Copyright 2016 The Rust Project Developers. See the COPYRIGHT","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: internal compiler error: unresolved inference variable in outlives: _#34t\n\n"}
[01:10:38] {"message":"unresolved inference variable in outlives: _#36t","code":null,"level":"error: internal compiler error","spans":[{"file_name":"/checkout/src/test/run-pass/drop/dropck-eyepatch.rsror: unresolved inference variable in outlives: _#41t\n\n"}
[01:10:38] {"message":"unresolved inference variable in outlives: _#43t","code":null,"level":"error: internal compiler error","spans":[{"file_name":"/checkout/src/test/run-pass/drop/dropck-eyepatch.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"// Copyright 2016 The Rust Project Developers. See the COPYRIGHT","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: internal compiler error: unresolved inference variable in outlives: _#43t\n\n"}
[01:10:38] {"message":"unresolved inference variable in outlives: _#46t","code":null,"level":"error: internal compiler error","spans":[{"file_name":"/checkout/src/test/run-pass/drop/dropck-eyepatch.rs","byte_start":0,"byte_end":0,"line_start":1,"line_end":1,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"// Copyright 2016 The Rust Project Developers. See the COPYRIGHT","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: internal compiler error: unresolved inference variable in outlives: _#46t\n\n"}
[01:10:38] thread 'main' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:334:17
[01:10:38] 
[01:10:38] error: internal compiler error: unexpected panic
[01:10:38] 
[01:10:38] 
[01:10:38] note: the compiler uneunknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "run-pass" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"
[01:10:38] 
[01:10:38] 
[01:10:38] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:10:38] Build completed unsuccessfully in 0:19:51
[01:10:38] Build completed unsuccessfully in 0:19:51
[01:10:38] Makefile:58: recipe for target 'check' failed
[01:10:38] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:091e8245
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Nov 23 12:09:38 UTC 2018
---
travis_time:end:0743672b:start=1542974979946501731,finish=1542974979950461099,duration=3959368
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:128f73fa
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog
