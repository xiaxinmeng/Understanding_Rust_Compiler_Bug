plain
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.73
   Compiling unwind v0.0.0 (/checkout/library/unwind)
   Compiling rustc-std-workspace-core v1.99.0 (/checkout/library/rustc-std-workspace-core)
error: internal compiler error: /checkout/compiler/rustc_codegen_ssa/src/mir/operand.rs:116:18: not immediate: OperandRef(Pair(([0 x i8]*:[0 x i8]* %2), (i64:i64 %3)) @ TyAndLayout { ty: *mut [mem::maybe_uninit::MaybeUninit<u8>], layout: Layout { fields: Arbitrary { offsets: [Size(0 bytes), Size(8 bytes)], memory_index: [0, 1] }, variants: Single { index: 0 }, abi: ScalarPair(Initialized { value: Pointer, valid_range: 0..=18446744073709551615 }, Initialized { value: Int(I64, false), valid_range: 0..=18446744073709551615 }), largest_niche: None, align: AbiAndPrefAlign { abi: Align(8 bytes), pref: Align(8 bytes) }, size: Size(16 bytes) } })
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1362:9
stack backtrace:
   0:     0x7fa4cd64de12 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h08649ce12940e8c1
   0:     0x7fa4cd64de12 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h08649ce12940e8c1
   1:     0x7fa4cd6b5b38 - core::fmt::write::ha01458c252ca8e28
   2:     0x7fa4cd63e171 - std::io::Write::write_fmt::h4fb7f0f47561e7a9
   3:     0x7fa4cd651149 - std::panicking::default_hook::{{closure}}::h61addd9ad436ef38
   4:     0x7fa4cd650dea - std::panicking::default_hook::h46350f1a9fa39981
   5:     0x7fa4ce1d25a1 - rustc_driver[e104fe6c42eb06b6]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fa4cd6519af - std::panicking::rust_panic_with_hook::h70294a24cb020d21
   7:     0x7fa4d0de9b63 - std[836a811975e52724]::panicking::begin_panic::<rustc_errors[9d4038935dd1aec6]::ExplicitBug>::{closure#0}
   8:     0x7fa4d0de8dd6 - std[836a811975e52724]::sys_common::backtrace::__rust_end_short_backtrace::<std[836a811975e52724]::panicking::begin_panic<rustc_errors[9d4038935dd1aec6]::ExplicitBug>::{closure#0}, !>
   9:     0x7fa4ce1442b6 - std[836a811975e52724]::panicking::begin_panic::<rustc_errors[9d4038935dd1aec6]::ExplicitBug>
  10:     0x7fa4d0ce5b36 - std[836a811975e52724]::panic::panic_any::<rustc_errors[9d4038935dd1aec6]::ExplicitBug>
  11:     0x7fa4d0cdef66 - <rustc_errors[9d4038935dd1aec6]::HandlerInner>::bug::<&alloc[f55ce12b9f25f528]::string::String>
  12:     0x7fa4d0cde718 - <rustc_errors[9d4038935dd1aec6]::Handler>::bug::<&alloc[f55ce12b9f25f528]::string::String>
  13:     0x7fa4d0e087fe - rustc_middle[ab74dfa1154d8ddb]::ty::context::tls::with_opt::<rustc_middle[ab74dfa1154d8ddb]::util::bug::opt_span_bug_fmt<rustc_span[3917deda49487040]::span_encoding::Span>::{closure#0}, ()>
  14:     0x7fa4d0e08ad9 - rustc_middle[ab74dfa1154d8ddb]::util::bug::opt_span_bug_fmt::<rustc_span[3917deda49487040]::span_encoding::Span>
  15:     0x7fa4ce14a1a5 - rustc_middle[ab74dfa1154d8ddb]::util::bug::bug_fmt
  16:     0x7fa4ce54ee06 - <rustc_codegen_ssa[862c57e9339d54b]::mir::operand::OperandRef<&rustc_codegen_llvm[f75b547731610f0b]::llvm_::ffi::Value>>::immediate
  17:     0x7fa4ce566e88 - <rustc_codegen_ssa[862c57e9339d54b]::mir::FunctionCx<rustc_codegen_llvm[f75b547731610f0b]::builder::Builder>>::codegen_rvalue_operand
  18:     0x7fa4ce56e037 - <rustc_codegen_ssa[862c57e9339d54b]::mir::FunctionCx<rustc_codegen_llvm[f75b547731610f0b]::builder::Builder>>::codegen_block
  19:     0x7fa4ce562e36 - rustc_codegen_ssa[862c57e9339d54b]::mir::codegen_mir::<rustc_codegen_llvm[f75b547731610f0b]::builder::Builder>
  20:     0x7fa4ce49c796 - rustc_codegen_ssa[862c57e9339d54b]::base::codegen_instance::<rustc_codegen_llvm[f75b547731610f0b]::builder::Builder>
  21:     0x7fa4ce51bcd9 - <rustc_middle[ab74dfa1154d8ddb]::mir::mono::MonoItem as rustc_codegen_ssa[862c57e9339d54b]::mono_item::MonoItemExt>::define::<rustc_codegen_llvm[f75b547731610f0b]::builder::Builder>
  22:     0x7fa4ce3f35a4 - rustc_codegen_llvm[f75b547731610f0b]::base::compile_codegen_unit::module_codegen
  23:     0x7fa4ce3f1c69 - rustc_codegen_llvm[f75b547731610f0b]::base::compile_codegen_unit
  24:     0x7fa4ce4c91d9 - <rustc_codegen_llvm[f75b547731610f0b]::LlvmCodegenBackend as rustc_codegen_ssa[862c57e9339d54b]::traits::backend::CodegenBackend>::codegen_crate
  25:     0x7fa4ce3239d8 - <rustc_session[797bd57f3adb2fd1]::session::Session>::time::<alloc[f55ce12b9f25f528]::boxed::Box<dyn core[6d9550a4e960c99f]::any::Any>, rustc_interface[f326250b504794b4]::passes::start_codegen::{closure#0}>
  26:     0x7fa4ce304854 - <rustc_interface[f326250b504794b4]::passes::QueryContext>::enter::<<rustc_interface[f326250b504794b4]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[6d9550a4e960c99f]::result::Result<alloc[f55ce12b9f25f528]::boxed::Box<dyn core[6d9550a4e960c99f]::any::Any>, rustc_errors[9d4038935dd1aec6]::ErrorGuaranteed>>
  27:     0x7fa4ce3bbaa8 - <rustc_interface[f326250b504794b4]::queries::Queries>::ongoing_codegen
  28:     0x7fa4ce1f01b7 - <rustc_interface[f326250b504794b4]::interface::Compiler>::enter::<rustc_driver[e104fe6c42eb06b6]::run_compiler::{closure#1}::{closure#2}, core[6d9550a4e960c99f]::result::Result<core[6d9550a4e960c99f]::option::Option<rustc_interface[f326250b504794b4]::queries::Linker>, rustc_errors[9d4038935dd1aec6]::ErrorGuaranteed>>
  29:     0x7fa4ce25b596 - rustc_span[3917deda49487040]::with_source_map::<core[6d9550a4e960c99f]::result::Result<(), rustc_errors[9d4038935dd1aec6]::ErrorGuaranteed>, rustc_interface[f326250b504794b4]::interface::create_compiler_and_run<core[6d9550a4e960c99f]::result::Result<(), rustc_errors[9d4038935dd1aec6]::ErrorGuaranteed>, rustc_driver[e104fe6c42eb06b6]::run_compiler::{closure#1}>::{closure#1}>
  30:     0x7fa4ce1f15ed - <scoped_tls[112f8d9a5d871235]::ScopedKey<rustc_span[3917deda49487040]::SessionGlobals>>::set::<rustc_interface[f326250b504794b4]::interface::run_compiler<core[6d9550a4e960c99f]::result::Result<(), rustc_errors[9d4038935dd1aec6]::ErrorGuaranteed>, rustc_driver[e104fe6c42eb06b6]::run_compiler::{closure#1}>::{closure#0}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[9d4038935dd1aec6]::ErrorGuaranteed>>
  31:     0x7fa4ce26068f - std[836a811975e52724]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[f326250b504794b4]::util::run_in_thread_pool_with_globals<rustc_interface[f326250b504794b4]::interface::run_compiler<core[6d9550a4e960c99f]::result::Result<(), rustc_errors[9d4038935dd1aec6]::ErrorGuaranteed>, rustc_driver[e104fe6c42eb06b6]::run_compiler::{closure#1}>::{closure#0}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[9d4038935dd1aec6]::ErrorGuaranteed>>::{closure#0}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[9d4038935dd1aec6]::ErrorGuaranteed>>
  32:     0x7fa4ce24ec81 - <<std[836a811975e52724]::thread::Builder>::spawn_unchecked_<rustc_interface[f326250b504794b4]::util::run_in_thread_pool_with_globals<rustc_interface[f326250b504794b4]::interface::run_compiler<core[6d9550a4e960c99f]::result::Result<(), rustc_errors[9d4038935dd1aec6]::ErrorGuaranteed>, rustc_driver[e104fe6c42eb06b6]::run_compiler::{closure#1}>::{closure#0}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[9d4038935dd1aec6]::ErrorGuaranteed>>::{closure#0}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[9d4038935dd1aec6]::ErrorGuaranteed>>::{closure#1} as core[6d9550a4e960c99f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  33:     0x7fa4cd65e3f3 - std::sys::unix::thread::Thread::new::thread_start::h09105972e562a0e6
  34:     0x7fa4c7bae609 - start_thread
  35:     0x7fa4cd4c1133 - clone
  36:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.63.0-nightly (d1ed86801 2022-06-14) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -Z unstable-options -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
