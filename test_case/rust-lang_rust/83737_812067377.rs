

error: internal compiler error: compiler/rustc_mir/src/transform/generator.rs:756:13: Broken MIR: generator contains type impl futures::Future in MIR, but typeck only knows about for<'r, 's, 't0, 't1, 't2, 't3, 't4, 't5> {ResumeTy, ServiceFs<ServiceObjLocal<'r, ControlRequestStream>>, impl futures::Future, (), Backlight, Arc<futures::lock::Mutex<Backlight>>, impl futures::Future, Sensor, Arc<futures::lock::Mutex<Sensor>>, SenderChannel<f32>, Arc<futures::lock::Mutex<SenderChannel<f32>>>, SenderChannel<bool>, Arc<futures::lock::Mutex<SenderChannel<bool>>>, Arc<futures::lock::Mutex<(dyn SensorControl + 's)>>, Arc<futures::lock::Mutex<(dyn BacklightControl + 't0)>>, impl futures::Future, Control, Arc<futures::lock::Mutex<Control>>, fn(ControlRequestStream, Arc<futures::lock::Mutex<(dyn ControlTrait + 't1)>>) -> Pin<Box<(dyn futures::Future<Output = Result<(), anyhow::Error>> + 't2)>>, Arc<futures::lock::Mutex<(dyn ControlTrait + 't3)>>, impl futures::Future}
   --> ../../src/ui/bin/brightness_manager/src/main.rs:154:10
    |
154 | async fn main() -> Result<(), Error> {
    |          ^^^^

thread 'rustc' panicked at 'Box<Any>', /usr/local/google/home/richkadel/rust/library/std/src/panic.rs:59:5
stack backtrace:
   0:     0x7f6ad13436c0 - std::backtrace_rs::backtrace::libunwind::trace::h3070b301bb5ab74e
                               at /usr/local/google/home/richkadel/rust/library/std/src/../../backtrace/src/backtrace/libunwind.rs:90:5
   1:     0x7f6ad13436c0 - std::backtrace_rs::backtrace::trace_unsynchronized::h7dc08349d0fccbdb
                               at /usr/local/google/home/richkadel/rust/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7f6ad13436c0 - std::sys_common::backtrace::_print_fmt::hf0a8b45a4142c4c8
                               at /usr/local/google/home/richkadel/rust/library/std/src/sys_common/backtrace.rs:67:5
   3:     0x7f6ad13436c0 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h9fcd8c5a937d304b
                               at /usr/local/google/home/richkadel/rust/library/std/src/sys_common/backtrace.rs:46:22
   4:     0x7f6ad13be2ef - core::fmt::write::h66ce6b9f5c3dd1ae
                               at /usr/local/google/home/richkadel/rust/library/core/src/fmt/mod.rs:1094:17
   5:     0x7f6ad1337d25 - std::io::Write::write_fmt::h73b66a0a6c7e21cb
                               at /usr/local/google/home/richkadel/rust/library/std/src/io/mod.rs:1578:15
   6:     0x7f6ad13472fb - std::sys_common::backtrace::_print::h7235368ebf13bc36
                               at /usr/local/google/home/richkadel/rust/library/std/src/sys_common/backtrace.rs:49:5
   7:     0x7f6ad13472fb - std::sys_common::backtrace::print::h6646fe3fd64eb87e
                               at /usr/local/google/home/richkadel/rust/library/std/src/sys_common/backtrace.rs:36:9
   8:     0x7f6ad13472fb - std::panicking::default_hook::{{closure}}::h9d50014b1a836df3
                               at /usr/local/google/home/richkadel/rust/library/std/src/panicking.rs:208:50
   9:     0x7f6ad1346ddd - std::panicking::default_hook::h0dc17d3ed8af201e
                               at /usr/local/google/home/richkadel/rust/library/std/src/panicking.rs:225:9
  10:     0x7f6ad2489f08 - rustc_driver::report_ice::hf9ac142861742502
  11:     0x7f6ab16df8d3 - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::h76ac0e733404f6ba
                               at /usr/local/google/home/richkadel/rust/library/alloc/src/boxed.rs:1560:9
  12:     0x7f6ab16dc916 - proc_macro::bridge::client::<impl proc_macro::bridge::Bridge>::enter::{{closure}}::{{closure}}::hbcde13bfc6eca3f3
                               at /usr/local/google/home/richkadel/rust/library/proc_macro/src/bridge/client.rs:320:21
  13:     0x7f6ad1347b80 - std::panicking::rust_panic_with_hook::h795bd5f87591701f
                               at /usr/local/google/home/richkadel/rust/library/std/src/panicking.rs:595:17
  14:     0x7f6ad3afe3c0 - std::panicking::begin_panic::{{closure}}::h67f28f272ba5b299
  15:     0x7f6ad3afe289 - std::sys_common::backtrace::__rust_end_short_backtrace::h97fb83ee94bace97
  16:     0x7f6ad3afe342 - std::panicking::begin_panic::h9467f1cbeb0680aa
  17:     0x7f6ad3930b30 - std::panic::panic_any::h47ac84098d20382f
  18:     0x7f6ad392d405 - rustc_errors::HandlerInner::span_bug::hd47e97195754211a
  19:     0x7f6ad392d8c3 - rustc_errors::Handler::span_bug::ha07d3f2efbc4b3d3
  20:     0x7f6ad38ed4c0 - rustc_middle::util::bug::opt_span_bug_fmt::{{closure}}::h12cd570e8acb76b4
  21:     0x7f6ad38ec5cb - rustc_middle::ty::context::tls::with_opt::{{closure}}::h7aa59fd781092bae
  22:     0x7f6ad38ec57a - rustc_middle::ty::context::tls::with_opt::h0acb529f18cf2f6c
  23:     0x7f6ad38ed3d3 - rustc_middle::util::bug::opt_span_bug_fmt::hc86428558dc512b5
  24:     0x7f6ad38ed397 - rustc_middle::util::bug::span_bug_fmt::h9e46786f09ae7929
  25:     0x7f6ad381d517 - <rustc_mir::transform::generator::StateTransform as rustc_mir::transform::MirPass>::run_pass::h35fb8f3bebb5fa76
  26:     0x7f6ad3a23580 - rustc_mir::transform::run_passes::hadd0ed6897143b9e
  27:     0x7f6ad3a29509 - rustc_mir::transform::optimized_mir::h39ed0b27b4aefc95
  28:     0x7f6ad340c5b9 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::hd2659dfa0ecf2f94
  29:     0x7f6ad34cacef - rustc_data_structures::stack::ensure_sufficient_stack::h005ce2d6bf554ae7
  30:     0x7f6ad3109e68 - rustc_query_system::query::plumbing::force_query_with_job::h18c0a204e19f48b1
  31:     0x7f6ad308bfe9 - rustc_query_system::query::plumbing::get_query_impl::h7b8f88aa3b9abc9b
  32:     0x7f6ad33845bc - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::optimized_mir::h6781c46c4fe79c1b
  33:     0x7f6ad476f897 - rustc_middle::ty::<impl rustc_middle::ty::context::TyCtxt>::generator_layout::h2c9a7f7e78ab7724
  34:     0x7f6ad4838dc1 - rustc_middle::ty::layout::LayoutCx<rustc_middle::ty::context::TyCtxt>::layout_raw_uncached::h64afc7180f63c225
  35:     0x7f6ad4834be3 - rustc_middle::ty::layout::layout_raw::h18811fb33f919669
  36:     0x7f6ad31a0a53 - rustc_query_impl::<impl rustc_query_system::query::config::QueryAccessors<rustc_query_impl::plumbing::QueryCtxt> for rustc_query_impl::queries::layout_raw>::compute::haa2138f32522ab2b
  37:     0x7f6ad33fa886 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::h9900c6a25fa944b2
  38:     0x7f6ad34e725e - rustc_data_structures::stack::ensure_sufficient_stack::had5614b0d10064a9
  39:     0x7f6ad3120ac2 - rustc_query_system::query::plumbing::force_query_with_job::h87be078355284b1c
  40:     0x7f6ad306f4e8 - rustc_query_system::query::plumbing::get_query_impl::h3efad482f5b7ac0d
  41:     0x7f6ad33896fd - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::layout_raw::h637bf42247bed6c9
  42:     0x7f6ad4844643 - <rustc_middle::ty::layout::LayoutCx<rustc_middle::ty::context::TyCtxt> as rustc_target::abi::LayoutOf>::layout_of::he37b273ad2a07ac3
  43:     0x7f6ad47e14e3 - <alloc::vec::Vec<T> as alloc::vec::spec_from_iter::SpecFromIter<T,I>>::from_iter::h0737e81a2d4725ad
  44:     0x7f6ad4787541 - <core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::try_fold::h0916a09c46063fdd
  45:     0x7f6ad47eb501 - <alloc::vec::Vec<T> as alloc::vec::spec_from_iter::SpecFromIter<T,I>>::from_iter::hde16bed4df464c63
  46:     0x7f6ad483921c - rustc_middle::ty::layout::LayoutCx<rustc_middle::ty::context::TyCtxt>::layout_raw_uncached::h64afc7180f63c225
  47:     0x7f6ad4834be3 - rustc_middle::ty::layout::layout_raw::h18811fb33f919669
  48:     0x7f6ad31a0a53 - rustc_query_impl::<impl rustc_query_system::query::config::QueryAccessors<rustc_query_impl::plumbing::QueryCtxt> for rustc_query_impl::queries::layout_raw>::compute::haa2138f32522ab2b
  49:     0x7f6ad33fa886 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::h9900c6a25fa944b2
  50:     0x7f6ad34e725e - rustc_data_structures::stack::ensure_sufficient_stack::had5614b0d10064a9
  51:     0x7f6ad3120ac2 - rustc_query_system::query::plumbing::force_query_with_job::h87be078355284b1c
  52:     0x7f6ad306f4e8 - rustc_query_system::query::plumbing::get_query_impl::h3efad482f5b7ac0d
  53:     0x7f6ad33896fd - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::layout_raw::h637bf42247bed6c9
  54:     0x7f6ad4844643 - <rustc_middle::ty::layout::LayoutCx<rustc_middle::ty::context::TyCtxt> as rustc_target::abi::LayoutOf>::layout_of::he37b273ad2a07ac3
  55:     0x7f6ad386a08d - <rustc_mir::transform::const_prop::ConstProp as rustc_mir::transform::MirPass>::run_pass::he6bd2716bb6efaf5
  56:     0x7f6ad3a23580 - rustc_mir::transform::run_passes::hadd0ed6897143b9e
  57:     0x7f6ad3a29560 - rustc_mir::transform::optimized_mir::h39ed0b27b4aefc95
  58:     0x7f6ad340c5b9 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::hd2659dfa0ecf2f94
  59:     0x7f6ad34cacef - rustc_data_structures::stack::ensure_sufficient_stack::h005ce2d6bf554ae7
  60:     0x7f6ad3109e68 - rustc_query_system::query::plumbing::force_query_with_job::h18c0a204e19f48b1
  61:     0x7f6ad308bfe9 - rustc_query_system::query::plumbing::get_query_impl::h7b8f88aa3b9abc9b
  62:     0x7f6ad33845bc - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::optimized_mir::h6781c46c4fe79c1b
  63:     0x7f6ad476e89b - rustc_middle::ty::<impl rustc_middle::ty::context::TyCtxt>::instance_mir::h2adcb21a67bb16aa
  64:     0x7f6ad391dcc8 - rustc_mir::monomorphize::collector::collect_neighbours::h1bbe0e2725316449
  65:     0x7f6ad3917348 - rustc_mir::monomorphize::collector::collect_items_rec::h43d412ee589f7bda
  66:     0x7f6ad3c53bf9 - rustc_session::utils::<impl rustc_session::session::Session>::time::h7a56f9b4149bfbea
  67:     0x7f6ad3915916 - rustc_mir::monomorphize::collector::collect_crate_mono_items::hf86b7fa3f70cb1f4
  68:     0x7f6ad3c9be6c - rustc_mir::monomorphize::partitioning::collect_and_partition_mono_items::ha60093a41932fa60
  69:     0x7f6ad31a1299 - rustc_query_impl::<impl rustc_query_system::query::config::QueryAccessors<rustc_query_impl::plumbing::QueryCtxt> for rustc_query_impl::queries::collect_and_partition_mono_items>::compute::h6a07781687f36953
  70:     0x7f6ad341432a - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::hf2ef1b6c3a448afe
  71:     0x7f6ad34ef34c - rustc_data_structures::stack::ensure_sufficient_stack::he2ad1d418c205690
  72:     0x7f6ad312616a - rustc_query_system::query::plumbing::force_query_with_job::ha07662fb9d448595
  73:     0x7f6ad3095afd - rustc_query_system::query::plumbing::get_query_impl::h8c3d153258eb6b4b
  74:     0x7f6ad338c9a0 - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::collect_and_partition_mono_items::h45ac5f56bedc5b06
  75:     0x7f6ad2742988 - <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate::h1b3edce9a41a8cd9
  76:     0x7f6ad25e2f7f - rustc_interface::passes::QueryContext::enter::hd53f7e1828200bf7
  77:     0x7f6ad25eec23 - rustc_interface::queries::Queries::ongoing_codegen::h95a8778005c3b397
  78:     0x7f6ad2495873 - rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter::hc7b98d0ba93057dc
  79:     0x7f6ad248be1d - rustc_span::with_source_map::h681f81f772630eab
  80:     0x7f6ad249681b - rustc_interface::interface::create_compiler_and_run::h78eeee49dabc90f2
  81:     0x7f6ad24ba980 - scoped_tls::ScopedKey<T>::set::hd80250aa98541d63
  82:     0x7f6ad24bcde0 - std::sys_common::backtrace::__rust_begin_short_backtrace::h77920e05c4fb0f5a
  83:     0x7f6ad24c8033 - core::ops::function::FnOnce::call_once{{vtable.shim}}::hcac471670da73d1d
  84:     0x7f6ad1358573 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hfb93e846e02b3a24
                               at /usr/local/google/home/richkadel/rust/library/alloc/src/boxed.rs:1546:9
  85:     0x7f6ad1358573 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h1b4addec7ba447a1
                               at /usr/local/google/home/richkadel/rust/library/alloc/src/boxed.rs:1546:9
  86:     0x7f6ad1358573 - std::sys::unix::thread::Thread::new::thread_start::h48d9f3e91a2da8f4
                               at /usr/local/google/home/richkadel/rust/library/std/src/sys/unix/thread.rs:71:17
  87:     0x7f6acd590ea7 - start_thread
  88:     0x7f6ad1166def - clone

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.53.0-nightly (2d280e569 2021-04-01) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z dep-info-omit-d-target -Z allow-features= -Z panic_abort_tests -C link-args=--Map=./exe.unstripped/brightness_manager.map -C linker=/usr/local/google/home/richkadel/fuchsia/prebuilt/third_party/clang/linux-x64/bin/lld -C link-arg=--sysroot=gen/zircon/public/sysroot/cpp -C link-arg=-L/usr/local/google/home/richkadel/fuchsia/prebuilt/third_party/clang/linux-x64/lib/clang/13.0.0/x86_64-fuchsia/lib -C link-arg=--pack-dyn-relocs=relr -C link-arg=-dynamic-linker=ld.so.1 -C link-arg=--icf=all -C force-frame-pointers -C opt-level=0 -C debuginfo=2 -C link-args=-zstack-size=0x200000 -C panic=abort -C force-unwind-tables=yes -C prefer-dynamic -C link-arg=obj/sdk/lib/syslog/cpp/cpp.log_settings.cc.o -C link-arg=obj/sdk/lib/syslog/cpp/cpp.macros.cc.o -C link-arg=obj/out/core.x64/fidling/gen/sdk/fidl/fuchsia.diagnostics.stream/fuchsia.diagnostics.stream/hlcpp/fuchsia/diagnostics/stream/cpp/fuchsia.diagnostics.stream_hlcpp.fidl.cc.o -C link-arg=obj/out/core.x64/fidling/gen/sdk/fidl/fuchsia.diagnostics.stream/fuchsia.diagnostics.stream_tables.fuchsia.diagnostics.stream.fidl.tables.c.o -C link-arg=obj/out/core.x64/fidling/gen/sdk/fidl/fuchsia.diagnostics/fuchsia.diagnostics/hlcpp/fuchsia/diagnostics/cpp/fuchsia.diagnostics_hlcpp.fidl.cc.o -C link-arg=obj/out/core.x64/fidling/gen/sdk/fidl/fuchsia.diagnostics/fuchsia.diagnostics_tables.fuchsia.diagnostics.fidl.tables.c.o -C link-arg=obj/out/core.x64/fidling/gen/sdk/fidl/fuchsia.mem/fuchsia.mem/hlcpp/fuchsia/mem/cpp/fuchsia.mem_hlcpp.fidl.cc.o -C link-arg=obj/out/core.x64/fidling/gen/sdk/fidl/fuchsia.mem/fuchsia.mem_tables.fuchsia.mem.fidl.tables.c.o -C link-arg=obj/out/core.x64/fidling/gen/zircon/vdso/zx/zx/hlcpp/zx/cpp/zx_hlcpp.fidl.cc.o -C link-arg=obj/out/core.x64/fidling/gen/zircon/vdso/zx/zx_tables.zx.fidl.tables.c.o -C link-arg=obj/sdk/lib/fidl/cpp/internal/cpp.message_handler.cc.o -C link-arg=obj/sdk/lib/fidl/cpp/internal/cpp.message_reader.cc.o -C link-arg=obj/sdk/lib/fidl/cpp/internal/cpp.pending_response.cc.o -C link-arg=obj/sdk/lib/fidl/cpp/internal/cpp.proxy.cc.o -C link-arg=obj/sdk/lib/fidl/cpp/internal/cpp.proxy_controller.cc.o -C link-arg=obj/sdk/lib/fidl/cpp/internal/cpp.stub.cc.o -C link-arg=obj/sdk/lib/fidl/cpp/internal/cpp.stub_controller.cc.o -C link-arg=obj/sdk/lib/fidl/cpp/internal/cpp.weak_stub_controller.cc.o -C link-arg=obj/sdk/lib/fidl/cpp/internal/cpp_sync.logging.cc.o -C link-arg=obj/sdk/lib/fidl/cpp/internal/cpp_sync.message_sender.cc.o -C link-arg=obj/sdk/lib/fidl/cpp/internal/cpp_sync.synchronous_proxy.cc.o -C link-arg=obj/sdk/lib/fidl/cpp/cpp_base.builder.cc.o -C link-arg=obj/sdk/lib/fidl/cpp/cpp_base.clone.cc.o -C link-arg=obj/sdk/lib/fidl/cpp/cpp_base.decoder.cc.o -C link-arg=obj/sdk/lib/fidl/cpp/cpp_base.encoder.cc.o -C link-arg=obj/sdk/lib/fidl/cpp/cpp_base.message.cc.o -C link-arg=obj/sdk/lib/fidl/cpp/cpp_base.message_buffer.cc.o -C link-arg=obj/sdk/lib/fidl/cpp/cpp_base.message_builder.cc.o -C link-arg=obj/out/core.x64/fidling/gen/sdk/fidl/fuchsia.logger/fuchsia.logger/hlcpp/fuchsia/logger/cpp/fuchsia.logger_hlcpp.fidl.cc.o -C link-arg=obj/out/core.x64/fidling/gen/sdk/fidl/fuchsia.logger/fuchsia.logger_tables.fuchsia.logger.fidl.tables.c.o -C link-arg=obj/sdk/lib/syslog/streams/cpp/streams-encoder.encode.cc.o -C link-arg=user.libc_x64/obj/zircon/system/ulib/c/crt1.Scrt1.cc.o -C link-arg=obj/sdk/lib/syslog/cpp/libbackend_fuchsia_lib_rust.a -C link-arg=x64-shared/lib.unstripped/libbackend_fuchsia_globals.so -C link-arg=obj/sdk/lib/fit/libfit.a -C link-arg=obj/sdk/lib/stdcompat/libstdcompat.a -C link-arg=obj/zircon/system/ulib/fidl/libfidl_base.a -C link-arg=obj/sdk/lib/fit-promise/libfit-promise.a -C link-arg=obj/zircon/system/ulib/fidl/libfidl.a -C link-arg=obj/zircon/system/ulib/zx/libzx.a -C link-arg=obj/zircon/system/ulib/async/libasync.a -C link-arg=x64-shared/lib.unstripped/libasync-default.so -C link-arg=x64-shared/lib.unstripped/libfdio.so -C link-arg=x64-shared/lib.unstripped/libsyslog.so -C link-arg=user.libc_x64/libc.so.debug -C link-arg=libzircon.so --crate-type bin

query stack during panic:
#0 [optimized_mir] optimizing MIR for `main::func::{closure#0}`
#1 [layout_raw] computing layout of `[static generator@../../src/ui/bin/brightness_manager/src/main.rs:154:10: 154:14 for<'r, 's, 't0, 't1, 't2, 't3, 't4, 't5> {std::future::ResumeTy, fuchsia_component::server::ServiceFs<fuchsia_component::server::ServiceObjLocal<'r, fidl_fuchsia_ui_brightness::ControlRequestStream>>, std::future::from_generator::GenFuture<[static generator@../../src/ui/bin/brightness_manager/src/backlight.rs:30:52: 34:6 {}]>, (), backlight::Backlight, std::sync::Arc<futures::lock::Mutex<backlight::Backlight>>, std::future::from_generator::GenFuture<[static generator@../../src/ui/bin/brightness_manager/src/sensor.rs:97:34: 106:6 {std::future::ResumeTy, std::future::from_generator::GenFuture<[static generator@../../src/ui/bin/brightness_manager/src/sensor.rs:32:54: 50:2 for<'t6, 't7, 't8> {std::future::ResumeTy, &'t6 str, &'t7 std::path::Path, std::fs::ReadDir, std::result::Result<std::fs::DirEntry, std::io::Error>, std::fs::DirEntry, fidl_fuchsia_hardware_input::DeviceProxy, &'t8 fidl_fuchsia_hardware_input::DeviceProxy, fidl::client::QueryResponseFut<std::vec::Vec<u8>>, ()}]>, ()}]>, sensor::Sensor, std::sync::Arc<futures::lock::Mutex<sensor::Sensor>>, sender_channel::SenderChannel<f32>, std::sync::Arc<futures::lock::Mutex<sender_channel::SenderChannel<f32>>>, sender_channel::SenderChannel<bool>, std::sync::Arc<futures::lock::Mutex<sender_channel::SenderChannel<bool>>>, std::sync::Arc<futures::lock::Mutex<(dyn sensor::SensorControl + 's)>>, std::sync::Arc<futures::lock::Mutex<(dyn backlight::BacklightControl + 't0)>>, std::future::from_generator::GenFuture<[static generator@../../src/ui/bin/brightness_manager/src/control.rs:161:18: 188:6 for<'t6, 't7, 't8, 't9> {std::future::ResumeTy, std::sync::Arc<futures::lock::Mutex<(dyn sensor::SensorControl + 't6)>>, std::sync::Arc<futures::lock::Mutex<(dyn backlight::BacklightControl + 't7)>>, std::sync::Arc<futures::lock::Mutex<sender_channel::SenderChannel<f32>>>, std::sync::Arc<futures::lock::Mutex<sender_channel::SenderChannel<bool>>>, std::sync::Arc<futures::lock::Mutex<std::option::Option<futures::future::AbortHandle>>>, &'t8 futures::lock::Mutex<control::BrightnessTable>, control::BRIGHTNESS_TABLE, futures::lock::MutexLockFuture<'t9, control::BrightnessTable>, ()}]>, control::Control, std::sync::Arc<futures::lock::Mutex<control::Control>>, fn(fidl_fuchsia_ui_brightness::ControlRequestStream, std::sync::Arc<futures::lock::Mutex<(dyn control::ControlTrait + 't1)>>) -> std::pin::Pin<std::boxed::Box<(dyn futures::Future<Output = std::result::Result<(), anyhow::Error>> + 't2)>>, std::sync::Arc<futures::lock::Mutex<(dyn control::ControlTrait + 't3)>>, impl futures::Future}]`
#2 [layout_raw] computing layout of `std::future::from_generator::GenFuture<[static generator@../../src/ui/bin/brightness_manager/src/main.rs:154:10: 154:14 for<'r, 's, 't0, 't1, 't2, 't3, 't4, 't5> {std::future::ResumeTy, fuchsia_component::server::ServiceFs<fuchsia_component::server::ServiceObjLocal<'r, fidl_fuchsia_ui_brightness::ControlRequestStream>>, std::future::from_generator::GenFuture<[static generator@../../src/ui/bin/brightness_manager/src/backlight.rs:30:52: 34:6 {}]>, (), backlight::Backlight, std::sync::Arc<futures::lock::Mutex<backlight::Backlight>>, std::future::from_generator::GenFuture<[static generator@../../src/ui/bin/brightness_manager/src/sensor.rs:97:34: 106:6 {std::future::ResumeTy, std::future::from_generator::GenFuture<[static generator@../../src/ui/bin/brightness_manager/src/sensor.rs:32:54: 50:2 for<'t6, 't7, 't8> {std::future::ResumeTy, &'t6 str, &'t7 std::path::Path, std::fs::ReadDir, std::result::Result<std::fs::DirEntry, std::io::Error>, std::fs::DirEntry, fidl_fuchsia_hardware_input::DeviceProxy, &'t8 fidl_fuchsia_hardware_input::DeviceProxy, fidl::client::QueryResponseFut<std::vec::Vec<u8>>, ()}]>, ()}]>, sensor::Sensor, std::sync::Arc<futures::lock::Mutex<sensor::Sensor>>, sender_channel::SenderChannel<f32>, std::sync::Arc<futures::lock::Mutex<sender_channel::SenderChannel<f32>>>, sender_channel::SenderChannel<bool>, std::sync::Arc<futures::lock::Mutex<sender_channel::SenderChannel<bool>>>, std::sync::Arc<futures::lock::Mutex<(dyn sensor::SensorControl + 's)>>, std::sync::Arc<futures::lock::Mutex<(dyn backlight::BacklightControl + 't0)>>, std::future::from_generator::GenFuture<[static generator@../../src/ui/bin/brightness_manager/src/control.rs:161:18: 188:6 for<'t6, 't7, 't8, 't9> {std::future::ResumeTy, std::sync::Arc<futures::lock::Mutex<(dyn sensor::SensorControl + 't6)>>, std::sync::Arc<futures::lock::Mutex<(dyn backlight::BacklightControl + 't7)>>, std::sync::Arc<futures::lock::Mutex<sender_channel::SenderChannel<f32>>>, std::sync::Arc<futures::lock::Mutex<sender_channel::SenderChannel<bool>>>, std::sync::Arc<futures::lock::Mutex<std::option::Option<futures::future::AbortHandle>>>, &'t8 futures::lock::Mutex<control::BrightnessTable>, control::BRIGHTNESS_TABLE, futures::lock::MutexLockFuture<'t9, control::BrightnessTable>, ()}]>, control::Control, std::sync::Arc<futures::lock::Mutex<control::Control>>, fn(fidl_fuchsia_ui_brightness::ControlRequestStream, std::sync::Arc<futures::lock::Mutex<(dyn control::ControlTrait + 't1)>>) -> std::pin::Pin<std::boxed::Box<(dyn futures::Future<Output = std::result::Result<(), anyhow::Error>> + 't2)>>, std::sync::Arc<futures::lock::Mutex<(dyn control::ControlTrait + 't3)>>, impl futures::Future}]>`
#3 [optimized_mir] optimizing MIR for `main`
#4 [collect_and_partition_mono_items] collect_and_partition_mono_items
end of query stack
error: aborting due to previous error
