
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'main' panicked at 'failed printing to stdout: Broken pipe (os error 32)', ../src/libstd/io/stdio.rs:617
stack backtrace:
   1:     0x7f973012861f - std::sys::backtrace::tracing::imp::write::h6528da8103c51ab9
   2:     0x7f973013629b - std::panicking::default_hook::_$u7b$$u7b$closure$u7d$$u7d$::hbe741a5cc3c49508
   3:     0x7f9730135ecf - std::panicking::default_hook::he0146e6a74621cb4
   4:     0x7f97300fc16e - std::panicking::rust_panic_with_hook::h983af77c1a2e581b
   5:     0x7f97301364e1 - std::panicking::begin_panic::he426e15a3766089a
   6:     0x7f97300fe1ca - std::panicking::begin_panic_fmt::hdddb415186c241e7
   7:     0x7f973010ad7b - std::io::stdio::_print::h4cd911b6be2080b8
   8:     0x7f973060cbd0 - rustc_driver::run::h2fcb030617c036bd
   9:     0x7f9730647ef9 - rustc_driver::main::h01ed9408eefe73b5
  10:     0x7f9730135828 - std::panicking::try::call::h852b0d5f2eec25e4
  11:     0x7f973014475b - __rust_try
  12:     0x7f97301446fe - __rust_maybe_catch_panic
  13:     0x7f97301352ce - std::rt::lang_start::hfe4efe1fc39e4a30
  14:     0x7f972fcbc740 - __libc_start_main
  15:     0x55ccd21ed798 - <unknown>
