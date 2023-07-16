
thread 'rustc' panicked at 'range end index 3 out of range for slice of length 2', library\core\src\slice\index.rs:73:5
stack backtrace:
   0:     0x7ffbd88d9fbf - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hf958371dc84a30d1
   1:     0x7ffbd8914b5a - core::fmt::write::h8bc5f8dfcde4777f
   2:     0x7ffbd88cc749 - <std::io::IoSlice as core::fmt::Debug>::fmt::hab1bb118c5bfde47
   3:     0x7ffbd88dd8bb - std::panicking::default_hook::hcfea80c086f9466a
   4:     0x7ffbd88dd535 - std::panicking::default_hook::hcfea80c086f9466a
   5:     0x7ffbc668c404 - rustc_driver[a0a607043376aa59]::pretty::print_after_hir_lowering
   6:     0x7ffbd88de062 - std::panicking::rust_panic_with_hook::h43b18a7cf7089063
   7:     0x7ffbd88dddfd - <std::panicking::begin_panic_handler::StrPanicPayload as core::panic::BoxMeUp>::get::h51b3df288982544c
   8:     0x7ffbd88dabf7 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hf958371dc84a30d1
   9:     0x7ffbd88ddad9 - rust_begin_unwind
  10:     0x7ffbd894ae95 - core::panicking::panic_fmt::h8a30ca8e1c32c5a5
  11:     0x7ffbd891819b - core::slice::index::slice_end_index_len_fail_rt::h50f0026ee1d37708
  12:     0x7ffbd8908cd9 - <alloc::string::FromUtf16Error as core::fmt::Debug>::fmt::h3c7bd1d5ad2e3173
  13:     0x7ffbd890fb59 - <core::num::error::ParseIntError as core::fmt::Display>::fmt::h02d30396acb0f40b
  14:     0x7ffbd894b039 - core::slice::index::slice_end_index_len_fail::hd043e08f80e8bf95
  15:     0x7ffbc88a4a6b - <rustc_middle[345e23a01d15c30c]::ty::generics::Generics>::own_substs
  16:     0x7ffbc87c4791 - <rustc_infer[9921ae7fd9eefb8a]::infer::error_reporting::need_type_info::FindInferSourceVisitor as rustc_hir[94f5a242fc3ad22c]::intravisit::Visitor>::visit_expr
  17:     0x7ffbc87c45aa - <rustc_infer[9921ae7fd9eefb8a]::infer::error_reporting::need_type_info::FindInferSourceVisitor as rustc_hir[94f5a242fc3ad22c]::intravisit::Visitor>::visit_expr
  18:     0x7ffbc87c2d61 - <rustc_infer[9921ae7fd9eefb8a]::infer::error_reporting::need_type_info::FindInferSourceVisitor as rustc_hir[94f5a242fc3ad22c]::intravisit::Visitor>::visit_body
  19:     0x7ffbc87bfb66 - <rustc_infer[9921ae7fd9eefb8a]::infer::resolve::UnresolvedTypeFinder as rustc_middle[345e23a01d15c30c]::ty::visit::TypeVisitor>::visit_ty
  20:     0x7ffbc8817164 - <rustc_infer[9921ae7fd9eefb8a]::infer::lexical_region_resolve::RegionResolutionError as core[f4e460384c48c425]::fmt::Debug>::fmt
  21:     0x7ffbc87c45aa - <rustc_infer[9921ae7fd9eefb8a]::infer::error_reporting::need_type_info::FindInferSourceVisitor as rustc_hir[94f5a242fc3ad22c]::intravisit::Visitor>::visit_expr
  22:     0x7ffbc8816ff0 - <rustc_infer[9921ae7fd9eefb8a]::infer::lexical_region_resolve::RegionResolutionError as core[f4e460384c48c425]::fmt::Debug>::fmt
  23:     0x7ffbc87c45aa - <rustc_infer[9921ae7fd9eefb8a]::infer::error_reporting::need_type_info::FindInferSourceVisitor as rustc_hir[94f5a242fc3ad22c]::intravisit::Visitor>::visit_expr
  24:     0x7ffbc8816ff0 - <rustc_infer[9921ae7fd9eefb8a]::infer::lexical_region_resolve::RegionResolutionError as core[f4e460384c48c425]::fmt::Debug>::fmt
  25:     0x7ffbc87c45aa - <rustc_infer[9921ae7fd9eefb8a]::infer::error_reporting::need_type_info::FindInferSourceVisitor as rustc_hir[94f5a242fc3ad22c]::intravisit::Visitor>::visit_expr
  26:     0x7ffbc87c4582 - <rustc_infer[9921ae7fd9eefb8a]::infer::error_reporting::need_type_info::FindInferSourceVisitor as rustc_hir[94f5a242fc3ad22c]::intravisit::Visitor>::visit_expr
  27:     0x7ffbc8817074 - <rustc_infer[9921ae7fd9eefb8a]::infer::lexical_region_resolve::RegionResolutionError as core[f4e460384c48c425]::fmt::Debug>::fmt
  28:     0x7ffbc87c45aa - <rustc_infer[9921ae7fd9eefb8a]::infer::error_reporting::need_type_info::FindInferSourceVisitor as rustc_hir[94f5a242fc3ad22c]::intravisit::Visitor>::visit_expr
  29:     0x7ffbc8813e5d - <rustc_infer[9921ae7fd9eefb8a]::infer::lexical_region_resolve::RegionResolutionError as core[f4e460384c48c425]::fmt::Debug>::fmt
  30:     0x7ffbc87c2b19 - <rustc_infer[9921ae7fd9eefb8a]::infer::error_reporting::need_type_info::FindInferSourceVisitor as rustc_hir[94f5a242fc3ad22c]::intravisit::Visitor>::visit_local
  31:     0x7ffbc8813ce8 - <rustc_infer[9921ae7fd9eefb8a]::infer::lexical_region_resolve::RegionResolutionError as core[f4e460384c48c425]::fmt::Debug>::fmt
  32:     0x7ffbc87c45aa - <rustc_infer[9921ae7fd9eefb8a]::infer::error_reporting::need_type_info::FindInferSourceVisitor as rustc_hir[94f5a242fc3ad22c]::intravisit::Visitor>::visit_expr
  33:     0x7ffbc878baf8 - <rustc_infer[9921ae7fd9eefb8a]::infer::InferCtxt>::emit_inference_failure_err
  34:     0x7ffbc87581f6 - <rustc_infer[9921ae7fd9eefb8a]::infer::InferCtxt as rustc_trait_selection[d95719c364848bec]::traits::error_reporting::InferCtxtPrivExt>::maybe_report_ambiguity
  35:     0x7ffbc87463af - <rustc_infer[9921ae7fd9eefb8a]::infer::InferCtxt as rustc_trait_selection[d95719c364848bec]::traits::error_reporting::InferCtxtExt>::report_fulfillment_errors
  36:     0x7ffbc5b8c05c - <rustc_infer[9921ae7fd9eefb8a]::infer::outlives::env::OutlivesEnvironment as rustc_typeck[d2a199ddac7b0ad4]::check::regionck::OutlivesEnvironmentExt>::add_implied_bounds
  37:     0x7ffbc5b489a5 - <rustc_typeck[d2a199ddac7b0ad4]::check::UnsafetyState>::recurse
  38:     0x7ffbc4a8ee88 - rustc_query_impl[eb123e80d0f4deec]::profiling_support::alloc_self_profile_query_strings
  39:     0x7ffbc4bd5081 - <&[rustc_span[7d253b2ad3f54c9a]::def_id::DefId] as rustc_serialize[f7496f00b61bd66d]::serialize::Decodable<rustc_query_impl[eb123e80d0f4deec]::on_disk_cache::CacheDecoder>>::decode
  40:     0x7ffbc6059adc - <rustc_span[7d253b2ad3f54c9a]::def_id::DefId as rustc_serialize[f7496f00b61bd66d]::serialize::Encodable<rustc_query_impl[eb123e80d0f4deec]::on_disk_cache::CacheEncoder>>::encode
  41:     0x7ffbc5f9cbd2 - <rustc_query_impl[eb123e80d0f4deec]::Queries as rustc_middle[345e23a01d15c30c]::ty::query::QueryEngine>::as_any
  42:     0x7ffbc5b77be5 - <<rustc_typeck[d2a199ddac7b0ad4]::check::fn_ctxt::FnCtxt>::instantiate_value_path::CreateCtorSubstsContext as rustc_typeck[d2a199ddac7b0ad4]::astconv::CreateSubstsForGenericArgsCtxt>::inferred_kind
  43:     0x7ffbc488006a - <rustc_typeck[d2a199ddac7b0ad4]::check::fn_ctxt::FnCtxt>::write_user_type_annotation
  44:     0x7ffbc4896b11 - rustc_typeck[d2a199ddac7b0ad4]::check::provide
  45:     0x7ffbc4a909ae - rustc_query_impl[eb123e80d0f4deec]::profiling_support::alloc_self_profile_query_strings
  46:     0x7ffbc4bf555e - <&[rustc_span[7d253b2ad3f54c9a]::def_id::DefId] as rustc_serialize[f7496f00b61bd66d]::serialize::Decodable<rustc_query_impl[eb123e80d0f4deec]::on_disk_cache::CacheDecoder>>::decode
  47:     0x7ffbc4b6a08d - <&[rustc_span[7d253b2ad3f54c9a]::def_id::DefId] as rustc_serialize[f7496f00b61bd66d]::serialize::Decodable<rustc_query_impl[eb123e80d0f4deec]::on_disk_cache::CacheDecoder>>::decode
  48:     0x7ffbc4ba8b3c - <&[rustc_span[7d253b2ad3f54c9a]::def_id::DefId] as rustc_serialize[f7496f00b61bd66d]::serialize::Decodable<rustc_query_impl[eb123e80d0f4deec]::on_disk_cache::CacheDecoder>>::decode
  49:     0x7ffbc487fb9a - <rustc_typeck[d2a199ddac7b0ad4]::check::fn_ctxt::FnCtxt>::write_user_type_annotation
  50:     0x7ffbc482e62b - rustc_typeck[d2a199ddac7b0ad4]::check_crate
  51:     0x7ffbc4095e45 - rustc_interface[e31b1d572a2c621e]::passes::analysis
  52:     0x7ffbc4a905de - rustc_query_impl[eb123e80d0f4deec]::profiling_support::alloc_self_profile_query_strings
  53:     0x7ffbc4beffd6 - <&[rustc_span[7d253b2ad3f54c9a]::def_id::DefId] as rustc_serialize[f7496f00b61bd66d]::serialize::Decodable<rustc_query_impl[eb123e80d0f4deec]::on_disk_cache::CacheDecoder>>::decode
  54:     0x7ffbc4b5a65f - <&[rustc_span[7d253b2ad3f54c9a]::def_id::DefId] as rustc_serialize[f7496f00b61bd66d]::serialize::Decodable<rustc_query_impl[eb123e80d0f4deec]::on_disk_cache::CacheDecoder>>::decode
  55:     0x7ffbc4bc189d - <&[rustc_span[7d253b2ad3f54c9a]::def_id::DefId] as rustc_serialize[f7496f00b61bd66d]::serialize::Decodable<rustc_query_impl[eb123e80d0f4deec]::on_disk_cache::CacheDecoder>>::decode
  56:     0x7ffbc4057cb4 - <unknown>
  57:     0x7ffbc403bd8b - <unknown>
  58:     0x7ffbc4033110 - <unknown>
  59:     0x7ffbc403cdf0 - <unknown>
  60:     0x7ffbc4068cb9 - rustc_driver[a0a607043376aa59]::args::arg_expand_all
  61:     0x7ffbc40583bd - <unknown>
  62:     0x7ffbd88eebdc - std::sys::windows::thread::Thread::new::hdf946fbfa5577747
  63:     0x7ffc315e7034 - BaseThreadInitThunk
  64:     0x7ffc31fc26a1 - RtlUserThreadStart

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.64.0 (a55dd71d5 2022-09-19) running on x86_64-pc-windows-msvc

note: compiler flags: --crate-type lib -C embed-bitcode=no -C debuginfo=2 -C incremental

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [typeck] type-checking `parse::list`
#1 [typeck_item_bodies] type-checking all item bodies
#2 [analysis] running analysis passes on this crate
end of query stack
