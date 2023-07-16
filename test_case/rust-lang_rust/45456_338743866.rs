
Copying stage0 rustc from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / sparc64-unknown-linux-gnu)
Assembling stage1 compiler (sparc64-unknown-linux-gnu)
Uplifting stage1 std (x86_64-unknown-linux-gnu -> sparc64-unknown-linux-gnu)
Copying stage2 std from stage1 (x86_64-unknown-linux-gnu -> sparc64-unknown-linux-gnu / sparc64-unknown-linux-gnu)
Uplifting stage1 test (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
Copying stage2 test from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
thread 'main' panicked at 'File::open(stamp) failed with No such file or directory (os error 2)', src/bootstrap/util.rs:57:7
stack backtrace:
   0: std::sys::imp::backtrace::tracing::imp::unwind_backtrace
             at /checkout/src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::_print
             at /checkout/src/libstd/sys_common/backtrace.rs:71
   2: std::panicking::default_hook::{{closure}}
             at /checkout/src/libstd/sys_common/backtrace.rs:60
             at /checkout/src/libstd/panicking.rs:381
   3: std::panicking::default_hook
             at /checkout/src/libstd/panicking.rs:397
   4: std::panicking::rust_panic_with_hook
             at /checkout/src/libstd/panicking.rs:611
   5: std::panicking::begin_panic
             at /checkout/src/libstd/panicking.rs:572
   6: std::panicking::begin_panic_fmt
             at /checkout/src/libstd/panicking.rs:522
   7: bootstrap::util::read_stamp_file
   8: bootstrap::compile::add_to_sysroot
   9: <bootstrap::compile::TestLink as bootstrap::builder::Step>::run
  10: bootstrap::builder::Builder::ensure
  11: <bootstrap::compile::Test as bootstrap::builder::Step>::run
  12: bootstrap::builder::Builder::ensure
  13: <bootstrap::compile::Test as bootstrap::builder::Step>::make_run
  14: bootstrap::builder::StepDescription::maybe_run
  15: bootstrap::builder::StepDescription::run
  16: bootstrap::builder::Builder::run
  17: bootstrap::Build::build
  18: bootstrap::main
  19: __rust_maybe_catch_panic
             at /checkout/src/libpanic_unwind/lib.rs:99
  20: std::rt::lang_start
             at /checkout/src/libstd/panicking.rs:459
             at /checkout/src/libstd/panic.rs:361
             at /checkout/src/libstd/rt.rs:61
  21: main
  22: __libc_start_main
  23: _start
failed to run: /local_scratch/glaubitz/rust/rust/build/bootstrap/debug/bootstrap build
Build completed unsuccessfully in 0:23:34
Makefile:22: recipe for target 'all' failed
make: *** [all] Error 1
