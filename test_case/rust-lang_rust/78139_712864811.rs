
[INFO] [stderr]    Compiling dilbert-feed v0.2.1 (/opt/rustwide/workdir)
[INFO] [stderr] thread 'rustc' panicked at 'Box<Any>', compiler/rustc_errors/src/lib.rs:945:9
[INFO] [stdout] error: internal compiler error: compiler/rustc_traits/src/normalize_erasing_regions.rs:37:32: could not fully normalize `<tokio::prelude::future::Either<tokio::prelude::future::Map<tokio::prelude::future::AndThen<tokio::prelude::future::AndThen<tokio::prelude::future::FutureResult<std::string::String, failure::Error>, std::result::Result<hyper::Uri, failure::Error>, [closure@src/main.rs:162:51: 162:96]>, tokio::prelude::future::Map<tokio::prelude::future::FromErr<tokio::prelude::future::AndThen<hyper::client::ResponseFuture, tokio::prelude::future::Map<tokio::prelude::stream::Concat2<hyper::Body>, [closure@src/main.rs:46:40: 46:62]>, fn(hyper::Response<hyper::Body>) -> impl hyper::rt::Future {concat_body}>, failure::Error>, [closure@src/main.rs:91:14: 95:10]>, [closure@src/main.rs:163:51: 163:92]>, [closure@src/main.rs:164:46: 167:42]>, tokio::prelude::future::FutureResult<ComicInfo, failure::Error>> as tokio::prelude::IntoFuture>::Future`
[INFO] [stdout] 
[INFO] [stdout] 
[INFO] [stderr] stack backtrace:
[INFO] [stderr]    0:     0x7f495f236410 - std::backtrace_rs::backtrace::libunwind::trace::h7cf3ca97bd83fbab
[INFO] [stderr]                                at /rustc/212e76c31d461462544b5d516d3bdc99fc8f016e/library/std/src/../../backtrace/src/backtrace/libunwind.rs:96
[INFO] [stderr]    1:     0x7f495f236410 - std::backtrace_rs::backtrace::trace_unsynchronized::h5a69c674b2d9ea5c
[INFO] [stderr]                                at /rustc/212e76c31d461462544b5d516d3bdc99fc8f016e/library/std/src/../../backtrace/src/backtrace/mod.rs:66
[INFO] [stderr]    2:     0x7f495f236410 - std::sys_common::backtrace::_print_fmt::hdf72478dfcb50678
[INFO] [stderr]                                at /rustc/212e76c31d461462544b5d516d3bdc99fc8f016e/library/std/src/sys_common/backtrace.rs:79
[INFO] [stderr]    3:     0x7f495f236410 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h1e0bdbb35e83deca
[INFO] [stderr]                                at /rustc/212e76c31d461462544b5d516d3bdc99fc8f016e/library/std/src/sys_common/backtrace.rs:58
[INFO] [stderr]    4:     0x7f495f2a45cc - core::fmt::write::he67f9b17bcb7ee73
[INFO] [stderr]                                at /rustc/212e76c31d461462544b5d516d3bdc99fc8f016e/library/core/src/fmt/mod.rs:1080
[INFO] [stderr]    5:     0x7f495f228072 - std::io::Write::write_fmt::hac97d5cb5753e2ec
[INFO] [stderr]                                at /rustc/212e76c31d461462544b5d516d3bdc99fc8f016e/library/std/src/io/mod.rs:1516
[INFO] [stderr]    6:     0x7f495f23b18d - std::sys_common::backtrace::_print::haa20918e28a126c5
[INFO] [stderr]                                at /rustc/212e76c31d461462544b5d516d3bdc99fc8f016e/library/std/src/sys_common/backtrace.rs:61
[INFO] [stderr]    7:     0x7f495f23b18d - std::sys_common::backtrace::print::h49e772a4873d9076
[INFO] [stderr]                                at /rustc/212e76c31d461462544b5d516d3bdc99fc8f016e/library/std/src/sys_common/backtrace.rs:48
[INFO] [stderr]    8:     0x7f495f23b18d - std::panicking::default_hook::{{closure}}::hb218b305d15e3f11
[INFO] [stderr]                                at /rustc/212e76c31d461462544b5d516d3bdc99fc8f016e/library/std/src/panicking.rs:208
[INFO] [stderr]    9:     0x7f495f23ae38 - std::panicking::default_hook::h63997b7b949e1ce0
[INFO] [stderr]                                at /rustc/212e76c31d461462544b5d516d3bdc99fc8f016e/library/std/src/panicking.rs:227
[INFO] [stderr]   10:     0x7f495fac40b4 - rustc_driver::report_ice::h5b9fb8474972c25e
[INFO] [stderr]   11:     0x7f494ee38076 - <alloc::boxed::Box<F> as core::ops::function::Fn<A>>::call::h28cbcde4fa87c883
[INFO] [stderr]                                at /rustc/212e76c31d461462544b5d516d3bdc99fc8f016e/library/alloc/src/boxed.rs:1056
[INFO] [stderr]   12:     0x7f494ee1c0cb - proc_macro::bridge::client::<impl proc_macro::bridge::Bridge>::enter::{{closure}}::{{closure}}::h0b15805078ae0080
[INFO] [stderr]                                at /rustc/212e76c31d461462544b5d516d3bdc99fc8f016e/library/proc_macro/src/bridge/client.rs:320
[INFO] [stderr]   13:     0x7f495f23b9d6 - std::panicking::rust_panic_with_hook::hb54f09ccadf64bd3
[INFO] [stderr]                                at /rustc/212e76c31d461462544b5d516d3bdc99fc8f016e/library/std/src/panicking.rs:581
[INFO] [stderr]   14:     0x7f4962b4ab0d - std::panicking::begin_panic::{{closure}}::hf63b06b6f25b2bdf
[INFO] [stderr]   15:     0x7f4962b4aa66 - std::sys_common::backtrace::__rust_end_short_backtrace::h4a10ac286d57b878
[INFO] [stderr]   16:     0x7f4962b509df - std::panicking::begin_panic::hce65d2b360d5b6cd
[INFO] [stderr]   17:     0x7f4962b88a20 - rustc_errors::HandlerInner::bug::h6efa9d026e354399
[INFO] [stderr]   18:     0x7f4962b87500 - rustc_errors::Handler::bug::h65ab0cb6919f7f68
[INFO] [stderr]   19:     0x7f496249e4a4 - rustc_middle::util::bug::opt_span_bug_fmt::{{closure}}::hece8a2fab170b9d8
[INFO] [stderr]   20:     0x7f4962497afb - rustc_middle::ty::context::tls::with_opt::{{closure}}::he2d94fe1646e47bf
[INFO] [stderr]   21:     0x7f4962497472 - rustc_middle::ty::context::tls::with_opt::h5babe8f0efdba1b5
[INFO] [stderr]   22:     0x7f496249e3c9 - rustc_middle::util::bug::opt_span_bug_fmt::heb60f98da3d119cb
[INFO] [stderr]   23:     0x7f496249e33e - rustc_middle::util::bug::bug_fmt::hc7a67a8c724be98c
[INFO] [stderr]   24:     0x7f49601ff06a - rustc_infer::infer::InferCtxtBuilder::enter::hdcfa59718b739dd1
[INFO] [stderr]   25:     0x7f496023e7d4 - rustc_traits::normalize_erasing_regions::normalize_generic_arg_after_erasing_regions::h4faa5ca4fd539b92
[INFO] [stderr]   26:     0x7f496239f7a2 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::hc9b7506c8b93697c
[INFO] [stderr]   27:     0x7f49624c93c8 - rustc_data_structures::stack::ensure_sufficient_stack::hd9ab99097ffe3d5e
[INFO] [stderr]   28:     0x7f496296b720 - rustc_query_system::query::plumbing::get_query_impl::hd34aababc4489571
[INFO] [stderr]   29:     0x7f496289058b - <rustc_middle::ty::normalize_erasing_regions::NormalizeAfterErasingRegionsFolder as rustc_middle::ty::fold::TypeFolder>::fold_ty::he529649e2980b5dd
[INFO] [stderr]   30:     0x7f4960167275 - rustc_ty::needs_drop::needs_drop_raw::hd417eb508c68d13b
[INFO] [stderr]   31:     0x7f49613b062a - rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::needs_drop_raw>::compute::h0dbd8a48d8398db8
[INFO] [stderr]   32:     0x7f496118ae64 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::h1251534d91b81268
[INFO] [stderr]   33:     0x7f49611aaeb4 - rustc_data_structures::stack::ensure_sufficient_stack::h5274271eaf9e5286
[INFO] [stderr]   34:     0x7f496107b8d0 - rustc_query_system::query::plumbing::get_query_impl::hf908e02f3102192c
[INFO] [stderr]   35:     0x7f49613ab037 - rustc_middle::ty::util::<impl rustc_middle::ty::TyS>::needs_drop::h656f3b489dc02bfb
[INFO] [stderr]   36:     0x7f49613d227a - rustc_mir::interpret::intrinsics::eval_nullary_intrinsic::hee586a8fdec5b2f1
[INFO] [stderr]   37:     0x7f49611d222c - rustc_mir::const_eval::eval_queries::eval_to_const_value_raw_provider::h50fca309bc380e76
[INFO] [stderr]   38:     0x7f4961170f64 - rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::eval_to_const_value_raw>::compute::hdec9e8fd7d16a2a0
[INFO] [stderr]   39:     0x7f496119dd0a - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::he380ffdd791a99b1
[INFO] [stderr]   40:     0x7f49611af57d - rustc_data_structures::stack::ensure_sufficient_stack::h8a026875ee2a98c2
[INFO] [stderr]   41:     0x7f4961037167 - rustc_query_system::query::plumbing::get_query_impl::h827a4c4640c690f5
[INFO] [stderr]   42:     0x7f49611d21c9 - rustc_mir::const_eval::eval_queries::eval_to_const_value_raw_provider::h50fca309bc380e76
[INFO] [stderr]   43:     0x7f49625bf59a - rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::eval_to_const_value_raw>::compute::hdec9e8fd7d16a2a0
[INFO] [stderr]   44:     0x7f4962376232 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::h0da45621f956e501
[INFO] [stderr]   45:     0x7f49624c6fab - rustc_data_structures::stack::ensure_sufficient_stack::hc1b74219642dfd11
[INFO] [stderr]   46:     0x7f49628bdef4 - rustc_query_system::query::plumbing::get_query_impl::h05a7bc00404321f6
[INFO] [stderr]   47:     0x7f4962586282 - rustc_middle::mir::interpret::queries::<impl rustc_middle::ty::context::TyCtxt>::const_eval_global_id::he6317e65dda8742f
[INFO] [stderr]   48:     0x7f496258604e - rustc_middle::mir::interpret::queries::<impl rustc_middle::ty::context::TyCtxt>::const_eval_instance::h08ec2e26f3402a63
[INFO] [stderr]   49:     0x7f495fe9f6b7 - rustc_codegen_ssa::mir::intrinsic::<impl rustc_codegen_ssa::mir::FunctionCx<Bx>>::codegen_intrinsic_call::h28c5ed91586afe47
[INFO] [stderr]   50:     0x7f495fe9589d - rustc_codegen_ssa::mir::block::<impl rustc_codegen_ssa::mir::FunctionCx<Bx>>::codegen_call_terminator::hcab9d1205df20a31
[INFO] [stderr]   51:     0x7f495fe903a6 - rustc_codegen_ssa::mir::block::<impl rustc_codegen_ssa::mir::FunctionCx<Bx>>::codegen_block::h62948860027c86bc
[INFO] [stderr]   52:     0x7f495fe8ddbe - rustc_codegen_ssa::mir::codegen_mir::h28a81876122b9b75
[INFO] [stderr]   53:     0x7f495ff9f868 - rustc_codegen_ssa::base::codegen_instance::haac9416663ce073f
[INFO] [stderr]   54:     0x7f495fdf42a4 - <rustc_middle::mir::mono::MonoItem as rustc_codegen_ssa::mono_item::MonoItemExt>::define::h14112b0e0302d0b6
[INFO] [stderr]   55:     0x7f495fe33b1b - rustc_codegen_llvm::base::compile_codegen_unit::module_codegen::h392634f7370ae0df
[INFO] [stderr]   56:     0x7f495fef323a - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task::hab6ab1281f48d446
[INFO] [stderr]   57:     0x7f495fe3363a - rustc_codegen_llvm::base::compile_codegen_unit::h66a62c9d61aa9cb1
[INFO] [stderr]   58:     0x7f495ff9d8c5 - rustc_codegen_ssa::base::codegen_crate::h0f310b8e6539577c
[INFO] [stderr]   59:     0x7f495ff937e5 - <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate::hf0175e4b8c8a4032
[INFO] [stderr]   60:     0x7f495fd08d10 - rustc_session::utils::<impl rustc_session::session::Session>::time::h2a17f1881818a252
[INFO] [stderr]   61:     0x7f495fd5083c - rustc_interface::passes::QueryContext::enter::h23a2ddc99d800d56
[INFO] [stderr]   62:     0x7f495fd7c253 - rustc_interface::queries::Queries::ongoing_codegen::h2c31f6799bcf60e3
[INFO] [stderr]   63:     0x7f495fb128e4 - rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter::had791f63f727c367
[INFO] [stderr]   64:     0x7f495fae7a67 - rustc_span::with_source_map::h755817392b17a216
[INFO] [stderr]   65:     0x7f495fb0eac1 - scoped_tls::ScopedKey<T>::set::he59500e39ec4abad
[INFO] [stderr]   66:     0x7f495fb15e21 - std::sys_common::backtrace::__rust_begin_short_backtrace::hd47de080afaa24c1
[INFO] [stderr]   67:     0x7f495fa90f4e - core::ops::function::FnOnce::call_once{{vtable.shim}}::hd9b6798a9391b23c
[INFO] [stderr]   68:     0x7f495f24a70a - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::hd4d30e37db281e8f
[INFO] [stderr]                                at /rustc/212e76c31d461462544b5d516d3bdc99fc8f016e/library/alloc/src/boxed.rs:1042
[INFO] [stderr]   69:     0x7f495f24a70a - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::h019185a215ba16e4
[INFO] [stderr]                                at /rustc/212e76c31d461462544b5d516d3bdc99fc8f016e/library/alloc/src/boxed.rs:1042
[INFO] [stderr]   70:     0x7f495f24a70a - std::sys::unix::thread::Thread::new::thread_start::h33c6d7cf25f9291f
[INFO] [stderr]                                at /rustc/212e76c31d461462544b5d516d3bdc99fc8f016e/library/std/src/sys/unix/thread.rs:87
[INFO] [stderr]   71:     0x7f495f166609 - start_thread
[INFO] [stderr]   72:     0x7f495f087103 - clone
[INFO] [stderr]   73:                0x0 - <unknown>
[INFO] [stderr] 
[INFO] [stderr] note: the compiler unexpectedly panicked. this is a bug.
[INFO] [stderr] 
[INFO] [stderr] note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md
[INFO] [stderr] 
[INFO] [stderr] note: rustc 1.48.0-beta.2 (212e76c31 2020-10-08) running on x86_64-unknown-linux-gnu
[INFO] [stderr] 
[INFO] [stderr] note: compiler flags: -C embed-bitcode=no -C debuginfo=2 --crate-type bin
[INFO] [stderr] 
[INFO] [stderr] note: some of the compiler flags provided by cargo are hidden
[INFO] [stderr] 
[INFO] [stderr] query stack during panic:
[INFO] [stderr] #0 [normalize_generic_arg_after_erasing_regions] normalizing `<tokio::prelude::future::Either<tokio::prelude::future::Map<tokio::prelude::future::AndThen<tokio::prelude::future::AndThen<tokio::prelude::future::FutureResult<std::string::String, failure::Error>, std::result::Result<hyper::Uri, failure::Error>, [closure@src/main.rs:162:51: 162:96]>, tokio::prelude::future::Map<tokio::prelude::future::FromErr<tokio::prelude::future::AndThen<hyper::client::ResponseFuture, tokio::prelude::future::Map<tokio::prelude::stream::Concat2<hyper::Body>, [closure@src/main.rs:46:40: 46:62]>, fn(hyper::Response<hyper::Body>) -> impl hyper::rt::Future {concat_body}>, failure::Error>, [closure@src/main.rs:91:14: 95:10]>, [closure@src/main.rs:163:51: 163:92]>, [closure@src/main.rs:164:46: 167:42]>, tokio::prelude::future::FutureResult<ComicInfo, failure::Error>> as tokio::prelude::IntoFuture>::Future`
[INFO] [stderr] #1 [needs_drop_raw] computing whether `tokio::prelude::future::Map<tokio::prelude::future::AndThen<tokio::prelude::future::AndThen<tokio::prelude::future::FromErr<tokio::prelude::future::AndThen<hyper::client::ResponseFuture, tokio::prelude::future::Map<tokio::prelude::stream::Concat2<hyper::Body>, [closure@src/main.rs:46:40: 46:62]>, fn(hyper::Response<hyper::Body>) -> impl hyper::rt::Future {concat_body}>, failure::Error>, std::result::Result<ComicInfo, failure::Error>, [closure@src/main.rs:60:19: 80:10]>, tokio::prelude::future::Either<tokio::prelude::future::Map<tokio::prelude::future::AndThen<tokio::prelude::future::AndThen<tokio::prelude::future::FutureResult<std::string::String, failure::Error>, std::result::Result<hyper::Uri, failure::Error>, [closure@src/main.rs:162:51: 162:96]>, tokio::prelude::future::Map<tokio::prelude::future::FromErr<tokio::prelude::future::AndThen<hyper::client::ResponseFuture, tokio::prelude::future::Map<tokio::prelude::stream::Concat2<hyper::Body>, [closure@src/main.rs:46:40: 46:62]>, fn(hyper::Response<hyper::Body>) -> impl hyper::rt::Future {concat_body}>, failure::Error>, [closure@src/main.rs:91:14: 95:10]>, [closure@src/main.rs:163:51: 163:92]>, [closure@src/main.rs:164:46: 167:42]>, tokio::prelude::future::FutureResult<ComicInfo, failure::Error>>, [closure@src/main.rs:154:35: 172:26]>, [closure@src/main.rs:172:32: 180:26]>` needs drop
[INFO] [stderr] #2 [eval_to_const_value_raw] simplifying constant for the type system `std::intrinsics::needs_drop`
[INFO] [stderr] #3 [eval_to_const_value_raw] simplifying constant for the type system `std::intrinsics::needs_drop`
[INFO] [stderr] end of query stack
[INFO] [stdout] error: aborting due to previous error
[INFO] [stdout] 
[INFO] [stdout] 
[INFO] [stderr] error: could not compile `dilbert-feed`
