plain
[00:44:28] ............................................................................i.......................
[00:44:33] ....................................................................................................
[00:44:38] ....................................................................................................
[00:44:44] ....................................................................................................
[00:44:49] .........i.................iiiiiiiii...................................................
[00:44:49] 
[00:44:49] travis_fold:start:test_ui_nll
travis_time:start:test_ui_nll
Check compiletest suite=ui mode=ui compare_mode=nll (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
[00:45:39] ............................................................................i.......................
[00:45:44] ....................................................................................................
[00:45:49] ....................................................................................................
[00:45:55] ....................................................................................................
[00:45:59] .........i.................iiiiiiiii...................................................
[00:45:59] 
[00:45:59]  finished in 70.455
[00:45:59] travis_fold:end:test_ui_nll

---
[01:08:29] travis_fold:start:test_stage1-core
travis_time:start:test_stage1-core
Testing core stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:08:29]    Compiling core v0.0.0 (file:///checkout/src/libcore)
[01:08:32] error: `...` range patterns are to be deprecated
[01:08:32]   --> libcore/../libcore/tests/slice.rs:63:45
[01:08:32]    |
[01:08:32] 63 |     assert!(match b.binary_search(&3) { Ok(1...3) => true, _ => false });
[01:08:32]    |                                             ^^^ help: use `..=` for an inclusive range
[01:08:32]    |
[01:08:32]    = note: `-D ellipsis-inclusive-range-patterns` implied by `-D warnings`
[01:08:32] 
[01:08:32] error: `...` range patterns are to be deprecated
[01:08:32]   --> libcore/../libcore/tests/slice.rs:64:45
[01:08:32]    |
[01:08:32] 64 |     assert!(match b.binary_search(&3) { Ok(1...3) => true, _ => false });
[01:08:32]    |                                             ^^^ help: use `..=` for an inclusive range
[01:08:44] error: aborting due to 2 previous errors
[01:08:44] 
ze  Used Avail Use% Mounted on
udev            7.4G  4.0K  7.4G   1% /dev
