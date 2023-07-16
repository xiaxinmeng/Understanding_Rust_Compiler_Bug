plain
Documenting book redirect pages (x86_64-unknown-linux-gnu)
Documenting standalone (x86_64-unknown-linux-gnu)
Documenting stage2 library (x86_64-unknown-linux-gnu) in HTML format
 Documenting core v0.0.0 (/checkout/library/core)
thread 'rustc' panicked at 'parent_stack is empty', src/librustdoc/formats/cache.rs:277:34
   0:     0x7f73ff68f8c5 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hac94484aeb9a6628
   1:     0x7f73ff6ffd78 - core::fmt::write::he3850e0f15379e69
   2:     0x7f73ff681811 - std::io::Write::write_fmt::h7e82e7d813f95dcf
   3:     0x7f73ff68f6d1 - std::sys_common::backtrace::print::hfa40bd6e1038c0f1
   3:     0x7f73ff68f6d1 - std::sys_common::backtrace::print::hfa40bd6e1038c0f1
   4:     0x7f73ff692ab4 - std::panicking::default_hook::{{closure}}::h098f10eafb73bb8c
   5:     0x7f73ff69277a - std::panicking::default_hook::h253408f8279e32a0
   6:     0x7f7400102662 - rustc_driver[76ceb86076a0ddcf]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f73ff693221 - std::panicking::rust_panic_with_hook::h306ee51e1330c0ea
   8:     0x7f73ff692f89 - std::panicking::begin_panic_handler::{{closure}}::he4cff1ddd65a785b
   9:     0x7f73ff68fde4 - std::sys_common::backtrace::__rust_end_short_backtrace::hb14f5c10eaadf893
  10:     0x7f73ff692c32 - rust_begin_unwind
  11:     0x7f73ff642fe3 - core::panicking::panic_fmt::h57433676040d4176
  12:     0x7f73ff6fc711 - core::panicking::panic_display::h33c0461cabfb1954
  13:     0x7f73ff6fc6bb - core::panicking::panic_str::hd62edb38448d8a8f
  14:     0x7f73ff642fa6 - core::option::expect_failed::hdc7bbcf4624ca389
  15:     0x562f6e96650a - <rustdoc[9fb8f286222dc82d]::formats::cache::CacheBuilder as rustdoc[9fb8f286222dc82d]::fold::DocFolder>::fold_item
  16:     0x562f6ea09f28 - <alloc[d92c3739639b08b2]::vec::Vec<rustdoc[9fb8f286222dc82d]::clean::types::Item> as alloc[d92c3739639b08b2]::vec::spec_from_iter::SpecFromIter<rustdoc[9fb8f286222dc82d]::clean::types::Item, core[ab38b23a0681e1b1]::iter::adapters::filter_map::FilterMap<alloc[d92c3739639b08b2]::vec::into_iter::IntoIter<rustdoc[9fb8f286222dc82d]::clean::types::Item>, <rustdoc[9fb8f286222dc82d]::formats::cache::CacheBuilder as rustdoc[9fb8f286222dc82d]::fold::DocFolder>::fold_crate::{closure#0}>>>::from_iter
  17:     0x562f6e9617b5 - <rustdoc[9fb8f286222dc82d]::formats::cache::CacheBuilder as rustdoc[9fb8f286222dc82d]::fold::DocFolder>::fold_inner_recur
  18:     0x562f6e961c7f - <rustdoc[9fb8f286222dc82d]::formats::cache::CacheBuilder as rustdoc[9fb8f286222dc82d]::fold::DocFolder>::fold_item_recur
  19:     0x562f6e964f09 - <rustdoc[9fb8f286222dc82d]::formats::cache::CacheBuilder as rustdoc[9fb8f286222dc82d]::fold::DocFolder>::fold_item
  20:     0x562f6ea09f28 - <alloc[d92c3739639b08b2]::vec::Vec<rustdoc[9fb8f286222dc82d]::clean::types::Item> as alloc[d92c3739639b08b2]::vec::spec_from_iter::SpecFromIter<rustdoc[9fb8f286222dc82d]::clean::types::Item, core[ab38b23a0681e1b1]::iter::adapters::filter_map::FilterMap<alloc[d92c3739639b08b2]::vec::into_iter::IntoIter<rustdoc[9fb8f286222dc82d]::clean::types::Item>, <rustdoc[9fb8f286222dc82d]::formats::cache::CacheBuilder as rustdoc[9fb8f286222dc82d]::fold::DocFolder>::fold_crate::{closure#0}>>>::from_iter
  21:     0x562f6e9618e8 - <rustdoc[9fb8f286222dc82d]::formats::cache::CacheBuilder as rustdoc[9fb8f286222dc82d]::fold::DocFolder>::fold_inner_recur
  22:     0x562f6e961c7f - <rustdoc[9fb8f286222dc82d]::formats::cache::CacheBuilder as rustdoc[9fb8f286222dc82d]::fold::DocFolder>::fold_item_recur
  23:     0x562f6e964f09 - <rustdoc[9fb8f286222dc82d]::formats::cache::CacheBuilder as rustdoc[9fb8f286222dc82d]::fold::DocFolder>::fold_item
  24:     0x562f6ea09f28 - <alloc[d92c3739639b08b2]::vec::Vec<rustdoc[9fb8f286222dc82d]::clean::types::Item> as alloc[d92c3739639b08b2]::vec::spec_from_iter::SpecFromIter<rustdoc[9fb8f286222dc82d]::clean::types::Item, core[ab38b23a0681e1b1]::iter::adapters::filter_map::FilterMap<alloc[d92c3739639b08b2]::vec::into_iter::IntoIter<rustdoc[9fb8f286222dc82d]::clean::types::Item>, <rustdoc[9fb8f286222dc82d]::formats::cache::CacheBuilder as rustdoc[9fb8f286222dc82d]::fold::DocFolder>::fold_crate::{closure#0}>>>::from_iter
  25:     0x562f6e9618e8 - <rustdoc[9fb8f286222dc82d]::formats::cache::CacheBuilder as rustdoc[9fb8f286222dc82d]::fold::DocFolder>::fold_inner_recur
  26:     0x562f6e961cfe - <rustdoc[9fb8f286222dc82d]::formats::cache::CacheBuilder as rustdoc[9fb8f286222dc82d]::fold::DocFolder>::fold_item_recur
  27:     0x562f6e964f09 - <rustdoc[9fb8f286222dc82d]::formats::cache::CacheBuilder as rustdoc[9fb8f286222dc82d]::fold::DocFolder>::fold_item
  28:     0x562f6ea09f28 - <alloc[d92c3739639b08b2]::vec::Vec<rustdoc[9fb8f286222dc82d]::clean::types::Item> as alloc[d92c3739639b08b2]::vec::spec_from_iter::SpecFromIter<rustdoc[9fb8f286222dc82d]::clean::types::Item, core[ab38b23a0681e1b1]::iter::adapters::filter_map::FilterMap<alloc[d92c3739639b08b2]::vec::into_iter::IntoIter<rustdoc[9fb8f286222dc82d]::clean::types::Item>, <rustdoc[9fb8f286222dc82d]::formats::cache::CacheBuilder as rustdoc[9fb8f286222dc82d]::fold::DocFolder>::fold_crate::{closure#0}>>>::from_iter
  29:     0x562f6e9618e8 - <rustdoc[9fb8f286222dc82d]::formats::cache::CacheBuilder as rustdoc[9fb8f286222dc82d]::fold::DocFolder>::fold_inner_recur
  30:     0x562f6e961cfe - <rustdoc[9fb8f286222dc82d]::formats::cache::CacheBuilder as rustdoc[9fb8f286222dc82d]::fold::DocFolder>::fold_item_recur
  31:     0x562f6e964f09 - <rustdoc[9fb8f286222dc82d]::formats::cache::CacheBuilder as rustdoc[9fb8f286222dc82d]::fold::DocFolder>::fold_item
  32:     0x562f6e962943 - <rustdoc[9fb8f286222dc82d]::formats::cache::Cache>::populate
  33:     0x562f6e8a5669 - <rustc_session[55a6777e03aa5349]::session::Session>::time::<rustdoc[9fb8f286222dc82d]::clean::types::Crate, rustdoc[9fb8f286222dc82d]::core::run_global_ctxt::{closure#9}>
  34:     0x562f6e83d501 - rustdoc[9fb8f286222dc82d]::core::run_global_ctxt
  35:     0x562f6e8a8330 - <rustc_interface[639dc67c96dc38c7]::passes::QueryContext>::enter::<rustdoc[9fb8f286222dc82d]::main_args::{closure#1}::{closure#0}::{closure#1}, core[ab38b23a0681e1b1]::result::Result<(), rustc_errors[7a90d71b2bf2cf12]::ErrorGuaranteed>>
  36:     0x562f6e77aba1 - <rustc_interface[639dc67c96dc38c7]::queries::QueryResult<rustc_interface[639dc67c96dc38c7]::passes::QueryContext>>::enter::<core[ab38b23a0681e1b1]::result::Result<(), rustc_errors[7a90d71b2bf2cf12]::ErrorGuaranteed>, rustdoc[9fb8f286222dc82d]::main_args::{closure#1}::{closure#0}::{closure#1}>
  37:     0x562f6ea6afbc - <rustc_interface[639dc67c96dc38c7]::interface::Compiler>::enter::<rustdoc[9fb8f286222dc82d]::main_args::{closure#1}::{closure#0}, core[ab38b23a0681e1b1]::result::Result<(), rustc_errors[7a90d71b2bf2cf12]::ErrorGuaranteed>>
  38:     0x562f6e86096f - rustc_span[121e22b2f16e8a8b]::with_source_map::<core[ab38b23a0681e1b1]::result::Result<(), rustc_errors[7a90d71b2bf2cf12]::ErrorGuaranteed>, rustc_interface[639dc67c96dc38c7]::interface::run_compiler<core[ab38b23a0681e1b1]::result::Result<(), rustc_errors[7a90d71b2bf2cf12]::ErrorGuaranteed>, rustdoc[9fb8f286222dc82d]::main_args::{closure#1}>::{closure#0}::{closure#0}>
  39:     0x562f6e77c072 - <scoped_tls[c8cf18b7ad4fa1ae]::ScopedKey<rustc_span[121e22b2f16e8a8b]::SessionGlobals>>::set::<rustc_interface[639dc67c96dc38c7]::interface::run_compiler<core[ab38b23a0681e1b1]::result::Result<(), rustc_errors[7a90d71b2bf2cf12]::ErrorGuaranteed>, rustdoc[9fb8f286222dc82d]::main_args::{closure#1}>::{closure#0}, core[ab38b23a0681e1b1]::result::Result<(), rustc_errors[7a90d71b2bf2cf12]::ErrorGuaranteed>>
  40:     0x562f6e86e639 - std[a7386e97e135bae3]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[639dc67c96dc38c7]::util::run_in_thread_pool_with_globals<rustc_interface[639dc67c96dc38c7]::interface::run_compiler<core[ab38b23a0681e1b1]::result::Result<(), rustc_errors[7a90d71b2bf2cf12]::ErrorGuaranteed>, rustdoc[9fb8f286222dc82d]::main_args::{closure#1}>::{closure#0}, core[ab38b23a0681e1b1]::result::Result<(), rustc_errors[7a90d71b2bf2cf12]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[ab38b23a0681e1b1]::result::Result<(), rustc_errors[7a90d71b2bf2cf12]::ErrorGuaranteed>>
  41:     0x562f6e7918b6 - std[a7386e97e135bae3]::panicking::try::<core[ab38b23a0681e1b1]::result::Result<(), rustc_errors[7a90d71b2bf2cf12]::ErrorGuaranteed>, core[ab38b23a0681e1b1]::panic::unwind_safe::AssertUnwindSafe<<std[a7386e97e135bae3]::thread::Builder>::spawn_unchecked_<rustc_interface[639dc67c96dc38c7]::util::run_in_thread_pool_with_globals<rustc_interface[639dc67c96dc38c7]::interface::run_compiler<core[ab38b23a0681e1b1]::result::Result<(), rustc_errors[7a90d71b2bf2cf12]::ErrorGuaranteed>, rustdoc[9fb8f286222dc82d]::main_args::{closure#1}>::{closure#0}, core[ab38b23a0681e1b1]::result::Result<(), rustc_errors[7a90d71b2bf2cf12]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[ab38b23a0681e1b1]::result::Result<(), rustc_errors[7a90d71b2bf2cf12]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  42:     0x562f6ea3cda5 - <<std[a7386e97e135bae3]::thread::Builder>::spawn_unchecked_<rustc_interface[639dc67c96dc38c7]::util::run_in_thread_pool_with_globals<rustc_interface[639dc67c96dc38c7]::interface::run_compiler<core[ab38b23a0681e1b1]::result::Result<(), rustc_errors[7a90d71b2bf2cf12]::ErrorGuaranteed>, rustdoc[9fb8f286222dc82d]::main_args::{closure#1}>::{closure#0}, core[ab38b23a0681e1b1]::result::Result<(), rustc_errors[7a90d71b2bf2cf12]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[ab38b23a0681e1b1]::result::Result<(), rustc_errors[7a90d71b2bf2cf12]::ErrorGuaranteed>>::{closure#1} as core[ab38b23a0681e1b1]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  43:     0x7f73ff6a00be - std::sys::unix::thread::Thread::new::thread_start::hdd32faf8bdb95594
  44:     0x7f73ff32bb43 - <unknown>
  45:     0x7f73ff3bda00 - <unknown>
  46:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.68.0-nightly (33d3af4b5 2023-01-19) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -Z unstable-options -Z unstable-options -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z force-unstable-if-unmarked -Z unstable-options
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
end of query stack
error: could not document `core`

Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2021 --crate-type lib --crate-name core library/core/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc -Zunstable-options --check-cfg 'names()' --check-cfg 'values()' --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat -Z unstable-options --resource-suffix 1.68.0 --markdown-css rust.css --markdown-no-toc --index-page /checkout/src/doc/index.md -C metadata=b1f251f1f3d92979 -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/release/deps -Csymbol-mangling-version=legacy -Zunstable-options -Zunstable-options '--check-cfg=values(bootstrap)' '--check-cfg=values(stdarch_intel_sde)' '--check-cfg=values(no_fp_fmt_parse)' '--check-cfg=values(no_global_oom_handling)' '--check-cfg=values(no_rc)' '--check-cfg=values(no_sync)' '--check-cfg=values(freebsd12)' '--check-cfg=values(backtrace_in_libstd)' '--check-cfg=values(target_env,"libnx")' '--check-cfg=values(target_os,"watchos")' '--check-cfg=values(target_arch,"asmjs","spirv","nvptx","nvptx64","le32","xtensa")' '--check-cfg=values(dont_compile_me)' '--check-cfg=values(rustix_use_libc)' -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.68.0-nightly
  (33d3af4b5
  2023-01-19)' '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/")'` (exit status: 101)
