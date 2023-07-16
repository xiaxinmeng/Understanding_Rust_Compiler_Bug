plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:24cb9080177205b6e8c946b17badbe402adc938f)
Download action repository 'rust-lang/simpleinfra@master' (SHA:3fb2b44a4eaebb9ed8086446bde46c27199ef5ed)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---
failures:

---- [ui] tests/ui/async-await/in-trait/generics-mismatch.rs#next stdout ----

error in revision `next`: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/async-await/in-trait/generics-mismatch.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "next" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/in-trait/generics-mismatch.next" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/in-trait/generics-mismatch.next/auxiliary" "--edition=2021" "-Zlower-impl-trait-in-trait-to-assoc-ty"
stdout: none
--- stderr -------------------------------
error: internal compiler error: compiler/rustc_hir_analysis/src/astconv/mod.rs:3146:78: impossible case reached
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1644:9
stack backtrace:
   0:     0x7f46d1e8baf5 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h12560b3207919d28
   1:     0x7f46d1ef8cc8 - core::fmt::write::hc498be3e1bfb827d
   1:     0x7f46d1ef8cc8 - core::fmt::write::hc498be3e1bfb827d
   2:     0x7f46d1e801d1 - std::io::Write::write_fmt::h8943f61c2f6ad7cc
   3:     0x7f46d1e8b901 - std::sys_common::backtrace::print::hce135d0b682671cd
   4:     0x7f46d1e8ead4 - std::panicking::default_hook::{{closure}}::h7e230fc05b20d61b
   5:     0x7f46d1e8e7ba - std::panicking::default_hook::h5fbe04abb158874f
   6:     0x7f46d29885a5 - rustc_driver_impl[fecf5d64d50cc564]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f46d1e8f1f1 - std::panicking::rust_panic_with_hook::hde476276cde715b3
   8:     0x7f46d56f23e3 - std[ccc32538840c0f4d]::panicking::begin_panic::<rustc_errors[1b154bdf16b571d5]::ExplicitBug>::{closure#0}
   9:     0x7f46d56e82a6 - std[ccc32538840c0f4d]::sys_common::backtrace::__rust_end_short_backtrace::<std[ccc32538840c0f4d]::panicking::begin_panic<rustc_errors[1b154bdf16b571d5]::ExplicitBug>::{closure#0}, !>
  10:     0x7f46d291f1e6 - std[ccc32538840c0f4d]::panicking::begin_panic::<rustc_errors[1b154bdf16b571d5]::ExplicitBug>
  11:     0x7f46d576ee66 - std[ccc32538840c0f4d]::panic::panic_any::<rustc_errors[1b154bdf16b571d5]::ExplicitBug>
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
  12:     0x7f46d576ba40 - <rustc_errors[1b154bdf16b571d5]::HandlerInner>::bug::<&alloc[a2b0785807acf1c1]::string::String>
  13:     0x7f46d576b6d0 - <rustc_errors[1b154bdf16b571d5]::Handler>::bug::<&alloc[a2b0785807acf1c1]::string::String>
  14:     0x7f46d5796f65 - rustc_middle[fc4ba994b49ba12]::util::bug::opt_span_bug_fmt::<rustc_span[88f32648f8ad1dde]::span_encoding::Span>::{closure#0}
  15:     0x7f46d579693c - rustc_middle[fc4ba994b49ba12]::ty::context::tls::with_opt::<rustc_middle[fc4ba994b49ba12]::util::bug::opt_span_bug_fmt<rustc_span[88f32648f8ad1dde]::span_encoding::Span>::{closure#0}, !>::{closure#0}
  16:     0x7f46d57968e6 - rustc_middle[fc4ba994b49ba12]::ty::context::tls::with_context_opt::<rustc_middle[fc4ba994b49ba12]::ty::context::tls::with_opt<rustc_middle[fc4ba994b49ba12]::util::bug::opt_span_bug_fmt<rustc_span[88f32648f8ad1dde]::span_encoding::Span>::{closure#0}, !>::{closure#0}, !>
  17:     0x7f46d5796ea9 - rustc_middle[fc4ba994b49ba12]::util::bug::opt_span_bug_fmt::<rustc_span[88f32648f8ad1dde]::span_encoding::Span>
  18:     0x7f46d2929de5 - rustc_middle[fc4ba994b49ba12]::util::bug::bug_fmt
  19:     0x7f46d3262477 - <rustc_middle[fc4ba994b49ba12]::ty::list::List<rustc_middle[fc4ba994b49ba12]::ty::subst::GenericArg>>::fill_item::<<dyn rustc_hir_analysis[807da05960a02b02]::astconv::AstConv>::impl_trait_ty_to_ty::{closure#0}::{closure#0}>
  20:     0x7f46d326147f - <rustc_middle[fc4ba994b49ba12]::ty::list::List<rustc_middle[fc4ba994b49ba12]::ty::subst::GenericArg>>::for_item::<<dyn rustc_hir_analysis[807da05960a02b02]::astconv::AstConv>::impl_trait_ty_to_ty::{closure#0}::{closure#0}>
  21:     0x7f46d32ffeb7 - <dyn rustc_hir_analysis[807da05960a02b02]::astconv::AstConv>::impl_trait_ty_to_ty
  22:     0x7f46d32fe553 - <dyn rustc_hir_analysis[807da05960a02b02]::astconv::AstConv>::ast_ty_to_ty_inner
  23:     0x7f46d33011ba - <dyn rustc_hir_analysis[807da05960a02b02]::astconv::AstConv>::ty_of_fn
  24:     0x7f46d325b4f2 - rustc_hir_analysis[807da05960a02b02]::collect::fn_sig
  25:     0x7f46d4807a61 - rustc_query_system[6b78adb57212ea73]::query::plumbing::try_execute_query::<rustc_query_impl[6a12184f0518c8b4]::queries::fn_sig, rustc_query_impl[6a12184f0518c8b4]::plumbing::QueryCtxt>
  26:     0x7f46d449ba16 - <rustc_query_impl[6a12184f0518c8b4]::Queries as rustc_middle[fc4ba994b49ba12]::ty::query::QueryEngine>::fn_sig
  27:     0x7f46d3248b35 - <rustc_hir_analysis[807da05960a02b02]::collect::CollectItemTypesVisitor as rustc_hir[a00d09cabbcd8017]::intravisit::Visitor>::visit_trait_item
  28:     0x7f46d31eed40 - <rustc_middle[fc4ba994b49ba12]::hir::map::Map>::visit_item_likes_in_module::<rustc_hir_analysis[807da05960a02b02]::collect::CollectItemTypesVisitor>
  29:     0x7f46d32453dd - rustc_hir_analysis[807da05960a02b02]::collect::collect_mod_item_types
  30:     0x7f46d477a498 - rustc_query_system[6b78adb57212ea73]::query::plumbing::try_execute_query::<rustc_query_impl[6a12184f0518c8b4]::queries::collect_mod_item_types, rustc_query_impl[6a12184f0518c8b4]::plumbing::QueryCtxt>
  31:     0x7f46d44a3479 - <rustc_query_impl[6a12184f0518c8b4]::Queries as rustc_middle[fc4ba994b49ba12]::ty::query::QueryEngine>::collect_mod_item_types
  32:     0x7f46d31ee557 - <rustc_middle[fc4ba994b49ba12]::hir::map::Map>::for_each_module::<rustc_hir_analysis[807da05960a02b02]::check_crate::{closure#0}::{closure#0}::{closure#0}>
  33:     0x7f46d310c8bc - <rustc_session[347b3efbc836d0cd]::session::Session>::track_errors::<rustc_hir_analysis[807da05960a02b02]::check_crate::{closure#0}, ()>
  34:     0x7f46d324e4fb - rustc_hir_analysis[807da05960a02b02]::check_crate
  35:     0x7f46d2a5f4c8 - rustc_interface[65d844889b0970f5]::passes::analysis
  36:     0x7f46d480f8af - rustc_query_system[6b78adb57212ea73]::query::plumbing::try_execute_query::<rustc_query_impl[6a12184f0518c8b4]::queries::analysis, rustc_query_impl[6a12184f0518c8b4]::plumbing::QueryCtxt>
  37:     0x7f46d4471773 - <rustc_query_impl[6a12184f0518c8b4]::Queries as rustc_middle[fc4ba994b49ba12]::ty::query::QueryEngine>::analysis
  38:     0x7f46d298a443 - <rustc_middle[fc4ba994b49ba12]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[fecf5d64d50cc564]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[6fd02c7427d3a34f]::result::Result<(), rustc_span[88f32648f8ad1dde]::ErrorGuaranteed>>
  39:     0x7f46d29d6dd8 - <rustc_interface[65d844889b0970f5]::interface::Compiler>::enter::<rustc_driver_impl[fecf5d64d50cc564]::run_compiler::{closure#1}::{closure#2}, core[6fd02c7427d3a34f]::result::Result<core[6fd02c7427d3a34f]::option::Option<rustc_interface[65d844889b0970f5]::queries::Linker>, rustc_span[88f32648f8ad1dde]::ErrorGuaranteed>>
  40:     0x7f46d2990b78 - rustc_span[88f32648f8ad1dde]::with_source_map::<core[6fd02c7427d3a34f]::result::Result<(), rustc_span[88f32648f8ad1dde]::ErrorGuaranteed>, rustc_interface[65d844889b0970f5]::interface::run_compiler<core[6fd02c7427d3a34f]::result::Result<(), rustc_span[88f32648f8ad1dde]::ErrorGuaranteed>, rustc_driver_impl[fecf5d64d50cc564]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  41:     0x7f46d29c79a7 - std[ccc32538840c0f4d]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[65d844889b0970f5]::util::run_in_thread_pool_with_globals<rustc_interface[65d844889b0970f5]::interface::run_compiler<core[6fd02c7427d3a34f]::result::Result<(), rustc_span[88f32648f8ad1dde]::ErrorGuaranteed>, rustc_driver_impl[fecf5d64d50cc564]::run_compiler::{closure#1}>::{closure#0}, core[6fd02c7427d3a34f]::result::Result<(), rustc_span[88f32648f8ad1dde]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[6fd02c7427d3a34f]::result::Result<(), rustc_span[88f32648f8ad1dde]::ErrorGuaranteed>>
  42:     0x7f46d29ed2e6 - std[ccc32538840c0f4d]::panicking::try::<core[6fd02c7427d3a34f]::result::Result<(), rustc_span[88f32648f8ad1dde]::ErrorGuaranteed>, core[6fd02c7427d3a34f]::panic::unwind_safe::AssertUnwindSafe<<std[ccc32538840c0f4d]::thread::Builder>::spawn_unchecked_<rustc_interface[65d844889b0970f5]::util::run_in_thread_pool_with_globals<rustc_interface[65d844889b0970f5]::interface::run_compiler<core[6fd02c7427d3a34f]::result::Result<(), rustc_span[88f32648f8ad1dde]::ErrorGuaranteed>, rustc_driver_impl[fecf5d64d50cc564]::run_compiler::{closure#1}>::{closure#0}, core[6fd02c7427d3a34f]::result::Result<(), rustc_span[88f32648f8ad1dde]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[6fd02c7427d3a34f]::result::Result<(), rustc_span[88f32648f8ad1dde]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  43:     0x7f46d29950e5 - <<std[ccc32538840c0f4d]::thread::Builder>::spawn_unchecked_<rustc_interface[65d844889b0970f5]::util::run_in_thread_pool_with_globals<rustc_interface[65d844889b0970f5]::interface::run_compiler<core[6fd02c7427d3a34f]::result::Result<(), rustc_span[88f32648f8ad1dde]::ErrorGuaranteed>, rustc_driver_impl[fecf5d64d50cc564]::run_compiler::{closure#1}>::{closure#0}, core[6fd02c7427d3a34f]::result::Result<(), rustc_span[88f32648f8ad1dde]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[6fd02c7427d3a34f]::result::Result<(), rustc_span[88f32648f8ad1dde]::ErrorGuaranteed>>::{closure#1} as core[6fd02c7427d3a34f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  44:     0x7f46d1e9b4de - std::sys::unix::thread::Thread::new::thread_start::hb789e17fea9a089b
  45:     0x7f46d1c35b43 - <unknown>
  46:     0x7f46d1cc7a00 - <unknown>
  47:                0x0 - <unknown>
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.70.0-nightly (9c57d5c9b 2023-03-17) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 -Z lower-impl-trait-in-trait-to-assoc-ty
query stack during panic:
query stack during panic:
#0 [fn_sig] computing function signature of `Foo::foo`
#1 [collect_mod_item_types] collecting item types in top-level module
#2 [analysis] running analysis passes on this crate
error: aborting due to previous error
------------------------------------------


