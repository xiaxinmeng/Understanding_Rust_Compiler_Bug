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


thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1333:9
stack backtrace:
   0:     0x7f13290ef9fc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h88ef7875ab741864
   1:     0x7f1329158198 - core::fmt::write::h95ab050eb4a876c8
   2:     0x7f13290e06f1 - std::io::Write::write_fmt::ha0c2d42a5b7f687c
   3:     0x7f13290f29ee - std::panicking::default_hook::{{closure}}::h412d0ff9ad077258
   4:     0x7f13290f26b7 - std::panicking::default_hook::hb794bded683e5e9d
   5:     0x7f1329a7eae4 - rustc_driver[c5980153e48cf13b]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f13290f31a1 - std::panicking::rust_panic_with_hook::h7d1317b4b128b4fc
   7:     0x7f132a050a83 - std[e791036a24d368b7]::panicking::begin_panic::<rustc_errors[c0fe8da4a22912a3]::ExplicitBug>::{closure#0}
   8:     0x7f132a04e146 - std[e791036a24d368b7]::sys_common::backtrace::__rust_end_short_backtrace::<std[e791036a24d368b7]::panicking::begin_panic<rustc_errors[c0fe8da4a22912a3]::ExplicitBug>::{closure#0}, !>
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   9:     0x7f13298ad4c6 - std[e791036a24d368b7]::panicking::begin_panic::<rustc_errors[c0fe8da4a22912a3]::ExplicitBug>
  10:     0x7f132a0c8fd6 - std[e791036a24d368b7]::panic::panic_any::<rustc_errors[c0fe8da4a22912a3]::ExplicitBug>
  11:     0x7f132a0c62d3 - <rustc_errors[c0fe8da4a22912a3]::HandlerInner>::span_bug::<rustc_span[d8fe0b440f56fba7]::span_encoding::Span, &alloc[d1443e666f22d28e]::string::String>
  12:     0x7f132a0c6010 - <rustc_errors[c0fe8da4a22912a3]::Handler>::span_bug::<rustc_span[d8fe0b440f56fba7]::span_encoding::Span, &alloc[d1443e666f22d28e]::string::String>
  13:     0x7f1329f5c2d2 - rustc_middle[4703dced25f2e482]::ty::context::tls::with_context_opt::<rustc_middle[4703dced25f2e482]::ty::context::tls::with_opt<rustc_middle[4703dced25f2e482]::util::bug::opt_span_bug_fmt<rustc_span[d8fe0b440f56fba7]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  14:     0x7f1329f5b569 - rustc_middle[4703dced25f2e482]::util::bug::opt_span_bug_fmt::<rustc_span[d8fe0b440f56fba7]::span_encoding::Span>
  15:     0x7f13298ad55a - rustc_middle[4703dced25f2e482]::util::bug::span_bug_fmt::<rustc_span[d8fe0b440f56fba7]::span_encoding::Span>
  16:     0x7f132a003ee3 - <rustc_mir_transform[867a8f12a738183d]::generator::StateTransform as rustc_middle[4703dced25f2e482]::mir::MirPass>::run_pass
  17:     0x7f1329facf57 - rustc_mir_transform[867a8f12a738183d]::pass_manager::run_passes
  18:     0x7f132a06bcb1 - rustc_mir_transform[867a8f12a738183d]::optimized_mir
  19:     0x7f132b653238 - rustc_query_system[e6ab6bb4cb63ce3f]::query::plumbing::try_execute_query::<rustc_query_impl[36cd38aa3e85ef29]::plumbing::QueryCtxt, rustc_query_system[e6ab6bb4cb63ce3f]::query::caches::DefaultCache<rustc_span[d8fe0b440f56fba7]::def_id::DefId, &rustc_middle[4703dced25f2e482]::mir::Body>>
  20:     0x7f132b6fc154 - rustc_query_system[e6ab6bb4cb63ce3f]::query::plumbing::get_query::<rustc_query_impl[36cd38aa3e85ef29]::queries::optimized_mir, rustc_query_impl[36cd38aa3e85ef29]::plumbing::QueryCtxt>
  21:     0x7f132b258659 - <rustc_query_impl[36cd38aa3e85ef29]::Queries as rustc_middle[4703dced25f2e482]::ty::query::QueryEngine>::optimized_mir
  22:     0x7f132c553bcf - <rustc_middle[4703dced25f2e482]::ty::context::TyCtxt>::generator_layout
  23:     0x7f132c3f0a89 - <rustc_middle[4703dced25f2e482]::ty::layout::LayoutCx<rustc_middle[4703dced25f2e482]::ty::context::TyCtxt>>::layout_of_uncached
  24:     0x7f132c40c9cc - rustc_middle[4703dced25f2e482]::ty::layout::layout_of
  25:     0x7f132b748f88 - rustc_query_system[e6ab6bb4cb63ce3f]::query::plumbing::get_query::<rustc_query_impl[36cd38aa3e85ef29]::queries::layout_of, rustc_query_impl[36cd38aa3e85ef29]::plumbing::QueryCtxt>
  26:     0x7f132b2830a1 - <rustc_query_impl[36cd38aa3e85ef29]::Queries as rustc_middle[4703dced25f2e482]::ty::query::QueryEngine>::layout_of
  27:     0x7f132c3fa145 - <rustc_middle[4703dced25f2e482]::ty::layout::LayoutCx<rustc_middle[4703dced25f2e482]::ty::context::TyCtxt> as rustc_middle[4703dced25f2e482]::ty::layout::LayoutOf>::spanned_layout_of
  28:     0x7f132c3d97ca - <core[55a6db15fdfe53de]::iter::adapters::map::Map<core[55a6db15fdfe53de]::slice::iter::Iter<rustc_middle[4703dced25f2e482]::ty::FieldDef>, <rustc_middle[4703dced25f2e482]::ty::layout::LayoutCx<rustc_middle[4703dced25f2e482]::ty::context::TyCtxt>>::layout_of_uncached::{closure#5}::{closure#0}> as core[55a6db15fdfe53de]::iter::traits::iterator::Iterator>::try_fold::<(), <core[55a6db15fdfe53de]::iter::adapters::GenericShunt<core[55a6db15fdfe53de]::iter::adapters::map::Map<core[55a6db15fdfe53de]::slice::iter::Iter<rustc_middle[4703dced25f2e482]::ty::FieldDef>, <rustc_middle[4703dced25f2e482]::ty::layout::LayoutCx<rustc_middle[4703dced25f2e482]::ty::context::TyCtxt>>::layout_of_uncached::{closure#5}::{closure#0}>, core[55a6db15fdfe53de]::result::Result<core[55a6db15fdfe53de]::convert::Infallible, rustc_middle[4703dced25f2e482]::ty::layout::LayoutError>> as core[55a6db15fdfe53de]::iter::traits::iterator::Iterator>::try_fold<(), core[55a6db15fdfe53de]::iter::traits::iterator::Iterator::try_for_each::call<rustc_target[abca9d85455cc236]::abi::TyAndLayout<rustc_middle[4703dced25f2e482]::ty::Ty>, core[55a6db15fdfe53de]::ops::control_flow::ControlFlow<rustc_target[abca9d85455cc236]::abi::TyAndLayout<rustc_middle[4703dced25f2e482]::ty::Ty>>, core[55a6db15fdfe53de]::ops::control_flow::ControlFlow<rustc_target[abca9d85455cc236]::abi::TyAndLayout<rustc_middle[4703dced25f2e482]::ty::Ty>>::Break>::{closure#0}, core[55a6db15fdfe53de]::ops::control_flow::ControlFlow<rustc_target[abca9d85455cc236]::abi::TyAndLayout<rustc_middle[4703dced25f2e482]::ty::Ty>>>::{closure#0}, core[55a6db15fdfe53de]::ops::control_flow::ControlFlow<core[55a6db15fdfe53de]::ops::control_flow::ControlFlow<rustc_target[abca9d85455cc236]::abi::TyAndLayout<rustc_middle[4703dced25f2e482]::ty::Ty>>>>
  29:     0x7f132c4869ea - <core[55a6db15fdfe53de]::iter::adapters::GenericShunt<core[55a6db15fdfe53de]::iter::adapters::map::Map<core[55a6db15fdfe53de]::slice::iter::Iter<rustc_middle[4703dced25f2e482]::ty::FieldDef>, <rustc_middle[4703dced25f2e482]::ty::layout::LayoutCx<rustc_middle[4703dced25f2e482]::ty::context::TyCtxt>>::layout_of_uncached::{closure#5}::{closure#0}>, core[55a6db15fdfe53de]::result::Result<core[55a6db15fdfe53de]::convert::Infallible, rustc_middle[4703dced25f2e482]::ty::layout::LayoutError>> as core[55a6db15fdfe53de]::iter::traits::iterator::Iterator>::next
  30:     0x7f132c4c972b - <alloc[d1443e666f22d28e]::vec::Vec<rustc_target[abca9d85455cc236]::abi::TyAndLayout<rustc_middle[4703dced25f2e482]::ty::Ty>> as alloc[d1443e666f22d28e]::vec::spec_from_iter::SpecFromIter<rustc_target[abca9d85455cc236]::abi::TyAndLayout<rustc_middle[4703dced25f2e482]::ty::Ty>, core[55a6db15fdfe53de]::iter::adapters::GenericShunt<core[55a6db15fdfe53de]::iter::adapters::map::Map<core[55a6db15fdfe53de]::slice::iter::Iter<rustc_middle[4703dced25f2e482]::ty::FieldDef>, <rustc_middle[4703dced25f2e482]::ty::layout::LayoutCx<rustc_middle[4703dced25f2e482]::ty::context::TyCtxt>>::layout_of_uncached::{closure#5}::{closure#0}>, core[55a6db15fdfe53de]::result::Result<core[55a6db15fdfe53de]::convert::Infallible, rustc_middle[4703dced25f2e482]::ty::layout::LayoutError>>>>::from_iter
  31:     0x7f132c47e1eb - core[55a6db15fdfe53de]::iter::adapters::try_process::<core[55a6db15fdfe53de]::iter::adapters::map::Map<core[55a6db15fdfe53de]::slice::iter::Iter<rustc_middle[4703dced25f2e482]::ty::FieldDef>, <rustc_middle[4703dced25f2e482]::ty::layout::LayoutCx<rustc_middle[4703dced25f2e482]::ty::context::TyCtxt>>::layout_of_uncached::{closure#5}::{closure#0}>, rustc_target[abca9d85455cc236]::abi::TyAndLayout<rustc_middle[4703dced25f2e482]::ty::Ty>, core[55a6db15fdfe53de]::result::Result<core[55a6db15fdfe53de]::convert::Infallible, rustc_middle[4703dced25f2e482]::ty::layout::LayoutError>, <core[55a6db15fdfe53de]::result::Result<alloc[d1443e666f22d28e]::vec::Vec<rustc_target[abca9d85455cc236]::abi::TyAndLayout<rustc_middle[4703dced25f2e482]::ty::Ty>>, rustc_middle[4703dced25f2e482]::ty::layout::LayoutError> as core[55a6db15fdfe53de]::iter::traits::collect::FromIterator<core[55a6db15fdfe53de]::result::Result<rustc_target[abca9d85455cc236]::abi::TyAndLayout<rustc_middle[4703dced25f2e482]::ty::Ty>, rustc_middle[4703dced25f2e482]::ty::layout::LayoutError>>>::from_iter<core[55a6db15fdfe53de]::iter::adapters::map::Map<core[55a6db15fdfe53de]::slice::iter::Iter<rustc_middle[4703dced25f2e482]::ty::FieldDef>, <rustc_middle[4703dced25f2e482]::ty::layout::LayoutCx<rustc_middle[4703dced25f2e482]::ty::context::TyCtxt>>::layout_of_uncached::{closure#5}::{closure#0}>>::{closure#0}, alloc[d1443e666f22d28e]::vec::Vec<rustc_target[abca9d85455cc236]::abi::TyAndLayout<rustc_middle[4703dced25f2e482]::ty::Ty>>>
  32:     0x7f132c3d96e2 - <core[55a6db15fdfe53de]::iter::adapters::map::Map<core[55a6db15fdfe53de]::slice::iter::Iter<rustc_middle[4703dced25f2e482]::ty::VariantDef>, <rustc_middle[4703dced25f2e482]::ty::layout::LayoutCx<rustc_middle[4703dced25f2e482]::ty::context::TyCtxt>>::layout_of_uncached::{closure#5}> as core[55a6db15fdfe53de]::iter::traits::iterator::Iterator>::try_fold::<(), <core[55a6db15fdfe53de]::iter::adapters::GenericShunt<core[55a6db15fdfe53de]::iter::adapters::map::Map<core[55a6db15fdfe53de]::slice::iter::Iter<rustc_middle[4703dced25f2e482]::ty::VariantDef>, <rustc_middle[4703dced25f2e482]::ty::layout::LayoutCx<rustc_middle[4703dced25f2e482]::ty::context::TyCtxt>>::layout_of_uncached::{closure#5}>, core[55a6db15fdfe53de]::result::Result<core[55a6db15fdfe53de]::convert::Infallible, rustc_middle[4703dced25f2e482]::ty::layout::LayoutError>> as core[55a6db15fdfe53de]::iter::traits::iterator::Iterator>::try_fold<(), core[55a6db15fdfe53de]::iter::traits::iterator::Iterator::try_for_each::call<alloc[d1443e666f22d28e]::vec::Vec<rustc_target[abca9d85455cc236]::abi::TyAndLayout<rustc_middle[4703dced25f2e482]::ty::Ty>>, core[55a6db15fdfe53de]::ops::control_flow::ControlFlow<alloc[d1443e666f22d28e]::vec::Vec<rustc_target[abca9d85455cc236]::abi::TyAndLayout<rustc_middle[4703dced25f2e482]::ty::Ty>>>, core[55a6db15fdfe53de]::ops::control_flow::ControlFlow<alloc[d1443e666f22d28e]::vec::Vec<rustc_target[abca9d85455cc236]::abi::TyAndLayout<rustc_middle[4703dced25f2e482]::ty::Ty>>>::Break>::{closure#0}, core[55a6db15fdfe53de]::ops::control_flow::ControlFlow<alloc[d1443e666f22d28e]::vec::Vec<rustc_target[abca9d85455cc236]::abi::TyAndLayout<rustc_middle[4703dced25f2e482]::ty::Ty>>>>::{closure#0}, core[55a6db15fdfe53de]::ops::control_flow::ControlFlow<core[55a6db15fdfe53de]::ops::control_flow::ControlFlow<alloc[d1443e666f22d28e]::vec::Vec<rustc_target[abca9d85455cc236]::abi::TyAndLayout<rustc_middle[4703dced25f2e482]::ty::Ty>>>>>
  33:     0x7f132c48696b - <core[55a6db15fdfe53de]::iter::adapters::GenericShunt<core[55a6db15fdfe53de]::iter::adapters::map::Map<core[55a6db15fdfe53de]::slice::iter::Iter<rustc_middle[4703dced25f2e482]::ty::VariantDef>, <rustc_middle[4703dced25f2e482]::ty::layout::LayoutCx<rustc_middle[4703dced25f2e482]::ty::context::TyCtxt>>::layout_of_uncached::{closure#5}>, core[55a6db15fdfe53de]::result::Result<core[55a6db15fdfe53de]::convert::Infallible, rustc_middle[4703dced25f2e482]::ty::layout::LayoutError>> as core[55a6db15fdfe53de]::iter::traits::iterator::Iterator>::next
  34:     0x7f132c4c8549 - <alloc[d1443e666f22d28e]::vec::Vec<alloc[d1443e666f22d28e]::vec::Vec<rustc_target[abca9d85455cc236]::abi::TyAndLayout<rustc_middle[4703dced25f2e482]::ty::Ty>>> as alloc[d1443e666f22d28e]::vec::spec_from_iter::SpecFromIter<alloc[d1443e666f22d28e]::vec::Vec<rustc_target[abca9d85455cc236]::abi::TyAndLayout<rustc_middle[4703dced25f2e482]::ty::Ty>>, core[55a6db15fdfe53de]::iter::adapters::GenericShunt<core[55a6db15fdfe53de]::iter::adapters::map::Map<core[55a6db15fdfe53de]::slice::iter::Iter<rustc_middle[4703dced25f2e482]::ty::VariantDef>, <rustc_middle[4703dced25f2e482]::ty::layout::LayoutCx<rustc_middle[4703dced25f2e482]::ty::context::TyCtxt>>::layout_of_uncached::{closure#5}>, core[55a6db15fdfe53de]::result::Result<core[55a6db15fdfe53de]::convert::Infallible, rustc_middle[4703dced25f2e482]::ty::layout::LayoutError>>>>::from_iter
  35:     0x7f132c47e0fb - core[55a6db15fdfe53de]::iter::adapters::try_process::<core[55a6db15fdfe53de]::iter::adapters::map::Map<core[55a6db15fdfe53de]::slice::iter::Iter<rustc_middle[4703dced25f2e482]::ty::VariantDef>, <rustc_middle[4703dced25f2e482]::ty::layout::LayoutCx<rustc_middle[4703dced25f2e482]::ty::context::TyCtxt>>::layout_of_uncached::{closure#5}>, alloc[d1443e666f22d28e]::vec::Vec<rustc_target[abca9d85455cc236]::abi::TyAndLayout<rustc_middle[4703dced25f2e482]::ty::Ty>>, core[55a6db15fdfe53de]::result::Result<core[55a6db15fdfe53de]::convert::Infallible, rustc_middle[4703dced25f2e482]::ty::layout::LayoutError>, <core[55a6db15fdfe53de]::result::Result<rustc_index[debbbd9e44e1d9d3]::vec::IndexVec<rustc_target[abca9d85455cc236]::abi::VariantIdx, alloc[d1443e666f22d28e]::vec::Vec<rustc_target[abca9d85455cc236]::abi::TyAndLayout<rustc_middle[4703dced25f2e482]::ty::Ty>>>, rustc_middle[4703dced25f2e482]::ty::layout::LayoutError> as core[55a6db15fdfe53de]::iter::traits::collect::FromIterator<core[55a6db15fdfe53de]::result::Result<alloc[d1443e666f22d28e]::vec::Vec<rustc_target[abca9d85455cc236]::abi::TyAndLayout<rustc_middle[4703dced25f2e482]::ty::Ty>>, rustc_middle[4703dced25f2e482]::ty::layout::LayoutError>>>::from_iter<core[55a6db15fdfe53de]::iter::adapters::map::Map<core[55a6db15fdfe53de]::slice::iter::Iter<rustc_middle[4703dced25f2e482]::ty::VariantDef>, <rustc_middle[4703dced25f2e482]::ty::layout::LayoutCx<rustc_middle[4703dced25f2e482]::ty::context::TyCtxt>>::layout_of_uncached::{closure#5}>>::{closure#0}, rustc_index[debbbd9e44e1d9d3]::vec::IndexVec<rustc_target[abca9d85455cc236]::abi::VariantIdx, alloc[d1443e666f22d28e]::vec::Vec<rustc_target[abca9d85455cc236]::abi::TyAndLayout<rustc_middle[4703dced25f2e482]::ty::Ty>>>>
  36:     0x7f132c3f1463 - <rustc_middle[4703dced25f2e482]::ty::layout::LayoutCx<rustc_middle[4703dced25f2e482]::ty::context::TyCtxt>>::layout_of_uncached
  37:     0x7f132c40c9cc - rustc_middle[4703dced25f2e482]::ty::layout::layout_of
  38:     0x7f132b748f88 - rustc_query_system[e6ab6bb4cb63ce3f]::query::plumbing::get_query::<rustc_query_impl[36cd38aa3e85ef29]::queries::layout_of, rustc_query_impl[36cd38aa3e85ef29]::plumbing::QueryCtxt>
  39:     0x7f132b2830a1 - <rustc_query_impl[36cd38aa3e85ef29]::Queries as rustc_middle[4703dced25f2e482]::ty::query::QueryEngine>::layout_of
  40:     0x7f132c40cd48 - rustc_middle[4703dced25f2e482]::ty::layout::layout_of
  41:     0x7f132b748f88 - rustc_query_system[e6ab6bb4cb63ce3f]::query::plumbing::get_query::<rustc_query_impl[36cd38aa3e85ef29]::queries::layout_of, rustc_query_impl[36cd38aa3e85ef29]::plumbing::QueryCtxt>
  42:     0x7f132b2830a1 - <rustc_query_impl[36cd38aa3e85ef29]::Queries as rustc_middle[4703dced25f2e482]::ty::query::QueryEngine>::layout_of
  43:     0x7f1329fc2237 - <rustc_mir_transform[867a8f12a738183d]::const_prop::CanConstProp>::check
  44:     0x7f132a1037eb - <rustc_mir_transform[867a8f12a738183d]::const_prop_lint::ConstProp as rustc_mir_transform[867a8f12a738183d]::pass_manager::MirLint>::run_lint
  45:     0x7f1329facf57 - rustc_mir_transform[867a8f12a738183d]::pass_manager::run_passes
  46:     0x7f132a06b2f0 - rustc_mir_transform[867a8f12a738183d]::run_post_borrowck_cleanup_passes
  47:     0x7f132a06ae78 - rustc_mir_transform[867a8f12a738183d]::mir_drops_elaborated_and_const_checked
  48:     0x7f132b61eef3 - rustc_query_system[e6ab6bb4cb63ce3f]::query::plumbing::try_execute_query::<rustc_query_impl[36cd38aa3e85ef29]::plumbing::QueryCtxt, rustc_query_system[e6ab6bb4cb63ce3f]::query::caches::DefaultCache<rustc_middle[4703dced25f2e482]::ty::WithOptConstParam<rustc_span[d8fe0b440f56fba7]::def_id::LocalDefId>, &rustc_data_structures[902cfba2b9bf1e81]::steal::Steal<rustc_middle[4703dced25f2e482]::mir::Body>>>
  49:     0x7f132b742e43 - rustc_query_system[e6ab6bb4cb63ce3f]::query::plumbing::get_query::<rustc_query_impl[36cd38aa3e85ef29]::queries::mir_drops_elaborated_and_const_checked, rustc_query_impl[36cd38aa3e85ef29]::plumbing::QueryCtxt>
  50:     0x7f132b256b87 - <rustc_query_impl[36cd38aa3e85ef29]::Queries as rustc_middle[4703dced25f2e482]::ty::query::QueryEngine>::mir_drops_elaborated_and_const_checked
  51:     0x7f132a06bb93 - rustc_mir_transform[867a8f12a738183d]::optimized_mir
  52:     0x7f132b653238 - rustc_query_system[e6ab6bb4cb63ce3f]::query::plumbing::try_execute_query::<rustc_query_impl[36cd38aa3e85ef29]::plumbing::QueryCtxt, rustc_query_system[e6ab6bb4cb63ce3f]::query::caches::DefaultCache<rustc_span[d8fe0b440f56fba7]::def_id::DefId, &rustc_middle[4703dced25f2e482]::mir::Body>>
  53:     0x7f132b6fc154 - rustc_query_system[e6ab6bb4cb63ce3f]::query::plumbing::get_query::<rustc_query_impl[36cd38aa3e85ef29]::queries::optimized_mir, rustc_query_impl[36cd38aa3e85ef29]::plumbing::QueryCtxt>
  54:     0x7f132b258659 - <rustc_query_impl[36cd38aa3e85ef29]::Queries as rustc_middle[4703dced25f2e482]::ty::query::QueryEngine>::optimized_mir
  55:     0x7f132c5621e9 - <rustc_middle[4703dced25f2e482]::ty::context::TyCtxt>::instance_mir
  56:     0x7f1329edd18a - rustc_monomorphize[f519a9d68a020ed3]::collector::collect_neighbours
  57:     0x7f1329ed7fe0 - rustc_monomorphize[f519a9d68a020ed3]::collector::collect_items_rec
  58:     0x7f1329ee3f7d - <core[55a6db15fdfe53de]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[902cfba2b9bf1e81]::sync::par_for_each_in<alloc[d1443e666f22d28e]::vec::Vec<rustc_middle[4703dced25f2e482]::mir::mono::MonoItem>, rustc_monomorphize[f519a9d68a020ed3]::collector::collect_crate_mono_items::{closure#1}::{closure#0}>::{closure#0}::{closure#0}> as core[55a6db15fdfe53de]::ops::function::FnOnce<()>>::call_once
  59:     0x7f1329f1ce15 - std[e791036a24d368b7]::panicking::try::<(), core[55a6db15fdfe53de]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[902cfba2b9bf1e81]::sync::par_for_each_in<alloc[d1443e666f22d28e]::vec::Vec<rustc_middle[4703dced25f2e482]::mir::mono::MonoItem>, rustc_monomorphize[f519a9d68a020ed3]::collector::collect_crate_mono_items::{closure#1}::{closure#0}>::{closure#0}::{closure#0}>>
  60:     0x7f1329ef6fee - rustc_data_structures[902cfba2b9bf1e81]::sync::par_for_each_in::<alloc[d1443e666f22d28e]::vec::Vec<rustc_middle[4703dced25f2e482]::mir::mono::MonoItem>, rustc_monomorphize[f519a9d68a020ed3]::collector::collect_crate_mono_items::{closure#1}::{closure#0}>
  61:     0x7f1329f1baf0 - <rustc_session[43a2b7bd1177bdfc]::session::Session>::time::<(), rustc_monomorphize[f519a9d68a020ed3]::collector::collect_crate_mono_items::{closure#1}>
  62:     0x7f1329ed4d07 - rustc_monomorphize[f519a9d68a020ed3]::collector::collect_crate_mono_items
  63:     0x7f1329ee59ca - rustc_monomorphize[f519a9d68a020ed3]::partitioning::collect_and_partition_mono_items
  64:     0x7f132b675751 - rustc_query_system[e6ab6bb4cb63ce3f]::query::plumbing::try_execute_query::<rustc_query_impl[36cd38aa3e85ef29]::plumbing::QueryCtxt, rustc_query_system[e6ab6bb4cb63ce3f]::query::caches::DefaultCache<(), (&std[e791036a24d368b7]::collections::hash::set::HashSet<rustc_span[d8fe0b440f56fba7]::def_id::DefId, core[55a6db15fdfe53de]::hash::BuildHasherDefault<rustc_hash[8c8adb68edebd683]::FxHasher>>, &[rustc_middle[4703dced25f2e482]::mir::mono::CodegenUnit])>>
  65:     0x7f132b73e6aa - rustc_query_system[e6ab6bb4cb63ce3f]::query::plumbing::get_query::<rustc_query_impl[36cd38aa3e85ef29]::queries::collect_and_partition_mono_items, rustc_query_impl[36cd38aa3e85ef29]::plumbing::QueryCtxt>
  66:     0x7f132b29c4f9 - <rustc_query_impl[36cd38aa3e85ef29]::Queries as rustc_middle[4703dced25f2e482]::ty::query::QueryEngine>::collect_and_partition_mono_items
  67:     0x7f1329cec241 - rustc_codegen_ssa[13b8157fa627809d]::base::codegen_crate::<rustc_codegen_llvm[4bc59b0b8439f48f]::LlvmCodegenBackend>
  68:     0x7f1329da21cd - <rustc_codegen_llvm[4bc59b0b8439f48f]::LlvmCodegenBackend as rustc_codegen_ssa[13b8157fa627809d]::traits::backend::CodegenBackend>::codegen_crate
  69:     0x7f1329c0224b - <rustc_session[43a2b7bd1177bdfc]::session::Session>::time::<alloc[d1443e666f22d28e]::boxed::Box<dyn core[55a6db15fdfe53de]::any::Any>, rustc_interface[aa4f229c7af5263c]::passes::start_codegen::{closure#0}>
  70:     0x7f1329bbbe25 - <rustc_interface[aa4f229c7af5263c]::passes::QueryContext>::enter::<<rustc_interface[aa4f229c7af5263c]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[55a6db15fdfe53de]::result::Result<alloc[d1443e666f22d28e]::boxed::Box<dyn core[55a6db15fdfe53de]::any::Any>, rustc_errors[c0fe8da4a22912a3]::ErrorGuaranteed>>
  71:     0x7f1329ba3c5d - <rustc_interface[aa4f229c7af5263c]::queries::Queries>::ongoing_codegen
  72:     0x7f1329a85fe6 - <rustc_interface[aa4f229c7af5263c]::interface::Compiler>::enter::<rustc_driver[c5980153e48cf13b]::run_compiler::{closure#1}::{closure#2}, core[55a6db15fdfe53de]::result::Result<core[55a6db15fdfe53de]::option::Option<rustc_interface[aa4f229c7af5263c]::queries::Linker>, rustc_errors[c0fe8da4a22912a3]::ErrorGuaranteed>>
  73:     0x7f1329a6b071 - rustc_span[d8fe0b440f56fba7]::with_source_map::<core[55a6db15fdfe53de]::result::Result<(), rustc_errors[c0fe8da4a22912a3]::ErrorGuaranteed>, rustc_interface[aa4f229c7af5263c]::interface::create_compiler_and_run<core[55a6db15fdfe53de]::result::Result<(), rustc_errors[c0fe8da4a22912a3]::ErrorGuaranteed>, rustc_driver[c5980153e48cf13b]::run_compiler::{closure#1}>::{closure#1}>
  74:     0x7f1329aa0561 - rustc_interface[aa4f229c7af5263c]::interface::create_compiler_and_run::<core[55a6db15fdfe53de]::result::Result<(), rustc_errors[c0fe8da4a22912a3]::ErrorGuaranteed>, rustc_driver[c5980153e48cf13b]::run_compiler::{closure#1}>
  75:     0x7f1329a62b92 - <scoped_tls[317155f9ccb9c1ae]::ScopedKey<rustc_span[d8fe0b440f56fba7]::SessionGlobals>>::set::<rustc_interface[aa4f229c7af5263c]::interface::run_compiler<core[55a6db15fdfe53de]::result::Result<(), rustc_errors[c0fe8da4a22912a3]::ErrorGuaranteed>, rustc_driver[c5980153e48cf13b]::run_compiler::{closure#1}>::{closure#0}, core[55a6db15fdfe53de]::result::Result<(), rustc_errors[c0fe8da4a22912a3]::ErrorGuaranteed>>
  76:     0x7f1329a6ede9 - std[e791036a24d368b7]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[aa4f229c7af5263c]::util::run_in_thread_pool_with_globals<rustc_interface[aa4f229c7af5263c]::interface::run_compiler<core[55a6db15fdfe53de]::result::Result<(), rustc_errors[c0fe8da4a22912a3]::ErrorGuaranteed>, rustc_driver[c5980153e48cf13b]::run_compiler::{closure#1}>::{closure#0}, core[55a6db15fdfe53de]::result::Result<(), rustc_errors[c0fe8da4a22912a3]::ErrorGuaranteed>>::{closure#0}, core[55a6db15fdfe53de]::result::Result<(), rustc_errors[c0fe8da4a22912a3]::ErrorGuaranteed>>
  77:     0x7f1329ae94ee - std[e791036a24d368b7]::panicking::try::<core[55a6db15fdfe53de]::result::Result<(), rustc_errors[c0fe8da4a22912a3]::ErrorGuaranteed>, core[55a6db15fdfe53de]::panic::unwind_safe::AssertUnwindSafe<<std[e791036a24d368b7]::thread::Builder>::spawn_unchecked_<rustc_interface[aa4f229c7af5263c]::util::run_in_thread_pool_with_globals<rustc_interface[aa4f229c7af5263c]::interface::run_compiler<core[55a6db15fdfe53de]::result::Result<(), rustc_errors[c0fe8da4a22912a3]::ErrorGuaranteed>, rustc_driver[c5980153e48cf13b]::run_compiler::{closure#1}>::{closure#0}, core[55a6db15fdfe53de]::result::Result<(), rustc_errors[c0fe8da4a22912a3]::ErrorGuaranteed>>::{closure#0}, core[55a6db15fdfe53de]::result::Result<(), rustc_errors[c0fe8da4a22912a3]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  78:     0x7f1329ae4800 - <<std[e791036a24d368b7]::thread::Builder>::spawn_unchecked_<rustc_interface[aa4f229c7af5263c]::util::run_in_thread_pool_with_globals<rustc_interface[aa4f229c7af5263c]::interface::run_compiler<core[55a6db15fdfe53de]::result::Result<(), rustc_errors[c0fe8da4a22912a3]::ErrorGuaranteed>, rustc_driver[c5980153e48cf13b]::run_compiler::{closure#1}>::{closure#0}, core[55a6db15fdfe53de]::result::Result<(), rustc_errors[c0fe8da4a22912a3]::ErrorGuaranteed>>::{closure#0}, core[55a6db15fdfe53de]::result::Result<(), rustc_errors[c0fe8da4a22912a3]::ErrorGuaranteed>>::{closure#1} as core[55a6db15fdfe53de]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  79:     0x7f13290ffd35 - std::sys::unix::thread::Thread::new::thread_start::h47417f75837f5cd1
  80:     0x7f1328e9cb43 - <unknown>
  81:     0x7f1328f2ea00 - <unknown>
  82:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.65.0-nightly (87b6fe291 2022-08-18) running on x86_64-unknown-linux-gnu

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



---- [ui] src/test/ui/lint/must_not_suspend/ref.rs stdout ----
diff of stderr:

- error: `Umm` held across a suspend point, but should not be
-   --> $DIR/ref.rs:18:26
+ error: reference to `Umm` held across a suspend point, but should not be
3    |
4 LL |         let guard = &mut self.u;
-    |                          ^^^^^^
+    |             ^^^^^
+    |             ^^^^^
6 LL |
7 LL |         other().await;
8    |                ------ the value is held across this suspend point

13 LL | #![deny(must_not_suspend)]
14    |         ^^^^^^^^^^^^^^^^
15 note: You gotta use Umm's, ya know?
-   --> $DIR/ref.rs:18:26
17    |
18 LL |         let guard = &mut self.u;
-    |                          ^^^^^^
+    |             ^^^^^
+    |             ^^^^^
20 help: consider using a block (`{ ... }`) to shrink the value's scope, ending before the suspend point
-   --> $DIR/ref.rs:18:26
22    |
23 LL |         let guard = &mut self.u;
-    |                          ^^^^^^
+    |             ^^^^^
---
To only update this specific test, also pass `--test-args lint/must_not_suspend/ref.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/must_not_suspend/ref.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/must_not_suspend/ref" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/must_not_suspend/ref/auxiliary"
stdout: none
--- stderr -------------------------------
error: reference to `Umm` held across a suspend point, but should not be
   |
   |
LL |         let guard = &mut self.u; //~ ERROR `Umm` held across
LL |
LL |         other().await;
LL |         other().await;
   |                ------ the value is held across this suspend point
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/must_not_suspend/ref.rs:3:9
   |
   |
LL | #![deny(must_not_suspend)]
   |         ^^^^^^^^^^^^^^^^
note: You gotta use Umm's, ya know?
   |
   |
LL |         let guard = &mut self.u; //~ ERROR `Umm` held across
   |             ^^^^^
help: consider using a block (`{ ... }`) to shrink the value's scope, ending before the suspend point
   |
   |
LL |         let guard = &mut self.u; //~ ERROR `Umm` held across

error: aborting due to previous error
------------------------------------------

