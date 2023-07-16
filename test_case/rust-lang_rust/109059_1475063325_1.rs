
error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: opaque type with non-universal region substs
  --> src/main.rs:10:9
   |
10 |         ()
   |         ^^
   |
   = note: delayed at    0: <rustc_errors::HandlerInner>::emit_diagnostic
              1: <rustc_errors::Handler>::delay_span_bug::<rustc_span::span_encoding::Span, &str>
              2: <rustc_borrowck::region_infer::RegionInferenceContext>::infer_opaque_types::{closure#0}::{closure#2}
              3: <rustc_middle::ty::fold::RegionFolder as rustc_type_ir::fold::FallibleTypeFolder<rustc_middle::ty::context::TyCtxt>>::try_fold_ty
              4: <rustc_middle::ty::fold::RegionFolder as rustc_type_ir::fold::FallibleTypeFolder<rustc_middle::ty::context::TyCtxt>>::try_fold_const
              5: <&rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg> as rustc_type_ir::fold::TypeFoldable<rustc_middle::ty::context::TyCtxt>>::fold_with::<rustc_middle::ty::fold::RegionFolder>
              6: <rustc_borrowck::region_infer::RegionInferenceContext>::infer_opaque_types
              7: rustc_borrowck::nll::compute_regions
              8: rustc_borrowck::do_mir_borrowck
              9: rustc_borrowck::mir_borrowck
             10: <rustc_borrowck::provide::{closure#0} as core::ops::function::FnOnce<(rustc_middle::ty::context::TyCtxt, rustc_span::def_id::LocalDefId)>>::call_once
             11: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::mir_borrowck, rustc_query_impl::plumbing::QueryCtxt>
             12: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::mir_borrowck
             13: rustc_hir_analysis::collect::type_of::type_of
             14: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::type_of, rustc_query_impl::plumbing::QueryCtxt>
             15: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::type_of
             16: rustc_hir_analysis::check::check::check_mod_item_types
             17: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::check_mod_item_types, rustc_query_impl::plumbing::QueryCtxt>
             18: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::check_mod_item_types
             19: <rustc_session::session::Session>::time::<(), rustc_hir_analysis::check_crate::{closure#6}>
             20: rustc_hir_analysis::check_crate
             21: rustc_interface::passes::analysis
             22: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::analysis, rustc_query_impl::plumbing::QueryCtxt>
             23: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::analysis
             24: <rustc_middle::ty::context::GlobalCtxt>::enter::<rustc_driver_impl::run_compiler::{closure#1}::{closure#2}::{closure#4}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
             25: <rustc_interface::interface::Compiler>::enter::<rustc_driver_impl::run_compiler::{closure#1}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_span::ErrorGuaranteed>>
             26: rustc_span::with_source_map::<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
             27: std::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>
             28: <<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_span::ErrorGuaranteed>, rustc_driver_impl::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_span::ErrorGuaranteed>>::{closure#1} as core::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
             29: <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once
                        at /rustc/511364e7874dba9649a264100407e4bffe7b5425/library/alloc/src/boxed.rs:1988:9
             30: <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once
                        at /rustc/511364e7874dba9649a264100407e4bffe7b5425/library/alloc/src/boxed.rs:1988:9
             31: std::sys::unix::thread::Thread::new::thread_start
                        at /rustc/511364e7874dba9649a264100407e4bffe7b5425/library/std/src/sys/unix/thread.rs:108:17
             32: <unknown>
             33: <unknown>
           

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md
note: rustc 1.70.0-nightly (511364e78 2023-03-16) running on x86_64-unknown-linux-gnu
note: compiler flags: --crate-type bin -C embed-bitcode=no -C debuginfo=2 -C incremental=[REDACTED]
note: some of the compiler flags provided by cargo are hidden
query stack during panic:
end of query stack
error: aborting due to 2 previous errors; 2 warnings emitted

error: could not compile `lear_rust` (bin "lear_rust") due to previous error; 2 warnings emitted
Process finished with exit code 101

