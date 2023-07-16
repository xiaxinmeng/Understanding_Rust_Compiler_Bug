
thread 'rustc' panicked at 'Box<dyn Any>', compiler\rustc_errors\src\lib.rs:1034:9
stack backtrace:
   0:     0x7fffef6f72af - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hf8f9ccaaf737268c
   1:     0x7fffef7202ea - core::fmt::write::h7e004fee938dc3bd
   2:     0x7fffef6eaa48 - <std::io::IoSlice as core::fmt::Debug>::fmt::h5e8f50613d775ee8
   3:     0x7fffef6faed6 - std::panicking::take_hook::hb09930640d1f28d1
   4:     0x7fffef6fa9b9 - std::panicking::take_hook::hb09930640d1f28d1
   5:     0x7fffe65b210e - <sha2::sha512::Sha512 as std::io::Write>::flush::h720d3aa9a35641a4
   6:     0x7fffef6fb7d0 - std::panicking::rust_panic_with_hook::ha77067ca1c3aa04d
   7:     0x7fffea84dff0 - rustc_errors::diagnostic_builder::DiagnosticBuilder::code::h17316800dfe2f643
   8:     0x7fffea84dfc9 - rustc_errors::diagnostic_builder::DiagnosticBuilder::code::h17316800dfe2f643
   9:     0x7fffeab0c6a1 - rustc_query_system::query::job::report_cycle::h37e4bf0df2043575
  10:     0x7fffea880290 - <rustc_errors::json::Diagnostic::from_errors_diagnostic::BufWriter as std::io::Write>::flush::h40a088e8c152c98f
  11:     0x7fffea886b15 - rustc_errors::HandlerInner::emit_diagnostic::h605f2fc202d4eea0
  12:     0x7fffea884622 - rustc_errors::Handler::bug::ha197644133cfa24e
  13:     0x7fffea7426d7 - rustc_middle::ty::walk::<impl rustc_middle::ty::subst::GenericArg>::walk_shallow::h2e0646470201bd9e
  14:     0x7fffea741fa0 - rustc_middle::ty::walk::<impl rustc_middle::ty::subst::GenericArg>::walk_shallow::h2e0646470201bd9e
  15:     0x7fffea741f48 - rustc_middle::ty::walk::<impl rustc_middle::ty::subst::GenericArg>::walk_shallow::h2e0646470201bd9e
  16:     0x7fffea742609 - rustc_middle::ty::walk::<impl rustc_middle::ty::subst::GenericArg>::walk_shallow::h2e0646470201bd9e
  17:     0x7fffeab08be7 - rustc_middle::util::bug::bug_fmt::h66e69068a677c29e
  18:     0x7fffea724198 - rustc_middle::ich::impls_ty::<impl rustc_data_structures::stable_hasher::HashStable<rustc_middle::ich::hcx::StableHashingContext> for rustc_middle::ty::sty::RegionKind>::hash_stable::h06caf495d24d485b
  19:     0x7fffea6e26e4 - <rustc_middle::ty::TyS as rustc_data_structures::stable_hasher::HashStable<rustc_middle::ich::hcx::StableHashingContext>>::hash_stable::h8bcc894d8e8fafcd
  20:     0x7fffea57b5df - <rustc_infer::traits::project::ProjectionCacheEntry as core::fmt::Debug>::fmt::h0f137517fe2c6105
  21:     0x7fffea72466e - rustc_middle::ich::impls_ty::<impl rustc_data_structures::stable_hasher::HashStable<rustc_middle::ich::hcx::StableHashingContext> for rustc_middle::ty::sty::RegionKind>::hash_stable::h06caf495d24d485b
  22:     0x7fffe99a2922 - <aho_corasick::packed::rabinkarp::RabinKarp as core::fmt::Debug>::fmt::hde8c8300ff16d93a
  23:     0x7fffe97ef07c - <aho_corasick::packed::rabinkarp::RabinKarp as core::fmt::Debug>::fmt::hde8c8300ff16d93a
  24:     0x7fffe9a7603b - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::try_mark_green::h00bfb97540d859ae
  25:     0x7fffea347d84 - <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::error_reporting::InferCtxtExt>::report_selection_error::h7556b0cf2e4f5cc4
  26:     0x7fffea3517fd - <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::error_reporting::InferCtxtPrivExt>::report_fulfillment_error::h763b7d46de7cfc90
  27:     0x7fffea346071 - <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::error_reporting::InferCtxtExt>::report_fulfillment_errors::ha2f2c168eee417d3
  28:     0x7fffe8c5da5d - rustc_typeck::check::fn_ctxt::_impl::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::field_ty::h3cbfaf9ed5d45b96
  29:     0x7fffe8d57501 - <rustc_typeck::check::regionck::RegionCtxt as rustc_hir::intravisit::Visitor>::visit_expr::h02adde056c81ec89
  30:     0x7fffe8cf7c7f - <rustc_typeck::check::method::probe::ProbeScope as core::fmt::Debug>::fmt::hb5b3726d1521d739
  31:     0x7fffe8e3ca40 - rustc_typeck::check::check::check_item_type::ha94a19386e8638d1
  32:     0x7fffe9aee0a6 - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::try_mark_green::h00bfb97540d859ae
  33:     0x7fffe9ac3059 - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::try_mark_green::h00bfb97540d859ae
  34:     0x7fffe9bccdcf - rustc_query_impl::on_disk_cache::__ty_decoder_impl::<impl rustc_serialize::serialize::Decoder for rustc_query_impl::on_disk_cache::CacheDecoder>::error::h1345e6f615df1d2d
  35:     0x7fffe98d5971 - <aho_corasick::packed::rabinkarp::RabinKarp as core::fmt::Debug>::fmt::hde8c8300ff16d93a
  36:     0x7fffe985580d - <aho_corasick::packed::rabinkarp::RabinKarp as core::fmt::Debug>::fmt::hde8c8300ff16d93a
  37:     0x7fffe9a71391 - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::try_mark_green::h00bfb97540d859ae
  38:     0x7fffe8ce337c - <rustc_typeck::check::wfcheck::CheckTypeWellFormedVisitor as rustc_hir::intravisit::Visitor>::visit_impl_item::h9e73050a69cf68be
  39:     0x7fffe8cc5dd3 - rustc_typeck::check::method::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::resolve_fully_qualified_call::h0e4aa83e76db2063
  40:     0x7fffe8dd715a - <rustc_typeck::constrained_generic_params::Parameter as core::fmt::Debug>::fmt::h3ea920eb9c7a4ad7
  41:     0x7fffe8d24175 - <rustc_typeck::outlives::explicit::ExplicitPredicatesMap as core::fmt::Debug>::fmt::he1d4e3e051288eac
  42:     0x7fffe8cf4fb1 - rustc_typeck::check_crate::h6bf0390329652898
  43:     0x7fffe66facd2 - rustc_interface::passes::analysis::h3a1cc36134a75432
  44:     0x7fffe9aef70b - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::try_mark_green::h00bfb97540d859ae
  45:     0x7fffe9aae79d - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::try_mark_green::h00bfb97540d859ae
  46:     0x7fffe9bcf745 - rustc_query_impl::on_disk_cache::__ty_decoder_impl::<impl rustc_serialize::serialize::Decoder for rustc_query_impl::on_disk_cache::CacheDecoder>::error::h1345e6f615df1d2d
  47:     0x7fffe98bdf63 - <aho_corasick::packed::rabinkarp::RabinKarp as core::fmt::Debug>::fmt::hde8c8300ff16d93a
  48:     0x7fffe97aa9dc - <aho_corasick::packed::rabinkarp::RabinKarp as core::fmt::Debug>::fmt::hde8c8300ff16d93a
  49:     0x7fffe9a6852c - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::try_mark_green::h00bfb97540d859ae
  50:     0x7fffe6606f01 - <rustc_driver::args::Error as core::fmt::Debug>::fmt::h5df0c4d8fb3d3c5e
  51:     0x7fffe65cbb35 - chalk_engine::TimeStamp::increment::h8dcb95b70021502b
  52:     0x7fffe6608759 - <rustc_driver::args::Error as core::fmt::Debug>::fmt::h5df0c4d8fb3d3c5e
  53:     0x7fffe65d7b20 - rustc_driver::pretty::print_after_hir_lowering::h5170af9d47465e28
  54:     0x7fffe65c6d7d - <rustc_driver::Compilation as core::fmt::Debug>::fmt::h64cba24964385564
  55:     0x7fffef70931c - std::sys::windows::thread::Thread::new::h0a6e657a03ae2a48
  56:     0x7ff873fc7034 - BaseThreadInitThunk
  57:     0x7ff875f22651 - RtlUserThreadStart

note: the compiler unexpectedly panicked. this is a bug.
