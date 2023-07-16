rust
error: internal compiler error: cat_expr Errd
     --> /home/simon/projects/servo/target/debug/build/style-9107aeac634bb1f9/out/properties.rs:75381:40
      |
75381 |                 align_items::center => align_self::center,
      |                                        ^^^^^^^^^^^^^^^^^^

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: run with `RUST_BACKTRACE=1` for a backtrace

thread 'rustc' panicked at 'Box<Any>', /home/simon/projects/rust/src/librustc_errors/lib.rs:382
stack backtrace:
   1:     0x7f2b65e50e53 - std::sys::imp::backtrace::tracing::imp::write::h34c2fe4536cff926
   2:     0x7f2b65e5f63d - std::panicking::default_hook::{{closure}}::he19e50a2201d99c4
   3:     0x7f2b65e5f1db - std::panicking::default_hook::h954c9f5a176f6e88
   4:     0x7f2b65e5fac8 - std::panicking::rust_panic_with_hook::h12277e75a2b70114
   5:     0x7f2b5f5f46b7 - std::panicking::begin_panic::hc146ae5a0122e959
   6:     0x7f2b5f609d9a - rustc_errors::Handler::abort_if_errors::h714fd9b9b0557ee7
   7:     0x7f2b645c6f1f - rustc_passes::consts::check_crate::h2923cde18866be7c
   8:     0x7f2b661f4e49 - rustc_driver::driver::phase_3_run_analysis_passes::{{closure}}::h1fd32502995d307a
   9:     0x7f2b661f163e - rustc_driver::driver::phase_3_run_analysis_passes::hfd6790d5cb183751
  10:     0x7f2b661d6091 - rustc_driver::driver::compile_input::h0e69e9ade6aaa4c7
  11:     0x7f2b6621f441 - rustc_driver::run_compiler::h72646ffc571b7660
  12:     0x7f2b6613454b - std::panicking::try::do_call::hbf38fb785c7d412c
  13:     0x7f2b65e68b66 - __rust_maybe_catch_panic
  14:     0x7f2b66155aa1 - <F as alloc::boxed::FnBox<A>>::call_box::h55b6e5fdb729b74d
  15:     0x7f2b65e5e4d0 - std::sys::imp::thread::Thread::new::thread_start::h0578282da79025df
  16:     0x7f2b5ed9e453 - start_thread
  17:     0x7f2b65b257de - __GI___clone
  18:                0x0 - <unknown>
