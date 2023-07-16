
# Third 30 entries excluding drop_in_place and various panic helpers.
n=20649 s=3  <core::cell::Cell<isize>>::replace
n=18672 s=1  <alloc::vec::Vec<u8> as core::ops::drop::Drop>::drop
n=8691  s=1  <core::ptr::unique::Unique<u8> as core::convert::Into<core::ptr::non_null::NonNull<u8>>>::into
n=6194  s=7  <[u8] as core::cmp::PartialEq>::eq
n=5741  s=3  <core::cell::Cell<usize>>::replace
n=5350  s=1  <rustc_query_system::dep_graph::graph::DepNodeIndex as core::convert::Into<rustc_data_structures::profiling::QueryInvocationId>>::into
n=4693  s=2  alloc::alloc::box_free::<rustc_ast::ast::Ty, alloc::alloc::Global>
n=1950  s=1  <[rustc_middle::ty::subst::GenericArg; 8] as smallvec::Array>::size
n=1930  s=2  <rustc_ast::ptr::P<rustc_ast::ast::Expr> as core::ops::deref::Deref>::deref
n=1760  s=1  <[rustc_middle::ty::Ty; 8] as smallvec::Array>::size
n=1740  s=2  <rustc_errors::diagnostic_builder::DiagnosticBuilder<rustc_errors::ErrorGuaranteed>>::emit
n=1699  s=1  rustc_data_structures::sync::assert_sync::<rustc_middle::ty::context::tls::ImplicitCtxt>
n=1560  s=1  <rustc_hir_analysis::check::inherited::Inherited as core::ops::deref::Deref>::deref
n=1509  s=1  <core::hash::BuildHasherDefault<rustc_hash::FxHasher> as core::default::Default>::default
n=1498  s=2  alloc::alloc::box_free::<rustc_ast::ast::Pat, alloc::alloc::Global>
n=1481  s=6  <&str as core::convert::Into<alloc::borrow::Cow<str>>>::into
n=1119  s=2  <rustc_ast::ptr::P<rustc_ast::ast::Ty> as core::ops::deref::Deref>::deref
n=1112  s=1  <[rustc_middle::ty::sty::Binder<rustc_middle::ty::sty::ExistentialPredicate>; 8] as smallvec::Array>::size
n=1047  s=6  smallvec::infallible::<()>
n=923   s=2  <rustc_target::abi::TyAndLayout<rustc_middle::ty::Ty> as core::ops::deref::Deref>::deref
n=874   s=6  <std::path::Path>::new::<std::ffi::os_str::OsString>
n=850   s=1  <alloc::vec::Vec<rustc_span::span_encoding::Span> as core::ops::drop::Drop>::drop
n=848   s=6  _$LT$alloc..string..String$u20$as$u20$core..clone..Clone$GT$::clone::h3a6e79de4ee6835e
n=825   s=2  alloc::alloc::box_free::<syn::expr::Expr, alloc::alloc::Global>
n=779   s=2  <tracing_core::metadata::Metadata>::fields
n=761   s=3  <hashbrown::set::HashSet<rustc_target::asm::InlineAsmReg, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>::insert
n=706   s=1  <rustc_span::def_id::DefId as core::borrow::Borrow<rustc_span::def_id::DefId>>::borrow
n=696   s=3  <rustc_data_structures::atomic_ref::AtomicRef<fn(rustc_span::def_id::LocalDefId)> as core::ops::deref::Deref>::deref
n=694   s=5  alloc::alloc::box_free::<dyn core::error::Error + core::marker::Send + core::marker::Sync, alloc::alloc::Global>
