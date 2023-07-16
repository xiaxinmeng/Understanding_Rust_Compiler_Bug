
 Documenting libp2p-core v0.1.0 (https://github.com/libp2p/rust-libp2p#7798e23e)                                                                                              
thread '<unnamed>' panicked at 'Unable to fulfill trait DefId(2/0:825 ~ core[c2d2]::marker[0]::Send[0]) for 'protocols_handler::node_handler::NodeHandlerWrapper<TProtoHandler>': [FulfillmentError(Obligation(predicate=Binder(TraitPredicate(<<<TProtoHandler as protocols_handler::ProtocolsHandler>::OutboundProtocol as upgrade::UpgradeInfo>::Info as std::marker::Send>)),depth=12),Unimplemented)]', librustc/traits/auto_trait.rs:218:17
stack backtrace:                                                                                                                                                              
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace                                                                                                               
   1: std::sys_common::backtrace::print                                                                                                                                       
   2: std::panicking::default_hook::{{closure}}                                                                                                                               
   3: std::panicking::default_hook                                                                                                                                            
   4: std::panicking::rust_panic_with_hook                                                                                                                                    
   5: std::panicking::continue_panic_fmt                                                                                                                                      
   6: std::panicking::begin_panic_fmt                                                                                                                                         
   7: rustc::traits::auto_trait::AutoTraitFinder::find_auto_trait_generics::{{closure}}::{{closure}}                                                                          
   8: rustc::ty::context::tls::with_context::{{closure}}                                                                                                                      
   9: rustc::ty::context::GlobalCtxt::enter_local                                                                                                                             
  10: rustc::traits::auto_trait::AutoTraitFinder::find_auto_trait_generics                                                                                                    
  11: rustdoc::clean::auto_trait::AutoTraitFinder::get_auto_trait_impl_for                                                                                                    
  12: rustdoc::clean::auto_trait::AutoTraitFinder::get_auto_trait_impls                                                                                                       
  13: rustdoc::clean::def_ctor::get_def_from_node_id                                                                                                                          
  14: <rustdoc::passes::collect_trait_impls::SyntheticImplCollector<'a, 'tcx, 'rcx, 'cstore> as rustdoc::fold::DocFolder>::fold_item                                          
  15: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::from_iter                                                                                                         
  16: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::from_iter                                                                                                         
  17: rustdoc::fold::DocFolder::fold_inner_recur                                                                                                                              
  18: <rustdoc::passes::collect_trait_impls::SyntheticImplCollector<'a, 'tcx, 'rcx, 'cstore> as rustdoc::fold::DocFolder>::fold_item                                          
  19: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::from_iter                                                                                                         
  20: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::from_iter                                                                                                         
  21: rustdoc::fold::DocFolder::fold_inner_recur                                                                                                                              
  22: <rustdoc::passes::collect_trait_impls::SyntheticImplCollector<'a, 'tcx, 'rcx, 'cstore> as rustdoc::fold::DocFolder>::fold_item                                          
  23: rustdoc::passes::collect_trait_impls::collect_trait_impls                                                                                                               
  24: rustdoc::core::run_core::{{closure}}::{{closure}}                                                                                                                       
  25: rustc::ty::context::tls::enter_context                                                                                                                                  
  26: <std::thread::local::LocalKey<T>>::with                                                                                                                                 
  27: rustc::ty::context::TyCtxt::create_and_enter                                                                                                                            
  28: rustc_driver::driver::phase_3_run_analysis_passes                                                                                                                       
  29: <scoped_tls::ScopedKey<T>>::set                                                                                                                                         
  30: rustdoc::core::run_core                                                                                                                                                 
  31: <scoped_tls::ScopedKey<T>>::set                                                                                                                                         
  32: syntax::with_globals                                                                                                                                                    
  33: <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once                                                                                         
  34: std::panicking::try::do_call                                                                                                                                            
  35: __rust_maybe_catch_panic                                                                                                                                                
  36: rustc_driver::monitor                                                                                                                                                   
  37: rustdoc::rust_input                                                                                                                                                     
  38: rustdoc::main_args                                                                                                                                                      
  39: <scoped_tls::ScopedKey<T>>::set                                                                                                                                         
  40: syntax::with_globals                                                                                                                                                    
  41: std::panicking::try::do_call                                                                                                                                            
  42: __rust_maybe_catch_panic                                                                                                                                                
  43: <F as alloc::boxed::FnBox<A>>::call_box                                                                                                                                 
  44: std::sys_common::thread::start_thread                                                                                                                                   
  45: std::sys::unix::thread::Thread::new::thread_start                                                                                                                       
  46: _pthread_body                                                                                                                                                           
  47: _pthread_start                                                                                                                                                          
                                                                                                                                                                              
error: internal compiler error: unexpected panic                                                                                                                              
                                                                                                                                                                              
note: the compiler unexpectedly panicked. this is a bug.                                                                                                                      
                                                                                                                                                                              
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports                                                             
                                                                                                                                                                              
note: rustc 1.31.1 (b6c32da9b 2018-12-18) running on x86_64-apple-darwin    
