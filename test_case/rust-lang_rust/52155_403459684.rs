
c:\Rust\GitHub\rust-lang>python x.py build --stage 1 src/bootstrap
Updating only changed submodules
Submodules updated in 0.28 seconds
    Finished dev [unoptimized] target(s) in 0.59s
thread 'main' panicked at 'Error: no rules matched src/bootstrap.', bootstrap\builder.rs:239:21
stack backtrace:
   0: std::sys::windows::backtrace::unwind_backtrace
             at libstd\sys\windows\backtrace/mod.rs:65
   1: std::sys_common::backtrace::print
             at libstd\sys_common/backtrace.rs:71
             at libstd\sys_common/backtrace.rs:59
   2: std::panicking::default_hook::{{closure}}
             at libstd/panicking.rs:211
   3: std::panicking::default_hook
             at libstd/panicking.rs:227
   4: std::panicking::rust_panic_with_hook
             at libstd/panicking.rs:511
   5: std::panicking::continue_panic_fmt
             at libstd/panicking.rs:426
   6: std::panicking::begin_panic_fmt
             at libstd/panicking.rs:413
   7: bootstrap::builder::StepDescription::run
             at bootstrap/builder.rs:239
   8: bootstrap::builder::Builder::run_step_descriptions
             at bootstrap/builder.rs:569
   9: bootstrap::builder::Builder::execute_cli
             at bootstrap/builder.rs:559
  10: bootstrap::Build::build
             at bootstrap/lib.rs:465
  11: bootstrap::main
             at bootstrap\bin/main.rs:29
  12: std::rt::lang_start::{{closure}}
             at C:\projects\rust\src\libstd/rt.rs:74
  13: std::panicking::try::do_call
             at libstd/rt.rs:59
             at libstd/panicking.rs:310
  14: _rust_maybe_catch_panic
             at libpanic_unwind/lib.rs:105
  15: std::panic::catch_unwind
             at libstd/panicking.rs:289
             at libstd/panic.rs:392
  16: std::rt::lang_start_internal
             at libstd/rt.rs:58
  17: std::rt::lang_start
             at C:\projects\rust\src\libstd/rt.rs:74
  18: main
  19: _tmainCRTStartup
  20: mainCRTStartup
  21: unit_addrs_search
  22: unit_addrs_search
failed to run: C:\Rust\GitHub\rust-lang\build\bootstrap\debug\bootstrap build --stage 1 src/bootstrap
Build completed unsuccessfully in 0:00:03
