plain
    Finished release [optimized] target(s) in 2m 07s
[TIMING] tool::Rustdoc { compiler: Compiler { stage: 2, host: x86_64-pc-windows-msvc } } -- 127.345
[TIMING] doc::Standalone { compiler: Compiler { stage: 2, host: x86_64-pc-windows-msvc }, target: x86_64-pc-windows-msvc } -- 0.912
Documenting book redirect pages (x86_64-pc-windows-msvc)
thread 'main' panicked at 'WorkerLocal can only be used on the thread pool it was created on', C:\Users\runneradmin\.cargo\registry\src\github.com-1ecc6299db9ec823\rustc-rayon-core-0.4.1\src\worker_local.rs:49:17
   0:     0x7ffcb7de90b2 - std::sys_common::backtrace::_print_fmt
                               at /rustc/c64c480f1eb2fe2012b71d74055ca499477b2049/library\std\src\sys_common\backtrace.rs:66
                               at /rustc/c64c480f1eb2fe2012b71d74055ca499477b2049/library\std\src\sys_common\backtrace.rs:66
   1:     0x7ffcb7de90b2 - std::sys_common::backtrace::_print::impl$0::fmt
                               at /rustc/c64c480f1eb2fe2012b71d74055ca499477b2049/library\std\src\sys_common\backtrace.rs:45
   2:     0x7ffcb7e244bb - core::fmt::write
                               at /rustc/c64c480f1eb2fe2012b71d74055ca499477b2049/library\core\src\fmt\mod.rs:1209
   3:     0x7ffcb7ddba8a - std::io::Write::write_fmt<std::sys::windows::stdio::Stderr>
                               at /rustc/c64c480f1eb2fe2012b71d74055ca499477b2049/library\std\src\io\mod.rs:1680
   4:     0x7ffcb7dec7a4 - std::sys_common::backtrace::_print
                               at /rustc/c64c480f1eb2fe2012b71d74055ca499477b2049/library\std\src\sys_common\backtrace.rs:48
   5:     0x7ffcb7dec7a4 - std::sys_common::backtrace::print
                               at /rustc/c64c480f1eb2fe2012b71d74055ca499477b2049/library\std\src\sys_common\backtrace.rs:35
   6:     0x7ffcb7dec7a4 - std::panicking::default_hook::closure$1
                               at /rustc/c64c480f1eb2fe2012b71d74055ca499477b2049/library\std\src\panicking.rs:267
   7:     0x7ffcb7dec3da - std::panicking::default_hook
                               at /rustc/c64c480f1eb2fe2012b71d74055ca499477b2049/library\std\src\panicking.rs:286
   8:     0x7ffcaecb2a3c - rustc_driver[7db8b137c33946d9]::handle_options
   9:     0x7ffcb7ded1e0 - std::panicking::rust_panic_with_hook
                               at /rustc/c64c480f1eb2fe2012b71d74055ca499477b2049/library\std\src\panicking.rs:692
  10:     0x7ffcb446bf59 - std[23c42bd656c2436b]::sys_common::backtrace::__rust_end_short_backtrace::<std[23c42bd656c2436b]::panicking::begin_panic<&str>::{closure#0}, !>
  11:     0x7ffcb446bf1f - std[23c42bd656c2436b]::sys_common::backtrace::__rust_end_short_backtrace::<std[23c42bd656c2436b]::panicking::begin_panic<&str>::{closure#0}, !>
  12:     0x7ffcb461005d - std[23c42bd656c2436b]::panicking::begin_panic::<&str>
  13:     0x7ffcb440aa86 - rustc_ast[86e109d338da6fdb]::attr::mk_attr_from_item
  14:     0x7ffcb3a12eae - <rustc_parse[1ad589f2cc154f38]::parser::Parser>::parse_meta_item_inner
  15:     0x7ffcb39fdcf4 - <&(rustc_parse[1ad589f2cc154f38]::parser::FlatToken, rustc_ast[86e109d338da6fdb]::tokenstream::Spacing) as core[670651f4eddda5c4]::fmt::Debug>::fmt
  16:     0x7ffcb3a76b62 - <rustc_parse[1ad589f2cc154f38]::parser::Parser>::parse_item
  17:     0x7ff67ff3363e - <scoped_tls[568048288be70118]::ScopedKey<rustc_span[7baa235a85bf913d]::SessionGlobals>>::with::<rustdoc[4850a6ad93e8fb58]::doctest::make_test::{closure#0}::{closure#0}, (bool, bool, bool)>
  18:     0x7ff67fe9414e - <core[670651f4eddda5c4]::panic::unwind_safe::AssertUnwindSafe<rustdoc[4850a6ad93e8fb58]::doctest::make_test::{closure#0}> as core[670651f4eddda5c4]::ops::function::FnOnce<()>>::call_once
  19:     0x7ff67fc3c438 - rustc_driver[7db8b137c33946d9]::catch_fatal_errors::<rustdoc[4850a6ad93e8fb58]::doctest::make_test::{closure#0}, (bool, bool, bool)>
  20:     0x7ff67feafbb9 - rustdoc[4850a6ad93e8fb58]::doctest::make_test
  21:     0x7ff67fbe2e17 - RNvXs1_NtNtCs6cVJPM5w3UG_7rustdoc4html8markdownINtB5_10CodeBlocksINtB5_12TableWrapperINtB5_12LinkReplacerINtNtNtNtCs8QoFTQto5sm_4core4iter8adapters3map3MapINtB5_9FootnotesINtB5_12HeadingLinksNtNtCshsnggBZDyQO_14pulldown_cmark5parse10OffsetIterEENCNvMsf_B5
  22:     0x7ff67ff685d1 - RNvMNtCshsnggBZDyQO_14pulldown_cmark4htmlINtB2_10HtmlWriterINtNtNtCs6cVJPM5w3UG_7rustdoc4html8markdown10CodeBlocksINtBY_12TableWrapperINtBY_12LinkReplacerINtNtNtNtCs8QoFTQto5sm_4core4iter8adapters3map3MapINtBY_9FootnotesINtBY_12HeadingLinksNtNtB4_5parse10
  23:     0x7ff67ff5fadc - RINvNtCshsnggBZDyQO_14pulldown_cmark4html9push_htmlINtNtNtCs6cVJPM5w3UG_7rustdoc4html8markdown10CodeBlocksINtBQ_12TableWrapperINtBQ_12LinkReplacerINtNtNtNtCs8QoFTQto5sm_4core4iter8adapters3map3MapINtBQ_9FootnotesINtBQ_12HeadingLinksNtNtB4_5parse10OffsetIt
  24:     0x7ff67fbee296 - <rustdoc[4850a6ad93e8fb58]::html::markdown::Markdown>::into_string
  25:     0x7ff67fec865c - rustdoc[4850a6ad93e8fb58]::markdown::render::<&std[23c42bd656c2436b]::path::PathBuf>
  26:     0x7ff67ff30e9d - <scoped_tls[568048288be70118]::ScopedKey<rustc_span[7baa235a85bf913d]::SessionGlobals>>::set::<rustdoc[4850a6ad93e8fb58]::main_args::{closure#0}, core[670651f4eddda5c4]::result::Result<(), alloc[f706bd10daeae793]::string::String>>
  27:     0x7ff67fba731f - rustdoc[4850a6ad93e8fb58]::main_args
  28:     0x7ff67fe93cda - <core[670651f4eddda5c4]::panic::unwind_safe::AssertUnwindSafe<rustdoc[4850a6ad93e8fb58]::main::{closure#0}> as core[670651f4eddda5c4]::ops::function::FnOnce<()>>::call_once
  29:     0x7ff67fc3c597 - rustc_driver[7db8b137c33946d9]::catch_with_exit_code::<rustdoc[4850a6ad93e8fb58]::main::{closure#0}>
  30:     0x7ff67fba15b9 - rustdoc[4850a6ad93e8fb58]::main
  31:     0x7ff67fb81086 - std[23c42bd656c2436b]::sys_common::backtrace::__rust_begin_short_backtrace::<fn(), ()>
  32:     0x7ff67fb8101c - std[23c42bd656c2436b]::rt::lang_start::<()>::{closure#0}
  33:     0x7ffcb7dcd0fe - core::ops::function::impls::impl$2::call_once
                               at /rustc/c64c480f1eb2fe2012b71d74055ca499477b2049/library\core\src\ops\function.rs:286
  34:     0x7ffcb7dcd0fe - std::panicking::try::do_call
                               at /rustc/c64c480f1eb2fe2012b71d74055ca499477b2049/library\std\src\panicking.rs:483
  35:     0x7ffcb7dcd0fe - std::panicking::try
                               at /rustc/c64c480f1eb2fe2012b71d74055ca499477b2049/library\std\src\panicking.rs:447
  36:     0x7ffcb7dcd0fe - std::panic::catch_unwind
                               at /rustc/c64c480f1eb2fe2012b71d74055ca499477b2049/library\std\src\panic.rs:137
  37:     0x7ffcb7dcd0fe - std::rt::lang_start_internal::closure$2
                               at /rustc/c64c480f1eb2fe2012b71d74055ca499477b2049/library\std\src\rt.rs:148
  38:     0x7ffcb7dcd0fe - std::panicking::try::do_call
                               at /rustc/c64c480f1eb2fe2012b71d74055ca499477b2049/library\std\src\panicking.rs:483
  39:     0x7ffcb7dcd0fe - std::panicking::try
                               at /rustc/c64c480f1eb2fe2012b71d74055ca499477b2049/library\std\src\panicking.rs:447
  40:     0x7ffcb7dcd0fe - std::panic::catch_unwind
                               at /rustc/c64c480f1eb2fe2012b71d74055ca499477b2049/library\std\src\panic.rs:137
  41:     0x7ffcb7dcd0fe - std::rt::lang_start_internal
                               at /rustc/c64c480f1eb2fe2012b71d74055ca499477b2049/library\std\src\rt.rs:148
  42:     0x7ff67fb8106c - main
  43:     0x7ff68006c0f0 - invoke_main
                               at D:\a\_work\1\s\src\vctools\crt\vcstartup\src\startup\exe_common.inl:78
  44:     0x7ff68006c0f0 - __scrt_common_main_seh
                               at D:\a\_work\1\s\src\vctools\crt\vcstartup\src\startup\exe_common.inl:288
  45:     0x7ffce98c7974 - BaseThreadInitThunk
  46:     0x7ffcea67a2f1 - RtlUserThreadStart
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.66.0-nightly (c64c480f1 2022-10-12) running on x86_64-pc-windows-msvc

note: compiler flags: -Z normalize-docs -Z unstable-options
query stack during panic:
end of query stack
Build completed unsuccessfully in 0:27:54
