plain
---- [ui] src/test/ui/aligned_enum_cast.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/aligned_enum_cast.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/aligned_enum_cast/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/aligned_enum_cast/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: /checkout/compiler/rustc_codegen_ssa/src/mir/operand.rs:116:18: not immediate: OperandRef(Ref((%Aligned*:  %7 = alloca %Aligned, align 8), None, Align { pow2: 3 }) @ TyAndLayout { ty: Aligned, layout: Layout { fields: Arbitrary { offsets: [Size { raw: 0 }], memory_index: [0] }, variants: Multiple { tag: Initialized { value: Int(I8, false), valid_range: 0..=1 }, tag_encoding: Direct, tag_field: 0, variants: [Layout { fields: Arbitrary { offsets: [], memory_index: [] }, variants: Single { index: 0 }, abi: Aggregate { sized: true }, largest_niche: None, align: AbiAndPrefAlign { abi: Align { pow2: 3 }, pref: Align { pow2: 3 } }, size: Size { raw: 8 } }, Layout { fields: Arbitrary { offsets: [], memory_index: [] }, variants: Single { index: 1 }, abi: Aggregate { sized: true }, largest_niche: None, align: AbiAndPrefAlign { abi: Align { pow2: 3 }, pref: Align { pow2: 3 } }, size: Size { raw: 8 } }] }, abi: Aggregate { sized: true }, largest_niche: Some(Niche { offset: Size { raw: 0 }, value: Int(I8, false), valid_range: 0..=1 }), align: AbiAndPrefAlign { abi: Align { pow2: 3 }, pref: Align { pow2: 3 } }, size: Size { raw: 8 } } })
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1324:9
stack backtrace:
   0:     0x7fc53bd6b9dc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h1247a4a12d33e5e9
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   1:     0x7fc53bdd1fe8 - core::fmt::write::ha1e5aec452c5fae7
   2:     0x7fc53bd5b841 - std::io::Write::write_fmt::hea47d0a0e46d86c1
   3:     0x7fc53bd6ea0e - std::panicking::default_hook::{{closure}}::h4dcf892fa0a185bc
   4:     0x7fc53bd6e63c - std::panicking::default_hook::h0a6580fc47ab2d50
   5:     0x7fc53c8ed591 - rustc_driver[d029148e8f103cc8]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fc53bd6f26e - std::panicking::rust_panic_with_hook::h58c93000094600a9
   7:     0x7fc53f0d3e63 - std[f384be362483fa8f]::panicking::begin_panic::<rustc_errors[4e8211b03e767340]::ExplicitBug>::{closure#0}
   8:     0x7fc53f0d2726 - std[f384be362483fa8f]::sys_common::backtrace::__rust_end_short_backtrace::<std[f384be362483fa8f]::panicking::begin_panic<rustc_errors[4e8211b03e767340]::ExplicitBug>::{closure#0}, !>
   9:     0x7fc53c7e69a6 - std[f384be362483fa8f]::panicking::begin_panic::<rustc_errors[4e8211b03e767340]::ExplicitBug>
  10:     0x7fc53f18ebe6 - std[f384be362483fa8f]::panic::panic_any::<rustc_errors[4e8211b03e767340]::ExplicitBug>
  11:     0x7fc53f188276 - <rustc_errors[4e8211b03e767340]::HandlerInner>::bug::<&alloc[1594e5c99fca178f]::string::String>
  12:     0x7fc53f187b00 - <rustc_errors[4e8211b03e767340]::Handler>::bug::<&alloc[1594e5c99fca178f]::string::String>
  13:     0x7fc53f2ad1a5 - rustc_middle[53182824d83a1056]::util::bug::opt_span_bug_fmt::<rustc_span[25fdda02bea48d0f]::span_encoding::Span>::{closure#0}
  14:     0x7fc53f2abe7b - rustc_middle[53182824d83a1056]::ty::context::tls::with_opt::<rustc_middle[53182824d83a1056]::util::bug::opt_span_bug_fmt<rustc_span[25fdda02bea48d0f]::span_encoding::Span>::{closure#0}, ()>::{closure#0}
  15:     0x7fc53f2abe2c - rustc_middle[53182824d83a1056]::ty::context::tls::with_opt::<rustc_middle[53182824d83a1056]::util::bug::opt_span_bug_fmt<rustc_span[25fdda02bea48d0f]::span_encoding::Span>::{closure#0}, ()>
  16:     0x7fc53f2ad0e9 - rustc_middle[53182824d83a1056]::util::bug::opt_span_bug_fmt::<rustc_span[25fdda02bea48d0f]::span_encoding::Span>
  17:     0x7fc53c7ed425 - rustc_middle[53182824d83a1056]::util::bug::bug_fmt
  18:     0x7fc53cbd63c6 - <rustc_codegen_ssa[365ef735bfca32d8]::mir::operand::OperandRef<&rustc_codegen_llvm[fde1d92ebfeceefa]::llvm_::ffi::Value>>::immediate
  19:     0x7fc53cbfe81b - <rustc_codegen_ssa[365ef735bfca32d8]::mir::FunctionCx<rustc_codegen_llvm[fde1d92ebfeceefa]::builder::Builder>>::codegen_rvalue_operand
  20:     0x7fc53cbfc341 - <rustc_codegen_ssa[365ef735bfca32d8]::mir::FunctionCx<rustc_codegen_llvm[fde1d92ebfeceefa]::builder::Builder>>::codegen_rvalue
  21:     0x7fc53cc05be7 - <rustc_codegen_ssa[365ef735bfca32d8]::mir::FunctionCx<rustc_codegen_llvm[fde1d92ebfeceefa]::builder::Builder>>::codegen_block
  22:     0x7fc53cbf92e6 - rustc_codegen_ssa[365ef735bfca32d8]::mir::codegen_mir::<rustc_codegen_llvm[fde1d92ebfeceefa]::builder::Builder>
  23:     0x7fc53cb2e996 - rustc_codegen_ssa[365ef735bfca32d8]::base::codegen_instance::<rustc_codegen_llvm[fde1d92ebfeceefa]::builder::Builder>
  24:     0x7fc53cbbb8f7 - <rustc_middle[53182824d83a1056]::mir::mono::MonoItem as rustc_codegen_ssa[365ef735bfca32d8]::mono_item::MonoItemExt>::define::<rustc_codegen_llvm[fde1d92ebfeceefa]::builder::Builder>
  25:     0x7fc53cb18ef4 - rustc_codegen_llvm[fde1d92ebfeceefa]::base::compile_codegen_unit::module_codegen
  26:     0x7fc53cb17352 - rustc_codegen_llvm[fde1d92ebfeceefa]::base::compile_codegen_unit
  27:     0x7fc53cb2d306 - rustc_codegen_ssa[365ef735bfca32d8]::base::codegen_crate::<rustc_codegen_llvm[fde1d92ebfeceefa]::LlvmCodegenBackend>
  28:     0x7fc53cb7a51b - <rustc_codegen_llvm[fde1d92ebfeceefa]::LlvmCodegenBackend as rustc_codegen_ssa[365ef735bfca32d8]::traits::backend::CodegenBackend>::codegen_crate
  29:     0x7fc53c9d64d1 - <rustc_session[1289992e51a44e18]::session::Session>::time::<alloc[1594e5c99fca178f]::boxed::Box<dyn core[9b5b5e7f1b31cd54]::any::Any>, rustc_interface[39cb3e0cf393987b]::passes::start_codegen::{closure#0}>
  30:     0x7fc53c9beb13 - <rustc_interface[39cb3e0cf393987b]::passes::QueryContext>::enter::<<rustc_interface[39cb3e0cf393987b]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[9b5b5e7f1b31cd54]::result::Result<alloc[1594e5c99fca178f]::boxed::Box<dyn core[9b5b5e7f1b31cd54]::any::Any>, rustc_errors[4e8211b03e767340]::ErrorGuaranteed>>
  31:     0x7fc53c9aa1de - <rustc_interface[39cb3e0cf393987b]::queries::Queries>::ongoing_codegen
  32:     0x7fc53c87c880 - <rustc_interface[39cb3e0cf393987b]::interface::Compiler>::enter::<rustc_driver[d029148e8f103cc8]::run_compiler::{closure#1}::{closure#2}, core[9b5b5e7f1b31cd54]::result::Result<core[9b5b5e7f1b31cd54]::option::Option<rustc_interface[39cb3e0cf393987b]::queries::Linker>, rustc_errors[4e8211b03e767340]::ErrorGuaranteed>>
  33:     0x7fc53c85dc5b - rustc_span[25fdda02bea48d0f]::with_source_map::<core[9b5b5e7f1b31cd54]::result::Result<(), rustc_errors[4e8211b03e767340]::ErrorGuaranteed>, rustc_interface[39cb3e0cf393987b]::interface::create_compiler_and_run<core[9b5b5e7f1b31cd54]::result::Result<(), rustc_errors[4e8211b03e767340]::ErrorGuaranteed>, rustc_driver[d029148e8f103cc8]::run_compiler::{closure#1}>::{closure#1}>
  34:     0x7fc53c87d9b9 - <scoped_tls[6705573ce8f88c90]::ScopedKey<rustc_span[25fdda02bea48d0f]::SessionGlobals>>::set::<rustc_interface[39cb3e0cf393987b]::interface::run_compiler<core[9b5b5e7f1b31cd54]::result::Result<(), rustc_errors[4e8211b03e767340]::ErrorGuaranteed>, rustc_driver[d029148e8f103cc8]::run_compiler::{closure#1}>::{closure#0}, core[9b5b5e7f1b31cd54]::result::Result<(), rustc_errors[4e8211b03e767340]::ErrorGuaranteed>>
  35:     0x7fc53c8d9369 - std[f384be362483fa8f]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[39cb3e0cf393987b]::util::run_in_thread_pool_with_globals<rustc_interface[39cb3e0cf393987b]::interface::run_compiler<core[9b5b5e7f1b31cd54]::result::Result<(), rustc_errors[4e8211b03e767340]::ErrorGuaranteed>, rustc_driver[d029148e8f103cc8]::run_compiler::{closure#1}>::{closure#0}, core[9b5b5e7f1b31cd54]::result::Result<(), rustc_errors[4e8211b03e767340]::ErrorGuaranteed>>::{closure#0}, core[9b5b5e7f1b31cd54]::result::Result<(), rustc_errors[4e8211b03e767340]::ErrorGuaranteed>>
  36:     0x7fc53c891fb1 - std[f384be362483fa8f]::panicking::try::<core[9b5b5e7f1b31cd54]::result::Result<(), rustc_errors[4e8211b03e767340]::ErrorGuaranteed>, core[9b5b5e7f1b31cd54]::panic::unwind_safe::AssertUnwindSafe<<std[f384be362483fa8f]::thread::Builder>::spawn_unchecked_<rustc_interface[39cb3e0cf393987b]::util::run_in_thread_pool_with_globals<rustc_interface[39cb3e0cf393987b]::interface::run_compiler<core[9b5b5e7f1b31cd54]::result::Result<(), rustc_errors[4e8211b03e767340]::ErrorGuaranteed>, rustc_driver[d029148e8f103cc8]::run_compiler::{closure#1}>::{closure#0}, core[9b5b5e7f1b31cd54]::result::Result<(), rustc_errors[4e8211b03e767340]::ErrorGuaranteed>>::{closure#0}, core[9b5b5e7f1b31cd54]::result::Result<(), rustc_errors[4e8211b03e767340]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  37:     0x7fc53c8d4312 - <<std[f384be362483fa8f]::thread::Builder>::spawn_unchecked_<rustc_interface[39cb3e0cf393987b]::util::run_in_thread_pool_with_globals<rustc_interface[39cb3e0cf393987b]::interface::run_compiler<core[9b5b5e7f1b31cd54]::result::Result<(), rustc_errors[4e8211b03e767340]::ErrorGuaranteed>, rustc_driver[d029148e8f103cc8]::run_compiler::{closure#1}>::{closure#0}, core[9b5b5e7f1b31cd54]::result::Result<(), rustc_errors[4e8211b03e767340]::ErrorGuaranteed>>::{closure#0}, core[9b5b5e7f1b31cd54]::result::Result<(), rustc_errors[4e8211b03e767340]::ErrorGuaranteed>>::{closure#1} as core[9b5b5e7f1b31cd54]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  38:     0x7fc53bd7a5c3 - std::sys::unix::thread::Thread::new::thread_start::h24d5b9919641643c
  39:     0x7fc5362cc609 - start_thread
  40:     0x7fc53bbdf163 - clone
  41:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.62.0-nightly (367f73b54 2022-05-07) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
error: aborting due to previous error
------------------------------------------
