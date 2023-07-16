
expected success, got: exit code: 101
thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1064:9
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
             at libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::_print
             at libstd/sys_common/backtrace.rs:71
   2: std::panicking::default_hook::{{closure}}
             at libstd/sys_common/backtrace.rs:59
             at libstd/panicking.rs:380
   3: std::panicking::default_hook
             at libstd/panicking.rs:396
   4: std::panicking::rust_panic_with_hook
             at libstd/panicking.rs:576
   5: std::panicking::begin_panic
             at /checkout/src/libstd/panicking.rs:537
   6: bootstrap::compile::run_cargo
             at bootstrap/compile.rs:1064
   7: <bootstrap::compile::Rustc as bootstrap::builder::Step>::run
             at bootstrap/compile.rs:496
   8: bootstrap::builder::Builder::ensure
             at bootstrap/builder.rs:841
   9: <bootstrap::compile::Assemble as bootstrap::builder::Step>::run
             at bootstrap/compile.rs:906
  10: bootstrap::builder::Builder::ensure
             at bootstrap/builder.rs:841
  11: bootstrap::builder::Builder::compiler
             at bootstrap/builder.rs:416
  12: <bootstrap::compile::Assemble as bootstrap::builder::Step>::run
             at bootstrap/compile.rs:888
  13: bootstrap::builder::Builder::ensure
             at bootstrap/builder.rs:841
  14: bootstrap::builder::Builder::compiler
             at bootstrap/builder.rs:416
  15: <bootstrap::compile::Std as bootstrap::builder::Step>::make_run
             at bootstrap/compile.rs:57
  16: bootstrap::builder::StepDescription::maybe_run
             at bootstrap/builder.rs:171
  17: bootstrap::builder::StepDescription::run
             at bootstrap/builder.rs:191
  18: bootstrap::builder::Builder::run
             at bootstrap/builder.rs:403
  19: bootstrap::Build::build
             at bootstrap/lib.rs:398
  20: bootstrap::main
             at bootstrap/bin/main.rs:29
  21: std::rt::lang_start::{{closure}}
             at /checkout/src/libstd/rt.rs:74
  22: std::panicking::try::do_call
             at libstd/rt.rs:59
             at libstd/panicking.rs:479
  23: __rust_maybe_catch_panic
             at libpanic_unwind/lib.rs:102
  24: std::rt::lang_start_internal
             at libstd/panicking.rs:458
             at libstd/panic.rs:358
             at libstd/rt.rs:58
  25: std::rt::lang_start
             at /checkout/src/libstd/rt.rs:74
  26: main
  27: __libc_start_main
Traceback (most recent call last):
  File "./x.py", line 20, in <module>
    bootstrap.main()
  File "/var/tmp/portage/dev-lang/rust-1.26.0/work/rustc-1.26.0-src/src/bootstrap/bootstrap.py", line 800, in main
    bootstrap(help_triggered)
  File "/var/tmp/portage/dev-lang/rust-1.26.0/work/rustc-1.26.0-src/src/bootstrap/bootstrap.py", line 791, in bootstrap
    run(args, env=env, verbose=build.verbose)
  File "/var/tmp/portage/dev-lang/rust-1.26.0/work/rustc-1.26.0-src/src/bootstrap/bootstrap.py", line 148, in run
    raise RuntimeError(err)
RuntimeError: failed to run: /var/tmp/portage/dev-lang/rust-1.26.0/work/rustc-1.26.0-src/build/bootstrap/debug/bootstrap build --verbose --config=/var/tmp/portage/dev-lang/rust-1.26.0/work/rustc-1.26.0-src/config.toml -j4
