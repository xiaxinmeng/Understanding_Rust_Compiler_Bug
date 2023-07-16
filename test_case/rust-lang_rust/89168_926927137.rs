backtrace

thread 'rustc' panicked at 'attempted to read from stolen value', /rustc/ac2d9fc509e36d1b32513744adf58c34bcc4f43c\compiler\rustc_data_structures\src\steal.rs:37:21
stack backtrace:
   0:     0x7fff9d127b2f - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h342cfc1ba001a676
   1:     0x7fff9d1527fa - core::fmt::write::h6ee6bf7a74285425
   2:     0x7fff9d11acf8 - <std::io::IoSliceMut as core::fmt::Debug>::fmt::h8ce43e4914059ee8
   3:     0x7fff9d12b616 - std::panicking::take_hook::h4296061bae338923
   4:     0x7fff9d12b104 - std::panicking::take_hook::h4296061bae338923
   5:     0x7fff58338a55 - <sha2::sha256::Sha256 as std::io::Write>::flush::h968fd60365b330d2
   6:     0x7fff9d12bf29 - std::panicking::rust_panic_with_hook::ha779c25870ce0d16
   7:     0x7fff9d12b99f - rust_begin_unwind
   8:     0x7fff9d128477 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h342cfc1ba001a676
   9:     0x7fff9d12b929 - rust_begin_unwind
  10:     0x7fff9d188ea0 - core::panicking::panic_fmt::he8f2c5437eb8215b
  11:     0x7fff5c54b9f1 - <rustc_trait_selection::traits::const_evaluatable::AbstractConstBuilder::new::IsThirPolymorphic as rustc_middle::thir::visit::Visitor>::visit_const::h545265a49fd5268a
  12:     0x7fff5c507e4d - <rustc_trait_selection::traits::specialize::OverlapError as core::fmt::Debug>::fmt::h13ef376dbff332d3
  13:     0x7fff5baad471 - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::try_mark_green::had5127b762eaa789
  14:     0x7fff5b918cdb - <rustc_middle::mir::ProjectionElem<rustc_middle::mir::Local,&rustc_middle::ty::TyS> as rustc_mir_dataflow::move_paths::abs_domain::Lift>::lift::hbec1935def6ab563
  15:     0x7fff5ba485b1 - rustc_query_impl::profiling_support::alloc_self_profile_query_strings::hd29f2e67246eba0a
  16:     0x7fff5b99abc1 - <rustc_middle::mir::ProjectionElem<rustc_middle::mir::Local,&rustc_middle::ty::TyS> as rustc_mir_dataflow::move_paths::abs_domain::Lift>::lift::hbec1935def6ab563
  17:     0x7fff5ba862ae - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::try_mark_green::had5127b762eaa789
  18:     0x7fff5c54a1dd - rustc_trait_selection::traits::const_evaluatable::AbstractConst::new::h414e0aa738092678
  19:     0x7fff5c54bc13 - <rustc_trait_selection::traits::const_evaluatable::AbstractConstBuilder::new::IsThirPolymorphic as rustc_middle::thir::visit::Visitor>::visit_const::h545265a49fd5268a
  20:     0x7fff5bab54a9 - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::try_mark_green::had5127b762eaa789
  21:     0x7fff5b926183 - <rustc_middle::mir::ProjectionElem<rustc_middle::mir::Local,&rustc_middle::ty::TyS> as rustc_mir_dataflow::move_paths::abs_domain::Lift>::lift::hbec1935def6ab563
  22:     0x7fff5ba30783 - rustc_query_impl::profiling_support::alloc_self_profile_query_strings::hd29f2e67246eba0a
  23:     0x7fff5b95f90b - <rustc_middle::mir::ProjectionElem<rustc_middle::mir::Local,&rustc_middle::ty::TyS> as rustc_mir_dataflow::move_paths::abs_domain::Lift>::lift::hbec1935def6ab563
  24:     0x7fff5ba8630c - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::try_mark_green::had5127b762eaa789
  25:     0x7fff5c5eebcd - rustc_infer::infer::InferCtxt::try_unify_abstract_consts::hef82dceb5044f9bf
  26:     0x7fff5c4edee4 - <rustc_trait_selection::traits::on_unimplemented::OnUnimplementedDirective as core::fmt::Debug>::fmt::h979a391978f48d5f
  27:     0x7fff5c52e466 - rustc_trait_selection::traits::select::SelectionContext::select::h609c60a63c61778c
  28:     0x7fff5c51eab5 - rustc_trait_selection::traits::select::SelectionContext::evaluate_root_obligation::h34d30ab57e0346fe
  29:     0x7fff5c441e5e - <unicode_normalization::stream_safe::Decomposition as core::fmt::Debug>::fmt::ha9479800486b1132
  30:     0x7fff5c51c373 - <rustc_trait_selection::traits::VtblSegment as core::fmt::Debug>::fmt::h1bcc95332b77490e
  31:     0x7fff5c5186b4 - <rustc_trait_selection::traits::VtblSegment as core::fmt::Debug>::fmt::h1bcc95332b77490e
  32:     0x7fff5c495648 - <rustc_trait_selection::traits::project::ProjectionTyCandidate as core::fmt::Debug>::fmt::h9f3f556d62cf8809
  33:     0x7fff5c53aa08 - <rustc_trait_selection::traits::select::ProvisionalEvaluation as core::fmt::Debug>::fmt::h3f8c74ae2996fcfc
  34:     0x7fff5c522760 - <rustc_trait_selection::traits::select::TraitObligationStack as core::fmt::Debug>::fmt::h396f16fd173a60e1
  35:     0x7fff5c51fdab - rustc_trait_selection::traits::select::SelectionContext::evaluate_root_obligation::h34d30ab57e0346fe
  36:     0x7fff5c4954d0 - <rustc_trait_selection::traits::project::ProjectionTyCandidate as core::fmt::Debug>::fmt::h9f3f556d62cf8809
  37:     0x7fff5c53a4e4 - <rustc_trait_selection::traits::select::ProvisionalEvaluation as core::fmt::Debug>::fmt::h3f8c74ae2996fcfc
  38:     0x7fff5c51f0e3 - rustc_trait_selection::traits::select::SelectionContext::evaluate_root_obligation::h34d30ab57e0346fe
  39:     0x7fff5c4edfc7 - <rustc_trait_selection::traits::on_unimplemented::OnUnimplementedDirective as core::fmt::Debug>::fmt::h979a391978f48d5f
  40:     0x7fff5c52e466 - rustc_trait_selection::traits::select::SelectionContext::select::h609c60a63c61778c
  41:     0x7fff5c51e895 - rustc_trait_selection::traits::select::SelectionContext::evaluate_root_obligation::h34d30ab57e0346fe
  42:     0x7fff5c442594 - <unicode_normalization::stream_safe::Decomposition as core::fmt::Debug>::fmt::ha9479800486b1132
  43:     0x7fff5c52e651 - rustc_trait_selection::traits::select::SelectionContext::select::h609c60a63c61778c
  44:     0x7fff5c51fdee - rustc_trait_selection::traits::select::SelectionContext::evaluate_root_obligation::h34d30ab57e0346fe
  45:     0x7fff5c4954d0 - <rustc_trait_selection::traits::project::ProjectionTyCandidate as core::fmt::Debug>::fmt::h9f3f556d62cf8809
  46:     0x7fff5c53a4e4 - <rustc_trait_selection::traits::select::ProvisionalEvaluation as core::fmt::Debug>::fmt::h3f8c74ae2996fcfc
  47:     0x7fff5c51f0e3 - rustc_trait_selection::traits::select::SelectionContext::evaluate_root_obligation::h34d30ab57e0346fe
  48:     0x7fff5c4edfc7 - <rustc_trait_selection::traits::on_unimplemented::OnUnimplementedDirective as core::fmt::Debug>::fmt::h979a391978f48d5f
  49:     0x7fff5c52e466 - rustc_trait_selection::traits::select::SelectionContext::select::h609c60a63c61778c
  50:     0x7fff5c441fe9 - <unicode_normalization::stream_safe::Decomposition as core::fmt::Debug>::fmt::ha9479800486b1132
  51:     0x7fff5b5e3c67 - ZN295_$LT$$RF$rustc_middle..ty..list..List$LT$rustc_middle..ty..sty..Binder$LT$rustc_middle..ty..sty..ExistentialPredicate$GT$$GT$$u20$as$u20$rustc_traits..chalk..lowering..LowerInto$LT$chalk_ir..Binders$LT$chalk_ir..QuantifiedWhereClauses$LT$rustc_middle
  52:     0x7fff5b673c1d - <rustc_traits::chalk::lowering::RegionsSubstitutor as rustc_middle::ty::fold::TypeFolder>::fold_region::h9ebda381bd21c3f7
  53:     0x7fff5bab4794 - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::try_mark_green::had5127b762eaa789
  54:     0x7fff5bb410c3 - rustc_query_impl::query_callbacks::limits::force_from_dep_node::h96cb857ad4a7d970
  55:     0x7fff5ba3ae81 - rustc_query_impl::profiling_support::alloc_self_profile_query_strings::hd29f2e67246eba0a
  56:     0x7fff5b98dea2 - <rustc_middle::mir::ProjectionElem<rustc_middle::mir::Local,&rustc_middle::ty::TyS> as rustc_mir_dataflow::move_paths::abs_domain::Lift>::lift::hbec1935def6ab563
  57:     0x7fff5ba910b2 - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::try_mark_green::had5127b762eaa789
  58:     0x7fff5c47b9ee - <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::query::evaluate_obligation::InferCtxtExt>::evaluate_obligation::h8403fc84f3f869fa
  59:     0x7fff5c47bd30 - <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::query::evaluate_obligation::InferCtxtExt>::evaluate_obligation_no_overflow::h0a6e294923a856e4
  60:     0x7fff5c511aac - rustc_trait_selection::traits::type_known_to_meet_bound_modulo_regions::h9fadd4373303a291
  61:     0x7fff5aec2282 - rustc_ty_utils::instance::provide::h78b2df9707c4f97e
  62:     0x7fff5aec6b45 - rustc_ty_utils::instance::provide::h78b2df9707c4f97e
  63:     0x7fff5bab927f - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::try_mark_green::had5127b762eaa789
  64:     0x7fff5bb2012e - rustc_query_impl::query_callbacks::limits::force_from_dep_node::h96cb857ad4a7d970
  65:     0x7fff5ba4d2a8 - rustc_query_impl::profiling_support::alloc_self_profile_query_strings::hd29f2e67246eba0a
  66:     0x7fff5b8a4163 - <rustc_middle::mir::ProjectionElem<rustc_middle::mir::Local,&rustc_middle::ty::TyS> as rustc_mir_dataflow::move_paths::abs_domain::Lift>::lift::hbec1935def6ab563
  67:     0x7fff5ba8bf8c - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::try_mark_green::had5127b762eaa789
  68:     0x7fff5c83fc6c - rustc_middle::ty::util::<impl rustc_middle::ty::TyS>::is_copy_modulo_regions::heb24e785df70a8ce
  69:     0x7fff5aed2b7f - rustc_ty_utils::provide::h82abcf6ccf13bbe8
  70:     0x7fff5bab927f - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::try_mark_green::had5127b762eaa789
  71:     0x7fff5bb2012e - rustc_query_impl::query_callbacks::limits::force_from_dep_node::h96cb857ad4a7d970
  72:     0x7fff5ba4d2a8 - rustc_query_impl::profiling_support::alloc_self_profile_query_strings::hd29f2e67246eba0a
  73:     0x7fff5b8a4163 - <rustc_middle::mir::ProjectionElem<rustc_middle::mir::Local,&rustc_middle::ty::TyS> as rustc_mir_dataflow::move_paths::abs_domain::Lift>::lift::hbec1935def6ab563
  74:     0x7fff5ba8c54c - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::try_mark_green::had5127b762eaa789
  75:     0x7fff5b1a2904 - rustc_passes::provide::hd8a5c7c0bd911f87
  76:     0x7fff5b1d673f - rustc_passes::provide::hd8a5c7c0bd911f87
  77:     0x7fff5b1a91b8 - rustc_passes::provide::hd8a5c7c0bd911f87
  78:     0x7fff5b23ab7a - <rustc_mir_build::build::matches::ArmHasGuard as core::fmt::Debug>::fmt::h6a893499fc82f91a
  79:     0x7fff5b1a7370 - rustc_passes::provide::hd8a5c7c0bd911f87
  80:     0x7fff5baafb8f - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::try_mark_green::had5127b762eaa789
  81:     0x7fff5bb1b55c - rustc_query_impl::query_callbacks::limits::force_from_dep_node::h96cb857ad4a7d970
  82:     0x7fff5ba3db90 - rustc_query_impl::profiling_support::alloc_self_profile_query_strings::hd29f2e67246eba0a
  83:     0x7fff5b8206de - <rustc_middle::mir::ProjectionElem<rustc_middle::mir::Local,&rustc_middle::ty::TyS> as rustc_mir_dataflow::move_paths::abs_domain::Lift>::lift::hbec1935def6ab563
  84:     0x7fff5ba8605c - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::try_mark_green::had5127b762eaa789
  85:     0x7fff5aa1e231 - <rustc_mir_transform::check_unsafety::UnusedUnsafeVisitor as rustc_hir::intravisit::Visitor>::visit_block::hf29957a85a0204a0
  86:     0x7fff5aa18dab - <rustc_mir_transform::mir_keys::GatherCtors as rustc_hir::intravisit::Visitor>::visit_variant_data::hb8a913f2fe721c2a
  87:     0x7fff5baac696 - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::try_mark_green::had5127b762eaa789
  88:     0x7fff5bb124dc - rustc_query_impl::query_callbacks::limits::force_from_dep_node::h96cb857ad4a7d970
  89:     0x7fff5ba41025 - rustc_query_impl::profiling_support::alloc_self_profile_query_strings::hd29f2e67246eba0a
  90:     0x7fff5b88783d - <rustc_middle::mir::ProjectionElem<rustc_middle::mir::Local,&rustc_middle::ty::TyS> as rustc_mir_dataflow::move_paths::abs_domain::Lift>::lift::hbec1935def6ab563
  91:     0x7fff5b7d42e5 - <rustc_middle::mir::ProjectionElem<rustc_middle::mir::Local,&rustc_middle::ty::TyS> as rustc_mir_dataflow::move_paths::abs_domain::Lift>::lift::hbec1935def6ab563
  92:     0x7fff5bad261e - rustc_query_impl::query_callbacks::unsafety_check_result::force_from_dep_node::h8b7aae90ae54903a
  93:     0x7fff5bb09772 - rustc_query_impl::query_callbacks::limits::force_from_dep_node::h96cb857ad4a7d970
  94:     0x7fff5bb0974a - rustc_query_impl::query_callbacks::limits::force_from_dep_node::h96cb857ad4a7d970
  95:     0x7fff5bb0974a - rustc_query_impl::query_callbacks::limits::force_from_dep_node::h96cb857ad4a7d970
  96:     0x7fff5bae282b - rustc_query_impl::query_callbacks::limits::force_from_dep_node::h96cb857ad4a7d970
  97:     0x7fff5b7bb34e - <rustc_middle::mir::ProjectionElem<rustc_middle::mir::Local,&rustc_middle::ty::TyS> as rustc_mir_dataflow::move_paths::abs_domain::Lift>::lift::hbec1935def6ab563
  98:     0x7fff5ba8a100 - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::try_mark_green::had5127b762eaa789
  99:     0x7fff5848651b - rustc_interface::queries::Linker::link::hc04c4a994ab9b768
 100:     0x7fff5847646b - rustc_interface::passes::analysis::h75d74c7cad0deeec
 101:     0x7fff5baace3b - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::try_mark_green::had5127b762eaa789
 102:     0x7fff5bb0f62b - rustc_query_impl::query_callbacks::limits::force_from_dep_node::h96cb857ad4a7d970
 103:     0x7fff5ba4b362 - rustc_query_impl::profiling_support::alloc_self_profile_query_strings::hd29f2e67246eba0a
 104:     0x7fff5b859d89 - <rustc_middle::mir::ProjectionElem<rustc_middle::mir::Local,&rustc_middle::ty::TyS> as rustc_mir_dataflow::move_paths::abs_domain::Lift>::lift::hbec1935def6ab563
 105:     0x7fff5ba855b5 - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::try_mark_green::had5127b762eaa789
 106:     0x7fff5838f5cf - <regex_syntax::hir::literal::Literal as core::convert::AsRef<[u8]>>::as_ref::hef8a3a16126ef78f
 107:     0x7fff58357f32 - rustc_driver::pretty::print_after_hir_lowering::h84138d7a30457a82
 108:     0x7fff58391816 - <regex_syntax::hir::literal::Literal as core::convert::AsRef<[u8]>>::as_ref::hef8a3a16126ef78f
 109:     0x7fff5835db1a - <tracing_subscriber::util::TryInitError as core::fmt::Display>::fmt::h05e1aa411f5833f8
 110:     0x7fff5834cbcd - <rustc_driver::Compilation as core::fmt::Debug>::fmt::hf017fdd1b4af7b64
 111:     0x7fff9d13a35c - std::sys::windows::thread::Thread::new::hd7c7d6731c34176f
 112:     0x7fffed9d7034 - BaseThreadInitThunk
 113:     0x7fffee622651 - RtlUserThreadStart

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.57.0-nightly (ac2d9fc50 2021-09-21) running on x86_64-pc-windows-msvc

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [thir_abstract_const_of_const_arg] building an abstract representation for the const argument core_simd::to_bytes::<impl at crates\core_simd\src\to_bytes.rs:3:9: 19:10>::from_ne_bytes::{constant#0}
#1 [try_unify_abstract_consts] trying to unify the generic constants core_simd::to_bytes::<impl at crates\core_simd\src\to_bytes.rs:3:9: 19:10>::from_ne_bytes::{constant#0} and core_simd::to_bytes::<impl at crates\core_simd\src\to_bytes.rs:3:9: 19:10>::{constant#0}
#2 [evaluate_obligation] evaluating trait selection obligation `core_simd::vector::Simd<u8, {{ $size * LANES }}>: std::marker::Copy`
#3 [is_copy_raw] computing whether `core_simd::vector::Simd<u8, {{ $size * LANES }}>` is `Copy`
#4 [needs_drop_raw] computing whether `core_simd::vector::Simd<u8, {{ $size * LANES }}>` needs drop
#5 [mir_built] building MIR for `core_simd::to_bytes::<impl at crates\core_simd\src\to_bytes.rs:3:9: 19:10>::from_ne_bytes`
#6 [unsafety_check_result] unsafety-checking `core_simd::to_bytes::<impl at crates\core_simd\src\to_bytes.rs:3:9: 19:10>::from_ne_bytes`
#7 [analysis] running analysis passes on this crate
end of query stack

