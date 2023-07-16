plain
   Compiling object v0.26.2
   Compiling hashbrown v0.12.0
   Compiling miniz_oxide v0.4.0
   Compiling addr2line v0.16.0
error: internal compiler error: /checkout/compiler/rustc_codegen_ssa/src/mir/operand.rs:116:18: not immediate: OperandRef(Pair(([0 x %"addr2line::function::FunctionAddress"]*:  %151 = bitcast i64* %150 to [0 x %"addr2line::function::FunctionAddress"]*), (i64:  %149 = load i64, i64* %148, align 8)) @ TyAndLayout { ty: *const [addr2line::function::FunctionAddress], layout: Layout { fields: Arbitrary { offsets: [Size(0 bytes), Size(8 bytes)], memory_index: [0, 1] }, variants: Single { index: 0 }, abi: ScalarPair(Initialized { value: Pointer, valid_range: 0..=18446744073709551615 }, Initialized { value: Int(I64, false), valid_range: 0..=18446744073709551615 }), largest_niche: None, align: AbiAndPrefAlign { abi: Align(8 bytes), pref: Align(8 bytes) }, size: Size(16 bytes) } })
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1391:9
stack backtrace:
stack backtrace:
   0:     0x7fd69777de22 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h08649ce12940e8c1
   1:     0x7fd6977e5aa8 - core::fmt::write::ha01458c252ca8e28
   2:     0x7fd69776e0f1 - std::io::Write::write_fmt::h4fb7f0f47561e7a9
   3:     0x7fd697781139 - std::panicking::default_hook::{{closure}}::h61addd9ad436ef38
   4:     0x7fd697780dda - std::panicking::default_hook::h46350f1a9fa39981
   5:     0x7fd69827e3d4 - rustc_driver[48e10e3c4f250537]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fd69778199f - std::panicking::rust_panic_with_hook::h70294a24cb020d21
   7:     0x7fd69ac2b4a3 - std[836a811975e52724]::panicking::begin_panic::<rustc_errors[7c07a63214684621]::ExplicitBug>::{closure#0}
   8:     0x7fd69ac2a3b6 - std[836a811975e52724]::sys_common::backtrace::__rust_end_short_backtrace::<std[836a811975e52724]::panicking::begin_panic<rustc_errors[7c07a63214684621]::ExplicitBug>::{closure#0}, !>
   9:     0x7fd698207be6 - std[836a811975e52724]::panicking::begin_panic::<rustc_errors[7c07a63214684621]::ExplicitBug>
  10:     0x7fd69aa3abb6 - std[836a811975e52724]::panic::panic_any::<rustc_errors[7c07a63214684621]::ExplicitBug>
  11:     0x7fd69aa2dab6 - <rustc_errors[7c07a63214684621]::HandlerInner>::bug::<&alloc[f55ce12b9f25f528]::string::String>
  12:     0x7fd69aa2d780 - <rustc_errors[7c07a63214684621]::Handler>::bug::<&alloc[f55ce12b9f25f528]::string::String>
  13:     0x7fd69ac2134e - rustc_middle[c9f99ab0490ddc12]::ty::context::tls::with_opt::<rustc_middle[c9f99ab0490ddc12]::util::bug::opt_span_bug_fmt<rustc_span[d6a89a0bfce8bb5]::span_encoding::Span>::{closure#0}, ()>
  14:     0x7fd69ac21a79 - rustc_middle[c9f99ab0490ddc12]::util::bug::opt_span_bug_fmt::<rustc_span[d6a89a0bfce8bb5]::span_encoding::Span>
  15:     0x7fd6982093f5 - rustc_middle[c9f99ab0490ddc12]::util::bug::bug_fmt
  16:     0x7fd6985db126 - <rustc_codegen_ssa[aca223f6fa342053]::mir::operand::OperandRef<&rustc_codegen_llvm[5193eac684f0f511]::llvm_::ffi::Value>>::immediate
  17:     0x7fd6985b55bd - <rustc_codegen_ssa[aca223f6fa342053]::mir::FunctionCx<rustc_codegen_llvm[5193eac684f0f511]::builder::Builder>>::codegen_rvalue_operand
  18:     0x7fd6985bc128 - <rustc_codegen_ssa[aca223f6fa342053]::mir::FunctionCx<rustc_codegen_llvm[5193eac684f0f511]::builder::Builder>>::codegen_block
  19:     0x7fd6985b14f6 - rustc_codegen_ssa[aca223f6fa342053]::mir::codegen_mir::<rustc_codegen_llvm[5193eac684f0f511]::builder::Builder>
  20:     0x7fd6984fef86 - rustc_codegen_ssa[aca223f6fa342053]::base::codegen_instance::<rustc_codegen_llvm[5193eac684f0f511]::builder::Builder>
  21:     0x7fd698580da6 - <rustc_middle[c9f99ab0490ddc12]::mir::mono::MonoItem as rustc_codegen_ssa[aca223f6fa342053]::mono_item::MonoItemExt>::define::<rustc_codegen_llvm[5193eac684f0f511]::builder::Builder>
  22:     0x7fd6984f0594 - rustc_codegen_llvm[5193eac684f0f511]::base::compile_codegen_unit::module_codegen
  23:     0x7fd6984eec2c - rustc_codegen_llvm[5193eac684f0f511]::base::compile_codegen_unit
  24:     0x7fd6984fe142 - rustc_codegen_ssa[aca223f6fa342053]::base::codegen_crate::<rustc_codegen_llvm[5193eac684f0f511]::LlvmCodegenBackend>
  25:     0x7fd698473a69 - <rustc_codegen_llvm[5193eac684f0f511]::LlvmCodegenBackend as rustc_codegen_ssa[aca223f6fa342053]::traits::backend::CodegenBackend>::codegen_crate
  26:     0x7fd6983c7e68 - <rustc_session[6823c539bda58cf9]::session::Session>::time::<alloc[f55ce12b9f25f528]::boxed::Box<dyn core[6d9550a4e960c99f]::any::Any>, rustc_interface[2ec13e7f8be7b8f4]::passes::start_codegen::{closure#0}>
  27:     0x7fd6983b2a8c - <rustc_interface[2ec13e7f8be7b8f4]::passes::QueryContext>::enter::<<rustc_interface[2ec13e7f8be7b8f4]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[6d9550a4e960c99f]::result::Result<alloc[f55ce12b9f25f528]::boxed::Box<dyn core[6d9550a4e960c99f]::any::Any>, rustc_errors[7c07a63214684621]::ErrorGuaranteed>>
  28:     0x7fd69839d86e - <rustc_interface[2ec13e7f8be7b8f4]::queries::Queries>::ongoing_codegen
  29:     0x7fd69829cb3f - <rustc_interface[2ec13e7f8be7b8f4]::interface::Compiler>::enter::<rustc_driver[48e10e3c4f250537]::run_compiler::{closure#1}::{closure#2}, core[6d9550a4e960c99f]::result::Result<core[6d9550a4e960c99f]::option::Option<rustc_interface[2ec13e7f8be7b8f4]::queries::Linker>, rustc_errors[7c07a63214684621]::ErrorGuaranteed>>
  30:     0x7fd69827f926 - rustc_span[d6a89a0bfce8bb5]::with_source_map::<core[6d9550a4e960c99f]::result::Result<(), rustc_errors[7c07a63214684621]::ErrorGuaranteed>, rustc_interface[2ec13e7f8be7b8f4]::interface::create_compiler_and_run<core[6d9550a4e960c99f]::result::Result<(), rustc_errors[7c07a63214684621]::ErrorGuaranteed>, rustc_driver[48e10e3c4f250537]::run_compiler::{closure#1}>::{closure#1}>
  31:     0x7fd69829dd5e - <scoped_tls[112f8d9a5d871235]::ScopedKey<rustc_span[d6a89a0bfce8bb5]::SessionGlobals>>::set::<rustc_interface[2ec13e7f8be7b8f4]::interface::run_compiler<core[6d9550a4e960c99f]::result::Result<(), rustc_errors[7c07a63214684621]::ErrorGuaranteed>, rustc_driver[48e10e3c4f250537]::run_compiler::{closure#1}>::{closure#0}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[7c07a63214684621]::ErrorGuaranteed>>
  32:     0x7fd6982f5419 - std[836a811975e52724]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[2ec13e7f8be7b8f4]::util::run_in_thread_pool_with_globals<rustc_interface[2ec13e7f8be7b8f4]::interface::run_compiler<core[6d9550a4e960c99f]::result::Result<(), rustc_errors[7c07a63214684621]::ErrorGuaranteed>, rustc_driver[48e10e3c4f250537]::run_compiler::{closure#1}>::{closure#0}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[7c07a63214684621]::ErrorGuaranteed>>::{closure#0}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[7c07a63214684621]::ErrorGuaranteed>>
  33:     0x7fd6982ed259 - <<std[836a811975e52724]::thread::Builder>::spawn_unchecked_<rustc_interface[2ec13e7f8be7b8f4]::util::run_in_thread_pool_with_globals<rustc_interface[2ec13e7f8be7b8f4]::interface::run_compiler<core[6d9550a4e960c99f]::result::Result<(), rustc_errors[7c07a63214684621]::ErrorGuaranteed>, rustc_driver[48e10e3c4f250537]::run_compiler::{closure#1}>::{closure#0}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[7c07a63214684621]::ErrorGuaranteed>>::{closure#0}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[7c07a63214684621]::ErrorGuaranteed>>::{closure#1} as core[6d9550a4e960c99f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  34:     0x7fd69778e363 - std::sys::unix::thread::Thread::new::thread_start::h09105972e562a0e6
  35:     0x7fd691cde609 - start_thread
  36:     0x7fd6975f1133 - clone
  37:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.64.0-nightly (0a45b7eb8 2022-07-03) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type dylib --crate-type rlib -C prefer-dynamic -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -Z unstable-options -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
