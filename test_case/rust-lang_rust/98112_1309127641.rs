plain
   Compiling termcolor v1.1.2
   Compiling getopts v0.2.21
   Compiling rustc_fs_util v0.0.0 (/checkout/compiler/rustc_fs_util)
   Compiling either v1.6.0
error: internal compiler error: /checkout/compiler/rustc_codegen_ssa/src/mir/operand.rs:113:18: not immediate: OperandRef(Pair((%"alloc::sync::ArcInner<dyn subscriber::Subscriber + core::marker::Send + core::marker::Sync>"*:  %18 = bitcast i64* %15 to %"alloc::sync::ArcInner<dyn subscriber::Subscriber + core::marker::Send + core::marker::Sync>"*), ([3 x i64]*:  %19 = bitcast i64* %17 to [3 x i64]*)) @ TyAndLayout { ty: *mut alloc::sync::ArcInner<dyn subscriber::Subscriber + std::marker::Send + std::marker::Sync>, layout: Layout { size: Size(16 bytes), align: AbiAndPrefAlign { abi: Align(8 bytes), pref: Align(8 bytes) }, abi: ScalarPair(Initialized { value: Pointer, valid_range: 0..=18446744073709551615 }, Initialized { value: Pointer, valid_range: 1..=18446744073709551615 }), fields: Arbitrary { offsets: [Size(0 bytes), Size(8 bytes)], memory_index: [0, 1] }, largest_niche: Some(Niche { offset: Size(8 bytes), value: Pointer, valid_range: 1..=18446744073709551615 }), variants: Single { index: 0 } } })
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1551:9
stack backtrace:
stack backtrace:
   0:     0x7ff913308572 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h751d52596fbe295b
   1:     0x7ff913376308 - core::fmt::write::h6d6e96066401bc0f
   2:     0x7ff9132f8da1 - std::io::Write::write_fmt::h9fe4e6d9c9b927ef
   3:     0x7ff913308335 - std::sys_common::backtrace::print::hf38d1633e21dbba9
   4:     0x7ff91330b717 - std::panicking::default_hook::{{closure}}::h12022b1a20dd35ce
   5:     0x7ff91330b475 - std::panicking::default_hook::h5ab5b9712723f5dd
   Compiling rustc_apfloat v0.0.0 (/checkout/compiler/rustc_apfloat)
   Compiling rustc_apfloat v0.0.0 (/checkout/compiler/rustc_apfloat)
   6:     0x7ff913c8b0a4 - rustc_driver[b2f871249018b53f]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7ff91330c023 - std::panicking::rust_panic_with_hook::h57cd9b8bb3f6a82b
   8:     0x7ff9166885a3 - std[389b380b19480123]::panicking::begin_panic::<rustc_errors[c6390cd32a21b219]::ExplicitBug>::{closure#0}
   9:     0x7ff916688296 - std[389b380b19480123]::sys_common::backtrace::__rust_end_short_backtrace::<std[389b380b19480123]::panicking::begin_panic<rustc_errors[c6390cd32a21b219]::ExplicitBug>::{closure#0}, !>
  10:     0x7ff913c32356 - std[389b380b19480123]::panicking::begin_panic::<rustc_errors[c6390cd32a21b219]::ExplicitBug>
  11:     0x7ff916726696 - std[389b380b19480123]::panic::panic_any::<rustc_errors[c6390cd32a21b219]::ExplicitBug>
  12:     0x7ff916721867 - <rustc_errors[c6390cd32a21b219]::HandlerInner>::bug::<&alloc[a40b22d2678a71d4]::string::String>
  13:     0x7ff916721350 - <rustc_errors[c6390cd32a21b219]::Handler>::bug::<&alloc[a40b22d2678a71d4]::string::String>
  14:     0x7ff9167e8b10 - rustc_middle[890156e5e48c7d12]::ty::context::tls::with_context_opt::<rustc_middle[890156e5e48c7d12]::ty::context::tls::with_opt<rustc_middle[890156e5e48c7d12]::util::bug::opt_span_bug_fmt<rustc_span[b4ca3e966db910d5]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  15:     0x7ff9167e9d69 - rustc_middle[890156e5e48c7d12]::util::bug::opt_span_bug_fmt::<rustc_span[b4ca3e966db910d5]::span_encoding::Span>
  16:     0x7ff913c37745 - rustc_middle[890156e5e48c7d12]::util::bug::bug_fmt
  17:     0x7ff913ee2666 - <rustc_codegen_ssa[8adc42e30123fca]::mir::operand::OperandRef<&rustc_codegen_llvm[b9a1145668b1a60a]::llvm_::ffi::Value>>::immediate
  18:     0x7ff913fbb1da - <rustc_codegen_ssa[8adc42e30123fca]::mir::FunctionCx<rustc_codegen_llvm[b9a1145668b1a60a]::builder::Builder>>::codegen_rvalue_operand
  19:     0x7ff913fb59cb - rustc_codegen_ssa[8adc42e30123fca]::mir::codegen_mir::<rustc_codegen_llvm[b9a1145668b1a60a]::builder::Builder>
  20:     0x7ff913ef5eec - rustc_codegen_ssa[8adc42e30123fca]::base::codegen_instance::<rustc_codegen_llvm[b9a1145668b1a60a]::builder::Builder>
  21:     0x7ff913f772b3 - rustc_codegen_llvm[b9a1145668b1a60a]::base::compile_codegen_unit::module_codegen
  22:     0x7ff913e88f5e - <rustc_query_system[883dbf6bccf65d97]::dep_graph::graph::DepGraph<rustc_middle[890156e5e48c7d12]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[890156e5e48c7d12]::ty::context::TyCtxt, rustc_span[b4ca3e966db910d5]::symbol::Symbol, rustc_codegen_ssa[8adc42e30123fca]::ModuleCodegen<rustc_codegen_llvm[b9a1145668b1a60a]::ModuleLlvm>>
  23:     0x7ff913f7650d - rustc_codegen_llvm[b9a1145668b1a60a]::base::compile_codegen_unit
  24:     0x7ff913ef523d - rustc_codegen_ssa[8adc42e30123fca]::base::codegen_crate::<rustc_codegen_llvm[b9a1145668b1a60a]::LlvmCodegenBackend>
  25:     0x7ff913fe798d - <rustc_codegen_llvm[b9a1145668b1a60a]::LlvmCodegenBackend as rustc_codegen_ssa[8adc42e30123fca]::traits::backend::CodegenBackend>::codegen_crate
  26:     0x7ff913da3b9b - <rustc_session[9207d45749bb4341]::session::Session>::time::<alloc[a40b22d2678a71d4]::boxed::Box<dyn core[69c2305d6fa5d54f]::any::Any>, rustc_interface[f22dc26c8a2764b8]::passes::start_codegen::{closure#0}>
  27:     0x7ff913dced08 - <rustc_interface[f22dc26c8a2764b8]::passes::QueryContext>::enter::<<rustc_interface[f22dc26c8a2764b8]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[69c2305d6fa5d54f]::result::Result<alloc[a40b22d2678a71d4]::boxed::Box<dyn core[69c2305d6fa5d54f]::any::Any>, rustc_errors[c6390cd32a21b219]::ErrorGuaranteed>>
  28:     0x7ff913db5ad2 - <rustc_interface[f22dc26c8a2764b8]::queries::Queries>::ongoing_codegen
  29:     0x7ff913cf88ab - <rustc_interface[f22dc26c8a2764b8]::interface::Compiler>::enter::<rustc_driver[b2f871249018b53f]::run_compiler::{closure#1}::{closure#2}, core[69c2305d6fa5d54f]::result::Result<core[69c2305d6fa5d54f]::option::Option<rustc_interface[f22dc26c8a2764b8]::queries::Linker>, rustc_errors[c6390cd32a21b219]::ErrorGuaranteed>>
  30:     0x7ff913c8c772 - rustc_span[b4ca3e966db910d5]::with_source_map::<core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[c6390cd32a21b219]::ErrorGuaranteed>, rustc_interface[f22dc26c8a2764b8]::interface::run_compiler<core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[c6390cd32a21b219]::ErrorGuaranteed>, rustc_driver[b2f871249018b53f]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  31:     0x7ff913ceb92c - <scoped_tls[ce9fa241ba16890b]::ScopedKey<rustc_span[b4ca3e966db910d5]::SessionGlobals>>::set::<rustc_interface[f22dc26c8a2764b8]::interface::run_compiler<core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[c6390cd32a21b219]::ErrorGuaranteed>, rustc_driver[b2f871249018b53f]::run_compiler::{closure#1}>::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[c6390cd32a21b219]::ErrorGuaranteed>>
  32:     0x7ff913caa71a - std[389b380b19480123]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[f22dc26c8a2764b8]::util::run_in_thread_pool_with_globals<rustc_interface[f22dc26c8a2764b8]::interface::run_compiler<core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[c6390cd32a21b219]::ErrorGuaranteed>, rustc_driver[b2f871249018b53f]::run_compiler::{closure#1}>::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[c6390cd32a21b219]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[c6390cd32a21b219]::ErrorGuaranteed>>
  33:     0x7ff913cec956 - std[389b380b19480123]::panic::catch_unwind::<core[69c2305d6fa5d54f]::panic::unwind_safe::AssertUnwindSafe<<std[389b380b19480123]::thread::Builder>::spawn_unchecked_<rustc_interface[f22dc26c8a2764b8]::util::run_in_thread_pool_with_globals<rustc_interface[f22dc26c8a2764b8]::interface::run_compiler<core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[c6390cd32a21b219]::ErrorGuaranteed>, rustc_driver[b2f871249018b53f]::run_compiler::{closure#1}>::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[c6390cd32a21b219]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[c6390cd32a21b219]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[c6390cd32a21b219]::ErrorGuaranteed>>
  34:     0x7ff913c9ccd9 - <<std[389b380b19480123]::thread::Builder>::spawn_unchecked_<rustc_interface[f22dc26c8a2764b8]::util::run_in_thread_pool_with_globals<rustc_interface[f22dc26c8a2764b8]::interface::run_compiler<core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[c6390cd32a21b219]::ErrorGuaranteed>, rustc_driver[b2f871249018b53f]::run_compiler::{closure#1}>::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[c6390cd32a21b219]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[c6390cd32a21b219]::ErrorGuaranteed>>::{closure#1} as core[69c2305d6fa5d54f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  35:     0x7ff913318c8e - std::sys::unix::thread::Thread::new::thread_start::hb7aff98f3d211bd8
  36:     0x7ff9130aeb43 - <unknown>
  37:     0x7ff913140a00 - <unknown>
  38:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.67.0-nightly (036219742 2022-11-09) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=v0 -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C split-debuginfo=off -Z unstable-options -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -Z binary-dep-depinfo -Z tls-model=initial-exec -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
