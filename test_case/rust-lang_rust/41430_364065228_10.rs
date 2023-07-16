
   Compiling test_crate v0.1.0 (file:///C:/Dev/Projects/crate_level_proc_macro/test_crate)
thread '<unnamed>' panicked at 'index out of bounds: the len is 0 but the index is 0', C:\Dev\Projects\rust\src\liballoc\vec.rs:1551:10
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
   0: std::sys::windows::backtrace::unwind_backtrace
             at C:\Dev\Projects\rust\src\libstd\sys\windows\backtrace\mod.rs:65
   1: std::sys_common::backtrace::_print
             at C:\Dev\Projects\rust\src\libstd\sys_common\backtrace.rs:71
   2: std::sys_common::backtrace::print
             at C:\Dev\Projects\rust\src\libstd\sys_common\backtrace.rs:58
   3: std::panicking::default_hook::{{closure}}
             at C:\Dev\Projects\rust\src\libstd\panicking.rs:380
   4: std::panicking::default_hook
             at C:\Dev\Projects\rust\src\libstd\panicking.rs:396
   5: std::panicking::rust_panic_with_hook
             at C:\Dev\Projects\rust\src\libstd\panicking.rs:576
   6: std::panicking::begin_panic<alloc::string::String>
             at C:\Dev\Projects\rust\src\libstd\panicking.rs:537
   7: std::panicking::begin_panic_fmt
             at C:\Dev\Projects\rust\src\libstd\panicking.rs:521
   8: std::panicking::rust_begin_panic
             at C:\Dev\Projects\rust\src\libstd\panicking.rs:497
   9: core::panicking::panic_fmt
             at C:\Dev\Projects\rust\src\libcore\panicking.rs:71
  10: core::panicking::panic_bounds_check
             at C:\Dev\Projects\rust\src\libcore\panicking.rs:58
  11: alloc::vec::{{impl}}::index
             at C:\Dev\Projects\rust\src\liballoc\vec.rs:1551
  12: syntax_pos::span_encoding::SpanInterner::get
             at C:\Dev\Projects\rust\src\libsyntax_pos\span_encoding.rs:134
  13: syntax_pos::span_encoding::decode::{{closure}}
             at C:\Dev\Projects\rust\src\libsyntax_pos\span_encoding.rs:109
  14: syntax_pos::span_encoding::with_span_interner::{{closure}}
             at C:\Dev\Projects\rust\src\libsyntax_pos\span_encoding.rs:144
  15: std::thread::local::LocalKey<core::cell::RefCell<syntax_pos::span_encoding::SpanInterner>>::try_with
             at C:\Dev\Projects\rust\src\libstd\thread\local.rs:377
  16: std::thread::local::LocalKey<core::cell::RefCell<syntax_pos::span_encoding::SpanInterner>>::with<core::cell::RefCell<syntax_pos::span_encoding::SpanInterner>,closure,syntax_pos::SpanData>
             at C:\Dev\Projects\rust\src\libstd\thread\local.rs:290
  17: syntax_pos::span_encoding::with_span_interner
             at C:\Dev\Projects\rust\src\libsyntax_pos\span_encoding.rs:144
  18: syntax_pos::span_encoding::decode
             at C:\Dev\Projects\rust\src\libsyntax_pos\span_encoding.rs:109
  19: syntax_pos::span_encoding::Span::data
             at C:\Dev\Projects\rust\src\libsyntax_pos\span_encoding.rs:47
  20: syntax_pos::span_encoding::Span::lo
             at C:\Dev\Projects\rust\src\libsyntax_pos\lib.rs:196
  21: syntax::print::pprust::State::print_item
             at C:\Dev\Projects\rust\src\libsyntax\print\pprust.rs:1173
  22: syntax::print::pprust::item_to_string::{{closure}}
             at C:\Dev\Projects\rust\src\libsyntax\print\pprust.rs:332
  23: syntax::print::pprust::to_string
             at C:\Dev\Projects\rust\src\libsyntax\print\pprust.rs:165
  24: syntax::print::pprust::item_to_string
             at C:\Dev\Projects\rust\src\libsyntax\print\pprust.rs:331
  25: syntax::print::pprust::token_to_string
             at C:\Dev\Projects\rust\src\libsyntax\print\pprust.rs:269
  26: syntax::print::pprust::PrintState::print_tt<syntax::print::pprust::State>
             at C:\Dev\Projects\rust\src\libsyntax\print\pprust.rs:794
  27: syntax::print::pprust::PrintState::print_tts<syntax::print::pprust::State>
             at C:\Dev\Projects\rust\src\libsyntax\print\pprust.rs:818
  28: syntax::print::pprust::tokens_to_string::{{closure}}
             at C:\Dev\Projects\rust\src\libsyntax\print\pprust.rs:320
  29: syntax::print::pprust::to_string
             at C:\Dev\Projects\rust\src\libsyntax\print\pprust.rs:165
  30: syntax::print::pprust::tokens_to_string
             at C:\Dev\Projects\rust\src\libsyntax\print\pprust.rs:320
  31: syntax::tokenstream::{{impl}}::fmt
             at C:\Dev\Projects\rust\src\libsyntax\tokenstream.rs:555
  32: core::fmt::{{impl}}::fmt<proc_macro::TokenStream>
             at C:\Dev\Projects\rust\src\libcore\fmt\mod.rs:1566
  33: core::fmt::Formatter::run
             at C:\Dev\Projects\rust\src\libcore\fmt\mod.rs:1084
  34: core::fmt::write
             at C:\Dev\Projects\rust\src\libcore\fmt\mod.rs:1030
  35: core::fmt::Write::write_fmt<alloc::string::String>
             at C:\Dev\Projects\rust\src\libcore\fmt\mod.rs:226
  36: alloc::string::{{impl}}::to_string<proc_macro::TokenStream>
             at C:\Dev\Projects\rust\src\liballoc\string.rs:2054
  37: crate_level_proc_macro::attribute
             at C:\Dev\Projects\crate_level_proc_macro\src\lib.rs:54
  38: std::panicking::try::do_call
  39: _rust_maybe_catch_panic
  40: <std::thread::local::LocalKey<T>>::with
  41: <syntax_ext::proc_macro_impl::AttrProcMacro as syntax::ext::base::AttrProcMacro>::expand
  42: syntax::ext::expand::MacroExpander::expand
  43: syntax::ext::expand::MacroExpander::expand
  44: syntax::ext::expand::MacroExpander::expand_crate
  45: rustc_driver::driver::count_nodes
  46: rustc_driver::driver::count_nodes
  47: rustc_driver::driver::compile_input
  48: rustc_driver::run_compiler
error: custom attribute panicked
 --> src\main.rs:2:1
  |
2 | #![::crate_level_proc_macro::attribute]
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: Could not compile `test_crate`.
