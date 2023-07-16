
   Compiling num-traits v0.1.43
   Compiling syn v0.14.7
thread 'main' panicked at 'specified instant was later than self', libcore\option.rs:914:5
stack backtrace:
   0:     0x7ffd61b15bd5 - <u128 as compiler_builtins::int::Int>::min_value::h9677a6134f517472
   1:     0x7ffd61b1c1ff - core::alloc::<impl core::alloc::::Opaque>::null_mut::h5057fcb0fb4b9a64
   2:     0x7ffd61b1a8b5 - std::panicking::take_hook::hec38ed21b43fe057
   3:     0x7ffd61b1a4ec - std::panicking::take_hook::hec38ed21b43fe057
   4:     0x7ffd5e06d439 - rustc::ty::structural_impls::<impl rustc::ty::context::Lift<'tcx> for rustc::middle::const_val::ErrKind<'a>>::lift_to_tcx::hcbea0cf3d5a4c41c
   5:     0x7ffd61b1b06f - std::panicking::rust_panic_with_hook::hcd28e5dfec0e3af8
   6:     0x7ffd61b1ab76 - std::panicking::begin_panic_fmt::hc5ca911ddece0631
   7:     0x7ffd61b1ab2f - rust_begin_unwind
   8:     0x7ffd61b5817c - core::panicking::panic_fmt::h4892ba3c15feb17f
   9:     0x7ffd61b69b41 - core::option::expect_failed::h3e5561a5bf3400d7
  10:     0x7ffd61b22260 - std::time::Instant::elapsed::ha0639ec21b19c6f8
  11:     0x7ffd7bf5543b - <rustc_trans_utils::trans_crate::MetadataOnlyTransCrate as rustc_trans_utils::trans_crate::TransCrate>::provide_extern::h940466a7d74a5262
  12:     0x7ffd7bf568a0 - rustc_trans_utils::symbol_names::provide::h0b93799f774cb512
  13:     0x7ffd5e086032 - rustc::ty::maps::<impl rustc::ty::maps::config::QueryConfig<'tcx> for rustc::ty::maps::queries::symbol_name<'tcx>>::compute::h434d1f456a5dee7e
  14:     0x7ffd5dbaa719 - rustc::dep_graph::graph::DepGraph::assert_ignored::hf7aea613bc733bb5
  15:     0x7ffd5ddadb85 - rustc::ty::context::tls::track_diagnostic::h66cc7851580ae713
  16:     0x7ffd5de030f3 - rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::try_mark_green_and_read::h1877c133479ca0e6
  17:     0x7ffd5dedb79e - rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::try_mark_green_and_read::h1877c133479ca0e6
  18:     0x7ffd5df1941c - rustc::ty::maps::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::symbol_name::he3fd5921efcfe7bb
  19:     0x7ffd5715b47a - <rustc_trans::builder::Builder<'a, 'tcx> as core::ops::drop::Drop>::drop::h322d11a95248e30a
  20:     0x7ffd570cebec - <rustc_trans::meth::VirtualIndex as core::fmt::Debug>::fmt::h3878f7ab587deb2c
  21:     0x7ffd5704d5ee - LLVMRustThinLTOAvailable
  22:     0x7ffd570f3062 - <rustc_trans::base::ValueIter as core::iter::iterator::Iterator>::next::h95403e2bead2716d
  23:     0x7ffd5db8d485 - rustc::dep_graph::graph::DepGraph::assert_ignored::hf7aea613bc733bb5
  24:     0x7ffd5dd7b961 - rustc::ty::context::tls::track_diagnostic::h66cc7851580ae713
  25:     0x7ffd5ddfec27 - rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::try_mark_green_and_read::h1877c133479ca0e6
  26:     0x7ffd5de8c18a - rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::try_mark_green_and_read::h1877c133479ca0e6
  27:     0x7ffd571458ec - <rustc_trans::back::linker::WasmLd as rustc_trans::back::linker::Linker>::finalize::h807558c41ba955d3
  28:     0x7ffd5db904f0 - rustc::dep_graph::graph::DepGraph::assert_ignored::hf7aea613bc733bb5
  29:     0x7ffd5ddbbf07 - rustc::ty::context::tls::track_diagnostic::h66cc7851580ae713
  30:     0x7ffd5de0ab73 - rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::try_mark_green_and_read::h1877c133479ca0e6
  31:     0x7ffd5ded33bc - rustc::ty::maps::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::try_mark_green_and_read::h1877c133479ca0e6
  32:     0x7ffd60776b98 - <rustc_metadata::encoder::ImplVisitor<'a, 'tcx> as rustc::hir::itemlikevisit::ItemLikeVisitor<'v>>::visit_item::h6fbd8be0b12dc211
  33:     0x7ffd6088cbaa - rustc_metadata::cstore_impl::<impl rustc::middle::cstore::CrateStore for rustc_metadata::cstore::CStore>::encode_metadata::hd842f9cf7949e0d3
  34:     0x7ffd5df112fd - rustc::ty::context::TyCtxt::encode_metadata::hbb881fad4f945633
  35:     0x7ffd570ef4aa - rustc_trans::base::trans_instance::h7f44600d98746c44
  36:     0x7ffd56ff807a - <unknown>
  37:     0x7ffd570efee0 - <rustc_trans::base::ValueIter as core::iter::iterator::Iterator>::next::h95403e2bead2716d
  38:     0x7ffd5704324f - <rustc_trans::LlvmTransCrate as rustc_trans_utils::trans_crate::TransCrate>::trans_crate::he1ce52bc7ac27ad4
  39:     0x7ffd61ecfff9 - rustc_driver::driver::build_output_filenames::hfe7c8d798b282910
  40:     0x7ffd61ec2d1a - rustc_driver::driver::phase_4_translate_to_llvm::h6de8823c2b178633
  41:     0x7ffd61f5ec1c - rustc_driver::profile::trace::write_style::h1440147d1e9aaf66
  42:     0x7ffd61f59c94 - <humantime::duration::Error as std::error::Error>::cause::hda0dca1bca393eeb
  43:     0x7ffd61f1ab22 - <rustc_driver::pretty::UserIdentifiedItem as core::fmt::Debug>::fmt::h299247c872c03d2d
  44:     0x7ffd61e55788 - <unknown>
  45:     0x7ffd61eb7a1c - rustc_driver::driver::compile_input::h01ffbfc578755e48
  46:     0x7ffd61f65df3 - rustc_driver::run_compiler::hb5db22dd9e7f4b75
  47:     0x7ffd61ed7b5e - rustc_driver::driver::build_output_filenames::hfe7c8d798b282910
  48:     0x7ffd61e5ee20 - <unknown>
  49:     0x7ffd61ed4f6e - rustc_driver::driver::build_output_filenames::hfe7c8d798b282910
  50:     0x7ffd61b536b1 - _rust_maybe_catch_panic
  51:     0x7ffd61f611d6 - rustc_driver::profile::trace::write_style::h1440147d1e9aaf66
  52:     0x7ffd61f750fd - rustc_driver::main::h02399fb94c78baca
  53:     0x7ff691921005 - <unknown>
  54:     0x7ffd61b1aa76 - std::panicking::update_panic_count::h34d4460f41db263e
  55:     0x7ffd61b536b1 - _rust_maybe_catch_panic
  56:     0x7ffd61b156c5 - std::rt::lang_start_internal::h3f38d7833c8c1ec2
  57:     0x7ff691921079 - <unknown>
  58:     0x7ff691921298 - <unknown>
  59:     0x7ffd89ce8363 - BaseThreadInitThunk
  60:     0x7ffd8c6070d0 - RtlUserThreadStart
query stack during panic:
#0 [symbol_name] computing the symbol for `<u8 as roots::Roots>::sqrt::<i8, extern "rust-call" fn((u8,)) -> u8, &&u8>::{{closure}}`
#1 [collect_and_partition_translation_items] collect_and_partition_translation_items
#2 [exported_symbols] exported_symbols
end of query stack

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.27.1 (5f2b325f6 2018-07-07) running on x86_64-pc-windows-msvc

note: compiler flags: -C debuginfo=2 --crate-type lib

note: some of the compiler flags provided by cargo are hidden

error: Could not compile `num-integer`.
