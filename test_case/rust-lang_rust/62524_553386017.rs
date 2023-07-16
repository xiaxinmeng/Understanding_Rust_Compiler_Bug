
error: this file contains an un-closed delimiter
 --> 62524-2.rs:2:3
  |
1 | y![
  |   - un-closed delimiter
2 | Ϥ,
  |   ^

error: macros that expand to items must be delimited with braces or followed by a semicolon
 --> 62524-2.rs:1:3
  |
1 |   y![
  |  ___^
2 | | Ϥ,
  | |__^
  |
thread 'rustc' panicked at 'byte index 1 is not a char boundary; it is inside 'Ϥ' (bytes 0..2) of `Ϥ,`', src\libcore\str\mod.rs:2069:5
stack backtrace:
   0: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
   1: core::fmt::write
   2: <std::io::IoSliceMut as core::fmt::Debug>::fmt
   3: std::panicking::take_hook
   4: std::panicking::take_hook
   5: rustc_driver::report_ice
   6: std::panicking::rust_panic_with_hook
   7: std::panicking::begin_panic_fmt
   8: rust_begin_unwind
   9: core::panicking::panic_fmt
  10: core::str::slice_error_fail
  11: <rustc_driver::args::Error as core::fmt::Debug>::fmt
  12: <rustc_errors::lock::acquire_global_lock::Handle as core::ops::drop::Drop>::drop
  13: rustc_errors::annotate_snippet_emitter_writer::AnnotateSnippetEmitterWriter::ui_testing
  14: <rustc_errors::emitter::EmitterWriter as rustc_errors::emitter::Emitter>::emit_diagnostic
  15: rustc_errors::HandlerInner::emit_diagnostic
  16: rustc_errors::diagnostic_builder::DiagnosticBuilder::emit
  17: rustc_parse::parser::item::<impl rustc_parse::parser::Parser>::parse_foreign_item
  18: rustc_parse::parser::item::<impl rustc_parse::parser::Parser>::parse_item
  19: rustc_parse::parser::item::<impl rustc_parse::parser::Parser>::parse_item
  20: rustc_parse::parser::item::<impl rustc_parse::parser::Parser>::parse_item
  21: rustc_parse::parser::module::<impl rustc_parse::parser::Parser>::parse_crate_mod
  22: rustc_parse::parser::module::<impl rustc_parse::parser::Parser>::parse_crate_mod
  23: rustc_parse::parse_crate_from_file
  24: <rustc_interface::proc_macro_decls::Finder as rustc::hir::itemlikevisit::ItemLikeVisitor>::visit_item
  25: <rustc_interface::proc_macro_decls::Finder as rustc::hir::itemlikevisit::ItemLikeVisitor>::visit_item
  26: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::compile
  27: rustc_interface::queries::<impl rustc_interface::interface::Compiler>::parse
  28: <syntax_pos::symbol::SymbolStr as core::fmt::Display>::fmt
  29: <syntax_pos::symbol::SymbolStr as core::fmt::Display>::fmt
  30: <syntax_pos::symbol::SymbolStr as core::fmt::Display>::fmt
  31: <syntax_pos::symbol::SymbolStr as core::fmt::Display>::fmt
  32: _rust_maybe_catch_panic
  33: <syntax_pos::symbol::SymbolStr as core::fmt::Display>::fmt
  34: ZN244_$LT$std..error..$LT$impl$u20$core..convert..From$LT$alloc..string..String$GT$$u20$for$u20$alloc..boxed..Box$LT$dyn$u20$std..error..Error$u2b$core..marker..Send$u2b$core..marker..Sync$GT$$GT$..from..StringError$u20$as$u20$core..fmt..Display$GT$3fmt17
  35: std::sys::windows::thread::Thread::new
  36: BaseThreadInitThunk
  37: RtlUserThreadStart
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.40.0-nightly (4f03f4a98 2019-11-12) running on x86_64-pc-windows-msvc

query stack during panic:
end of query stack
error: aborting due to previous error
