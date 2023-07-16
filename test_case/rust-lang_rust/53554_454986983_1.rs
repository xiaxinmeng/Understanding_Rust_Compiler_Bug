
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
             at libstd/panicking.rs:480                                                                                                        
   6: std::panicking::begin_panic                                                                                                              
   7: rustc_codegen_llvm::back::link::link_natively                                                                                            
   8: rustc_codegen_llvm::back::link::link_binary                                                                                              
   9: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_utils::codegen_backend::CodegenBackend>::join_codegen_and_link::{{closure}}     
  10: rustc::util::common::time                                                                                                                
  11: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_utils::codegen_backend::CodegenBackend>::join_codegen_and_link                  
  12: rustc_driver::driver::compile_input                                                                                                      
  13: rustc_driver::run_compiler_with_pool                                                                                                     
  14: rustc_driver::driver::spawn_thread_pool                                                                                                  
  15: rustc_driver::run_compiler                                                                                                               
  16: <scoped_tls::ScopedKey<T>>::set                                                                                                          
  17: <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once                                                          
  18: __rust_maybe_catch_panic                                                                                                                 
             at libpanic_unwind/lib.rs:102                                                                                                     
  19: rustc_driver::run                                                                                                                        
  20: rustc_driver::main                       
