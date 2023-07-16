
error: internal compiler error: compiler\rustc_infer\src\infer\region_constraints\mod.rs:568:17: cannot relate bound region: '_#0r <= ReLateBound(DebruijnIndex(0), BoundRegion { var: 1, kind: BrNamed(DefId(0:351 ~ e24_epd[3467]::serial_1::serial_2::{impl#0}::split_method::{opaque#0}::'_#1), '_) })

thread 'rustc' panicked at 'Box<dyn Any>', /rustc/9eb3afe9ebe9c7d2b84b71002d44f4a0edac95e0\compiler\rustc_errors\src\lib.rs:987:33
stack backtrace:
   0:     0x7ff8520d9d02 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hbb6eec21cb8e3d1b
   1:     0x7ff85211570b - core::fmt::write::h3f34d6e08b5a11e8
   2:     0x7ff8520ccdfa - <std::io::IoSlice as core::fmt::Debug>::fmt::h1d8b4e60e4c9e548
   3:     0x7ff8520d9a4b - std::sys::common::alloc::realloc_fallback::h258dcd65da5985d2
   4:     0x7ff8520dd3f9 - std::panicking::default_hook::h90b70b966050cb60
   5:     0x7ff8520dd07b - std::panicking::default_hook::h90b70b966050cb60
   6:     0x7ff82e6a9425 - rustc_driver[fbeb20e0897b27eb]::describe_lints
   7:     0x7ff8520ddd5f - std::panicking::rust_panic_with_hook::h1d50ae3462f62f01
   8:     0x7ff83096a703 - <<rustc_infer[e1f2f17455595243]::infer::error_reporting::TypeErrCtxt>::construct_generic_bound_failure::SubOrigin as core[15ae0b8045847941]::fmt::Debug>::fmt
   9:     0x7ff830968c19 - <<rustc_infer[e1f2f17455595243]::infer::error_reporting::TypeErrCtxt>::construct_generic_bound_failure::SubOrigin as core[15ae0b8045847941]::fmt::Debug>::fmt
  10:     0x7ff830968bb9 - <<rustc_infer[e1f2f17455595243]::infer::error_reporting::TypeErrCtxt>::construct_generic_bound_failure::SubOrigin as core[15ae0b8045847941]::fmt::Debug>::fmt
  11:     0x7ff830968ba9 - <<rustc_infer[e1f2f17455595243]::infer::error_reporting::TypeErrCtxt>::construct_generic_bound_failure::SubOrigin as core[15ae0b8045847941]::fmt::Debug>::fmt
  12:     0x7ff830968220 - <<rustc_infer[e1f2f17455595243]::infer::error_reporting::TypeErrCtxt>::construct_generic_bound_failure::SubOrigin as core[15ae0b8045847941]::fmt::Debug>::fmt
  13:     0x7ff830968079 - <<rustc_infer[e1f2f17455595243]::infer::error_reporting::TypeErrCtxt>::construct_generic_bound_failure::SubOrigin as core[15ae0b8045847941]::fmt::Debug>::fmt
  14:     0x7ff8309704c9 - <rustc_infer[e1f2f17455595243]::infer::opaque_types::table::OpaqueTypeStorage as core[15ae0b8045847941]::fmt::Debug>::fmt
  15:     0x7ff83097052d - <rustc_infer[e1f2f17455595243]::infer::opaque_types::table::OpaqueTypeStorage as core[15ae0b8045847941]::fmt::Debug>::fmt
  16:     0x7ff83096f73a - <rustc_infer[e1f2f17455595243]::infer::opaque_types::table::OpaqueTypeStorage as core[15ae0b8045847941]::fmt::Debug>::fmt
  17:     0x7ff83096f678 - <rustc_infer[e1f2f17455595243]::infer::opaque_types::table::OpaqueTypeStorage as core[15ae0b8045847941]::fmt::Debug>::fmt
  18:     0x7ff83096f636 - <rustc_infer[e1f2f17455595243]::infer::opaque_types::table::OpaqueTypeStorage as core[15ae0b8045847941]::fmt::Debug>::fmt
  19:     0x7ff82e4d776a - <rustc_infer[e1f2f17455595243]::infer::combine::ConstInferUnifier as rustc_middle[1d4803ea9c08b419]::ty::relate::TypeRelation>::consts
  20:     0x7ff82e4e61c7 - <rustc_middle[1d4803ea9c08b419]::ty::sty::AliasTy as rustc_infer[e1f2f17455595243]::infer::at::ToTrace>::to_trace
  21:     0x7ff82e4a1909 - <rustc_infer[e1f2f17455595243]::infer::equate::Equate as rustc_middle[1d4803ea9c08b419]::ty::relate::TypeRelation>::tys
  22:     0x7ff82cc6d78a - <rustc_middle[1d4803ea9c08b419]::traits::specialization_graph::Children as rustc_trait_selection[5f0f54a03cd0c301]::traits::specialize::specialization_graph::ChildrenExt>::insert_blindly
  23:     0x7ff82cc5b10b - <rustc_trait_selection[5f0f54a03cd0c301]::traits::project::AssocTypeNormalizer as rustc_middle[1d4803ea9c08b419]::ty::fold::TypeFolder>::fold_const
  24:     0x7ff8308b9218 - <rustc_trait_selection[5f0f54a03cd0c301]::solve::search_graph::StackDepth as core[15ae0b8045847941]::fmt::Debug>::fmt
  25:     0x7ff830857a09 - <rustc_trait_selection[5f0f54a03cd0c301]::traits::error_reporting::method_chain::CollectAllMismatches as rustc_middle[1d4803ea9c08b419]::ty::relate::TypeRelation>::consts
  26:     0x7ff8308d16c3 - <rustc_infer[e1f2f17455595243]::infer::error_reporting::TypeErrCtxt as rustc_trait_selection[5f0f54a03cd0c301]::traits::error_reporting::suggestions::TypeErrCtxtExt>::report_closure_arg_mismatch
  27:     0x7ff8308e4bdb - <rustc_infer[e1f2f17455595243]::infer::error_reporting::TypeErrCtxt as rustc_trait_selection[5f0f54a03cd0c301]::traits::error_reporting::TypeErrCtxtExt>::report_selection_error     
  28:     0x7ff8308f1805 - <rustc_infer[e1f2f17455595243]::infer::error_reporting::TypeErrCtxt as rustc_trait_selection[5f0f54a03cd0c301]::traits::error_reporting::InferCtxtPrivExt>::report_fulfillment_error 
  29:     0x7ff8308e1569 - <rustc_infer[e1f2f17455595243]::infer::error_reporting::TypeErrCtxt as rustc_trait_selection[5f0f54a03cd0c301]::traits::error_reporting::TypeErrCtxtExt>::report_fulfillment_errors  
  30:     0x7ff82d7f646e - <rustc_hir_typeck[527c1058ba999897]::fn_ctxt::FnCtxt>::demand_coerce
  31:     0x7ff82d7f7537 - <rustc_hir_typeck[527c1058ba999897]::fn_ctxt::FnCtxt>::demand_coerce
  32:     0x7ff82d83a622 - <rustc_hir_typeck[527c1058ba999897]::fn_ctxt::FnCtxt>::check_struct_path
  33:     0x7ff82d7f7fbf - <rustc_hir_typeck[527c1058ba999897]::fn_ctxt::FnCtxt>::demand_coerce
  34:     0x7ff82d80d7c0 - <rustc_hir_typeck[527c1058ba999897]::fn_ctxt::FnCtxt>::demand_coerce
  35:     0x7ff82d8ac29d - <rustc_hir_typeck[527c1058ba999897]::writeback::WritebackCx as rustc_hir[38532e4666ac8dc2]::intravisit::Visitor>::visit_ty
  36:     0x7ff82d7befcc - <rustc_hir_typeck[527c1058ba999897]::inherited::Inherited as core[15ae0b8045847941]::ops::deref::Deref>::deref
  37:     0x7ff82c88e669 - <rustc_query_impl[22fc8f5fe5921e7d]::on_disk_cache::CacheEncoder as rustc_type_ir[76501457ae8f3cde]::codec::TyEncoder>::encode_alloc_id
  38:     0x7ff82c8f47c5 - <&[(rustc_middle[1d4803ea9c08b419]::ty::Predicate, rustc_span[31f727a51b7d57bd]::span_encoding::Span)] as rustc_serialize[637af69557f69e4e]::serialize::Decodable<rustc_query_impl[22fc8f5fe5921e7d]::on_disk_cache::CacheDecoder>>::decode
  39:     0x7ff82e0808ad - <&[(rustc_middle[1d4803ea9c08b419]::ty::Clause, rustc_span[31f727a51b7d57bd]::span_encoding::Span)] as rustc_serialize[637af69557f69e4e]::serialize::Decodable<rustc_query_impl[22fc8f5fe5921e7d]::on_disk_cache::CacheDecoder>>::decode
  40:     0x7ff82df57093 - <rustc_query_impl[22fc8f5fe5921e7d]::Queries as rustc_middle[1d4803ea9c08b419]::ty::query::QueryEngine>::as_any
  41:     0x7ff82e5ad3f5 - <rustc_middle[1d4803ea9c08b419]::ty::context::TyCtxt>::typeck_opt_const_arg
  42:     0x7ff82dc30cc9 - <rustc_passes[bd5f324aecd13b61]::liveness::Liveness as rustc_hir[38532e4666ac8dc2]::intravisit::Visitor>::visit_arm
  43:     0x7ff82c88ce23 - <rustc_query_impl[22fc8f5fe5921e7d]::on_disk_cache::CacheEncoder as rustc_type_ir[76501457ae8f3cde]::codec::TyEncoder>::encode_alloc_id
  44:     0x7ff82c8e987d - <&[(rustc_middle[1d4803ea9c08b419]::ty::Predicate, rustc_span[31f727a51b7d57bd]::span_encoding::Span)] as rustc_serialize[637af69557f69e4e]::serialize::Decodable<rustc_query_impl[22fc8f5fe5921e7d]::on_disk_cache::CacheDecoder>>::decode
  45:     0x7ff82e088fb0 - <&[(rustc_middle[1d4803ea9c08b419]::ty::Clause, rustc_span[31f727a51b7d57bd]::span_encoding::Span)] as rustc_serialize[637af69557f69e4e]::serialize::Decodable<rustc_query_impl[22fc8f5fe5921e7d]::on_disk_cache::CacheDecoder>>::decode
  46:     0x7ff82df535aa - <rustc_query_impl[22fc8f5fe5921e7d]::Queries as rustc_middle[1d4803ea9c08b419]::ty::query::QueryEngine>::as_any
  47:     0x7ff82dc6103e - <rustc_mir_build[edc916ca84483e0b]::thir::pattern::check_match::MatchVisitor as rustc_hir[38532e4666ac8dc2]::intravisit::Visitor>::visit_param
  48:     0x7ff82c88cf93 - <rustc_query_impl[22fc8f5fe5921e7d]::on_disk_cache::CacheEncoder as rustc_type_ir[76501457ae8f3cde]::codec::TyEncoder>::encode_alloc_id
  49:     0x7ff82c8e9ed0 - <&[(rustc_middle[1d4803ea9c08b419]::ty::Predicate, rustc_span[31f727a51b7d57bd]::span_encoding::Span)] as rustc_serialize[637af69557f69e4e]::serialize::Decodable<rustc_query_impl[22fc8f5fe5921e7d]::on_disk_cache::CacheDecoder>>::decode
  50:     0x7ff82e0856b2 - <&[(rustc_middle[1d4803ea9c08b419]::ty::Clause, rustc_span[31f727a51b7d57bd]::span_encoding::Span)] as rustc_serialize[637af69557f69e4e]::serialize::Decodable<rustc_query_impl[22fc8f5fe5921e7d]::on_disk_cache::CacheDecoder>>::decode
  51:     0x7ff82df537f8 - <rustc_query_impl[22fc8f5fe5921e7d]::Queries as rustc_middle[1d4803ea9c08b419]::ty::query::QueryEngine>::as_any
  52:     0x7ff82da1c9f3 - <rustc_mir_transform[146ad2592ddb67a2]::check_unsafety::UnsafetyChecker as rustc_middle[1d4803ea9c08b419]::mir::visit::Visitor>::visit_place
  53:     0x7ff82da10578 - <rustc_mir_transform[146ad2592ddb67a2]::deduce_param_attrs::DeduceReadOnly as rustc_middle[1d4803ea9c08b419]::mir::visit::Visitor>::visit_terminator
  54:     0x7ff82c88e669 - <rustc_query_impl[22fc8f5fe5921e7d]::on_disk_cache::CacheEncoder as rustc_type_ir[76501457ae8f3cde]::codec::TyEncoder>::encode_alloc_id
  55:     0x7ff82c8f47c5 - <&[(rustc_middle[1d4803ea9c08b419]::ty::Predicate, rustc_span[31f727a51b7d57bd]::span_encoding::Span)] as rustc_serialize[637af69557f69e4e]::serialize::Decodable<rustc_query_impl[22fc8f5fe5921e7d]::on_disk_cache::CacheDecoder>>::decode
  56:     0x7ff82e05bafd - <&[(rustc_middle[1d4803ea9c08b419]::ty::Clause, rustc_span[31f727a51b7d57bd]::span_encoding::Span)] as rustc_serialize[637af69557f69e4e]::serialize::Decodable<rustc_query_impl[22fc8f5fe5921e7d]::on_disk_cache::CacheDecoder>>::decode
  57:     0x7ff82df56ae3 - <rustc_query_impl[22fc8f5fe5921e7d]::Queries as rustc_middle[1d4803ea9c08b419]::ty::query::QueryEngine>::as_any
  58:     0x7ff82d9e110a - <rustc_mir_transform[146ad2592ddb67a2]::unreachable_prop::UnreachablePropagation as rustc_middle[1d4803ea9c08b419]::mir::MirPass>::run_pass
  59:     0x7ff82c88cf93 - <rustc_query_impl[22fc8f5fe5921e7d]::on_disk_cache::CacheEncoder as rustc_type_ir[76501457ae8f3cde]::codec::TyEncoder>::encode_alloc_id
  60:     0x7ff82c8e9ed0 - <&[(rustc_middle[1d4803ea9c08b419]::ty::Predicate, rustc_span[31f727a51b7d57bd]::span_encoding::Span)] as rustc_serialize[637af69557f69e4e]::serialize::Decodable<rustc_query_impl[22fc8f5fe5921e7d]::on_disk_cache::CacheDecoder>>::decode
  61:     0x7ff82e08733f - <&[(rustc_middle[1d4803ea9c08b419]::ty::Clause, rustc_span[31f727a51b7d57bd]::span_encoding::Span)] as rustc_serialize[637af69557f69e4e]::serialize::Decodable<rustc_query_impl[22fc8f5fe5921e7d]::on_disk_cache::CacheDecoder>>::decode
  62:     0x7ff82df53908 - <rustc_query_impl[22fc8f5fe5921e7d]::Queries as rustc_middle[1d4803ea9c08b419]::ty::query::QueryEngine>::as_any
  63:     0x7ff82d9e385a - <rustc_mir_transform[146ad2592ddb67a2]::unreachable_prop::UnreachablePropagation as rustc_middle[1d4803ea9c08b419]::mir::MirPass>::run_pass
  64:     0x7ff82c88d103 - <rustc_query_impl[22fc8f5fe5921e7d]::on_disk_cache::CacheEncoder as rustc_type_ir[76501457ae8f3cde]::codec::TyEncoder>::encode_alloc_id
  65:     0x7ff82c8ea57d - <&[(rustc_middle[1d4803ea9c08b419]::ty::Predicate, rustc_span[31f727a51b7d57bd]::span_encoding::Span)] as rustc_serialize[637af69557f69e4e]::serialize::Decodable<rustc_query_impl[22fc8f5fe5921e7d]::on_disk_cache::CacheDecoder>>::decode
  66:     0x7ff82e0267b4 - <&[(rustc_middle[1d4803ea9c08b419]::ty::Clause, rustc_span[31f727a51b7d57bd]::span_encoding::Span)] as rustc_serialize[637af69557f69e4e]::serialize::Decodable<rustc_query_impl[22fc8f5fe5921e7d]::on_disk_cache::CacheDecoder>>::decode
  67:     0x7ff82df53d7a - <rustc_query_impl[22fc8f5fe5921e7d]::Queries as rustc_middle[1d4803ea9c08b419]::ty::query::QueryEngine>::as_any
  68:     0x7ff82dcaf0c3 - <rustc_mir_build[edc916ca84483e0b]::thir::pattern::check_match::MatchVisitor as rustc_hir[38532e4666ac8dc2]::intravisit::Visitor>::visit_param
  69:     0x7ff82dcaead8 - <rustc_mir_build[edc916ca84483e0b]::thir::pattern::check_match::MatchVisitor as rustc_hir[38532e4666ac8dc2]::intravisit::Visitor>::visit_param
  70:     0x7ff82c88e669 - <rustc_query_impl[22fc8f5fe5921e7d]::on_disk_cache::CacheEncoder as rustc_type_ir[76501457ae8f3cde]::codec::TyEncoder>::encode_alloc_id
  71:     0x7ff82c8f47c5 - <&[(rustc_middle[1d4803ea9c08b419]::ty::Predicate, rustc_span[31f727a51b7d57bd]::span_encoding::Span)] as rustc_serialize[637af69557f69e4e]::serialize::Decodable<rustc_query_impl[22fc8f5fe5921e7d]::on_disk_cache::CacheDecoder>>::decode
  72:     0x7ff82e023435 - <&[(rustc_middle[1d4803ea9c08b419]::ty::Clause, rustc_span[31f727a51b7d57bd]::span_encoding::Span)] as rustc_serialize[637af69557f69e4e]::serialize::Decodable<rustc_query_impl[22fc8f5fe5921e7d]::on_disk_cache::CacheDecoder>>::decode
  73:     0x7ff82df5726f - <rustc_query_impl[22fc8f5fe5921e7d]::Queries as rustc_middle[1d4803ea9c08b419]::ty::query::QueryEngine>::as_any
  74:     0x7ff82d99a720 - <rustc_hir_analysis[9d3badbe7d08bcd8]::collect::lifetimes::LifetimeContext as rustc_hir[38532e4666ac8dc2]::intravisit::Visitor>::visit_lifetime
  75:     0x7ff82e000297 - <&[(rustc_middle[1d4803ea9c08b419]::ty::Clause, rustc_span[31f727a51b7d57bd]::span_encoding::Span)] as rustc_serialize[637af69557f69e4e]::serialize::Decodable<rustc_query_impl[22fc8f5fe5921e7d]::on_disk_cache::CacheDecoder>>::decode
  76:     0x7ff82e0c6f10 - <&[(rustc_middle[1d4803ea9c08b419]::ty::Clause, rustc_span[31f727a51b7d57bd]::span_encoding::Span)] as rustc_serialize[637af69557f69e4e]::serialize::Decodable<rustc_query_impl[22fc8f5fe5921e7d]::on_disk_cache::CacheDecoder>>::decode
  77:     0x7ff82c5bcff8 - rustc_hir_analysis[9d3badbe7d08bcd8]::hir_wf_check::provide
  78:     0x7ff82d94a2ea - rustc_hir_analysis[9d3badbe7d08bcd8]::check::check::check_abi
  79:     0x7ff82c88d3d9 - <rustc_query_impl[22fc8f5fe5921e7d]::on_disk_cache::CacheEncoder as rustc_type_ir[76501457ae8f3cde]::codec::TyEncoder>::encode_alloc_id
  80:     0x7ff82c8f69d5 - <&[(rustc_middle[1d4803ea9c08b419]::ty::Predicate, rustc_span[31f727a51b7d57bd]::span_encoding::Span)] as rustc_serialize[637af69557f69e4e]::serialize::Decodable<rustc_query_impl[22fc8f5fe5921e7d]::on_disk_cache::CacheDecoder>>::decode
  81:     0x7ff82c9915a1 - <&[(rustc_middle[1d4803ea9c08b419]::ty::Predicate, rustc_span[31f727a51b7d57bd]::span_encoding::Span)] as rustc_serialize[637af69557f69e4e]::serialize::Decodable<rustc_query_impl[22fc8f5fe5921e7d]::on_disk_cache::CacheDecoder>>::decode
  82:     0x7ff82c862b0f - <rustc_query_impl[22fc8f5fe5921e7d]::Queries as rustc_middle[1d4803ea9c08b419]::ty::query::QueryEngine>::try_mark_green
  83:     0x7ff82c599e30 - <<dyn rustc_hir_analysis[9d3badbe7d08bcd8]::astconv::AstConv>::create_substs_for_ast_path::{closure#0}::SubstsForAstPathCtxt as rustc_hir_analysis[9d3badbe7d08bcd8]::astconv::CreateSubstsForGenericArgsCtxt>::inferred_kind
  84:     0x7ff82c591e89 - rustc_hir_analysis[9d3badbe7d08bcd8]::check_crate
  85:     0x7ff82be72295 - rustc_interface[5b253cef4d5aad89]::passes::analysis
  86:     0x7ff82c88ff1f - <rustc_query_impl[22fc8f5fe5921e7d]::on_disk_cache::CacheEncoder as rustc_type_ir[76501457ae8f3cde]::codec::TyEncoder>::encode_alloc_id
  87:     0x7ff82c90ed19 - <&[(rustc_middle[1d4803ea9c08b419]::ty::Predicate, rustc_span[31f727a51b7d57bd]::span_encoding::Span)] as rustc_serialize[637af69557f69e4e]::serialize::Decodable<rustc_query_impl[22fc8f5fe5921e7d]::on_disk_cache::CacheDecoder>>::decode
  88:     0x7ff82c9fb611 - <&[(rustc_middle[1d4803ea9c08b419]::ty::Predicate, rustc_span[31f727a51b7d57bd]::span_encoding::Span)] as rustc_serialize[637af69557f69e4e]::serialize::Decodable<rustc_query_impl[22fc8f5fe5921e7d]::on_disk_cache::CacheDecoder>>::decode
  89:     0x7ff82c86094f - <rustc_query_impl[22fc8f5fe5921e7d]::Queries as rustc_middle[1d4803ea9c08b419]::ty::query::QueryEngine>::try_mark_green
  90:     0x7ff82be1d1a9 - <rustc_middle[1d4803ea9c08b419]::ty::SymbolName as core[15ae0b8045847941]::fmt::Debug>::fmt
  91:     0x7ff82be2e7fa - rustc_driver[fbeb20e0897b27eb]::args::arg_expand_all
  92:     0x7ff82be12ac1 - <unknown>
  93:     0x7ff82be1cacb - <rustc_middle[1d4803ea9c08b419]::ty::SymbolName as core[15ae0b8045847941]::fmt::Debug>::fmt
  94:     0x7ff82be1c319 - <rustc_middle[1d4803ea9c08b419]::ty::SymbolName as core[15ae0b8045847941]::fmt::Debug>::fmt
  95:     0x7ff82be1404d - <rustc_middle[1d4803ea9c08b419]::ty::SymbolName as core[15ae0b8045847941]::fmt::Debug>::fmt
  96:     0x7ff8520f021c - std::sys::windows::thread::Thread::new::hf7aab9c244c58572
  97:     0x7ff8a7a67614 - BaseThreadInitThunk
  98:     0x7ff8a8f226a1 - RtlUserThreadStart

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.68.2 (9eb3afe9e 2023-03-27) running on x86_64-pc-windows-msvc

note: compiler flags: --crate-type bin -C embed-bitcode=no -C debuginfo=2 -C incremental=[REDACTED]

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [typeck] type-checking `serial_1::serial_2::<impl at src\serial_1\serial_2.rs:70:1: 70:35>::split_method`
#1 [thir_body] building THIR for `serial_1::serial_2::<impl at src\serial_1\serial_2.rs:70:1: 70:35>::split_method`
#2 [mir_built] building MIR for `serial_1::serial_2::<impl at src\serial_1\serial_2.rs:70:1: 70:35>::split_method`
#3 [unsafety_check_result] unsafety-checking `serial_1::serial_2::<impl at src\serial_1\serial_2.rs:70:1: 70:35>::split_method`
#4 [mir_const] preparing `serial_1::serial_2::<impl at src\serial_1\serial_2.rs:70:1: 70:35>::split_method` for borrow checking
#5 [mir_promoted] processing MIR for `serial_1::serial_2::<impl at src\serial_1\serial_2.rs:70:1: 70:35>::split_method`
#6 [mir_borrowck] borrow-checking `serial_1::serial_2::<impl at src\serial_1\serial_2.rs:70:1: 70:35>::split_method`
#7 [type_of] computing type of `serial_1::serial_2::<impl at src\serial_1\serial_2.rs:70:1: 70:35>::split_method::{opaque#0}`
#8 [check_mod_item_types] checking item types in module `serial_1::serial_2`
#9 [analysis] running analysis passes on this crate
end of query stack
