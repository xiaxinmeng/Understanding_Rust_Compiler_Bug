
thread 'rustc' panicked at 'failed to lookup `SourceFile` in new context', compiler\rustc_query_impl\src\on_disk_cache.rs:500:22
stack backtrace:
   0:     0x7ffbba9b9c4f - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hb627657c32fb20ba
   1:     0x7ffbba9e47fa - core::fmt::write::h54eb8d7da15814eb
   2:     0x7ffbba9ab738 - <std::io::IoSliceMut as core::fmt::Debug>::fmt::hc3eb0e97c5326934
   3:     0x7ffbba9bd3a6 - std::panicking::take_hook::hf5b13a50efed9e15
   4:     0x7ffbba9bce89 - std::panicking::take_hook::hf5b13a50efed9e15
   5:     0x7ffb9cc999f6 - <rustc_typeck[981eab1a4c12faaf]::coherence::inherent_impls_overlap::InherentOverlapChecker as rustc_hir[a7510ee90ae70616]::itemlikevisit::ItemLikeVisitor>::visit_impl_item
   6:     0x7ffbba9bdc09 - std::panicking::rust_panic_with_hook::hfaaf6cbad4a745da
   7:     0x7ffbba9bd75b - rust_begin_unwind
   8:     0x7ffbba9ba577 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hb627657c32fb20ba
   9:     0x7ffbba9bd6b9 - rust_begin_unwind
  10:     0x7ffbbaa18a20 - core::panicking::panic_fmt::h94b2d3b80b2c8df8
  11:     0x7ffbba9e1410 - <core::panic::panic_info::PanicInfo as core::fmt::Display>::fmt::h515aead6be4ecac0
  12:     0x7ffbbaa1891b - core::option::expect_failed::h44068138c8379b84
  13:     0x7ffba0468693 - <rustc_query_impl[d9ad865d4dbd08ba]::on_disk_cache::OnDiskCache>::load_side_effects
  14:     0x7ffba05c9957 - <rustc_span[98c5fae7be2fedaa]::span_encoding::Span as rustc_serialize[656dd67819cd77eb]::serialize::Decodable<rustc_query_impl[d9ad865d4dbd08ba]::on_disk_cache::CacheDecoder>>::decode
  15:     0x7ffba054e6b6 - <&[rustc_ast[f68e92c20283ad0]::ast::InlineAsmTemplatePiece] as rustc_serialize[656dd67819cd77eb]::serialize::Decodable<rustc_query_impl[d9ad865d4dbd08ba]::on_disk_cache::CacheDecoder>>::decode
  16:     0x7ffba0457190 - rustc_query_impl[d9ad865d4dbd08ba]::query_callbacks::diagnostic_hir_wf_check
  17:     0x7ffba0418ae3 - <rustc_query_impl[d9ad865d4dbd08ba]::Queries as rustc_middle[1767ea85553f7985]::ty::query::QueryEngine>::try_mark_green
  18:     0x7ffba03adcc8 - <rustc_mir_dataflow[3ff2d8458acef131]::impls::storage_liveness::MaybeRequiresStorage as rustc_mir_dataflow[3ff2d8458acef131]::framework::AnalysisDomain>::initialize_start_block
  19:     0x7ffba055b1b9 - <&[rustc_ast[f68e92c20283ad0]::ast::InlineAsmTemplatePiece] as rustc_serialize[656dd67819cd77eb]::serialize::Decodable<rustc_query_impl[d9ad865d4dbd08ba]::on_disk_cache::CacheDecoder>>::decode
  20:     0x7ffba0468844 - <rustc_query_impl[d9ad865d4dbd08ba]::on_disk_cache::OnDiskCache>::load_side_effects
  21:     0x7ffba055b2a8 - <&[rustc_ast[f68e92c20283ad0]::ast::InlineAsmTemplatePiece] as rustc_serialize[656dd67819cd77eb]::serialize::Decodable<rustc_query_impl[d9ad865d4dbd08ba]::on_disk_cache::CacheDecoder>>::decode
  22:     0x7ffba04548c3 - rustc_query_impl[d9ad865d4dbd08ba]::query_callbacks::diagnostic_hir_wf_check
  23:     0x7ffba04056e0 - <rustc_query_impl[d9ad865d4dbd08ba]::Queries as rustc_middle[1767ea85553f7985]::ty::query::QueryEngine>::try_mark_green
  24:     0x7ffba04672a7 - <rustc_query_impl[d9ad865d4dbd08ba]::on_disk_cache::OnDiskCache>::load_side_effects
  25:     0x7ffba030072b - <rustc_mir_dataflow[3ff2d8458acef131]::impls::storage_liveness::MaybeRequiresStorage as rustc_mir_dataflow[3ff2d8458acef131]::framework::AnalysisDomain>::initialize_start_block
  26:     0x7ffba0389422 - <rustc_mir_dataflow[3ff2d8458acef131]::impls::storage_liveness::MaybeRequiresStorage as rustc_mir_dataflow[3ff2d8458acef131]::framework::AnalysisDomain>::initialize_start_block
  27:     0x7ffba024224d - <rustc_mir_dataflow[3ff2d8458acef131]::impls::storage_liveness::MaybeRequiresStorage as rustc_mir_dataflow[3ff2d8458acef131]::framework::AnalysisDomain>::initialize_start_block
  28:     0x7ffba0363245 - <rustc_mir_dataflow[3ff2d8458acef131]::impls::storage_liveness::MaybeRequiresStorage as rustc_mir_dataflow[3ff2d8458acef131]::framework::AnalysisDomain>::initialize_start_block
  29:     0x7ffba11bce09 - <rustc_middle[1767ea85553f7985]::ty::context::TyCtxt>::typeck_body
  30:     0x7ffb9faa2c86 - <rustc_ast_lowering[70a0538daa5dcc6b]::index::NodeCollector as rustc_hir[a7510ee90ae70616]::intravisit::Visitor>::visit_impl_item_ref
  31:     0x7ffb9fac2591 - <rustc_ast_lowering[70a0538daa5dcc6b]::index::NodeCollector as rustc_hir[a7510ee90ae70616]::intravisit::Visitor>::visit_impl_item_ref
  32:     0x7ffb9fb20bbe - rustc_passes[3916d9b1c55b1c4f]::dead::check_crate
  33:     0x7ffb9cec39a7 - <rustc_interface[ac5c56627e25a53e]::passes::boxed_resolver::BoxedResolver>::to_resolver_outputs
  34:     0x7ffb9cec5088 - <rustc_interface[ac5c56627e25a53e]::passes::boxed_resolver::BoxedResolver>::to_resolver_outputs
  35:     0x7ffb9ceb398e - <rustc_interface[ac5c56627e25a53e]::passes::boxed_resolver::BoxedResolver>::to_resolver_outputs
  36:     0x7ffb9ce01a67 - rustc_interface[ac5c56627e25a53e]::passes::analysis
  37:     0x7ffba03d8ceb - <rustc_query_impl[d9ad865d4dbd08ba]::Queries as rustc_middle[1767ea85553f7985]::ty::query::QueryEngine>::try_mark_green
  38:     0x7ffba052025e - <&[rustc_ast[f68e92c20283ad0]::ast::InlineAsmTemplatePiece] as rustc_serialize[656dd67819cd77eb]::serialize::Decodable<rustc_query_impl[d9ad865d4dbd08ba]::on_disk_cache::CacheDecoder>>::decode
  39:     0x7ffba0396601 - <rustc_mir_dataflow[3ff2d8458acef131]::impls::storage_liveness::MaybeRequiresStorage as rustc_mir_dataflow[3ff2d8458acef131]::framework::AnalysisDomain>::initialize_start_block
  40:     0x7ffba02c7a39 - <rustc_mir_dataflow[3ff2d8458acef131]::impls::storage_liveness::MaybeRequiresStorage as rustc_mir_dataflow[3ff2d8458acef131]::framework::AnalysisDomain>::initialize_start_block
  41:     0x7ffba0363692 - <rustc_mir_dataflow[3ff2d8458acef131]::impls::storage_liveness::MaybeRequiresStorage as rustc_mir_dataflow[3ff2d8458acef131]::framework::AnalysisDomain>::initialize_start_block
  42:     0x7ffb9cd1bf50 - <rustc_driver[2fa120ec4306dc43]::args::Error as core[714c049513c47125]::fmt::Debug>::fmt
  43:     0x7ffb9ccc08b5 - <rustc_middle[1767ea85553f7985]::ty::SymbolName as core[714c049513c47125]::fmt::Display>::fmt
  44:     0x7ffb9ccb4a05 - rustc_driver[2fa120ec4306dc43]::pretty::print_after_hir_lowering
  45:     0x7ffb9ccdcf91 - <rustc_middle[1767ea85553f7985]::ty::SymbolName as core[714c049513c47125]::fmt::Display>::fmt
  46:     0x7ffb9ccbd8d3 - rustc_driver[2fa120ec4306dc43]::pretty::print_after_hir_lowering
  47:     0x7ffb9cd38018 - <rustc_driver[2fa120ec4306dc43]::args::Error as core[714c049513c47125]::fmt::Debug>::fmt
  48:     0x7ffbba9cb38c - std::sys::windows::thread::Thread::new::h675ce0e4b2379fd4
  49:     0x7ffc884054e0 - BaseThreadInitThunk
  50:     0x7ffc896c485b - RtlUserThreadStart

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.59.0-nightly (23f69235a 2021-12-20) running on x86_64-pc-windows-msvc

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental -C target-feature=+crt-static --crate-type cdylib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [typeck] type-checking `hax::init::__process_voice_data`
#1 [analysis] running analysis passes on this crate
end of query stack
