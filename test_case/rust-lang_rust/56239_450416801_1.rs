
 Documenting libp2p-core v0.1.0 (https://github.com/libp2p/rust-libp2p#7798e23e)                                                              
thread '<unnamed>' panicked at 'Unable to fulfill trait DefId(2/0:825 ~ core[5bfb]::marker[0]::Send[0]) for 'protocols_handler::node_handler::NodeHandlerWrapper<TProtoHandler>': [FulfillmentError(Obligation(predicate=Binder(TraitPredicate(<<<TProtoHandler as protocols_handler::ProtocolsHandler>::OutboundProtocol as upgrade::UpgradeInfo>::Info as std::marker::Send>)),depth=12),Unimplemented)]', librustc/traits/auto_trait.rs:218:17
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
   5: std::panicking::continue_panic_fmt                                                                                                      
             at libstd/panicking.rs:390                                                                                                       
   6: std::panicking::begin_panic_fmt                                                                                                         
             at libstd/panicking.rs:345                                                                                                       
   7: rustc::traits::auto_trait::AutoTraitFinder::find_auto_trait_generics::{{closure}}::{{closure}}                                          
             at /rustc/b6c32da9b0481e3e9d737153286b3ff8aa39a22c/src/<::std::macros::panic macros>:8                                           
   8: rustc::ty::context::tls::with_context::{{closure}}                                                                                      
             at libcore/result.rs:774                                                                                                         
             at librustc/traits/auto_trait.rs:217                                                                                             
             at librustc/infer/mod.rs:526                                                                                                     
             at librustc/ty/context.rs:1681                                                                                                   
             at librustc/ty/context.rs:2045                                                                                                   
             at librustc/ty/context.rs:1984                                                                                                   
             at librustc/ty/context.rs:2044                                                                                                   
             at librustc/ty/context.rs:1680                                                                                                   
             at librustc/ty/context.rs:2145                                                                                                   
             at librustc/ty/context.rs:2129                                                                                                   
   9: rustc::ty::context::GlobalCtxt::enter_local                                                                                             
             at librustc/ty/context.rs:2120                                                                                                   
             at librustc/ty/context.rs:2129                                                                                                   
             at librustc/ty/context.rs:2140                                                                                                   
             at librustc/ty/context.rs:1673                                                                                                   
  10: rustc::traits::auto_trait::AutoTraitFinder::find_auto_trait_generics                                                                    
             at librustc/infer/mod.rs:525                                                                                                     
             at librustc/traits/auto_trait.rs:133                                                                                             
  11: rustdoc::clean::auto_trait::AutoTraitFinder::get_auto_trait_impl_for                                                                    
             at librustdoc/clean/auto_trait.rs:200                                                                                            
             at librustdoc/clean/auto_trait.rs:123                                                                                            
  12: rustdoc::clean::auto_trait::AutoTraitFinder::get_auto_trait_impls                                                                       
             at librustdoc/clean/auto_trait.rs:76                                                                                             
             at libcore/option.rs:632                                                                                                         
             at librustdoc/clean/auto_trait.rs:73                                                                                             
  13: rustdoc::clean::def_ctor::get_def_from_node_id                                                                                          
             at librustdoc/clean/auto_trait.rs:41                                                                                             
             at librustdoc/clean/def_ctor.rs:59                                                                                               
  14: <rustdoc::passes::collect_trait_impls::SyntheticImplCollector<'a, 'tcx, 'rcx, 'cstore> as rustdoc::fold::DocFolder>::fold_item          
             at librustdoc/clean/auto_trait.rs:39                                                                                             
             at librustdoc/clean/mod.rs:3353                                                                                                  
             at librustdoc/passes/collect_trait_impls.rs:170                                                                                  
  15: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::from_iter                                                                         
             at librustdoc/fold.rs:41                                                                                                         
             at libcore/iter/mod.rs:1572                                                                                                      
             at liballoc/vec.rs:1908                                                                                                          
             at liballoc/vec.rs:1805                                                                                                          
             at liballoc/vec.rs:1800                                                                                                          
  16: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::from_iter                                                                         
  17: rustdoc::fold::DocFolder::fold_inner_recur                                                                                              
             at liballoc/vec.rs:1700                                                                                                          
             at libcore/iter/iterator.rs:1476                                                                                                 
             at librustdoc/fold.rs:110                                                                                                        
             at librustdoc/fold.rs:37                                                                                                         
  18: <rustdoc::passes::collect_trait_impls::SyntheticImplCollector<'a, 'tcx, 'rcx, 'cstore> as rustdoc::fold::DocFolder>::fold_item          
             at librustdoc/fold.rs:100                                                                                                        
             at librustdoc/passes/collect_trait_impls.rs:178                                                                                  
  19: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::from_iter                                                                         
             at librustdoc/fold.rs:41                                                                                                         
             at libcore/iter/mod.rs:1572                                                                                                      
             at liballoc/vec.rs:1908                                                                                                          
             at liballoc/vec.rs:1805                                                                                                          
             at liballoc/vec.rs:1800                                                                                                          
  20: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::from_iter                                                                         
  21: rustdoc::fold::DocFolder::fold_inner_recur                                                                                              
             at liballoc/vec.rs:1700                                                                                                          
             at libcore/iter/iterator.rs:1476                                                                                                 
             at librustdoc/fold.rs:110                                                                                                        
             at librustdoc/fold.rs:37                                                                                                         
  22: <rustdoc::passes::collect_trait_impls::SyntheticImplCollector<'a, 'tcx, 'rcx, 'cstore> as rustdoc::fold::DocFolder>::fold_item          
             at librustdoc/fold.rs:100                                                                                                        
             at librustdoc/passes/collect_trait_impls.rs:178                                                                                  
  23: rustdoc::passes::collect_trait_impls::collect_trait_impls                                                                               
             at librustdoc/fold.rs:115                                                                                                        
             at libcore/option.rs:632                                                                                                         
             at librustdoc/fold.rs:115                                                                                                        
             at librustdoc/passes/collect_trait_impls.rs:26                                                                                   
  24: rustdoc::core::run_core::{{closure}}::{{closure}}                                                                                       
             at librustdoc/core.rs:618                                                                                                        
  25: rustc::ty::context::tls::enter_context                                                                                                  
             at librustc_driver/driver.rs:1355                                                                                                
             at librustc/ty/context.rs:2077                                                                                                   
             at librustc/ty/context.rs:2045                                                                                                   
             at librustc/ty/context.rs:1984                                                                                                   
             at librustc/ty/context.rs:2044                                                                                                   
  26: <std::thread::local::LocalKey<T>>::with                                                                                                 
             at librustc/ty/context.rs:2076                                                                                                   
             at librustc/ty/context.rs:2034                                                                                                   
             at libstd/thread/local.rs:300                                                                                                    
             at libstd/thread/local.rs:254                                                                                                    
             at librustc/ty/context.rs:2026                                                                                                   
             at libstd/thread/local.rs:300                                                                                                    
             at libstd/thread/local.rs:254                                                                                                    
  27: rustc::ty::context::TyCtxt::create_and_enter                                                                                            
             at librustc/ty/context.rs:2018                                                                                                   
             at librustc/ty/context.rs:2056                                                                                                   
             at librustc/ty/context.rs:1247                                                                                                   
  28: rustc_driver::driver::phase_3_run_analysis_passes                                                                                       
             at librustc_driver/driver.rs:1263                                                                                                
  29: <scoped_tls::ScopedKey<T>>::set                                                                                                         
             at librustdoc/core.rs:500                                                                                                        
             at librustc_driver/driver.rs:76                                                                                                  
             at /cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-0.1.2/src/lib.rs:155                                               
  30: rustdoc::core::run_core                                                                                                                 
             at librustc_driver/driver.rs:75                                                                                                  
             at librustdoc/core.rs:407                                                                                                        
  31: <scoped_tls::ScopedKey<T>>::set                                                                                                         
             at librustdoc/lib.rs:720                                                                                                         
             at /cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-0.1.2/src/lib.rs:155                                               
             at libsyntax/lib.rs:106                                                                                                          
             at /cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-0.1.2/src/lib.rs:155                                               
  32: syntax::with_globals                                                                                                                    
             at libsyntax/lib.rs:105                                                                                                          
  33: <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once                                                         
             at librustdoc/lib.rs:716                                                                                                         
             at librustc_driver/lib.rs:1634                                                                                                   
             at libstd/panic.rs:313                                                                                                           
  34: std::panicking::try::do_call                                                                                                            
             at libstd/panicking.rs:310                                                                                                       
  35: __rust_maybe_catch_panic                                                                                                                
             at libpanic_unwind/lib.rs:102                                                                                                    
  36: rustc_driver::monitor                                                                                                                   
             at libstd/panicking.rs:289                                                                                                       
             at libstd/panic.rs:392                                                                                                           
             at librustc_driver/lib.rs:1548                                                                                                   
             at librustc_driver/lib.rs:1559                                                                                                   
             at librustc_driver/lib.rs:1633                                                                                                   
  37: rustdoc::rust_input                                                                                                                     
             at librustdoc/lib.rs:716                                                                                                         
  38: rustdoc::main_args                                                                                                                      
             at librustdoc/lib.rs:613                                                                                                         
             at librustdoc/lib.rs:568                                                                                                         
  39: <scoped_tls::ScopedKey<T>>::set                                                                                                         
             at librustdoc/lib.rs:115                                                                                                         
             at libcore/option.rs:424                                                                                                         
             at librustdoc/lib.rs:115                                                                                                         
             at /cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-0.1.2/src/lib.rs:155                                               
             at libsyntax/lib.rs:106                                                                                                          
             at /cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-0.1.2/src/lib.rs:155                                               
  40: syntax::with_globals                                                                                                                    
             at libsyntax/lib.rs:105                                                                                                          
  41: std::panicking::try::do_call                                                                                                            
             at libstd/thread/mod.rs:409                                                                                                      
             at libstd/panic.rs:313                                                                                                           
             at libstd/panicking.rs:310                                                                                                       
  42: __rust_maybe_catch_panic                                                                                                                
             at libpanic_unwind/lib.rs:102                                                                                                    
  43: <F as alloc::boxed::FnBox<A>>::call_box                                                                                                 
             at libstd/panicking.rs:289                                                                                                       
             at libstd/panic.rs:392                                                                                                           
             at libstd/thread/mod.rs:408                                                                                                      
             at liballoc/boxed.rs:672                                                                                                         
  44: std::sys_common::thread::start_thread                                                                                                   
             at liballoc/boxed.rs:682                                                                                                         
             at libstd/sys_common/thread.rs:24                                                                                                
  45: std::sys::unix::thread::Thread::new::thread_start                                                                                       
             at libstd/sys/unix/thread.rs:90                                                                                                  
  46: start_thread                                                                                                                            
  47: __clone                                                                                                                                 
                                                                                                                                              
error: internal compiler error: unexpected panic                                                                                              
                                                                                                                                              
note: the compiler unexpectedly panicked. this is a bug.                                                                                      
                                                                                                                                              
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports                             
                                                                                                                                              
note: rustc 1.31.1 (b6c32da9b 2018-12-18) running on x86_64-unknown-linux-gnu                                                                 
                                                                                                                                              
error: Could not document `libp2p-core`.                                                                                                      

Caused by:
  process didn't exit successfully: `rustdoc --crate-name libp2p_core /home/maciej/.cargo/git/checkouts/rust-libp2p-98135dbcf5b63918/7798e23/core/src/lib.rs --cap-lints allow --color always -o /home/maciej/Projekte/ssh-tool/target/doc -L dependency=/home/maciej/Projekte/ssh-tool/target/debug/deps --extern bs58=/home/maciej/Projekte/ssh-tool/target/debug/deps/libbs58-1b44444b3e88b862.rmeta --extern bytes=/home/maciej/Projekte/ssh-tool/target/debug/deps/libbytes-57294ab3ba6d385d.rmeta --extern fnv=/home/maciej/Projekte/ssh-tool/target/debug/deps/libfnv-d1540d2dbd6cc1ab.rmeta --extern futures=/home/maciej/Projekte/ssh-tool/target/debug/deps/libfutures-d5ee614fdb98fcee.rmeta --extern log=/home/maciej/Projekte/ssh-tool/target/debug/deps/liblog-fef7f360db950dde.rmeta --extern multistream_select=/home/maciej/Projekte/ssh-tool/target/debug/deps/libmultistream_select-dc5dc2f38f244791.rmeta --extern multiaddr=/home/maciej/Projekte/ssh-tool/target/debug/deps/libparity_multiaddr-dfc286807f8a3939.rmeta --extern multihash=/home/maciej/Projekte/ssh-tool/target/debug/deps/libparity_multihash-233c6195db2b7b8a.rmeta --extern parking_lot=/home/maciej/Projekte/ssh-tool/target/debug/deps/libparking_lot-2fc1623cb5b5ed93.rmeta --extern protobuf=/home/maciej/Projekte/ssh-tool/target/debug/deps/libprotobuf-cc9a049d2afe4b36.rmeta --extern quick_error=/home/maciej/Projekte/ssh-tool/target/debug/deps/libquick_error-20bc5fa753657fcd.rmeta --extern rw_stream_sink=/home/maciej/Projekte/ssh-tool/target/debug/deps/librw_stream_sink-676bca186bd958c4.rmeta --extern smallvec=/home/maciej/Projekte/ssh-tool/target/debug/deps/libsmallvec-52a6998238b84886.rmeta --extern tokio_executor=/home/maciej/Projekte/ssh-tool/target/debug/deps/libtokio_executor-58c081ebb9a9aebb.rmeta --extern tokio_io=/home/maciej/Projekte/ssh-tool/target/debug/deps/libtokio_io-7708ccf14e17b1ca.rmeta --extern tokio_timer=/home/maciej/Projekte/ssh-tool/target/debug/deps/libtokio_timer-fe53d970fa8c594c.rmeta --extern void=/home/maciej/Projekte/ssh-tool/target/debug/deps/libvoid-89466812b616a6f1.rmeta` (exit code: 1)
