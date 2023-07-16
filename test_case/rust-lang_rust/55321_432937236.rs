
RUST_BACKTRACE=1 cargo doc --verbose
 Documenting rust-test-2 v0.1.0 (/home/carado/tmp/rust-test-2)                  
     Running `rustdoc --edition=2018 --crate-name rust_test_2 src/lib.rs --color always -o /home/carado/tmp/rust-test-2/target/doc -L dependency=/home/carado/tmp/rust-test-2/target/debug/deps`
thread '<unnamed>' panicked at 'Failed to fully process: B<T> DefId(2/0:825 ~ core[f7f8]::marker[0]::Send[0]) ParamEnv { caller_bounds: [], reveal: UserFacing }', librustc/traits/auto_trait.rs:193:17
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
             at /rustc/f99911a4a0bead7dd1f9ef2f90442844434cc391/src/<::std::macros::panic macros>:8
   8: rustc::ty::context::tls::with_context::{{closure}}                        
             at libcore/option.rs:396                                           
             at librustc/traits/auto_trait.rs:183                               
             at librustc/infer/mod.rs:526                                       
             at librustc/ty/context.rs:1670                                     
             at librustc/ty/context.rs:2016                                     
             at librustc/ty/context.rs:1955                                     
             at librustc/ty/context.rs:2015                                     
             at librustc/ty/context.rs:1669                                     
             at librustc/ty/context.rs:2116                                     
             at librustc/ty/context.rs:2100                                     
   9: rustc::ty::context::GlobalCtxt::enter_local                               
             at librustc/ty/context.rs:2091                                     
             at librustc/ty/context.rs:2100                                     
             at librustc/ty/context.rs:2111                                     
             at librustc/ty/context.rs:1662                                     
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
             at librustdoc/fold.rs:61                                           
             at libcore/iter/mod.rs:1640                                        
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
  19: rustdoc::passes::collect_trait_impls::collect_trait_impls                 
             at librustdoc/fold.rs:115                                          
             at libcore/option.rs:632                                           
             at librustdoc/fold.rs:115                                          
             at librustdoc/passes/collect_trait_impls.rs:26                     
  20: rustdoc::core::run_core::{{closure}}::{{closure}}                         
             at librustdoc/core.rs:614                                          
  21: rustc::ty::context::tls::enter_context                                    
             at librustc_driver/driver.rs:1353                                  
             at librustc/ty/context.rs:2048                                     
             at librustc/ty/context.rs:2016                                     
             at librustc/ty/context.rs:1955                                     
             at librustc/ty/context.rs:2015                                     
  22: <std::thread::local::LocalKey<T>>::with                                   
             at librustc/ty/context.rs:2047                                     
             at librustc/ty/context.rs:2005                                     
             at libstd/thread/local.rs:300                                      
             at libstd/thread/local.rs:254                                      
             at librustc/ty/context.rs:1997                                     
             at libstd/thread/local.rs:300                                      
             at libstd/thread/local.rs:254                                      
  23: rustc::ty::context::TyCtxt::create_and_enter                              
             at librustc/ty/context.rs:1989                                     
             at librustc/ty/context.rs:2027                                     
             at librustc/ty/context.rs:1243                                     
  24: rustc_driver::driver::phase_3_run_analysis_passes                         
             at librustc_driver/driver.rs:1261                                  
  25: <scoped_tls::ScopedKey<T>>::set                                           
             at librustdoc/core.rs:496                                          
             at librustc_driver/driver.rs:76                                    
             at /cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-0.1.2/src/lib.rs:155
  26: rustdoc::core::run_core                                                   
             at librustc_driver/driver.rs:75                                    
             at librustdoc/core.rs:405                                          
  27: <scoped_tls::ScopedKey<T>>::set                                           
             at librustdoc/lib.rs:720                                           
             at /cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-0.1.2/src/lib.rs:155
             at libsyntax/lib.rs:106                                            
             at /cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-0.1.2/src/lib.rs:155
  28: syntax::with_globals                                                      
             at libsyntax/lib.rs:105                                            
  29: <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
             at librustdoc/lib.rs:716                                           
             at librustc_driver/lib.rs:1632                                     
             at libstd/panic.rs:313                                             
  30: std::panicking::try::do_call                                              
             at libstd/panicking.rs:310                                         
  31: __rust_maybe_catch_panic                                                  
             at libpanic_unwind/lib.rs:102                                      
  32: rustc_driver::monitor                                                     
             at libstd/panicking.rs:289                                         
             at libstd/panic.rs:392                                             
             at librustc_driver/lib.rs:1546                                     
             at librustc_driver/lib.rs:1557                                     
             at librustc_driver/lib.rs:1631                                     
  33: rustdoc::rust_input                                                       
             at librustdoc/lib.rs:716                                           
  34: rustdoc::main_args                                                        
             at librustdoc/lib.rs:613                                           
             at librustdoc/lib.rs:568                                           
  35: <scoped_tls::ScopedKey<T>>::set                                           
             at librustdoc/lib.rs:115                                           
             at libcore/option.rs:424                                           
             at librustdoc/lib.rs:115                                           
             at /cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-0.1.2/src/lib.rs:155
             at libsyntax/lib.rs:106                                            
             at /cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-0.1.2/src/lib.rs:155
  36: syntax::with_globals                                                      
             at libsyntax/lib.rs:105                                            
  37: std::panicking::try::do_call                                              
             at libstd/thread/mod.rs:409                                        
             at libstd/panic.rs:313                                             
             at libstd/panicking.rs:310                                         
  38: __rust_maybe_catch_panic                                                  
             at libpanic_unwind/lib.rs:102                                      
  39: <F as alloc::boxed::FnBox<A>>::call_box                                   
             at libstd/panicking.rs:289                                         
             at libstd/panic.rs:392                                             
             at libstd/thread/mod.rs:408                                        
             at liballoc/boxed.rs:672                                           
  40: std::sys_common::thread::start_thread                                     
             at liballoc/boxed.rs:682                                           
             at libstd/sys_common/thread.rs:24                                  
  41: std::sys::unix::thread::Thread::new::thread_start                         
             at libstd/sys/unix/thread.rs:90                                    
  42: start_thread                                                              
             at /builddir/glibc-2.28/nptl/pthread_create.c:486                  
  43: __clone                                                                   
             at ../sysdeps/unix/sysv/linux/x86_64/clone.S:95                    
                                                                                
error: internal compiler error: unexpected panic                                
                                                                                
note: the compiler unexpectedly panicked. this is a bug.                        
                                                                                
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
                                                                                
note: rustc 1.31.0-nightly (f99911a4a 2018-10-23) running on x86_64-unknown-linux-gnu
                                                                                
error: Could not document `rust-test-2`.                                        

Caused by:
  process didn't exit successfully: `rustdoc --edition=2018 --crate-name rust_test_2 src/lib.rs --color always -o /home/carado/tmp/rust-test-2/target/doc -L dependency=/home/carado/tmp/rust-test-2/target/debug/deps` (exit code: 1)
