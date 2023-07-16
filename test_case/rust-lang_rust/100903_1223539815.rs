plain
.....................i.................i................................................ 264/13422
........................................................................................ 352/13422
........................................................................................ 440/13422
........................................................................................ 528/13422
.....................................................................................FFF 616/13422
.............F..F..F...................................................F................ 704/13422
...........................................................i............................ 880/13422
........................................................................................ 968/13422
........................................................................................ 1056/13422
........................................................................................ 1144/13422
---
failures:

---- [ui] src/test/ui/async-await/async-await.rs#nomiropt stdout ----

error in revision `nomiropt`: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/async-await.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "nomiropt" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-await.nomiropt/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "mir-opt-level=0" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-await.nomiropt/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: compiler/rustc_codegen_llvm/src/context.rs:992:21: `fn_abi_of_instance(<std::future::from_generator::GenFuture<[static generator@/checkout/src/test/ui/async-await/async-await.rs:182:28: 184:18]> as Future>::poll, [])` failed: the type `[type error]` has an unknown layout
   |
   |
LL |         fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {

thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1393:9
stack backtrace:
stack backtrace:
   0:     0x7f5269316afc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hd897be327daa99cb
   1:     0x7f526937efd8 - core::fmt::write::h76bfdbaa0723b595
   2:     0x7f52693076d1 - std::io::Write::write_fmt::h7566bf0b3653ad51
   3:     0x7f5269319ade - std::panicking::default_hook::{{closure}}::h52a899c6038ffa08
   4:     0x7f52693197a7 - std::panicking::default_hook::hc3cc699b84eeea6f
   5:     0x7f5269c9faf4 - rustc_driver[76860643812d0d25]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f526931a291 - std::panicking::rust_panic_with_hook::h3ff7d17b1bef18bf
   7:     0x7f5269fd90f3 - std[ce4413523c7f5639]::panicking::begin_panic::<rustc_errors[7acd77a66447d1d8]::ExplicitBug>::{closure#0}
   8:     0x7f5269fd0d76 - std[ce4413523c7f5639]::sys_common::backtrace::__rust_end_short_backtrace::<std[ce4413523c7f5639]::panicking::begin_panic<rustc_errors[7acd77a66447d1d8]::ExplicitBug>::{closure#0}, !>
   9:     0x7f5269abf986 - std[ce4413523c7f5639]::panicking::begin_panic::<rustc_errors[7acd77a66447d1d8]::ExplicitBug>
  10:     0x7f5269fb2a46 - std[ce4413523c7f5639]::panic::panic_any::<rustc_errors[7acd77a66447d1d8]::ExplicitBug>
  11:     0x7f5269fadc73 - <rustc_errors[7acd77a66447d1d8]::HandlerInner>::span_bug::<rustc_span[9e88627fbda4c1cd]::span_encoding::Span, &alloc[92664884fc986e55]::string::String>
  12:     0x7f5269fad8d0 - <rustc_errors[7acd77a66447d1d8]::Handler>::span_bug::<rustc_span[9e88627fbda4c1cd]::span_encoding::Span, &alloc[92664884fc986e55]::string::String>
  13:     0x7f5269eefc42 - rustc_middle[f9a8a3b120de9b6f]::ty::context::tls::with_context_opt::<rustc_middle[f9a8a3b120de9b6f]::ty::context::tls::with_opt<rustc_middle[f9a8a3b120de9b6f]::util::bug::opt_span_bug_fmt<rustc_span[9e88627fbda4c1cd]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  14:     0x7f5269eefa79 - rustc_middle[f9a8a3b120de9b6f]::util::bug::opt_span_bug_fmt::<rustc_span[9e88627fbda4c1cd]::span_encoding::Span>
  15:     0x7f5269ab561a - rustc_middle[f9a8a3b120de9b6f]::util::bug::span_bug_fmt::<rustc_span[9e88627fbda4c1cd]::span_encoding::Span>
  16:     0x7f5269e90554 - <rustc_codegen_llvm[7b178298eedd8479]::context::CodegenCx as rustc_middle[f9a8a3b120de9b6f]::ty::layout::FnAbiOfHelpers>::handle_fn_abi_err
  17:     0x7f5269e80995 - <rustc_codegen_llvm[7b178298eedd8479]::context::CodegenCx as rustc_middle[f9a8a3b120de9b6f]::ty::layout::FnAbiOf>::fn_abi_of_instance::{closure#0}
  18:     0x7f5269e9adaa - <rustc_codegen_llvm[7b178298eedd8479]::context::CodegenCx as rustc_codegen_ssa[29e4e69b69f50724]::traits::declare::PreDefineMethods>::predefine_fn
  19:     0x7f5269f7f8fe - rustc_codegen_llvm[7b178298eedd8479]::base::compile_codegen_unit::module_codegen
  20:     0x7f5269e9bfdd - <rustc_query_system[47adcd4433a5da6d]::dep_graph::graph::DepGraph<rustc_middle[f9a8a3b120de9b6f]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[f9a8a3b120de9b6f]::ty::context::TyCtxt, rustc_span[9e88627fbda4c1cd]::symbol::Symbol, rustc_codegen_ssa[29e4e69b69f50724]::ModuleCodegen<rustc_codegen_llvm[7b178298eedd8479]::ModuleLlvm>>
  21:     0x7f5269f7f57e - rustc_codegen_llvm[7b178298eedd8479]::base::compile_codegen_unit
  22:     0x7f5269f0601a - rustc_codegen_ssa[29e4e69b69f50724]::base::codegen_crate::<rustc_codegen_llvm[7b178298eedd8479]::LlvmCodegenBackend>
  23:     0x7f5269fc54dd - <rustc_codegen_llvm[7b178298eedd8479]::LlvmCodegenBackend as rustc_codegen_ssa[29e4e69b69f50724]::traits::backend::CodegenBackend>::codegen_crate
  24:     0x7f5269e2288b - <rustc_session[d09f6f8e5f6d4931]::session::Session>::time::<alloc[92664884fc986e55]::boxed::Box<dyn core[6c815afa72b208e9]::any::Any>, rustc_interface[dddbe1f16a5244f9]::passes::start_codegen::{closure#0}>
  25:     0x7f5269de05d5 - <rustc_interface[dddbe1f16a5244f9]::passes::QueryContext>::enter::<<rustc_interface[dddbe1f16a5244f9]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[6c815afa72b208e9]::result::Result<alloc[92664884fc986e55]::boxed::Box<dyn core[6c815afa72b208e9]::any::Any>, rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>>
  26:     0x7f5269dc52ad - <rustc_interface[dddbe1f16a5244f9]::queries::Queries>::ongoing_codegen
  27:     0x7f5269ca8b16 - <rustc_interface[dddbe1f16a5244f9]::interface::Compiler>::enter::<rustc_driver[76860643812d0d25]::run_compiler::{closure#1}::{closure#2}, core[6c815afa72b208e9]::result::Result<core[6c815afa72b208e9]::option::Option<rustc_interface[dddbe1f16a5244f9]::queries::Linker>, rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>>
  28:     0x7f5269c8c731 - rustc_span[9e88627fbda4c1cd]::with_source_map::<core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>, rustc_interface[dddbe1f16a5244f9]::interface::create_compiler_and_run<core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>, rustc_driver[76860643812d0d25]::run_compiler::{closure#1}>::{closure#1}>
  29:     0x7f5269cc4581 - rustc_interface[dddbe1f16a5244f9]::interface::create_compiler_and_run::<core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>, rustc_driver[76860643812d0d25]::run_compiler::{closure#1}>
  30:     0x7f5269c84302 - <scoped_tls[f70245fe0c4e39bd]::ScopedKey<rustc_span[9e88627fbda4c1cd]::SessionGlobals>>::set::<rustc_interface[dddbe1f16a5244f9]::interface::run_compiler<core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>, rustc_driver[76860643812d0d25]::run_compiler::{closure#1}>::{closure#0}, core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>>
  31:     0x7f5269c8ffa9 - std[ce4413523c7f5639]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[dddbe1f16a5244f9]::util::run_in_thread_pool_with_globals<rustc_interface[dddbe1f16a5244f9]::interface::run_compiler<core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>, rustc_driver[76860643812d0d25]::run_compiler::{closure#1}>::{closure#0}, core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>>::{closure#0}, core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>>
  32:     0x7f5269d0acae - std[ce4413523c7f5639]::panicking::try::<core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>, core[6c815afa72b208e9]::panic::unwind_safe::AssertUnwindSafe<<std[ce4413523c7f5639]::thread::Builder>::spawn_unchecked_<rustc_interface[dddbe1f16a5244f9]::util::run_in_thread_pool_with_globals<rustc_interface[dddbe1f16a5244f9]::interface::run_compiler<core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>, rustc_driver[76860643812d0d25]::run_compiler::{closure#1}>::{closure#0}, core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>>::{closure#0}, core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  33:     0x7f5269d05de0 - <<std[ce4413523c7f5639]::thread::Builder>::spawn_unchecked_<rustc_interface[dddbe1f16a5244f9]::util::run_in_thread_pool_with_globals<rustc_interface[dddbe1f16a5244f9]::interface::run_compiler<core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>, rustc_driver[76860643812d0d25]::run_compiler::{closure#1}>::{closure#0}, core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>>::{closure#0}, core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>>::{closure#1} as core[6c815afa72b208e9]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  34:     0x7f5269326b45 - std::sys::unix::thread::Thread::new::thread_start::hea2c413ca0105153
  35:     0x7f52690c3b43 - <unknown>
  36:     0x7f5269155a00 - <unknown>
  37:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.65.0-nightly (29e5879de 2022-08-23) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 -Z mir-opt-level=0
query stack during panic:
end of query stack
error: aborting due to previous error
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/async-await/async-await.rs#default stdout ----

error in revision `default`: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/async-await.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "default" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-await.default/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-await.default/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: compiler/rustc_codegen_llvm/src/context.rs:992:21: `fn_abi_of_instance(<std::future::from_generator::GenFuture<[static generator@/checkout/src/test/ui/async-await/async-await.rs:182:28: 184:18]> as Future>::poll, [])` failed: the type `[type error]` has an unknown layout
   |
   |
LL |         fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {

thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1393:9
stack backtrace:
stack backtrace:
   0:     0x7efdc968bafc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hd897be327daa99cb
   1:     0x7efdc96f3fd8 - core::fmt::write::h76bfdbaa0723b595
   2:     0x7efdc967c6d1 - std::io::Write::write_fmt::h7566bf0b3653ad51
   3:     0x7efdc968eade - std::panicking::default_hook::{{closure}}::h52a899c6038ffa08
   4:     0x7efdc968e7a7 - std::panicking::default_hook::hc3cc699b84eeea6f
   5:     0x7efdca014af4 - rustc_driver[76860643812d0d25]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7efdc968f291 - std::panicking::rust_panic_with_hook::h3ff7d17b1bef18bf
   7:     0x7efdca34e0f3 - std[ce4413523c7f5639]::panicking::begin_panic::<rustc_errors[7acd77a66447d1d8]::ExplicitBug>::{closure#0}
   8:     0x7efdca345d76 - std[ce4413523c7f5639]::sys_common::backtrace::__rust_end_short_backtrace::<std[ce4413523c7f5639]::panicking::begin_panic<rustc_errors[7acd77a66447d1d8]::ExplicitBug>::{closure#0}, !>
   9:     0x7efdc9e34986 - std[ce4413523c7f5639]::panicking::begin_panic::<rustc_errors[7acd77a66447d1d8]::ExplicitBug>
  10:     0x7efdca327a46 - std[ce4413523c7f5639]::panic::panic_any::<rustc_errors[7acd77a66447d1d8]::ExplicitBug>
  11:     0x7efdca322c73 - <rustc_errors[7acd77a66447d1d8]::HandlerInner>::span_bug::<rustc_span[9e88627fbda4c1cd]::span_encoding::Span, &alloc[92664884fc986e55]::string::String>
  12:     0x7efdca3228d0 - <rustc_errors[7acd77a66447d1d8]::Handler>::span_bug::<rustc_span[9e88627fbda4c1cd]::span_encoding::Span, &alloc[92664884fc986e55]::string::String>
  13:     0x7efdca264c42 - rustc_middle[f9a8a3b120de9b6f]::ty::context::tls::with_context_opt::<rustc_middle[f9a8a3b120de9b6f]::ty::context::tls::with_opt<rustc_middle[f9a8a3b120de9b6f]::util::bug::opt_span_bug_fmt<rustc_span[9e88627fbda4c1cd]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  14:     0x7efdca264a79 - rustc_middle[f9a8a3b120de9b6f]::util::bug::opt_span_bug_fmt::<rustc_span[9e88627fbda4c1cd]::span_encoding::Span>
  15:     0x7efdc9e2a61a - rustc_middle[f9a8a3b120de9b6f]::util::bug::span_bug_fmt::<rustc_span[9e88627fbda4c1cd]::span_encoding::Span>
  16:     0x7efdca205554 - <rustc_codegen_llvm[7b178298eedd8479]::context::CodegenCx as rustc_middle[f9a8a3b120de9b6f]::ty::layout::FnAbiOfHelpers>::handle_fn_abi_err
  17:     0x7efdca1f5995 - <rustc_codegen_llvm[7b178298eedd8479]::context::CodegenCx as rustc_middle[f9a8a3b120de9b6f]::ty::layout::FnAbiOf>::fn_abi_of_instance::{closure#0}
  18:     0x7efdca20fdaa - <rustc_codegen_llvm[7b178298eedd8479]::context::CodegenCx as rustc_codegen_ssa[29e4e69b69f50724]::traits::declare::PreDefineMethods>::predefine_fn
  19:     0x7efdca2f48fe - rustc_codegen_llvm[7b178298eedd8479]::base::compile_codegen_unit::module_codegen
  20:     0x7efdca210fdd - <rustc_query_system[47adcd4433a5da6d]::dep_graph::graph::DepGraph<rustc_middle[f9a8a3b120de9b6f]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[f9a8a3b120de9b6f]::ty::context::TyCtxt, rustc_span[9e88627fbda4c1cd]::symbol::Symbol, rustc_codegen_ssa[29e4e69b69f50724]::ModuleCodegen<rustc_codegen_llvm[7b178298eedd8479]::ModuleLlvm>>
  21:     0x7efdca2f457e - rustc_codegen_llvm[7b178298eedd8479]::base::compile_codegen_unit
  22:     0x7efdca27b01a - rustc_codegen_ssa[29e4e69b69f50724]::base::codegen_crate::<rustc_codegen_llvm[7b178298eedd8479]::LlvmCodegenBackend>
  23:     0x7efdca33a4dd - <rustc_codegen_llvm[7b178298eedd8479]::LlvmCodegenBackend as rustc_codegen_ssa[29e4e69b69f50724]::traits::backend::CodegenBackend>::codegen_crate
  24:     0x7efdca19788b - <rustc_session[d09f6f8e5f6d4931]::session::Session>::time::<alloc[92664884fc986e55]::boxed::Box<dyn core[6c815afa72b208e9]::any::Any>, rustc_interface[dddbe1f16a5244f9]::passes::start_codegen::{closure#0}>
  25:     0x7efdca1555d5 - <rustc_interface[dddbe1f16a5244f9]::passes::QueryContext>::enter::<<rustc_interface[dddbe1f16a5244f9]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[6c815afa72b208e9]::result::Result<alloc[92664884fc986e55]::boxed::Box<dyn core[6c815afa72b208e9]::any::Any>, rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>>
  26:     0x7efdca13a2ad - <rustc_interface[dddbe1f16a5244f9]::queries::Queries>::ongoing_codegen
  27:     0x7efdca01db16 - <rustc_interface[dddbe1f16a5244f9]::interface::Compiler>::enter::<rustc_driver[76860643812d0d25]::run_compiler::{closure#1}::{closure#2}, core[6c815afa72b208e9]::result::Result<core[6c815afa72b208e9]::option::Option<rustc_interface[dddbe1f16a5244f9]::queries::Linker>, rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>>
  28:     0x7efdca001731 - rustc_span[9e88627fbda4c1cd]::with_source_map::<core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>, rustc_interface[dddbe1f16a5244f9]::interface::create_compiler_and_run<core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>, rustc_driver[76860643812d0d25]::run_compiler::{closure#1}>::{closure#1}>
  29:     0x7efdca039581 - rustc_interface[dddbe1f16a5244f9]::interface::create_compiler_and_run::<core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>, rustc_driver[76860643812d0d25]::run_compiler::{closure#1}>
  30:     0x7efdc9ff9302 - <scoped_tls[f70245fe0c4e39bd]::ScopedKey<rustc_span[9e88627fbda4c1cd]::SessionGlobals>>::set::<rustc_interface[dddbe1f16a5244f9]::interface::run_compiler<core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>, rustc_driver[76860643812d0d25]::run_compiler::{closure#1}>::{closure#0}, core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>>
  31:     0x7efdca004fa9 - std[ce4413523c7f5639]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[dddbe1f16a5244f9]::util::run_in_thread_pool_with_globals<rustc_interface[dddbe1f16a5244f9]::interface::run_compiler<core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>, rustc_driver[76860643812d0d25]::run_compiler::{closure#1}>::{closure#0}, core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>>::{closure#0}, core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>>
  32:     0x7efdca07fcae - std[ce4413523c7f5639]::panicking::try::<core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>, core[6c815afa72b208e9]::panic::unwind_safe::AssertUnwindSafe<<std[ce4413523c7f5639]::thread::Builder>::spawn_unchecked_<rustc_interface[dddbe1f16a5244f9]::util::run_in_thread_pool_with_globals<rustc_interface[dddbe1f16a5244f9]::interface::run_compiler<core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>, rustc_driver[76860643812d0d25]::run_compiler::{closure#1}>::{closure#0}, core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>>::{closure#0}, core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  33:     0x7efdca07ade0 - <<std[ce4413523c7f5639]::thread::Builder>::spawn_unchecked_<rustc_interface[dddbe1f16a5244f9]::util::run_in_thread_pool_with_globals<rustc_interface[dddbe1f16a5244f9]::interface::run_compiler<core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>, rustc_driver[76860643812d0d25]::run_compiler::{closure#1}>::{closure#0}, core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>>::{closure#0}, core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>>::{closure#1} as core[6c815afa72b208e9]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  34:     0x7efdc969bb45 - std::sys::unix::thread::Thread::new::thread_start::hea2c413ca0105153
  35:     0x7efdc9438b43 - <unknown>
  36:     0x7efdc94caa00 - <unknown>
  37:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.65.0-nightly (29e5879de 2022-08-23) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
error: aborting due to previous error
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/async-await/async-await.rs#thirunsafeck stdout ----

error in revision `thirunsafeck`: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/async-await.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "thirunsafeck" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-await.thirunsafeck/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zthir-unsafeck" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-await.thirunsafeck/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: compiler/rustc_codegen_llvm/src/context.rs:992:21: `fn_abi_of_instance(<std::future::from_generator::GenFuture<[static generator@/checkout/src/test/ui/async-await/async-await.rs:182:28: 184:18]> as Future>::poll, [])` failed: the type `[type error]` has an unknown layout
   |
   |
LL |         fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {

thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1393:9
stack backtrace:
stack backtrace:
   0:     0x7f1385e85afc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hd897be327daa99cb
   1:     0x7f1385eedfd8 - core::fmt::write::h76bfdbaa0723b595
   2:     0x7f1385e766d1 - std::io::Write::write_fmt::h7566bf0b3653ad51
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   3:     0x7f1385e88ade - std::panicking::default_hook::{{closure}}::h52a899c6038ffa08
   4:     0x7f1385e887a7 - std::panicking::default_hook::hc3cc699b84eeea6f
   5:     0x7f138680eaf4 - rustc_driver[76860643812d0d25]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f1385e89291 - std::panicking::rust_panic_with_hook::h3ff7d17b1bef18bf
   7:     0x7f1386b480f3 - std[ce4413523c7f5639]::panicking::begin_panic::<rustc_errors[7acd77a66447d1d8]::ExplicitBug>::{closure#0}
   8:     0x7f1386b3fd76 - std[ce4413523c7f5639]::sys_common::backtrace::__rust_end_short_backtrace::<std[ce4413523c7f5639]::panicking::begin_panic<rustc_errors[7acd77a66447d1d8]::ExplicitBug>::{closure#0}, !>
   9:     0x7f138662e986 - std[ce4413523c7f5639]::panicking::begin_panic::<rustc_errors[7acd77a66447d1d8]::ExplicitBug>
  10:     0x7f1386b21a46 - std[ce4413523c7f5639]::panic::panic_any::<rustc_errors[7acd77a66447d1d8]::ExplicitBug>
  11:     0x7f1386b1cc73 - <rustc_errors[7acd77a66447d1d8]::HandlerInner>::span_bug::<rustc_span[9e88627fbda4c1cd]::span_encoding::Span, &alloc[92664884fc986e55]::string::String>
  12:     0x7f1386b1c8d0 - <rustc_errors[7acd77a66447d1d8]::Handler>::span_bug::<rustc_span[9e88627fbda4c1cd]::span_encoding::Span, &alloc[92664884fc986e55]::string::String>
  13:     0x7f1386a5ec42 - rustc_middle[f9a8a3b120de9b6f]::ty::context::tls::with_context_opt::<rustc_middle[f9a8a3b120de9b6f]::ty::context::tls::with_opt<rustc_middle[f9a8a3b120de9b6f]::util::bug::opt_span_bug_fmt<rustc_span[9e88627fbda4c1cd]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  14:     0x7f1386a5ea79 - rustc_middle[f9a8a3b120de9b6f]::util::bug::opt_span_bug_fmt::<rustc_span[9e88627fbda4c1cd]::span_encoding::Span>
  15:     0x7f138662461a - rustc_middle[f9a8a3b120de9b6f]::util::bug::span_bug_fmt::<rustc_span[9e88627fbda4c1cd]::span_encoding::Span>
  16:     0x7f13869ff554 - <rustc_codegen_llvm[7b178298eedd8479]::context::CodegenCx as rustc_middle[f9a8a3b120de9b6f]::ty::layout::FnAbiOfHelpers>::handle_fn_abi_err
  17:     0x7f13869ef995 - <rustc_codegen_llvm[7b178298eedd8479]::context::CodegenCx as rustc_middle[f9a8a3b120de9b6f]::ty::layout::FnAbiOf>::fn_abi_of_instance::{closure#0}
  18:     0x7f1386a09daa - <rustc_codegen_llvm[7b178298eedd8479]::context::CodegenCx as rustc_codegen_ssa[29e4e69b69f50724]::traits::declare::PreDefineMethods>::predefine_fn
  19:     0x7f1386aee8fe - rustc_codegen_llvm[7b178298eedd8479]::base::compile_codegen_unit::module_codegen
  20:     0x7f1386a0afdd - <rustc_query_system[47adcd4433a5da6d]::dep_graph::graph::DepGraph<rustc_middle[f9a8a3b120de9b6f]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[f9a8a3b120de9b6f]::ty::context::TyCtxt, rustc_span[9e88627fbda4c1cd]::symbol::Symbol, rustc_codegen_ssa[29e4e69b69f50724]::ModuleCodegen<rustc_codegen_llvm[7b178298eedd8479]::ModuleLlvm>>
  21:     0x7f1386aee57e - rustc_codegen_llvm[7b178298eedd8479]::base::compile_codegen_unit
  22:     0x7f1386a7501a - rustc_codegen_ssa[29e4e69b69f50724]::base::codegen_crate::<rustc_codegen_llvm[7b178298eedd8479]::LlvmCodegenBackend>
  23:     0x7f1386b344dd - <rustc_codegen_llvm[7b178298eedd8479]::LlvmCodegenBackend as rustc_codegen_ssa[29e4e69b69f50724]::traits::backend::CodegenBackend>::codegen_crate
  24:     0x7f138699188b - <rustc_session[d09f6f8e5f6d4931]::session::Session>::time::<alloc[92664884fc986e55]::boxed::Box<dyn core[6c815afa72b208e9]::any::Any>, rustc_interface[dddbe1f16a5244f9]::passes::start_codegen::{closure#0}>
  25:     0x7f138694f5d5 - <rustc_interface[dddbe1f16a5244f9]::passes::QueryContext>::enter::<<rustc_interface[dddbe1f16a5244f9]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[6c815afa72b208e9]::result::Result<alloc[92664884fc986e55]::boxed::Box<dyn core[6c815afa72b208e9]::any::Any>, rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>>
  26:     0x7f13869342ad - <rustc_interface[dddbe1f16a5244f9]::queries::Queries>::ongoing_codegen
  27:     0x7f1386817b16 - <rustc_interface[dddbe1f16a5244f9]::interface::Compiler>::enter::<rustc_driver[76860643812d0d25]::run_compiler::{closure#1}::{closure#2}, core[6c815afa72b208e9]::result::Result<core[6c815afa72b208e9]::option::Option<rustc_interface[dddbe1f16a5244f9]::queries::Linker>, rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>>
  28:     0x7f13867fb731 - rustc_span[9e88627fbda4c1cd]::with_source_map::<core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>, rustc_interface[dddbe1f16a5244f9]::interface::create_compiler_and_run<core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>, rustc_driver[76860643812d0d25]::run_compiler::{closure#1}>::{closure#1}>
  29:     0x7f1386833581 - rustc_interface[dddbe1f16a5244f9]::interface::create_compiler_and_run::<core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>, rustc_driver[76860643812d0d25]::run_compiler::{closure#1}>
  30:     0x7f13867f3302 - <scoped_tls[f70245fe0c4e39bd]::ScopedKey<rustc_span[9e88627fbda4c1cd]::SessionGlobals>>::set::<rustc_interface[dddbe1f16a5244f9]::interface::run_compiler<core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>, rustc_driver[76860643812d0d25]::run_compiler::{closure#1}>::{closure#0}, core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>>
  31:     0x7f13867fefa9 - std[ce4413523c7f5639]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[dddbe1f16a5244f9]::util::run_in_thread_pool_with_globals<rustc_interface[dddbe1f16a5244f9]::interface::run_compiler<core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>, rustc_driver[76860643812d0d25]::run_compiler::{closure#1}>::{closure#0}, core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>>::{closure#0}, core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>>
  32:     0x7f1386879cae - std[ce4413523c7f5639]::panicking::try::<core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>, core[6c815afa72b208e9]::panic::unwind_safe::AssertUnwindSafe<<std[ce4413523c7f5639]::thread::Builder>::spawn_unchecked_<rustc_interface[dddbe1f16a5244f9]::util::run_in_thread_pool_with_globals<rustc_interface[dddbe1f16a5244f9]::interface::run_compiler<core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>, rustc_driver[76860643812d0d25]::run_compiler::{closure#1}>::{closure#0}, core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>>::{closure#0}, core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  33:     0x7f1386874de0 - <<std[ce4413523c7f5639]::thread::Builder>::spawn_unchecked_<rustc_interface[dddbe1f16a5244f9]::util::run_in_thread_pool_with_globals<rustc_interface[dddbe1f16a5244f9]::interface::run_compiler<core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>, rustc_driver[76860643812d0d25]::run_compiler::{closure#1}>::{closure#0}, core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>>::{closure#0}, core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>>::{closure#1} as core[6c815afa72b208e9]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  34:     0x7f1385e95b45 - std::sys::unix::thread::Thread::new::thread_start::hea2c413ca0105153
  35:     0x7f1385c32b43 - <unknown>
  36:     0x7f1385cc4a00 - <unknown>
  37:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.65.0-nightly (29e5879de 2022-08-23) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 -Z thir-unsafeck
query stack during panic:
end of query stack
error: aborting due to previous error
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/async-await/drop-order/drop-order-for-temporary-in-tail-return-expr.rs#default stdout ----

error in revision `default`: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/drop-order/drop-order-for-temporary-in-tail-return-expr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "default" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/drop-order/drop-order-for-temporary-in-tail-return-expr.default/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/drop-order/drop-order-for-temporary-in-tail-return-expr.default/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: compiler/rustc_codegen_llvm/src/context.rs:992:21: `fn_abi_of_instance(<std::future::from_generator::GenFuture<[static generator@/checkout/src/test/ui/async-await/drop-order/drop-order-for-temporary-in-tail-return-expr.rs:53:33: 58:2]> as Future>::poll, [])` failed: the type `[type error]` has an unknown layout
   |
   |
LL |         fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {

thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1393:9
stack backtrace:
stack backtrace:
   0:     0x7f884f909afc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hd897be327daa99cb
   1:     0x7f884f971fd8 - core::fmt::write::h76bfdbaa0723b595
   2:     0x7f884f8fa6d1 - std::io::Write::write_fmt::h7566bf0b3653ad51
   3:     0x7f884f90cade - std::panicking::default_hook::{{closure}}::h52a899c6038ffa08
   4:     0x7f884f90c7a7 - std::panicking::default_hook::hc3cc699b84eeea6f
   5:     0x7f8850292af4 - rustc_driver[76860643812d0d25]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f884f90d291 - std::panicking::rust_panic_with_hook::h3ff7d17b1bef18bf
   7:     0x7f88505cc0f3 - std[ce4413523c7f5639]::panicking::begin_panic::<rustc_errors[7acd77a66447d1d8]::ExplicitBug>::{closure#0}
   8:     0x7f88505c3d76 - std[ce4413523c7f5639]::sys_common::backtrace::__rust_end_short_backtrace::<std[ce4413523c7f5639]::panicking::begin_panic<rustc_errors[7acd77a66447d1d8]::ExplicitBug>::{closure#0}, !>
   9:     0x7f88500b2986 - std[ce4413523c7f5639]::panicking::begin_panic::<rustc_errors[7acd77a66447d1d8]::ExplicitBug>
  10:     0x7f88505a5a46 - std[ce4413523c7f5639]::panic::panic_any::<rustc_errors[7acd77a66447d1d8]::ExplicitBug>
  11:     0x7f88505a0c73 - <rustc_errors[7acd77a66447d1d8]::HandlerInner>::span_bug::<rustc_span[9e88627fbda4c1cd]::span_encoding::Span, &alloc[92664884fc986e55]::string::String>
  12:     0x7f88505a08d0 - <rustc_errors[7acd77a66447d1d8]::Handler>::span_bug::<rustc_span[9e88627fbda4c1cd]::span_encoding::Span, &alloc[92664884fc986e55]::string::String>
  13:     0x7f88504e2c42 - rustc_middle[f9a8a3b120de9b6f]::ty::context::tls::with_context_opt::<rustc_middle[f9a8a3b120de9b6f]::ty::context::tls::with_opt<rustc_middle[f9a8a3b120de9b6f]::util::bug::opt_span_bug_fmt<rustc_span[9e88627fbda4c1cd]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  14:     0x7f88504e2a79 - rustc_middle[f9a8a3b120de9b6f]::util::bug::opt_span_bug_fmt::<rustc_span[9e88627fbda4c1cd]::span_encoding::Span>
  15:     0x7f88500a861a - rustc_middle[f9a8a3b120de9b6f]::util::bug::span_bug_fmt::<rustc_span[9e88627fbda4c1cd]::span_encoding::Span>
  16:     0x7f8850483554 - <rustc_codegen_llvm[7b178298eedd8479]::context::CodegenCx as rustc_middle[f9a8a3b120de9b6f]::ty::layout::FnAbiOfHelpers>::handle_fn_abi_err
  17:     0x7f8850473995 - <rustc_codegen_llvm[7b178298eedd8479]::context::CodegenCx as rustc_middle[f9a8a3b120de9b6f]::ty::layout::FnAbiOf>::fn_abi_of_instance::{closure#0}
  18:     0x7f885048ddaa - <rustc_codegen_llvm[7b178298eedd8479]::context::CodegenCx as rustc_codegen_ssa[29e4e69b69f50724]::traits::declare::PreDefineMethods>::predefine_fn
  19:     0x7f88505728fe - rustc_codegen_llvm[7b178298eedd8479]::base::compile_codegen_unit::module_codegen
  20:     0x7f885048efdd - <rustc_query_system[47adcd4433a5da6d]::dep_graph::graph::DepGraph<rustc_middle[f9a8a3b120de9b6f]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[f9a8a3b120de9b6f]::ty::context::TyCtxt, rustc_span[9e88627fbda4c1cd]::symbol::Symbol, rustc_codegen_ssa[29e4e69b69f50724]::ModuleCodegen<rustc_codegen_llvm[7b178298eedd8479]::ModuleLlvm>>
  21:     0x7f885057257e - rustc_codegen_llvm[7b178298eedd8479]::base::compile_codegen_unit
  22:     0x7f88504f901a - rustc_codegen_ssa[29e4e69b69f50724]::base::codegen_crate::<rustc_codegen_llvm[7b178298eedd8479]::LlvmCodegenBackend>
  23:     0x7f88505b84dd - <rustc_codegen_llvm[7b178298eedd8479]::LlvmCodegenBackend as rustc_codegen_ssa[29e4e69b69f50724]::traits::backend::CodegenBackend>::codegen_crate
  24:     0x7f885041588b - <rustc_session[d09f6f8e5f6d4931]::session::Session>::time::<alloc[92664884fc986e55]::boxed::Box<dyn core[6c815afa72b208e9]::any::Any>, rustc_interface[dddbe1f16a5244f9]::passes::start_codegen::{closure#0}>
  25:     0x7f88503d35d5 - <rustc_interface[dddbe1f16a5244f9]::passes::QueryContext>::enter::<<rustc_interface[dddbe1f16a5244f9]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[6c815afa72b208e9]::result::Result<alloc[92664884fc986e55]::boxed::Box<dyn core[6c815afa72b208e9]::any::Any>, rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>>
  26:     0x7f88503b82ad - <rustc_interface[dddbe1f16a5244f9]::queries::Queries>::ongoing_codegen
  27:     0x7f885029bb16 - <rustc_interface[dddbe1f16a5244f9]::interface::Compiler>::enter::<rustc_driver[76860643812d0d25]::run_compiler::{closure#1}::{closure#2}, core[6c815afa72b208e9]::result::Result<core[6c815afa72b208e9]::option::Option<rustc_interface[dddbe1f16a5244f9]::queries::Linker>, rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>>
  28:     0x7f885027f731 - rustc_span[9e88627fbda4c1cd]::with_source_map::<core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>, rustc_interface[dddbe1f16a5244f9]::interface::create_compiler_and_run<core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>, rustc_driver[76860643812d0d25]::run_compiler::{closure#1}>::{closure#1}>
  29:     0x7f88502b7581 - rustc_interface[dddbe1f16a5244f9]::interface::create_compiler_and_run::<core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>, rustc_driver[76860643812d0d25]::run_compiler::{closure#1}>
  30:     0x7f8850277302 - <scoped_tls[f70245fe0c4e39bd]::ScopedKey<rustc_span[9e88627fbda4c1cd]::SessionGlobals>>::set::<rustc_interface[dddbe1f16a5244f9]::interface::run_compiler<core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>, rustc_driver[76860643812d0d25]::run_compiler::{closure#1}>::{closure#0}, core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>>
  31:     0x7f8850282fa9 - std[ce4413523c7f5639]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[dddbe1f16a5244f9]::util::run_in_thread_pool_with_globals<rustc_interface[dddbe1f16a5244f9]::interface::run_compiler<core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>, rustc_driver[76860643812d0d25]::run_compiler::{closure#1}>::{closure#0}, core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>>::{closure#0}, core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>>
  32:     0x7f88502fdcae - std[ce4413523c7f5639]::panicking::try::<core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>, core[6c815afa72b208e9]::panic::unwind_safe::AssertUnwindSafe<<std[ce4413523c7f5639]::thread::Builder>::spawn_unchecked_<rustc_interface[dddbe1f16a5244f9]::util::run_in_thread_pool_with_globals<rustc_interface[dddbe1f16a5244f9]::interface::run_compiler<core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>, rustc_driver[76860643812d0d25]::run_compiler::{closure#1}>::{closure#0}, core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>>::{closure#0}, core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  33:     0x7f88502f8de0 - <<std[ce4413523c7f5639]::thread::Builder>::spawn_unchecked_<rustc_interface[dddbe1f16a5244f9]::util::run_in_thread_pool_with_globals<rustc_interface[dddbe1f16a5244f9]::interface::run_compiler<core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>, rustc_driver[76860643812d0d25]::run_compiler::{closure#1}>::{closure#0}, core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>>::{closure#0}, core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>>::{closure#1} as core[6c815afa72b208e9]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  34:     0x7f884f919b45 - std::sys::unix::thread::Thread::new::thread_start::hea2c413ca0105153
  35:     0x7f884f6b6b43 - <unknown>
  36:     0x7f884f748a00 - <unknown>
  37:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.65.0-nightly (29e5879de 2022-08-23) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
error: aborting due to previous error
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/async-await/drop-order/drop-order-for-temporary-in-tail-return-expr.rs#nomiropt stdout ----

error in revision `nomiropt`: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/drop-order/drop-order-for-temporary-in-tail-return-expr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "nomiropt" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/drop-order/drop-order-for-temporary-in-tail-return-expr.nomiropt/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-Z" "mir-opt-level=0" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/drop-order/drop-order-for-temporary-in-tail-return-expr.nomiropt/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: compiler/rustc_codegen_llvm/src/context.rs:992:21: `fn_abi_of_instance(<std::future::from_generator::GenFuture<[static generator@/checkout/src/test/ui/async-await/drop-order/drop-order-for-temporary-in-tail-return-expr.rs:53:33: 58:2]> as Future>::poll, [])` failed: the type `[type error]` has an unknown layout
   |
   |
LL |         fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {

thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1393:9
stack backtrace:
stack backtrace:
   0:     0x7f0382e59afc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hd897be327daa99cb
   1:     0x7f0382ec1fd8 - core::fmt::write::h76bfdbaa0723b595
   2:     0x7f0382e4a6d1 - std::io::Write::write_fmt::h7566bf0b3653ad51
   3:     0x7f0382e5cade - std::panicking::default_hook::{{closure}}::h52a899c6038ffa08
   4:     0x7f0382e5c7a7 - std::panicking::default_hook::hc3cc699b84eeea6f
   5:     0x7f03837e2af4 - rustc_driver[76860643812d0d25]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f0382e5d291 - std::panicking::rust_panic_with_hook::h3ff7d17b1bef18bf
   7:     0x7f0383b1c0f3 - std[ce4413523c7f5639]::panicking::begin_panic::<rustc_errors[7acd77a66447d1d8]::ExplicitBug>::{closure#0}
   8:     0x7f0383b13d76 - std[ce4413523c7f5639]::sys_common::backtrace::__rust_end_short_backtrace::<std[ce4413523c7f5639]::panicking::begin_panic<rustc_errors[7acd77a66447d1d8]::ExplicitBug>::{closure#0}, !>
   9:     0x7f0383602986 - std[ce4413523c7f5639]::panicking::begin_panic::<rustc_errors[7acd77a66447d1d8]::ExplicitBug>
  10:     0x7f0383af5a46 - std[ce4413523c7f5639]::panic::panic_any::<rustc_errors[7acd77a66447d1d8]::ExplicitBug>
  11:     0x7f0383af0c73 - <rustc_errors[7acd77a66447d1d8]::HandlerInner>::span_bug::<rustc_span[9e88627fbda4c1cd]::span_encoding::Span, &alloc[92664884fc986e55]::string::String>
  12:     0x7f0383af08d0 - <rustc_errors[7acd77a66447d1d8]::Handler>::span_bug::<rustc_span[9e88627fbda4c1cd]::span_encoding::Span, &alloc[92664884fc986e55]::string::String>
  13:     0x7f0383a32c42 - rustc_middle[f9a8a3b120de9b6f]::ty::context::tls::with_context_opt::<rustc_middle[f9a8a3b120de9b6f]::ty::context::tls::with_opt<rustc_middle[f9a8a3b120de9b6f]::util::bug::opt_span_bug_fmt<rustc_span[9e88627fbda4c1cd]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  14:     0x7f0383a32a79 - rustc_middle[f9a8a3b120de9b6f]::util::bug::opt_span_bug_fmt::<rustc_span[9e88627fbda4c1cd]::span_encoding::Span>
  15:     0x7f03835f861a - rustc_middle[f9a8a3b120de9b6f]::util::bug::span_bug_fmt::<rustc_span[9e88627fbda4c1cd]::span_encoding::Span>
  16:     0x7f03839d3554 - <rustc_codegen_llvm[7b178298eedd8479]::context::CodegenCx as rustc_middle[f9a8a3b120de9b6f]::ty::layout::FnAbiOfHelpers>::handle_fn_abi_err
  17:     0x7f03839c3995 - <rustc_codegen_llvm[7b178298eedd8479]::context::CodegenCx as rustc_middle[f9a8a3b120de9b6f]::ty::layout::FnAbiOf>::fn_abi_of_instance::{closure#0}
  18:     0x7f03839dddaa - <rustc_codegen_llvm[7b178298eedd8479]::context::CodegenCx as rustc_codegen_ssa[29e4e69b69f50724]::traits::declare::PreDefineMethods>::predefine_fn
  19:     0x7f0383ac28fe - rustc_codegen_llvm[7b178298eedd8479]::base::compile_codegen_unit::module_codegen
  20:     0x7f03839defdd - <rustc_query_system[47adcd4433a5da6d]::dep_graph::graph::DepGraph<rustc_middle[f9a8a3b120de9b6f]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[f9a8a3b120de9b6f]::ty::context::TyCtxt, rustc_span[9e88627fbda4c1cd]::symbol::Symbol, rustc_codegen_ssa[29e4e69b69f50724]::ModuleCodegen<rustc_codegen_llvm[7b178298eedd8479]::ModuleLlvm>>
  21:     0x7f0383ac257e - rustc_codegen_llvm[7b178298eedd8479]::base::compile_codegen_unit
  22:     0x7f0383a4901a - rustc_codegen_ssa[29e4e69b69f50724]::base::codegen_crate::<rustc_codegen_llvm[7b178298eedd8479]::LlvmCodegenBackend>
  23:     0x7f0383b084dd - <rustc_codegen_llvm[7b178298eedd8479]::LlvmCodegenBackend as rustc_codegen_ssa[29e4e69b69f50724]::traits::backend::CodegenBackend>::codegen_crate
  24:     0x7f038396588b - <rustc_session[d09f6f8e5f6d4931]::session::Session>::time::<alloc[92664884fc986e55]::boxed::Box<dyn core[6c815afa72b208e9]::any::Any>, rustc_interface[dddbe1f16a5244f9]::passes::start_codegen::{closure#0}>
  25:     0x7f03839235d5 - <rustc_interface[dddbe1f16a5244f9]::passes::QueryContext>::enter::<<rustc_interface[dddbe1f16a5244f9]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[6c815afa72b208e9]::result::Result<alloc[92664884fc986e55]::boxed::Box<dyn core[6c815afa72b208e9]::any::Any>, rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>>
  26:     0x7f03839082ad - <rustc_interface[dddbe1f16a5244f9]::queries::Queries>::ongoing_codegen
  27:     0x7f03837ebb16 - <rustc_interface[dddbe1f16a5244f9]::interface::Compiler>::enter::<rustc_driver[76860643812d0d25]::run_compiler::{closure#1}::{closure#2}, core[6c815afa72b208e9]::result::Result<core[6c815afa72b208e9]::option::Option<rustc_interface[dddbe1f16a5244f9]::queries::Linker>, rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>>
  28:     0x7f03837cf731 - rustc_span[9e88627fbda4c1cd]::with_source_map::<core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>, rustc_interface[dddbe1f16a5244f9]::interface::create_compiler_and_run<core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>, rustc_driver[76860643812d0d25]::run_compiler::{closure#1}>::{closure#1}>
  29:     0x7f0383807581 - rustc_interface[dddbe1f16a5244f9]::interface::create_compiler_and_run::<core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>, rustc_driver[76860643812d0d25]::run_compiler::{closure#1}>
  30:     0x7f03837c7302 - <scoped_tls[f70245fe0c4e39bd]::ScopedKey<rustc_span[9e88627fbda4c1cd]::SessionGlobals>>::set::<rustc_interface[dddbe1f16a5244f9]::interface::run_compiler<core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>, rustc_driver[76860643812d0d25]::run_compiler::{closure#1}>::{closure#0}, core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>>
  31:     0x7f03837d2fa9 - std[ce4413523c7f5639]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[dddbe1f16a5244f9]::util::run_in_thread_pool_with_globals<rustc_interface[dddbe1f16a5244f9]::interface::run_compiler<core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>, rustc_driver[76860643812d0d25]::run_compiler::{closure#1}>::{closure#0}, core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>>::{closure#0}, core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>>
  32:     0x7f038384dcae - std[ce4413523c7f5639]::panicking::try::<core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>, core[6c815afa72b208e9]::panic::unwind_safe::AssertUnwindSafe<<std[ce4413523c7f5639]::thread::Builder>::spawn_unchecked_<rustc_interface[dddbe1f16a5244f9]::util::run_in_thread_pool_with_globals<rustc_interface[dddbe1f16a5244f9]::interface::run_compiler<core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>, rustc_driver[76860643812d0d25]::run_compiler::{closure#1}>::{closure#0}, core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>>::{closure#0}, core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  33:     0x7f0383848de0 - <<std[ce4413523c7f5639]::thread::Builder>::spawn_unchecked_<rustc_interface[dddbe1f16a5244f9]::util::run_in_thread_pool_with_globals<rustc_interface[dddbe1f16a5244f9]::interface::run_compiler<core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>, rustc_driver[76860643812d0d25]::run_compiler::{closure#1}>::{closure#0}, core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>>::{closure#0}, core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>>::{closure#1} as core[6c815afa72b208e9]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  34:     0x7f0382e69b45 - std::sys::unix::thread::Thread::new::thread_start::hea2c413ca0105153
  35:     0x7f0382c06b43 - <unknown>
  36:     0x7f0382c98a00 - <unknown>
  37:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.65.0-nightly (29e5879de 2022-08-23) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 -Z mir-opt-level=0
query stack during panic:
end of query stack
error: aborting due to previous error
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/async-await/drop-order/drop-order-for-locals-when-cancelled.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/drop-order/drop-order-for-locals-when-cancelled.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/drop-order/drop-order-for-locals-when-cancelled/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/drop-order/drop-order-for-locals-when-cancelled/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: compiler/rustc_codegen_llvm/src/context.rs:992:21: `fn_abi_of_instance(<std::future::from_generator::GenFuture<[static generator@/checkout/src/test/ui/async-await/drop-order/drop-order-for-locals-when-cancelled.rs:65:79: 73:2]> as Future>::poll, [])` failed: the type `[type error]` has an unknown layout
   |
   |
LL |         fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {

thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1393:9
stack backtrace:
stack backtrace:
   0:     0x7fca0a48aafc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hd897be327daa99cb
   1:     0x7fca0a4f2fd8 - core::fmt::write::h76bfdbaa0723b595
   2:     0x7fca0a47b6d1 - std::io::Write::write_fmt::h7566bf0b3653ad51
   3:     0x7fca0a48dade - std::panicking::default_hook::{{closure}}::h52a899c6038ffa08
   4:     0x7fca0a48d7a7 - std::panicking::default_hook::hc3cc699b84eeea6f
   5:     0x7fca0ae13af4 - rustc_driver[76860643812d0d25]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fca0a48e291 - std::panicking::rust_panic_with_hook::h3ff7d17b1bef18bf
   7:     0x7fca0b14d0f3 - std[ce4413523c7f5639]::panicking::begin_panic::<rustc_errors[7acd77a66447d1d8]::ExplicitBug>::{closure#0}
   8:     0x7fca0b144d76 - std[ce4413523c7f5639]::sys_common::backtrace::__rust_end_short_backtrace::<std[ce4413523c7f5639]::panicking::begin_panic<rustc_errors[7acd77a66447d1d8]::ExplicitBug>::{closure#0}, !>
   9:     0x7fca0ac33986 - std[ce4413523c7f5639]::panicking::begin_panic::<rustc_errors[7acd77a66447d1d8]::ExplicitBug>
  10:     0x7fca0b126a46 - std[ce4413523c7f5639]::panic::panic_any::<rustc_errors[7acd77a66447d1d8]::ExplicitBug>
  11:     0x7fca0b121c73 - <rustc_errors[7acd77a66447d1d8]::HandlerInner>::span_bug::<rustc_span[9e88627fbda4c1cd]::span_encoding::Span, &alloc[92664884fc986e55]::string::String>
  12:     0x7fca0b1218d0 - <rustc_errors[7acd77a66447d1d8]::Handler>::span_bug::<rustc_span[9e88627fbda4c1cd]::span_encoding::Span, &alloc[92664884fc986e55]::string::String>
  13:     0x7fca0b063c42 - rustc_middle[f9a8a3b120de9b6f]::ty::context::tls::with_context_opt::<rustc_middle[f9a8a3b120de9b6f]::ty::context::tls::with_opt<rustc_middle[f9a8a3b120de9b6f]::util::bug::opt_span_bug_fmt<rustc_span[9e88627fbda4c1cd]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  14:     0x7fca0b063a79 - rustc_middle[f9a8a3b120de9b6f]::util::bug::opt_span_bug_fmt::<rustc_span[9e88627fbda4c1cd]::span_encoding::Span>
  15:     0x7fca0ac2961a - rustc_middle[f9a8a3b120de9b6f]::util::bug::span_bug_fmt::<rustc_span[9e88627fbda4c1cd]::span_encoding::Span>
  16:     0x7fca0b004554 - <rustc_codegen_llvm[7b178298eedd8479]::context::CodegenCx as rustc_middle[f9a8a3b120de9b6f]::ty::layout::FnAbiOfHelpers>::handle_fn_abi_err
  17:     0x7fca0aff4995 - <rustc_codegen_llvm[7b178298eedd8479]::context::CodegenCx as rustc_middle[f9a8a3b120de9b6f]::ty::layout::FnAbiOf>::fn_abi_of_instance::{closure#0}
  18:     0x7fca0b00edaa - <rustc_codegen_llvm[7b178298eedd8479]::context::CodegenCx as rustc_codegen_ssa[29e4e69b69f50724]::traits::declare::PreDefineMethods>::predefine_fn
  19:     0x7fca0b0f38fe - rustc_codegen_llvm[7b178298eedd8479]::base::compile_codegen_unit::module_codegen
  20:     0x7fca0b00ffdd - <rustc_query_system[47adcd4433a5da6d]::dep_graph::graph::DepGraph<rustc_middle[f9a8a3b120de9b6f]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[f9a8a3b120de9b6f]::ty::context::TyCtxt, rustc_span[9e88627fbda4c1cd]::symbol::Symbol, rustc_codegen_ssa[29e4e69b69f50724]::ModuleCodegen<rustc_codegen_llvm[7b178298eedd8479]::ModuleLlvm>>
  21:     0x7fca0b0f357e - rustc_codegen_llvm[7b178298eedd8479]::base::compile_codegen_unit
  22:     0x7fca0b07a01a - rustc_codegen_ssa[29e4e69b69f50724]::base::codegen_crate::<rustc_codegen_llvm[7b178298eedd8479]::LlvmCodegenBackend>
  23:     0x7fca0b1394dd - <rustc_codegen_llvm[7b178298eedd8479]::LlvmCodegenBackend as rustc_codegen_ssa[29e4e69b69f50724]::traits::backend::CodegenBackend>::codegen_crate
  24:     0x7fca0af9688b - <rustc_session[d09f6f8e5f6d4931]::session::Session>::time::<alloc[92664884fc986e55]::boxed::Box<dyn core[6c815afa72b208e9]::any::Any>, rustc_interface[dddbe1f16a5244f9]::passes::start_codegen::{closure#0}>
  25:     0x7fca0af545d5 - <rustc_interface[dddbe1f16a5244f9]::passes::QueryContext>::enter::<<rustc_interface[dddbe1f16a5244f9]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[6c815afa72b208e9]::result::Result<alloc[92664884fc986e55]::boxed::Box<dyn core[6c815afa72b208e9]::any::Any>, rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>>
  26:     0x7fca0af392ad - <rustc_interface[dddbe1f16a5244f9]::queries::Queries>::ongoing_codegen
  27:     0x7fca0ae1cb16 - <rustc_interface[dddbe1f16a5244f9]::interface::Compiler>::enter::<rustc_driver[76860643812d0d25]::run_compiler::{closure#1}::{closure#2}, core[6c815afa72b208e9]::result::Result<core[6c815afa72b208e9]::option::Option<rustc_interface[dddbe1f16a5244f9]::queries::Linker>, rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>>
  28:     0x7fca0ae00731 - rustc_span[9e88627fbda4c1cd]::with_source_map::<core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>, rustc_interface[dddbe1f16a5244f9]::interface::create_compiler_and_run<core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>, rustc_driver[76860643812d0d25]::run_compiler::{closure#1}>::{closure#1}>
  29:     0x7fca0ae38581 - rustc_interface[dddbe1f16a5244f9]::interface::create_compiler_and_run::<core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>, rustc_driver[76860643812d0d25]::run_compiler::{closure#1}>
  30:     0x7fca0adf8302 - <scoped_tls[f70245fe0c4e39bd]::ScopedKey<rustc_span[9e88627fbda4c1cd]::SessionGlobals>>::set::<rustc_interface[dddbe1f16a5244f9]::interface::run_compiler<core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>, rustc_driver[76860643812d0d25]::run_compiler::{closure#1}>::{closure#0}, core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>>
  31:     0x7fca0ae03fa9 - std[ce4413523c7f5639]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[dddbe1f16a5244f9]::util::run_in_thread_pool_with_globals<rustc_interface[dddbe1f16a5244f9]::interface::run_compiler<core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>, rustc_driver[76860643812d0d25]::run_compiler::{closure#1}>::{closure#0}, core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>>::{closure#0}, core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>>
  32:     0x7fca0ae7ecae - std[ce4413523c7f5639]::panicking::try::<core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>, core[6c815afa72b208e9]::panic::unwind_safe::AssertUnwindSafe<<std[ce4413523c7f5639]::thread::Builder>::spawn_unchecked_<rustc_interface[dddbe1f16a5244f9]::util::run_in_thread_pool_with_globals<rustc_interface[dddbe1f16a5244f9]::interface::run_compiler<core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>, rustc_driver[76860643812d0d25]::run_compiler::{closure#1}>::{closure#0}, core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>>::{closure#0}, core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  33:     0x7fca0ae79de0 - <<std[ce4413523c7f5639]::thread::Builder>::spawn_unchecked_<rustc_interface[dddbe1f16a5244f9]::util::run_in_thread_pool_with_globals<rustc_interface[dddbe1f16a5244f9]::interface::run_compiler<core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>, rustc_driver[76860643812d0d25]::run_compiler::{closure#1}>::{closure#0}, core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>>::{closure#0}, core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>>::{closure#1} as core[6c815afa72b208e9]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  34:     0x7fca0a49ab45 - std::sys::unix::thread::Thread::new::thread_start::hea2c413ca0105153
  35:     0x7fca0a237b43 - <unknown>
  36:     0x7fca0a2c9a00 - <unknown>
  37:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.65.0-nightly (29e5879de 2022-08-23) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
error: aborting due to previous error
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/async-await/issue-72470-llvm-dominate.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issue-72470-llvm-dominate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-72470-llvm-dominate" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "opt-level=3" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-72470-llvm-dominate/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: compiler/rustc_codegen_llvm/src/context.rs:992:21: `fn_abi_of_instance(<std::future::from_generator::GenFuture<[static generator@/checkout/src/test/ui/async-await/issue-72470-llvm-dominate.rs:32:37: 65:6]> as Future>::poll, [])` failed: the type `[type error]` has an unknown layout
   |
   |
LL |         fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {

thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1393:9
stack backtrace:
stack backtrace:
   0:     0x7f2bc0f53afc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hd897be327daa99cb
   1:     0x7f2bc0fbbfd8 - core::fmt::write::h76bfdbaa0723b595
   2:     0x7f2bc0f446d1 - std::io::Write::write_fmt::h7566bf0b3653ad51
   3:     0x7f2bc0f56ade - std::panicking::default_hook::{{closure}}::h52a899c6038ffa08
   4:     0x7f2bc0f567a7 - std::panicking::default_hook::hc3cc699b84eeea6f
   5:     0x7f2bc18dcaf4 - rustc_driver[76860643812d0d25]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f2bc0f57291 - std::panicking::rust_panic_with_hook::h3ff7d17b1bef18bf
   7:     0x7f2bc1c160f3 - std[ce4413523c7f5639]::panicking::begin_panic::<rustc_errors[7acd77a66447d1d8]::ExplicitBug>::{closure#0}
   8:     0x7f2bc1c0dd76 - std[ce4413523c7f5639]::sys_common::backtrace::__rust_end_short_backtrace::<std[ce4413523c7f5639]::panicking::begin_panic<rustc_errors[7acd77a66447d1d8]::ExplicitBug>::{closure#0}, !>
   9:     0x7f2bc16fc986 - std[ce4413523c7f5639]::panicking::begin_panic::<rustc_errors[7acd77a66447d1d8]::ExplicitBug>
  10:     0x7f2bc1befa46 - std[ce4413523c7f5639]::panic::panic_any::<rustc_errors[7acd77a66447d1d8]::ExplicitBug>
  11:     0x7f2bc1beac73 - <rustc_errors[7acd77a66447d1d8]::HandlerInner>::span_bug::<rustc_span[9e88627fbda4c1cd]::span_encoding::Span, &alloc[92664884fc986e55]::string::String>
  12:     0x7f2bc1bea8d0 - <rustc_errors[7acd77a66447d1d8]::Handler>::span_bug::<rustc_span[9e88627fbda4c1cd]::span_encoding::Span, &alloc[92664884fc986e55]::string::String>
  13:     0x7f2bc1b2cc42 - rustc_middle[f9a8a3b120de9b6f]::ty::context::tls::with_context_opt::<rustc_middle[f9a8a3b120de9b6f]::ty::context::tls::with_opt<rustc_middle[f9a8a3b120de9b6f]::util::bug::opt_span_bug_fmt<rustc_span[9e88627fbda4c1cd]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  14:     0x7f2bc1b2ca79 - rustc_middle[f9a8a3b120de9b6f]::util::bug::opt_span_bug_fmt::<rustc_span[9e88627fbda4c1cd]::span_encoding::Span>
  15:     0x7f2bc16f261a - rustc_middle[f9a8a3b120de9b6f]::util::bug::span_bug_fmt::<rustc_span[9e88627fbda4c1cd]::span_encoding::Span>
  16:     0x7f2bc1acd554 - <rustc_codegen_llvm[7b178298eedd8479]::context::CodegenCx as rustc_middle[f9a8a3b120de9b6f]::ty::layout::FnAbiOfHelpers>::handle_fn_abi_err
  17:     0x7f2bc1abd995 - <rustc_codegen_llvm[7b178298eedd8479]::context::CodegenCx as rustc_middle[f9a8a3b120de9b6f]::ty::layout::FnAbiOf>::fn_abi_of_instance::{closure#0}
  18:     0x7f2bc1ad7daa - <rustc_codegen_llvm[7b178298eedd8479]::context::CodegenCx as rustc_codegen_ssa[29e4e69b69f50724]::traits::declare::PreDefineMethods>::predefine_fn
  19:     0x7f2bc1bbc8fe - rustc_codegen_llvm[7b178298eedd8479]::base::compile_codegen_unit::module_codegen
  20:     0x7f2bc1ad8fdd - <rustc_query_system[47adcd4433a5da6d]::dep_graph::graph::DepGraph<rustc_middle[f9a8a3b120de9b6f]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[f9a8a3b120de9b6f]::ty::context::TyCtxt, rustc_span[9e88627fbda4c1cd]::symbol::Symbol, rustc_codegen_ssa[29e4e69b69f50724]::ModuleCodegen<rustc_codegen_llvm[7b178298eedd8479]::ModuleLlvm>>
  21:     0x7f2bc1bbc57e - rustc_codegen_llvm[7b178298eedd8479]::base::compile_codegen_unit
  22:     0x7f2bc1b4301a - rustc_codegen_ssa[29e4e69b69f50724]::base::codegen_crate::<rustc_codegen_llvm[7b178298eedd8479]::LlvmCodegenBackend>
  23:     0x7f2bc1c024dd - <rustc_codegen_llvm[7b178298eedd8479]::LlvmCodegenBackend as rustc_codegen_ssa[29e4e69b69f50724]::traits::backend::CodegenBackend>::codegen_crate
  24:     0x7f2bc1a5f88b - <rustc_session[d09f6f8e5f6d4931]::session::Session>::time::<alloc[92664884fc986e55]::boxed::Box<dyn core[6c815afa72b208e9]::any::Any>, rustc_interface[dddbe1f16a5244f9]::passes::start_codegen::{closure#0}>
  25:     0x7f2bc1a1d5d5 - <rustc_interface[dddbe1f16a5244f9]::passes::QueryContext>::enter::<<rustc_interface[dddbe1f16a5244f9]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[6c815afa72b208e9]::result::Result<alloc[92664884fc986e55]::boxed::Box<dyn core[6c815afa72b208e9]::any::Any>, rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>>
  26:     0x7f2bc1a022ad - <rustc_interface[dddbe1f16a5244f9]::queries::Queries>::ongoing_codegen
  27:     0x7f2bc18e5b16 - <rustc_interface[dddbe1f16a5244f9]::interface::Compiler>::enter::<rustc_driver[76860643812d0d25]::run_compiler::{closure#1}::{closure#2}, core[6c815afa72b208e9]::result::Result<core[6c815afa72b208e9]::option::Option<rustc_interface[dddbe1f16a5244f9]::queries::Linker>, rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>>
  28:     0x7f2bc18c9731 - rustc_span[9e88627fbda4c1cd]::with_source_map::<core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>, rustc_interface[dddbe1f16a5244f9]::interface::create_compiler_and_run<core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>, rustc_driver[76860643812d0d25]::run_compiler::{closure#1}>::{closure#1}>
  29:     0x7f2bc1901581 - rustc_interface[dddbe1f16a5244f9]::interface::create_compiler_and_run::<core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>, rustc_driver[76860643812d0d25]::run_compiler::{closure#1}>
  30:     0x7f2bc18c1302 - <scoped_tls[f70245fe0c4e39bd]::ScopedKey<rustc_span[9e88627fbda4c1cd]::SessionGlobals>>::set::<rustc_interface[dddbe1f16a5244f9]::interface::run_compiler<core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>, rustc_driver[76860643812d0d25]::run_compiler::{closure#1}>::{closure#0}, core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>>
  31:     0x7f2bc18ccfa9 - std[ce4413523c7f5639]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[dddbe1f16a5244f9]::util::run_in_thread_pool_with_globals<rustc_interface[dddbe1f16a5244f9]::interface::run_compiler<core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>, rustc_driver[76860643812d0d25]::run_compiler::{closure#1}>::{closure#0}, core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>>::{closure#0}, core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>>
  32:     0x7f2bc1947cae - std[ce4413523c7f5639]::panicking::try::<core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>, core[6c815afa72b208e9]::panic::unwind_safe::AssertUnwindSafe<<std[ce4413523c7f5639]::thread::Builder>::spawn_unchecked_<rustc_interface[dddbe1f16a5244f9]::util::run_in_thread_pool_with_globals<rustc_interface[dddbe1f16a5244f9]::interface::run_compiler<core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>, rustc_driver[76860643812d0d25]::run_compiler::{closure#1}>::{closure#0}, core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>>::{closure#0}, core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  33:     0x7f2bc1942de0 - <<std[ce4413523c7f5639]::thread::Builder>::spawn_unchecked_<rustc_interface[dddbe1f16a5244f9]::util::run_in_thread_pool_with_globals<rustc_interface[dddbe1f16a5244f9]::interface::run_compiler<core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>, rustc_driver[76860643812d0d25]::run_compiler::{closure#1}>::{closure#0}, core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>>::{closure#0}, core[6c815afa72b208e9]::result::Result<(), rustc_errors[7acd77a66447d1d8]::ErrorGuaranteed>>::{closure#1} as core[6c815afa72b208e9]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  34:     0x7f2bc0f63b45 - std::sys::unix::thread::Thread::new::thread_start::hea2c413ca0105153
  35:     0x7f2bc0d00b43 - <unknown>
  36:     0x7f2bc0d92a00 - <unknown>
  37:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.65.0-nightly (29e5879de 2022-08-23) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 -C opt-level=3
query stack during panic:
end of query stack
error: aborting due to previous error
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/drop/dynamic-drop-async.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/drop/dynamic-drop-async.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/drop/dynamic-drop-async/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/drop/dynamic-drop-async/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: compiler/rustc_codegen_llvm/src/context.rs:992:21: `fn_abi_of_instance(<std::future::from_generator::GenFuture<[static generator@/checkout/src/test/ui/drop/dynamic-drop-async.rs:99:50: 104:2]> as Future>::poll, [])` failed: the type `[type error]` has an unknown layout
   |
   |
LL |         fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {

thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1393:9
stack backtrace:
   0:     0x7fd0fc62cafc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hd897be327daa99cb
   0:     0x7fd0fc62cafc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hd897be327daa99cb
   1:     0x7fd0fc694fd8 - core::fmt::write::h76bfdbaa0723b595
   2:     0x7fd0fc61d6d1 - std::io::Write::write_fmt::h7566bf0b3653ad51
   3:     0x7fd0fc62fade - std::panicking::default_hook::{{closure}}::h52a899c6038ffa08
   4:     0x7fd0fc62f7a7 - std::panicking::default_hook::hc3cc699b84eeea6f
   5:     0x7fd0fcfb5af4 - rustc_driver[76860643812d0d25]::DEFAULT_HOOK::{closure#0}::{closure#0}
