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
error: internal compiler error: compiler/rustc_hir_analysis/src/astconv/mod.rs:3157:78: impossible case reached
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1644:9
stack backtrace:
stack backtrace:
   0:     0x7f84360f1b35 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h508522f9de4a85d0
   1:     0x7f843615ebc8 - core::fmt::write::h3cc9915f12734699
   2:     0x7f84360e63f1 - std::io::Write::write_fmt::hfc39fbcc2e314c76
   3:     0x7f84360f1941 - std::sys_common::backtrace::print::h56f4184070bbc461
   4:     0x7f84360f4b14 - std::panicking::default_hook::{{closure}}::h7b923d63d46d6a51
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   5:     0x7f84360f47fa - std::panicking::default_hook::h1f48b2141acfff4a
   6:     0x7f8436beafa5 - rustc_driver_impl[7a99ead85f88e6e3]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f84360f5231 - std::panicking::rust_panic_with_hook::h7f971ab3ce04a94e
   8:     0x7f8439938713 - std[308602202b5ed449]::panicking::begin_panic::<rustc_errors[f51f6e6e10446e85]::ExplicitBug>::{closure#0}
   9:     0x7f843992e476 - std[308602202b5ed449]::sys_common::backtrace::__rust_end_short_backtrace::<std[308602202b5ed449]::panicking::begin_panic<rustc_errors[f51f6e6e10446e85]::ExplicitBug>::{closure#0}, !>
  10:     0x7f8436b82366 - std[308602202b5ed449]::panicking::begin_panic::<rustc_errors[f51f6e6e10446e85]::ExplicitBug>
  11:     0x7f84399b3a16 - std[308602202b5ed449]::panic::panic_any::<rustc_errors[f51f6e6e10446e85]::ExplicitBug>
  12:     0x7f84399b1820 - <rustc_errors[f51f6e6e10446e85]::HandlerInner>::bug::<&alloc[c83b324fe061851]::string::String>
  13:     0x7f84399b14b0 - <rustc_errors[f51f6e6e10446e85]::Handler>::bug::<&alloc[c83b324fe061851]::string::String>
  14:     0x7f84399dcd75 - rustc_middle[5667ae8d5ee3cb83]::util::bug::opt_span_bug_fmt::<rustc_span[939028fdbb561ab1]::span_encoding::Span>::{closure#0}
  15:     0x7f84399dc74c - rustc_middle[5667ae8d5ee3cb83]::ty::context::tls::with_opt::<rustc_middle[5667ae8d5ee3cb83]::util::bug::opt_span_bug_fmt<rustc_span[939028fdbb561ab1]::span_encoding::Span>::{closure#0}, !>::{closure#0}
  16:     0x7f84399dc6f6 - rustc_middle[5667ae8d5ee3cb83]::ty::context::tls::with_context_opt::<rustc_middle[5667ae8d5ee3cb83]::ty::context::tls::with_opt<rustc_middle[5667ae8d5ee3cb83]::util::bug::opt_span_bug_fmt<rustc_span[939028fdbb561ab1]::span_encoding::Span>::{closure#0}, !>::{closure#0}, !>
  17:     0x7f84399dccb9 - rustc_middle[5667ae8d5ee3cb83]::util::bug::opt_span_bug_fmt::<rustc_span[939028fdbb561ab1]::span_encoding::Span>
  18:     0x7f8436b8ce35 - rustc_middle[5667ae8d5ee3cb83]::util::bug::bug_fmt
  19:     0x7f84374bc107 - <rustc_middle[5667ae8d5ee3cb83]::ty::list::List<rustc_middle[5667ae8d5ee3cb83]::ty::subst::GenericArg>>::fill_item::<<dyn rustc_hir_analysis[d981736e8295d922]::astconv::AstConv>::impl_trait_ty_to_ty::{closure#0}::{closure#0}>
  20:     0x7f84374bb10f - <rustc_middle[5667ae8d5ee3cb83]::ty::list::List<rustc_middle[5667ae8d5ee3cb83]::ty::subst::GenericArg>>::for_item::<<dyn rustc_hir_analysis[d981736e8295d922]::astconv::AstConv>::impl_trait_ty_to_ty::{closure#0}::{closure#0}>
  21:     0x7f8437559b87 - <dyn rustc_hir_analysis[d981736e8295d922]::astconv::AstConv>::impl_trait_ty_to_ty
  22:     0x7f8437558223 - <dyn rustc_hir_analysis[d981736e8295d922]::astconv::AstConv>::ast_ty_to_ty_inner
  23:     0x7f843755ae8a - <dyn rustc_hir_analysis[d981736e8295d922]::astconv::AstConv>::ty_of_fn
  24:     0x7f84374b5182 - rustc_hir_analysis[d981736e8295d922]::collect::fn_sig
  25:     0x7f8438a53368 - rustc_query_system[ccf74b58e89af4c5]::query::plumbing::try_execute_query::<rustc_query_impl[72c04503f72f3b0]::queries::fn_sig, rustc_query_impl[72c04503f72f3b0]::plumbing::QueryCtxt>
  26:     0x7f84386e87b6 - <rustc_query_impl[72c04503f72f3b0]::Queries as rustc_middle[5667ae8d5ee3cb83]::ty::query::QueryEngine>::fn_sig
  27:     0x7f84374a27c5 - <rustc_hir_analysis[d981736e8295d922]::collect::CollectItemTypesVisitor as rustc_hir[6084683baa469896]::intravisit::Visitor>::visit_trait_item
  28:     0x7f8437448a50 - <rustc_middle[5667ae8d5ee3cb83]::hir::map::Map>::visit_item_likes_in_module::<rustc_hir_analysis[d981736e8295d922]::collect::CollectItemTypesVisitor>
  29:     0x7f843749f06d - rustc_hir_analysis[d981736e8295d922]::collect::collect_mod_item_types
  30:     0x7f84389c6419 - rustc_query_system[ccf74b58e89af4c5]::query::plumbing::try_execute_query::<rustc_query_impl[72c04503f72f3b0]::queries::collect_mod_item_types, rustc_query_impl[72c04503f72f3b0]::plumbing::QueryCtxt>
  31:     0x7f84386f0219 - <rustc_query_impl[72c04503f72f3b0]::Queries as rustc_middle[5667ae8d5ee3cb83]::ty::query::QueryEngine>::collect_mod_item_types
  32:     0x7f8437448267 - <rustc_middle[5667ae8d5ee3cb83]::hir::map::Map>::for_each_module::<rustc_hir_analysis[d981736e8295d922]::check_crate::{closure#0}::{closure#0}::{closure#0}>
  33:     0x7f8437366a8c - <rustc_session[623ed47862729687]::session::Session>::track_errors::<rustc_hir_analysis[d981736e8295d922]::check_crate::{closure#0}, ()>
  34:     0x7f84374a818b - rustc_hir_analysis[d981736e8295d922]::check_crate
  35:     0x7f8436cbe678 - rustc_interface[416d337036bd2e35]::passes::analysis
  36:     0x7f8438a5b219 - rustc_query_system[ccf74b58e89af4c5]::query::plumbing::try_execute_query::<rustc_query_impl[72c04503f72f3b0]::queries::analysis, rustc_query_impl[72c04503f72f3b0]::plumbing::QueryCtxt>
  37:     0x7f84386be513 - <rustc_query_impl[72c04503f72f3b0]::Queries as rustc_middle[5667ae8d5ee3cb83]::ty::query::QueryEngine>::analysis
  38:     0x7f8436bece23 - <rustc_middle[5667ae8d5ee3cb83]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[7a99ead85f88e6e3]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[9cfff55fe570b32]::result::Result<(), rustc_span[939028fdbb561ab1]::ErrorGuaranteed>>
  39:     0x7f8436c389a8 - <rustc_interface[416d337036bd2e35]::interface::Compiler>::enter::<rustc_driver_impl[7a99ead85f88e6e3]::run_compiler::{closure#1}::{closure#2}, core[9cfff55fe570b32]::result::Result<core[9cfff55fe570b32]::option::Option<rustc_interface[416d337036bd2e35]::queries::Linker>, rustc_span[939028fdbb561ab1]::ErrorGuaranteed>>
  40:     0x7f8436bf9158 - rustc_span[939028fdbb561ab1]::with_source_map::<core[9cfff55fe570b32]::result::Result<(), rustc_span[939028fdbb561ab1]::ErrorGuaranteed>, rustc_interface[416d337036bd2e35]::interface::run_compiler<core[9cfff55fe570b32]::result::Result<(), rustc_span[939028fdbb561ab1]::ErrorGuaranteed>, rustc_driver_impl[7a99ead85f88e6e3]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  41:     0x7f8436c29497 - std[308602202b5ed449]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[416d337036bd2e35]::util::run_in_thread_pool_with_globals<rustc_interface[416d337036bd2e35]::interface::run_compiler<core[9cfff55fe570b32]::result::Result<(), rustc_span[939028fdbb561ab1]::ErrorGuaranteed>, rustc_driver_impl[7a99ead85f88e6e3]::run_compiler::{closure#1}>::{closure#0}, core[9cfff55fe570b32]::result::Result<(), rustc_span[939028fdbb561ab1]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[9cfff55fe570b32]::result::Result<(), rustc_span[939028fdbb561ab1]::ErrorGuaranteed>>
  42:     0x7f8436c3af26 - std[308602202b5ed449]::panicking::try::<core[9cfff55fe570b32]::result::Result<(), rustc_span[939028fdbb561ab1]::ErrorGuaranteed>, core[9cfff55fe570b32]::panic::unwind_safe::AssertUnwindSafe<<std[308602202b5ed449]::thread::Builder>::spawn_unchecked_<rustc_interface[416d337036bd2e35]::util::run_in_thread_pool_with_globals<rustc_interface[416d337036bd2e35]::interface::run_compiler<core[9cfff55fe570b32]::result::Result<(), rustc_span[939028fdbb561ab1]::ErrorGuaranteed>, rustc_driver_impl[7a99ead85f88e6e3]::run_compiler::{closure#1}>::{closure#0}, core[9cfff55fe570b32]::result::Result<(), rustc_span[939028fdbb561ab1]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[9cfff55fe570b32]::result::Result<(), rustc_span[939028fdbb561ab1]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  43:     0x7f8436bf6c65 - <<std[308602202b5ed449]::thread::Builder>::spawn_unchecked_<rustc_interface[416d337036bd2e35]::util::run_in_thread_pool_with_globals<rustc_interface[416d337036bd2e35]::interface::run_compiler<core[9cfff55fe570b32]::result::Result<(), rustc_span[939028fdbb561ab1]::ErrorGuaranteed>, rustc_driver_impl[7a99ead85f88e6e3]::run_compiler::{closure#1}>::{closure#0}, core[9cfff55fe570b32]::result::Result<(), rustc_span[939028fdbb561ab1]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[9cfff55fe570b32]::result::Result<(), rustc_span[939028fdbb561ab1]::ErrorGuaranteed>>::{closure#1} as core[9cfff55fe570b32]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  44:     0x7f843610147e - std::sys::unix::thread::Thread::new::thread_start::h88e21b4fe8b9342f
  45:     0x7f8435e9bb43 - <unknown>
  46:     0x7f8435f2da00 - <unknown>
  47:                0x0 - <unknown>
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.70.0-nightly (b68faf07d 2023-03-17) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 -Z lower-impl-trait-in-trait-to-assoc-ty
query stack during panic:
query stack during panic:
#0 [fn_sig] computing function signature of `Foo::foo`
#1 [collect_mod_item_types] collecting item types in top-level module
#2 [analysis] running analysis passes on this crate
error: aborting due to previous error
------------------------------------------


