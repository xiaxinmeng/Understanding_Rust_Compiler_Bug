plain
   Compiling hashbrown v0.12.3
   Compiling std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
   Compiling miniz_oxide v0.4.0
   Compiling addr2line v0.16.0
error: internal compiler error: /checkout/compiler/rustc_codegen_ssa/src/mir/operand.rs:113:18: not immediate: OperandRef(Pair(([0 x %"addr2line::function::FunctionAddress"]*:  %157 = bitcast i64* %156 to [0 x %"addr2line::function::FunctionAddress"]*), (i64:  %155 = load i64, i64* %154, align 8)) @ TyAndLayout { ty: *const [addr2line::function::FunctionAddress], layout: Layout { size: Size(16 bytes), align: AbiAndPrefAlign { abi: Align(8 bytes), pref: Align(8 bytes) }, abi: ScalarPair(Initialized { value: Pointer, valid_range: 0..=18446744073709551615 }, Initialized { value: Int(I64, false), valid_range: 0..=18446744073709551615 }), fields: Arbitrary { offsets: [Size(0 bytes), Size(8 bytes)], memory_index: [0, 1] }, largest_niche: None, variants: Single { index: 0 } } })
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1458:9
stack backtrace:
   0:     0x7f0265ab12c0 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h7aceaaec384abd8d
   1:     0x7f0265b1bc59 - core::fmt::write::h1935094ec1b611e9
   1:     0x7f0265b1bc59 - core::fmt::write::h1935094ec1b611e9
   2:     0x7f0265aa1a91 - std::io::Write::write_fmt::h924a08855ab35277
   3:     0x7f0265ab451e - std::panicking::default_hook::{{closure}}::hdcba8bc567463877
   4:     0x7f0265ab41eb - std::panicking::default_hook::h5e023012d652ad1c
   5:     0x7f0266491ea4 - rustc_driver[2b12b44ab49049e7]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f0265ab4cf1 - std::panicking::rust_panic_with_hook::hc50598e1e398727d
   7:     0x7f0269268d43 - std[8a3c335779a4ef7b]::panicking::begin_panic::<rustc_errors[a9e596aa33e6456]::ExplicitBug>::{closure#0}
   8:     0x7f02692684d6 - std[8a3c335779a4ef7b]::sys_common::backtrace::__rust_end_short_backtrace::<std[8a3c335779a4ef7b]::panicking::begin_panic<rustc_errors[a9e596aa33e6456]::ExplicitBug>::{closure#0}, !>
   9:     0x7f026642d2f6 - std[8a3c335779a4ef7b]::panicking::begin_panic::<rustc_errors[a9e596aa33e6456]::ExplicitBug>
  10:     0x7f02692ed126 - std[8a3c335779a4ef7b]::panic::panic_any::<rustc_errors[a9e596aa33e6456]::ExplicitBug>
  11:     0x7f02692e7519 - <rustc_errors[a9e596aa33e6456]::HandlerInner>::bug::<&alloc[45623a189840f9f9]::string::String>
  12:     0x7f02692e6ff0 - <rustc_errors[a9e596aa33e6456]::Handler>::bug::<&alloc[45623a189840f9f9]::string::String>
  13:     0x7f02693583be - rustc_middle[df447d51b984e2b2]::ty::context::tls::with_context_opt::<rustc_middle[df447d51b984e2b2]::ty::context::tls::with_opt<rustc_middle[df447d51b984e2b2]::util::bug::opt_span_bug_fmt<rustc_span[8c7477ded0a91ee5]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  14:     0x7f026935ba29 - rustc_middle[df447d51b984e2b2]::util::bug::opt_span_bug_fmt::<rustc_span[8c7477ded0a91ee5]::span_encoding::Span>
  15:     0x7f0266437395 - rustc_middle[df447d51b984e2b2]::util::bug::bug_fmt
  16:     0x7f026672f506 - <rustc_codegen_ssa[e12a987a4a0cb9e2]::mir::operand::OperandRef<&rustc_codegen_llvm[c114de5f8d8bff59]::llvm_::ffi::Value>>::immediate
  17:     0x7f026682508f - <rustc_codegen_ssa[e12a987a4a0cb9e2]::mir::FunctionCx<rustc_codegen_llvm[c114de5f8d8bff59]::builder::Builder>>::codegen_rvalue_operand
  18:     0x7f026681f729 - rustc_codegen_ssa[e12a987a4a0cb9e2]::mir::codegen_mir::<rustc_codegen_llvm[c114de5f8d8bff59]::builder::Builder>
  19:     0x7f0266740248 - rustc_codegen_ssa[e12a987a4a0cb9e2]::base::codegen_instance::<rustc_codegen_llvm[c114de5f8d8bff59]::builder::Builder>
  20:     0x7f02667c5ae2 - rustc_codegen_llvm[c114de5f8d8bff59]::base::compile_codegen_unit::module_codegen
  21:     0x7f0266703d5a - <rustc_query_system[75c107cad7cbc69e]::dep_graph::graph::DepGraph<rustc_middle[df447d51b984e2b2]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[df447d51b984e2b2]::ty::context::TyCtxt, rustc_span[8c7477ded0a91ee5]::symbol::Symbol, rustc_codegen_ssa[e12a987a4a0cb9e2]::ModuleCodegen<rustc_codegen_llvm[c114de5f8d8bff59]::ModuleLlvm>>
  22:     0x7f02667c4f1e - rustc_codegen_llvm[c114de5f8d8bff59]::base::compile_codegen_unit
  23:     0x7f026673f3a8 - rustc_codegen_ssa[e12a987a4a0cb9e2]::base::codegen_crate::<rustc_codegen_llvm[c114de5f8d8bff59]::LlvmCodegenBackend>
  24:     0x7f02666c635d - <rustc_codegen_llvm[c114de5f8d8bff59]::LlvmCodegenBackend as rustc_codegen_ssa[e12a987a4a0cb9e2]::traits::backend::CodegenBackend>::codegen_crate
  25:     0x7f026663d15b - <rustc_session[5e51afea9c8b884e]::session::Session>::time::<alloc[45623a189840f9f9]::boxed::Box<dyn core[c1e30f1bd259d119]::any::Any>, rustc_interface[4989f6b1fe17a7df]::passes::start_codegen::{closure#0}>
  26:     0x7f02665e95f5 - <rustc_interface[4989f6b1fe17a7df]::passes::QueryContext>::enter::<<rustc_interface[4989f6b1fe17a7df]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[c1e30f1bd259d119]::result::Result<alloc[45623a189840f9f9]::boxed::Box<dyn core[c1e30f1bd259d119]::any::Any>, rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>>
  27:     0x7f02665d27cd - <rustc_interface[4989f6b1fe17a7df]::queries::Queries>::ongoing_codegen
  28:     0x7f0266493d87 - <rustc_interface[4989f6b1fe17a7df]::interface::Compiler>::enter::<rustc_driver[2b12b44ab49049e7]::run_compiler::{closure#1}::{closure#2}, core[c1e30f1bd259d119]::result::Result<core[c1e30f1bd259d119]::option::Option<rustc_interface[4989f6b1fe17a7df]::queries::Linker>, rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>>
  29:     0x7f026647da70 - rustc_span[8c7477ded0a91ee5]::with_source_map::<core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>, rustc_interface[4989f6b1fe17a7df]::interface::create_compiler_and_run<core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>, rustc_driver[2b12b44ab49049e7]::run_compiler::{closure#1}>::{closure#1}>
  30:     0x7f0266494ca2 - rustc_interface[4989f6b1fe17a7df]::interface::create_compiler_and_run::<core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>, rustc_driver[2b12b44ab49049e7]::run_compiler::{closure#1}>
  31:     0x7f0266516d9f - <scoped_tls[5efd78fa53ce51fd]::ScopedKey<rustc_span[8c7477ded0a91ee5]::SessionGlobals>>::set::<rustc_interface[4989f6b1fe17a7df]::interface::run_compiler<core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>, rustc_driver[2b12b44ab49049e7]::run_compiler::{closure#1}>::{closure#0}, core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>>
  32:     0x7f026650062f - std[8a3c335779a4ef7b]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[4989f6b1fe17a7df]::util::run_in_thread_pool_with_globals<rustc_interface[4989f6b1fe17a7df]::interface::run_compiler<core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>, rustc_driver[2b12b44ab49049e7]::run_compiler::{closure#1}>::{closure#0}, core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>>::{closure#0}, core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>>
  33:     0x7f026647f3c1 - std[8a3c335779a4ef7b]::panic::catch_unwind::<core[c1e30f1bd259d119]::panic::unwind_safe::AssertUnwindSafe<<std[8a3c335779a4ef7b]::thread::Builder>::spawn_unchecked_<rustc_interface[4989f6b1fe17a7df]::util::run_in_thread_pool_with_globals<rustc_interface[4989f6b1fe17a7df]::interface::run_compiler<core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>, rustc_driver[2b12b44ab49049e7]::run_compiler::{closure#1}>::{closure#0}, core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>>::{closure#0}, core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>>
  34:     0x7f02665028a2 - <<std[8a3c335779a4ef7b]::thread::Builder>::spawn_unchecked_<rustc_interface[4989f6b1fe17a7df]::util::run_in_thread_pool_with_globals<rustc_interface[4989f6b1fe17a7df]::interface::run_compiler<core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>, rustc_driver[2b12b44ab49049e7]::run_compiler::{closure#1}>::{closure#0}, core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>>::{closure#0}, core[c1e30f1bd259d119]::result::Result<(), rustc_errors[a9e596aa33e6456]::ErrorGuaranteed>>::{closure#1} as core[c1e30f1bd259d119]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  35:     0x7f0265ac0f35 - std::sys::unix::thread::Thread::new::thread_start::h3a069647ae68a533
  36:     0x7f026585db43 - <unknown>
  37:     0x7f02658efa00 - <unknown>
  38:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.65.0-nightly (6924792be 2022-09-15) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type dylib --crate-type rlib -C prefer-dynamic -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
