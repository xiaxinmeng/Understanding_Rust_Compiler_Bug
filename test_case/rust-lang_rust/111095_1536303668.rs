plain
Documenting book redirect pages (x86_64-unknown-linux-gnu)
Documenting standalone (x86_64-unknown-linux-gnu)
Documenting stage2 library (x86_64-unknown-linux-gnu) in HTML format
 Documenting core v0.0.0 (/checkout/library/core)
thread 'rustc' panicked at 'parent_stack is empty', src/librustdoc/formats/cache.rs:276:34
stack backtrace:
   0:     0x7f71211564b4 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h52dc76d7050253b6
   1:     0x7f71211bd8e8 - core::fmt::write::h1c4f95126470b75d
   2:     0x7f712114ab21 - std::io::Write::write_fmt::h3a98a3bf7c7f0a61
   3:     0x7f71211562c1 - std::sys_common::backtrace::print::ha973a3ed23d931eb
   4:     0x7f712115944a - std::panicking::default_hook::{{closure}}::he6d0ba329fbd7997
   5:     0x7f712115912c - std::panicking::default_hook::h8be2f85705b12f91
   6:     0x7f7121c1b525 - rustc_driver_impl[4443de1d8eb7edd8]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f7121159b57 - std::panicking::rust_panic_with_hook::hedc098a92d2eb7e6
   8:     0x7f71211598d7 - std::panicking::begin_panic_handler::{{closure}}::hb863a3f309d472da
   9:     0x7f7121156996 - std::sys_common::backtrace::__rust_end_short_backtrace::h24a29a0393f3cccb
  10:     0x7f71211595c7 - rust_begin_unwind
  11:     0x7f712110e0c3 - core::panicking::panic_fmt::hbf67e2bb71039799
  12:     0x7f712110e083 - core::option::expect_failed::h8eacc349848b6f6a
  13:     0x55a7c7762a9a - <rustdoc[bb9165771e148a4]::formats::cache::CacheBuilder as rustdoc[bb9165771e148a4]::fold::DocFolder>::fold_item
  14:     0x55a7c79b2441 - <alloc[547386982980159e]::vec::Vec<rustdoc[bb9165771e148a4]::clean::types::Item> as alloc[547386982980159e]::vec::spec_from_iter::SpecFromIter<rustdoc[bb9165771e148a4]::clean::types::Item, core[464138903f013c03]::iter::adapters::filter_map::FilterMap<alloc[547386982980159e]::vec::into_iter::IntoIter<rustdoc[bb9165771e148a4]::clean::types::Item>, <rustdoc[bb9165771e148a4]::formats::cache::CacheBuilder as rustdoc[bb9165771e148a4]::fold::DocFolder>::fold_inner_recur::{closure#1}>>>::from_iter
  15:     0x55a7c775ec45 - <rustdoc[bb9165771e148a4]::formats::cache::CacheBuilder as rustdoc[bb9165771e148a4]::fold::DocFolder>::fold_inner_recur
  16:     0x55a7c775f0ce - <rustdoc[bb9165771e148a4]::formats::cache::CacheBuilder as rustdoc[bb9165771e148a4]::fold::DocFolder>::fold_item_recur
  17:     0x55a7c7761a7b - <rustdoc[bb9165771e148a4]::formats::cache::CacheBuilder as rustdoc[bb9165771e148a4]::fold::DocFolder>::fold_item
  18:     0x55a7c79b2441 - <alloc[547386982980159e]::vec::Vec<rustdoc[bb9165771e148a4]::clean::types::Item> as alloc[547386982980159e]::vec::spec_from_iter::SpecFromIter<rustdoc[bb9165771e148a4]::clean::types::Item, core[464138903f013c03]::iter::adapters::filter_map::FilterMap<alloc[547386982980159e]::vec::into_iter::IntoIter<rustdoc[bb9165771e148a4]::clean::types::Item>, <rustdoc[bb9165771e148a4]::formats::cache::CacheBuilder as rustdoc[bb9165771e148a4]::fold::DocFolder>::fold_inner_recur::{closure#1}>>>::from_iter
  19:     0x55a7c775ed78 - <rustdoc[bb9165771e148a4]::formats::cache::CacheBuilder as rustdoc[bb9165771e148a4]::fold::DocFolder>::fold_inner_recur
  20:     0x55a7c775f0ce - <rustdoc[bb9165771e148a4]::formats::cache::CacheBuilder as rustdoc[bb9165771e148a4]::fold::DocFolder>::fold_item_recur
  21:     0x55a7c7761a7b - <rustdoc[bb9165771e148a4]::formats::cache::CacheBuilder as rustdoc[bb9165771e148a4]::fold::DocFolder>::fold_item
  22:     0x55a7c79b2441 - <alloc[547386982980159e]::vec::Vec<rustdoc[bb9165771e148a4]::clean::types::Item> as alloc[547386982980159e]::vec::spec_from_iter::SpecFromIter<rustdoc[bb9165771e148a4]::clean::types::Item, core[464138903f013c03]::iter::adapters::filter_map::FilterMap<alloc[547386982980159e]::vec::into_iter::IntoIter<rustdoc[bb9165771e148a4]::clean::types::Item>, <rustdoc[bb9165771e148a4]::formats::cache::CacheBuilder as rustdoc[bb9165771e148a4]::fold::DocFolder>::fold_inner_recur::{closure#1}>>>::from_iter
  23:     0x55a7c775ed78 - <rustdoc[bb9165771e148a4]::formats::cache::CacheBuilder as rustdoc[bb9165771e148a4]::fold::DocFolder>::fold_inner_recur
  24:     0x55a7c775f14d - <rustdoc[bb9165771e148a4]::formats::cache::CacheBuilder as rustdoc[bb9165771e148a4]::fold::DocFolder>::fold_item_recur
  25:     0x55a7c7761a7b - <rustdoc[bb9165771e148a4]::formats::cache::CacheBuilder as rustdoc[bb9165771e148a4]::fold::DocFolder>::fold_item
  26:     0x55a7c79b2441 - <alloc[547386982980159e]::vec::Vec<rustdoc[bb9165771e148a4]::clean::types::Item> as alloc[547386982980159e]::vec::spec_from_iter::SpecFromIter<rustdoc[bb9165771e148a4]::clean::types::Item, core[464138903f013c03]::iter::adapters::filter_map::FilterMap<alloc[547386982980159e]::vec::into_iter::IntoIter<rustdoc[bb9165771e148a4]::clean::types::Item>, <rustdoc[bb9165771e148a4]::formats::cache::CacheBuilder as rustdoc[bb9165771e148a4]::fold::DocFolder>::fold_inner_recur::{closure#1}>>>::from_iter
  27:     0x55a7c775ed78 - <rustdoc[bb9165771e148a4]::formats::cache::CacheBuilder as rustdoc[bb9165771e148a4]::fold::DocFolder>::fold_inner_recur
  28:     0x55a7c775f14d - <rustdoc[bb9165771e148a4]::formats::cache::CacheBuilder as rustdoc[bb9165771e148a4]::fold::DocFolder>::fold_item_recur
  29:     0x55a7c7761a7b - <rustdoc[bb9165771e148a4]::formats::cache::CacheBuilder as rustdoc[bb9165771e148a4]::fold::DocFolder>::fold_item
  30:     0x55a7c775fc52 - <rustdoc[bb9165771e148a4]::formats::cache::Cache>::populate
  31:     0x55a7c7698dec - <rustc_session[9e3bfaaec72dc71a]::session::Session>::time::<rustdoc[bb9165771e148a4]::clean::types::Crate, rustdoc[bb9165771e148a4]::core::run_global_ctxt::{closure#10}>
  32:     0x55a7c77aba8f - rustdoc[bb9165771e148a4]::core::run_global_ctxt
  33:     0x55a7c7698f23 - <rustc_session[9e3bfaaec72dc71a]::session::Session>::time::<(rustdoc[bb9165771e148a4]::clean::types::Crate, rustdoc[bb9165771e148a4]::config::RenderOptions, rustdoc[bb9165771e148a4]::formats::cache::Cache), rustdoc[bb9165771e148a4]::main_args::{closure#1}::{closure#0}::{closure#0}::{closure#0}>
  34:     0x55a7c783dc0b - <rustc_middle[595dcd6ac1fa1087]::ty::context::GlobalCtxt>::enter::<rustdoc[bb9165771e148a4]::main_args::{closure#1}::{closure#0}::{closure#0}, core[464138903f013c03]::result::Result<(), rustc_span[5b89c8eb7bdd3449]::ErrorGuaranteed>>
  35:     0x55a7c77167fc - <rustc_interface[956e28b074e00291]::queries::QueryResult<&rustc_middle[595dcd6ac1fa1087]::ty::context::GlobalCtxt>>::enter::<core[464138903f013c03]::result::Result<(), rustc_span[5b89c8eb7bdd3449]::ErrorGuaranteed>, rustdoc[bb9165771e148a4]::main_args::{closure#1}::{closure#0}::{closure#0}>
  36:     0x55a7c78841fd - <rustc_interface[956e28b074e00291]::interface::Compiler>::enter::<rustdoc[bb9165771e148a4]::main_args::{closure#1}::{closure#0}, core[464138903f013c03]::result::Result<(), rustc_span[5b89c8eb7bdd3449]::ErrorGuaranteed>>
  37:     0x55a7c77b167c - rustc_span[5b89c8eb7bdd3449]::set_source_map::<core[464138903f013c03]::result::Result<(), rustc_span[5b89c8eb7bdd3449]::ErrorGuaranteed>, rustc_interface[956e28b074e00291]::interface::run_compiler<core[464138903f013c03]::result::Result<(), rustc_span[5b89c8eb7bdd3449]::ErrorGuaranteed>, rustdoc[bb9165771e148a4]::main_args::{closure#1}>::{closure#0}::{closure#0}>
  38:     0x55a7c7774ae5 - <scoped_tls[cf8d768e635d9123]::ScopedKey<rustc_span[5b89c8eb7bdd3449]::SessionGlobals>>::set::<rustc_interface[956e28b074e00291]::interface::run_compiler<core[464138903f013c03]::result::Result<(), rustc_span[5b89c8eb7bdd3449]::ErrorGuaranteed>, rustdoc[bb9165771e148a4]::main_args::{closure#1}>::{closure#0}, core[464138903f013c03]::result::Result<(), rustc_span[5b89c8eb7bdd3449]::ErrorGuaranteed>>
  39:     0x55a7c764cf66 - std[4b7f08141140a549]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[956e28b074e00291]::util::run_in_thread_pool_with_globals<rustc_interface[956e28b074e00291]::interface::run_compiler<core[464138903f013c03]::result::Result<(), rustc_span[5b89c8eb7bdd3449]::ErrorGuaranteed>, rustdoc[bb9165771e148a4]::main_args::{closure#1}>::{closure#0}, core[464138903f013c03]::result::Result<(), rustc_span[5b89c8eb7bdd3449]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[464138903f013c03]::result::Result<(), rustc_span[5b89c8eb7bdd3449]::ErrorGuaranteed>>
  40:     0x55a7c7745469 - <<std[4b7f08141140a549]::thread::Builder>::spawn_unchecked_<rustc_interface[956e28b074e00291]::util::run_in_thread_pool_with_globals<rustc_interface[956e28b074e00291]::interface::run_compiler<core[464138903f013c03]::result::Result<(), rustc_span[5b89c8eb7bdd3449]::ErrorGuaranteed>, rustdoc[bb9165771e148a4]::main_args::{closure#1}>::{closure#0}, core[464138903f013c03]::result::Result<(), rustc_span[5b89c8eb7bdd3449]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[464138903f013c03]::result::Result<(), rustc_span[5b89c8eb7bdd3449]::ErrorGuaranteed>>::{closure#1} as core[464138903f013c03]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  41:     0x7f712116649e - std::sys::unix::thread::Thread::new::thread_start::h047a712c3d88da9b
  42:     0x7f7120df7b43 - <unknown>
  43:     0x7f7120e89a00 - <unknown>
  44:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.71.0-nightly (c82b66703 2023-05-05) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -Z unstable-options -Z unstable-options -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z force-unstable-if-unmarked -Z unstable-options
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
end of query stack
error: could not document `core`

Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2021 --crate-type lib --crate-name core library/core/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc -Zunstable-options --check-cfg 'names()' --check-cfg 'values()' --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat -Z unstable-options --resource-suffix 1.71.0 --markdown-css rust.css --markdown-no-toc --index-page /checkout/src/doc/index.md -C metadata=b1f251f1f3d92979 -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/release/deps -Csymbol-mangling-version=legacy -Zunstable-options -Zunstable-options '--check-cfg=values(bootstrap)' '--check-cfg=values(stdarch_intel_sde)' '--check-cfg=values(no_fp_fmt_parse)' '--check-cfg=values(no_global_oom_handling)' '--check-cfg=values(no_rc)' '--check-cfg=values(no_sync)' '--check-cfg=values(freebsd12)' '--check-cfg=values(freebsd13)' '--check-cfg=values(backtrace_in_libstd)' '--check-cfg=values(target_env,"libnx")' '--check-cfg=values(target_arch,"asmjs","spirv","nvptx","xtensa")' -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.71.0-nightly
  (c82b66703
  2023-05-05)' '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/")'` (exit status: 101)
