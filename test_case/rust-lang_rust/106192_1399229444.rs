plain
Rustbook (x86_64-unknown-linux-gnu) - book/2018-edition
Documenting book redirect pages (x86_64-unknown-linux-gnu)
thread 'rustc' panicked at 'attempt to subtract with overflow', compiler/rustc_span/src/source_map.rs:1070:33
stack backtrace:
   0:     0x7f4e0b498445 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hd48741730fc64b23
   1:     0x7f4e0b5088f8 - core::fmt::write::hf0e61b0f493b11f2
   2:     0x7f4e0b48a391 - std::io::Write::write_fmt::hfad5fc1e18ebb74f
   3:     0x7f4e0b498251 - std::sys_common::backtrace::print::h103e5373b835e332
   4:     0x7f4e0b49b634 - std::panicking::default_hook::{{closure}}::h26e39fc7ad7c45b1
   5:     0x7f4e0b49b2fa - std::panicking::default_hook::ha2c788efbdad078d
   6:     0x7f4e0bf156d2 - rustc_driver[f981bb68ac023289]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f4e0b49bda1 - std::panicking::rust_panic_with_hook::h56bdd03e546eb30b
   8:     0x7f4e0b49baca - std::panicking::begin_panic_handler::{{closure}}::h91eb458a90a9ee53
   9:     0x7f4e0b498964 - std::sys_common::backtrace::__rust_end_short_backtrace::h7a24cf49e9d8a92a
  10:     0x7f4e0b49b7b2 - rust_begin_unwind
  11:     0x7f4e0b44afe3 - core::panicking::panic_fmt::h6a9170ecf7dcc870
  12:     0x7f4e0b44b07d - core::panicking::panic::had2da98ed858a7d6
  13:     0x7f4e0f2672ee - <rustc_span[fc789a285f2c4128]::source_map::SourceMap>::lookup_source_file_idx
  14:     0x7f4e0f267041 - <rustc_span[fc789a285f2c4128]::source_map::SourceMap>::lookup_byte_offset
  15:     0x7f4e0e62c461 - <scoped_tls[a10e633ac8948f8b]::ScopedKey<rustc_span[fc789a285f2c4128]::SessionGlobals>>::with::<<rustc_span[fc789a285f2c4128]::span_encoding::Span>::new::{closure#0}, ()>
  16:     0x7f4e0e674057 - <rustc_span[fc789a285f2c4128]::span_encoding::Span>::new
  17:     0x7f4e0e676d99 - <rustc_parse[c7f810b813eeed74]::lexer::StringReader>::next_token
  18:     0x7f4e0e68d7e9 - <rustc_parse[c7f810b813eeed74]::lexer::tokentrees::TokenTreesReader>::parse_token_trees
  19:     0x7f4e0e68d6c8 - <rustc_parse[c7f810b813eeed74]::lexer::tokentrees::TokenTreesReader>::parse_all_token_trees
  20:     0x7f4e0e6750ee - rustc_parse[c7f810b813eeed74]::lexer::parse_token_trees
  21:     0x7f4e0e671021 - rustc_parse[c7f810b813eeed74]::maybe_file_to_stream
  22:     0x7f4e0e670b3f - rustc_parse[c7f810b813eeed74]::maybe_source_file_to_parser
  23:     0x7f4e0e6705e7 - rustc_parse[c7f810b813eeed74]::maybe_new_parser_from_source_str
  24:     0x55f17d8ac34c - <scoped_tls[a10e633ac8948f8b]::ScopedKey<rustc_span[fc789a285f2c4128]::SessionGlobals>>::with::<rustdoc[4e1d943dfb4ac66c]::doctest::make_test::{closure#0}::{closure#0}, (bool, bool, bool)>
  25:     0x55f17d8039dc - <core[41286710297e859b]::panic::unwind_safe::AssertUnwindSafe<rustdoc[4e1d943dfb4ac66c]::doctest::make_test::{closure#0}> as core[41286710297e859b]::ops::function::FnOnce<()>>::call_once
  26:     0x55f17d55e4db - std[b774e7b3d1033480]::panicking::try::<(bool, bool, bool), core[41286710297e859b]::panic::unwind_safe::AssertUnwindSafe<rustdoc[4e1d943dfb4ac66c]::doctest::make_test::{closure#0}>>
  27:     0x55f17d7e4612 - rustc_driver[f981bb68ac023289]::catch_fatal_errors::<rustdoc[4e1d943dfb4ac66c]::doctest::make_test::{closure#0}, (bool, bool, bool)>
  28:     0x55f17d5d4001 - rustdoc[4e1d943dfb4ac66c]::doctest::make_test
  29:     0x55f17d613e7d - <rustdoc[4e1d943dfb4ac66c]::html::markdown::CodeBlocks<rustdoc[4e1d943dfb4ac66c]::html::markdown::TableWrapper<rustdoc[4e1d943dfb4ac66c]::html::markdown::LinkReplacer<core[41286710297e859b]::iter::adapters::map::Map<rustdoc[4e1d943dfb4ac66c]::html::markdown::Footnotes<rustdoc[4e1d943dfb4ac66c]::html::markdown::HeadingLinks<pulldown_cmark[650bec5504f52a4f]::parse::OffsetIter>>, <rustdoc[4e1d943dfb4ac66c]::html::markdown::Markdown>::into_string::{closure#1}>>>> as core[41286710297e859b]::iter::traits::iterator::Iterator>::next
  30:     0x55f17d688b58 - <pulldown_cmark[650bec5504f52a4f]::html::HtmlWriter<rustdoc[4e1d943dfb4ac66c]::html::markdown::CodeBlocks<rustdoc[4e1d943dfb4ac66c]::html::markdown::TableWrapper<rustdoc[4e1d943dfb4ac66c]::html::markdown::LinkReplacer<core[41286710297e859b]::iter::adapters::map::Map<rustdoc[4e1d943dfb4ac66c]::html::markdown::Footnotes<rustdoc[4e1d943dfb4ac66c]::html::markdown::HeadingLinks<pulldown_cmark[650bec5504f52a4f]::parse::OffsetIter>>, <rustdoc[4e1d943dfb4ac66c]::html::markdown::Markdown>::into_string::{closure#1}>>>>, &mut alloc[d9d0463954be5cff]::string::String>>::run
  31:     0x55f17d686c70 - pulldown_cmark[650bec5504f52a4f]::html::push_html::<rustdoc[4e1d943dfb4ac66c]::html::markdown::CodeBlocks<rustdoc[4e1d943dfb4ac66c]::html::markdown::TableWrapper<rustdoc[4e1d943dfb4ac66c]::html::markdown::LinkReplacer<core[41286710297e859b]::iter::adapters::map::Map<rustdoc[4e1d943dfb4ac66c]::html::markdown::Footnotes<rustdoc[4e1d943dfb4ac66c]::html::markdown::HeadingLinks<pulldown_cmark[650bec5504f52a4f]::parse::OffsetIter>>, <rustdoc[4e1d943dfb4ac66c]::html::markdown::Markdown>::into_string::{closure#1}>>>>>
  32:     0x55f17d628797 - rustdoc[4e1d943dfb4ac66c]::markdown::render::<&std[b774e7b3d1033480]::path::PathBuf>
  33:     0x55f17d62fafe - rustc_span[fc789a285f2c4128]::with_source_map::<core[41286710297e859b]::result::Result<(), alloc[d9d0463954be5cff]::string::String>, rustc_interface[bbd78ea14843252c]::interface::run_compiler<core[41286710297e859b]::result::Result<(), alloc[d9d0463954be5cff]::string::String>, rustdoc[4e1d943dfb4ac66c]::main_args::{closure#0}>::{closure#0}::{closure#0}>
  34:     0x55f17d8a9ef3 - <scoped_tls[a10e633ac8948f8b]::ScopedKey<rustc_span[fc789a285f2c4128]::SessionGlobals>>::set::<rustc_interface[bbd78ea14843252c]::interface::run_compiler<core[41286710297e859b]::result::Result<(), alloc[d9d0463954be5cff]::string::String>, rustdoc[4e1d943dfb4ac66c]::main_args::{closure#0}>::{closure#0}, core[41286710297e859b]::result::Result<(), alloc[d9d0463954be5cff]::string::String>>
  35:     0x55f17d63dc12 - std[b774e7b3d1033480]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[bbd78ea14843252c]::util::run_in_thread_pool_with_globals<rustc_interface[bbd78ea14843252c]::interface::run_compiler<core[41286710297e859b]::result::Result<(), alloc[d9d0463954be5cff]::string::String>, rustdoc[4e1d943dfb4ac66c]::main_args::{closure#0}>::{closure#0}, core[41286710297e859b]::result::Result<(), alloc[d9d0463954be5cff]::string::String>>::{closure#0}::{closure#0}, core[41286710297e859b]::result::Result<(), alloc[d9d0463954be5cff]::string::String>>
  36:     0x55f17d55e441 - std[b774e7b3d1033480]::panicking::try::<core[41286710297e859b]::result::Result<(), alloc[d9d0463954be5cff]::string::String>, core[41286710297e859b]::panic::unwind_safe::AssertUnwindSafe<<std[b774e7b3d1033480]::thread::Builder>::spawn_unchecked_<rustc_interface[bbd78ea14843252c]::util::run_in_thread_pool_with_globals<rustc_interface[bbd78ea14843252c]::interface::run_compiler<core[41286710297e859b]::result::Result<(), alloc[d9d0463954be5cff]::string::String>, rustdoc[4e1d943dfb4ac66c]::main_args::{closure#0}>::{closure#0}, core[41286710297e859b]::result::Result<(), alloc[d9d0463954be5cff]::string::String>>::{closure#0}::{closure#0}, core[41286710297e859b]::result::Result<(), alloc[d9d0463954be5cff]::string::String>>::{closure#1}::{closure#0}>>
  37:     0x55f17d7fa545 - <<std[b774e7b3d1033480]::thread::Builder>::spawn_unchecked_<rustc_interface[bbd78ea14843252c]::util::run_in_thread_pool_with_globals<rustc_interface[bbd78ea14843252c]::interface::run_compiler<core[41286710297e859b]::result::Result<(), alloc[d9d0463954be5cff]::string::String>, rustdoc[4e1d943dfb4ac66c]::main_args::{closure#0}>::{closure#0}, core[41286710297e859b]::result::Result<(), alloc[d9d0463954be5cff]::string::String>>::{closure#0}::{closure#0}, core[41286710297e859b]::result::Result<(), alloc[d9d0463954be5cff]::string::String>>::{closure#1} as core[41286710297e859b]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  38:     0x7f4e0b4a8c3e - std::sys::unix::thread::Thread::new::thread_start::hd32288ac36331058
  39:     0x7f4e0b133b43 - <unknown>
  40:     0x7f4e0b1c5a00 - <unknown>
  41:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.68.0-nightly (790960b88 2023-01-21) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z normalize-docs -Z unstable-options
query stack during panic:
end of query stack
Build completed unsuccessfully in 0:21:16
