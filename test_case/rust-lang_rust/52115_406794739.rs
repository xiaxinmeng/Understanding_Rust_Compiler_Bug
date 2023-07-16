plain
travis_time:start:test_mir-opt
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:55:33] 
[00:55:33] running 48 tests
[00:55:43] ERROR 2018-07-21T12:50:24Z: compiletest::runtest: None
[00:55:52] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:55:52] ...............................F................
[00:55:52] 
[00:55:52] ---- [mir-opt] mir-opt/nll/region-liveness-basic.rs stdout ----
[00:55:52] ---- [mir-opt] mir-opt/nll/region-liveness-basic.rs stdout ----
[00:55:52] thread '[mir-opt] mir-opt/nll/region-liveness-basic.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[00:55:52] Current block: None
[00:55:52] Actual Line: "                                         | Live variables on entry to bb2[0]: []"
[00:55:52] Expected Line: "           | Live variables on entry to bb2[0]: [_1, _3]"
[00:55:52] Test Name: rustc.main.nll.0.mir
[00:55:52] Expected:
[00:55:52] ... (elided)
[00:55:52]    bb2: {
[00:55:52]            | Live variables on entry to bb2[0]: [_1, _3]
[00:55:52]        _2 = &'_#2r _1[_3];
[00:55:52]            | Live variables on entry to bb2[1]: [_2]
[00:55:52]        switchInt(const true) -> [false: bb4, otherwise: bb3];
[00:55:52]            | Live variables on exit from bb2: [_2]
[00:55:52] Actual:
[00:55:52] | Free Region Mapping
[00travis_fold:start:after_failure.1
travis_time:start:31aa95e8
