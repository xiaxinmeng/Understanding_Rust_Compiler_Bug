
  left: `true`,                                                                                         
 right: `false`', libstd/net/ip.rs:1946:13                                                              
stack backtrace:                                           
   0:     0x55d5563891cf - std::sys::unix::backtrace::tracing::imp::unwind_backtrace::h94bf54f902935668 
   1:     0x55d55635d9f7 - std::sys_common::backtrace::print::h29623c1aa060fc2f                         
   2:     0x55d5563f3e2f - std::panicking::default_hook::{{closure}}::h131049a1fc0672a2                 
   3:     0x55d5563f45ad - std::panicking::rust_panic_with_hook::h61610c5f8013cd16                      
   4:     0x55d5563f3fd0 - std::panicking::continue_panic_fmt::h721927dbbb0e40f5                        
   5:     0x55d5563f3f1e - std::panicking::begin_panic_fmt::h81505cc781e6dfe2                           
   6:     0x55d5563fb005 - std::net::ip::tests::ip_properties::check6::hf70d0d1be4e61588                
   7:     0x55d5563fa5af - std::net::ip::tests::ip_properties::h6be1fabe5a9bc980                        
   8:     0x7f281962949e - <F as alloc::boxed::FnBox<A>>::call_box::h94f7e1459d039765                   
   9:     0x7f2819583819 - __rust_maybe_catch_panic                                                     
  10:     0x7f281961797d - std::sys_common::backtrace::__rust_begin_short_backtrace::h4b5c31c858f402b8  
  11:     0x7f281960db84 - std::panicking::try::do_call::h2f9e4885c52d06ae
  12:     0x7f2819583819 - __rust_maybe_catch_panic                                                     
  13:     0x7f281960912c - <F as alloc::boxed::FnBox<A>>::call_box::h6396d3a5d65778c6                   
  14:     0x7f281954c0ad - std::sys_common::thread::start_thread::h8809e36f95be609f                     
  15:     0x7f2819578d85 - std::sys::unix::thread::Thread::new::thread_start::hda87df3389a6b90f
  16:     0x7f2819660a9c - start_thread
  17:     0x7f281940cb22 - clone
  18:                0x0 - <unknown>
