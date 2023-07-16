plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:697bea7ddceb6696743da8f159f268aef8bfb3c6)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---
Rustbook (x86_64-unknown-linux-gnu) - book/2018-edition
Documenting book redirect pages (x86_64-unknown-linux-gnu)
thread 'rustc' panicked at 'attempt to subtract with overflow', compiler/rustc_span/src/source_map.rs:1070:33
stack backtrace:
   0:     0x7ff2c9e05085 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hf5a849bd1a92e1d8
   1:     0x7ff2c9e6e938 - core::fmt::write::h0d4262a3a79696d2
   2:     0x7ff2c9df7081 - std::io::Write::write_fmt::h5d65e74602c416fa
   3:     0x7ff2c9e04e91 - std::sys_common::backtrace::print::h319dcc21d6ad1d69
   4:     0x7ff2c9e08094 - std::panicking::default_hook::{{closure}}::h4794098ae69f6e0e
   5:     0x7ff2c9e07d7a - std::panicking::default_hook::hd738b2163a534d6d
   6:     0x7ff2ca8ea735 - rustc_driver_impl[8fb70e63b9698f68]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7ff2c9e087b1 - std::panicking::rust_panic_with_hook::h0d1b8e7b56b6f5f1
   8:     0x7ff2c9e084ea - std::panicking::begin_panic_handler::{{closure}}::h3671cbf881c13468
   9:     0x7ff2c9e05556 - std::sys_common::backtrace::__rust_end_short_backtrace::h30fce96823fdf4be
  10:     0x7ff2c9e08207 - rust_begin_unwind
  11:     0x7ff2c9dbb303 - core::panicking::panic_fmt::h11be2aa706896e61
  12:     0x7ff2c9dbb39d - core::panicking::panic::hbe639ebec1552a23
  13:     0x7ff2cdb5d4de - <rustc_span[33bf41446a234c13]::source_map::SourceMap>::lookup_source_file_idx
  14:     0x7ff2cdb5d231 - <rustc_span[33bf41446a234c13]::source_map::SourceMap>::lookup_byte_offset
  15:     0x7ff2cceecf31 - <scoped_tls[b1fc37971024cbbc]::ScopedKey<rustc_span[33bf41446a234c13]::SessionGlobals>>::with::<<rustc_span[33bf41446a234c13]::span_encoding::Span>::new::{closure#0}, ()>
  16:     0x7ff2ccee69ed - <rustc_span[33bf41446a234c13]::span_encoding::Span>::new
  17:     0x7ff2ccee8f05 - <rustc_parse[721dc28e1c0b7e66]::lexer::StringReader>::next_token
  18:     0x7ff2ccf6f56d - <rustc_parse[721dc28e1c0b7e66]::lexer::tokentrees::TokenTreesReader>::parse_token_trees
  19:     0x7ff2ccf6f45c - <rustc_parse[721dc28e1c0b7e66]::lexer::tokentrees::TokenTreesReader>::parse_all_token_trees
  20:     0x7ff2ccee6f73 - rustc_parse[721dc28e1c0b7e66]::lexer::parse_token_trees
  21:     0x7ff2ccf65e80 - rustc_parse[721dc28e1c0b7e66]::maybe_file_to_stream
  22:     0x7ff2ccf65ae0 - rustc_parse[721dc28e1c0b7e66]::maybe_source_file_to_parser
  23:     0x7ff2ccf65667 - rustc_parse[721dc28e1c0b7e66]::maybe_new_parser_from_source_str
  24:     0x55d03e8a0992 - <scoped_tls[b1fc37971024cbbc]::ScopedKey<rustc_span[33bf41446a234c13]::SessionGlobals>>::with::<rustdoc[65688b809c4b4640]::doctest::make_test::{closure#0}::{closure#0}, (bool, bool, bool)>
  25:     0x55d03e9b1952 - <core[442eb58226ce4530]::panic::unwind_safe::AssertUnwindSafe<rustdoc[65688b809c4b4640]::doctest::make_test::{closure#0}> as core[442eb58226ce4530]::ops::function::FnOnce<()>>::call_once
  26:     0x55d03e6f616b - std[f33851b21b9baba2]::panicking::try::<(bool, bool, bool), core[442eb58226ce4530]::panic::unwind_safe::AssertUnwindSafe<rustdoc[65688b809c4b4640]::doctest::make_test::{closure#0}>>
  27:     0x55d03e7251d2 - rustc_driver_impl[8fb70e63b9698f68]::catch_fatal_errors::<rustdoc[65688b809c4b4640]::doctest::make_test::{closure#0}, (bool, bool, bool)>
  28:     0x55d03e9ba1de - rustdoc[65688b809c4b4640]::doctest::make_test
  29:     0x55d03e6a02ce - <rustdoc[65688b809c4b4640]::html::markdown::CodeBlocks<rustdoc[65688b809c4b4640]::html::markdown::TableWrapper<rustdoc[65688b809c4b4640]::html::markdown::LinkReplacer<core[442eb58226ce4530]::iter::adapters::map::Map<rustdoc[65688b809c4b4640]::html::markdown::Footnotes<rustdoc[65688b809c4b4640]::html::markdown::HeadingLinks<pulldown_cmark[1a504b82e5d86aeb]::parse::OffsetIter>>, <rustdoc[65688b809c4b4640]::html::markdown::Markdown>::into_string::{closure#1}>>>> as core[442eb58226ce4530]::iter::traits::iterator::Iterator>::next
  30:     0x55d03e7c8d25 - <pulldown_cmark[1a504b82e5d86aeb]::html::HtmlWriter<rustdoc[65688b809c4b4640]::html::markdown::CodeBlocks<rustdoc[65688b809c4b4640]::html::markdown::TableWrapper<rustdoc[65688b809c4b4640]::html::markdown::LinkReplacer<core[442eb58226ce4530]::iter::adapters::map::Map<rustdoc[65688b809c4b4640]::html::markdown::Footnotes<rustdoc[65688b809c4b4640]::html::markdown::HeadingLinks<pulldown_cmark[1a504b82e5d86aeb]::parse::OffsetIter>>, <rustdoc[65688b809c4b4640]::html::markdown::Markdown>::into_string::{closure#1}>>>>, &mut alloc[5c8aae13a1457cd8]::string::String>>::run
  31:     0x55d03e7b589d - pulldown_cmark[1a504b82e5d86aeb]::html::push_html::<rustdoc[65688b809c4b4640]::html::markdown::CodeBlocks<rustdoc[65688b809c4b4640]::html::markdown::TableWrapper<rustdoc[65688b809c4b4640]::html::markdown::LinkReplacer<core[442eb58226ce4530]::iter::adapters::map::Map<rustdoc[65688b809c4b4640]::html::markdown::Footnotes<rustdoc[65688b809c4b4640]::html::markdown::HeadingLinks<pulldown_cmark[1a504b82e5d86aeb]::parse::OffsetIter>>, <rustdoc[65688b809c4b4640]::html::markdown::Markdown>::into_string::{closure#1}>>>>>
  32:     0x55d03e6aa06e - <rustdoc[65688b809c4b4640]::html::markdown::Markdown>::into_string
  33:     0x55d03e974df0 - rustdoc[65688b809c4b4640]::markdown::render::<&std[f33851b21b9baba2]::path::PathBuf>
  34:     0x55d03e66dc0e - rustc_span[33bf41446a234c13]::with_source_map::<core[442eb58226ce4530]::result::Result<(), alloc[5c8aae13a1457cd8]::string::String>, rustc_interface[4ea018935ddb1886]::interface::run_compiler<core[442eb58226ce4530]::result::Result<(), alloc[5c8aae13a1457cd8]::string::String>, rustdoc[65688b809c4b4640]::main_args::{closure#0}>::{closure#0}::{closure#0}>
  35:     0x55d03e89e37c - <scoped_tls[b1fc37971024cbbc]::ScopedKey<rustc_span[33bf41446a234c13]::SessionGlobals>>::set::<rustc_interface[4ea018935ddb1886]::interface::run_compiler<core[442eb58226ce4530]::result::Result<(), alloc[5c8aae13a1457cd8]::string::String>, rustdoc[65688b809c4b4640]::main_args::{closure#0}>::{closure#0}, core[442eb58226ce4530]::result::Result<(), alloc[5c8aae13a1457cd8]::string::String>>
  36:     0x55d03e6f6a42 - std[f33851b21b9baba2]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[4ea018935ddb1886]::util::run_in_thread_pool_with_globals<rustc_interface[4ea018935ddb1886]::interface::run_compiler<core[442eb58226ce4530]::result::Result<(), alloc[5c8aae13a1457cd8]::string::String>, rustdoc[65688b809c4b4640]::main_args::{closure#0}>::{closure#0}, core[442eb58226ce4530]::result::Result<(), alloc[5c8aae13a1457cd8]::string::String>>::{closure#0}::{closure#0}, core[442eb58226ce4530]::result::Result<(), alloc[5c8aae13a1457cd8]::string::String>>
  37:     0x55d03e6f60d1 - std[f33851b21b9baba2]::panicking::try::<core[442eb58226ce4530]::result::Result<(), alloc[5c8aae13a1457cd8]::string::String>, core[442eb58226ce4530]::panic::unwind_safe::AssertUnwindSafe<<std[f33851b21b9baba2]::thread::Builder>::spawn_unchecked_<rustc_interface[4ea018935ddb1886]::util::run_in_thread_pool_with_globals<rustc_interface[4ea018935ddb1886]::interface::run_compiler<core[442eb58226ce4530]::result::Result<(), alloc[5c8aae13a1457cd8]::string::String>, rustdoc[65688b809c4b4640]::main_args::{closure#0}>::{closure#0}, core[442eb58226ce4530]::result::Result<(), alloc[5c8aae13a1457cd8]::string::String>>::{closure#0}::{closure#0}, core[442eb58226ce4530]::result::Result<(), alloc[5c8aae13a1457cd8]::string::String>>::{closure#1}::{closure#0}>>
  38:     0x55d03e7c65c5 - <<std[f33851b21b9baba2]::thread::Builder>::spawn_unchecked_<rustc_interface[4ea018935ddb1886]::util::run_in_thread_pool_with_globals<rustc_interface[4ea018935ddb1886]::interface::run_compiler<core[442eb58226ce4530]::result::Result<(), alloc[5c8aae13a1457cd8]::string::String>, rustdoc[65688b809c4b4640]::main_args::{closure#0}>::{closure#0}, core[442eb58226ce4530]::result::Result<(), alloc[5c8aae13a1457cd8]::string::String>>::{closure#0}::{closure#0}, core[442eb58226ce4530]::result::Result<(), alloc[5c8aae13a1457cd8]::string::String>>::{closure#1} as core[442eb58226ce4530]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  39:     0x7ff2c9e14a2e - std::sys::unix::thread::Thread::new::thread_start::hfb5f6683ddccc548
  40:     0x7ff2c9aa2b43 - <unknown>
  41:     0x7ff2c9b34a00 - <unknown>
  42:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.69.0-nightly (fe28079bc 2023-02-22) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z normalize-docs -Z unstable-options
query stack during panic:
end of query stack
Build completed unsuccessfully in 0:25:19
