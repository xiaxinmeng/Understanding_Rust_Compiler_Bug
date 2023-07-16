
   Compiling clippy v0.0.134 (https://github.com/Manishearth/rust-clippy.git#dd905150)                                                                                                                                                                                                      
error: internal compiler error: unexpected panic                                                                                                                                                                                                                                            
                                                                                                                                                                                                                                                                                            
note: the compiler unexpectedly panicked. this is a bug.                                                                                                                                                                                                                                    
                                                                                                                                                                                                                                                                                            
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports                             

note: run with `RUST_BACKTRACE=1` for a backtrace                      

thread 'rustc' panicked at 'already have hash for FileMap(DefId { krate: CrateNum(62), node: DefIndex(0) => cargo_metadata/6da37b3662e680ee62d183ed0639e271 }, "/home/jozias/<proc-macro source code>")', /checkout/src/librustc_incremental/persist/hash.rs:254                            
stack backtrace:                                                       
   0:     0x7fe8f1fb8953 - std::sys::imp::backtrace::tracing::imp::unwind_backtrace::h5301f9f0d14e6b67                                        
                               at /checkout/src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:49                                                 
   1:     0x7fe8f1fb391a - std::sys_common::backtrace::_print::h65c3509ebfb6cbe2                                                              
                               at /checkout/src/libstd/sys_common/backtrace.rs:71                                                             
   2:     0x7fe8f1fc771a - std::panicking::default_hook::{{closure}}::hf5d8886edb67784b                                                       
                               at /checkout/src/libstd/sys_common/backtrace.rs:60                                                             
                               at /checkout/src/libstd/panicking.rs:355
   3:     0x7fe8f1fc72bb - std::panicking::default_hook::h755dd67224d34962                                                                    
                               at /checkout/src/libstd/panicking.rs:365
   4:     0x7fe8f1fc7b2b - std::panicking::rust_panic_with_hook::haea0fb9c3665ef73                                                            
                               at /checkout/src/libstd/panicking.rs:549
   5:     0x7fe8f1fc79b4 - std::panicking::begin_panic::hda25ba764f58caf0                                                                     
                               at /checkout/src/libstd/panicking.rs:511
   6:     0x7fe8f1fc7939 - std::panicking::begin_panic_fmt::h6b075a9283828bd5                                                                 
                               at /checkout/src/libstd/panicking.rs:495
   7:     0x7fe8ef51f9f4 - rustc_incremental::persist::hash::HashContext::load_data::hae1f3fc2ccf17866                                        
   8:     0x7fe8ef51dada - rustc_incremental::persist::hash::HashContext::hash::h1232829f2d1e3e5d                                             
   9:     0x7fe8ef531d49 - rustc_incremental::persist::save::save_dep_graph::h8e9814f873d08e7f                                                
  10:     0x7fe8f238b85a - rustc_driver::driver::phase_4_translate_to_llvm::hb14a1476b0c4dc8c                                                 
  11:     0x7fe8f235677b - rustc_driver::driver::compile_input::{{closure}}::h0687853991e0d748                                                
  12:     0x7fe8f2389afc - rustc_driver::driver::phase_3_run_analysis_passes::{{closure}}::h84f0c6a235da6c1e                                  
  13:     0x7fe8f236a9b0 - rustc_driver::driver::phase_3_run_analysis_passes::h6f3ddf052db34356                                               
  14:     0x7fe8f235431c - rustc_driver::driver::compile_input::he6e8d6b966d9c6e4                                                             
  15:     0x7fe8f239b330 - rustc_driver::run_compiler::h3831281dea1e4e2e                                                                      
  16:     0x7fe8f22a522b - std::sys_common::backtrace::__rust_begin_short_backtrace::ha0f945ccf2b93dc0                                        
  17:     0x7fe8f1fd0e7a - __rust_maybe_catch_panic                    
                               at /checkout/src/libpanic_unwind/lib.rs:98                                                                     
  18:     0x7fe8f22d8510 - <F as alloc::boxed::FnBox<A>>::call_box::hc6e0e3dbbddf221c                                                         
  19:     0x7fe8f1fc6525 - std::sys::imp::thread::Thread::new::thread_start::h74d0491aca13f8bd                                                
                               at /checkout/src/liballoc/boxed.rs:658  
                               at /checkout/src/libstd/sys_common/thread.rs:21                                                                
                               at /checkout/src/libstd/sys/unix/thread.rs:84                                                                  
  20:     0x7fe8ed38f296 - start_thread                                
  21:     0x7fe8f1c7f25e - __clone                                     
  22:                0x0 - <unknown>                                   

error: failed to compile `clippy v0.0.134 (https://github.com/Manishearth/rust-clippy.git#dd905150)`, intermediate artifacts can be found at `/tmp/cargo-install.f6nCYMDsVbC0`
