plain
[00:43:57] ...............................................................i....................................
[00:44:01] ....................................................................................................
[00:44:06] ....................................................................................................
[00:44:13] ............................................................................................i.......
[00:44:15] ..........iiiiiiiii...................................................
[00:44:15] 
[00:44:15] travis_fold:start:test_ui_nll
travis_time:start:test_ui_nll
Check compiletest suite=ui mode=ui compare_mode=nll (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
[00:44:56] ......................................................................................i.............
[00:45:01] ...............................................................i....................................
[00:45:05] ....................................................................................................
[00:45:10] ....................................................................................................
[00:45:16] ................F...........................................................................i.......
ould not find any constraint to blame for '_#2r: bb1[0]","code":null,"level":"error: internal compiler error","spans":[],"children":[],"rendered":"error: internal compiler error: librustc_mir/borrow_check/nll/region_infer/mod.rs:1089: could not find any constraint to blame for '_#2r: bb1[0]\n\n"}
[00:45:18] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:45:18] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:45:18] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:45:18] 
[00:45:18] note: the compiler unexpectedly panicked. this is a bug.
[00:45:18] 
[00:45:18] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:45:18] note: rustc 1.28.0-dev running on x86_64-unknown-linux-gnu
[00:45:18] 
[00:45:18] 
[00:45:18] note: compiler flags: -Z ui-testing -Z borrowck=mir -Z two-phase-borrows -Z unstable-options -C prefer-dynamic -C rpath
[00:45:18] 
[00:45:18] ------------------------------------------
[00:45:18] 
[00:45:18] thread '[ui (nll)] ui/span/regions-escape-loop-via-variable.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3053:9
