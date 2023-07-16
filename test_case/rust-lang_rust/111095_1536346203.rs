plain
Documenting book redirect pages (x86_64-unknown-linux-gnu)
Documenting standalone (x86_64-unknown-linux-gnu)
Documenting stage2 library (x86_64-unknown-linux-gnu) in HTML format
 Documenting core v0.0.0 (/checkout/library/core)
thread 'rustc' panicked at 'parent_stack is empty', src/librustdoc/formats/cache.rs:276:34
   0:     0x7f73c1d174b4 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha94d50d3c003a464
   1:     0x7f73c1d7e8e8 - core::fmt::write::h1d3ce6205605d080
   2:     0x7f73c1d0ba81 - std::io::Write::write_fmt::h53f0a99af878e183
   3:     0x7f73c1d172c1 - std::sys_common::backtrace::print::h47ee8e7e58f6c4e2
   3:     0x7f73c1d172c1 - std::sys_common::backtrace::print::h47ee8e7e58f6c4e2
   4:     0x7f73c1d1a44a - std::panicking::default_hook::{{closure}}::h2a576359d7ed6bcd
   5:     0x7f73c1d1a12c - std::panicking::default_hook::h616cce22a64c7052
   6:     0x7f73c27dd595 - rustc_driver_impl[f531d33270b3eaa0]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f73c1d1ab57 - std::panicking::rust_panic_with_hook::h18ad6a174f0dd4ee
   8:     0x7f73c1d1a8d7 - std::panicking::begin_panic_handler::{{closure}}::h589f42727a626297
   9:     0x7f73c1d17996 - std::sys_common::backtrace::__rust_end_short_backtrace::hc2bc5d8ccaa8b94f
  10:     0x7f73c1d1a5c7 - rust_begin_unwind
  11:     0x7f73c1ccf0c3 - core::panicking::panic_fmt::h920d75942dbc736c
  12:     0x7f73c1ccf083 - core::option::expect_failed::hc6f3fcc1cd7f855e
  13:     0x5602a496c33a - <rustdoc[c3c59795d77ff1be]::formats::cache::CacheBuilder as rustdoc[c3c59795d77ff1be]::fold::DocFolder>::fold_item
  14:     0x5602a4bbc6b1 - <alloc[6708b2931850d87c]::vec::Vec<rustdoc[c3c59795d77ff1be]::clean::types::Item> as alloc[6708b2931850d87c]::vec::spec_from_iter::SpecFromIter<rustdoc[c3c59795d77ff1be]::clean::types::Item, core[c67d3e033db4f200]::iter::adapters::filter_map::FilterMap<alloc[6708b2931850d87c]::vec::into_iter::IntoIter<rustdoc[c3c59795d77ff1be]::clean::types::Item>, <rustdoc[c3c59795d77ff1be]::formats::cache::CacheBuilder as rustdoc[c3c59795d77ff1be]::fold::DocFolder>::fold_crate::{closure#0}>>>::from_iter
  15:     0x5602a49684e5 - <rustdoc[c3c59795d77ff1be]::formats::cache::CacheBuilder as rustdoc[c3c59795d77ff1be]::fold::DocFolder>::fold_inner_recur
  16:     0x5602a496896e - <rustdoc[c3c59795d77ff1be]::formats::cache::CacheBuilder as rustdoc[c3c59795d77ff1be]::fold::DocFolder>::fold_item_recur
  17:     0x5602a496b31b - <rustdoc[c3c59795d77ff1be]::formats::cache::CacheBuilder as rustdoc[c3c59795d77ff1be]::fold::DocFolder>::fold_item
  18:     0x5602a4bbc6b1 - <alloc[6708b2931850d87c]::vec::Vec<rustdoc[c3c59795d77ff1be]::clean::types::Item> as alloc[6708b2931850d87c]::vec::spec_from_iter::SpecFromIter<rustdoc[c3c59795d77ff1be]::clean::types::Item, core[c67d3e033db4f200]::iter::adapters::filter_map::FilterMap<alloc[6708b2931850d87c]::vec::into_iter::IntoIter<rustdoc[c3c59795d77ff1be]::clean::types::Item>, <rustdoc[c3c59795d77ff1be]::formats::cache::CacheBuilder as rustdoc[c3c59795d77ff1be]::fold::DocFolder>::fold_crate::{closure#0}>>>::from_iter
  19:     0x5602a4968618 - <rustdoc[c3c59795d77ff1be]::formats::cache::CacheBuilder as rustdoc[c3c59795d77ff1be]::fold::DocFolder>::fold_inner_recur
  20:     0x5602a496896e - <rustdoc[c3c59795d77ff1be]::formats::cache::CacheBuilder as rustdoc[c3c59795d77ff1be]::fold::DocFolder>::fold_item_recur
  21:     0x5602a496b31b - <rustdoc[c3c59795d77ff1be]::formats::cache::CacheBuilder as rustdoc[c3c59795d77ff1be]::fold::DocFolder>::fold_item
  22:     0x5602a4bbc6b1 - <alloc[6708b2931850d87c]::vec::Vec<rustdoc[c3c59795d77ff1be]::clean::types::Item> as alloc[6708b2931850d87c]::vec::spec_from_iter::SpecFromIter<rustdoc[c3c59795d77ff1be]::clean::types::Item, core[c67d3e033db4f200]::iter::adapters::filter_map::FilterMap<alloc[6708b2931850d87c]::vec::into_iter::IntoIter<rustdoc[c3c59795d77ff1be]::clean::types::Item>, <rustdoc[c3c59795d77ff1be]::formats::cache::CacheBuilder as rustdoc[c3c59795d77ff1be]::fold::DocFolder>::fold_crate::{closure#0}>>>::from_iter
  23:     0x5602a4968618 - <rustdoc[c3c59795d77ff1be]::formats::cache::CacheBuilder as rustdoc[c3c59795d77ff1be]::fold::DocFolder>::fold_inner_recur
  24:     0x5602a49689ed - <rustdoc[c3c59795d77ff1be]::formats::cache::CacheBuilder as rustdoc[c3c59795d77ff1be]::fold::DocFolder>::fold_item_recur
  25:     0x5602a496b31b - <rustdoc[c3c59795d77ff1be]::formats::cache::CacheBuilder as rustdoc[c3c59795d77ff1be]::fold::DocFolder>::fold_item
  26:     0x5602a4bbc6b1 - <alloc[6708b2931850d87c]::vec::Vec<rustdoc[c3c59795d77ff1be]::clean::types::Item> as alloc[6708b2931850d87c]::vec::spec_from_iter::SpecFromIter<rustdoc[c3c59795d77ff1be]::clean::types::Item, core[c67d3e033db4f200]::iter::adapters::filter_map::FilterMap<alloc[6708b2931850d87c]::vec::into_iter::IntoIter<rustdoc[c3c59795d77ff1be]::clean::types::Item>, <rustdoc[c3c59795d77ff1be]::formats::cache::CacheBuilder as rustdoc[c3c59795d77ff1be]::fold::DocFolder>::fold_crate::{closure#0}>>>::from_iter
  27:     0x5602a4968618 - <rustdoc[c3c59795d77ff1be]::formats::cache::CacheBuilder as rustdoc[c3c59795d77ff1be]::fold::DocFolder>::fold_inner_recur
  28:     0x5602a49689ed - <rustdoc[c3c59795d77ff1be]::formats::cache::CacheBuilder as rustdoc[c3c59795d77ff1be]::fold::DocFolder>::fold_item_recur
  29:     0x5602a496b31b - <rustdoc[c3c59795d77ff1be]::formats::cache::CacheBuilder as rustdoc[c3c59795d77ff1be]::fold::DocFolder>::fold_item
  30:     0x5602a49694f2 - <rustdoc[c3c59795d77ff1be]::formats::cache::Cache>::populate
  31:     0x5602a48a271c - <rustc_session[3dce2949159c0677]::session::Session>::time::<rustdoc[c3c59795d77ff1be]::clean::types::Crate, rustdoc[c3c59795d77ff1be]::core::run_global_ctxt::{closure#10}>
  32:     0x5602a49b4fff - rustdoc[c3c59795d77ff1be]::core::run_global_ctxt
  33:     0x5602a48a2853 - <rustc_session[3dce2949159c0677]::session::Session>::time::<(rustdoc[c3c59795d77ff1be]::clean::types::Crate, rustdoc[c3c59795d77ff1be]::config::RenderOptions, rustdoc[c3c59795d77ff1be]::formats::cache::Cache), rustdoc[c3c59795d77ff1be]::main_args::{closure#1}::{closure#0}::{closure#0}::{closure#0}>
  34:     0x5602a49ef93b - <rustc_middle[913ae17afe5ed594]::ty::context::GlobalCtxt>::enter::<rustdoc[c3c59795d77ff1be]::main_args::{closure#1}::{closure#0}::{closure#0}, core[c67d3e033db4f200]::result::Result<(), rustc_span[c04a3b7f38c31f99]::ErrorGuaranteed>>
  35:     0x5602a492078c - <rustc_interface[52e3334fe4da8722]::queries::QueryResult<&rustc_middle[913ae17afe5ed594]::ty::context::GlobalCtxt>>::enter::<core[c67d3e033db4f200]::result::Result<(), rustc_span[c04a3b7f38c31f99]::ErrorGuaranteed>, rustdoc[c3c59795d77ff1be]::main_args::{closure#1}::{closure#0}::{closure#0}>
  36:     0x5602a4a8e53d - <rustc_interface[52e3334fe4da8722]::interface::Compiler>::enter::<rustdoc[c3c59795d77ff1be]::main_args::{closure#1}::{closure#0}, core[c67d3e033db4f200]::result::Result<(), rustc_span[c04a3b7f38c31f99]::ErrorGuaranteed>>
  37:     0x5602a49baabc - rustc_span[c04a3b7f38c31f99]::set_source_map::<core[c67d3e033db4f200]::result::Result<(), rustc_span[c04a3b7f38c31f99]::ErrorGuaranteed>, rustc_interface[52e3334fe4da8722]::interface::run_compiler<core[c67d3e033db4f200]::result::Result<(), rustc_span[c04a3b7f38c31f99]::ErrorGuaranteed>, rustdoc[c3c59795d77ff1be]::main_args::{closure#1}>::{closure#0}::{closure#0}>
  38:     0x5602a497e365 - <scoped_tls[2f9bb92df82c0cbd]::ScopedKey<rustc_span[c04a3b7f38c31f99]::SessionGlobals>>::set::<rustc_interface[52e3334fe4da8722]::interface::run_compiler<core[c67d3e033db4f200]::result::Result<(), rustc_span[c04a3b7f38c31f99]::ErrorGuaranteed>, rustdoc[c3c59795d77ff1be]::main_args::{closure#1}>::{closure#0}, core[c67d3e033db4f200]::result::Result<(), rustc_span[c04a3b7f38c31f99]::ErrorGuaranteed>>
  39:     0x5602a4856db6 - std[69b2d3e3b20adbfe]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[52e3334fe4da8722]::util::run_in_thread_pool_with_globals<rustc_interface[52e3334fe4da8722]::interface::run_compiler<core[c67d3e033db4f200]::result::Result<(), rustc_span[c04a3b7f38c31f99]::ErrorGuaranteed>, rustdoc[c3c59795d77ff1be]::main_args::{closure#1}>::{closure#0}, core[c67d3e033db4f200]::result::Result<(), rustc_span[c04a3b7f38c31f99]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[c67d3e033db4f200]::result::Result<(), rustc_span[c04a3b7f38c31f99]::ErrorGuaranteed>>
  40:     0x5602a494e769 - <<std[69b2d3e3b20adbfe]::thread::Builder>::spawn_unchecked_<rustc_interface[52e3334fe4da8722]::util::run_in_thread_pool_with_globals<rustc_interface[52e3334fe4da8722]::interface::run_compiler<core[c67d3e033db4f200]::result::Result<(), rustc_span[c04a3b7f38c31f99]::ErrorGuaranteed>, rustdoc[c3c59795d77ff1be]::main_args::{closure#1}>::{closure#0}, core[c67d3e033db4f200]::result::Result<(), rustc_span[c04a3b7f38c31f99]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[c67d3e033db4f200]::result::Result<(), rustc_span[c04a3b7f38c31f99]::ErrorGuaranteed>>::{closure#1} as core[c67d3e033db4f200]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  41:     0x7f73c1d2749e - std::sys::unix::thread::Thread::new::thread_start::h5bc7e5ba3300a44e
  42:     0x7f73c19b8b43 - <unknown>
  43:     0x7f73c1a4aa00 - <unknown>
  44:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.71.0-nightly (795b06f43 2023-05-05) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -Z unstable-options -Z unstable-options -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z force-unstable-if-unmarked -Z unstable-options
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
end of query stack
error: could not document `core`

Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2021 --crate-type lib --crate-name core library/core/src/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/doc -Zunstable-options --check-cfg 'names()' --check-cfg 'values()' --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat -Z unstable-options --resource-suffix 1.71.0 --markdown-css rust.css --markdown-no-toc --index-page /checkout/src/doc/index.md -C metadata=b1f251f1f3d92979 -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/x86_64-unknown-linux-gnu/doc/release/deps -Csymbol-mangling-version=legacy -Zunstable-options -Zunstable-options '--check-cfg=values(bootstrap)' '--check-cfg=values(stdarch_intel_sde)' '--check-cfg=values(no_fp_fmt_parse)' '--check-cfg=values(no_global_oom_handling)' '--check-cfg=values(no_rc)' '--check-cfg=values(no_sync)' '--check-cfg=values(freebsd12)' '--check-cfg=values(freebsd13)' '--check-cfg=values(backtrace_in_libstd)' '--check-cfg=values(target_env,"libnx")' '--check-cfg=values(target_arch,"asmjs","spirv","nvptx","xtensa")' -Dwarnings '-Wrustdoc::invalid_codeblock_attributes' --crate-version '1.71.0-nightly
  (795b06f43
  2023-05-05)' '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/")'` (exit status: 101)
