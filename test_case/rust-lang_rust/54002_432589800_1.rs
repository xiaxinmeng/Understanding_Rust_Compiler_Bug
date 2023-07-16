
$ RUST_BACKTRACE=1 cargo doc --open
 Documenting differential-dataflow v0.7.0                                                               
error: internal compiler error: librustc/traits/select.rs:2764: Where clause `Binder(<_ as std::marker::Send>)` was applicable to `Obligation(predicate=Binder(TraitPredicate(<operators::ValueHistory<'_, _, _, _> as std::marker::Send>)),depth=2)` but now is not
                                                                                                        
thread '<unnamed>' panicked at 'Box<Any>', librustc_errors/lib.rs:600:9                                 
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
   4: std::panicking::rust_panic_with_hook                                                              
             at libstd/panicking.rs:476                                                                 
   5: std::panicking::begin_panic                                                                       
   6: rustc_errors::Handler::bug                                                                        
   7: rustc::util::bug::opt_span_bug_fmt::{{closure}}                                                   
   8: rustc::ty::context::tls::with_opt::{{closure}}                                                    
   9: rustc::ty::context::tls::with_context_opt                                                         
  10: rustc::ty::context::tls::with_opt                                                                 
  11: rustc::util::bug::opt_span_bug_fmt                                                                
  12: rustc::util::bug::bug_fmt                                                                         
  13: rustc::traits::select::SelectionContext::confirm_candidate                                        
  14: rustc::infer::InferCtxt::probe                                                                    
  15: rustc::traits::select::SelectionContext::evaluate_stack                                           
  16: rustc::dep_graph::graph::DepGraph::with_anon_task                                                 
  17: rustc::traits::select::SelectionContext::evaluate_predicate_recursively                           
  18: rustc::infer::InferCtxt::probe                                                                    
  19: rustc::traits::select::SelectionContext::evaluate_stack                                           
  20: rustc::dep_graph::graph::DepGraph::with_anon_task                                                 
  21: rustc::traits::select::SelectionContext::evaluate_predicate_recursively                           
  22: rustc::infer::InferCtxt::probe                                                                    
  23: rustc::traits::select::SelectionContext::evaluate_stack                                           
  24: rustc::dep_graph::graph::DepGraph::with_anon_task                                                 
  25: rustc::traits::select::SelectionContext::evaluate_predicate_recursively                           
  26: rustc::traits::select::SelectionContext::evaluate_obligation_recursively                          
  27: rustc::ty::context::GlobalCtxt::enter_local                                                       
  28: rustc_traits::evaluate_obligation::evaluate_obligation                                            
  29: rustc::ty::query::__query_compute::evaluate_obligation                                            
  30: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::evaluate_obligation<'tcx>>::compute
  31: rustc::dep_graph::graph::DepGraph::with_task_impl                                                 
  32: rustc::ty::context::tls::with_related_context                                                     
  33: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job
  34: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query          
  35: rustc::traits::query::evaluate_obligation::<impl rustc::infer::InferCtxt<'cx, 'gcx, 'tcx>>::evaluate_obligation
  36: rustc::ty::context::tls::with_context::{{closure}}                                                
             at librustdoc/clean/blanket_impl.rs:110                                                    
             at librustc/infer/mod.rs:526                                                               
             at librustc/ty/context.rs:1670                                                             
             at librustc/ty/context.rs:2016                                                             
             at librustc/ty/context.rs:1955                                                             
             at librustc/ty/context.rs:2015                                                             
             at librustc/ty/context.rs:1669                                                             
             at librustc/ty/context.rs:2116                                                             
             at librustc/ty/context.rs:2100                                                             
  37: rustc::ty::context::GlobalCtxt::enter_local                                                       
             at librustc/ty/context.rs:2091                                                             
             at librustc/ty/context.rs:2100                                                             
             at librustc/ty/context.rs:2111                                                             
             at librustc/ty/context.rs:1662                                                             
  38: rustdoc::clean::blanket_impl::BlanketImplFinder::get_blanket_impls::{{closure}}                   
             at librustc/infer/mod.rs:525                                                               
             at librustdoc/clean/blanket_impl.rs:80                                                     
  39: rustc::ty::trait_def::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::for_each_relevant_impl   
             at librustc/ty/trait_def.rs:106                                                            
  40: rustdoc::clean::blanket_impl::BlanketImplFinder::get_blanket_impls                                
             at librustdoc/clean/blanket_impl.rs:79                                                     
  41: rustdoc::clean::def_ctor::get_def_from_node_id                                                    
             at librustdoc/clean/blanket_impl.rs:42                                                     
             at librustdoc/clean/def_ctor.rs:59                                                         
  42: <rustdoc::passes::collect_trait_impls::SyntheticImplCollector<'a, 'tcx, 'rcx, 'cstore> as rustdoc::fold::DocFolder>::fold_item
             at librustdoc/clean/blanket_impl.rs:40                                                     
             at librustdoc/clean/mod.rs:3364                                                            
             at librustdoc/passes/collect_trait_impls.rs:171                                            
  43: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::from_iter                                   
             at librustdoc/fold.rs:61                                                                   
             at libcore/iter/mod.rs:1640                                                                
             at liballoc/vec.rs:1908                                                                    
             at liballoc/vec.rs:1805                                                                    
             at liballoc/vec.rs:1800                                                                    
  44: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::from_iter                                   
  45: rustdoc::fold::DocFolder::fold_inner_recur                                                        
             at liballoc/vec.rs:1700                                                                    
             at libcore/iter/iterator.rs:1476                                                           
             at librustdoc/fold.rs:110                                                                  
             at librustdoc/fold.rs:37                                                                   
  46: <rustdoc::passes::collect_trait_impls::SyntheticImplCollector<'a, 'tcx, 'rcx, 'cstore> as rustdoc::fold::DocFolder>::fold_item
             at librustdoc/fold.rs:100                                                                  
             at librustdoc/passes/collect_trait_impls.rs:178                                            
  47: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::from_iter                                   
             at librustdoc/fold.rs:61                                                                   
             at libcore/iter/mod.rs:1640                                                                
             at liballoc/vec.rs:1908                                                                    
             at liballoc/vec.rs:1805                                                                    
             at liballoc/vec.rs:1800                                                                    
  48: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::from_iter                                   
  49: rustdoc::fold::DocFolder::fold_inner_recur                                                        
             at liballoc/vec.rs:1700                                                                    
             at libcore/iter/iterator.rs:1476                                                           
             at librustdoc/fold.rs:110                                                                  
             at librustdoc/fold.rs:37                                                                   
  50: <rustdoc::passes::collect_trait_impls::SyntheticImplCollector<'a, 'tcx, 'rcx, 'cstore> as rustdoc::fold::DocFolder>::fold_item
             at librustdoc/fold.rs:100                                                                  
             at librustdoc/passes/collect_trait_impls.rs:178                                            
  51: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::from_iter                                   
             at librustdoc/fold.rs:61                                                                   
             at libcore/iter/mod.rs:1640                                                                
             at liballoc/vec.rs:1908                                                                    
             at liballoc/vec.rs:1805                                                                    
             at liballoc/vec.rs:1800                                                                    
  52: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::from_iter                                   
  53: rustdoc::fold::DocFolder::fold_inner_recur                                                        
             at liballoc/vec.rs:1700                                                                    
             at libcore/iter/iterator.rs:1476                                                           
             at librustdoc/fold.rs:110                                                                  
             at librustdoc/fold.rs:37                                                                   
  54: <rustdoc::passes::collect_trait_impls::SyntheticImplCollector<'a, 'tcx, 'rcx, 'cstore> as rustdoc::fold::DocFolder>::fold_item
             at librustdoc/fold.rs:100                                                                  
             at librustdoc/passes/collect_trait_impls.rs:178                                            
  55: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::from_iter                                   
             at librustdoc/fold.rs:61                                                                   
             at libcore/iter/mod.rs:1640                                                                
             at liballoc/vec.rs:1908                                                                    
             at liballoc/vec.rs:1805                                                                    
             at liballoc/vec.rs:1800                                                                    
  56: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::from_iter                                   
  57: rustdoc::fold::DocFolder::fold_inner_recur                                                        
             at liballoc/vec.rs:1700                                                                    
             at libcore/iter/iterator.rs:1476                                                           
             at librustdoc/fold.rs:110                                                                  
             at librustdoc/fold.rs:37                                                                   
  58: <rustdoc::passes::collect_trait_impls::SyntheticImplCollector<'a, 'tcx, 'rcx, 'cstore> as rustdoc::fold::DocFolder>::fold_item
             at librustdoc/fold.rs:100                                                                  
             at librustdoc/passes/collect_trait_impls.rs:178                                            
  59: rustdoc::passes::collect_trait_impls::collect_trait_impls                                         
             at librustdoc/fold.rs:115                                                                  
             at libcore/option.rs:632                                                                   
             at librustdoc/fold.rs:115                                                                  
             at librustdoc/passes/collect_trait_impls.rs:26                                             
  60: rustdoc::core::run_core::{{closure}}::{{closure}}                                                 
             at librustdoc/core.rs:614                                                                  
  61: rustc::ty::context::tls::enter_context                                                            
             at librustc_driver/driver.rs:1353                                                          
             at librustc/ty/context.rs:2048                                                             
             at librustc/ty/context.rs:2016                                                             
             at librustc/ty/context.rs:1955                                                             
             at librustc/ty/context.rs:2015                                                             
  62: <std::thread::local::LocalKey<T>>::with                                                           
             at librustc/ty/context.rs:2047                                                             
             at librustc/ty/context.rs:2005                                                             
             at libstd/thread/local.rs:300                                                              
             at libstd/thread/local.rs:254                                                              
             at librustc/ty/context.rs:1997                                                             
             at libstd/thread/local.rs:300                                                              
             at libstd/thread/local.rs:254                                                              
  63: rustc::ty::context::TyCtxt::create_and_enter                                                      
             at librustc/ty/context.rs:1989                                                             
             at librustc/ty/context.rs:2027                                                             
             at librustc/ty/context.rs:1243                                                             
  64: rustc_driver::driver::phase_3_run_analysis_passes                                                 
             at librustc_driver/driver.rs:1261                                                          
  65: <scoped_tls::ScopedKey<T>>::set                                                                   
             at librustdoc/core.rs:496                                                                  
             at librustc_driver/driver.rs:76                                                            
             at /cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-0.1.2/src/lib.rs:155         
  66: rustdoc::core::run_core                                                                           
             at librustc_driver/driver.rs:75                                                            
             at librustdoc/core.rs:405                                                                  
  67: <scoped_tls::ScopedKey<T>>::set                                                                   
             at librustdoc/lib.rs:720                                                                   
             at /cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-0.1.2/src/lib.rs:155         
             at libsyntax/lib.rs:106                                                                    
             at /cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-0.1.2/src/lib.rs:155         
  68: syntax::with_globals                                                                              
             at libsyntax/lib.rs:105                                                                    
  69: <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once                   
             at librustdoc/lib.rs:716                                                                   
             at librustc_driver/lib.rs:1632                                                             
             at libstd/panic.rs:313                                                                     
  70: std::panicking::try::do_call                                                                      
             at libstd/panicking.rs:310                                                                 
  71: __rust_maybe_catch_panic                                                                          
             at libpanic_unwind/lib.rs:102                                                              
  72: rustc_driver::monitor                                                                             
             at libstd/panicking.rs:289                                                                 
             at libstd/panic.rs:392                                                                     
             at librustc_driver/lib.rs:1546                                                             
             at librustc_driver/lib.rs:1557                                                             
             at librustc_driver/lib.rs:1631                                                             
  73: rustdoc::rust_input                                                                               
             at librustdoc/lib.rs:716                                                                   
  74: rustdoc::main_args                                                                                
             at librustdoc/lib.rs:613                                                                   
             at librustdoc/lib.rs:568                                                                   
  75: <scoped_tls::ScopedKey<T>>::set                                                                   
             at librustdoc/lib.rs:115                                                                   
             at libcore/option.rs:424                                                                   
             at librustdoc/lib.rs:115                                                                   
             at /cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-0.1.2/src/lib.rs:155         
             at libsyntax/lib.rs:106                                                                    
             at /cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-0.1.2/src/lib.rs:155         
  76: syntax::with_globals                                                                              
             at libsyntax/lib.rs:105                                                                    
  77: std::panicking::try::do_call                                                                      
             at libstd/thread/mod.rs:409                                                                
             at libstd/panic.rs:313                                                                     
             at libstd/panicking.rs:310                                                                 
  78: __rust_maybe_catch_panic                                                                          
             at libpanic_unwind/lib.rs:102                                                              
  79: <F as alloc::boxed::FnBox<A>>::call_box                                                           
             at libstd/panicking.rs:289                                                                 
             at libstd/panic.rs:392                                                                     
             at libstd/thread/mod.rs:408                                                                
             at liballoc/boxed.rs:672                                                                   
  80: std::sys_common::thread::start_thread                                                             
             at liballoc/boxed.rs:682                                                                   
             at libstd/sys_common/thread.rs:24                                                          
  81: std::sys::unix::thread::Thread::new::thread_start                                                 
             at libstd/sys/unix/thread.rs:90                                                            
  82: start_thread                                                                                      
  83: __clone                    
