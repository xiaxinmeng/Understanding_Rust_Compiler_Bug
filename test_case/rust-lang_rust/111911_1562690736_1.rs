
Standard Error

   Compiling playground v0.0.1 (/playground)
warning: the feature `adt_const_params` is incomplete and may not be safe to use and/or cause compiler crashes
 --> src/lib.rs:1:12
  |
1 | #![feature(adt_const_params)]
  |            ^^^^^^^^^^^^^^^^
  |
  = note: [see issue #95174 <https://github.com/rust-lang/rust/issues/95174>](https://github.com/rust-lang/rust/issues/95174) for more information
  = note: `#[warn(incomplete_features)]` on by default

warning: function `foo` is never used
 --> src/lib.rs:3:4
  |
3 | fn foo<const N: &'static u8>() -> impl Sized {}
  |    ^^^
  |
  = note: `#[warn(dead_code)]` on by default

error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: opaque type with non-universal region substs
 --> src/lib.rs:3:46
  |
3 | fn foo<const N: &'static u8>() -> impl Sized {}
  |                                              ^^
  |
  = note: delayed at compiler/rustc_borrowck/src/region_infer/opaque_types.rs:94:35
             0: <rustc_errors::HandlerInner>::emit_diagnostic
             1: <rustc_session::session::Session>::delay_span_bug::<rustc_span::span_encoding::Span, &str>
             2: <rustc_borrowck::region_infer::RegionInferenceContext>::infer_opaque_types::{closure#0}::{closure#2}
             3: <rustc_middle::ty::fold::RegionFolder as rustc_type_ir::fold::FallibleTypeFolder<rustc_middle::ty::context::TyCtxt>>::try_fold_const
             4: <&rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg> as rustc_type_ir::fold::TypeFoldable<rustc_middle::ty::context::TyCtxt>>::fold_with::<rustc_middle::ty::fold::RegionFolder>
             5: <rustc_borrowck::region_infer::RegionInferenceContext>::infer_opaque_types
             6: rustc_borrowck::nll::compute_regions
             7: rustc_borrowck::do_mir_borrowck
             8: rustc_borrowck::mir_borrowck
             9: rustc_query_impl::plumbing::__rust_begin_short_backtrace::<rustc_query_impl::query_impl::mir_borrowck::dynamic_query::{closure#2}::{closure#0}, rustc_middle::query::erase::Erased<[u8; 8]>>
            10: <rustc_query_impl::query_impl::mir_borrowck::dynamic_query::{closure#2} as core::ops::function::FnOnce<(rustc_middle::ty::context::TyCtxt, rustc_span::def_id::LocalDefId)>>::call_once
            11: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::DynamicConfig<rustc_query_system::query::caches::VecCache<rustc_span::def_id::LocalDefId, rustc_middle::query::erase::Erased<[u8; 8]>>, false, false, false>, rustc_query_impl::plumbing::QueryCtxt, false>
            12: rustc_query_impl::query_impl::mir_borrowck::get_query_non_incr::__rust_end_short_backtrace
            13: rustc_hir_analysis::collect::type_of::type_of
            14: rustc_query_impl::plumbing::__rust_begin_short_backtrace::<rustc_query_impl::query_impl::type_of::dynamic_query::{closure#2}::{closure#0}, rustc_middle::query::erase::Erased<[u8; 8]>>
            15: <rustc_query_impl::query_impl::type_of::dynamic_query::{closure#2} as core::ops::function::FnOnce<(rustc_middle::ty::context::TyCtxt, rustc_span::def_id::DefId)>>::call_once
            16: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::DynamicConfig<rustc_query_system::query::caches::DefaultCache<rustc_span::def_id::DefId, rustc_middle::query::erase::Erased<[u8; 8]>>, false, false, false>, rustc_query_impl::plumbing::QueryCtxt, false>
            17: rustc_query_impl::query_impl::type_of::get_query_non_incr::__rust_end_short_backtrace
            18: rustc_middle::query::plumbing::query_get_at::<rustc_query_system::query::caches::DefaultCache<rustc_span::def_id::DefId, rustc_middle::query::erase::Erased<[u8; 8]>>>
            19: rustc_hir_analysis::check::check::check_mod_item_types
            20: rustc_query_impl::plumbing::__rust_begin_short_backtrace::<rustc_query_impl::query_impl::check_mod_item_types::dynamic_query::{closure#2}::{closure#0}, rustc_middle::query::erase::Erased<[u8; 0]>>
            21: <rustc_query_impl::query_impl::check_mod_item_types::dynamic_query::{closure#2} as core::ops::function::FnOnce<(rustc_middle::ty::context::TyCtxt, rustc_span::def_id::LocalDefId)>>::call_once
            22: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::DynamicConfig<rustc_query_system::query::caches::VecCache<rustc_span::def_id::LocalDefId, rustc_middle::query::erase::Erased<[u8; 0]>>, false, false, false>, rustc_query_impl::plumbing::QueryCtxt, false>
            23: rustc_query_impl::query_impl::check_mod_item_types::get_query_non_incr::__rust_end_short_backtrace
            24: <rustc_middle::hir::map::Map>::for_each_module::<rustc_hir_analysis::check_crate::{closure#6}::{closure#0}>
            25: <rustc_session::session::Session>::time::<(), rustc_hir_analysis::check_crate::{closure#6}>
            26: rustc_hir_analysis::check_crate
            27: rustc_interface::passes::analysis
            28: rustc_query_impl::plumbing::__rust_begin_short_backtrace::<rustc_query_impl::query_impl::analysis::dynamic_query::{closure#2}::{closure#0}, rustc_middle::query::erase::Erased<[u8; 1]>>
            29: <rustc_query_impl::query_impl::analysis::dynamic_query::{closure#2} as core::ops::function::FnOnce<(rustc_middle::ty::context::TyCtxt, ())>>::call_once
            30: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::DynamicConfig<rustc_query_system::query::caches::SingleCache<rustc_middle::query::erase::Erased<[u8; 1]>>, false, false, false>, rustc_query_impl::plumbing::QueryCtxt, false>
            31: rustc_query_impl::query_impl::analysis::get_query_non_incr::__rust_end_short_backtrace
            32: <rustc_middle::ty::context::GlobalCtxt>::enter::<rustc_driver_impl::run_compiler::{closure#1}::{closure#2}::{closure#4}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
            33: <rustc_interface::interface::Compiler>::enter::<rustc_driver_impl::run_compiler::{closure#1}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_span::ErrorGuaranteed>>
            34: std::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
            35: <<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#1} as core::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
            36: <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once
                       at /rustc/5ea3f0ae08c07472239a94ce55601e9b63eb1f45/library/alloc/src/boxed.rs:1985:9
            37: <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once
                       at /rustc/5ea3f0ae08c07472239a94ce55601e9b63eb1f45/library/alloc/src/boxed.rs:1985:9
            38: std::sys::unix::thread::Thread::new::thread_start
                       at /rustc/5ea3f0ae08c07472239a94ce55601e9b63eb1f45/library/std/src/sys/unix/thread.rs:108:17
            39: start_thread
            40: clone
          

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.71.0-nightly (5ea3f0ae0 2023-05-23) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C embed-bitcode=no -C codegen-units=1 -C debuginfo=2

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
