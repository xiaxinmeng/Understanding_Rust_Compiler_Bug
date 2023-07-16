plain
........................................................................................ 352/13165
........................................................................................ 440/13165
........................................................................................ 528/13165
........................................................................................ 616/13165
...........F.............F.............................................................. 704/13165
..........................................i............................................. 880/13165
........................................................................................ 968/13165
........................................................................................ 1056/13165
........................................................................................ 1144/13165
---
..........i..........i..........i....................................................... 3872/13165
........................................................................................ 3960/13165
......i................................................................................. 4048/13165
...................................................i.................................... 4136/13165
..........................................................F........F.................... 4224/13165
..................FF.......F................F.....................F..................... 4312/13165
........................................................................................ 4488/13165
........................................................................................ 4576/13165
........................................................................................ 4664/13165
........................................................................................ 4752/13165
---
---- [ui] src/test/ui/async-await/issue-62658.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issue-62658.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-62658" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-62658/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: compiler/rustc_mir_transform/src/generator.rs:760:13: Broken MIR: generator contains type [u8; 17] in MIR, but typeck only knows about {ResumeTy, impl Future<Output = ()>, ()} and []
   |
LL |   async fn foo() {
   |  ________________^
LL | |     // This suspend should be the largest variant.
LL | |     // This suspend should be the largest variant.
LL | |     {
LL | |         let x = [0u8; 17];
LL | |     }
LL | | }
   | |_^


thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1331:9
stack backtrace:
   0:     0x7f3b132a479c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h50e432a33816f17f
   1:     0x7f3b1330a8b8 - core::fmt::write::he1187f1c735a6d05
   2:     0x7f3b13294d31 - std::io::Write::write_fmt::h39a703c43ca9e701
   3:     0x7f3b132a77ae - std::panicking::default_hook::{{closure}}::h3ba44f2c4a1ab003
   4:     0x7f3b132a7476 - std::panicking::default_hook::hf88b5e434d766a47
   5:     0x7f3b13c609b4 - rustc_driver[a26527cb16da58d4]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f3b132a7f62 - std::panicking::rust_panic_with_hook::hd6afc83c75fe8feb
   7:     0x7f3b14266873 - std[ed124ce8448b1b72]::panicking::begin_panic::<rustc_errors[4c24d11ef95ca307]::ExplicitBug>::{closure#0}
   8:     0x7f3b142620c6 - std[ed124ce8448b1b72]::sys_common::backtrace::__rust_end_short_backtrace::<std[ed124ce8448b1b72]::panicking::begin_panic<rustc_errors[4c24d11ef95ca307]::ExplicitBug>::{closure#0}, !>
   9:     0x7f3b13a8f4c6 - std[ed124ce8448b1b72]::panicking::begin_panic::<rustc_errors[4c24d11ef95ca307]::ExplicitBug>
  10:     0x7f3b14191966 - std[ed124ce8448b1b72]::panic::panic_any::<rustc_errors[4c24d11ef95ca307]::ExplicitBug>
  11:     0x7f3b1418f9d3 - <rustc_errors[4c24d11ef95ca307]::HandlerInner>::span_bug::<rustc_span[85cbc7cdad3332d9]::span_encoding::Span, &alloc[4cd35cc4a7f04eb5]::string::String>
  12:     0x7f3b1418f880 - <rustc_errors[4c24d11ef95ca307]::Handler>::span_bug::<rustc_span[85cbc7cdad3332d9]::span_encoding::Span, &alloc[4cd35cc4a7f04eb5]::string::String>
  13:     0x7f3b143353c5 - rustc_middle[e06d1b0d4a982b9c]::util::bug::opt_span_bug_fmt::<rustc_span[85cbc7cdad3332d9]::span_encoding::Span>::{closure#0}
  14:     0x7f3b1433543b - rustc_middle[e06d1b0d4a982b9c]::ty::context::tls::with_opt::<rustc_middle[e06d1b0d4a982b9c]::util::bug::opt_span_bug_fmt<rustc_span[85cbc7cdad3332d9]::span_encoding::Span>::{closure#0}, ()>::{closure#0}
  15:     0x7f3b1432f8a6 - rustc_middle[e06d1b0d4a982b9c]::ty::context::tls::with_context_opt::<rustc_middle[e06d1b0d4a982b9c]::ty::context::tls::with_opt<rustc_middle[e06d1b0d4a982b9c]::util::bug::opt_span_bug_fmt<rustc_span[85cbc7cdad3332d9]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  16:     0x7f3b1432f819 - rustc_middle[e06d1b0d4a982b9c]::util::bug::opt_span_bug_fmt::<rustc_span[85cbc7cdad3332d9]::span_encoding::Span>
  17:     0x7f3b13a93ff7 - rustc_middle[e06d1b0d4a982b9c]::util::bug::span_bug_fmt::<rustc_span[85cbc7cdad3332d9]::span_encoding::Span>
  18:     0x7f3b1421981f - <rustc_mir_transform[4f795c61437e85be]::generator::StateTransform as rustc_middle[e06d1b0d4a982b9c]::mir::MirPass>::run_pass
  19:     0x7f3b1419f367 - rustc_mir_transform[4f795c61437e85be]::pass_manager::run_passes
  20:     0x7f3b142a4f7e - rustc_mir_transform[4f795c61437e85be]::optimized_mir
  21:     0x7f3b1583b4c0 - rustc_query_system[66850b7ae8af285]::query::plumbing::try_execute_query::<rustc_query_impl[d84d3f2479a1091a]::plumbing::QueryCtxt, rustc_query_system[66850b7ae8af285]::query::caches::DefaultCache<rustc_span[85cbc7cdad3332d9]::def_id::DefId, &rustc_middle[e06d1b0d4a982b9c]::mir::Body>>
  22:     0x7f3b158e8192 - rustc_query_system[66850b7ae8af285]::query::plumbing::get_query::<rustc_query_impl[d84d3f2479a1091a]::queries::optimized_mir, rustc_query_impl[d84d3f2479a1091a]::plumbing::QueryCtxt>
  23:     0x7f3b154166e9 - <rustc_query_impl[d84d3f2479a1091a]::Queries as rustc_middle[e06d1b0d4a982b9c]::ty::query::QueryEngine>::optimized_mir
  24:     0x7f3b166f0dfe - <rustc_middle[e06d1b0d4a982b9c]::ty::context::TyCtxt>::generator_layout
  25:     0x7f3b16685139 - <rustc_middle[e06d1b0d4a982b9c]::ty::layout::LayoutCx<rustc_middle[e06d1b0d4a982b9c]::ty::context::TyCtxt>>::layout_of_uncached
  26:     0x7f3b166af66e - rustc_middle[e06d1b0d4a982b9c]::ty::context::tls::with_context::<rustc_middle[e06d1b0d4a982b9c]::ty::context::tls::with_related_context<rustc_middle[e06d1b0d4a982b9c]::ty::layout::layout_of::{closure#0}, core[cee74f9f94ff5162]::result::Result<rustc_target[25189b89ae46f52c]::abi::TyAndLayout<rustc_middle[e06d1b0d4a982b9c]::ty::Ty>, rustc_middle[e06d1b0d4a982b9c]::ty::layout::LayoutError>>::{closure#0}, core[cee74f9f94ff5162]::result::Result<rustc_target[25189b89ae46f52c]::abi::TyAndLayout<rustc_middle[e06d1b0d4a982b9c]::ty::Ty>, rustc_middle[e06d1b0d4a982b9c]::ty::layout::LayoutError>>::{closure#0}
  27:     0x7f3b166b324f - rustc_middle[e06d1b0d4a982b9c]::ty::layout::layout_of
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
  28:     0x7f3b15939d8d - rustc_query_system[66850b7ae8af285]::query::plumbing::get_query::<rustc_query_impl[d84d3f2479a1091a]::queries::layout_of, rustc_query_impl[d84d3f2479a1091a]::plumbing::QueryCtxt>
  29:     0x7f3b154413c3 - <rustc_query_impl[d84d3f2479a1091a]::Queries as rustc_middle[e06d1b0d4a982b9c]::ty::query::QueryEngine>::layout_of
  30:     0x7f3b16631e3f - <rustc_middle[e06d1b0d4a982b9c]::ty::layout::LayoutCx<rustc_middle[e06d1b0d4a982b9c]::ty::context::TyCtxt> as rustc_middle[e06d1b0d4a982b9c]::ty::layout::LayoutOf>::spanned_layout_of
  31:     0x7f3b16626f4f - <core[cee74f9f94ff5162]::iter::adapters::map::Map<core[cee74f9f94ff5162]::slice::iter::Iter<rustc_middle[e06d1b0d4a982b9c]::ty::FieldDef>, <rustc_middle[e06d1b0d4a982b9c]::ty::layout::LayoutCx<rustc_middle[e06d1b0d4a982b9c]::ty::context::TyCtxt>>::layout_of_uncached::{closure#5}::{closure#0}> as core[cee74f9f94ff5162]::iter::traits::iterator::Iterator>::try_fold::<(), <core[cee74f9f94ff5162]::iter::adapters::GenericShunt<core[cee74f9f94ff5162]::iter::adapters::map::Map<core[cee74f9f94ff5162]::slice::iter::Iter<rustc_middle[e06d1b0d4a982b9c]::ty::FieldDef>, <rustc_middle[e06d1b0d4a982b9c]::ty::layout::LayoutCx<rustc_middle[e06d1b0d4a982b9c]::ty::context::TyCtxt>>::layout_of_uncached::{closure#5}::{closure#0}>, core[cee74f9f94ff5162]::result::Result<core[cee74f9f94ff5162]::convert::Infallible, rustc_middle[e06d1b0d4a982b9c]::ty::layout::LayoutError>> as core[cee74f9f94ff5162]::iter::traits::iterator::Iterator>::try_fold<(), core[cee74f9f94ff5162]::iter::traits::iterator::Iterator::try_for_each::call<rustc_target[25189b89ae46f52c]::abi::TyAndLayout<rustc_middle[e06d1b0d4a982b9c]::ty::Ty>, core[cee74f9f94ff5162]::ops::control_flow::ControlFlow<rustc_target[25189b89ae46f52c]::abi::TyAndLayout<rustc_middle[e06d1b0d4a982b9c]::ty::Ty>>, core[cee74f9f94ff5162]::ops::control_flow::ControlFlow<rustc_target[25189b89ae46f52c]::abi::TyAndLayout<rustc_middle[e06d1b0d4a982b9c]::ty::Ty>>::Break>::{closure#0}, core[cee74f9f94ff5162]::ops::control_flow::ControlFlow<rustc_target[25189b89ae46f52c]::abi::TyAndLayout<rustc_middle[e06d1b0d4a982b9c]::ty::Ty>>>::{closure#0}, core[cee74f9f94ff5162]::ops::control_flow::ControlFlow<core[cee74f9f94ff5162]::ops::control_flow::ControlFlow<rustc_target[25189b89ae46f52c]::abi::TyAndLayout<rustc_middle[e06d1b0d4a982b9c]::ty::Ty>>>>
  32:     0x7f3b1660bcba - <core[cee74f9f94ff5162]::iter::adapters::GenericShunt<core[cee74f9f94ff5162]::iter::adapters::map::Map<core[cee74f9f94ff5162]::slice::iter::Iter<rustc_middle[e06d1b0d4a982b9c]::ty::FieldDef>, <rustc_middle[e06d1b0d4a982b9c]::ty::layout::LayoutCx<rustc_middle[e06d1b0d4a982b9c]::ty::context::TyCtxt>>::layout_of_uncached::{closure#5}::{closure#0}>, core[cee74f9f94ff5162]::result::Result<core[cee74f9f94ff5162]::convert::Infallible, rustc_middle[e06d1b0d4a982b9c]::ty::layout::LayoutError>> as core[cee74f9f94ff5162]::iter::traits::iterator::Iterator>::next
  33:     0x7f3b1664cf8b - <alloc[4cd35cc4a7f04eb5]::vec::Vec<rustc_target[25189b89ae46f52c]::abi::TyAndLayout<rustc_middle[e06d1b0d4a982b9c]::ty::Ty>> as alloc[4cd35cc4a7f04eb5]::vec::spec_from_iter::SpecFromIter<rustc_target[25189b89ae46f52c]::abi::TyAndLayout<rustc_middle[e06d1b0d4a982b9c]::ty::Ty>, core[cee74f9f94ff5162]::iter::adapters::GenericShunt<core[cee74f9f94ff5162]::iter::adapters::map::Map<core[cee74f9f94ff5162]::slice::iter::Iter<rustc_middle[e06d1b0d4a982b9c]::ty::FieldDef>, <rustc_middle[e06d1b0d4a982b9c]::ty::layout::LayoutCx<rustc_middle[e06d1b0d4a982b9c]::ty::context::TyCtxt>>::layout_of_uncached::{closure#5}::{closure#0}>, core[cee74f9f94ff5162]::result::Result<core[cee74f9f94ff5162]::convert::Infallible, rustc_middle[e06d1b0d4a982b9c]::ty::layout::LayoutError>>>>::from_iter
  34:     0x7f3b1660848b - core[cee74f9f94ff5162]::iter::adapters::try_process::<core[cee74f9f94ff5162]::iter::adapters::map::Map<core[cee74f9f94ff5162]::slice::iter::Iter<rustc_middle[e06d1b0d4a982b9c]::ty::FieldDef>, <rustc_middle[e06d1b0d4a982b9c]::ty::layout::LayoutCx<rustc_middle[e06d1b0d4a982b9c]::ty::context::TyCtxt>>::layout_of_uncached::{closure#5}::{closure#0}>, rustc_target[25189b89ae46f52c]::abi::TyAndLayout<rustc_middle[e06d1b0d4a982b9c]::ty::Ty>, core[cee74f9f94ff5162]::result::Result<core[cee74f9f94ff5162]::convert::Infallible, rustc_middle[e06d1b0d4a982b9c]::ty::layout::LayoutError>, <core[cee74f9f94ff5162]::result::Result<alloc[4cd35cc4a7f04eb5]::vec::Vec<rustc_target[25189b89ae46f52c]::abi::TyAndLayout<rustc_middle[e06d1b0d4a982b9c]::ty::Ty>>, rustc_middle[e06d1b0d4a982b9c]::ty::layout::LayoutError> as core[cee74f9f94ff5162]::iter::traits::collect::FromIterator<core[cee74f9f94ff5162]::result::Result<rustc_target[25189b89ae46f52c]::abi::TyAndLayout<rustc_middle[e06d1b0d4a982b9c]::ty::Ty>, rustc_middle[e06d1b0d4a982b9c]::ty::layout::LayoutError>>>::from_iter<core[cee74f9f94ff5162]::iter::adapters::map::Map<core[cee74f9f94ff5162]::slice::iter::Iter<rustc_middle[e06d1b0d4a982b9c]::ty::FieldDef>, <rustc_middle[e06d1b0d4a982b9c]::ty::layout::LayoutCx<rustc_middle[e06d1b0d4a982b9c]::ty::context::TyCtxt>>::layout_of_uncached::{closure#5}::{closure#0}>>::{closure#0}, alloc[4cd35cc4a7f04eb5]::vec::Vec<rustc_target[25189b89ae46f52c]::abi::TyAndLayout<rustc_middle[e06d1b0d4a982b9c]::ty::Ty>>>
  35:     0x7f3b16626e2a - <core[cee74f9f94ff5162]::iter::adapters::map::Map<core[cee74f9f94ff5162]::slice::iter::Iter<rustc_middle[e06d1b0d4a982b9c]::ty::VariantDef>, <rustc_middle[e06d1b0d4a982b9c]::ty::layout::LayoutCx<rustc_middle[e06d1b0d4a982b9c]::ty::context::TyCtxt>>::layout_of_uncached::{closure#5}> as core[cee74f9f94ff5162]::iter::traits::iterator::Iterator>::try_fold::<(), <core[cee74f9f94ff5162]::iter::adapters::GenericShunt<core[cee74f9f94ff5162]::iter::adapters::map::Map<core[cee74f9f94ff5162]::slice::iter::Iter<rustc_middle[e06d1b0d4a982b9c]::ty::VariantDef>, <rustc_middle[e06d1b0d4a982b9c]::ty::layout::LayoutCx<rustc_middle[e06d1b0d4a982b9c]::ty::context::TyCtxt>>::layout_of_uncached::{closure#5}>, core[cee74f9f94ff5162]::result::Result<core[cee74f9f94ff5162]::convert::Infallible, rustc_middle[e06d1b0d4a982b9c]::ty::layout::LayoutError>> as core[cee74f9f94ff5162]::iter::traits::iterator::Iterator>::try_fold<(), core[cee74f9f94ff5162]::iter::traits::iterator::Iterator::try_for_each::call<alloc[4cd35cc4a7f04eb5]::vec::Vec<rustc_target[25189b89ae46f52c]::abi::TyAndLayout<rustc_middle[e06d1b0d4a982b9c]::ty::Ty>>, core[cee74f9f94ff5162]::ops::control_flow::ControlFlow<alloc[4cd35cc4a7f04eb5]::vec::Vec<rustc_target[25189b89ae46f52c]::abi::TyAndLayout<rustc_middle[e06d1b0d4a982b9c]::ty::Ty>>>, core[cee74f9f94ff5162]::ops::control_flow::ControlFlow<alloc[4cd35cc4a7f04eb5]::vec::Vec<rustc_target[25189b89ae46f52c]::abi::TyAndLayout<rustc_middle[e06d1b0d4a982b9c]::ty::Ty>>>::Break>::{closure#0}, core[cee74f9f94ff5162]::ops::control_flow::ControlFlow<alloc[4cd35cc4a7f04eb5]::vec::Vec<rustc_target[25189b89ae46f52c]::abi::TyAndLayout<rustc_middle[e06d1b0d4a982b9c]::ty::Ty>>>>::{closure#0}, core[cee74f9f94ff5162]::ops::control_flow::ControlFlow<core[cee74f9f94ff5162]::ops::control_flow::ControlFlow<alloc[4cd35cc4a7f04eb5]::vec::Vec<rustc_target[25189b89ae46f52c]::abi::TyAndLayout<rustc_middle[e06d1b0d4a982b9c]::ty::Ty>>>>>
  36:     0x7f3b1660bc3b - <core[cee74f9f94ff5162]::iter::adapters::GenericShunt<core[cee74f9f94ff5162]::iter::adapters::map::Map<core[cee74f9f94ff5162]::slice::iter::Iter<rustc_middle[e06d1b0d4a982b9c]::ty::VariantDef>, <rustc_middle[e06d1b0d4a982b9c]::ty::layout::LayoutCx<rustc_middle[e06d1b0d4a982b9c]::ty::context::TyCtxt>>::layout_of_uncached::{closure#5}>, core[cee74f9f94ff5162]::result::Result<core[cee74f9f94ff5162]::convert::Infallible, rustc_middle[e06d1b0d4a982b9c]::ty::layout::LayoutError>> as core[cee74f9f94ff5162]::iter::traits::iterator::Iterator>::next
  37:     0x7f3b1664c149 - <alloc[4cd35cc4a7f04eb5]::vec::Vec<alloc[4cd35cc4a7f04eb5]::vec::Vec<rustc_target[25189b89ae46f52c]::abi::TyAndLayout<rustc_middle[e06d1b0d4a982b9c]::ty::Ty>>> as alloc[4cd35cc4a7f04eb5]::vec::spec_from_iter::SpecFromIter<alloc[4cd35cc4a7f04eb5]::vec::Vec<rustc_target[25189b89ae46f52c]::abi::TyAndLayout<rustc_middle[e06d1b0d4a982b9c]::ty::Ty>>, core[cee74f9f94ff5162]::iter::adapters::GenericShunt<core[cee74f9f94ff5162]::iter::adapters::map::Map<core[cee74f9f94ff5162]::slice::iter::Iter<rustc_middle[e06d1b0d4a982b9c]::ty::VariantDef>, <rustc_middle[e06d1b0d4a982b9c]::ty::layout::LayoutCx<rustc_middle[e06d1b0d4a982b9c]::ty::context::TyCtxt>>::layout_of_uncached::{closure#5}>, core[cee74f9f94ff5162]::result::Result<core[cee74f9f94ff5162]::convert::Infallible, rustc_middle[e06d1b0d4a982b9c]::ty::layout::LayoutError>>>>::from_iter
  38:     0x7f3b1660839b - core[cee74f9f94ff5162]::iter::adapters::try_process::<core[cee74f9f94ff5162]::iter::adapters::map::Map<core[cee74f9f94ff5162]::slice::iter::Iter<rustc_middle[e06d1b0d4a982b9c]::ty::VariantDef>, <rustc_middle[e06d1b0d4a982b9c]::ty::layout::LayoutCx<rustc_middle[e06d1b0d4a982b9c]::ty::context::TyCtxt>>::layout_of_uncached::{closure#5}>, alloc[4cd35cc4a7f04eb5]::vec::Vec<rustc_target[25189b89ae46f52c]::abi::TyAndLayout<rustc_middle[e06d1b0d4a982b9c]::ty::Ty>>, core[cee74f9f94ff5162]::result::Result<core[cee74f9f94ff5162]::convert::Infallible, rustc_middle[e06d1b0d4a982b9c]::ty::layout::LayoutError>, <core[cee74f9f94ff5162]::result::Result<rustc_index[c4648b84434bbbde]::vec::IndexVec<rustc_target[25189b89ae46f52c]::abi::VariantIdx, alloc[4cd35cc4a7f04eb5]::vec::Vec<rustc_target[25189b89ae46f52c]::abi::TyAndLayout<rustc_middle[e06d1b0d4a982b9c]::ty::Ty>>>, rustc_middle[e06d1b0d4a982b9c]::ty::layout::LayoutError> as core[cee74f9f94ff5162]::iter::traits::collect::FromIterator<core[cee74f9f94ff5162]::result::Result<alloc[4cd35cc4a7f04eb5]::vec::Vec<rustc_target[25189b89ae46f52c]::abi::TyAndLayout<rustc_middle[e06d1b0d4a982b9c]::ty::Ty>>, rustc_middle[e06d1b0d4a982b9c]::ty::layout::LayoutError>>>::from_iter<core[cee74f9f94ff5162]::iter::adapters::map::Map<core[cee74f9f94ff5162]::slice::iter::Iter<rustc_middle[e06d1b0d4a982b9c]::ty::VariantDef>, <rustc_middle[e06d1b0d4a982b9c]::ty::layout::LayoutCx<rustc_middle[e06d1b0d4a982b9c]::ty::context::TyCtxt>>::layout_of_uncached::{closure#5}>>::{closure#0}, rustc_index[c4648b84434bbbde]::vec::IndexVec<rustc_target[25189b89ae46f52c]::abi::VariantIdx, alloc[4cd35cc4a7f04eb5]::vec::Vec<rustc_target[25189b89ae46f52c]::abi::TyAndLayout<rustc_middle[e06d1b0d4a982b9c]::ty::Ty>>>>
  39:     0x7f3b16685d79 - <rustc_middle[e06d1b0d4a982b9c]::ty::layout::LayoutCx<rustc_middle[e06d1b0d4a982b9c]::ty::context::TyCtxt>>::layout_of_uncached
  40:     0x7f3b166af66e - rustc_middle[e06d1b0d4a982b9c]::ty::context::tls::with_context::<rustc_middle[e06d1b0d4a982b9c]::ty::context::tls::with_related_context<rustc_middle[e06d1b0d4a982b9c]::ty::layout::layout_of::{closure#0}, core[cee74f9f94ff5162]::result::Result<rustc_target[25189b89ae46f52c]::abi::TyAndLayout<rustc_middle[e06d1b0d4a982b9c]::ty::Ty>, rustc_middle[e06d1b0d4a982b9c]::ty::layout::LayoutError>>::{closure#0}, core[cee74f9f94ff5162]::result::Result<rustc_target[25189b89ae46f52c]::abi::TyAndLayout<rustc_middle[e06d1b0d4a982b9c]::ty::Ty>, rustc_middle[e06d1b0d4a982b9c]::ty::layout::LayoutError>>::{closure#0}
  41:     0x7f3b166b324f - rustc_middle[e06d1b0d4a982b9c]::ty::layout::layout_of
  42:     0x7f3b15939d8d - rustc_query_system[66850b7ae8af285]::query::plumbing::get_query::<rustc_query_impl[d84d3f2479a1091a]::queries::layout_of, rustc_query_impl[d84d3f2479a1091a]::plumbing::QueryCtxt>
  43:     0x7f3b154413c3 - <rustc_query_impl[d84d3f2479a1091a]::Queries as rustc_middle[e06d1b0d4a982b9c]::ty::query::QueryEngine>::layout_of
  44:     0x7f3b166af709 - rustc_middle[e06d1b0d4a982b9c]::ty::context::tls::with_context::<rustc_middle[e06d1b0d4a982b9c]::ty::context::tls::with_related_context<rustc_middle[e06d1b0d4a982b9c]::ty::layout::layout_of::{closure#0}, core[cee74f9f94ff5162]::result::Result<rustc_target[25189b89ae46f52c]::abi::TyAndLayout<rustc_middle[e06d1b0d4a982b9c]::ty::Ty>, rustc_middle[e06d1b0d4a982b9c]::ty::layout::LayoutError>>::{closure#0}, core[cee74f9f94ff5162]::result::Result<rustc_target[25189b89ae46f52c]::abi::TyAndLayout<rustc_middle[e06d1b0d4a982b9c]::ty::Ty>, rustc_middle[e06d1b0d4a982b9c]::ty::layout::LayoutError>>::{closure#0}
  45:     0x7f3b166b324f - rustc_middle[e06d1b0d4a982b9c]::ty::layout::layout_of
  46:     0x7f3b15939d8d - rustc_query_system[66850b7ae8af285]::query::plumbing::get_query::<rustc_query_impl[d84d3f2479a1091a]::queries::layout_of, rustc_query_impl[d84d3f2479a1091a]::plumbing::QueryCtxt>
  47:     0x7f3b154413c3 - <rustc_query_impl[d84d3f2479a1091a]::Queries as rustc_middle[e06d1b0d4a982b9c]::ty::query::QueryEngine>::layout_of
  48:     0x7f3b14136fbb - <rustc_mir_transform[4f795c61437e85be]::const_prop_lint::ConstProp as rustc_mir_transform[4f795c61437e85be]::pass_manager::MirLint>::run_lint
  49:     0x7f3b1419f367 - rustc_mir_transform[4f795c61437e85be]::pass_manager::run_passes
  50:     0x7f3b142a47e0 - rustc_mir_transform[4f795c61437e85be]::run_post_borrowck_cleanup_passes
  51:     0x7f3b142a3eb7 - rustc_mir_transform[4f795c61437e85be]::mir_drops_elaborated_and_const_checked
  52:     0x7f3b15807b84 - rustc_query_system[66850b7ae8af285]::query::plumbing::try_execute_query::<rustc_query_impl[d84d3f2479a1091a]::plumbing::QueryCtxt, rustc_query_system[66850b7ae8af285]::query::caches::DefaultCache<rustc_middle[e06d1b0d4a982b9c]::ty::WithOptConstParam<rustc_span[85cbc7cdad3332d9]::def_id::LocalDefId>, &rustc_data_structures[176448742242e606]::steal::Steal<rustc_middle[e06d1b0d4a982b9c]::mir::Body>>>
  53:     0x7f3b15933723 - rustc_query_system[66850b7ae8af285]::query::plumbing::get_query::<rustc_query_impl[d84d3f2479a1091a]::queries::mir_drops_elaborated_and_const_checked, rustc_query_impl[d84d3f2479a1091a]::plumbing::QueryCtxt>
  54:     0x7f3b15414b8a - <rustc_query_impl[d84d3f2479a1091a]::Queries as rustc_middle[e06d1b0d4a982b9c]::ty::query::QueryEngine>::mir_drops_elaborated_and_const_checked
  55:     0x7f3b142a4e48 - rustc_mir_transform[4f795c61437e85be]::optimized_mir
  56:     0x7f3b1583b4c0 - rustc_query_system[66850b7ae8af285]::query::plumbing::try_execute_query::<rustc_query_impl[d84d3f2479a1091a]::plumbing::QueryCtxt, rustc_query_system[66850b7ae8af285]::query::caches::DefaultCache<rustc_span[85cbc7cdad3332d9]::def_id::DefId, &rustc_middle[e06d1b0d4a982b9c]::mir::Body>>
  57:     0x7f3b158e8192 - rustc_query_system[66850b7ae8af285]::query::plumbing::get_query::<rustc_query_impl[d84d3f2479a1091a]::queries::optimized_mir, rustc_query_impl[d84d3f2479a1091a]::plumbing::QueryCtxt>
  58:     0x7f3b154166e9 - <rustc_query_impl[d84d3f2479a1091a]::Queries as rustc_middle[e06d1b0d4a982b9c]::ty::query::QueryEngine>::optimized_mir
  59:     0x7f3b166ffcfd - <rustc_middle[e06d1b0d4a982b9c]::ty::context::TyCtxt>::instance_mir
  60:     0x7f3b140cef95 - rustc_monomorphize[6a7659ee93aed75a]::collector::collect_neighbours
  61:     0x7f3b140c9a6a - rustc_monomorphize[6a7659ee93aed75a]::collector::collect_items_rec
  62:     0x7f3b1410f981 - <rustc_session[f3aed6530d3c609e]::session::Session>::time::<(), rustc_monomorphize[6a7659ee93aed75a]::collector::collect_crate_mono_items::{closure#1}>
  63:     0x7f3b140c5ce4 - rustc_monomorphize[6a7659ee93aed75a]::collector::collect_crate_mono_items
  64:     0x7f3b140d63ba - rustc_monomorphize[6a7659ee93aed75a]::partitioning::collect_and_partition_mono_items
  65:     0x7f3b1585d652 - rustc_query_system[66850b7ae8af285]::query::plumbing::try_execute_query::<rustc_query_impl[d84d3f2479a1091a]::plumbing::QueryCtxt, rustc_query_system[66850b7ae8af285]::query::caches::DefaultCache<(), (&std[ed124ce8448b1b72]::collections::hash::set::HashSet<rustc_span[85cbc7cdad3332d9]::def_id::DefId, core[cee74f9f94ff5162]::hash::BuildHasherDefault<rustc_hash[a96122035317b32e]::FxHasher>>, &[rustc_middle[e06d1b0d4a982b9c]::mir::mono::CodegenUnit])>>
  66:     0x7f3b1592eb4a - rustc_query_system[66850b7ae8af285]::query::plumbing::get_query::<rustc_query_impl[d84d3f2479a1091a]::queries::collect_and_partition_mono_items, rustc_query_impl[d84d3f2479a1091a]::plumbing::QueryCtxt>
  67:     0x7f3b1545a419 - <rustc_query_impl[d84d3f2479a1091a]::Queries as rustc_middle[e06d1b0d4a982b9c]::ty::query::QueryEngine>::collect_and_partition_mono_items
  68:     0x7f3b13ec2b08 - rustc_codegen_ssa[c9066fae9df7bbdc]::base::codegen_crate::<rustc_codegen_llvm[f25c4d6ffc7893b1]::LlvmCodegenBackend>
  69:     0x7f3b13fade59 - <rustc_codegen_llvm[f25c4d6ffc7893b1]::LlvmCodegenBackend as rustc_codegen_ssa[c9066fae9df7bbdc]::traits::backend::CodegenBackend>::codegen_crate
  70:     0x7f3b13dcd401 - <rustc_session[f3aed6530d3c609e]::session::Session>::time::<alloc[4cd35cc4a7f04eb5]::boxed::Box<dyn core[cee74f9f94ff5162]::any::Any>, rustc_interface[c2e2522df7b8878]::passes::start_codegen::{closure#0}>
  71:     0x7f3b13d75266 - <rustc_interface[c2e2522df7b8878]::passes::QueryContext>::enter::<<rustc_interface[c2e2522df7b8878]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[cee74f9f94ff5162]::result::Result<alloc[4cd35cc4a7f04eb5]::boxed::Box<dyn core[cee74f9f94ff5162]::any::Any>, rustc_errors[4c24d11ef95ca307]::ErrorGuaranteed>>
  72:     0x7f3b13d8eead - <rustc_interface[c2e2522df7b8878]::queries::Queries>::ongoing_codegen
  73:     0x7f3b13c6816c - <rustc_interface[c2e2522df7b8878]::interface::Compiler>::enter::<rustc_driver[a26527cb16da58d4]::run_compiler::{closure#1}::{closure#2}, core[cee74f9f94ff5162]::result::Result<core[cee74f9f94ff5162]::option::Option<rustc_interface[c2e2522df7b8878]::queries::Linker>, rustc_errors[4c24d11ef95ca307]::ErrorGuaranteed>>
  74:     0x7f3b13c50068 - rustc_span[85cbc7cdad3332d9]::with_source_map::<core[cee74f9f94ff5162]::result::Result<(), rustc_errors[4c24d11ef95ca307]::ErrorGuaranteed>, rustc_interface[c2e2522df7b8878]::interface::create_compiler_and_run<core[cee74f9f94ff5162]::result::Result<(), rustc_errors[4c24d11ef95ca307]::ErrorGuaranteed>, rustc_driver[a26527cb16da58d4]::run_compiler::{closure#1}>::{closure#1}>
  75:     0x7f3b13c69cd9 - rustc_interface[c2e2522df7b8878]::interface::create_compiler_and_run::<core[cee74f9f94ff5162]::result::Result<(), rustc_errors[4c24d11ef95ca307]::ErrorGuaranteed>, rustc_driver[a26527cb16da58d4]::run_compiler::{closure#1}>
  76:     0x7f3b13c4a35f - <scoped_tls[3af945965f2be09]::ScopedKey<rustc_span[85cbc7cdad3332d9]::SessionGlobals>>::set::<rustc_interface[c2e2522df7b8878]::interface::run_compiler<core[cee74f9f94ff5162]::result::Result<(), rustc_errors[4c24d11ef95ca307]::ErrorGuaranteed>, rustc_driver[a26527cb16da58d4]::run_compiler::{closure#1}>::{closure#0}, core[cee74f9f94ff5162]::result::Result<(), rustc_errors[4c24d11ef95ca307]::ErrorGuaranteed>>
  77:     0x7f3b13c53dc9 - std[ed124ce8448b1b72]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[c2e2522df7b8878]::util::run_in_thread_pool_with_globals<rustc_interface[c2e2522df7b8878]::interface::run_compiler<core[cee74f9f94ff5162]::result::Result<(), rustc_errors[4c24d11ef95ca307]::ErrorGuaranteed>, rustc_driver[a26527cb16da58d4]::run_compiler::{closure#1}>::{closure#0}, core[cee74f9f94ff5162]::result::Result<(), rustc_errors[4c24d11ef95ca307]::ErrorGuaranteed>>::{closure#0}, core[cee74f9f94ff5162]::result::Result<(), rustc_errors[4c24d11ef95ca307]::ErrorGuaranteed>>
  78:     0x7f3b13cbdf79 - <<std[ed124ce8448b1b72]::thread::Builder>::spawn_unchecked_<rustc_interface[c2e2522df7b8878]::util::run_in_thread_pool_with_globals<rustc_interface[c2e2522df7b8878]::interface::run_compiler<core[cee74f9f94ff5162]::result::Result<(), rustc_errors[4c24d11ef95ca307]::ErrorGuaranteed>, rustc_driver[a26527cb16da58d4]::run_compiler::{closure#1}>::{closure#0}, core[cee74f9f94ff5162]::result::Result<(), rustc_errors[4c24d11ef95ca307]::ErrorGuaranteed>>::{closure#0}, core[cee74f9f94ff5162]::result::Result<(), rustc_errors[4c24d11ef95ca307]::ErrorGuaranteed>>::{closure#1} as core[cee74f9f94ff5162]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  79:     0x7f3b132b3303 - std::sys::unix::thread::Thread::new::thread_start::hfccf706fa2afb703
  80:     0x7f3b0d7ff609 - start_thread
  81:     0x7f3b13112133 - clone
  82:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.64.0-nightly (4720ed2d1 2022-07-10) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
#0 [optimized_mir] optimizing MIR for `foo::{closure#0}`
#1 [layout_of] computing layout of `[static generator@/checkout/src/test/ui/async-await/issue-62658.rs:9:16: 23:2]`
#2 [layout_of] computing layout of `core::future::from_generator::GenFuture<[static generator@/checkout/src/test/ui/async-await/issue-62658.rs:9:16: 23:2]>`
#2 [layout_of] computing layout of `core::future::from_generator::GenFuture<[static generator@/checkout/src/test/ui/async-await/issue-62658.rs:9:16: 23:2]>`
#3 [layout_of] computing layout of `impl core::future::future::Future<Output = ()>`
#4 [mir_drops_elaborated_and_const_checked] elaborating drops for `main`
#5 [optimized_mir] optimizing MIR for `main`
#6 [collect_and_partition_mono_items] collect_and_partition_mono_items
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/async-await/issue-67252-unnamed-future.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issue-67252-unnamed-future.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-67252-unnamed-future" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-67252-unnamed-future/auxiliary"
stdout: none
stderr: none

---- [ui] src/test/ui/generator/auto-trait-regions.rs stdout ----
diff of stderr:


25    = note: consider using a `let` binding to create a longer lived value
26 
27 error: implementation of `Foo` is not general enough
-   --> $DIR/auto-trait-regions.rs:31:5
-    |
- LL |     assert_foo(gen);
-    |     ^^^^^^^^^^^^^^^ implementation of `Foo` is not general enough
-    |
-    = note: `&'0 OnlyFooIfStaticRef` must implement `Foo`, for any lifetime `'0`...
-    = note: ...but `Foo` is actually implemented for the type `&'static OnlyFooIfStaticRef`
- error: implementation of `Foo` is not general enough
37   --> $DIR/auto-trait-regions.rs:51:5
38    |
38    |
39 LL |     assert_foo(gen);

42    = note: `Foo` would have to be implemented for the type `A<'0, '1>`, for any two lifetimes `'0` and `'1`...
43    = note: ...but `Foo` is actually implemented for the type `A<'_, '2>`, for some specific lifetime `'2`
- error: aborting due to 4 previous errors
+ error: aborting due to 3 previous errors
46 
47 For more information about this error, try `rustc --explain E0716`.
---
To only update this specific test, also pass `--test-args generator/auto-trait-regions.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generator/auto-trait-regions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/auto-trait-regions" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/auto-trait-regions/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/generator/auto-trait-regions.rs:45:24
   |
   |
LL |         let a = A(&mut true, &mut true, No);
   |                        |
   |                        creates a temporary which is freed while still in use
...
...
LL |         assert_foo(a);
   |
   = note: consider using a `let` binding to create a longer lived value

error[E0716]: temporary value dropped while borrowed
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/generator/auto-trait-regions.rs:45:35
   |
LL |         let a = A(&mut true, &mut true, No);
   |                                   |
   |                                   creates a temporary which is freed while still in use
...
...
LL |         assert_foo(a);
   |
   = note: consider using a `let` binding to create a longer lived value

error: implementation of `Foo` is not general enough
error: implementation of `Foo` is not general enough
  --> /checkout/src/test/ui/generator/auto-trait-regions.rs:51:5
   |
LL |     assert_foo(gen);
   |     ^^^^^^^^^^^^^^^ implementation of `Foo` is not general enough
   |
   = note: `Foo` would have to be implemented for the type `A<'0, '1>`, for any two lifetimes `'0` and `'1`...
   = note: ...but `Foo` is actually implemented for the type `A<'_, '2>`, for some specific lifetime `'2`
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0716`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/generator/drop-control-flow.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generator/drop-control-flow.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/drop-control-flow" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zdrop-tracking" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/drop-control-flow/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: compiler/rustc_mir_transform/src/generator.rs:760:13: Broken MIR: generator contains type i32 in MIR, but typeck only knows about {[Ptr; 1], bool, ()} and []
   |
LL |     let _ = || {
   |             ^^


thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1331:9
stack backtrace:
   0:     0x7f31e33dc79c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h50e432a33816f17f
   1:     0x7f31e34428b8 - core::fmt::write::he1187f1c735a6d05
   2:     0x7f31e33ccd31 - std::io::Write::write_fmt::h39a703c43ca9e701
   3:     0x7f31e33df7ae - std::panicking::default_hook::{{closure}}::h3ba44f2c4a1ab003
   4:     0x7f31e33df476 - std::panicking::default_hook::hf88b5e434d766a47
   5:     0x7f31e3d989b4 - rustc_driver[a26527cb16da58d4]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f31e33dff62 - std::panicking::rust_panic_with_hook::hd6afc83c75fe8feb
   7:     0x7f31e439e873 - std[ed124ce8448b1b72]::panicking::begin_panic::<rustc_errors[4c24d11ef95ca307]::ExplicitBug>::{closure#0}
   8:     0x7f31e439a0c6 - std[ed124ce8448b1b72]::sys_common::backtrace::__rust_end_short_backtrace::<std[ed124ce8448b1b72]::panicking::begin_panic<rustc_errors[4c24d11ef95ca307]::ExplicitBug>::{closure#0}, !>
   9:     0x7f31e3bc74c6 - std[ed124ce8448b1b72]::panicking::begin_panic::<rustc_errors[4c24d11ef95ca307]::ExplicitBug>
  10:     0x7f31e42c9966 - std[ed124ce8448b1b72]::panic::panic_any::<rustc_errors[4c24d11ef95ca307]::ExplicitBug>
  11:     0x7f31e42c79d3 - <rustc_errors[4c24d11ef95ca307]::HandlerInner>::span_bug::<rustc_span[85cbc7cdad3332d9]::span_encoding::Span, &alloc[4cd35cc4a7f04eb5]::string::String>
  12:     0x7f31e42c7880 - <rustc_errors[4c24d11ef95ca307]::Handler>::span_bug::<rustc_span[85cbc7cdad3332d9]::span_encoding::Span, &alloc[4cd35cc4a7f04eb5]::string::String>
  13:     0x7f31e446d3c5 - rustc_middle[e06d1b0d4a982b9c]::util::bug::opt_span_bug_fmt::<rustc_span[85cbc7cdad3332d9]::span_encoding::Span>::{closure#0}
  14:     0x7f31e446d43b - rustc_middle[e06d1b0d4a982b9c]::ty::context::tls::with_opt::<rustc_middle[e06d1b0d4a982b9c]::util::bug::opt_span_bug_fmt<rustc_span[85cbc7cdad3332d9]::span_encoding::Span>::{closure#0}, ()>::{closure#0}
  15:     0x7f31e44678a6 - rustc_middle[e06d1b0d4a982b9c]::ty::context::tls::with_context_opt::<rustc_middle[e06d1b0d4a982b9c]::ty::context::tls::with_opt<rustc_middle[e06d1b0d4a982b9c]::util::bug::opt_span_bug_fmt<rustc_span[85cbc7cdad3332d9]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  16:     0x7f31e4467819 - rustc_middle[e06d1b0d4a982b9c]::util::bug::opt_span_bug_fmt::<rustc_span[85cbc7cdad3332d9]::span_encoding::Span>
  17:     0x7f31e3bcbff7 - rustc_middle[e06d1b0d4a982b9c]::util::bug::span_bug_fmt::<rustc_span[85cbc7cdad3332d9]::span_encoding::Span>
  18:     0x7f31e435181f - <rustc_mir_transform[4f795c61437e85be]::generator::StateTransform as rustc_middle[e06d1b0d4a982b9c]::mir::MirPass>::run_pass
  19:     0x7f31e42d7367 - rustc_mir_transform[4f795c61437e85be]::pass_manager::run_passes
  20:     0x7f31e43dcf7e - rustc_mir_transform[4f795c61437e85be]::optimized_mir
  21:     0x7f31e59734c0 - rustc_query_system[66850b7ae8af285]::query::plumbing::try_execute_query::<rustc_query_impl[d84d3f2479a1091a]::plumbing::QueryCtxt, rustc_query_system[66850b7ae8af285]::query::caches::DefaultCache<rustc_span[85cbc7cdad3332d9]::def_id::DefId, &rustc_middle[e06d1b0d4a982b9c]::mir::Body>>
  22:     0x7f31e5a20192 - rustc_query_system[66850b7ae8af285]::query::plumbing::get_query::<rustc_query_impl[d84d3f2479a1091a]::queries::optimized_mir, rustc_query_impl[d84d3f2479a1091a]::plumbing::QueryCtxt>
  23:     0x7f31e554e6e9 - <rustc_query_impl[d84d3f2479a1091a]::Queries as rustc_middle[e06d1b0d4a982b9c]::ty::query::QueryEngine>::optimized_mir
  24:     0x7f31e6828dfe - <rustc_middle[e06d1b0d4a982b9c]::ty::context::TyCtxt>::generator_layout
  25:     0x7f31e67bd139 - <rustc_middle[e06d1b0d4a982b9c]::ty::layout::LayoutCx<rustc_middle[e06d1b0d4a982b9c]::ty::context::TyCtxt>>::layout_of_uncached
  26:     0x7f31e67e766e - rustc_middle[e06d1b0d4a982b9c]::ty::context::tls::with_context::<rustc_middle[e06d1b0d4a982b9c]::ty::context::tls::with_related_context<rustc_middle[e06d1b0d4a982b9c]::ty::layout::layout_of::{closure#0}, core[cee74f9f94ff5162]::result::Result<rustc_target[25189b89ae46f52c]::abi::TyAndLayout<rustc_middle[e06d1b0d4a982b9c]::ty::Ty>, rustc_middle[e06d1b0d4a982b9c]::ty::layout::LayoutError>>::{closure#0}, core[cee74f9f94ff5162]::result::Result<rustc_target[25189b89ae46f52c]::abi::TyAndLayout<rustc_middle[e06d1b0d4a982b9c]::ty::Ty>, rustc_middle[e06d1b0d4a982b9c]::ty::layout::LayoutError>>::{closure#0}
  27:     0x7f31e67eb24f - rustc_middle[e06d1b0d4a982b9c]::ty::layout::layout_of
  28:     0x7f31e5a71d8d - rustc_query_system[66850b7ae8af285]::query::plumbing::get_query::<rustc_query_impl[d84d3f2479a1091a]::queries::layout_of, rustc_query_impl[d84d3f2479a1091a]::plumbing::QueryCtxt>
  29:     0x7f31e55793c3 - <rustc_query_impl[d84d3f2479a1091a]::Queries as rustc_middle[e06d1b0d4a982b9c]::ty::query::QueryEngine>::layout_of
  30:     0x7f31e426efbb - <rustc_mir_transform[4f795c61437e85be]::const_prop_lint::ConstProp as rustc_mir_transform[4f795c61437e85be]::pass_manager::MirLint>::run_lint
  31:     0x7f31e42d7367 - rustc_mir_transform[4f795c61437e85be]::pass_manager::run_passes
  32:     0x7f31e43dc7e0 - rustc_mir_transform[4f795c61437e85be]::run_post_borrowck_cleanup_passes
  33:     0x7f31e43dbeb7 - rustc_mir_transform[4f795c61437e85be]::mir_drops_elaborated_and_const_checked
  34:     0x7f31e593fb84 - rustc_query_system[66850b7ae8af285]::query::plumbing::try_execute_query::<rustc_query_impl[d84d3f2479a1091a]::plumbing::QueryCtxt, rustc_query_system[66850b7ae8af285]::query::caches::DefaultCache<rustc_middle[e06d1b0d4a982b9c]::ty::WithOptConstParam<rustc_span[85cbc7cdad3332d9]::def_id::LocalDefId>, &rustc_data_structures[176448742242e606]::steal::Steal<rustc_middle[e06d1b0d4a982b9c]::mir::Body>>>
  35:     0x7f31e5a6b723 - rustc_query_system[66850b7ae8af285]::query::plumbing::get_query::<rustc_query_impl[d84d3f2479a1091a]::queries::mir_drops_elaborated_and_const_checked, rustc_query_impl[d84d3f2479a1091a]::plumbing::QueryCtxt>
  36:     0x7f31e554cb8a - <rustc_query_impl[d84d3f2479a1091a]::Queries as rustc_middle[e06d1b0d4a982b9c]::ty::query::QueryEngine>::mir_drops_elaborated_and_const_checked
  37:     0x7f31e43dce48 - rustc_mir_transform[4f795c61437e85be]::optimized_mir
  38:     0x7f31e59734c0 - rustc_query_system[66850b7ae8af285]::query::plumbing::try_execute_query::<rustc_query_impl[d84d3f2479a1091a]::plumbing::QueryCtxt, rustc_query_system[66850b7ae8af285]::query::caches::DefaultCache<rustc_span[85cbc7cdad3332d9]::def_id::DefId, &rustc_middle[e06d1b0d4a982b9c]::mir::Body>>
  39:     0x7f31e5a20192 - rustc_query_system[66850b7ae8af285]::query::plumbing::get_query::<rustc_query_impl[d84d3f2479a1091a]::queries::optimized_mir, rustc_query_impl[d84d3f2479a1091a]::plumbing::QueryCtxt>
  40:     0x7f31e554e6e9 - <rustc_query_impl[d84d3f2479a1091a]::Queries as rustc_middle[e06d1b0d4a982b9c]::ty::query::QueryEngine>::optimized_mir
  41:     0x7f31e6837cfd - <rustc_middle[e06d1b0d4a982b9c]::ty::context::TyCtxt>::instance_mir
  42:     0x7f31e4206f95 - rustc_monomorphize[6a7659ee93aed75a]::collector::collect_neighbours
  43:     0x7f31e4201a6a - rustc_monomorphize[6a7659ee93aed75a]::collector::collect_items_rec
  44:     0x7f31e4201fb9 - rustc_monomorphize[6a7659ee93aed75a]::collector::collect_items_rec
  45:     0x7f31e4247981 - <rustc_session[f3aed6530d3c609e]::session::Session>::time::<(), rustc_monomorphize[6a7659ee93aed75a]::collector::collect_crate_mono_items::{closure#1}>
  46:     0x7f31e41fdce4 - rustc_monomorphize[6a7659ee93aed75a]::collector::collect_crate_mono_items
  47:     0x7f31e420e3ba - rustc_monomorphize[6a7659ee93aed75a]::partitioning::collect_and_partition_mono_items
  48:     0x7f31e5995652 - rustc_query_system[66850b7ae8af285]::query::plumbing::try_execute_query::<rustc_query_impl[d84d3f2479a1091a]::plumbing::QueryCtxt, rustc_query_system[66850b7ae8af285]::query::caches::DefaultCache<(), (&std[ed124ce8448b1b72]::collections::hash::set::HashSet<rustc_span[85cbc7cdad3332d9]::def_id::DefId, core[cee74f9f94ff5162]::hash::BuildHasherDefault<rustc_hash[a96122035317b32e]::FxHasher>>, &[rustc_middle[e06d1b0d4a982b9c]::mir::mono::CodegenUnit])>>
  49:     0x7f31e5a66b4a - rustc_query_system[66850b7ae8af285]::query::plumbing::get_query::<rustc_query_impl[d84d3f2479a1091a]::queries::collect_and_partition_mono_items, rustc_query_impl[d84d3f2479a1091a]::plumbing::QueryCtxt>
  50:     0x7f31e5592419 - <rustc_query_impl[d84d3f2479a1091a]::Queries as rustc_middle[e06d1b0d4a982b9c]::ty::query::QueryEngine>::collect_and_partition_mono_items
  51:     0x7f31e3ffab08 - rustc_codegen_ssa[c9066fae9df7bbdc]::base::codegen_crate::<rustc_codegen_llvm[f25c4d6ffc7893b1]::LlvmCodegenBackend>
  52:     0x7f31e40e5e59 - <rustc_codegen_llvm[f25c4d6ffc7893b1]::LlvmCodegenBackend as rustc_codegen_ssa[c9066fae9df7bbdc]::traits::backend::CodegenBackend>::codegen_crate
  53:     0x7f31e3f05401 - <rustc_session[f3aed6530d3c609e]::session::Session>::time::<alloc[4cd35cc4a7f04eb5]::boxed::Box<dyn core[cee74f9f94ff5162]::any::Any>, rustc_interface[c2e2522df7b8878]::passes::start_codegen::{closure#0}>
  54:     0x7f31e3ead266 - <rustc_interface[c2e2522df7b8878]::passes::QueryContext>::enter::<<rustc_interface[c2e2522df7b8878]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[cee74f9f94ff5162]::result::Result<alloc[4cd35cc4a7f04eb5]::boxed::Box<dyn core[cee74f9f94ff5162]::any::Any>, rustc_errors[4c24d11ef95ca307]::ErrorGuaranteed>>
  55:     0x7f31e3ec6ead - <rustc_interface[c2e2522df7b8878]::queries::Queries>::ongoing_codegen
  56:     0x7f31e3da016c - <rustc_interface[c2e2522df7b8878]::interface::Compiler>::enter::<rustc_driver[a26527cb16da58d4]::run_compiler::{closure#1}::{closure#2}, core[cee74f9f94ff5162]::result::Result<core[cee74f9f94ff5162]::option::Option<rustc_interface[c2e2522df7b8878]::queries::Linker>, rustc_errors[4c24d11ef95ca307]::ErrorGuaranteed>>
  57:     0x7f31e3d88068 - rustc_span[85cbc7cdad3332d9]::with_source_map::<core[cee74f9f94ff5162]::result::Result<(), rustc_errors[4c24d11ef95ca307]::ErrorGuaranteed>, rustc_interface[c2e2522df7b8878]::interface::create_compiler_and_run<core[cee74f9f94ff5162]::result::Result<(), rustc_errors[4c24d11ef95ca307]::ErrorGuaranteed>, rustc_driver[a26527cb16da58d4]::run_compiler::{closure#1}>::{closure#1}>
  58:     0x7f31e3da1cd9 - rustc_interface[c2e2522df7b8878]::interface::create_compiler_and_run::<core[cee74f9f94ff5162]::result::Result<(), rustc_errors[4c24d11ef95ca307]::ErrorGuaranteed>, rustc_driver[a26527cb16da58d4]::run_compiler::{closure#1}>
  59:     0x7f31e3d8235f - <scoped_tls[3af945965f2be09]::ScopedKey<rustc_span[85cbc7cdad3332d9]::SessionGlobals>>::set::<rustc_interface[c2e2522df7b8878]::interface::run_compiler<core[cee74f9f94ff5162]::result::Result<(), rustc_errors[4c24d11ef95ca307]::ErrorGuaranteed>, rustc_driver[a26527cb16da58d4]::run_compiler::{closure#1}>::{closure#0}, core[cee74f9f94ff5162]::result::Result<(), rustc_errors[4c24d11ef95ca307]::ErrorGuaranteed>>
  60:     0x7f31e3d8bdc9 - std[ed124ce8448b1b72]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[c2e2522df7b8878]::util::run_in_thread_pool_with_globals<rustc_interface[c2e2522df7b8878]::interface::run_compiler<core[cee74f9f94ff5162]::result::Result<(), rustc_errors[4c24d11ef95ca307]::ErrorGuaranteed>, rustc_driver[a26527cb16da58d4]::run_compiler::{closure#1}>::{closure#0}, core[cee74f9f94ff5162]::result::Result<(), rustc_errors[4c24d11ef95ca307]::ErrorGuaranteed>>::{closure#0}, core[cee74f9f94ff5162]::result::Result<(), rustc_errors[4c24d11ef95ca307]::ErrorGuaranteed>>
  61:     0x7f31e3df5f79 - <<std[ed124ce8448b1b72]::thread::Builder>::spawn_unchecked_<rustc_interface[c2e2522df7b8878]::util::run_in_thread_pool_with_globals<rustc_interface[c2e2522df7b8878]::interface::run_compiler<core[cee74f9f94ff5162]::result::Result<(), rustc_errors[4c24d11ef95ca307]::ErrorGuaranteed>, rustc_driver[a26527cb16da58d4]::run_compiler::{closure#1}>::{closure#0}, core[cee74f9f94ff5162]::result::Result<(), rustc_errors[4c24d11ef95ca307]::ErrorGuaranteed>>::{closure#0}, core[cee74f9f94ff5162]::result::Result<(), rustc_errors[4c24d11ef95ca307]::ErrorGuaranteed>>::{closure#1} as core[cee74f9f94ff5162]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  62:     0x7f31e33eb303 - std::sys::unix::thread::Thread::new::thread_start::hfccf706fa2afb703
  63:     0x7f31dd937609 - start_thread
  64:     0x7f31e324a133 - clone
  65:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.64.0-nightly (4720ed2d1 2022-07-10) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 -Z drop-tracking
query stack during panic:
#0 [optimized_mir] optimizing MIR for `loop_uninit::{closure#0}`
#1 [layout_of] computing layout of `[generator@/checkout/src/test/ui/generator/drop-control-flow.rs:87:13: 87:15]`
#1 [layout_of] computing layout of `[generator@/checkout/src/test/ui/generator/drop-control-flow.rs:87:13: 87:15]`
#2 [mir_drops_elaborated_and_const_checked] elaborating drops for `loop_uninit`
#3 [optimized_mir] optimizing MIR for `loop_uninit`
#4 [collect_and_partition_mono_items] collect_and_partition_mono_items
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/generator/issue-93161.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generator/issue-93161.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/issue-93161/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2021" "-Zdrop-tracking" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/issue-93161/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: compiler/rustc_mir_transform/src/generator.rs:760:13: Broken MIR: generator contains type [u8; 1] in MIR, but typeck only knows about {ResumeTy, impl Future<Output = Option<()>>, ()} and []
   |
LL |   async fn hello_world() {
   |  ________________________^
LL | |     let data = [0u8; 1];
LL | |     let data = [0u8; 1];
LL | |     let mut reader = &data[..];
LL | |
LL | |     let mut marker = [0u8; 1];
LL | |     read_exact(&mut reader, &mut marker).await.unwrap();
LL | | }

thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1331:9
stack backtrace:
   0:     0x7fe94a92579c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h50e432a33816f17f
   0:     0x7fe94a92579c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h50e432a33816f17f
   1:     0x7fe94a98b8b8 - core::fmt::write::he1187f1c735a6d05
   2:     0x7fe94a915d31 - std::io::Write::write_fmt::h39a703c43ca9e701
   3:     0x7fe94a9287ae - std::panicking::default_hook::{{closure}}::h3ba44f2c4a1ab003
   4:     0x7fe94a928476 - std::panicking::default_hook::hf88b5e434d766a47
   5:     0x7fe94b2e19b4 - rustc_driver[a26527cb16da58d4]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fe94a928f62 - std::panicking::rust_panic_with_hook::hd6afc83c75fe8feb
   7:     0x7fe94b8e7873 - std[ed124ce8448b1b72]::panicking::begin_panic::<rustc_errors[4c24d11ef95ca307]::ExplicitBug>::{closure#0}
   8:     0x7fe94b8e30c6 - std[ed124ce8448b1b72]::sys_common::backtrace::__rust_end_short_backtrace::<std[ed124ce8448b1b72]::panicking::begin_panic<rustc_errors[4c24d11ef95ca307]::ExplicitBug>::{closure#0}, !>
   9:     0x7fe94b1104c6 - std[ed124ce8448b1b72]::panicking::begin_panic::<rustc_errors[4c24d11ef95ca307]::ExplicitBug>
  10:     0x7fe94b812966 - std[ed124ce8448b1b72]::panic::panic_any::<rustc_errors[4c24d11ef95ca307]::ExplicitBug>
  11:     0x7fe94b8109d3 - <rustc_errors[4c24d11ef95ca307]::HandlerInner>::span_bug::<rustc_span[85cbc7cdad3332d9]::span_encoding::Span, &alloc[4cd35cc4a7f04eb5]::string::String>
  12:     0x7fe94b810880 - <rustc_errors[4c24d11ef95ca307]::Handler>::span_bug::<rustc_span[85cbc7cdad3332d9]::span_encoding::Span, &alloc[4cd35cc4a7f04eb5]::string::String>
  13:     0x7fe94b9b63c5 - rustc_middle[e06d1b0d4a982b9c]::util::bug::opt_span_bug_fmt::<rustc_span[85cbc7cdad3332d9]::span_encoding::Span>::{closure#0}
  14:     0x7fe94b9b643b - rustc_middle[e06d1b0d4a982b9c]::ty::context::tls::with_opt::<rustc_middle[e06d1b0d4a982b9c]::util::bug::opt_span_bug_fmt<rustc_span[85cbc7cdad3332d9]::span_encoding::Span>::{closure#0}, ()>::{closure#0}
  15:     0x7fe94b9b08a6 - rustc_middle[e06d1b0d4a982b9c]::ty::context::tls::with_context_opt::<rustc_middle[e06d1b0d4a982b9c]::ty::context::tls::with_opt<rustc_middle[e06d1b0d4a982b9c]::util::bug::opt_span_bug_fmt<rustc_span[85cbc7cdad3332d9]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  16:     0x7fe94b9b0819 - rustc_middle[e06d1b0d4a982b9c]::util::bug::opt_span_bug_fmt::<rustc_span[85cbc7cdad3332d9]::span_encoding::Span>
  17:     0x7fe94b114ff7 - rustc_middle[e06d1b0d4a982b9c]::util::bug::span_bug_fmt::<rustc_span[85cbc7cdad3332d9]::span_encoding::Span>
  18:     0x7fe94b89a81f - <rustc_mir_transform[4f795c61437e85be]::generator::StateTransform as rustc_middle[e06d1b0d4a982b9c]::mir::MirPass>::run_pass
  19:     0x7fe94b820367 - rustc_mir_transform[4f795c61437e85be]::pass_manager::run_passes
  20:     0x7fe94b925f7e - rustc_mir_transform[4f795c61437e85be]::optimized_mir
  21:     0x7fe94cebc4c0 - rustc_query_system[66850b7ae8af285]::query::plumbing::try_execute_query::<rustc_query_impl[d84d3f2479a1091a]::plumbing::QueryCtxt, rustc_query_system[66850b7ae8af285]::query::caches::DefaultCache<rustc_span[85cbc7cdad3332d9]::def_id::DefId, &rustc_middle[e06d1b0d4a982b9c]::mir::Body>>
  22:     0x7fe94cf69192 - rustc_query_system[66850b7ae8af285]::query::plumbing::get_query::<rustc_query_impl[d84d3f2479a1091a]::queries::optimized_mir, rustc_query_impl[d84d3f2479a1091a]::plumbing::QueryCtxt>
  23:     0x7fe94ca976e9 - <rustc_query_impl[d84d3f2479a1091a]::Queries as rustc_middle[e06d1b0d4a982b9c]::ty::query::QueryEngine>::optimized_mir
  24:     0x7fe94dd71dfe - <rustc_middle[e06d1b0d4a982b9c]::ty::context::TyCtxt>::generator_layout
  25:     0x7fe94dd06139 - <rustc_middle[e06d1b0d4a982b9c]::ty::layout::LayoutCx<rustc_middle[e06d1b0d4a982b9c]::ty::context::TyCtxt>>::layout_of_uncached
  26:     0x7fe94dd3066e - rustc_middle[e06d1b0d4a982b9c]::ty::context::tls::with_context::<rustc_middle[e06d1b0d4a982b9c]::ty::context::tls::with_related_context<rustc_middle[e06d1b0d4a982b9c]::ty::layout::layout_of::{closure#0}, core[cee74f9f94ff5162]::result::Result<rustc_target[25189b89ae46f52c]::abi::TyAndLayout<rustc_middle[e06d1b0d4a982b9c]::ty::Ty>, rustc_middle[e06d1b0d4a982b9c]::ty::layout::LayoutError>>::{closure#0}, core[cee74f9f94ff5162]::result::Result<rustc_target[25189b89ae46f52c]::abi::TyAndLayout<rustc_middle[e06d1b0d4a982b9c]::ty::Ty>, rustc_middle[e06d1b0d4a982b9c]::ty::layout::LayoutError>>::{closure#0}
  27:     0x7fe94dd3424f - rustc_middle[e06d1b0d4a982b9c]::ty::layout::layout_of
  28:     0x7fe94cfbad8d - rustc_query_system[66850b7ae8af285]::query::plumbing::get_query::<rustc_query_impl[d84d3f2479a1091a]::queries::layout_of, rustc_query_impl[d84d3f2479a1091a]::plumbing::QueryCtxt>
  29:     0x7fe94cac23c3 - <rustc_query_impl[d84d3f2479a1091a]::Queries as rustc_middle[e06d1b0d4a982b9c]::ty::query::QueryEngine>::layout_of
  30:     0x7fe94dcb2e3f - <rustc_middle[e06d1b0d4a982b9c]::ty::layout::LayoutCx<rustc_middle[e06d1b0d4a982b9c]::ty::context::TyCtxt> as rustc_middle[e06d1b0d4a982b9c]::ty::layout::LayoutOf>::spanned_layout_of
  31:     0x7fe94dca7f4f - <core[cee74f9f94ff5162]::iter::adapters::map::Map<core[cee74f9f94ff5162]::slice::iter::Iter<rustc_middle[e06d1b0d4a982b9c]::ty::FieldDef>, <rustc_middle[e06d1b0d4a982b9c]::ty::layout::LayoutCx<rustc_middle[e06d1b0d4a982b9c]::ty::context::TyCtxt>>::layout_of_uncached::{closure#5}::{closure#0}> as core[cee74f9f94ff5162]::iter::traits::iterator::Iterator>::try_fold::<(), <core[cee74f9f94ff5162]::iter::adapters::GenericShunt<core[cee74f9f94ff5162]::iter::adapters::map::Map<core[cee74f9f94ff5162]::slice::iter::Iter<rustc_middle[e06d1b0d4a982b9c]::ty::FieldDef>, <rustc_middle[e06d1b0d4a982b9c]::ty::layout::LayoutCx<rustc_middle[e06d1b0d4a982b9c]::ty::context::TyCtxt>>::layout_of_uncached::{closure#5}::{closure#0}>, core[cee74f9f94ff5162]::result::Result<core[cee74f9f94ff5162]::convert::Infallible, rustc_middle[e06d1b0d4a982b9c]::ty::layout::LayoutError>> as core[cee74f9f94ff5162]::iter::traits::iterator::Iterator>::try_fold<(), core[cee74f9f94ff5162]::iter::traits::iterator::Iterator::try_for_each::call<rustc_target[25189b89ae46f52c]::abi::TyAndLayout<rustc_middle[e06d1b0d4a982b9c]::ty::Ty>, core[cee74f9f94ff5162]::ops::control_flow::ControlFlow<rustc_target[25189b89ae46f52c]::abi::TyAndLayout<rustc_middle[e06d1b0d4a982b9c]::ty::Ty>>, core[cee74f9f94ff5162]::ops::control_flow::ControlFlow<rustc_target[25189b89ae46f52c]::abi::TyAndLayout<rustc_middle[e06d1b0d4a982b9c]::ty::Ty>>::Break>::{closure#0}, core[cee74f9f94ff5162]::ops::control_flow::ControlFlow<rustc_target[25189b89ae46f52c]::abi::TyAndLayout<rustc_middle[e06d1b0d4a982b9c]::ty::Ty>>>::{closure#0}, core[cee74f9f94ff5162]::ops::control_flow::ControlFlow<core[cee74f9f94ff5162]::ops::control_flow::ControlFlow<rustc_target[25189b89ae46f52c]::abi::TyAndLayout<rustc_middle[e06d1b0d4a982b9c]::ty::Ty>>>>
  32:     0x7fe94dc8ccba - <core[cee74f9f94ff5162]::iter::adapters::GenericShunt<core[cee74f9f94ff5162]::iter::adapters::map::Map<core[cee74f9f94ff5162]::slice::iter::Iter<rustc_middle[e06d1b0d4a982b9c]::ty::FieldDef>, <rustc_middle[e06d1b0d4a982b9c]::ty::layout::LayoutCx<rustc_middle[e06d1b0d4a982b9c]::ty::context::TyCtxt>>::layout_of_uncached::{closure#5}::{closure#0}>, core[cee74f9f94ff5162]::result::Result<core[cee74f9f94ff5162]::convert::Infallible, rustc_middle[e06d1b0d4a982b9c]::ty::layout::LayoutError>> as core[cee74f9f94ff5162]::iter::traits::iterator::Iterator>::next
  33:     0x7fe94dccdf8b - <alloc[4cd35cc4a7f04eb5]::vec::Vec<rustc_target[25189b89ae46f52c]::abi::TyAndLayout<rustc_middle[e06d1b0d4a982b9c]::ty::Ty>> as alloc[4cd35cc4a7f04eb5]::vec::spec_from_iter::SpecFromIter<rustc_target[25189b89ae46f52c]::abi::TyAndLayout<rustc_middle[e06d1b0d4a982b9c]::ty::Ty>, core[cee74f9f94ff5162]::iter::adapters::GenericShunt<core[cee74f9f94ff5162]::iter::adapters::map::Map<core[cee74f9f94ff5162]::slice::iter::Iter<rustc_middle[e06d1b0d4a982b9c]::ty::FieldDef>, <rustc_middle[e06d1b0d4a982b9c]::ty::layout::LayoutCx<rustc_middle[e06d1b0d4a982b9c]::ty::context::TyCtxt>>::layout_of_uncached::{closure#5}::{closure#0}>, core[cee74f9f94ff5162]::result::Result<core[cee74f9f94ff5162]::convert::Infallible, rustc_middle[e06d1b0d4a982b9c]::ty::layout::LayoutError>>>>::from_iter
  34:     0x7fe94dc8948b - core[cee74f9f94ff5162]::iter::adapters::try_process::<core[cee74f9f94ff5162]::iter::adapters::map::Map<core[cee74f9f94ff5162]::slice::iter::Iter<rustc_middle[e06d1b0d4a982b9c]::ty::FieldDef>, <rustc_middle[e06d1b0d4a982b9c]::ty::layout::LayoutCx<rustc_middle[e06d1b0d4a982b9c]::ty::context::TyCtxt>>::layout_of_uncached::{closure#5}::{closure#0}>, rustc_target[25189b89ae46f52c]::abi::TyAndLayout<rustc_middle[e06d1b0d4a982b9c]::ty::Ty>, core[cee74f9f94ff5162]::result::Result<core[cee74f9f94ff5162]::convert::Infallible, rustc_middle[e06d1b0d4a982b9c]::ty::layout::LayoutError>, <core[cee74f9f94ff5162]::result::Result<alloc[4cd35cc4a7f04eb5]::vec::Vec<rustc_target[25189b89ae46f52c]::abi::TyAndLayout<rustc_middle[e06d1b0d4a982b9c]::ty::Ty>>, rustc_middle[e06d1b0d4a982b9c]::ty::layout::LayoutError> as core[cee74f9f94ff5162]::iter::traits::collect::FromIterator<core[cee74f9f94ff5162]::result::Result<rustc_target[25189b89ae46f52c]::abi::TyAndLayout<rustc_middle[e06d1b0d4a982b9c]::ty::Ty>, rustc_middle[e06d1b0d4a982b9c]::ty::layout::LayoutError>>>::from_iter<core[cee74f9f94ff5162]::iter::adapters::map::Map<core[cee74f9f94ff5162]::slice::iter::Iter<rustc_middle[e06d1b0d4a982b9c]::ty::FieldDef>, <rustc_middle[e06d1b0d4a982b9c]::ty::layout::LayoutCx<rustc_middle[e06d1b0d4a982b9c]::ty::context::TyCtxt>>::layout_of_uncached::{closure#5}::{closure#0}>>::{closure#0}, alloc[4cd35cc4a7f04eb5]::vec::Vec<rustc_target[25189b89ae46f52c]::abi::TyAndLayout<rustc_middle[e06d1b0d4a982b9c]::ty::Ty>>>
  35:     0x7fe94dca7e2a - <core[cee74f9f94ff5162]::iter::adapters::map::Map<core[cee74f9f94ff5162]::slice::iter::Iter<rustc_middle[e06d1b0d4a982b9c]::ty::VariantDef>, <rustc_middle[e06d1b0d4a982b9c]::ty::layout::LayoutCx<rustc_middle[e06d1b0d4a982b9c]::ty::context::TyCtxt>>::layout_of_uncached::{closure#5}> as core[cee74f9f94ff5162]::iter::traits::iterator::Iterator>::try_fold::<(), <core[cee74f9f94ff5162]::iter::adapters::GenericShunt<core[cee74f9f94ff5162]::iter::adapters::map::Map<core[cee74f9f94ff5162]::slice::iter::Iter<rustc_middle[e06d1b0d4a982b9c]::ty::VariantDef>, <rustc_middle[e06d1b0d4a982b9c]::ty::layout::LayoutCx<rustc_middle[e06d1b0d4a982b9c]::ty::context::TyCtxt>>::layout_of_uncached::{closure#5}>, core[cee74f9f94ff5162]::result::Result<core[cee74f9f94ff5162]::convert::Infallible, rustc_middle[e06d1b0d4a982b9c]::ty::layout::LayoutError>> as core[cee74f9f94ff5162]::iter::traits::iterator::Iterator>::try_fold<(), core[cee74f9f94ff5162]::iter::traits::iterator::Iterator::try_for_each::call<alloc[4cd35cc4a7f04eb5]::vec::Vec<rustc_target[25189b89ae46f52c]::abi::TyAndLayout<rustc_middle[e06d1b0d4a982b9c]::ty::Ty>>, core[cee74f9f94ff5162]::ops::control_flow::ControlFlow<alloc[4cd35cc4a7f04eb5]::vec::Vec<rustc_target[25189b89ae46f52c]::abi::TyAndLayout<rustc_middle[e06d1b0d4a982b9c]::ty::Ty>>>, core[cee74f9f94ff5162]::ops::control_flow::ControlFlow<alloc[4cd35cc4a7f04eb5]::vec::Vec<rustc_target[25189b89ae46f52c]::abi::TyAndLayout<rustc_middle[e06d1b0d4a982b9c]::ty::Ty>>>::Break>::{closure#0}, core[cee74f9f94ff5162]::ops::control_flow::ControlFlow<alloc[4cd35cc4a7f04eb5]::vec::Vec<rustc_target[25189b89ae46f52c]::abi::TyAndLayout<rustc_middle[e06d1b0d4a982b9c]::ty::Ty>>>>::{closure#0}, core[cee74f9f94ff5162]::ops::control_flow::ControlFlow<core[cee74f9f94ff5162]::ops::control_flow::ControlFlow<alloc[4cd35cc4a7f04eb5]::vec::Vec<rustc_target[25189b89ae46f52c]::abi::TyAndLayout<rustc_middle[e06d1b0d4a982b9c]::ty::Ty>>>>>
  36:     0x7fe94dc8cc3b - <core[cee74f9f94ff5162]::iter::adapters::GenericShunt<core[cee74f9f94ff5162]::iter::adapters::map::Map<core[cee74f9f94ff5162]::slice::iter::Iter<rustc_middle[e06d1b0d4a982b9c]::ty::VariantDef>, <rustc_middle[e06d1b0d4a982b9c]::ty::layout::LayoutCx<rustc_middle[e06d1b0d4a982b9c]::ty::context::TyCtxt>>::layout_of_uncached::{closure#5}>, core[cee74f9f94ff5162]::result::Result<core[cee74f9f94ff5162]::convert::Infallible, rustc_middle[e06d1b0d4a982b9c]::ty::layout::LayoutError>> as core[cee74f9f94ff5162]::iter::traits::iterator::Iterator>::next
  37:     0x7fe94dccd149 - <alloc[4cd35cc4a7f04eb5]::vec::Vec<alloc[4cd35cc4a7f04eb5]::vec::Vec<rustc_target[25189b89ae46f52c]::abi::TyAndLayout<rustc_middle[e06d1b0d4a982b9c]::ty::Ty>>> as alloc[4cd35cc4a7f04eb5]::vec::spec_from_iter::SpecFromIter<alloc[4cd35cc4a7f04eb5]::vec::Vec<rustc_target[25189b89ae46f52c]::abi::TyAndLayout<rustc_middle[e06d1b0d4a982b9c]::ty::Ty>>, core[cee74f9f94ff5162]::iter::adapters::GenericShunt<core[cee74f9f94ff5162]::iter::adapters::map::Map<core[cee74f9f94ff5162]::slice::iter::Iter<rustc_middle[e06d1b0d4a982b9c]::ty::VariantDef>, <rustc_middle[e06d1b0d4a982b9c]::ty::layout::LayoutCx<rustc_middle[e06d1b0d4a982b9c]::ty::context::TyCtxt>>::layout_of_uncached::{closure#5}>, core[cee74f9f94ff5162]::result::Result<core[cee74f9f94ff5162]::convert::Infallible, rustc_middle[e06d1b0d4a982b9c]::ty::layout::LayoutError>>>>::from_iter
  38:     0x7fe94dc8939b - core[cee74f9f94ff5162]::iter::adapters::try_process::<core[cee74f9f94ff5162]::iter::adapters::map::Map<core[cee74f9f94ff5162]::slice::iter::Iter<rustc_middle[e06d1b0d4a982b9c]::ty::VariantDef>, <rustc_middle[e06d1b0d4a982b9c]::ty::layout::LayoutCx<rustc_middle[e06d1b0d4a982b9c]::ty::context::TyCtxt>>::layout_of_uncached::{closure#5}>, alloc[4cd35cc4a7f04eb5]::vec::Vec<rustc_target[25189b89ae46f52c]::abi::TyAndLayout<rustc_middle[e06d1b0d4a982b9c]::ty::Ty>>, core[cee74f9f94ff5162]::result::Result<core[cee74f9f94ff5162]::convert::Infallible, rustc_middle[e06d1b0d4a982b9c]::ty::layout::LayoutError>, <core[cee74f9f94ff5162]::result::Result<rustc_index[c4648b84434bbbde]::vec::IndexVec<rustc_target[25189b89ae46f52c]::abi::VariantIdx, alloc[4cd35cc4a7f04eb5]::vec::Vec<rustc_target[25189b89ae46f52c]::abi::TyAndLayout<rustc_middle[e06d1b0d4a982b9c]::ty::Ty>>>, rustc_middle[e06d1b0d4a982b9c]::ty::layout::LayoutError> as core[cee74f9f94ff5162]::iter::traits::collect::FromIterator<core[cee74f9f94ff5162]::result::Result<alloc[4cd35cc4a7f04eb5]::vec::Vec<rustc_target[25189b89ae46f52c]::abi::TyAndLayout<rustc_middle[e06d1b0d4a982b9c]::ty::Ty>>, rustc_middle[e06d1b0d4a982b9c]::ty::layout::LayoutError>>>::from_iter<core[cee74f9f94ff5162]::iter::adapters::map::Map<core[cee74f9f94ff5162]::slice::iter::Iter<rustc_middle[e06d1b0d4a982b9c]::ty::VariantDef>, <rustc_middle[e06d1b0d4a982b9c]::ty::layout::LayoutCx<rustc_middle[e06d1b0d4a982b9c]::ty::context::TyCtxt>>::layout_of_uncached::{closure#5}>>::{closure#0}, rustc_index[c4648b84434bbbde]::vec::IndexVec<rustc_target[25189b89ae46f52c]::abi::VariantIdx, alloc[4cd35cc4a7f04eb5]::vec::Vec<rustc_target[25189b89ae46f52c]::abi::TyAndLayout<rustc_middle[e06d1b0d4a982b9c]::ty::Ty>>>>
  39:     0x7fe94dd06d79 - <rustc_middle[e06d1b0d4a982b9c]::ty::layout::LayoutCx<rustc_middle[e06d1b0d4a982b9c]::ty::context::TyCtxt>>::layout_of_uncached
  40:     0x7fe94dd3066e - rustc_middle[e06d1b0d4a982b9c]::ty::context::tls::with_context::<rustc_middle[e06d1b0d4a982b9c]::ty::context::tls::with_related_context<rustc_middle[e06d1b0d4a982b9c]::ty::layout::layout_of::{closure#0}, core[cee74f9f94ff5162]::result::Result<rustc_target[25189b89ae46f52c]::abi::TyAndLayout<rustc_middle[e06d1b0d4a982b9c]::ty::Ty>, rustc_middle[e06d1b0d4a982b9c]::ty::layout::LayoutError>>::{closure#0}, core[cee74f9f94ff5162]::result::Result<rustc_target[25189b89ae46f52c]::abi::TyAndLayout<rustc_middle[e06d1b0d4a982b9c]::ty::Ty>, rustc_middle[e06d1b0d4a982b9c]::ty::layout::LayoutError>>::{closure#0}
  41:     0x7fe94dd3424f - rustc_middle[e06d1b0d4a982b9c]::ty::layout::layout_of
  42:     0x7fe94cfbad8d - rustc_query_system[66850b7ae8af285]::query::plumbing::get_query::<rustc_query_impl[d84d3f2479a1091a]::queries::layout_of, rustc_query_impl[d84d3f2479a1091a]::plumbing::QueryCtxt>
  43:     0x7fe94cac23c3 - <rustc_query_impl[d84d3f2479a1091a]::Queries as rustc_middle[e06d1b0d4a982b9c]::ty::query::QueryEngine>::layout_of
  44:     0x7fe94dd30709 - rustc_middle[e06d1b0d4a982b9c]::ty::context::tls::with_context::<rustc_middle[e06d1b0d4a982b9c]::ty::context::tls::with_related_context<rustc_middle[e06d1b0d4a982b9c]::ty::layout::layout_of::{closure#0}, core[cee74f9f94ff5162]::result::Result<rustc_target[25189b89ae46f52c]::abi::TyAndLayout<rustc_middle[e06d1b0d4a982b9c]::ty::Ty>, rustc_middle[e06d1b0d4a982b9c]::ty::layout::LayoutError>>::{closure#0}, core[cee74f9f94ff5162]::result::Result<rustc_target[25189b89ae46f52c]::abi::TyAndLayout<rustc_middle[e06d1b0d4a982b9c]::ty::Ty>, rustc_middle[e06d1b0d4a982b9c]::ty::layout::LayoutError>>::{closure#0}
  45:     0x7fe94dd3424f - rustc_middle[e06d1b0d4a982b9c]::ty::layout::layout_of
  46:     0x7fe94cfbad8d - rustc_query_system[66850b7ae8af285]::query::plumbing::get_query::<rustc_query_impl[d84d3f2479a1091a]::queries::layout_of, rustc_query_impl[d84d3f2479a1091a]::plumbing::QueryCtxt>
  47:     0x7fe94cac23c3 - <rustc_query_impl[d84d3f2479a1091a]::Queries as rustc_middle[e06d1b0d4a982b9c]::ty::query::QueryEngine>::layout_of
  48:     0x7fe94b7b7fbb - <rustc_mir_transform[4f795c61437e85be]::const_prop_lint::ConstProp as rustc_mir_transform[4f795c61437e85be]::pass_manager::MirLint>::run_lint
  49:     0x7fe94b820367 - rustc_mir_transform[4f795c61437e85be]::pass_manager::run_passes
  50:     0x7fe94b9257e0 - rustc_mir_transform[4f795c61437e85be]::run_post_borrowck_cleanup_passes
  51:     0x7fe94b924eb7 - rustc_mir_transform[4f795c61437e85be]::mir_drops_elaborated_and_const_checked
  52:     0x7fe94ce88b84 - rustc_query_system[66850b7ae8af285]::query::plumbing::try_execute_query::<rustc_query_impl[d84d3f2479a1091a]::plumbing::QueryCtxt, rustc_query_system[66850b7ae8af285]::query::caches::DefaultCache<rustc_middle[e06d1b0d4a982b9c]::ty::WithOptConstParam<rustc_span[85cbc7cdad3332d9]::def_id::LocalDefId>, &rustc_data_structures[176448742242e606]::steal::Steal<rustc_middle[e06d1b0d4a982b9c]::mir::Body>>>
  53:     0x7fe94cfb4723 - rustc_query_system[66850b7ae8af285]::query::plumbing::get_query::<rustc_query_impl[d84d3f2479a1091a]::queries::mir_drops_elaborated_and_const_checked, rustc_query_impl[d84d3f2479a1091a]::plumbing::QueryCtxt>
  54:     0x7fe94ca95b8a - <rustc_query_impl[d84d3f2479a1091a]::Queries as rustc_middle[e06d1b0d4a982b9c]::ty::query::QueryEngine>::mir_drops_elaborated_and_const_checked
  55:     0x7fe94b925e48 - rustc_mir_transform[4f795c61437e85be]::optimized_mir
  56:     0x7fe94cebc4c0 - rustc_query_system[66850b7ae8af285]::query::plumbing::try_execute_query::<rustc_query_impl[d84d3f2479a1091a]::plumbing::QueryCtxt, rustc_query_system[66850b7ae8af285]::query::caches::DefaultCache<rustc_span[85cbc7cdad3332d9]::def_id::DefId, &rustc_middle[e06d1b0d4a982b9c]::mir::Body>>
  57:     0x7fe94cf69192 - rustc_query_system[66850b7ae8af285]::query::plumbing::get_query::<rustc_query_impl[d84d3f2479a1091a]::queries::optimized_mir, rustc_query_impl[d84d3f2479a1091a]::plumbing::QueryCtxt>
  58:     0x7fe94ca976e9 - <rustc_query_impl[d84d3f2479a1091a]::Queries as rustc_middle[e06d1b0d4a982b9c]::ty::query::QueryEngine>::optimized_mir
  59:     0x7fe94dd80cfd - <rustc_middle[e06d1b0d4a982b9c]::ty::context::TyCtxt>::instance_mir
  60:     0x7fe94b74ff95 - rustc_monomorphize[6a7659ee93aed75a]::collector::collect_neighbours
  61:     0x7fe94b74aa6a - rustc_monomorphize[6a7659ee93aed75a]::collector::collect_items_rec
  62:     0x7fe94b790981 - <rustc_session[f3aed6530d3c609e]::session::Session>::time::<(), rustc_monomorphize[6a7659ee93aed75a]::collector::collect_crate_mono_items::{closure#1}>
  63:     0x7fe94b746ce4 - rustc_monomorphize[6a7659ee93aed75a]::collector::collect_crate_mono_items
  64:     0x7fe94b7573ba - rustc_monomorphize[6a7659ee93aed75a]::partitioning::collect_and_partition_mono_items
  65:     0x7fe94cede652 - rustc_query_system[66850b7ae8af285]::query::plumbing::try_execute_query::<rustc_query_impl[d84d3f2479a1091a]::plumbing::QueryCtxt, rustc_query_system[66850b7ae8af285]::query::caches::DefaultCache<(), (&std[ed124ce8448b1b72]::collections::hash::set::HashSet<rustc_span[85cbc7cdad3332d9]::def_id::DefId, core[cee74f9f94ff5162]::hash::BuildHasherDefault<rustc_hash[a96122035317b32e]::FxHasher>>, &[rustc_middle[e06d1b0d4a982b9c]::mir::mono::CodegenUnit])>>
  66:     0x7fe94cfafb4a - rustc_query_system[66850b7ae8af285]::query::plumbing::get_query::<rustc_query_impl[d84d3f2479a1091a]::queries::collect_and_partition_mono_items, rustc_query_impl[d84d3f2479a1091a]::plumbing::QueryCtxt>
  67:     0x7fe94cadb419 - <rustc_query_impl[d84d3f2479a1091a]::Queries as rustc_middle[e06d1b0d4a982b9c]::ty::query::QueryEngine>::collect_and_partition_mono_items
  68:     0x7fe94b543b08 - rustc_codegen_ssa[c9066fae9df7bbdc]::base::codegen_crate::<rustc_codegen_llvm[f25c4d6ffc7893b1]::LlvmCodegenBackend>
  69:     0x7fe94b62ee59 - <rustc_codegen_llvm[f25c4d6ffc7893b1]::LlvmCodegenBackend as rustc_codegen_ssa[c9066fae9df7bbdc]::traits::backend::CodegenBackend>::codegen_crate
  70:     0x7fe94b44e401 - <rustc_session[f3aed6530d3c609e]::session::Session>::time::<alloc[4cd35cc4a7f04eb5]::boxed::Box<dyn core[cee74f9f94ff5162]::any::Any>, rustc_interface[c2e2522df7b8878]::passes::start_codegen::{closure#0}>
  71:     0x7fe94b3f6266 - <rustc_interface[c2e2522df7b8878]::passes::QueryContext>::enter::<<rustc_interface[c2e2522df7b8878]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[cee74f9f94ff5162]::result::Result<alloc[4cd35cc4a7f04eb5]::boxed::Box<dyn core[cee74f9f94ff5162]::any::Any>, rustc_errors[4c24d11ef95ca307]::ErrorGuaranteed>>
  72:     0x7fe94b40fead - <rustc_interface[c2e2522df7b8878]::queries::Queries>::ongoing_codegen
  73:     0x7fe94b2e916c - <rustc_interface[c2e2522df7b8878]::interface::Compiler>::enter::<rustc_driver[a26527cb16da58d4]::run_compiler::{closure#1}::{closure#2}, core[cee74f9f94ff5162]::result::Result<core[cee74f9f94ff5162]::option::Option<rustc_interface[c2e2522df7b8878]::queries::Linker>, rustc_errors[4c24d11ef95ca307]::ErrorGuaranteed>>
  74:     0x7fe94b2d1068 - rustc_span[85cbc7cdad3332d9]::with_source_map::<core[cee74f9f94ff5162]::result::Result<(), rustc_errors[4c24d11ef95ca307]::ErrorGuaranteed>, rustc_interface[c2e2522df7b8878]::interface::create_compiler_and_run<core[cee74f9f94ff5162]::result::Result<(), rustc_errors[4c24d11ef95ca307]::ErrorGuaranteed>, rustc_driver[a26527cb16da58d4]::run_compiler::{closure#1}>::{closure#1}>
  75:     0x7fe94b2eacd9 - rustc_interface[c2e2522df7b8878]::interface::create_compiler_and_run::<core[cee74f9f94ff5162]::result::Result<(), rustc_errors[4c24d11ef95ca307]::ErrorGuaranteed>, rustc_driver[a26527cb16da58d4]::run_compiler::{closure#1}>
  76:     0x7fe94b2cb35f - <scoped_tls[3af945965f2be09]::ScopedKey<rustc_span[85cbc7cdad3332d9]::SessionGlobals>>::set::<rustc_interface[c2e2522df7b8878]::interface::run_compiler<core[cee74f9f94ff5162]::result::Result<(), rustc_errors[4c24d11ef95ca307]::ErrorGuaranteed>, rustc_driver[a26527cb16da58d4]::run_compiler::{closure#1}>::{closure#0}, core[cee74f9f94ff5162]::result::Result<(), rustc_errors[4c24d11ef95ca307]::ErrorGuaranteed>>
  77:     0x7fe94b2d4dc9 - std[ed124ce8448b1b72]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[c2e2522df7b8878]::util::run_in_thread_pool_with_globals<rustc_interface[c2e2522df7b8878]::interface::run_compiler<core[cee74f9f94ff5162]::result::Result<(), rustc_errors[4c24d11ef95ca307]::ErrorGuaranteed>, rustc_driver[a26527cb16da58d4]::run_compiler::{closure#1}>::{closure#0}, core[cee74f9f94ff5162]::result::Result<(), rustc_errors[4c24d11ef95ca307]::ErrorGuaranteed>>::{closure#0}, core[cee74f9f94ff5162]::result::Result<(), rustc_errors[4c24d11ef95ca307]::ErrorGuaranteed>>
  78:     0x7fe94b33ef79 - <<std[ed124ce8448b1b72]::thread::Builder>::spawn_unchecked_<rustc_interface[c2e2522df7b8878]::util::run_in_thread_pool_with_globals<rustc_interface[c2e2522df7b8878]::interface::run_compiler<core[cee74f9f94ff5162]::result::Result<(), rustc_errors[4c24d11ef95ca307]::ErrorGuaranteed>, rustc_driver[a26527cb16da58d4]::run_compiler::{closure#1}>::{closure#0}, core[cee74f9f94ff5162]::result::Result<(), rustc_errors[4c24d11ef95ca307]::ErrorGuaranteed>>::{closure#0}, core[cee74f9f94ff5162]::result::Result<(), rustc_errors[4c24d11ef95ca307]::ErrorGuaranteed>>::{closure#1} as core[cee74f9f94ff5162]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  79:     0x7fe94a934303 - std::sys::unix::thread::Thread::new::thread_start::hfccf706fa2afb703
  80:     0x7fe944e80609 - start_thread
  81:     0x7fe94a793133 - clone
  82:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.64.0-nightly (4720ed2d1 2022-07-10) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 -Z drop-tracking
query stack during panic:
#0 [optimized_mir] optimizing MIR for `hello_world::{closure#0}`
#1 [layout_of] computing layout of `[static generator@/checkout/src/test/ui/generator/issue-93161.rs:56:24: 62:2]`
#2 [layout_of] computing layout of `core::future::from_generator::GenFuture<[static generator@/checkout/src/test/ui/generator/issue-93161.rs:56:24: 62:2]>`
#2 [layout_of] computing layout of `core::future::from_generator::GenFuture<[static generator@/checkout/src/test/ui/generator/issue-93161.rs:56:24: 62:2]>`
#3 [layout_of] computing layout of `impl core::future::future::Future<Output = ()>`
#4 [mir_drops_elaborated_and_const_checked] elaborating drops for `main`
#5 [optimized_mir] optimizing MIR for `main`
#6 [collect_and_partition_mono_items] collect_and_partition_mono_items
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/generator/non-static-is-unpin.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generator/non-static-is-unpin.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/non-static-is-unpin/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/non-static-is-unpin/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: compiler/rustc_mir_transform/src/generator.rs:760:13: Broken MIR: generator contains type PhantomPinned in MIR, but typeck only knows about {()} and []
   |
   |
LL |     assert_unpin(|| {

thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1331:9
stack backtrace:
   0:     0x7fd0d536479c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h50e432a33816f17f
   0:     0x7fd0d536479c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h50e432a33816f17f
   1:     0x7fd0d53ca8b8 - core::fmt::write::he1187f1c735a6d05
   2:     0x7fd0d5354d31 - std::io::Write::write_fmt::h39a703c43ca9e701
   3:     0x7fd0d53677ae - std::panicking::default_hook::{{closure}}::h3ba44f2c4a1ab003
   4:     0x7fd0d5367476 - std::panicking::default_hook::hf88b5e434d766a47
   5:     0x7fd0d5d209b4 - rustc_driver[a26527cb16da58d4]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fd0d5367f62 - std::panicking::rust_panic_with_hook::hd6afc83c75fe8feb
   7:     0x7fd0d6326873 - std[ed124ce8448b1b72]::panicking::begin_panic::<rustc_errors[4c24d11ef95ca307]::ExplicitBug>::{closure#0}
   8:     0x7fd0d63220c6 - std[ed124ce8448b1b72]::sys_common::backtrace::__rust_end_short_backtrace::<std[ed124ce8448b1b72]::panicking::begin_panic<rustc_errors[4c24d11ef95ca307]::ExplicitBug>::{closure#0}, !>
   9:     0x7fd0d5b4f4c6 - std[ed124ce8448b1b72]::panicking::begin_panic::<rustc_errors[4c24d11ef95ca307]::ExplicitBug>
  10:     0x7fd0d6251966 - std[ed124ce8448b1b72]::panic::panic_any::<rustc_errors[4c24d11ef95ca307]::ExplicitBug>
  11:     0x7fd0d624f9d3 - <rustc_errors[4c24d11ef95ca307]::HandlerInner>::span_bug::<rustc_span[85cbc7cdad3332d9]::span_encoding::Span, &alloc[4cd35cc4a7f04eb5]::string::String>
  12:     0x7fd0d624f880 - <rustc_errors[4c24d11ef95ca307]::Handler>::span_bug::<rustc_span[85cbc7cdad3332d9]::span_encoding::Span, &alloc[4cd35cc4a7f04eb5]::string::String>
  13:     0x7fd0d63f53c5 - rustc_middle[e06d1b0d4a982b9c]::util::bug::opt_span_bug_fmt::<rustc_span[85cbc7cdad3332d9]::span_encoding::Span>::{closure#0}
  14:     0x7fd0d63f543b - rustc_middle[e06d1b0d4a982b9c]::ty::context::tls::with_opt::<rustc_middle[e06d1b0d4a982b9c]::util::bug::opt_span_bug_fmt<rustc_span[85cbc7cdad3332d9]::span_encoding::Span>::{closure#0}, ()>::{closure#0}
  15:     0x7fd0d63ef8a6 - rustc_middle[e06d1b0d4a982b9c]::ty::context::tls::with_context_opt::<rustc_middle[e06d1b0d4a982b9c]::ty::context::tls::with_opt<rustc_middle[e06d1b0d4a982b9c]::util::bug::opt_span_bug_fmt<rustc_span[85cbc7cdad3332d9]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  16:     0x7fd0d63ef819 - rustc_middle[e06d1b0d4a982b9c]::util::bug::opt_span_bug_fmt::<rustc_span[85cbc7cdad3332d9]::span_encoding::Span>
  17:     0x7fd0d5b53ff7 - rustc_middle[e06d1b0d4a982b9c]::util::bug::span_bug_fmt::<rustc_span[85cbc7cdad3332d9]::span_encoding::Span>
  18:     0x7fd0d62d981f - <rustc_mir_transform[4f795c61437e85be]::generator::StateTransform as rustc_middle[e06d1b0d4a982b9c]::mir::MirPass>::run_pass
  19:     0x7fd0d625f367 - rustc_mir_transform[4f795c61437e85be]::pass_manager::run_passes
  20:     0x7fd0d6364f7e - rustc_mir_transform[4f795c61437e85be]::optimized_mir
  21:     0x7fd0d78fb4c0 - rustc_query_system[66850b7ae8af285]::query::plumbing::try_execute_query::<rustc_query_impl[d84d3f2479a1091a]::plumbing::QueryCtxt, rustc_query_system[66850b7ae8af285]::query::caches::DefaultCache<rustc_span[85cbc7cdad3332d9]::def_id::DefId, &rustc_middle[e06d1b0d4a982b9c]::mir::Body>>
  22:     0x7fd0d79a8192 - rustc_query_system[66850b7ae8af285]::query::plumbing::get_query::<rustc_query_impl[d84d3f2479a1091a]::queries::optimized_mir, rustc_query_impl[d84d3f2479a1091a]::plumbing::QueryCtxt>
  23:     0x7fd0d74d66e9 - <rustc_query_impl[d84d3f2479a1091a]::Queries as rustc_middle[e06d1b0d4a982b9c]::ty::query::QueryEngine>::optimized_mir
  24:     0x7fd0d87b0dfe - <rustc_middle[e06d1b0d4a982b9c]::ty::context::TyCtxt>::generator_layout
  25:     0x7fd0d8745139 - <rustc_middle[e06d1b0d4a982b9c]::ty::layout::LayoutCx<rustc_middle[e06d1b0d4a982b9c]::ty::context::TyCtxt>>::layout_of_uncached
  26:     0x7fd0d876f66e - rustc_middle[e06d1b0d4a982b9c]::ty::context::tls::with_context::<rustc_middle[e06d1b0d4a982b9c]::ty::context::tls::with_related_context<rustc_middle[e06d1b0d4a982b9c]::ty::layout::layout_of::{closure#0}, core[cee74f9f94ff5162]::result::Result<rustc_target[25189b89ae46f52c]::abi::TyAndLayout<rustc_middle[e06d1b0d4a982b9c]::ty::Ty>, rustc_middle[e06d1b0d4a982b9c]::ty::layout::LayoutError>>::{closure#0}, core[cee74f9f94ff5162]::result::Result<rustc_target[25189b89ae46f52c]::abi::TyAndLayout<rustc_middle[e06d1b0d4a982b9c]::ty::Ty>, rustc_middle[e06d1b0d4a982b9c]::ty::layout::LayoutError>>::{closure#0}
  27:     0x7fd0d877324f - rustc_middle[e06d1b0d4a982b9c]::ty::layout::layout_of
  28:     0x7fd0d79f9d8d - rustc_query_system[66850b7ae8af285]::query::plumbing::get_query::<rustc_query_impl[d84d3f2479a1091a]::queries::layout_of, rustc_query_impl[d84d3f2479a1091a]::plumbing::QueryCtxt>
  29:     0x7fd0d75013c3 - <rustc_query_impl[d84d3f2479a1091a]::Queries as rustc_middle[e06d1b0d4a982b9c]::ty::query::QueryEngine>::layout_of
  30:     0x7fd0d61f6fbb - <rustc_mir_transform[4f795c61437e85be]::const_prop_lint::ConstProp as rustc_mir_transform[4f795c61437e85be]::pass_manager::MirLint>::run_lint
  31:     0x7fd0d625f367 - rustc_mir_transform[4f795c61437e85be]::pass_manager::run_passes
  32:     0x7fd0d63647e0 - rustc_mir_transform[4f795c61437e85be]::run_post_borrowck_cleanup_passes
  33:     0x7fd0d6363eb7 - rustc_mir_transform[4f795c61437e85be]::mir_drops_elaborated_and_const_checked
  34:     0x7fd0d78c7b84 - rustc_query_system[66850b7ae8af285]::query::plumbing::try_execute_query::<rustc_query_impl[d84d3f2479a1091a]::plumbing::QueryCtxt, rustc_query_system[66850b7ae8af285]::query::caches::DefaultCache<rustc_middle[e06d1b0d4a982b9c]::ty::WithOptConstParam<rustc_span[85cbc7cdad3332d9]::def_id::LocalDefId>, &rustc_data_structures[176448742242e606]::steal::Steal<rustc_middle[e06d1b0d4a982b9c]::mir::Body>>>
  35:     0x7fd0d79f3723 - rustc_query_system[66850b7ae8af285]::query::plumbing::get_query::<rustc_query_impl[d84d3f2479a1091a]::queries::mir_drops_elaborated_and_const_checked, rustc_query_impl[d84d3f2479a1091a]::plumbing::QueryCtxt>
  36:     0x7fd0d74d4b8a - <rustc_query_impl[d84d3f2479a1091a]::Queries as rustc_middle[e06d1b0d4a982b9c]::ty::query::QueryEngine>::mir_drops_elaborated_and_const_checked
  37:     0x7fd0d6364e48 - rustc_mir_transform[4f795c61437e85be]::optimized_mir
  38:     0x7fd0d78fb4c0 - rustc_query_system[66850b7ae8af285]::query::plumbing::try_execute_query::<rustc_query_impl[d84d3f2479a1091a]::plumbing::QueryCtxt, rustc_query_system[66850b7ae8af285]::query::caches::DefaultCache<rustc_span[85cbc7cdad3332d9]::def_id::DefId, &rustc_middle[e06d1b0d4a982b9c]::mir::Body>>
  39:     0x7fd0d79a8192 - rustc_query_system[66850b7ae8af285]::query::plumbing::get_query::<rustc_query_impl[d84d3f2479a1091a]::queries::optimized_mir, rustc_query_impl[d84d3f2479a1091a]::plumbing::QueryCtxt>
  40:     0x7fd0d74d66e9 - <rustc_query_impl[d84d3f2479a1091a]::Queries as rustc_middle[e06d1b0d4a982b9c]::ty::query::QueryEngine>::optimized_mir
  41:     0x7fd0d87bfcfd - <rustc_middle[e06d1b0d4a982b9c]::ty::context::TyCtxt>::instance_mir
  42:     0x7fd0d618ef95 - rustc_monomorphize[6a7659ee93aed75a]::collector::collect_neighbours
  43:     0x7fd0d6189a6a - rustc_monomorphize[6a7659ee93aed75a]::collector::collect_items_rec
  44:     0x7fd0d61cf981 - <rustc_session[f3aed6530d3c609e]::session::Session>::time::<(), rustc_monomorphize[6a7659ee93aed75a]::collector::collect_crate_mono_items::{closure#1}>
  45:     0x7fd0d6185ce4 - rustc_monomorphize[6a7659ee93aed75a]::collector::collect_crate_mono_items
  46:     0x7fd0d61963ba - rustc_monomorphize[6a7659ee93aed75a]::partitioning::collect_and_partition_mono_items
  47:     0x7fd0d791d652 - rustc_query_system[66850b7ae8af285]::query::plumbing::try_execute_query::<rustc_query_impl[d84d3f2479a1091a]::plumbing::QueryCtxt, rustc_query_system[66850b7ae8af285]::query::caches::DefaultCache<(), (&std[ed124ce8448b1b72]::collections::hash::set::HashSet<rustc_span[85cbc7cdad3332d9]::def_id::DefId, core[cee74f9f94ff5162]::hash::BuildHasherDefault<rustc_hash[a96122035317b32e]::FxHasher>>, &[rustc_middle[e06d1b0d4a982b9c]::mir::mono::CodegenUnit])>>
  48:     0x7fd0d79eeb4a - rustc_query_system[66850b7ae8af285]::query::plumbing::get_query::<rustc_query_impl[d84d3f2479a1091a]::queries::collect_and_partition_mono_items, rustc_query_impl[d84d3f2479a1091a]::plumbing::QueryCtxt>
  49:     0x7fd0d751a419 - <rustc_query_impl[d84d3f2479a1091a]::Queries as rustc_middle[e06d1b0d4a982b9c]::ty::query::QueryEngine>::collect_and_partition_mono_items
  50:     0x7fd0d5f82b08 - rustc_codegen_ssa[c9066fae9df7bbdc]::base::codegen_crate::<rustc_codegen_llvm[f25c4d6ffc7893b1]::LlvmCodegenBackend>
  51:     0x7fd0d606de59 - <rustc_codegen_llvm[f25c4d6ffc7893b1]::LlvmCodegenBackend as rustc_codegen_ssa[c9066fae9df7bbdc]::traits::backend::CodegenBackend>::codegen_crate
  52:     0x7fd0d5e8d401 - <rustc_session[f3aed6530d3c609e]::session::Session>::time::<alloc[4cd35cc4a7f04eb5]::boxed::Box<dyn core[cee74f9f94ff5162]::any::Any>, rustc_interface[c2e2522df7b8878]::passes::start_codegen::{closure#0}>
  53:     0x7fd0d5e35266 - <rustc_interface[c2e2522df7b8878]::passes::QueryContext>::enter::<<rustc_interface[c2e2522df7b8878]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[cee74f9f94ff5162]::result::Result<alloc[4cd35cc4a7f04eb5]::boxed::Box<dyn core[cee74f9f94ff5162]::any::Any>, rustc_errors[4c24d11ef95ca307]::ErrorGuaranteed>>
  54:     0x7fd0d5e4eead - <rustc_interface[c2e2522df7b8878]::queries::Queries>::ongoing_codegen
  55:     0x7fd0d5d2816c - <rustc_interface[c2e2522df7b8878]::interface::Compiler>::enter::<rustc_driver[a26527cb16da58d4]::run_compiler::{closure#1}::{closure#2}, core[cee74f9f94ff5162]::result::Result<core[cee74f9f94ff5162]::option::Option<rustc_interface[c2e2522df7b8878]::queries::Linker>, rustc_errors[4c24d11ef95ca307]::ErrorGuaranteed>>
  56:     0x7fd0d5d10068 - rustc_span[85cbc7cdad3332d9]::with_source_map::<core[cee74f9f94ff5162]::result::Result<(), rustc_errors[4c24d11ef95ca307]::ErrorGuaranteed>, rustc_interface[c2e2522df7b8878]::interface::create_compiler_and_run<core[cee74f9f94ff5162]::result::Result<(), rustc_errors[4c24d11ef95ca307]::ErrorGuaranteed>, rustc_driver[a26527cb16da58d4]::run_compiler::{closure#1}>::{closure#1}>
  57:     0x7fd0d5d29cd9 - rustc_interface[c2e2522df7b8878]::interface::create_compiler_and_run::<core[cee74f9f94ff5162]::result::Result<(), rustc_errors[4c24d11ef95ca307]::ErrorGuaranteed>, rustc_driver[a26527cb16da58d4]::run_compiler::{closure#1}>
  58:     0x7fd0d5d0a35f - <scoped_tls[3af945965f2be09]::ScopedKey<rustc_span[85cbc7cdad3332d9]::SessionGlobals>>::set::<rustc_interface[c2e2522df7b8878]::interface::run_compiler<core[cee74f9f94ff5162]::result::Result<(), rustc_errors[4c24d11ef95ca307]::ErrorGuaranteed>, rustc_driver[a26527cb16da58d4]::run_compiler::{closure#1}>::{closure#0}, core[cee74f9f94ff5162]::result::Result<(), rustc_errors[4c24d11ef95ca307]::ErrorGuaranteed>>
  59:     0x7fd0d5d13dc9 - std[ed124ce8448b1b72]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[c2e2522df7b8878]::util::run_in_thread_pool_with_globals<rustc_interface[c2e2522df7b8878]::interface::run_compiler<core[cee74f9f94ff5162]::result::Result<(), rustc_errors[4c24d11ef95ca307]::ErrorGuaranteed>, rustc_driver[a26527cb16da58d4]::run_compiler::{closure#1}>::{closure#0}, core[cee74f9f94ff5162]::result::Result<(), rustc_errors[4c24d11ef95ca307]::ErrorGuaranteed>>::{closure#0}, core[cee74f9f94ff5162]::result::Result<(), rustc_errors[4c24d11ef95ca307]::ErrorGuaranteed>>
  60:     0x7fd0d5d7df79 - <<std[ed124ce8448b1b72]::thread::Builder>::spawn_unchecked_<rustc_interface[c2e2522df7b8878]::util::run_in_thread_pool_with_globals<rustc_interface[c2e2522df7b8878]::interface::run_compiler<core[cee74f9f94ff5162]::result::Result<(), rustc_errors[4c24d11ef95ca307]::ErrorGuaranteed>, rustc_driver[a26527cb16da58d4]::run_compiler::{closure#1}>::{closure#0}, core[cee74f9f94ff5162]::result::Result<(), rustc_errors[4c24d11ef95ca307]::ErrorGuaranteed>>::{closure#0}, core[cee74f9f94ff5162]::result::Result<(), rustc_errors[4c24d11ef95ca307]::ErrorGuaranteed>>::{closure#1} as core[cee74f9f94ff5162]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  61:     0x7fd0d5373303 - std::sys::unix::thread::Thread::new::thread_start::hfccf706fa2afb703
  62:     0x7fd0cf8bf609 - start_thread
  63:     0x7fd0d51d2133 - clone
  64:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.64.0-nightly (4720ed2d1 2022-07-10) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
#0 [optimized_mir] optimizing MIR for `main::{closure#0}`
#1 [layout_of] computing layout of `[generator@/checkout/src/test/ui/generator/non-static-is-unpin.rs:13:18: 13:20]`
#1 [layout_of] computing layout of `[generator@/checkout/src/test/ui/generator/non-static-is-unpin.rs:13:18: 13:20]`
#2 [mir_drops_elaborated_and_const_checked] elaborating drops for `main`
#3 [optimized_mir] optimizing MIR for `main`
#4 [collect_and_partition_mono_items] collect_and_partition_mono_items
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/generator/overlap-locals.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generator/overlap-locals.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/overlap-locals/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/overlap-locals/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: compiler/rustc_mir_transform/src/generator.rs:760:13: Broken MIR: generator contains type i32 in MIR, but typeck only knows about {()} and []
   |
LL |     let a = || {
   |             ^^


thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1331:9
stack backtrace:
   0:     0x7fe135f5c79c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h50e432a33816f17f
   1:     0x7fe135fc28b8 - core::fmt::write::he1187f1c735a6d05
   2:     0x7fe135f4cd31 - std::io::Write::write_fmt::h39a703c43ca9e701
   3:     0x7fe135f5f7ae - std::panicking::default_hook::{{closure}}::h3ba44f2c4a1ab003
   4:     0x7fe135f5f476 - std::panicking::default_hook::hf88b5e434d766a47
   5:     0x7fe1369189b4 - rustc_driver[a26527cb16da58d4]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fe135f5ff62 - std::panicking::rust_panic_with_hook::hd6afc83c75fe8feb
   7:     0x7fe136f1e873 - std[ed124ce8448b1b72]::panicking::begin_panic::<rustc_errors[4c24d11ef95ca307]::ExplicitBug>::{closure#0}
   8:     0x7fe136f1a0c6 - std[ed124ce8448b1b72]::sys_common::backtrace::__rust_end_short_backtrace::<std[ed124ce8448b1b72]::panicking::begin_panic<rustc_errors[4c24d11ef95ca307]::ExplicitBug>::{closure#0}, !>
   9:     0x7fe1367474c6 - std[ed124ce8448b1b72]::panicking::begin_panic::<rustc_errors[4c24d11ef95ca307]::ExplicitBug>
  10:     0x7fe136e49966 - std[ed124ce8448b1b72]::panic::panic_any::<rustc_errors[4c24d11ef95ca307]::ExplicitBug>
  11:     0x7fe136e479d3 - <rustc_errors[4c24d11ef95ca307]::HandlerInner>::span_bug::<rustc_span[85cbc7cdad3332d9]::span_encoding::Span, &alloc[4cd35cc4a7f04eb5]::string::String>
  12:     0x7fe136e47880 - <rustc_errors[4c24d11ef95ca307]::Handler>::span_bug::<rustc_span[85cbc7cdad3332d9]::span_encoding::Span, &alloc[4cd35cc4a7f04eb5]::string::String>
  13:     0x7fe136fed3c5 - rustc_middle[e06d1b0d4a982b9c]::util::bug::opt_span_bug_fmt::<rustc_span[85cbc7cdad3332d9]::span_encoding::Span>::{closure#0}
  14:     0x7fe136fed43b - rustc_middle[e06d1b0d4a982b9c]::ty::context::tls::with_opt::<rustc_middle[e06d1b0d4a982b9c]::util::bug::opt_span_bug_fmt<rustc_span[85cbc7cdad3332d9]::span_encoding::Span>::{closure#0}, ()>::{closure#0}
  15:     0x7fe136fe78a6 - rustc_middle[e06d1b0d4a982b9c]::ty::context::tls::with_context_opt::<rustc_middle[e06d1b0d4a982b9c]::ty::context::tls::with_opt<rustc_middle[e06d1b0d4a982b9c]::util::bug::opt_span_bug_fmt<rustc_span[85cbc7cdad3332d9]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  16:     0x7fe136fe7819 - rustc_middle[e06d1b0d4a982b9c]::util::bug::opt_span_bug_fmt::<rustc_span[85cbc7cdad3332d9]::span_encoding::Span>
  17:     0x7fe13674bff7 - rustc_middle[e06d1b0d4a982b9c]::util::bug::span_bug_fmt::<rustc_span[85cbc7cdad3332d9]::span_encoding::Span>
  18:     0x7fe136ed181f - <rustc_mir_transform[4f795c61437e85be]::generator::StateTransform as rustc_middle[e06d1b0d4a982b9c]::mir::MirPass>::run_pass
  19:     0x7fe136e57367 - rustc_mir_transform[4f795c61437e85be]::pass_manager::run_passes
  20:     0x7fe136f5cf7e - rustc_mir_transform[4f795c61437e85be]::optimized_mir
  21:     0x7fe1384f34c0 - rustc_query_system[66850b7ae8af285]::query::plumbing::try_execute_query::<rustc_query_impl[d84d3f2479a1091a]::plumbing::QueryCtxt, rustc_query_system[66850b7ae8af285]::query::caches::DefaultCache<rustc_span[85cbc7cdad3332d9]::def_id::DefId, &rustc_middle[e06d1b0d4a982b9c]::mir::Body>>
  22:     0x7fe1385a0192 - rustc_query_system[66850b7ae8af285]::query::plumbing::get_query::<rustc_query_impl[d84d3f2479a1091a]::queries::optimized_mir, rustc_query_impl[d84d3f2479a1091a]::plumbing::QueryCtxt>
  23:     0x7fe1380ce6e9 - <rustc_query_impl[d84d3f2479a1091a]::Queries as rustc_middle[e06d1b0d4a982b9c]::ty::query::QueryEngine>::optimized_mir
  24:     0x7fe1393a8dfe - <rustc_middle[e06d1b0d4a982b9c]::ty::context::TyCtxt>::generator_layout
  25:     0x7fe13933d139 - <rustc_middle[e06d1b0d4a982b9c]::ty::layout::LayoutCx<rustc_middle[e06d1b0d4a982b9c]::ty::context::TyCtxt>>::layout_of_uncached
  26:     0x7fe13936766e - rustc_middle[e06d1b0d4a982b9c]::ty::context::tls::with_context::<rustc_middle[e06d1b0d4a982b9c]::ty::context::tls::with_related_context<rustc_middle[e06d1b0d4a982b9c]::ty::layout::layout_of::{closure#0}, core[cee74f9f94ff5162]::result::Result<rustc_target[25189b89ae46f52c]::abi::TyAndLayout<rustc_middle[e06d1b0d4a982b9c]::ty::Ty>, rustc_middle[e06d1b0d4a982b9c]::ty::layout::LayoutError>>::{closure#0}, core[cee74f9f94ff5162]::result::Result<rustc_target[25189b89ae46f52c]::abi::TyAndLayout<rustc_middle[e06d1b0d4a982b9c]::ty::Ty>, rustc_middle[e06d1b0d4a982b9c]::ty::layout::LayoutError>>::{closure#0}
  27:     0x7fe13936b24f - rustc_middle[e06d1b0d4a982b9c]::ty::layout::layout_of
  28:     0x7fe1385f1d8d - rustc_query_system[66850b7ae8af285]::query::plumbing::get_query::<rustc_query_impl[d84d3f2479a1091a]::queries::layout_of, rustc_query_impl[d84d3f2479a1091a]::plumbing::QueryCtxt>
  29:     0x7fe1380f93c3 - <rustc_query_impl[d84d3f2479a1091a]::Queries as rustc_middle[e06d1b0d4a982b9c]::ty::query::QueryEngine>::layout_of
  30:     0x7fe136deefbb - <rustc_mir_transform[4f795c61437e85be]::const_prop_lint::ConstProp as rustc_mir_transform[4f795c61437e85be]::pass_manager::MirLint>::run_lint
  31:     0x7fe136e57367 - rustc_mir_transform[4f795c61437e85be]::pass_manager::run_passes
  32:     0x7fe136f5c7e0 - rustc_mir_transform[4f795c61437e85be]::run_post_borrowck_cleanup_passes
  33:     0x7fe136f5beb7 - rustc_mir_transform[4f795c61437e85be]::mir_drops_elaborated_and_const_checked
  34:     0x7fe1384bfb84 - rustc_query_system[66850b7ae8af285]::query::plumbing::try_execute_query::<rustc_query_impl[d84d3f2479a1091a]::plumbing::QueryCtxt, rustc_query_system[66850b7ae8af285]::query::caches::DefaultCache<rustc_middle[e06d1b0d4a982b9c]::ty::WithOptConstParam<rustc_span[85cbc7cdad3332d9]::def_id::LocalDefId>, &rustc_data_structures[176448742242e606]::steal::Steal<rustc_middle[e06d1b0d4a982b9c]::mir::Body>>>
  35:     0x7fe1385eb723 - rustc_query_system[66850b7ae8af285]::query::plumbing::get_query::<rustc_query_impl[d84d3f2479a1091a]::queries::mir_drops_elaborated_and_const_checked, rustc_query_impl[d84d3f2479a1091a]::plumbing::QueryCtxt>
  36:     0x7fe1380ccb8a - <rustc_query_impl[d84d3f2479a1091a]::Queries as rustc_middle[e06d1b0d4a982b9c]::ty::query::QueryEngine>::mir_drops_elaborated_and_const_checked
  37:     0x7fe136f5ce48 - rustc_mir_transform[4f795c61437e85be]::optimized_mir
  38:     0x7fe1384f34c0 - rustc_query_system[66850b7ae8af285]::query::plumbing::try_execute_query::<rustc_query_impl[d84d3f2479a1091a]::plumbing::QueryCtxt, rustc_query_system[66850b7ae8af285]::query::caches::DefaultCache<rustc_span[85cbc7cdad3332d9]::def_id::DefId, &rustc_middle[e06d1b0d4a982b9c]::mir::Body>>
  39:     0x7fe1385a0192 - rustc_query_system[66850b7ae8af285]::query::plumbing::get_query::<rustc_query_impl[d84d3f2479a1091a]::queries::optimized_mir, rustc_query_impl[d84d3f2479a1091a]::plumbing::QueryCtxt>
  40:     0x7fe1380ce6e9 - <rustc_query_impl[d84d3f2479a1091a]::Queries as rustc_middle[e06d1b0d4a982b9c]::ty::query::QueryEngine>::optimized_mir
  41:     0x7fe1393b7cfd - <rustc_middle[e06d1b0d4a982b9c]::ty::context::TyCtxt>::instance_mir
  42:     0x7fe136d86f95 - rustc_monomorphize[6a7659ee93aed75a]::collector::collect_neighbours
  43:     0x7fe136d81a6a - rustc_monomorphize[6a7659ee93aed75a]::collector::collect_items_rec
  44:     0x7fe136dc7981 - <rustc_session[f3aed6530d3c609e]::session::Session>::time::<(), rustc_monomorphize[6a7659ee93aed75a]::collector::collect_crate_mono_items::{closure#1}>
  45:     0x7fe136d7dce4 - rustc_monomorphize[6a7659ee93aed75a]::collector::collect_crate_mono_items
  46:     0x7fe136d8e3ba - rustc_monomorphize[6a7659ee93aed75a]::partitioning::collect_and_partition_mono_items
  47:     0x7fe138515652 - rustc_query_system[66850b7ae8af285]::query::plumbing::try_execute_query::<rustc_query_impl[d84d3f2479a1091a]::plumbing::QueryCtxt, rustc_query_system[66850b7ae8af285]::query::caches::DefaultCache<(), (&std[ed124ce8448b1b72]::collections::hash::set::HashSet<rustc_span[85cbc7cdad3332d9]::def_id::DefId, core[cee74f9f94ff5162]::hash::BuildHasherDefault<rustc_hash[a96122035317b32e]::FxHasher>>, &[rustc_middle[e06d1b0d4a982b9c]::mir::mono::CodegenUnit])>>
  48:     0x7fe1385e6b4a - rustc_query_system[66850b7ae8af285]::query::plumbing::get_query::<rustc_query_impl[d84d3f2479a1091a]::queries::collect_and_partition_mono_items, rustc_query_impl[d84d3f2479a1091a]::plumbing::QueryCtxt>
  49:     0x7fe138112419 - <rustc_query_impl[d84d3f2479a1091a]::Queries as rustc_middle[e06d1b0d4a982b9c]::ty::query::QueryEngine>::collect_and_partition_mono_items
  50:     0x7fe136b7ab08 - rustc_codegen_ssa[c9066fae9df7bbdc]::base::codegen_crate::<rustc_codegen_llvm[f25c4d6ffc7893b1]::LlvmCodegenBackend>
  51:     0x7fe136c65e59 - <rustc_codegen_llvm[f25c4d6ffc7893b1]::LlvmCodegenBackend as rustc_codegen_ssa[c9066fae9df7bbdc]::traits::backend::CodegenBackend>::codegen_crate
  52:     0x7fe136a85401 - <rustc_session[f3aed6530d3c609e]::session::Session>::time::<alloc[4cd35cc4a7f04eb5]::boxed::Box<dyn core[cee74f9f94ff5162]::any::Any>, rustc_interface[c2e2522df7b8878]::passes::start_codegen::{closure#0}>
  53:     0x7fe136a2d266 - <rustc_interface[c2e2522df7b8878]::passes::QueryContext>::enter::<<rustc_interface[c2e2522df7b8878]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[cee74f9f94ff5162]::result::Result<alloc[4cd35cc4a7f04eb5]::boxed::Box<dyn core[cee74f9f94ff5162]::any::Any>, rustc_errors[4c24d11ef95ca307]::ErrorGuaranteed>>
  54:     0x7fe136a46ead - <rustc_interface[c2e2522df7b8878]::queries::Queries>::ongoing_codegen
  55:     0x7fe13692016c - <rustc_interface[c2e2522df7b8878]::interface::Compiler>::enter::<rustc_driver[a26527cb16da58d4]::run_compiler::{closure#1}::{closure#2}, core[cee74f9f94ff5162]::result::Result<core[cee74f9f94ff5162]::option::Option<rustc_interface[c2e2522df7b8878]::queries::Linker>, rustc_errors[4c24d11ef95ca307]::ErrorGuaranteed>>
  56:     0x7fe136908068 - rustc_span[85cbc7cdad3332d9]::with_source_map::<core[cee74f9f94ff5162]::result::Result<(), rustc_errors[4c24d11ef95ca307]::ErrorGuaranteed>, rustc_interface[c2e2522df7b8878]::interface::create_compiler_and_run<core[cee74f9f94ff5162]::result::Result<(), rustc_errors[4c24d11ef95ca307]::ErrorGuaranteed>, rustc_driver[a26527cb16da58d4]::run_compiler::{closure#1}>::{closure#1}>
  57:     0x7fe136921cd9 - rustc_interface[c2e2522df7b8878]::interface::create_compiler_and_run::<core[cee74f9f94ff5162]::result::Result<(), rustc_errors[4c24d11ef95ca307]::ErrorGuaranteed>, rustc_driver[a26527cb16da58d4]::run_compiler::{closure#1}>
  58:     0x7fe13690235f - <scoped_tls[3af945965f2be09]::ScopedKey<rustc_span[85cbc7cdad3332d9]::SessionGlobals>>::set::<rustc_interface[c2e2522df7b8878]::interface::run_compiler<core[cee74f9f94ff5162]::result::Result<(), rustc_errors[4c24d11ef95ca307]::ErrorGuaranteed>, rustc_driver[a26527cb16da58d4]::run_compiler::{closure#1}>::{closure#0}, core[cee74f9f94ff5162]::result::Result<(), rustc_errors[4c24d11ef95ca307]::ErrorGuaranteed>>
  59:     0x7fe13690bdc9 - std[ed124ce8448b1b72]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[c2e2522df7b8878]::util::run_in_thread_pool_with_globals<rustc_interface[c2e2522df7b8878]::interface::run_compiler<core[cee74f9f94ff5162]::result::Result<(), rustc_errors[4c24d11ef95ca307]::ErrorGuaranteed>, rustc_driver[a26527cb16da58d4]::run_compiler::{closure#1}>::{closure#0}, core[cee74f9f94ff5162]::result::Result<(), rustc_errors[4c24d11ef95ca307]::ErrorGuaranteed>>::{closure#0}, core[cee74f9f94ff5162]::result::Result<(), rustc_errors[4c24d11ef95ca307]::ErrorGuaranteed>>
  60:     0x7fe136975f79 - <<std[ed124ce8448b1b72]::thread::Builder>::spawn_unchecked_<rustc_interface[c2e2522df7b8878]::util::run_in_thread_pool_with_globals<rustc_interface[c2e2522df7b8878]::interface::run_compiler<core[cee74f9f94ff5162]::result::Result<(), rustc_errors[4c24d11ef95ca307]::ErrorGuaranteed>, rustc_driver[a26527cb16da58d4]::run_compiler::{closure#1}>::{closure#0}, core[cee74f9f94ff5162]::result::Result<(), rustc_errors[4c24d11ef95ca307]::ErrorGuaranteed>>::{closure#0}, core[cee74f9f94ff5162]::result::Result<(), rustc_errors[4c24d11ef95ca307]::ErrorGuaranteed>>::{closure#1} as core[cee74f9f94ff5162]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  61:     0x7fe135f6b303 - std::sys::unix::thread::Thread::new::thread_start::hfccf706fa2afb703
  62:     0x7fe1304b7609 - start_thread
  63:     0x7fe135dca133 - clone
  64:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.64.0-nightly (4720ed2d1 2022-07-10) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
#0 [optimized_mir] optimizing MIR for `main::{closure#0}`
#1 [layout_of] computing layout of `[generator@/checkout/src/test/ui/generator/overlap-locals.rs:6:13: 6:15]`
#1 [layout_of] computing layout of `[generator@/checkout/src/test/ui/generator/overlap-locals.rs:6:13: 6:15]`
#2 [mir_drops_elaborated_and_const_checked] elaborating drops for `main`
#3 [optimized_mir] optimizing MIR for `main`
#4 [collect_and_partition_mono_items] collect_and_partition_mono_items
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/generator/static-generators.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generator/static-generators.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/static-generators/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/static-generators/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: compiler/rustc_mir_transform/src/generator.rs:760:13: Broken MIR: generator contains type bool in MIR, but typeck only knows about {()} and []
   |
LL |     let mut generator = static || {
   |                         ^^^^^^^^^


thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1331:9
stack backtrace:
   0:     0x7f7528ca679c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h50e432a33816f17f
   1:     0x7f7528d0c8b8 - core::fmt::write::he1187f1c735a6d05
   2:     0x7f7528c96d31 - std::io::Write::write_fmt::h39a703c43ca9e701
   3:     0x7f7528ca97ae - std::panicking::default_hook::{{closure}}::h3ba44f2c4a1ab003
   4:     0x7f7528ca9476 - std::panicking::default_hook::hf88b5e434d766a47
   5:     0x7f75296629b4 - rustc_driver[a26527cb16da58d4]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f7528ca9f62 - std::panicking::rust_panic_with_hook::hd6afc83c75fe8feb
   7:     0x7f7529c68873 - std[ed124ce8448b1b72]::panicking::begin_panic::<rustc_errors[4c24d11ef95ca307]::ExplicitBug>::{closure#0}
   8:     0x7f7529c640c6 - std[ed124ce8448b1b72]::sys_common::backtrace::__rust_end_short_backtrace::<std[ed124ce8448b1b72]::panicking::begin_panic<rustc_errors[4c24d11ef95ca307]::ExplicitBug>::{closure#0}, !>
