plain
    Finished release [optimized] target(s) in 1m 44s
[TIMING] tool::Rustdoc { compiler: Compiler { stage: 2, host: x86_64-pc-windows-msvc } } -- 104.285
[TIMING] doc::Standalone { compiler: Compiler { stage: 2, host: x86_64-pc-windows-msvc }, target: x86_64-pc-windows-msvc } -- 0.691
Documenting book redirect pages (x86_64-pc-windows-msvc)
thread 'main' panicked at 'WorkerLocal can only be used on the thread pool it was created on', C:\Users\runneradmin\.cargo\registry\src\github.com-1ecc6299db9ec823\rustc-rayon-core-0.4.1\src\worker_local.rs:49:17
   0:     0x7ffd707790e2 - std::sys_common::backtrace::_print_fmt
                               at /rustc/7d6f6a66e3d69e4045adcf7dcd58681175b04450/library\std\src\sys_common\backtrace.rs:66
                               at /rustc/7d6f6a66e3d69e4045adcf7dcd58681175b04450/library\std\src\sys_common\backtrace.rs:66
   1:     0x7ffd707790e2 - std::sys_common::backtrace::_print::impl$0::fmt
                               at /rustc/7d6f6a66e3d69e4045adcf7dcd58681175b04450/library\std\src\sys_common\backtrace.rs:45
   2:     0x7ffd707b4a5b - core::fmt::write
                               at /rustc/7d6f6a66e3d69e4045adcf7dcd58681175b04450/library\core\src\fmt\mod.rs:1209
   3:     0x7ffd7076b98a - std::io::Write::write_fmt<std::sys::windows::stdio::Stderr>
                               at /rustc/7d6f6a66e3d69e4045adcf7dcd58681175b04450/library\std\src\io\mod.rs:1682
   4:     0x7ffd7077c7d4 - std::sys_common::backtrace::_print
                               at /rustc/7d6f6a66e3d69e4045adcf7dcd58681175b04450/library\std\src\sys_common\backtrace.rs:48
   5:     0x7ffd7077c7d4 - std::sys_common::backtrace::print
                               at /rustc/7d6f6a66e3d69e4045adcf7dcd58681175b04450/library\std\src\sys_common\backtrace.rs:35
   6:     0x7ffd7077c7d4 - std::panicking::default_hook::closure$1
                               at /rustc/7d6f6a66e3d69e4045adcf7dcd58681175b04450/library\std\src\panicking.rs:267
   7:     0x7ffd7077c40a - std::panicking::default_hook
                               at /rustc/7d6f6a66e3d69e4045adcf7dcd58681175b04450/library\std\src\panicking.rs:286
   8:     0x7ffd6638432c - rustc_driver[40e58cc0c0127815]::handle_options
   9:     0x7ffd7077d210 - std::panicking::rust_panic_with_hook
                               at /rustc/7d6f6a66e3d69e4045adcf7dcd58681175b04450/library\std\src\panicking.rs:692
  10:     0x7ffd6bb212d9 - std[f887b617411af23e]::sys_common::backtrace::__rust_end_short_backtrace::<std[f887b617411af23e]::panicking::begin_panic<&str>::{closure#0}, !>
  11:     0x7ffd6bb2129f - std[f887b617411af23e]::sys_common::backtrace::__rust_end_short_backtrace::<std[f887b617411af23e]::panicking::begin_panic<&str>::{closure#0}, !>
  12:     0x7ffd6bcedf8d - std[f887b617411af23e]::panicking::begin_panic::<&str>
  13:     0x7ffd6bae8cf6 - rustc_ast[c655eefda66d570d]::attr::mk_attr_from_item
  14:     0x7ffd6b0f573e - <rustc_parse[aad88c3f63b15ccf]::parser::Parser>::parse_meta_item_inner
  15:     0x7ffd6b0e0584 - <&(rustc_parse[aad88c3f63b15ccf]::parser::FlatToken, rustc_ast[c655eefda66d570d]::tokenstream::Spacing) as core[a902fac075254dc7]::fmt::Debug>::fmt
  16:     0x7ffd6b159402 - <rustc_parse[aad88c3f63b15ccf]::parser::Parser>::parse_item
  17:     0x7ff74638811e - <scoped_tls[594c24817b4d0cfd]::ScopedKey<rustc_span[af9668afa196594b]::SessionGlobals>>::with::<rustdoc[713014432d3c56d9]::doctest::make_test::{closure#0}::{closure#0}, (bool, bool, bool)>
  18:     0x7ff74633166e - <core[a902fac075254dc7]::panic::unwind_safe::AssertUnwindSafe<rustdoc[713014432d3c56d9]::doctest::make_test::{closure#0}> as core[a902fac075254dc7]::ops::function::FnOnce<()>>::call_once
  19:     0x7ff7460e2bb8 - rustc_driver[40e58cc0c0127815]::catch_fatal_errors::<rustdoc[713014432d3c56d9]::doctest::make_test::{closure#0}, (bool, bool, bool)>
  20:     0x7ff74634cd39 - rustdoc[713014432d3c56d9]::doctest::make_test
  21:     0x7ff746154899 - RNvXs1_NtNtCs9IuKP9CUtxB_7rustdoc4html8markdownINtB5_10CodeBlocksINtB5_12TableWrapperINtB5_12LinkReplacerINtNtNtNtCsevDZkphHm9D_4core4iter8adapters3map3MapINtB5_9FootnotesINtB5_12HeadingLinksNtNtCs9tGCKQXI8gd_14pulldown_cmark5parse10OffsetIterEENCNvMsf_B5
  22:     0x7ff746407e61 - RNvMNtCs9tGCKQXI8gd_14pulldown_cmark4htmlINtB2_10HtmlWriterINtNtNtCs9IuKP9CUtxB_7rustdoc4html8markdown10CodeBlocksINtBY_12TableWrapperINtBY_12LinkReplacerINtNtNtNtCsevDZkphHm9D_4core4iter8adapters3map3MapINtBY_9FootnotesINtBY_12HeadingLinksNtNtB4_5parse10
  23:     0x7ff7463fe88c - RINvNtCs9tGCKQXI8gd_14pulldown_cmark4html9push_htmlINtNtNtCs9IuKP9CUtxB_7rustdoc4html8markdown10CodeBlocksINtBQ_12TableWrapperINtBQ_12LinkReplacerINtNtNtNtCsevDZkphHm9D_4core4iter8adapters3map3MapINtBQ_9FootnotesINtBQ_12HeadingLinksNtNtB4_5parse10OffsetIt
  24:     0x7ff746160086 - <rustdoc[713014432d3c56d9]::html::markdown::Markdown>::into_string
  25:     0x7ff74636562c - rustdoc[713014432d3c56d9]::markdown::render::<&std[f887b617411af23e]::path::PathBuf>
  26:     0x7ff746385b8d - <scoped_tls[594c24817b4d0cfd]::ScopedKey<rustc_span[af9668afa196594b]::SessionGlobals>>::set::<rustdoc[713014432d3c56d9]::main_args::{closure#0}, core[a902fac075254dc7]::result::Result<(), alloc[7f2e145fcc8fa4b]::string::String>>
  27:     0x7ff7460494dd - rustdoc[713014432d3c56d9]::main_args
  28:     0x7ff7463311fa - <core[a902fac075254dc7]::panic::unwind_safe::AssertUnwindSafe<rustdoc[713014432d3c56d9]::main::{closure#0}> as core[a902fac075254dc7]::ops::function::FnOnce<()>>::call_once
  29:     0x7ff7460e2d17 - rustc_driver[40e58cc0c0127815]::catch_with_exit_code::<rustdoc[713014432d3c56d9]::main::{closure#0}>
  30:     0x7ff746043799 - rustdoc[713014432d3c56d9]::main
  31:     0x7ff746021086 - std[f887b617411af23e]::sys_common::backtrace::__rust_begin_short_backtrace::<fn(), ()>
  32:     0x7ff74602101c - std[f887b617411af23e]::rt::lang_start::<()>::{closure#0}
  33:     0x7ffd7075d12e - core::ops::function::impls::impl$2::call_once
                               at /rustc/7d6f6a66e3d69e4045adcf7dcd58681175b04450/library\core\src\ops\function.rs:286
  34:     0x7ffd7075d12e - std::panicking::try::do_call
                               at /rustc/7d6f6a66e3d69e4045adcf7dcd58681175b04450/library\std\src\panicking.rs:483
  35:     0x7ffd7075d12e - std::panicking::try
                               at /rustc/7d6f6a66e3d69e4045adcf7dcd58681175b04450/library\std\src\panicking.rs:447
  36:     0x7ffd7075d12e - std::panic::catch_unwind
                               at /rustc/7d6f6a66e3d69e4045adcf7dcd58681175b04450/library\std\src\panic.rs:137
  37:     0x7ffd7075d12e - std::rt::lang_start_internal::closure$2
                               at /rustc/7d6f6a66e3d69e4045adcf7dcd58681175b04450/library\std\src\rt.rs:148
  38:     0x7ffd7075d12e - std::panicking::try::do_call
                               at /rustc/7d6f6a66e3d69e4045adcf7dcd58681175b04450/library\std\src\panicking.rs:483
  39:     0x7ffd7075d12e - std::panicking::try
                               at /rustc/7d6f6a66e3d69e4045adcf7dcd58681175b04450/library\std\src\panicking.rs:447
  40:     0x7ffd7075d12e - std::panic::catch_unwind
                               at /rustc/7d6f6a66e3d69e4045adcf7dcd58681175b04450/library\std\src\panic.rs:137
  41:     0x7ffd7075d12e - std::rt::lang_start_internal
                               at /rustc/7d6f6a66e3d69e4045adcf7dcd58681175b04450/library\std\src\rt.rs:148
  42:     0x7ff74602106c - main
  43:     0x7ff74650bbdc - invoke_main
                               at D:\a\_work\1\s\src\vctools\crt\vcstartup\src\startup\exe_common.inl:78
  44:     0x7ff74650bbdc - __scrt_common_main_seh
                               at D:\a\_work\1\s\src\vctools\crt\vcstartup\src\startup\exe_common.inl:288
  45:     0x7ffda20e7974 - BaseThreadInitThunk
  46:     0x7ffda301a371 - RtlUserThreadStart
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.66.0-nightly (7d6f6a66e 2022-10-15) running on x86_64-pc-windows-msvc

note: compiler flags: -Z normalize-docs -Z unstable-options
query stack during panic:
end of query stack
Build completed unsuccessfully in 0:23:38
