plain
$ RUST_BACKTRACE=1 ./x.py test src/test/rustdoc-js --keep-stage 1
Updating only changed submodules
Submodules updated in 0.06 seconds
    Finished dev [unoptimized] target(s) in 0.13s
Building stage0 std artifacts (x86_64-apple-darwin -> x86_64-apple-darwin)
    Finished release [optimized] target(s) in 0.20s
Copying stage0 std from stage0 (x86_64-apple-darwin -> x86_64-apple-darwin / x86_64-apple-darwin)
Building stage0 compiler artifacts (x86_64-apple-darwin -> x86_64-apple-darwin)
    Finished release [optimized] target(s) in 0.21s
Copying stage0 rustc from stage0 (x86_64-apple-darwin -> x86_64-apple-darwin / x86_64-apple-darwin)
Building stage0 codegen artifacts (x86_64-apple-darwin -> x86_64-apple-darwin, llvm)
    Finished release [optimized] target(s) in 0.22s
Assembling stage1 compiler (x86_64-apple-darwin)
Warning: Using a potentially old libstd. This may not behave well.
Copying stage1 std from stage1 (x86_64-apple-darwin -> x86_64-apple-darwin / x86_64-apple-darwin)
Warning: Using a potentially old librustc. This may not behave well.
Copying stage1 rustc from stage1 (x86_64-apple-darwin -> x86_64-apple-darwin / x86_64-apple-darwin)
thread 'main' panicked at 'fs::read(stamp) failed with No such file or directory (os error 2)', src/bootstrap/lib.rs:1129:24
stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /Users/vsts/.cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.34/src/backtrace/libunwind.rs:88
   1: backtrace::backtrace::trace_unsynchronized
             at /Users/vsts/.cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.34/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:47
   3: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:36
   4: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:200
   5: std::panicking::default_hook
             at src/libstd/panicking.rs:214
   6: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:477
   7: std::panicking::continue_panic_fmt
             at src/libstd/panicking.rs:384
   8: std::panicking::begin_panic_fmt
             at src/libstd/panicking.rs:339
   9: bootstrap::Build::read_stamp_file
             at src/bootstrap/lib.rs:1129
  10: bootstrap::compile::add_to_sysroot
             at src/bootstrap/compile.rs:941
  11: <bootstrap::compile::RustcLink as bootstrap::builder::Step>::run
             at src/bootstrap/compile.rs:509
  12: bootstrap::builder::Builder::ensure
             at src/bootstrap/builder.rs:1238
  13: <bootstrap::compile::Rustc as bootstrap::builder::Step>::run
             at src/bootstrap/compile.rs:396
  14: bootstrap::builder::Builder::ensure
             at src/bootstrap/builder.rs:1238
  15: <bootstrap::compile::Assemble as bootstrap::builder::Step>::run
             at src/bootstrap/compile.rs:871
  16: bootstrap::builder::Builder::ensure
             at src/bootstrap/builder.rs:1238
  17: bootstrap::builder::Builder::compiler
             at src/bootstrap/builder.rs:564
  18: <bootstrap::test::RustdocJSNotStd as bootstrap::builder::Step>::make_run
             at src/bootstrap/test.rs:696
  19: bootstrap::builder::StepDescription::maybe_run
             at src/bootstrap/builder.rs:183
  20: bootstrap::builder::StepDescription::run
             at src/bootstrap/builder.rs:226
  21: bootstrap::builder::Builder::run_step_descriptions
             at src/bootstrap/builder.rs:556
  22: bootstrap::builder::Builder::execute_cli
             at src/bootstrap/builder.rs:546
  23: bootstrap::Build::build
             at src/bootstrap/lib.rs:449
  24: bootstrap::main
             at src/bootstrap/bin/main.rs:15
  25: std::rt::lang_start::{{closure}}
             at /rustc/e450539c2a8d7f791268d699cbe45ab3e57d43a1/src/libstd/rt.rs:64
  26: std::rt::lang_start_internal::{{closure}}
             at src/libstd/rt.rs:49
  27: std::panicking::try::do_call
             at src/libstd/panicking.rs:296
  28: __rust_maybe_catch_panic
             at src/libpanic_unwind/lib.rs:80
  29: std::panicking::try
             at src/libstd/panicking.rs:275
  30: std::panic::catch_unwind
             at src/libstd/panic.rs:394
  31: std::rt::lang_start_internal
             at src/libstd/rt.rs:48
  32: std::rt::lang_start
             at /rustc/e450539c2a8d7f791268d699cbe45ab3e57d43a1/src/libstd/rt.rs:64
  33: bootstrap::main
