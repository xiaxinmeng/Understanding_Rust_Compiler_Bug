
{"message":"src/librustc/ty/maps/plumbing.rs:712: force_from_dep_node() - Encountered HirBody","code":null,"level":"error: internal compiler error","spans":[],"children":[],"rendered":null}
note: the compiler unexpectedly panicked. this is a bug.                                  
                                                                                                                                      
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
                                                                                          
note: rustc 1.22.0-dev running on x86_64-unknown-linux-gnu                                                                                                                                                                                                                                                                                                                                                                            note: run with `RUST_BACKTRACE=1` for a backtrace                                                                                                                                                                                                                                                                                                                                                                                     incremental: session directory: 8 files hard-linked                                                                                                                                                                incremental: session directory: 0 files copied                                                                                                                                                                     thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:492:8                                          
stack backtrace:                                   
   0:     0x7fc01699a822 - std::sys::imp::backtrace::tracing::imp::unwind_backtrace::h2bee036dd609b02b
   1:     0x7fc01699505b - std::sys_common::backtrace::_print::h2a9bd6df49be92d1
   2:     0x7fc0169a8119 - std::panicking::default_hook::{{closure}}::hcae6c022a3505fce                            
   3:     0x7fc0169a7df5 - std::panicking::default_hook::h2b2460f26d5a082b
   4:     0x7fc0169a8628 - std::panicking::rust_panic_with_hook::hf4116880dbeebeea
   5:     0x7fc012445408 - std::panicking::begin_panic::hd4dfc6e452a13272                                                      
   6:     0x7fc012462f25 - rustc_errors::Handler::bug::hfb6778b3fbcffb64
   7:     0x7fc0135659b0 - rustc::session::opt_span_bug_fmt::{{closure}}::h9a48bc05a56034c0           
   8:     0x7fc0135658c1 - rustc::session::opt_span_bug_fmt::h1ace49cd7b33ef98  
   9:     0x7fc013565596 - rustc::session::bug_fmt::h843af0f745f1a8a0                  
  10:     0x7fc0135e2f16 - rustc::ty::maps::plumbing::force_from_dep_node::hbfa0094c577c659d
  11:     0x7fc013d838c3 - rustc_incremental::persist::dirty_clean::DirtyCleanVisitor::check_item::h08ed8b33920106ea
  12:     0x7fc013d926eb - rustc_incremental::persist::save::save_dep_graph::hb615f7e262903cee
  13:     0x7fc0142a4520 - rustc_trans::base::assert_and_save_dep_graph::hf030c35a2e864247
  14:     0x7fc0142a26e4 - rustc_trans::base::trans_crate::h71af7ef3ccb676ae              
  15:     0x7fc014307b53 - <rustc_trans::LlvmTransCrate as rustc_trans_utils::trans_crate::TransCrate>::trans_crate::h1f1b9a0b572dd021
  16:     0x7fc016d0e95f - rustc_driver::driver::compile_input::{{closure}}::h721acac80473dd4a
  17:     0x7fc016c9589e - rustc::ty::context::TyCtxt::create_and_enter::h933e6b78f81d28a4
  18:     0x7fc016d0cd83 - rustc_driver::driver::compile_input::h02b2fda6f05ad3f7                                                                                                                                  
  19:     0x7fc016d2548d - rustc_driver::run_compiler::he5d0cfda82a2592f                                                                                                                                           
  20:     0x7fc016c46691 - std::sys_common::backtrace::__rust_begin_short_backtrace::h898eaaf417b02194                                                                                                             
  21:     0x7fc0169b1bf7 - __rust_maybe_catch_panic                                                                                                                                                                
  22:     0x7fc016c818f0 - <F as alloc::boxed::FnBox<A>>::call_box::h79e40ba85b627dc8                                                                                                                              
  23:     0x7fc0169a6fba - std::sys::imp::thread::Thread::new::thread_start::hc30eb3356e75323f                                                                                                                     
  24:     0x7fc0117c8089 - start_thread                                                                          
  25:     0x7fc01666924e - __clone     

