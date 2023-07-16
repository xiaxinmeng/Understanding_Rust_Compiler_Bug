plain
---- [ui] src/test/ui/async-await/issue-73137.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issue-73137.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-73137/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-73137/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: compiler/rustc_mir_transform/src/generator.rs:760:13: Broken MIR: generator contains type &u32 in MIR, but typeck only knows about {ResumeTy, u32, impl Future<Output = usize>, (), Foo, impl Future<Output = ()>} and []
   |
   |
LL |       let mut fut = Box::pin(async {
LL | |         let action = Foo {
LL | |             b: &42,
LL | |             b: &42,
LL | |             a: async { 0 }.await,
LL | |         async {}.await;
LL | |     });
   | |_____^


thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1394:9
stack backtrace:
   0:     0x7ff1bd04812c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha8c73e784f41a498
   1:     0x7ff1bd0b0ca8 - core::fmt::write::h18c0c41b86554260
   2:     0x7ff1bd0389a1 - std::io::Write::write_fmt::h8d4cba692f8c1148
   3:     0x7ff1bd04b11e - std::panicking::default_hook::{{closure}}::h906c38fe7f9debfd
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   4:     0x7ff1bd04ade7 - std::panicking::default_hook::hbb8f50d94fd7a77a
   5:     0x7ff1bd9d6634 - rustc_driver[196adfa4431ee8bf]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7ff1bd04b8d1 - std::panicking::rust_panic_with_hook::h6c90257650617ebd
   7:     0x7ff1bdfcf3f3 - std[1ac4e76fc44f83ad]::panicking::begin_panic::<rustc_errors[be7bc68bbaf343fb]::ExplicitBug>::{closure#0}
   8:     0x7ff1bdfc6276 - std[1ac4e76fc44f83ad]::sys_common::backtrace::__rust_end_short_backtrace::<std[1ac4e76fc44f83ad]::panicking::begin_panic<rustc_errors[be7bc68bbaf343fb]::ExplicitBug>::{closure#0}, !>
   9:     0x7ff1bd805386 - std[1ac4e76fc44f83ad]::panicking::begin_panic::<rustc_errors[be7bc68bbaf343fb]::ExplicitBug>
  10:     0x7ff1be03ad26 - std[1ac4e76fc44f83ad]::panic::panic_any::<rustc_errors[be7bc68bbaf343fb]::ExplicitBug>
  11:     0x7ff1be038743 - <rustc_errors[be7bc68bbaf343fb]::HandlerInner>::span_bug::<rustc_span[ce1bde991b2071e3]::span_encoding::Span, &alloc[4b4ce5a15409a95f]::string::String>
  12:     0x7ff1be038480 - <rustc_errors[be7bc68bbaf343fb]::Handler>::span_bug::<rustc_span[ce1bde991b2071e3]::span_encoding::Span, &alloc[4b4ce5a15409a95f]::string::String>
  13:     0x7ff1be08b8e2 - rustc_middle[343a9fb40ae486e2]::ty::context::tls::with_context_opt::<rustc_middle[343a9fb40ae486e2]::ty::context::tls::with_opt<rustc_middle[343a9fb40ae486e2]::util::bug::opt_span_bug_fmt<rustc_span[ce1bde991b2071e3]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  14:     0x7ff1be08b4f9 - rustc_middle[343a9fb40ae486e2]::util::bug::opt_span_bug_fmt::<rustc_span[ce1bde991b2071e3]::span_encoding::Span>
  15:     0x7ff1bd8164fa - rustc_middle[343a9fb40ae486e2]::util::bug::span_bug_fmt::<rustc_span[ce1bde991b2071e3]::span_encoding::Span>
  16:     0x7ff1bdf7b3b6 - <rustc_mir_transform[cde92db398d96739]::generator::StateTransform as rustc_middle[343a9fb40ae486e2]::mir::MirPass>::run_pass
  17:     0x7ff1bdee9f7f - rustc_mir_transform[cde92db398d96739]::pass_manager::run_passes
  18:     0x7ff1be00ba73 - rustc_mir_transform[cde92db398d96739]::optimized_mir
  19:     0x7ff1bf5351a9 - rustc_query_system[c36224424c2aea41]::query::plumbing::try_execute_query::<rustc_query_impl[3215d361fc278f3a]::plumbing::QueryCtxt, rustc_query_system[c36224424c2aea41]::query::caches::DefaultCache<rustc_span[ce1bde991b2071e3]::def_id::DefId, &rustc_middle[343a9fb40ae486e2]::mir::Body>>
  20:     0x7ff1bf5e1ca9 - rustc_query_system[c36224424c2aea41]::query::plumbing::get_query::<rustc_query_impl[3215d361fc278f3a]::queries::optimized_mir, rustc_query_impl[3215d361fc278f3a]::plumbing::QueryCtxt>
  21:     0x7ff1bf42c389 - <rustc_query_impl[3215d361fc278f3a]::Queries as rustc_middle[343a9fb40ae486e2]::ty::query::QueryEngine>::optimized_mir
  22:     0x7ff1c04b7e0f - <rustc_middle[343a9fb40ae486e2]::ty::context::TyCtxt>::generator_layout
  23:     0x7ff1c0350b35 - <rustc_middle[343a9fb40ae486e2]::ty::layout::LayoutCx<rustc_middle[343a9fb40ae486e2]::ty::context::TyCtxt>>::layout_of_uncached
  24:     0x7ff1c036dd2b - rustc_middle[343a9fb40ae486e2]::ty::layout::layout_of
  25:     0x7ff1bf6335b1 - rustc_query_system[c36224424c2aea41]::query::plumbing::get_query::<rustc_query_impl[3215d361fc278f3a]::queries::layout_of, rustc_query_impl[3215d361fc278f3a]::plumbing::QueryCtxt>
  26:     0x7ff1bf45605b - <rustc_query_impl[3215d361fc278f3a]::Queries as rustc_middle[343a9fb40ae486e2]::ty::query::QueryEngine>::layout_of
  27:     0x7ff1c035abda - <rustc_middle[343a9fb40ae486e2]::ty::layout::LayoutCx<rustc_middle[343a9fb40ae486e2]::ty::context::TyCtxt> as rustc_middle[343a9fb40ae486e2]::ty::layout::LayoutOf>::spanned_layout_of
  28:     0x7ff1c033826a - <core[22eb594ac9d95f1e]::iter::adapters::map::Map<core[22eb594ac9d95f1e]::slice::iter::Iter<rustc_middle[343a9fb40ae486e2]::ty::FieldDef>, <rustc_middle[343a9fb40ae486e2]::ty::layout::LayoutCx<rustc_middle[343a9fb40ae486e2]::ty::context::TyCtxt>>::layout_of_uncached::{closure#5}::{closure#0}> as core[22eb594ac9d95f1e]::iter::traits::iterator::Iterator>::try_fold::<(), <core[22eb594ac9d95f1e]::iter::adapters::GenericShunt<core[22eb594ac9d95f1e]::iter::adapters::map::Map<core[22eb594ac9d95f1e]::slice::iter::Iter<rustc_middle[343a9fb40ae486e2]::ty::FieldDef>, <rustc_middle[343a9fb40ae486e2]::ty::layout::LayoutCx<rustc_middle[343a9fb40ae486e2]::ty::context::TyCtxt>>::layout_of_uncached::{closure#5}::{closure#0}>, core[22eb594ac9d95f1e]::result::Result<core[22eb594ac9d95f1e]::convert::Infallible, rustc_middle[343a9fb40ae486e2]::ty::layout::LayoutError>> as core[22eb594ac9d95f1e]::iter::traits::iterator::Iterator>::try_fold<(), core[22eb594ac9d95f1e]::iter::traits::iterator::Iterator::try_for_each::call<rustc_target[9eea7ee19a6f7b65]::abi::TyAndLayout<rustc_middle[343a9fb40ae486e2]::ty::Ty>, core[22eb594ac9d95f1e]::ops::control_flow::ControlFlow<rustc_target[9eea7ee19a6f7b65]::abi::TyAndLayout<rustc_middle[343a9fb40ae486e2]::ty::Ty>>, core[22eb594ac9d95f1e]::ops::control_flow::ControlFlow<rustc_target[9eea7ee19a6f7b65]::abi::TyAndLayout<rustc_middle[343a9fb40ae486e2]::ty::Ty>>::Break>::{closure#0}, core[22eb594ac9d95f1e]::ops::control_flow::ControlFlow<rustc_target[9eea7ee19a6f7b65]::abi::TyAndLayout<rustc_middle[343a9fb40ae486e2]::ty::Ty>>>::{closure#0}, core[22eb594ac9d95f1e]::ops::control_flow::ControlFlow<core[22eb594ac9d95f1e]::ops::control_flow::ControlFlow<rustc_target[9eea7ee19a6f7b65]::abi::TyAndLayout<rustc_middle[343a9fb40ae486e2]::ty::Ty>>>>
  29:     0x7ff1c041d404 - core[22eb594ac9d95f1e]::iter::adapters::try_process::<core[22eb594ac9d95f1e]::iter::adapters::map::Map<core[22eb594ac9d95f1e]::slice::iter::Iter<rustc_middle[343a9fb40ae486e2]::ty::FieldDef>, <rustc_middle[343a9fb40ae486e2]::ty::layout::LayoutCx<rustc_middle[343a9fb40ae486e2]::ty::context::TyCtxt>>::layout_of_uncached::{closure#5}::{closure#0}>, rustc_target[9eea7ee19a6f7b65]::abi::TyAndLayout<rustc_middle[343a9fb40ae486e2]::ty::Ty>, core[22eb594ac9d95f1e]::result::Result<core[22eb594ac9d95f1e]::convert::Infallible, rustc_middle[343a9fb40ae486e2]::ty::layout::LayoutError>, <core[22eb594ac9d95f1e]::result::Result<alloc[4b4ce5a15409a95f]::vec::Vec<rustc_target[9eea7ee19a6f7b65]::abi::TyAndLayout<rustc_middle[343a9fb40ae486e2]::ty::Ty>>, rustc_middle[343a9fb40ae486e2]::ty::layout::LayoutError> as core[22eb594ac9d95f1e]::iter::traits::collect::FromIterator<core[22eb594ac9d95f1e]::result::Result<rustc_target[9eea7ee19a6f7b65]::abi::TyAndLayout<rustc_middle[343a9fb40ae486e2]::ty::Ty>, rustc_middle[343a9fb40ae486e2]::ty::layout::LayoutError>>>::from_iter<core[22eb594ac9d95f1e]::iter::adapters::map::Map<core[22eb594ac9d95f1e]::slice::iter::Iter<rustc_middle[343a9fb40ae486e2]::ty::FieldDef>, <rustc_middle[343a9fb40ae486e2]::ty::layout::LayoutCx<rustc_middle[343a9fb40ae486e2]::ty::context::TyCtxt>>::layout_of_uncached::{closure#5}::{closure#0}>>::{closure#0}, alloc[4b4ce5a15409a95f]::vec::Vec<rustc_target[9eea7ee19a6f7b65]::abi::TyAndLayout<rustc_middle[343a9fb40ae486e2]::ty::Ty>>>
  30:     0x7ff1c0338182 - <core[22eb594ac9d95f1e]::iter::adapters::map::Map<core[22eb594ac9d95f1e]::slice::iter::Iter<rustc_middle[343a9fb40ae486e2]::ty::VariantDef>, <rustc_middle[343a9fb40ae486e2]::ty::layout::LayoutCx<rustc_middle[343a9fb40ae486e2]::ty::context::TyCtxt>>::layout_of_uncached::{closure#5}> as core[22eb594ac9d95f1e]::iter::traits::iterator::Iterator>::try_fold::<(), <core[22eb594ac9d95f1e]::iter::adapters::GenericShunt<core[22eb594ac9d95f1e]::iter::adapters::map::Map<core[22eb594ac9d95f1e]::slice::iter::Iter<rustc_middle[343a9fb40ae486e2]::ty::VariantDef>, <rustc_middle[343a9fb40ae486e2]::ty::layout::LayoutCx<rustc_middle[343a9fb40ae486e2]::ty::context::TyCtxt>>::layout_of_uncached::{closure#5}>, core[22eb594ac9d95f1e]::result::Result<core[22eb594ac9d95f1e]::convert::Infallible, rustc_middle[343a9fb40ae486e2]::ty::layout::LayoutError>> as core[22eb594ac9d95f1e]::iter::traits::iterator::Iterator>::try_fold<(), core[22eb594ac9d95f1e]::iter::traits::iterator::Iterator::try_for_each::call<alloc[4b4ce5a15409a95f]::vec::Vec<rustc_target[9eea7ee19a6f7b65]::abi::TyAndLayout<rustc_middle[343a9fb40ae486e2]::ty::Ty>>, core[22eb594ac9d95f1e]::ops::control_flow::ControlFlow<alloc[4b4ce5a15409a95f]::vec::Vec<rustc_target[9eea7ee19a6f7b65]::abi::TyAndLayout<rustc_middle[343a9fb40ae486e2]::ty::Ty>>>, core[22eb594ac9d95f1e]::ops::control_flow::ControlFlow<alloc[4b4ce5a15409a95f]::vec::Vec<rustc_target[9eea7ee19a6f7b65]::abi::TyAndLayout<rustc_middle[343a9fb40ae486e2]::ty::Ty>>>::Break>::{closure#0}, core[22eb594ac9d95f1e]::ops::control_flow::ControlFlow<alloc[4b4ce5a15409a95f]::vec::Vec<rustc_target[9eea7ee19a6f7b65]::abi::TyAndLayout<rustc_middle[343a9fb40ae486e2]::ty::Ty>>>>::{closure#0}, core[22eb594ac9d95f1e]::ops::control_flow::ControlFlow<core[22eb594ac9d95f1e]::ops::control_flow::ControlFlow<alloc[4b4ce5a15409a95f]::vec::Vec<rustc_target[9eea7ee19a6f7b65]::abi::TyAndLayout<rustc_middle[343a9fb40ae486e2]::ty::Ty>>>>>
  31:     0x7ff1c041d03a - core[22eb594ac9d95f1e]::iter::adapters::try_process::<core[22eb594ac9d95f1e]::iter::adapters::map::Map<core[22eb594ac9d95f1e]::slice::iter::Iter<rustc_middle[343a9fb40ae486e2]::ty::VariantDef>, <rustc_middle[343a9fb40ae486e2]::ty::layout::LayoutCx<rustc_middle[343a9fb40ae486e2]::ty::context::TyCtxt>>::layout_of_uncached::{closure#5}>, alloc[4b4ce5a15409a95f]::vec::Vec<rustc_target[9eea7ee19a6f7b65]::abi::TyAndLayout<rustc_middle[343a9fb40ae486e2]::ty::Ty>>, core[22eb594ac9d95f1e]::result::Result<core[22eb594ac9d95f1e]::convert::Infallible, rustc_middle[343a9fb40ae486e2]::ty::layout::LayoutError>, <core[22eb594ac9d95f1e]::result::Result<rustc_index[ad5a6f35b8f9c06c]::vec::IndexVec<rustc_target[9eea7ee19a6f7b65]::abi::VariantIdx, alloc[4b4ce5a15409a95f]::vec::Vec<rustc_target[9eea7ee19a6f7b65]::abi::TyAndLayout<rustc_middle[343a9fb40ae486e2]::ty::Ty>>>, rustc_middle[343a9fb40ae486e2]::ty::layout::LayoutError> as core[22eb594ac9d95f1e]::iter::traits::collect::FromIterator<core[22eb594ac9d95f1e]::result::Result<alloc[4b4ce5a15409a95f]::vec::Vec<rustc_target[9eea7ee19a6f7b65]::abi::TyAndLayout<rustc_middle[343a9fb40ae486e2]::ty::Ty>>, rustc_middle[343a9fb40ae486e2]::ty::layout::LayoutError>>>::from_iter<core[22eb594ac9d95f1e]::iter::adapters::map::Map<core[22eb594ac9d95f1e]::slice::iter::Iter<rustc_middle[343a9fb40ae486e2]::ty::VariantDef>, <rustc_middle[343a9fb40ae486e2]::ty::layout::LayoutCx<rustc_middle[343a9fb40ae486e2]::ty::context::TyCtxt>>::layout_of_uncached::{closure#5}>>::{closure#0}, rustc_index[ad5a6f35b8f9c06c]::vec::IndexVec<rustc_target[9eea7ee19a6f7b65]::abi::VariantIdx, alloc[4b4ce5a15409a95f]::vec::Vec<rustc_target[9eea7ee19a6f7b65]::abi::TyAndLayout<rustc_middle[343a9fb40ae486e2]::ty::Ty>>>>
  32:     0x7ff1c035150d - <rustc_middle[343a9fb40ae486e2]::ty::layout::LayoutCx<rustc_middle[343a9fb40ae486e2]::ty::context::TyCtxt>>::layout_of_uncached
  33:     0x7ff1c036dd2b - rustc_middle[343a9fb40ae486e2]::ty::layout::layout_of
  34:     0x7ff1bf6335b1 - rustc_query_system[c36224424c2aea41]::query::plumbing::get_query::<rustc_query_impl[3215d361fc278f3a]::queries::layout_of, rustc_query_impl[3215d361fc278f3a]::plumbing::QueryCtxt>
  35:     0x7ff1bf45605b - <rustc_query_impl[3215d361fc278f3a]::Queries as rustc_middle[343a9fb40ae486e2]::ty::query::QueryEngine>::layout_of
  36:     0x7ff1c036e0f8 - rustc_middle[343a9fb40ae486e2]::ty::layout::layout_of
  37:     0x7ff1bf6335b1 - rustc_query_system[c36224424c2aea41]::query::plumbing::get_query::<rustc_query_impl[3215d361fc278f3a]::queries::layout_of, rustc_query_impl[3215d361fc278f3a]::plumbing::QueryCtxt>
  38:     0x7ff1bf45605b - <rustc_query_impl[3215d361fc278f3a]::Queries as rustc_middle[343a9fb40ae486e2]::ty::query::QueryEngine>::layout_of
  39:     0x7ff1bdf1f980 - <rustc_mir_transform[cde92db398d96739]::const_prop::CanConstProp>::check
  40:     0x7ff1be04d14f - <rustc_mir_transform[cde92db398d96739]::const_prop_lint::ConstProp as rustc_mir_transform[cde92db398d96739]::pass_manager::MirLint>::run_lint
  41:     0x7ff1bdee9f7f - rustc_mir_transform[cde92db398d96739]::pass_manager::run_passes
  42:     0x7ff1be00b0ec - rustc_mir_transform[cde92db398d96739]::run_post_borrowck_cleanup_passes
  43:     0x7ff1be00ac78 - rustc_mir_transform[cde92db398d96739]::mir_drops_elaborated_and_const_checked
  44:     0x7ff1bf4fa576 - rustc_query_system[c36224424c2aea41]::query::plumbing::try_execute_query::<rustc_query_impl[3215d361fc278f3a]::plumbing::QueryCtxt, rustc_query_system[c36224424c2aea41]::query::caches::DefaultCache<rustc_middle[343a9fb40ae486e2]::ty::WithOptConstParam<rustc_span[ce1bde991b2071e3]::def_id::LocalDefId>, &rustc_data_structures[afc146bf68135db4]::steal::Steal<rustc_middle[343a9fb40ae486e2]::mir::Body>>>
  45:     0x7ff1bf62cf05 - rustc_query_system[c36224424c2aea41]::query::plumbing::get_query::<rustc_query_impl[3215d361fc278f3a]::queries::mir_drops_elaborated_and_const_checked, rustc_query_impl[3215d361fc278f3a]::plumbing::QueryCtxt>
  46:     0x7ff1bf42a956 - <rustc_query_impl[3215d361fc278f3a]::Queries as rustc_middle[343a9fb40ae486e2]::ty::query::QueryEngine>::mir_drops_elaborated_and_const_checked
  47:     0x7ff1be00b955 - rustc_mir_transform[cde92db398d96739]::optimized_mir
  48:     0x7ff1bf5351a9 - rustc_query_system[c36224424c2aea41]::query::plumbing::try_execute_query::<rustc_query_impl[3215d361fc278f3a]::plumbing::QueryCtxt, rustc_query_system[c36224424c2aea41]::query::caches::DefaultCache<rustc_span[ce1bde991b2071e3]::def_id::DefId, &rustc_middle[343a9fb40ae486e2]::mir::Body>>
  49:     0x7ff1bf5e1ca9 - rustc_query_system[c36224424c2aea41]::query::plumbing::get_query::<rustc_query_impl[3215d361fc278f3a]::queries::optimized_mir, rustc_query_impl[3215d361fc278f3a]::plumbing::QueryCtxt>
  50:     0x7ff1bf42c389 - <rustc_query_impl[3215d361fc278f3a]::Queries as rustc_middle[343a9fb40ae486e2]::ty::query::QueryEngine>::optimized_mir
  51:     0x7ff1c04c62f4 - <rustc_middle[343a9fb40ae486e2]::ty::context::TyCtxt>::instance_mir
  52:     0x7ff1bde35f89 - rustc_monomorphize[9382b5909fb7b1e0]::collector::collect_neighbours
  53:     0x7ff1bde310d6 - rustc_monomorphize[9382b5909fb7b1e0]::collector::collect_items_rec
  54:     0x7ff1bde3b4ed - <core[22eb594ac9d95f1e]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[afc146bf68135db4]::sync::par_for_each_in<alloc[4b4ce5a15409a95f]::vec::Vec<rustc_middle[343a9fb40ae486e2]::mir::mono::MonoItem>, rustc_monomorphize[9382b5909fb7b1e0]::collector::collect_crate_mono_items::{closure#1}::{closure#0}>::{closure#0}::{closure#0}> as core[22eb594ac9d95f1e]::ops::function::FnOnce<()>>::call_once
  55:     0x7ff1bde75f05 - std[1ac4e76fc44f83ad]::panicking::try::<(), core[22eb594ac9d95f1e]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[afc146bf68135db4]::sync::par_for_each_in<alloc[4b4ce5a15409a95f]::vec::Vec<rustc_middle[343a9fb40ae486e2]::mir::mono::MonoItem>, rustc_monomorphize[9382b5909fb7b1e0]::collector::collect_crate_mono_items::{closure#1}::{closure#0}>::{closure#0}::{closure#0}>>
  56:     0x7ff1bde4ce5e - rustc_data_structures[afc146bf68135db4]::sync::par_for_each_in::<alloc[4b4ce5a15409a95f]::vec::Vec<rustc_middle[343a9fb40ae486e2]::mir::mono::MonoItem>, rustc_monomorphize[9382b5909fb7b1e0]::collector::collect_crate_mono_items::{closure#1}::{closure#0}>
  57:     0x7ff1bde74b40 - <rustc_session[923ab149e4687334]::session::Session>::time::<(), rustc_monomorphize[9382b5909fb7b1e0]::collector::collect_crate_mono_items::{closure#1}>
  58:     0x7ff1bde2df6f - rustc_monomorphize[9382b5909fb7b1e0]::collector::collect_crate_mono_items
  59:     0x7ff1bde3d3aa - rustc_monomorphize[9382b5909fb7b1e0]::partitioning::collect_and_partition_mono_items
  60:     0x7ff1bf55aa21 - rustc_query_system[c36224424c2aea41]::query::plumbing::try_execute_query::<rustc_query_impl[3215d361fc278f3a]::plumbing::QueryCtxt, rustc_query_system[c36224424c2aea41]::query::caches::DefaultCache<(), (&std[1ac4e76fc44f83ad]::collections::hash::set::HashSet<rustc_span[ce1bde991b2071e3]::def_id::DefId, core[22eb594ac9d95f1e]::hash::BuildHasherDefault<rustc_hash[93bca5ca72a21695]::FxHasher>>, &[rustc_middle[343a9fb40ae486e2]::mir::mono::CodegenUnit])>>
  61:     0x7ff1bf62825c - rustc_query_system[c36224424c2aea41]::query::plumbing::get_query::<rustc_query_impl[3215d361fc278f3a]::queries::collect_and_partition_mono_items, rustc_query_impl[3215d361fc278f3a]::plumbing::QueryCtxt>
  62:     0x7ff1bf46ef23 - <rustc_query_impl[3215d361fc278f3a]::Queries as rustc_middle[343a9fb40ae486e2]::ty::query::QueryEngine>::collect_and_partition_mono_items
  63:     0x7ff1bdc47043 - rustc_codegen_ssa[f63f7e51ae6221bb]::base::codegen_crate::<rustc_codegen_llvm[4b46e876e27e2f6d]::LlvmCodegenBackend>
  64:     0x7ff1bdd0c19d - <rustc_codegen_llvm[4b46e876e27e2f6d]::LlvmCodegenBackend as rustc_codegen_ssa[f63f7e51ae6221bb]::traits::backend::CodegenBackend>::codegen_crate
  65:     0x7ff1bdb63c5b - <rustc_session[923ab149e4687334]::session::Session>::time::<alloc[4b4ce5a15409a95f]::boxed::Box<dyn core[22eb594ac9d95f1e]::any::Any>, rustc_interface[1038902ec4022196]::passes::start_codegen::{closure#0}>
  66:     0x7ff1bdb2c5a5 - <rustc_interface[1038902ec4022196]::passes::QueryContext>::enter::<<rustc_interface[1038902ec4022196]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[22eb594ac9d95f1e]::result::Result<alloc[4b4ce5a15409a95f]::boxed::Box<dyn core[22eb594ac9d95f1e]::any::Any>, rustc_errors[be7bc68bbaf343fb]::ErrorGuaranteed>>
  67:     0x7ff1bdb1261d - <rustc_interface[1038902ec4022196]::queries::Queries>::ongoing_codegen
  68:     0x7ff1bd9de8a3 - <rustc_interface[1038902ec4022196]::interface::Compiler>::enter::<rustc_driver[196adfa4431ee8bf]::run_compiler::{closure#1}::{closure#2}, core[22eb594ac9d95f1e]::result::Result<core[22eb594ac9d95f1e]::option::Option<rustc_interface[1038902ec4022196]::queries::Linker>, rustc_errors[be7bc68bbaf343fb]::ErrorGuaranteed>>
  69:     0x7ff1bd9c0fc5 - rustc_span[ce1bde991b2071e3]::with_source_map::<core[22eb594ac9d95f1e]::result::Result<(), rustc_errors[be7bc68bbaf343fb]::ErrorGuaranteed>, rustc_interface[1038902ec4022196]::interface::create_compiler_and_run<core[22eb594ac9d95f1e]::result::Result<(), rustc_errors[be7bc68bbaf343fb]::ErrorGuaranteed>, rustc_driver[196adfa4431ee8bf]::run_compiler::{closure#1}>::{closure#1}>
  70:     0x7ff1bd9e0961 - rustc_interface[1038902ec4022196]::interface::create_compiler_and_run::<core[22eb594ac9d95f1e]::result::Result<(), rustc_errors[be7bc68bbaf343fb]::ErrorGuaranteed>, rustc_driver[196adfa4431ee8bf]::run_compiler::{closure#1}>
  71:     0x7ff1bd9c2bf2 - <scoped_tls[1a0b58be114ec1f]::ScopedKey<rustc_span[ce1bde991b2071e3]::SessionGlobals>>::set::<rustc_interface[1038902ec4022196]::interface::run_compiler<core[22eb594ac9d95f1e]::result::Result<(), rustc_errors[be7bc68bbaf343fb]::ErrorGuaranteed>, rustc_driver[196adfa4431ee8bf]::run_compiler::{closure#1}>::{closure#0}, core[22eb594ac9d95f1e]::result::Result<(), rustc_errors[be7bc68bbaf343fb]::ErrorGuaranteed>>
  72:     0x7ff1bda39009 - std[1ac4e76fc44f83ad]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[1038902ec4022196]::util::run_in_thread_pool_with_globals<rustc_interface[1038902ec4022196]::interface::run_compiler<core[22eb594ac9d95f1e]::result::Result<(), rustc_errors[be7bc68bbaf343fb]::ErrorGuaranteed>, rustc_driver[196adfa4431ee8bf]::run_compiler::{closure#1}>::{closure#0}, core[22eb594ac9d95f1e]::result::Result<(), rustc_errors[be7bc68bbaf343fb]::ErrorGuaranteed>>::{closure#0}, core[22eb594ac9d95f1e]::result::Result<(), rustc_errors[be7bc68bbaf343fb]::ErrorGuaranteed>>
  73:     0x7ff1bd9c53ce - std[1ac4e76fc44f83ad]::panicking::try::<core[22eb594ac9d95f1e]::result::Result<(), rustc_errors[be7bc68bbaf343fb]::ErrorGuaranteed>, core[22eb594ac9d95f1e]::panic::unwind_safe::AssertUnwindSafe<<std[1ac4e76fc44f83ad]::thread::Builder>::spawn_unchecked_<rustc_interface[1038902ec4022196]::util::run_in_thread_pool_with_globals<rustc_interface[1038902ec4022196]::interface::run_compiler<core[22eb594ac9d95f1e]::result::Result<(), rustc_errors[be7bc68bbaf343fb]::ErrorGuaranteed>, rustc_driver[196adfa4431ee8bf]::run_compiler::{closure#1}>::{closure#0}, core[22eb594ac9d95f1e]::result::Result<(), rustc_errors[be7bc68bbaf343fb]::ErrorGuaranteed>>::{closure#0}, core[22eb594ac9d95f1e]::result::Result<(), rustc_errors[be7bc68bbaf343fb]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  74:     0x7ff1bda3d630 - <<std[1ac4e76fc44f83ad]::thread::Builder>::spawn_unchecked_<rustc_interface[1038902ec4022196]::util::run_in_thread_pool_with_globals<rustc_interface[1038902ec4022196]::interface::run_compiler<core[22eb594ac9d95f1e]::result::Result<(), rustc_errors[be7bc68bbaf343fb]::ErrorGuaranteed>, rustc_driver[196adfa4431ee8bf]::run_compiler::{closure#1}>::{closure#0}, core[22eb594ac9d95f1e]::result::Result<(), rustc_errors[be7bc68bbaf343fb]::ErrorGuaranteed>>::{closure#0}, core[22eb594ac9d95f1e]::result::Result<(), rustc_errors[be7bc68bbaf343fb]::ErrorGuaranteed>>::{closure#1} as core[22eb594ac9d95f1e]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  75:     0x7ff1bd058645 - std::sys::unix::thread::Thread::new::thread_start::hf0205dea1a6ad4f9
  76:     0x7ff1bcdf4b43 - <unknown>
  77:     0x7ff1bce86a00 - <unknown>
  78:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.65.0-nightly (f502824b3 2022-08-30) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
#0 [optimized_mir] optimizing MIR for `main::{closure#0}`
#1 [layout_of] computing layout of `[static generator@/checkout/src/test/ui/async-await/issue-73137.rs:25:34: 37:6]`
#2 [layout_of] computing layout of `core::future::from_generator::GenFuture<[static generator@/checkout/src/test/ui/async-await/issue-73137.rs:25:34: 37:6]>`
#2 [layout_of] computing layout of `core::future::from_generator::GenFuture<[static generator@/checkout/src/test/ui/async-await/issue-73137.rs:25:34: 37:6]>`
#3 [layout_of] computing layout of `impl core::future::future::Future<Output = ()>`
#4 [mir_drops_elaborated_and_const_checked] elaborating drops for `main`
#5 [optimized_mir] optimizing MIR for `main`
#6 [collect_and_partition_mono_items] collect_and_partition_mono_items
error: aborting due to previous error
------------------------------------------


