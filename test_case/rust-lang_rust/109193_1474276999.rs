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
   0:     0x7fa964803b35 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h017520516404900f
   1:     0x7fa964870bc8 - core::fmt::write::hd31d67b1768c354c
   1:     0x7fa964870bc8 - core::fmt::write::hd31d67b1768c354c
   2:     0x7fa9647f83f1 - std::io::Write::write_fmt::he26e47b8ce470782
   3:     0x7fa964803941 - std::sys_common::backtrace::print::h4631a5ee47680122
   4:     0x7fa964806b14 - std::panicking::default_hook::{{closure}}::h11a6f6327ea5df0f
   5:     0x7fa9648067fa - std::panicking::default_hook::h4d97c97b0a6829c1
   6:     0x7fa9652fe755 - rustc_driver_impl[55608a866b050417]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fa964807231 - std::panicking::rust_panic_with_hook::h14c9ef1a4df04a28
   8:     0x7fa96804f193 - std[2edd8eb7d496cdca]::panicking::begin_panic::<rustc_errors[47f9c8d9db9806d9]::ExplicitBug>::{closure#0}
   9:     0x7fa968044f06 - std[2edd8eb7d496cdca]::sys_common::backtrace::__rust_end_short_backtrace::<std[2edd8eb7d496cdca]::panicking::begin_panic<rustc_errors[47f9c8d9db9806d9]::ExplicitBug>::{closure#0}, !>
  10:     0x7fa965295b16 - std[2edd8eb7d496cdca]::panicking::begin_panic::<rustc_errors[47f9c8d9db9806d9]::ExplicitBug>
  11:     0x7fa9680cb4a6 - std[2edd8eb7d496cdca]::panic::panic_any::<rustc_errors[47f9c8d9db9806d9]::ExplicitBug>
  12:     0x7fa9680c80d0 - <rustc_errors[47f9c8d9db9806d9]::HandlerInner>::bug::<&alloc[cb70cfa673cbe62c]::string::String>
  13:     0x7fa9680c7d60 - <rustc_errors[47f9c8d9db9806d9]::Handler>::bug::<&alloc[cb70cfa673cbe62c]::string::String>
  14:     0x7fa9680f31b5 - rustc_middle[96527d156dc263f2]::util::bug::opt_span_bug_fmt::<rustc_span[937acc1ada1d69b0]::span_encoding::Span>::{closure#0}
  15:     0x7fa9680f2b8c - rustc_middle[96527d156dc263f2]::ty::context::tls::with_opt::<rustc_middle[96527d156dc263f2]::util::bug::opt_span_bug_fmt<rustc_span[937acc1ada1d69b0]::span_encoding::Span>::{closure#0}, !>::{closure#0}
  16:     0x7fa9680f2b36 - rustc_middle[96527d156dc263f2]::ty::context::tls::with_context_opt::<rustc_middle[96527d156dc263f2]::ty::context::tls::with_opt<rustc_middle[96527d156dc263f2]::util::bug::opt_span_bug_fmt<rustc_span[937acc1ada1d69b0]::span_encoding::Span>::{closure#0}, !>::{closure#0}, !>
  17:     0x7fa9680f30f9 - rustc_middle[96527d156dc263f2]::util::bug::opt_span_bug_fmt::<rustc_span[937acc1ada1d69b0]::span_encoding::Span>
  18:     0x7fa9652a05e5 - rustc_middle[96527d156dc263f2]::util::bug::bug_fmt
  19:     0x7fa965bcf667 - <rustc_middle[96527d156dc263f2]::ty::list::List<rustc_middle[96527d156dc263f2]::ty::subst::GenericArg>>::fill_item::<<dyn rustc_hir_analysis[59084a90c6cce162]::astconv::AstConv>::impl_trait_ty_to_ty::{closure#0}::{closure#0}>
  20:     0x7fa965bce66f - <rustc_middle[96527d156dc263f2]::ty::list::List<rustc_middle[96527d156dc263f2]::ty::subst::GenericArg>>::for_item::<<dyn rustc_hir_analysis[59084a90c6cce162]::astconv::AstConv>::impl_trait_ty_to_ty::{closure#0}::{closure#0}>
  21:     0x7fa965c6d147 - <dyn rustc_hir_analysis[59084a90c6cce162]::astconv::AstConv>::impl_trait_ty_to_ty
  22:     0x7fa965c6b7e3 - <dyn rustc_hir_analysis[59084a90c6cce162]::astconv::AstConv>::ast_ty_to_ty_inner
  23:     0x7fa965c6e44a - <dyn rustc_hir_analysis[59084a90c6cce162]::astconv::AstConv>::ty_of_fn
  24:     0x7fa965bc86e2 - rustc_hir_analysis[59084a90c6cce162]::collect::fn_sig
  25:     0x7fa967167018 - rustc_query_system[49ae23cbe0ce811a]::query::plumbing::try_execute_query::<rustc_query_impl[c4b7d3387bef2edb]::queries::fn_sig, rustc_query_impl[c4b7d3387bef2edb]::plumbing::QueryCtxt>
  26:     0x7fa966dfc316 - <rustc_query_impl[c4b7d3387bef2edb]::Queries as rustc_middle[96527d156dc263f2]::ty::query::QueryEngine>::fn_sig
  27:     0x7fa965bb5d25 - <rustc_hir_analysis[59084a90c6cce162]::collect::CollectItemTypesVisitor as rustc_hir[8327ff18bacabb25]::intravisit::Visitor>::visit_trait_item
  28:     0x7fa965b5c090 - <rustc_middle[96527d156dc263f2]::hir::map::Map>::visit_item_likes_in_module::<rustc_hir_analysis[59084a90c6cce162]::collect::CollectItemTypesVisitor>
  29:     0x7fa965bb25cd - rustc_hir_analysis[59084a90c6cce162]::collect::collect_mod_item_types
  30:     0x7fa9670da0c9 - rustc_query_system[49ae23cbe0ce811a]::query::plumbing::try_execute_query::<rustc_query_impl[c4b7d3387bef2edb]::queries::collect_mod_item_types, rustc_query_impl[c4b7d3387bef2edb]::plumbing::QueryCtxt>
  31:     0x7fa966e03d79 - <rustc_query_impl[c4b7d3387bef2edb]::Queries as rustc_middle[96527d156dc263f2]::ty::query::QueryEngine>::collect_mod_item_types
  32:     0x7fa965b5b8a7 - <rustc_middle[96527d156dc263f2]::hir::map::Map>::for_each_module::<rustc_hir_analysis[59084a90c6cce162]::check_crate::{closure#0}::{closure#0}::{closure#0}>
  33:     0x7fa965a7a16c - <rustc_session[dea3f2117f899572]::session::Session>::track_errors::<rustc_hir_analysis[59084a90c6cce162]::check_crate::{closure#0}, ()>
  34:     0x7fa965bbb6eb - rustc_hir_analysis[59084a90c6cce162]::check_crate
  35:     0x7fa9653d1a48 - rustc_interface[f4bde108fc82817f]::passes::analysis
  36:     0x7fa96716eec9 - rustc_query_system[49ae23cbe0ce811a]::query::plumbing::try_execute_query::<rustc_query_impl[c4b7d3387bef2edb]::queries::analysis, rustc_query_impl[c4b7d3387bef2edb]::plumbing::QueryCtxt>
  37:     0x7fa966dd2073 - <rustc_query_impl[c4b7d3387bef2edb]::Queries as rustc_middle[96527d156dc263f2]::ty::query::QueryEngine>::analysis
  38:     0x7fa965300173 - <rustc_middle[96527d156dc263f2]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[55608a866b050417]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[29afcc4231d37838]::result::Result<(), rustc_span[937acc1ada1d69b0]::ErrorGuaranteed>>
  39:     0x7fa96534c308 - <rustc_interface[f4bde108fc82817f]::interface::Compiler>::enter::<rustc_driver_impl[55608a866b050417]::run_compiler::{closure#1}::{closure#2}, core[29afcc4231d37838]::result::Result<core[29afcc4231d37838]::option::Option<rustc_interface[f4bde108fc82817f]::queries::Linker>, rustc_span[937acc1ada1d69b0]::ErrorGuaranteed>>
  40:     0x7fa96530c978 - rustc_span[937acc1ada1d69b0]::with_source_map::<core[29afcc4231d37838]::result::Result<(), rustc_span[937acc1ada1d69b0]::ErrorGuaranteed>, rustc_interface[f4bde108fc82817f]::interface::run_compiler<core[29afcc4231d37838]::result::Result<(), rustc_span[937acc1ada1d69b0]::ErrorGuaranteed>, rustc_driver_impl[55608a866b050417]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  41:     0x7fa96533ceb7 - std[2edd8eb7d496cdca]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[f4bde108fc82817f]::util::run_in_thread_pool_with_globals<rustc_interface[f4bde108fc82817f]::interface::run_compiler<core[29afcc4231d37838]::result::Result<(), rustc_span[937acc1ada1d69b0]::ErrorGuaranteed>, rustc_driver_impl[55608a866b050417]::run_compiler::{closure#1}>::{closure#0}, core[29afcc4231d37838]::result::Result<(), rustc_span[937acc1ada1d69b0]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[29afcc4231d37838]::result::Result<(), rustc_span[937acc1ada1d69b0]::ErrorGuaranteed>>
  42:     0x7fa965361226 - std[2edd8eb7d496cdca]::panicking::try::<core[29afcc4231d37838]::result::Result<(), rustc_span[937acc1ada1d69b0]::ErrorGuaranteed>, core[29afcc4231d37838]::panic::unwind_safe::AssertUnwindSafe<<std[2edd8eb7d496cdca]::thread::Builder>::spawn_unchecked_<rustc_interface[f4bde108fc82817f]::util::run_in_thread_pool_with_globals<rustc_interface[f4bde108fc82817f]::interface::run_compiler<core[29afcc4231d37838]::result::Result<(), rustc_span[937acc1ada1d69b0]::ErrorGuaranteed>, rustc_driver_impl[55608a866b050417]::run_compiler::{closure#1}>::{closure#0}, core[29afcc4231d37838]::result::Result<(), rustc_span[937acc1ada1d69b0]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[29afcc4231d37838]::result::Result<(), rustc_span[937acc1ada1d69b0]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  43:     0x7fa96530a495 - <<std[2edd8eb7d496cdca]::thread::Builder>::spawn_unchecked_<rustc_interface[f4bde108fc82817f]::util::run_in_thread_pool_with_globals<rustc_interface[f4bde108fc82817f]::interface::run_compiler<core[29afcc4231d37838]::result::Result<(), rustc_span[937acc1ada1d69b0]::ErrorGuaranteed>, rustc_driver_impl[55608a866b050417]::run_compiler::{closure#1}>::{closure#0}, core[29afcc4231d37838]::result::Result<(), rustc_span[937acc1ada1d69b0]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[29afcc4231d37838]::result::Result<(), rustc_span[937acc1ada1d69b0]::ErrorGuaranteed>>::{closure#1} as core[29afcc4231d37838]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  44:     0x7fa96481347e - std::sys::unix::thread::Thread::new::thread_start::hcc94b3880c043ea5
  45:     0x7fa9645adb43 - <unknown>
  46:     0x7fa96463fa00 - <unknown>
  47:                0x0 - <unknown>
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.70.0-nightly (55d7590d2 2023-03-17) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 -Z lower-impl-trait-in-trait-to-assoc-ty
query stack during panic:
query stack during panic:
#0 [fn_sig] computing function signature of `Foo::foo`
#1 [collect_mod_item_types] collecting item types in top-level module
#2 [analysis] running analysis passes on this crate
error: aborting due to previous error
------------------------------------------


