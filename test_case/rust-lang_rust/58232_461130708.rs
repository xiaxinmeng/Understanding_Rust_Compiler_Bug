
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
thread '<unnamed>' panicked at 'src\librustc\hir\map\hir_id_validator.rs:26:
HirIdValidator: The recorded owner of path segment super (id=12) is ::bar[0]::{{?}}[0] instead of ::bar[0]::{{?}}[1]
HirIdValidator: Same HirId ::bar[0]::{{?}}[1]/2 assigned for nodes path segment super (id=12) and path segment baz (id=35)
HirIdValidator: The recorded owner of path segment super (id=12) is ::bar[0]::{{?}}[0] instead of ::bar[0]::{{?}}[2]
HirIdValidator: Same HirId ::bar[0]::{{?}}[2]/2 assigned for nodes path segment super (id=12) and path segment baz (id=39)', src\librustc\util\bug.rs:37:26
stack backtrace:
   0: _report_error
   1: _report_error
   2: _report_error
   3: _report_error
   4: _report_error
   5: _report_error
   6: _report_error
   7: _report_error
   8: _report_error
   9: _report_error
  10: _report_error
  11: _report_error
  12: _report_error
  13: _report_error
  14: _report_error
  15: _report_error
  16: _report_error
  17: _report_error
  18: _report_error
  19: _report_error
  20: _report_error
  21: _report_error
  22: _report_error
  23: _report_error
  24: _report_error
  25: _report_error
  26: _report_error
  27: _report_error
  28: _report_error
  29: _report_error
  30: _report_error
  31: _report_error
  32: _report_error
  33: _report_error
  34: _report_error
query stack during panic:
end of query stack

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.34.0-dev running on x86_64-pc-windows-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath


------------------------------------------

thread '[ui] ui\issues\issue-56128.rs' panicked at 'explicit panic', src\tools\compiletest\src\runtest.rs:3295:9
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
   0: std::sys_common::backtrace::_print
             at src\libstd\sys\windows\backtrace/mod.rs:94
             at src\libstd\sys\windows\backtrace/mod.rs:81
             at src\libstd\sys_common/backtrace.rs:70
   1: std::panicking::default_hook::{{closure}}
             at src\libstd\sys_common/backtrace.rs:58
             at src\libstd/panicking.rs:200
   2: std::panicking::default_hook
             at src\libstd/panicking.rs:209
   3: std::panicking::rust_panic_with_hook
             at src\libstd/panicking.rs:478
   4: std::panicking::begin_panic
   5: compiletest::runtest::ProcRes::fatal
   6: compiletest::runtest::TestCx::fatal_proc_rec
   7: compiletest::runtest::TestCx::check_if_test_should_compile
   8: compiletest::runtest::TestCx::run_revision
   9: compiletest::runtest::run
  10: <F as alloc::boxed::FnBox<A>>::call_box
  11: <F as alloc::boxed::FnBox<A>>::call_box
             at src\libtest/lib.rs:1468
             at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\liballoc/boxed.rs:734
  12: _rust_maybe_catch_panic
             at src\libpanic_unwind/lib.rs:92
  13: test::run_test::run_test_inner::{{closure}}
             at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd/panicking.rs:276
             at /rustc/d1add97236b64048294692d91fe82b601577dd1f\src\libstd/panic.rs:388
             at src\libtest/lib.rs:1430


failures:
    [ui] ui\issues\issue-56128.rs

test result: FAILED. 5338 passed; 1 failed; 27 ignored; 0 measured; 0 filtered out

thread 'main' panicked at 'Some tests failed', src\tools\compiletest\src\main.rs:502:22
stack backtrace:
   0: std::sys_common::backtrace::_print
             at src\libstd\sys\windows\backtrace/mod.rs:94
             at src\libstd\sys\windows\backtrace/mod.rs:81
             at src\libstd\sys_common/backtrace.rs:70
   1: std::panicking::default_hook::{{closure}}
             at src\libstd\sys_common/backtrace.rs:58
             at src\libstd/panicking.rs:200
   2: std::panicking::default_hook
             at src\libstd/panicking.rs:215
   3: std::panicking::rust_panic_with_hook
             at src\libstd/panicking.rs:478
   4: std::panicking::begin_panic
   5: compiletest::main
   6: std::rt::lang_start::{{closure}}
   7: std::panicking::try::do_call
             at src\libstd/rt.rs:49
             at src\libstd/panicking.rs:297
   8: _rust_maybe_catch_panic
             at src\libpanic_unwind/lib.rs:92
   9: std::rt::lang_start_internal
             at src\libstd/panicking.rs:276
             at src\libstd/panic.rs:388
             at src\libstd/rt.rs:48
  10: main
  11: _tmainCRTStartup
  12: mainCRTStartup
  13: unit_addrs_search
  14: unit_addrs_search
