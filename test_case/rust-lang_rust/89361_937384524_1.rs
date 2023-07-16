
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', /rustc/25ec8273855fde2d72ae877b397e054de5300e10/compiler/rustc_hir/src/definitions.rs:452:14                         
stack backtrace:                                                                               
   0:     0x7f3021e332ec - std::backtrace_rs::backtrace::libunwind::trace::h49c840f3a952c2eb
                               at /rustc/25ec8273855fde2d72ae877b397e054de5300e10/library/std/src/../../backtrace/src/backtrace/libunwind.rs:90:5                                              
   1:     0x7f3021e332ec - std::backtrace_rs::backtrace::trace_unsynchronized::h970fbed62f2c306d                                                                                               
                               at /rustc/25ec8273855fde2d72ae877b397e054de5300e10/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5                                                    
   2:     0x7f3021e332ec - std::sys_common::backtrace::_print_fmt::h41e6dcef610013be                                                                                                           
                               at /rustc/25ec8273855fde2d72ae877b397e054de5300e10/library/std/src/sys_common/backtrace.rs:67:5                                                                 
   3:     0x7f3021e332ec - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h47eb9d690c7ee406                                                                
                               at /rustc/25ec8273855fde2d72ae877b397e054de5300e10/library/std/src/sys_common/backtrace.rs:46:22                                                                
   4:     0x7f3021e90bcc - core::fmt::write::h72801a82c94e6ff1                                                                                                                                 
                               at /rustc/25ec8273855fde2d72ae877b397e054de5300e10/library/core/src/fmt/mod.rs:1162:17                                                                          
   5:     0x7f3021e23e05 - std::io::Write::write_fmt::hd6b82cb7eca8de25
                               at /rustc/25ec8273855fde2d72ae877b397e054de5300e10/library/std/src/io/mod.rs:1684:15                                                                            
   6:     0x7f3021e36560 - std::sys_common::backtrace::_print::hb52aa722c5512ecd
                               at /rustc/25ec8273855fde2d72ae877b397e054de5300e10/library/std/src/sys_common/backtrace.rs:49:5                                                                 
   7:     0x7f3021e36560 - std::sys_common::backtrace::print::he2e1e8a2ae490cb5
                               at /rustc/25ec8273855fde2d72ae877b397e054de5300e10/library/std/src/sys_common/backtrace.rs:36:9                                                                 
   8:     0x7f3021e36560 - std::panicking::default_hook::{{closure}}::h456b8dc7e1036ef7 
                               at /rustc/25ec8273855fde2d72ae877b397e054de5300e10/library/std/src/panicking.rs:210:50                                                                          
   9:     0x7f3021e3610b - std::panicking::default_hook::h112e1a03bb7f7411              
                               at /rustc/25ec8273855fde2d72ae877b397e054de5300e10/library/std/src/panicking.rs:227:9                                                                           
  10:     0x7f3022650c61 - rustc_driver::DEFAULT_HOOK::{{closure}}::{{closure}}::h4b4c6f10efa3d667                                                                                             
  11:     0x7f2ffc622a8b - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::h51f913132d10fc93                                                                                  
                               at /rustc/25ec8273855fde2d72ae877b397e054de5300e10/library/alloc/src/boxed.rs:1652:9                                                                            
  12:     0x7f2ffc622a8b - proc_macro::bridge::client::<impl proc_macro::bridge::Bridge>::enter::{{closure}}::{{closure}}::h411d854584c329f5                                                   
                               at /rustc/25ec8273855fde2d72ae877b397e054de5300e10/library/proc_macro/src/bridge/client.rs:320:21                                                               
  13:     0x7f3021e36d79 - std::panicking::rust_panic_with_hook::h9d04d157d3d966dc
                               at /rustc/25ec8273855fde2d72ae877b397e054de5300e10/library/std/src/panicking.rs:628:17                                                                          
  14:     0x7f3021e36802 - std::panicking::begin_panic_handler::{{closure}}::heb8583e1f0ca0bb3                                                                                                 
                               at /rustc/25ec8273855fde2d72ae877b397e054de5300e10/library/std/src/panicking.rs:519:13                                                                          
  15:     0x7f3021e337a4 - std::sys_common::backtrace::__rust_end_short_backtrace::hf7f7df37cc03b639                                                                                           
                               at /rustc/25ec8273855fde2d72ae877b397e054de5300e10/library/std/src/sys_common/backtrace.rs:141:18 
  16:     0x7f3021e36799 - rust_begin_unwind   
                               at /rustc/25ec8273855fde2d72ae877b397e054de5300e10/library/std/src/panicking.rs:517:5
  17:     0x7f3021dfc4f1 - core::panicking::panic_fmt::h7b8580d81fcbbacd
                               at /rustc/25ec8273855fde2d72ae877b397e054de5300e10/library/core/src/panicking.rs:100:14
  18:     0x7f3021dfc43d - core::panicking::panic::h50b51d19800453c0
                               at /rustc/25ec8273855fde2d72ae877b397e054de5300e10/library/core/src/panicking.rs:50:5
  19:     0x7f3023fe5237 - <rustc_query_impl::on_disk_cache::OnDiskCache as rustc_middle::ty::context::OnDiskCache>::def_path_hash_to_def_id::hc003ddf41aae4dd3
  20:     0x7f30244a12b1 - rustc_middle::dep_graph::dep_node::<impl rustc_query_system::dep_graph::dep_node::DepNodeParams<rustc_middle::ty::context::TyCtxt> for rustc_span::def_id::LocalDefI
d>::recover::h145032126081f004
  21:     0x7f3024aaa4ec - rustc_query_impl::query_callbacks::hir_owner::force_from_dep_node::h476eb9e8974ac027
  22:     0x7f3023fc5483 - rustc_query_system::dep_graph::graph::DepGraph<K>::try_mark_previous_green::h0a597ce211b4655d
  23:     0x7f3023fc4f8a - rustc_query_system::dep_graph::graph::DepGraph<K>::try_mark_previous_green::h0a597ce211b4655d
  24:     0x7f3024ab5d31 - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::try_mark_green::h13792bc06ebd8d7f
  25:     0x7f3024b73e3b - rustc_codegen_ssa::base::determine_cgu_reuse::hb06d9b8284b4d95f
  26:     0x7f302458ef59 - <alloc::vec::Vec<T> as alloc::vec::spec_from_iter::SpecFromIter<T,I>>::from_iter::h4c7b771af7b02f5f
  27:     0x7f30245c3170 - rustc_session::utils::<impl rustc_session::session::Session>::time::ha74abf1799ccf45f
  28:     0x7f30245a62f0 - rustc_codegen_ssa::base::codegen_crate::hd0da505449289dd5
  29:     0x7f30245cab5a - <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate::h5376336857a45526
  30:     0x7f3024575517 - rustc_session::utils::<impl rustc_session::session::Session>::time::ha5e95f6c912fa8bd
  31:     0x7f302454ff99 - rustc_interface::queries::Queries::ongoing_codegen::hbf81f4412ce3471d
  32:     0x7f302452f639 - rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter::hb21794c7afdd7aa9
  33:     0x7f302451fc4f - rustc_span::with_source_map::h1577f9979c3dd3cd
  34:     0x7f302452ee40 - scoped_tls::ScopedKey<T>::set::h1a43900433c14947
  35:     0x7f3024520395 - std::sys_common::backtrace::__rust_begin_short_backtrace::hda517dd2e82ac21b
  36:     0x7f3024542632 - core::ops::function::FnOnce::call_once{{vtable.shim}}::h32907797a154493d
  37:     0x7f3021e42083 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h3a82802338f49654
                               at /rustc/25ec8273855fde2d72ae877b397e054de5300e10/library/alloc/src/boxed.rs:1638:9
  38:     0x7f3021e42083 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hd13a4d0f57fcdf47
                               at /rustc/25ec8273855fde2d72ae877b397e054de5300e10/library/alloc/src/boxed.rs:1638:9
  39:     0x7f3021e42083 - std::sys::unix::thread::Thread::new::thread_start::hee01618890f121bb 
                               at /rustc/25ec8273855fde2d72ae877b397e054de5300e10/library/std/src/sys/unix/thread.rs:106:17
  40:     0x7f3021d81d3e - start_thread
  41:     0x7f3021c9c43f - __clone
  42:                0x0 - <unknown>

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.57.0-nightly (25ec82738 2021-10-05) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z share-generics=y -C embed-bitcode=no -C debuginfo=2 -C linker=clang -C incremental -C link-arg=-fuse-ld=lld

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack

