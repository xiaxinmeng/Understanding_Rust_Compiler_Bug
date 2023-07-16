
> cargo build --target=i686-pc-windows-msvc           
   Compiling dependency v0.1.0 (C:\Users\ryanl\Code\issue-repros\rustc-82991\dependency)
   Compiling main_crate v0.1.0 (C:\Users\ryanl\Code\issue-repros\rustc-82991\main_crate)
thread 'rustc' panicked at 'forcing query with already existing `DepNode`
- query-key: crate14
- dep-node: defined_lang_items(cb974dfd4fb62bad-4511b4e87d478f4)', /rustc/cb75ad5db02783e8b0222fee363c5f63f7e2cf5b\compiler\rustc_query_system\src\query\plumbing.rs:586:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.50.0 (cb75ad5db 2021-02-10) running on x86_64-pc-windows-msvc

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [get_lang_items] calculating the lang items map
#1 [codegen_fn_attrs] computing codegen attributes of `main`
end of query stack
error: internal compiler error: inconsistent resolution for a macro
 --> src\main.rs:2:5
  |
2 |     dep::foo!();
  |     ^^^^^^^^
  |
  = note: delayed at compiler\rustc_resolve\src\macros.rs:905:34

thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', compiler\rustc_errors\src\lib.rs:974:13
stack backtrace:
   0:     0x7ff9437bba1e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h199efbbe241ec08f
   1:     0x7ff9437e83ab - core::fmt::write::h63b6c7d72e7e2a54
   2:     0x7ff9437ad238 - <std::io::IoSlice as core::fmt::Debug>::fmt::h607ced2beaa31968
   3:     0x7ff9437bfcbd - std::panicking::take_hook::h0c6a997b269337ea
   4:     0x7ff9437bf724 - std::panicking::take_hook::h0c6a997b269337ea
   5:     0x7ff91d45bad7 - rustc_driver::report_ice::h2ae604e88a111d05
   6:     0x7ff9437c076f - std::panicking::rust_panic_with_hook::h556f87f9f9305c97
   7:     0x7ff9437c0261 - rust_begin_unwind
   8:     0x7ff9437bc37f - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h199efbbe241ec08f
   9:     0x7ff9437c01b9 - rust_begin_unwind
  10:     0x7ff9437c016c - std::panicking::begin_panic_fmt::hc026b96589146be6
  11:     0x7ff921bf7e34 - rustc_errors::HandlerInner::delay_as_bug::h28973f7e9d60a8f6
  12:     0x7ff921bf3514 - <rustc_errors::HandlerInner as core::ops::drop::Drop>::drop::hbc37fc6cd4566a50
  13:     0x7ff91d474867 - <rustc_driver::Compilation as core::fmt::Debug>::fmt::hee44c68edd0d4a9a
  14:     0x7ff91d47d40a - <rustc_driver::Compilation as core::fmt::Debug>::fmt::hee44c68edd0d4a9a
  15:     0x7ff91d47620c - <rustc_driver::Compilation as core::fmt::Debug>::fmt::hee44c68edd0d4a9a
  16:     0x7ff91d46008e - <rustc_driver::Compilation as core::fmt::Debug>::fmt::hee44c68edd0d4a9a
  17:     0x7ff921d95620 - _rust_alloc_zeroed
  18:     0x7ff921d94a6a - _rust_alloc_zeroed
  19:     0x7ff921d90874 - _rust_alloc_zeroed
  20:     0x7ff921d93dd1 - _rust_alloc_zeroed
  21:     0x7ff921d9118d - _rust_alloc_zeroed
  22:     0x7ff9ad171fef - _chkstk
  23:     0x7ff9ad100939 - RtlUnwindEx
  24:     0x7ff921d90d3e - _rust_alloc_zeroed
  25:     0x7ff921d92a41 - _rust_alloc_zeroed
  26:     0x7ff921d92e6b - _rust_alloc_zeroed
  27:     0x7ff921d93ecd - _rust_alloc_zeroed
  28:     0x7ff921d9118d - _rust_alloc_zeroed
  29:     0x7ff9ad171f6f - _chkstk
  30:     0x7ff9ad121454 - RtlRaiseException
  31:     0x7ff9ad1211a5 - RtlRaiseException
  32:     0x7ff9aac1d759 - RaiseException
  33:     0x7ff94380dad8 - _udivmodti4
  34:     0x7ff9437d3981 - _rust_start_panic
  35:     0x7ff9437d3909 - _rust_start_panic
  36:     0x7ff9437c0928 - rust_panic
  37:     0x7ff9437c0884 - std::panicking::rust_panic_with_hook::h556f87f9f9305c97
  38:     0x7ff9437b4099 - std::panic::resume_unwind::h668be6558c89fe0f
  39:     0x7ff91d66cb38 - rustc_interface::passes::QueryContext::print_stats::h865e85c5d05f5d22
  40:     0x7ff91d4a969b - rustc_ast::util::parser::prec_let_scrutinee_needs_par::ha8da07b6c5fa4d2d
  41:     0x7ff91d4ab576 - rustc_ast::util::parser::prec_let_scrutinee_needs_par::ha8da07b6c5fa4d2d
  42:     0x7ff91d41c7ed - chalk_engine::TimeStamp::increment::hfde8a5e4e8d69fa5
  43:     0x7ff91d46bc51 - <rustc_driver::Compilation as core::fmt::Debug>::fmt::hee44c68edd0d4a9a
  44:     0x7ff91d40f69c - <tracing_subscriber::util::TryInitError as core::fmt::Display>::fmt::hf3e9f3dfdba5224e
  45:     0x7ff91d4aba0c - rustc_ast::util::parser::prec_let_scrutinee_needs_par::ha8da07b6c5fa4d2d
  46:     0x7ff91d4851d4 - rustc_ast::util::parser::prec_let_scrutinee_needs_par::ha8da07b6c5fa4d2d
  47:     0x7ff91d45f8d5 - <rustc_driver::Compilation as core::fmt::Debug>::fmt::hee44c68edd0d4a9a
  48:     0x7ff91d486a09 - rustc_ast::util::parser::prec_let_scrutinee_needs_par::ha8da07b6c5fa4d2d
  49:     0x7ff91d46cc7b - <rustc_driver::Compilation as core::fmt::Debug>::fmt::hee44c68edd0d4a9a
  50:     0x7ff91d3f44bd - <tracing_subscriber::util::TryInitError as core::fmt::Display>::fmt::hf3e9f3dfdba5224e
  51:     0x7ff9437cfd73 - std::sys::windows::thread::Thread::new::h36b7d068c12d6134
  52:     0x7ff9abc87034 - BaseThreadInitThunk
  53:     0x7ff9ad122651 - RtlUserThreadStart

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.50.0 (cb75ad5db 2021-02-10) running on x86_64-pc-windows-msvc

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
thread panicked while panicking. aborting.
error: could not compile `main_crate`

Caused by:
  process didn't exit successfully: `rustc --crate-name main_crate --edition=2018 src\main.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 -C metadata=d3ac3ec5167edbd7 --out-dir C:\Users\ryanl\Code\issue-repros\rustc-82991\main_crate\target\i686-pc-windows-msvc\debug\deps --target i686-pc-windows-msvc -C incremental=C:\Users\ryanl\Code\issue-repros\rustc-82991\main_crate\target\i686-pc-windows-msvc\debug\incremental -L dependency=C:\Users\ryanl\Code\issue-repros\rustc-82991\main_crate\target\i686-pc-windows-msvc\debug\deps -L dependency=C:\Users\ryanl\Code\issue-repros\rustc-82991\main_crate\target\debug\deps --extern dep=C:\Users\ryanl\Code\issue-repros\rustc-82991\main_crate\target\debug\deps\dependency-908b25e1ff475f50.dll` (exit code: 0xc000001d, STATUS_ILLEGAL_INSTRUCTION)

> cargo build                              
   Compiling main_crate v0.1.0 (C:\Users\ryanl\Code\issue-repros\rustc-82991\main_crate)
    Finished dev [unoptimized + debuginfo] target(s) in 0.33s
