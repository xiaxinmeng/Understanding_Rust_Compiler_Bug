text
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
             at libstd/panicking.rs:479                                                                                
   6: std::panicking::continue_panic_fmt                                                                               
             at libstd/panicking.rs:390                                                                                
   7: rust_begin_unwind                                                                                                
             at libstd/panicking.rs:325                                                                                
   8: core::panicking::panic_fmt                                                                                       
             at libcore/panicking.rs:77                                                                                
   9: core::panicking::panic_bounds_check                                                                              
             at libcore/panicking.rs:59                                                                                
  10: <rustc::ty::query::on_disk_cache::CacheDecoder<'a, 'tcx, 'x> as serialize::serialize::SpecializedDecoder<syntax_p
os::span_encoding::Span>>::specialized_decode                                                                          
  11: serialize::serialize::Decoder::read_struct                                                                       
  12: serialize::serialize::Decoder::read_seq                                                                          
  13: serialize::serialize::Decoder::read_struct                                                                       
  14: serialize::serialize::Decoder::read_seq                                                                          
  15: <rustc::mir::Mir<'tcx> as serialize::serialize::Decodable>::decode::{{closure}}                                  
  16: rustc::ty::query::on_disk_cache::OnDiskCache::try_load_query_result                                              
  17: <rustc::ty::query::queries::optimized_mir<'tcx> as rustc::ty::query::config::QueryDescription<'tcx>>::try_load_fr
om_disk                                                                                                                
  18: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::try_get_query                     
  19: rustc::ty::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::instance_mir                                       
  20: rustc_mir::monomorphize::collector::collect_items_rec                                                            
  21: rustc_mir::monomorphize::collector::collect_items_rec                                                            
  22: rustc_mir::monomorphize::collector::collect_items_rec
  23: rustc_mir::monomorphize::collector::collect_items_rec
  24: rustc_mir::monomorphize::collector::collect_crate_mono_items::{{closure}}                                       
  25: rustc::util::common::time
  26: rustc_mir::monomorphize::collector::collect_crate_mono_items
  27: rustc::util::common::time
  28: rustc_codegen_llvm::base::collect_and_partition_mono_items
  29: rustc::ty::context::tls::with_context
  30: <rustc::ty::query::plumbing::JobOwner<'a, 'tcx, Q>>::start
  31: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query
  32: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_utils::codegen_backend::CodegenBackend>::codegen_crate
  33: rustc::util::common::time
  34: rustc_driver::driver::phase_4_codegen
  35: rustc_driver::driver::compile_input::{{closure}}
  36: rustc::ty::context::tls::enter_context
  37: <std::thread::local::LocalKey<T>>::with
  38: rustc::ty::context::TyCtxt::create_and_enter
  39: rustc_driver::driver::compile_input
  40: rustc_driver::run_compiler_with_pool
  41: syntax::with_globals
  42: <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
  43: __rust_maybe_catch_panic
             at libpanic_unwind/lib.rs:102
  44: rustc_driver::run
  45: rustc_driver::main
  46: std::rt::lang_start::{{closure}}
  47: std::panicking::try::do_call
             at libstd/rt.rs:59
             at libstd/panicking.rs:310
  48: __rust_maybe_catch_panic
             at libpanic_unwind/lib.rs:102
  49: std::rt::lang_start_internal
             at libstd/panicking.rs:289
             at libstd/panic.rs:392
             at libstd/rt.rs:58
  50: main
  51: __libc_start_main
  52: <unknown>
query stack during panic:
#0 [collect_and_partition_mono_items] collect_and_partition_mono_items
end of query stack

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports     

note: rustc 1.30.0-nightly (33b923fd4 2018-08-18) running on x86_64-unknown-linux-gnu                                 

note: compiler flags: -C debuginfo=2 -C incremental

note: some of the compiler flags provided by cargo are hidden          
