plain
...............................................iii...................................... 13200/13266
..................................................................
failures:

---- [ui] src/test/ui/impl-trait/rpit-assoc-pair-with-lifetime.rs stdout ----
error: test compilation failed although it shouldn't!
status: exit status: 101
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/rpit-assoc-pair-with-lifetime.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/rpit-assoc-pair-with-lifetime" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/rpit-assoc-pair-with-lifetime/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: compiler/rustc_typeck/src/check/inherited.rs:134:13: escaping bound vars in predicate Obligation(predicate=Binder(ProjectionPredicate(ProjectionTy { substs: [_], item_def_id: DefId(2:8587 ~ core[b1ce]::iter::traits::iterator::Iterator::Item) }, Ty((u32, &'a u32))), []), depth=0)
  --> /checkout/src/test/ui/impl-trait/rpit-assoc-pair-with-lifetime.rs:3:44
   |
LL | pub fn iter<'a>(v: Vec<(u32, &'a u32)>) -> impl DoubleEndedIterator<Item = (u32, &u32)> {

thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1332:9
stack backtrace:
stack backtrace:
   0:     0x7f98f3200f9c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hde8b2ba8ede34ebf
   1:     0x7f98f32679f8 - core::fmt::write::h7d6c89c06311476c
   2:     0x7f98f31f15b1 - std::io::Write::write_fmt::h88596bfc5d84d66d
   3:     0x7f98f3203f5e - std::panicking::default_hook::{{closure}}::hb93f855fb9bc0d48
   4:     0x7f98f3203c1f - std::panicking::default_hook::h60f8bee31477580c
   5:     0x7f98f3bb1bc4 - rustc_driver[535ce3a39664cfa7]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f98f3204712 - std::panicking::rust_panic_with_hook::h01371442757bdb57
   7:     0x7f98f4577363 - std[e2e40bd95004976a]::panicking::begin_panic::<rustc_errors[f662d9075717d011]::ExplicitBug>::{closure#0}
   8:     0x7f98f456c5c6 - std[e2e40bd95004976a]::sys_common::backtrace::__rust_end_short_backtrace::<std[e2e40bd95004976a]::panicking::begin_panic<rustc_errors[f662d9075717d011]::ExplicitBug>::{closure#0}, !>
   9:     0x7f98f3a04706 - std[e2e40bd95004976a]::panicking::begin_panic::<rustc_errors[f662d9075717d011]::ExplicitBug>
  10:     0x7f98f4786516 - std[e2e40bd95004976a]::panic::panic_any::<rustc_errors[f662d9075717d011]::ExplicitBug>
  11:     0x7f98f4769a73 - <rustc_errors[f662d9075717d011]::HandlerInner>::span_bug::<rustc_span[c788d8f490be92ad]::span_encoding::Span, &alloc[7891dea2ecb26796]::string::String>
  12:     0x7f98f4769060 - <rustc_errors[f662d9075717d011]::Handler>::span_bug::<rustc_span[c788d8f490be92ad]::span_encoding::Span, &alloc[7891dea2ecb26796]::string::String>
  13:     0x7f98f4738bf5 - rustc_middle[b6f4bae21e1833ed]::util::bug::opt_span_bug_fmt::<rustc_span[c788d8f490be92ad]::span_encoding::Span>::{closure#0}
  14:     0x7f98f4738c6b - rustc_middle[b6f4bae21e1833ed]::ty::context::tls::with_opt::<rustc_middle[b6f4bae21e1833ed]::util::bug::opt_span_bug_fmt<rustc_span[c788d8f490be92ad]::span_encoding::Span>::{closure#0}, ()>::{closure#0}
  15:     0x7f98f47378fe - rustc_middle[b6f4bae21e1833ed]::ty::context::tls::with_context_opt::<rustc_middle[b6f4bae21e1833ed]::ty::context::tls::with_opt<rustc_middle[b6f4bae21e1833ed]::util::bug::opt_span_bug_fmt<rustc_span[c788d8f490be92ad]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  16:     0x7f98f4737879 - rustc_middle[b6f4bae21e1833ed]::util::bug::opt_span_bug_fmt::<rustc_span[c788d8f490be92ad]::span_encoding::Span>
  17:     0x7f98f3a0ceb7 - rustc_middle[b6f4bae21e1833ed]::util::bug::span_bug_fmt::<rustc_span[c788d8f490be92ad]::span_encoding::Span>
  18:     0x7f98f4823bac - <rustc_typeck[f570e8b868e7d13e]::check::inherited::Inherited>::register_predicate
  19:     0x7f98f480f975 - <rustc_typeck[f570e8b868e7d13e]::check::inherited::Inherited>::register_infer_ok_obligations::<rustc_middle[b6f4bae21e1833ed]::ty::Ty>
  20:     0x7f98f47101f3 - rustc_typeck[f570e8b868e7d13e]::check::check::check_fn
  21:     0x7f98f46817f0 - <rustc_infer[320262f93f3e1ab2]::infer::InferCtxtBuilder>::enter::<&rustc_middle[b6f4bae21e1833ed]::ty::context::TypeckResults, <rustc_typeck[f570e8b868e7d13e]::check::inherited::InheritedBuilder>::enter<rustc_typeck[f570e8b868e7d13e]::check::typeck_with_fallback<rustc_typeck[f570e8b868e7d13e]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[b6f4bae21e1833ed]::ty::context::TypeckResults>::{closure#0}>
  22:     0x7f98f480f13e - <rustc_typeck[f570e8b868e7d13e]::check::inherited::InheritedBuilder>::enter::<rustc_typeck[f570e8b868e7d13e]::check::typeck_with_fallback<rustc_typeck[f570e8b868e7d13e]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[b6f4bae21e1833ed]::ty::context::TypeckResults>
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
  23:     0x7f98f45dbaf3 - rustc_typeck[f570e8b868e7d13e]::check::typeck
  24:     0x7f98f579fedd - rustc_query_system[26a8700115a605f6]::query::plumbing::try_execute_query::<rustc_query_impl[1ebcddd7f14b1fda]::plumbing::QueryCtxt, rustc_query_system[26a8700115a605f6]::query::caches::DefaultCache<rustc_span[c788d8f490be92ad]::def_id::LocalDefId, &rustc_middle[b6f4bae21e1833ed]::ty::context::TypeckResults>>
  25:     0x7f98f58c1647 - rustc_query_system[26a8700115a605f6]::query::plumbing::get_query::<rustc_query_impl[1ebcddd7f14b1fda]::queries::typeck, rustc_query_impl[1ebcddd7f14b1fda]::plumbing::QueryCtxt>
  26:     0x7f98f53acb34 - <rustc_query_impl[1ebcddd7f14b1fda]::Queries as rustc_middle[b6f4bae21e1833ed]::ty::query::QueryEngine>::typeck
  27:     0x7f98f66f0df9 - <rustc_middle[b6f4bae21e1833ed]::ty::context::TyCtxt>::typeck_opt_const_arg
  28:     0x7f98f4b46b7f - rustc_mir_build[b0115d17e547ab5e]::build::mir_built
  29:     0x7f98f578ed04 - rustc_query_system[26a8700115a605f6]::query::plumbing::try_execute_query::<rustc_query_impl[1ebcddd7f14b1fda]::plumbing::QueryCtxt, rustc_query_system[26a8700115a605f6]::query::caches::DefaultCache<rustc_middle[b6f4bae21e1833ed]::ty::WithOptConstParam<rustc_span[c788d8f490be92ad]::def_id::LocalDefId>, &rustc_data_structures[97701466670dbb00]::steal::Steal<rustc_middle[b6f4bae21e1833ed]::mir::Body>>>
  30:     0x7f98f58c3bc6 - rustc_query_system[26a8700115a605f6]::query::plumbing::get_query::<rustc_query_impl[1ebcddd7f14b1fda]::queries::mir_built, rustc_query_impl[1ebcddd7f14b1fda]::plumbing::QueryCtxt>
  31:     0x7f98f53958ca - <rustc_query_impl[1ebcddd7f14b1fda]::Queries as rustc_middle[b6f4bae21e1833ed]::ty::query::QueryEngine>::mir_built
  32:     0x7f98f4203a83 - rustc_mir_transform[167fd1ed7f697df4]::check_unsafety::unsafety_check_result
  33:     0x7f98f4200948 - <rustc_mir_transform[167fd1ed7f697df4]::check_unsafety::provide::{closure#0} as core[b1ce7b73d31b5fc4]::ops::function::FnOnce<(rustc_middle[b6f4bae21e1833ed]::ty::context::TyCtxt, rustc_span[c788d8f490be92ad]::def_id::LocalDefId)>>::call_once
  34:     0x7f98f57a199d - rustc_query_system[26a8700115a605f6]::query::plumbing::try_execute_query::<rustc_query_impl[1ebcddd7f14b1fda]::plumbing::QueryCtxt, rustc_query_system[26a8700115a605f6]::query::caches::DefaultCache<rustc_span[c788d8f490be92ad]::def_id::LocalDefId, &rustc_middle[b6f4bae21e1833ed]::mir::query::UnsafetyCheckResult>>
  35:     0x7f98f5892fc7 - rustc_query_system[26a8700115a605f6]::query::plumbing::get_query::<rustc_query_impl[1ebcddd7f14b1fda]::queries::unsafety_check_result, rustc_query_impl[1ebcddd7f14b1fda]::plumbing::QueryCtxt>
  36:     0x7f98f53a5494 - <rustc_query_impl[1ebcddd7f14b1fda]::Queries as rustc_middle[b6f4bae21e1833ed]::ty::query::QueryEngine>::unsafety_check_result
  37:     0x7f98f418530e - rustc_mir_transform[167fd1ed7f697df4]::mir_const
  38:     0x7f98f578ed04 - rustc_query_system[26a8700115a605f6]::query::plumbing::try_execute_query::<rustc_query_impl[1ebcddd7f14b1fda]::plumbing::QueryCtxt, rustc_query_system[26a8700115a605f6]::query::caches::DefaultCache<rustc_middle[b6f4bae21e1833ed]::ty::WithOptConstParam<rustc_span[c788d8f490be92ad]::def_id::LocalDefId>, &rustc_data_structures[97701466670dbb00]::steal::Steal<rustc_middle[b6f4bae21e1833ed]::mir::Body>>>
  39:     0x7f98f58c3d03 - rustc_query_system[26a8700115a605f6]::query::plumbing::get_query::<rustc_query_impl[1ebcddd7f14b1fda]::queries::mir_const, rustc_query_impl[1ebcddd7f14b1fda]::plumbing::QueryCtxt>
  40:     0x7f98f5395e4a - <rustc_query_impl[1ebcddd7f14b1fda]::Queries as rustc_middle[b6f4bae21e1833ed]::ty::query::QueryEngine>::mir_const
  41:     0x7f98f4185fce - rustc_mir_transform[167fd1ed7f697df4]::mir_promoted
  42:     0x7f98f586e95f - rustc_query_system[26a8700115a605f6]::query::plumbing::get_query::<rustc_query_impl[1ebcddd7f14b1fda]::queries::mir_promoted, rustc_query_impl[1ebcddd7f14b1fda]::plumbing::QueryCtxt>
  43:     0x7f98f53984fa - <rustc_query_impl[1ebcddd7f14b1fda]::Queries as rustc_middle[b6f4bae21e1833ed]::ty::query::QueryEngine>::mir_promoted
  44:     0x7f98f4e65c2f - rustc_borrowck[22a38b594e0ddcae]::mir_borrowck
  45:     0x7f98f4e2df28 - <rustc_borrowck[22a38b594e0ddcae]::provide::{closure#0} as core[b1ce7b73d31b5fc4]::ops::function::FnOnce<(rustc_middle[b6f4bae21e1833ed]::ty::context::TyCtxt, rustc_span[c788d8f490be92ad]::def_id::LocalDefId)>>::call_once
  46:     0x7f98f57a0c3d - rustc_query_system[26a8700115a605f6]::query::plumbing::try_execute_query::<rustc_query_impl[1ebcddd7f14b1fda]::plumbing::QueryCtxt, rustc_query_system[26a8700115a605f6]::query::caches::DefaultCache<rustc_span[c788d8f490be92ad]::def_id::LocalDefId, &rustc_middle[b6f4bae21e1833ed]::mir::query::BorrowCheckResult>>
  47:     0x7f98f586e238 - rustc_query_system[26a8700115a605f6]::query::plumbing::get_query::<rustc_query_impl[1ebcddd7f14b1fda]::queries::mir_borrowck, rustc_query_impl[1ebcddd7f14b1fda]::plumbing::QueryCtxt>
  48:     0x7f98f53aebb4 - <rustc_query_impl[1ebcddd7f14b1fda]::Queries as rustc_middle[b6f4bae21e1833ed]::ty::query::QueryEngine>::mir_borrowck
  49:     0x7f98f45f6744 - rustc_typeck[f570e8b868e7d13e]::collect::type_of::type_of
  50:     0x7f98f57b6e40 - rustc_query_system[26a8700115a605f6]::query::plumbing::try_execute_query::<rustc_query_impl[1ebcddd7f14b1fda]::plumbing::QueryCtxt, rustc_query_system[26a8700115a605f6]::query::caches::DefaultCache<rustc_span[c788d8f490be92ad]::def_id::DefId, rustc_middle[b6f4bae21e1833ed]::ty::Ty>>
  51:     0x7f98f58c1903 - rustc_query_system[26a8700115a605f6]::query::plumbing::get_query::<rustc_query_impl[1ebcddd7f14b1fda]::queries::type_of, rustc_query_impl[1ebcddd7f14b1fda]::plumbing::QueryCtxt>
  52:     0x7f98f538fc99 - <rustc_query_impl[1ebcddd7f14b1fda]::Queries as rustc_middle[b6f4bae21e1833ed]::ty::query::QueryEngine>::type_of
  53:     0x7f98f46f862d - rustc_typeck[f570e8b868e7d13e]::check::check::check_opaque
  54:     0x7f98f46fb82d - rustc_typeck[f570e8b868e7d13e]::check::check::check_item_type
  55:     0x7f98f470557a - rustc_typeck[f570e8b868e7d13e]::check::check::check_mod_item_types
  56:     0x7f98f57a3490 - rustc_query_system[26a8700115a605f6]::query::plumbing::try_execute_query::<rustc_query_impl[1ebcddd7f14b1fda]::plumbing::QueryCtxt, rustc_query_system[26a8700115a605f6]::query::caches::DefaultCache<rustc_span[c788d8f490be92ad]::def_id::LocalDefId, ()>>
  57:     0x7f98f588bef4 - rustc_query_system[26a8700115a605f6]::query::plumbing::get_query::<rustc_query_impl[1ebcddd7f14b1fda]::queries::check_mod_item_types, rustc_query_impl[1ebcddd7f14b1fda]::plumbing::QueryCtxt>
  58:     0x7f98f53a9584 - <rustc_query_impl[1ebcddd7f14b1fda]::Queries as rustc_middle[b6f4bae21e1833ed]::ty::query::QueryEngine>::check_mod_item_types
  59:     0x7f98f47167ea - <rustc_middle[b6f4bae21e1833ed]::hir::map::Map>::for_each_module::<rustc_typeck[f570e8b868e7d13e]::check_crate::{closure#6}::{closure#0}>
  60:     0x7f98f4608072 - <rustc_session[93e2a5c0b3dd4a59]::session::Session>::time::<(), rustc_typeck[f570e8b868e7d13e]::check_crate::{closure#6}>
  61:     0x7f98f4817513 - rustc_typeck[f570e8b868e7d13e]::check_crate
  62:     0x7f98f3cebd31 - rustc_interface[fdce399a96edc069]::passes::analysis
  63:     0x7f98f57db080 - rustc_query_system[26a8700115a605f6]::query::plumbing::try_execute_query::<rustc_query_impl[1ebcddd7f14b1fda]::plumbing::QueryCtxt, rustc_query_system[26a8700115a605f6]::query::caches::DefaultCache<(), core[b1ce7b73d31b5fc4]::result::Result<(), rustc_errors[f662d9075717d011]::ErrorGuaranteed>>>
  64:     0x7f98f58c1a22 - rustc_query_system[26a8700115a605f6]::query::plumbing::get_query::<rustc_query_impl[1ebcddd7f14b1fda]::queries::analysis, rustc_query_impl[1ebcddd7f14b1fda]::plumbing::QueryCtxt>
  65:     0x7f98f53901fe - <rustc_query_impl[1ebcddd7f14b1fda]::Queries as rustc_middle[b6f4bae21e1833ed]::ty::query::QueryEngine>::analysis
  66:     0x7f98f3c0a9fe - <rustc_interface[fdce399a96edc069]::passes::QueryContext>::enter::<rustc_driver[535ce3a39664cfa7]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[b1ce7b73d31b5fc4]::result::Result<(), rustc_errors[f662d9075717d011]::ErrorGuaranteed>>
  67:     0x7f98f3bb37ed - <rustc_interface[fdce399a96edc069]::interface::Compiler>::enter::<rustc_driver[535ce3a39664cfa7]::run_compiler::{closure#1}::{closure#2}, core[b1ce7b73d31b5fc4]::result::Result<core[b1ce7b73d31b5fc4]::option::Option<rustc_interface[fdce399a96edc069]::queries::Linker>, rustc_errors[f662d9075717d011]::ErrorGuaranteed>>
  68:     0x7f98f3ba37d1 - rustc_span[c788d8f490be92ad]::with_source_map::<core[b1ce7b73d31b5fc4]::result::Result<(), rustc_errors[f662d9075717d011]::ErrorGuaranteed>, rustc_interface[fdce399a96edc069]::interface::create_compiler_and_run<core[b1ce7b73d31b5fc4]::result::Result<(), rustc_errors[f662d9075717d011]::ErrorGuaranteed>, rustc_driver[535ce3a39664cfa7]::run_compiler::{closure#1}>::{closure#1}>
  69:     0x7f98f3bc83ca - rustc_interface[fdce399a96edc069]::interface::create_compiler_and_run::<core[b1ce7b73d31b5fc4]::result::Result<(), rustc_errors[f662d9075717d011]::ErrorGuaranteed>, rustc_driver[535ce3a39664cfa7]::run_compiler::{closure#1}>
  70:     0x7f98f3b9d712 - <scoped_tls[b5eaa558fa92b015]::ScopedKey<rustc_span[c788d8f490be92ad]::SessionGlobals>>::set::<rustc_interface[fdce399a96edc069]::interface::run_compiler<core[b1ce7b73d31b5fc4]::result::Result<(), rustc_errors[f662d9075717d011]::ErrorGuaranteed>, rustc_driver[535ce3a39664cfa7]::run_compiler::{closure#1}>::{closure#0}, core[b1ce7b73d31b5fc4]::result::Result<(), rustc_errors[f662d9075717d011]::ErrorGuaranteed>>
  71:     0x7f98f3bd52ff - std[e2e40bd95004976a]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[fdce399a96edc069]::util::run_in_thread_pool_with_globals<rustc_interface[fdce399a96edc069]::interface::run_compiler<core[b1ce7b73d31b5fc4]::result::Result<(), rustc_errors[f662d9075717d011]::ErrorGuaranteed>, rustc_driver[535ce3a39664cfa7]::run_compiler::{closure#1}>::{closure#0}, core[b1ce7b73d31b5fc4]::result::Result<(), rustc_errors[f662d9075717d011]::ErrorGuaranteed>>::{closure#0}, core[b1ce7b73d31b5fc4]::result::Result<(), rustc_errors[f662d9075717d011]::ErrorGuaranteed>>
  72:     0x7f98f3c0c8fe - std[e2e40bd95004976a]::panicking::try::<core[b1ce7b73d31b5fc4]::result::Result<(), rustc_errors[f662d9075717d011]::ErrorGuaranteed>, core[b1ce7b73d31b5fc4]::panic::unwind_safe::AssertUnwindSafe<<std[e2e40bd95004976a]::thread::Builder>::spawn_unchecked_<rustc_interface[fdce399a96edc069]::util::run_in_thread_pool_with_globals<rustc_interface[fdce399a96edc069]::interface::run_compiler<core[b1ce7b73d31b5fc4]::result::Result<(), rustc_errors[f662d9075717d011]::ErrorGuaranteed>, rustc_driver[535ce3a39664cfa7]::run_compiler::{closure#1}>::{closure#0}, core[b1ce7b73d31b5fc4]::result::Result<(), rustc_errors[f662d9075717d011]::ErrorGuaranteed>>::{closure#0}, core[b1ce7b73d31b5fc4]::result::Result<(), rustc_errors[f662d9075717d011]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  73:     0x7f98f3bd6e32 - <<std[e2e40bd95004976a]::thread::Builder>::spawn_unchecked_<rustc_interface[fdce399a96edc069]::util::run_in_thread_pool_with_globals<rustc_interface[fdce399a96edc069]::interface::run_compiler<core[b1ce7b73d31b5fc4]::result::Result<(), rustc_errors[f662d9075717d011]::ErrorGuaranteed>, rustc_driver[535ce3a39664cfa7]::run_compiler::{closure#1}>::{closure#0}, core[b1ce7b73d31b5fc4]::result::Result<(), rustc_errors[f662d9075717d011]::ErrorGuaranteed>>::{closure#0}, core[b1ce7b73d31b5fc4]::result::Result<(), rustc_errors[f662d9075717d011]::ErrorGuaranteed>>::{closure#1} as core[b1ce7b73d31b5fc4]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  74:     0x7f98f320f9e5 - std::sys::unix::thread::Thread::new::thread_start::h68c7073029ee0ec9
  75:     0x7f98ed75a609 - start_thread
  76:     0x7f98f306d133 - clone
  77:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.64.0-nightly (440b36716 2022-08-02) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `iter`
#1 [mir_built] building MIR for `iter`
#2 [unsafety_check_result] unsafety-checking `iter`
#3 [mir_const] processing MIR for `iter`
#4 [mir_promoted] processing `iter`
#5 [mir_borrowck] borrow-checking `iter`
#6 [type_of] computing type of `iter::{opaque#0}`
#7 [check_mod_item_types] checking item types in top-level module
#8 [analysis] running analysis passes on this crate
error: aborting due to previous error
------------------------------------------


