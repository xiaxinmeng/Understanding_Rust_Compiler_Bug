
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.21.0-dev (9b544108c 2017-08-16) running on x86_64-unknown-linux-gnu

note: run with `RUST_BACKTRACE=1` for a backtrace

thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', /home/ariel/Rust/rust-emergency/src/libcore/option.rs:335:20
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
   0: std::sys::imp::backtrace::tracing::imp::unwind_backtrace
             at /home/ariel/Rust/rust-emergency/src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::_print
             at /home/ariel/Rust/rust-emergency/src/libstd/sys_common/backtrace.rs:71
   2: std::panicking::default_hook::{{closure}}
             at /home/ariel/Rust/rust-emergency/src/libstd/sys_common/backtrace.rs:60
             at /home/ariel/Rust/rust-emergency/src/libstd/panicking.rs:381
   3: std::panicking::default_hook
             at /home/ariel/Rust/rust-emergency/src/libstd/panicking.rs:391
   4: std::panicking::rust_panic_with_hook
             at /home/ariel/Rust/rust-emergency/src/libstd/panicking.rs:611                                                                  
   5: std::panicking::begin_panic                                                                                                            
             at /home/ariel/Rust/rust-emergency/src/libstd/panicking.rs:572                                                                  
   6: std::panicking::begin_panic_fmt                                                                                                        
             at /home/ariel/Rust/rust-emergency/src/libstd/panicking.rs:522                                                                  
   7: rust_begin_unwind                                                                                                                      
             at /home/ariel/Rust/rust-emergency/src/libstd/panicking.rs:498                                                                  
   8: core::panicking::panic_fmt                                                                                                             
             at /home/ariel/Rust/rust-emergency/src/libcore/panicking.rs:71                                                                  
   9: core::panicking::panic                                                                                                                 
             at /home/ariel/Rust/rust-emergency/src/libcore/panicking.rs:51                                                                  
  10: <rustc_metadata::creader::CrateLoader<'a> as rustc::middle::cstore::CrateLoader>::process_item                                         
             at /home/ariel/Rust/rust-emergency/src/libcore/macros.rs:32                                                                     
             at /home/ariel/Rust/rust-emergency/src/librustc_metadata/creader.rs:1217                                                        
             at /home/ariel/Rust/rust-emergency/src/libcore/option.rs:398                                                                    
             at /home/ariel/Rust/rust-emergency/src/librustc_metadata/creader.rs:1216                                                        
             at /home/ariel/Rust/rust-emergency/src/librustc_metadata/creader.rs:1309                                                        
  11: rustc_resolve::build_reduced_graph::<impl rustc_resolve::Resolver<'a>>::build_reduced_graph_for_item                                   
             at /home/ariel/Rust/rust-emergency/src/librustc_resolve/build_reduced_graph.rs:299                                              
  12: <rustc_resolve::build_reduced_graph::BuildReducedGraphVisitor<'a, 'b> as syntax::visit::Visitor<'a>>::visit_item                       
             at /home/ariel/Rust/rust-emergency/src/librustc_resolve/build_reduced_graph.rs:772                                              
  13: <rustc_resolve::build_reduced_graph::BuildReducedGraphVisitor<'a, 'b> as syntax::visit::Visitor<'a>>::visit_item                       
             at /home/ariel/Rust/rust-emergency/src/libsyntax/visit.rs:146                                                                   
             at /home/ariel/Rust/rust-emergency/src/libsyntax/visit.rs:60                                                                    
             at /home/ariel/Rust/rust-emergency/src/libsyntax/visit.rs:257                                                                   
             at /home/ariel/Rust/rust-emergency/src/librustc_resolve/build_reduced_graph.rs:773                                              
  14: rustc_resolve::macros::<impl syntax::ext::base::Resolver for rustc_resolve::Resolver<'a>>::visit_expansion                             
             at /home/ariel/Rust/rust-emergency/src/libsyntax/ext/expand.rs:96                                                               
             at /home/ariel/Rust/rust-emergency/src/librustc_resolve/macros.rs:175                                                           
  15: syntax::ext::expand::MacroExpander::collect_invocations                                                                                
             at /home/ariel/Rust/rust-emergency/src/libsyntax/ext/expand.rs:370                                                              
  16: syntax::ext::expand::MacroExpander::expand                                                                                             
             at /home/ariel/Rust/rust-emergency/src/libsyntax/ext/expand.rs:244                                                              
  17: syntax::ext::expand::MacroExpander::expand_crate                                                                                       
             at /home/ariel/Rust/rust-emergency/src/libsyntax/ext/expand.rs:220                                                              
  18: rustc_driver::driver::phase_2_configure_and_expand::{{closure}}                                                                        
             at /home/ariel/Rust/rust-emergency/src/librustc_driver/driver.rs:739                                                            
  19: rustc_driver::driver::phase_2_configure_and_expand                                                                                     
             at /home/ariel/Rust/rust-emergency/src/librustc/util/common.rs:48                                                               
             at /home/ariel/Rust/rust-emergency/src/librustc_driver/driver.rs:698                                                            
  20: rustc_driver::driver::compile_input                                                                                                    
             at /home/ariel/Rust/rust-emergency/src/librustc_driver/driver.rs:139                                                            
  21: rustc_driver::run_compiler                                                                                                             
             at /home/ariel/Rust/rust-emergency/src/librustc_driver/lib.rs:316                                                               
