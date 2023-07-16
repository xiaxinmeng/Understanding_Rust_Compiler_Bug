
$ echo -n "fn eq (brnӍБ" > /tmp/bug.rs
$ RUST_BACKTRACE=1 rustc /tmp/bug.rs  
error: this file contains an un-closed delimiter
 --> /tmp/bug.rs:1:13
  |
1 | fn eq (brnӍБ
  |             ^
  |
help: did you mean to close this delimiter?
 --> /tmp/bug.rs:1:7
  |
1 | fn eq (brnӍБ
  |       ^

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.25.0-nightly (ae920dcc9 2018-01-22) running on x86_64-unknown-linux-gnu

note: run with `RUST_BACKTRACE=1` for a backtrace

thread 'rustc' panicked at 'assertion failed: bpos.to_usize() >= mbc.pos.to_usize() + mbc.bytes', libsyntax/codemap.rs:644:17
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
             at libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::print
             at libstd/sys_common/backtrace.rs:68
             at libstd/sys_common/backtrace.rs:57
   2: std::panicking::default_hook::{{closure}}
             at libstd/panicking.rs:380
   3: std::panicking::default_hook
             at libstd/panicking.rs:390
   4: std::panicking::rust_panic_with_hook
             at libstd/panicking.rs:576
   5: std::panicking::begin_panic
   6: syntax::codemap::CodeMap::bytepos_to_file_charpos
   7: syntax::codemap::CodeMap::lookup_char_pos
   8: syntax::codemap::CodeMap::span_to_filename
   9: <syntax::codemap::CodeMap as rustc_errors::CodeMapper>::call_span_if_macro
  10: rustc_errors::emitter::EmitterWriter::fix_multispan_in_std_macros
  11: <rustc_errors::emitter::EmitterWriter as rustc_errors::emitter::Emitter>::emit
  12: rustc_errors::Handler::emit_db
  13: rustc_errors::diagnostic_builder::DiagnosticBuilder::emit
  14: syntax::parse::parser::Parser::parse_fn_args::{{closure}}
  15: syntax::parse::parser::Parser::parse_fn_args
  16: syntax::parse::parser::Parser::parse_fn_decl
  17: syntax::parse::parser::Parser::parse_item_fn
  18: syntax::parse::parser::Parser::parse_item_
  19: syntax::parse::parser::Parser::parse_item
  20: syntax::parse::parser::Parser::parse_mod_items
  21: syntax::parse::parser::Parser::parse_crate_mod
  22: syntax::parse::parse_crate_from_file
  23: rustc_driver::driver::phase_1_parse_input::{{closure}}
  24: rustc::util::common::time
  25: rustc_driver::driver::phase_1_parse_input
  26: rustc_driver::driver::compile_input
  27: rustc_driver::run_compiler
