plain
    Finished release [optimized] target(s) in 1m 46s
[TIMING] tool::Rustdoc { compiler: Compiler { stage: 2, host: x86_64-pc-windows-msvc } } -- 106.436
[TIMING] doc::Standalone { compiler: Compiler { stage: 2, host: x86_64-pc-windows-msvc }, target: x86_64-pc-windows-msvc } -- 0.675
Documenting book redirect pages (x86_64-pc-windows-msvc)
thread 'main' panicked at 'WorkerLocal can only be used on the thread pool it was created on', C:\Users\runneradmin\.cargo\registry\src\github.com-1ecc6299db9ec823\rustc-rayon-core-0.4.1\src\worker_local.rs:49:17
   0:     0x7ffe36c390e2 - std::sys_common::backtrace::_print_fmt
                               at /rustc/8df04bd7c03103b83a8de2e7ceec9e8de0478591/library\std\src\sys_common\backtrace.rs:66
                               at /rustc/8df04bd7c03103b83a8de2e7ceec9e8de0478591/library\std\src\sys_common\backtrace.rs:66
   1:     0x7ffe36c390e2 - std::sys_common::backtrace::_print::impl$0::fmt
                               at /rustc/8df04bd7c03103b83a8de2e7ceec9e8de0478591/library\std\src\sys_common\backtrace.rs:45
   2:     0x7ffe36c74a5b - core::fmt::write
                               at /rustc/8df04bd7c03103b83a8de2e7ceec9e8de0478591/library\core\src\fmt\mod.rs:1209
   3:     0x7ffe36c2baba - std::io::Write::write_fmt<std::sys::windows::stdio::Stderr>
                               at /rustc/8df04bd7c03103b83a8de2e7ceec9e8de0478591/library\std\src\io\mod.rs:1682
                               at /rustc/8df04bd7c03103b83a8de2e7ceec9e8de0478591/library\std\src\sys_common\backtrace.rs:48
   5:     0x7ffe36c3c7d4 - std::sys_common::backtrace::print
                               at /rustc/8df04bd7c03103b83a8de2e7ceec9e8de0478591/library\std\src\sys_common\backtrace.rs:35
   6:     0x7ffe36c3c7d4 - std::panicking::default_hook::closure$1
   6:     0x7ffe36c3c7d4 - std::panicking::default_hook::closure$1
                               at /rustc/8df04bd7c03103b83a8de2e7ceec9e8de0478591/library\std\src\panicking.rs:267
   7:     0x7ffe36c3c40a - std::panicking::default_hook
                               at /rustc/8df04bd7c03103b83a8de2e7ceec9e8de0478591/library\std\src\panicking.rs:286
   8:     0x7ffe2e3a432c - rustc_driver[99224586ab6d1566]::handle_options
                               at /rustc/8df04bd7c03103b83a8de2e7ceec9e8de0478591/library\std\src\panicking.rs:692
                               at /rustc/8df04bd7c03103b83a8de2e7ceec9e8de0478591/library\std\src\panicking.rs:692
  10:     0x7ffe33b6aa29 - std[9f64c043ca95492d]::sys_common::backtrace::__rust_end_short_backtrace::<std[9f64c043ca95492d]::panicking::begin_panic<&str>::{closure#0}, !>
  11:     0x7ffe33b6a9ef - std[9f64c043ca95492d]::sys_common::backtrace::__rust_end_short_backtrace::<std[9f64c043ca95492d]::panicking::begin_panic<&str>::{closure#0}, !>
  12:     0x7ffe33d0e7ad - std[9f64c043ca95492d]::panicking::begin_panic::<&str>
  13:     0x7ffe33b09556 - rustc_ast[92c1cb422da947f1]::attr::mk_attr_from_item
  14:     0x7ffe3311612e - <rustc_parse[2aaa6f3e73492909]::parser::Parser>::parse_meta_item_inner
  15:     0x7ffe33100f74 - <&(rustc_parse[2aaa6f3e73492909]::parser::FlatToken, rustc_ast[92c1cb422da947f1]::tokenstream::Spacing) as core[343bf02eedee9c0a]::fmt::Debug>::fmt
  16:     0x7ffe33179df2 - <rustc_parse[2aaa6f3e73492909]::parser::Parser>::parse_item
  17:     0x7ff626517efe - <scoped_tls[683f74c81906d24f]::ScopedKey<rustc_span[80b1b8a5c3c553be]::SessionGlobals>>::with::<rustdoc[caa07696e35e4ea9]::doctest::make_test::{closure#0}::{closure#0}, (bool, bool, bool)>
  18:     0x7ff6264c140e - <core[343bf02eedee9c0a]::panic::unwind_safe::AssertUnwindSafe<rustdoc[caa07696e35e4ea9]::doctest::make_test::{closure#0}> as core[343bf02eedee9c0a]::ops::function::FnOnce<()>>::call_once
  19:     0x7ff6262725b8 - rustc_driver[99224586ab6d1566]::catch_fatal_errors::<rustdoc[caa07696e35e4ea9]::doctest::make_test::{closure#0}, (bool, bool, bool)>
  20:     0x7ff6264dcad9 - rustdoc[caa07696e35e4ea9]::doctest::make_test
  21:     0x7ff6262e3f49 - RNvXs1_NtNtCshozOSAXfEs7_7rustdoc4html8markdownINtB5_10CodeBlocksINtB5_12TableWrapperINtB5_12LinkReplacerINtNtNtNtCs4u2xRBjjfte_4core4iter8adapters3map3MapINtB5_9FootnotesINtB5_12HeadingLinksNtNtCsdfqYq8o0aup_14pulldown_cmark5parse10OffsetIterEENCNvMsf_B5
  22:     0x7ff626597ce1 - RNvMNtCsdfqYq8o0aup_14pulldown_cmark4htmlINtB2_10HtmlWriterINtNtNtCshozOSAXfEs7_7rustdoc4html8markdown10CodeBlocksINtBY_12TableWrapperINtBY_12LinkReplacerINtNtNtNtCs4u2xRBjjfte_4core4iter8adapters3map3MapINtBY_9FootnotesINtBY_12HeadingLinksNtNtB4_5parse10
  23:     0x7ff62658f1ec - RINvNtCsdfqYq8o0aup_14pulldown_cmark4html9push_htmlINtNtNtCshozOSAXfEs7_7rustdoc4html8markdown10CodeBlocksINtBQ_12TableWrapperINtBQ_12LinkReplacerINtNtNtNtCs4u2xRBjjfte_4core4iter8adapters3map3MapINtBQ_9FootnotesINtBQ_12HeadingLinksNtNtB4_5parse10OffsetIt
  24:     0x7ff6262ef736 - <rustdoc[caa07696e35e4ea9]::html::markdown::Markdown>::into_string
  25:     0x7ff6264f53cc - rustdoc[caa07696e35e4ea9]::markdown::render::<&std[9f64c043ca95492d]::path::PathBuf>
  26:     0x7ff62651596d - <scoped_tls[683f74c81906d24f]::ScopedKey<rustc_span[80b1b8a5c3c553be]::SessionGlobals>>::set::<rustdoc[caa07696e35e4ea9]::main_args::{closure#0}, core[343bf02eedee9c0a]::result::Result<(), alloc[f0b811c1c605c2b3]::string::String>>
  27:     0x7ff6261d94ed - rustdoc[caa07696e35e4ea9]::main_args
  28:     0x7ff6264c0f9a - <core[343bf02eedee9c0a]::panic::unwind_safe::AssertUnwindSafe<rustdoc[caa07696e35e4ea9]::main::{closure#0}> as core[343bf02eedee9c0a]::ops::function::FnOnce<()>>::call_once
  29:     0x7ff626272717 - rustc_driver[99224586ab6d1566]::catch_with_exit_code::<rustdoc[caa07696e35e4ea9]::main::{closure#0}>
  30:     0x7ff6261d37a9 - rustdoc[caa07696e35e4ea9]::main
  31:     0x7ff6261b1006 - std[9f64c043ca95492d]::sys_common::backtrace::__rust_begin_short_backtrace::<fn(), ()>
  32:     0x7ff6261b107c - std[9f64c043ca95492d]::rt::lang_start::<()>::{closure#0}
  33:     0x7ffe36c1d12e - core::ops::function::impls::impl$2::call_once
                               at /rustc/8df04bd7c03103b83a8de2e7ceec9e8de0478591/library\core\src\ops\function.rs:286
                               at /rustc/8df04bd7c03103b83a8de2e7ceec9e8de0478591/library\std\src\panicking.rs:483
  35:     0x7ffe36c1d12e - std::panicking::try
                               at /rustc/8df04bd7c03103b83a8de2e7ceec9e8de0478591/library\std\src\panicking.rs:447
  36:     0x7ffe36c1d12e - std::panic::catch_unwind
---
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.66.0-nightly (8df04bd7c 2022-10-15) running on x86_64-pc-windows-msvc

note: compiler flags: -Z normalize-docs -Z unstable-options
query stack during panic:
end of query stack
Build completed unsuccessfully in 0:23:39
