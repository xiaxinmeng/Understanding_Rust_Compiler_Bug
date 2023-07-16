
incremental: session directory: 5 files hard-linked                                  
incremental: session directory: 0 files copied                                                
thread 'rustc' panicked at 'no entry found for key', src/libcore/option.rs:839:4                                                                                                                                   
stack backtrace:                                                                                                                                                                                                   
   0:     0x7fbfda67e822 - std::sys::imp::backtrace::tracing::imp::unwind_backtrace::h2bee036dd609b02b                                                                                                             
   1:     0x7fbfda67905b - std::sys_common::backtrace::_print::h2a9bd6df49be92d1                                                                                                                                   
   2:     0x7fbfda68c119 - std::panicking::default_hook::{{closure}}::hcae6c022a3505fce                       
   3:     0x7fbfda68bdf5 - std::panicking::default_hook::h2b2460f26d5a082b
   4:     0x7fbfda68c628 - std::panicking::rust_panic_with_hook::hf4116880dbeebeea
   5:     0x7fbfda68c48b - std::panicking::begin_panic::h66e7ed06c82f53e8                                                        
   6:     0x7fbfda68c3ef - std::panicking::begin_panic_fmt::h0677c8c3f1de70eb
   7:     0x7fbfda68c375 - rust_begin_unwind                                                          
   8:     0x7fbfda6a65a2 - core::panicking::panic_fmt::ha635165eed891315        
   9:     0x7fbfda6a6615 - core::option::expect_failed::ha4257b2642d22559                                                                                                                                          
  10:     0x7fbfd712f258 - rustc::dep_graph::graph::DepGraph::fingerprint_of::h9f09b217a4c2989e                                                                                                                    
  11:     0x7fbfd7a67794 - rustc_incremental::persist::dirty_clean::DirtyCleanVisitor::check_item::h08ed8b33920106ea
  12:     0x7fbfd7a769eb - rustc_incremental::persist::save::save_dep_graph::hb615f7e262903cee
  13:     0x7fbfd7f88520 - rustc_trans::base::assert_and_save_dep_graph::hf030c35a2e864247
  14:     0x7fbfd7f866e4 - rustc_trans::base::trans_crate::h71af7ef3ccb676ae              
  15:     0x7fbfd7febb53 - <rustc_trans::LlvmTransCrate as rustc_trans_utils::trans_crate::TransCrate>::trans_crate::h1f1b9a0b572dd021
  16:     0x7fbfda9f295f - rustc_driver::driver::compile_input::{{closure}}::h721acac80473dd4a                   
  17:     0x7fbfda97989e - rustc::ty::context::TyCtxt::create_and_enter::h933e6b78f81d28a4
  18:     0x7fbfda9f0d83 - rustc_driver::driver::compile_input::h02b2fda6f05ad3f7
  19:     0x7fbfdaa0948d - rustc_driver::run_compiler::he5d0cfda82a2592f
  20:     0x7fbfda92a691 - std::sys_common::backtrace::__rust_begin_short_backtrace::h898eaaf417b02194
  21:     0x7fbfda695bf7 - __rust_maybe_catch_panic                       
  22:     0x7fbfda9658f0 - <F as alloc::boxed::FnBox<A>>::call_box::h79e40ba85b627dc8
  23:     0x7fbfda68afba - std::sys::imp::thread::Thread::new::thread_start::hc30eb3356e75323f
  24:     0x7fbfd54ac089 - start_thread                                                       
  25:     0x7fbfda34d24e - __clone                                                                                                                                                                                   26:                0x0 - <unknown>
