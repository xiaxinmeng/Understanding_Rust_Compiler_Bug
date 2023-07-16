
error: internal compiler error: compiler/rustc_codegen_llvm/src/context.rs:971:13: failed to get layout for `&mut Pin<&mut server::shutdown::Graceful<rocket::http::private::Incoming<rocket::ext::CancellableListener<rocket::Shutdown, L>>, service::make::MakeServiceFn<[closure@rocket::server::<impl Rocket<Orbit>>::http_server<L>::{closure#0}::{closure#2}]>, rocket::Shutdown, hyper::common::exec::Exec>>`: unable to determine layout for `&mut Pin<&mut server::shutdown::Graceful<rocket::http::private::Incoming<rocket::ext::CancellableListener<rocket::Shutdown, L>>, service::make::MakeServiceFn<[closure@rocket::server::<impl Rocket<Orbit>>::http_server<L>::{closure#0}::{closure#2}]>, rocket::Shutdown, hyper::common::exec::Exec>>` because `&mut Pin<&mut server::shutdown::Graceful<rocket::http::private::Incoming<rocket::ext::CancellableListener<rocket::Shutdown, L>>, service::make::MakeServiceFn<[closure@rocket::server::<impl Rocket<Orbit>>::http_server<L>::{closure#0}::{closure#2}]>, rocket::Shutdown, hyper::common::exec::Exec>>` cannot be normalized

thread 'rustc' panicked at 'Box<dyn Any>', /rustc/935dc07218b4bf6e20231e44eb9263b612fd649b/compiler/rustc_errors/src/lib.rs:973:33
stack backtrace:
   0:     0x7fea74365a7a - std::backtrace_rs::backtrace::libunwind::trace::h06c3ddb908575635
                               at /rustc/935dc07218b4bf6e20231e44eb9263b612fd649b/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7fea74365a7a - std::backtrace_rs::backtrace::trace_unsynchronized::h8bfc3753e1cc3345
                               at /rustc/935dc07218b4bf6e20231e44eb9263b612fd649b/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7fea74365a7a - std::sys_common::backtrace::_print_fmt::h3c304feda98d6679
                               at /rustc/935dc07218b4bf6e20231e44eb9263b612fd649b/library/std/src/sys_common/backtrace.rs:65:5
   3:     0x7fea74365a7a - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h98af9289a7a29dc2
                               at /rustc/935dc07218b4bf6e20231e44eb9263b612fd649b/library/std/src/sys_common/backtrace.rs:44:22
   4:     0x7fea743c87ae - core::fmt::write::h02538848739b7e2a
                               at /rustc/935dc07218b4bf6e20231e44eb9263b612fd649b/library/core/src/fmt/mod.rs:1208:17
   5:     0x7fea74355f45 - std::io::Write::write_fmt::h4aac2f2c3f0128af
                               at /rustc/935dc07218b4bf6e20231e44eb9263b612fd649b/library/std/src/io/mod.rs:1682:15
   6:     0x7fea74365845 - std::sys_common::backtrace::_print::h2e2acfe71f2fdb67
                               at /rustc/935dc07218b4bf6e20231e44eb9263b612fd649b/library/std/src/sys_common/backtrace.rs:47:5
   7:     0x7fea74365845 - std::sys_common::backtrace::print::h5bf077c253fab052
                               at /rustc/935dc07218b4bf6e20231e44eb9263b612fd649b/library/std/src/sys_common/backtrace.rs:34:9
   8:     0x7fea7436858f - std::panicking::default_hook::{{closure}}::h0ceeeedebdf177d0
                               at /rustc/935dc07218b4bf6e20231e44eb9263b612fd649b/library/std/src/panicking.rs:267:22
   9:     0x7fea743682cb - std::panicking::default_hook::h7d6f04e84940bd1d
                               at /rustc/935dc07218b4bf6e20231e44eb9263b612fd649b/library/std/src/panicking.rs:286:9
  10:     0x7fea77640511 - rustc_driver[3622bb61b1ebe64a]::DEFAULT_HOOK::{closure#0}::{closure#0}
  11:     0x7fea74368dcd - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::hc60f5ae29562b326
                               at /rustc/935dc07218b4bf6e20231e44eb9263b612fd649b/library/alloc/src/boxed.rs:2032:9
  12:     0x7fea74368dcd - std::panicking::rust_panic_with_hook::hce0f8648dbfb386b
                               at /rustc/935dc07218b4bf6e20231e44eb9263b612fd649b/library/std/src/panicking.rs:692:13
  13:     0x7fea774d3a81 - std[688abfdbcfe52504]::panicking::begin_panic::<rustc_errors[19d12f94925af316]::ExplicitBug>::{closure#0}
  14:     0x7fea774d3156 - std[688abfdbcfe52504]::sys_common::backtrace::__rust_end_short_backtrace::<std[688abfdbcfe52504]::panicking::begin_panic<rustc_errors[19d12f94925af316]::ExplicitBug>::{closure#0}, !>
  15:     0x7fea7746cb46 - std[688abfdbcfe52504]::panicking::begin_panic::<rustc_errors[19d12f94925af316]::ExplicitBug>
  16:     0x7fea774d0376 - std[688abfdbcfe52504]::panic::panic_any::<rustc_errors[19d12f94925af316]::ExplicitBug>
  17:     0x7fea774d0299 - <rustc_errors[19d12f94925af316]::HandlerInner>::span_bug::<rustc_span[ea560f4838c45ec0]::span_encoding::Span, &alloc[b02cc7237b8b6455]::string::String>
  18:     0x7fea774cff00 - <rustc_errors[19d12f94925af316]::Handler>::span_bug::<rustc_span[ea560f4838c45ec0]::span_encoding::Span, &alloc[b02cc7237b8b6455]::string::String>
  19:     0x7fea774d33d9 - rustc_middle[f828bfe4cd5e1a59]::ty::context::tls::with_context_opt::<rustc_middle[f828bfe4cd5e1a59]::ty::context::tls::with_opt<rustc_middle[f828bfe4cd5e1a59]::util::bug::opt_span_bug_fmt<rustc_span[ea560f4838c45ec0]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  20:     0x7fea774d3276 - rustc_middle[f828bfe4cd5e1a59]::util::bug::opt_span_bug_fmt::<rustc_span[ea560f4838c45ec0]::span_encoding::Span>
  21:     0x7fea75d8f694 - rustc_middle[f828bfe4cd5e1a59]::util::bug::span_bug_fmt::<rustc_span[ea560f4838c45ec0]::span_encoding::Span>
  22:     0x7fea77476cc2 - <rustc_codegen_llvm[74aa7289b70d0100]::context::CodegenCx as rustc_middle[f828bfe4cd5e1a59]::ty::layout::LayoutOfHelpers>::handle_layout_err
  23:     0x7fea77473801 - <rustc_codegen_llvm[74aa7289b70d0100]::context::CodegenCx as rustc_middle[f828bfe4cd5e1a59]::ty::layout::LayoutOf>::spanned_layout_of::{closure#0}
  24:     0x7fea774766fa - <rustc_codegen_llvm[74aa7289b70d0100]::context::CodegenCx as rustc_middle[f828bfe4cd5e1a59]::ty::layout::LayoutOf>::layout_of
  25:     0x7fea7746bb06 - <rustc_codegen_ssa[cc351e334a9b8a56]::mir::place::PlaceRef<&rustc_codegen_llvm[74aa7289b70d0100]::llvm_::ffi::Value>>::project_type::<rustc_codegen_llvm[74aa7289b70d0100]::builder::Builder>
  26:     0x7fea75d93c39 - <rustc_codegen_ssa[cc351e334a9b8a56]::mir::FunctionCx<rustc_codegen_llvm[74aa7289b70d0100]::builder::Builder>>::codegen_place
  27:     0x7fea75d6f4f8 - <rustc_codegen_ssa[cc351e334a9b8a56]::mir::FunctionCx<rustc_codegen_llvm[74aa7289b70d0100]::builder::Builder>>::codegen_block
  28:     0x7fea75d5d207 - rustc_codegen_ssa[cc351e334a9b8a56]::mir::codegen_mir::<rustc_codegen_llvm[74aa7289b70d0100]::builder::Builder>
  29:     0x7fea76b0fd16 - rustc_codegen_llvm[74aa7289b70d0100]::base::compile_codegen_unit::module_codegen
  30:     0x7fea76ce625c - <rustc_query_system[ffa0917f108ec877]::dep_graph::graph::DepGraph<rustc_middle[f828bfe4cd5e1a59]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[f828bfe4cd5e1a59]::ty::context::TyCtxt, rustc_span[ea560f4838c45ec0]::symbol::Symbol, rustc_codegen_ssa[cc351e334a9b8a56]::ModuleCodegen<rustc_codegen_llvm[74aa7289b70d0100]::ModuleLlvm>>
  31:     0x7fea76ce5e37 - <rustc_codegen_llvm[74aa7289b70d0100]::LlvmCodegenBackend as rustc_codegen_ssa[cc351e334a9b8a56]::traits::backend::ExtraBackendMethods>::compile_codegen_unit
  32:     0x7fea76ce3883 - rustc_codegen_ssa[cc351e334a9b8a56]::base::codegen_crate::<rustc_codegen_llvm[74aa7289b70d0100]::LlvmCodegenBackend>
  33:     0x7fea76ce3029 - <rustc_codegen_llvm[74aa7289b70d0100]::LlvmCodegenBackend as rustc_codegen_ssa[cc351e334a9b8a56]::traits::backend::CodegenBackend>::codegen_crate
  34:     0x7fea768d7481 - <rustc_session[951b5ce950426db2]::session::Session>::time::<alloc[b02cc7237b8b6455]::boxed::Box<dyn core[62c015fcd585be4a]::any::Any>, rustc_interface[299971c75fff0058]::passes::start_codegen::{closure#0}>
  35:     0x7fea768d6fa9 - rustc_interface[299971c75fff0058]::passes::start_codegen
  36:     0x7fea768d4ca6 - <rustc_interface[299971c75fff0058]::passes::QueryContext>::enter::<<rustc_interface[299971c75fff0058]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[62c015fcd585be4a]::result::Result<alloc[b02cc7237b8b6455]::boxed::Box<dyn core[62c015fcd585be4a]::any::Any>, rustc_errors[19d12f94925af316]::ErrorGuaranteed>>
  37:     0x7fea768d2156 - <rustc_interface[299971c75fff0058]::queries::Queries>::ongoing_codegen
  38:     0x7fea768d1677 - <rustc_interface[299971c75fff0058]::interface::Compiler>::enter::<rustc_driver[3622bb61b1ebe64a]::run_compiler::{closure#1}::{closure#2}, core[62c015fcd585be4a]::result::Result<core[62c015fcd585be4a]::option::Option<rustc_interface[299971c75fff0058]::queries::Linker>, rustc_errors[19d12f94925af316]::ErrorGuaranteed>>
  39:     0x7fea768cc658 - rustc_span[ea560f4838c45ec0]::with_source_map::<core[62c015fcd585be4a]::result::Result<(), rustc_errors[19d12f94925af316]::ErrorGuaranteed>, rustc_interface[299971c75fff0058]::interface::run_compiler<core[62c015fcd585be4a]::result::Result<(), rustc_errors[19d12f94925af316]::ErrorGuaranteed>, rustc_driver[3622bb61b1ebe64a]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  40:     0x7fea768cc145 - <scoped_tls[6f6de2f0677d8068]::ScopedKey<rustc_span[ea560f4838c45ec0]::SessionGlobals>>::set::<rustc_interface[299971c75fff0058]::interface::run_compiler<core[62c015fcd585be4a]::result::Result<(), rustc_errors[19d12f94925af316]::ErrorGuaranteed>, rustc_driver[3622bb61b1ebe64a]::run_compiler::{closure#1}>::{closure#0}, core[62c015fcd585be4a]::result::Result<(), rustc_errors[19d12f94925af316]::ErrorGuaranteed>>
  41:     0x7fea768cb732 - std[688abfdbcfe52504]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[299971c75fff0058]::util::run_in_thread_pool_with_globals<rustc_interface[299971c75fff0058]::interface::run_compiler<core[62c015fcd585be4a]::result::Result<(), rustc_errors[19d12f94925af316]::ErrorGuaranteed>, rustc_driver[3622bb61b1ebe64a]::run_compiler::{closure#1}>::{closure#0}, core[62c015fcd585be4a]::result::Result<(), rustc_errors[19d12f94925af316]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[62c015fcd585be4a]::result::Result<(), rustc_errors[19d12f94925af316]::ErrorGuaranteed>>
  42:     0x7fea76f03aba - <<std[688abfdbcfe52504]::thread::Builder>::spawn_unchecked_<rustc_interface[299971c75fff0058]::util::run_in_thread_pool_with_globals<rustc_interface[299971c75fff0058]::interface::run_compiler<core[62c015fcd585be4a]::result::Result<(), rustc_errors[19d12f94925af316]::ErrorGuaranteed>, rustc_driver[3622bb61b1ebe64a]::run_compiler::{closure#1}>::{closure#0}, core[62c015fcd585be4a]::result::Result<(), rustc_errors[19d12f94925af316]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[62c015fcd585be4a]::result::Result<(), rustc_errors[19d12f94925af316]::ErrorGuaranteed>>::{closure#1} as core[62c015fcd585be4a]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  43:     0x7fea74372dc3 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h16c8b1a5112fc07f
                               at /rustc/935dc07218b4bf6e20231e44eb9263b612fd649b/library/alloc/src/boxed.rs:2000:9
  44:     0x7fea74372dc3 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h0cc5ff4fc4b4dcff
                               at /rustc/935dc07218b4bf6e20231e44eb9263b612fd649b/library/alloc/src/boxed.rs:2000:9
  45:     0x7fea74372dc3 - std::sys::unix::thread::Thread::new::thread_start::ha7c6dff74603472f
                               at /rustc/935dc07218b4bf6e20231e44eb9263b612fd649b/library/std/src/sys/unix/thread.rs:108:17
  46:     0x7fea7411414d - start_thread
  47:     0x7fea74195a00 - __GI___clone3
  48:                0x0 - <unknown>

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.68.0-nightly (935dc0721 2022-12-19) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type bin -C embed-bitcode=no -C debuginfo=2 -C incremental=[REDACTED]

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
warning: `farmgate` (bin "farmgate") generated 7 warnings (run `cargo fix --bin "farmgate"` to apply 7 suggestions)
error: could not compile `farmgate`; 7 warnings emitted

