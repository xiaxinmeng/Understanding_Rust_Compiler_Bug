
thread 'main' panicked at 'did not find a cycle', librustc/ty/query/job.rs:144:9                         
stack backtrace:                                                                                         
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace                                          
             at libstd/sys/unix/backtrace/tracing/gcc_s.rs:49                                            
   1: std::sys_common::backtrace::print                                                                  
             at libstd/sys_common/backtrace.rs:71                                                        
             at libstd/sys_common/backtrace.rs:59                                                        
   2: std::panicking::default_hook::{{closure}}                                                          
             at libstd/panicking.rs:211                                                                  
   3: std::panicking::default_hook                                                                       
             at libstd/panicking.rs:227                                                                  
   4: rustc::util::common::panic_hook                                                                    
   5: std::panicking::rust_panic_with_hook                                                               
             at libstd/panicking.rs:481                                                                  
   6: std::panicking::begin_panic                                                                        
   7: rustc::ty::query::job::QueryJob::await                                                             
   8: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query           
   9: <rustc::traits::project::AssociatedTypeNormalizer<'a, 'b, 'gcx, 'tcx> as rustc::ty::fold::TypeFolder<'gcx, 'tcx>>::fold_const
  10: rustc::ty::structural_impls::<impl rustc::ty::fold::TypeFoldable<'tcx> for &'tcx rustc::ty::TyS<'tcx>>::super_fold_with
  11: <rustc::traits::project::AssociatedTypeNormalizer<'a, 'b, 'gcx, 'tcx> as rustc::ty::fold::TypeFolder<'gcx, 'tcx>>::fold_ty
  12: rustc::traits::project::opt_normalize_projection_type                                              
  13: rustc::traits::project::normalize_projection_type                                                  
  14: <rustc::traits::project::AssociatedTypeNormalizer<'a, 'b, 'gcx, 'tcx> as rustc::ty::fold::TypeFolder<'gcx, 'tcx>>::fold_ty
  15: <smallvec::SmallVec<A> as core::iter::traits::FromIterator<<A as smallvec::Array>::Item>>::from_iter
  16: rustc::ty::fold::TypeFoldable::fold_with                                                           
  17: rustc::ty::fold::TypeFoldable::fold_with                                                           
  18: rustc::ty::fold::TypeFoldable::fold_with                                                           
  19: rustc::traits::project::normalize                                                                  
  20: rustc::infer::InferCtxt::partially_normalize_associated_types_in                                   
  21: rustc_typeck::check::FnCtxt::instantiate_bounds                                                    
  22: rustc_typeck::check::FnCtxt::instantiate_value_path                                                
  23: rustc_typeck::check::FnCtxt::check_expr_kind                                                       
  24: rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs                                 
  25: rustc::ty::context::tls::with_related_context                                                      
  26: rustc::infer::InferCtxtBuilder::enter                                                              
  27: rustc_typeck::check::typeck_tables_of                                                              
  28: rustc::ty::query::__query_compute::typeck_tables_of                                                
  29: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::typeck_tables_of<'tcx>>::compute
  30: rustc::ty::context::tls::with_context                                                              
  31: rustc::dep_graph::graph::DepGraph::with_task_impl                                                  
  32: rustc::ty::context::tls::with_related_context                                                      
  33: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job
  34: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query         
  35: rustc::ty::query::plumbing::force_from_dep_node                                                    
  36: rustc::dep_graph::graph::DepGraph::try_mark_green                                                  
  37: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::try_mark_green_and_read
  38: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query           
  39: <rustc::traits::project::AssociatedTypeNormalizer<'a, 'b, 'gcx, 'tcx> as rustc::ty::fold::TypeFolder<'gcx, 'tcx>>::fold_const
  40: rustc::ty::structural_impls::<impl rustc::ty::fold::TypeFoldable<'tcx> for &'tcx rustc::ty::TyS<'tcx>>::super_fold_with
  41: <rustc::traits::project::AssociatedTypeNormalizer<'a, 'b, 'gcx, 'tcx> as rustc::ty::fold::TypeFolder<'gcx, 'tcx>>::fold_ty
  42: rustc::traits::project::opt_normalize_projection_type                                              
  43: rustc::traits::project::normalize_projection_type                                                  
  44: <rustc::traits::project::AssociatedTypeNormalizer<'a, 'b, 'gcx, 'tcx> as rustc::ty::fold::TypeFolder<'gcx, 'tcx>>::fold_ty
  45: rustc::ty::fold::TypeFoldable::fold_with                                                           
  46: rustc::ty::fold::TypeFoldable::fold_with                                                           
  47: rustc::traits::project::normalize                                                                  
  48: core::ops::function::impls::<impl core::ops::function::FnOnce<A> for &'a mut F>::call_once         
  49: <core::iter::FlatMap<I, U, F> as core::iter::iterator::Iterator>::next                             
  50: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::from_iter                                    
  51: rustc::ty::wf::WfPredicates::normalize                                                             
  52: rustc::ty::wf::trait_obligations                                                                   
  53: rustc::ty::context::tls::with_related_context                                                      
  54: rustc::infer::InferCtxtBuilder::enter                                                              
  55: rustc_typeck::check::wfcheck::check_item_well_formed                                               
  56: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::check_item_well_formed<'tcx>>::compute
  57: rustc::ty::context::tls::with_context                                                              
  58: rustc::dep_graph::graph::DepGraph::with_task_impl                                                  
  59: rustc::ty::context::tls::with_related_context                                                      
  60: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job
  61: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query           
  62: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::ensure_query        
  63: rustc::hir::Crate::visit_all_item_likes                                                            
  64: rustc::session::Session::track_errors                                                              
  65: rustc_typeck::check_crate                                                                          
  66: rustc::ty::context::tls::enter_context                                                             
  67: <std::thread::local::LocalKey<T>>::with                                                            
  68: rustc::ty::context::TyCtxt::create_and_enter                                                       
  69: rustc_driver::driver::compile_input                                                                
  70: rustc_driver::run_compiler_with_pool                                                               
  71: rustc_driver::driver::spawn_thread_pool                                                            
  72: rustc_driver::run_compiler                                                                         
  73: <scoped_tls::ScopedKey<T>>::set                                                                    
  74: syntax::with_globals                                                                               
  75: __rust_maybe_catch_panic                                                                           
             at libpanic_unwind/lib.rs:102                                                               
  76: rustc_driver::run                                                                                  
  77: rustc_driver::main                                                                                 
  78: std::rt::lang_start::{{closure}}                                                                   
  79: std::panicking::try::do_call                                                                       
             at libstd/rt.rs:59                                                                          
             at libstd/panicking.rs:310                                                                  
  80: __rust_maybe_catch_panic                                                                           
             at libpanic_unwind/lib.rs:102                                                               
  81: std::rt::lang_start_internal                                                                       
             at libstd/panicking.rs:289                                                                  
             at libstd/panic.rs:392                                                                      
             at libstd/rt.rs:58                                                                          
  82: main                                                                                               
  83: __libc_start_main                                                                                  
  84: <unknown>                                                                                          
query stack during panic:                                                                                
#0 [typeck_tables_of] processing `<str as Tr>::Arr::{{constant}}`                                        
#1 [check_item_well_formed] processing `<str as Tr>`                                                     
end of query stack                                                                                       
                                                                                                         
error: internal compiler error: unexpected panic                                                         
                                                                                                         
note: the compiler unexpectedly panicked. this is a bug.                                                 
                                                                                                         
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
                                                                                                         
note: rustc 1.30.0-nightly (2224a42c3 2018-09-17) running on x86_64-unknown-linux-gnu                    
                                                                                                         
note: compiler flags: -C debuginfo=2 -C incremental --crate-type lib                                     
                                                                                                         
note: some of the compiler flags provided by cargo are hidden                                            
                                                                                                         
error: Could not compile `associated-const`.                                                             

To learn more, run the command again with --verbose.
