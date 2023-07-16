
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', src/libcore/option.rs:323
stack backtrace:
   0:     0x7f0afe195a63 - std::sys::imp::backtrace::tracing::imp::unwind_backtrace::h84cf1b4471513b76
                               at /data/rust/src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1:     0x7f0afe18f487 - std::sys_common::backtrace::_print::hc1fb9e40cbd49ba2
                               at /data/rust/src/libstd/sys_common/backtrace.rs:71
   2:     0x7f0afe1a48fd - std::panicking::default_hook::{{closure}}::h6c995967d73dafc7
                               at /data/rust/src/libstd/sys_common/backtrace.rs:60
                               at /data/rust/src/libstd/panicking.rs:355
   3:     0x7f0afe1a445f - std::panicking::default_hook::hbce7d419506cf292
                               at /data/rust/src/libstd/panicking.rs:365
   4:     0x7f0afe1a4ef6 - std::panicking::rust_panic_with_hook::h0908b2962a1212f2
                               at /data/rust/src/libstd/panicking.rs:549
   5:     0x7f0afe1a4d84 - std::panicking::begin_panic::h28f23e30564aa045
                               at /data/rust/src/libstd/panicking.rs:511
   6:     0x7f0afe1a4cb9 - std::panicking::begin_panic_fmt::h119da7f4dd3dfd80
                               at /data/rust/src/libstd/panicking.rs:495
   7:     0x7f0afe1a4c47 - rust_begin_unwind
                               at /data/rust/src/libstd/panicking.rs:471
   8:     0x7f0afe1eb88d - core::panicking::panic_fmt::h0a6e703df3c39315
                               at /data/rust/src/libcore/panicking.rs:69
   9:     0x7f0afe1eb7c4 - core::panicking::panic::hcf9e3d59a8633672
                               at /data/rust/src/libcore/panicking.rs:49
  10:     0x7f0afd5ee1db - rustc_resolve::resolve_imports::ImportResolver::finalize_imports::h9e620209e106a194
                               at /data/rust/src/libcore/macros.rs:21
                               at /data/rust/src/librustc_resolve/resolve_imports.rs:693
                               at /data/rust/src/librustc_resolve/resolve_imports.rs:462
  11:     0x7f0afd5f685c - rustc_resolve::Resolver::resolve_crate::h6b4afa7bcd68dfdd
                               at /data/rust/src/librustc_resolve/lib.rs:1415
  12:     0x7f0afe52126c - rustc_driver::driver::phase_2_configure_and_expand::h44be10961c7d21bb
                               at /data/rust/src/librustc_driver/driver.rs:764
                               at /data/rust/src/librustc/util/common.rs:48
                               at /data/rust/src/librustc_driver/driver.rs:763
  13:     0x7f0afe518196 - rustc_driver::driver::compile_input::h0ca3d134ab854074
                               at /data/rust/src/librustc_driver/driver.rs:114
  14:     0x7f0afe55f711 - rustc_driver::run_compiler::h1760df6c109ed010
                               at /data/rust/src/librustc_driver/lib.rs:221
  15:     0x7f0afe48d291 - std::panicking::try::do_call::h77393471dabbbc8b
                               at /data/rust/src/librustc_driver/lib.rs:1130
                               at /data/rust/src/librustc_driver/lib.rs:137
                               at /data/rust/src/librustc_driver/lib.rs:1064
                               at /data/rust/src/libstd/panic.rs:296
                               at /data/rust/src/libstd/panicking.rs:454
  16:     0x7f0afe1ae57a - __rust_maybe_catch_panic
                               at /data/rust/src/libpanic_unwind/lib.rs:98
  17:     0x7f0afe4a9efb - <F as alloc::boxed::FnBox<A>>::call_box::h4f527d7492852135
                               at /data/rust/src/libstd/panicking.rs:433
                               at /data/rust/src/libstd/panic.rs:361
                               at /data/rust/src/libstd/thread/mod.rs:360
                               at /data/rust/src/liballoc/boxed.rs:640
  18:     0x7f0afe1a35e4 - std::sys::imp::thread::Thread::new::thread_start::hcc5fbed4d4c6ad91
                               at /data/rust/src/liballoc/boxed.rs:650
                               at /data/rust/src/libstd/sys_common/thread.rs:21
                               at /data/rust/src/libstd/sys/unix/thread.rs:84
  19:     0x7f0af61aa6b9 - start_thread
  20:     0x7f0afde4782c - clone
  21:                0x0 - <unknown>
