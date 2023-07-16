plain
   Compiling std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
   Compiling hashbrown v0.12.0
   Compiling object v0.26.2
   Compiling addr2line v0.16.0
error: internal compiler error: /checkout/compiler/rustc_codegen_ssa/src/mir/operand.rs:120:18: not immediate: OperandRef(Pair(([0 x %"addr2line::function::FunctionAddress"]*:  %151 = bitcast i64* %150 to [0 x %"addr2line::function::FunctionAddress"]*), (i64:  %149 = load i64, i64* %148, align 8)) @ TyAndLayout { ty: *const [addr2line::function::FunctionAddress], layout: Layout { fields: Arbitrary { offsets: [Size(0 bytes), Size(8 bytes)], memory_index: [0, 1] }, variants: Single { index: 0 }, abi: ScalarPair(Initialized { value: Pointer, valid_range: 0..=18446744073709551615 }, Initialized { value: Int(I64, false), valid_range: 0..=18446744073709551615 }), largest_niche: None, align: AbiAndPrefAlign { abi: Align(8 bytes), pref: Align(8 bytes) }, size: Size(16 bytes) } })
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1391:9
stack backtrace:
stack backtrace:
   0:     0x7fb2b227de32 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hb97170048eb2feed
   1:     0x7fb2b22e5a68 - core::fmt::write::hddd81543d4c11162
   2:     0x7fb2b226e221 - std::io::Write::write_fmt::hb10c7c89d9d85e51
   3:     0x7fb2b2281149 - std::panicking::default_hook::{{closure}}::hf26c6e572bb06902
   4:     0x7fb2b2280dea - std::panicking::default_hook::h5e275631e30996f3
   5:     0x7fb2b2d8d0a4 - rustc_driver[7e8a60621a26c2bc]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fb2b22819af - std::panicking::rust_panic_with_hook::h88a63d00b2da51a4
   7:     0x7fb2b5617583 - std[cf5d045d49351115]::panicking::begin_panic::<rustc_errors[55853087fe44a12]::ExplicitBug>::{closure#0}
   8:     0x7fb2b56110a6 - std[cf5d045d49351115]::sys_common::backtrace::__rust_end_short_backtrace::<std[cf5d045d49351115]::panicking::begin_panic<rustc_errors[55853087fe44a12]::ExplicitBug>::{closure#0}, !>
   9:     0x7fb2b2d085a6 - std[cf5d045d49351115]::panicking::begin_panic::<rustc_errors[55853087fe44a12]::ExplicitBug>
  10:     0x7fb2b56ab506 - std[cf5d045d49351115]::panic::panic_any::<rustc_errors[55853087fe44a12]::ExplicitBug>
  11:     0x7fb2b56aa486 - <rustc_errors[55853087fe44a12]::HandlerInner>::bug::<&alloc[b31f836c82ae9902]::string::String>
  12:     0x7fb2b56aa150 - <rustc_errors[55853087fe44a12]::Handler>::bug::<&alloc[b31f836c82ae9902]::string::String>
  13:     0x7fb2b578d45e - rustc_middle[46526c9c0dd900b0]::ty::context::tls::with_opt::<rustc_middle[46526c9c0dd900b0]::util::bug::opt_span_bug_fmt<rustc_span[4929ef0fa1883a40]::span_encoding::Span>::{closure#0}, ()>
  14:     0x7fb2b578d7c9 - rustc_middle[46526c9c0dd900b0]::util::bug::opt_span_bug_fmt::<rustc_span[4929ef0fa1883a40]::span_encoding::Span>
  15:     0x7fb2b2d0b885 - rustc_middle[46526c9c0dd900b0]::util::bug::bug_fmt
  16:     0x7fb2b30c0946 - <rustc_codegen_ssa[e27934b884219a20]::mir::operand::OperandRef<&rustc_codegen_llvm[d66ec9cbc0a6635b]::llvm_::ffi::Value>>::immediate
  17:     0x7fb2b30cb993 - <rustc_codegen_ssa[e27934b884219a20]::mir::FunctionCx<rustc_codegen_llvm[d66ec9cbc0a6635b]::builder::Builder>>::codegen_rvalue_operand
  18:     0x7fb2b30d2784 - <rustc_codegen_ssa[e27934b884219a20]::mir::FunctionCx<rustc_codegen_llvm[d66ec9cbc0a6635b]::builder::Builder>>::codegen_block
  19:     0x7fb2b30c7266 - rustc_codegen_ssa[e27934b884219a20]::mir::codegen_mir::<rustc_codegen_llvm[d66ec9cbc0a6635b]::builder::Builder>
  20:     0x7fb2b2f663a6 - rustc_codegen_ssa[e27934b884219a20]::base::codegen_instance::<rustc_codegen_llvm[d66ec9cbc0a6635b]::builder::Builder>
  21:     0x7fb2b3099636 - <rustc_middle[46526c9c0dd900b0]::mir::mono::MonoItem as rustc_codegen_ssa[e27934b884219a20]::mono_item::MonoItemExt>::define::<rustc_codegen_llvm[d66ec9cbc0a6635b]::builder::Builder>
  22:     0x7fb2b3005444 - rustc_codegen_llvm[d66ec9cbc0a6635b]::base::compile_codegen_unit::module_codegen
  23:     0x7fb2b3003ad4 - rustc_codegen_llvm[d66ec9cbc0a6635b]::base::compile_codegen_unit
  24:     0x7fb2b2f655cc - rustc_codegen_ssa[e27934b884219a20]::base::codegen_crate::<rustc_codegen_llvm[d66ec9cbc0a6635b]::LlvmCodegenBackend>
  25:     0x7fb2b302eb39 - <rustc_codegen_llvm[d66ec9cbc0a6635b]::LlvmCodegenBackend as rustc_codegen_ssa[e27934b884219a20]::traits::backend::CodegenBackend>::codegen_crate
  26:     0x7fb2b2ec7218 - <rustc_session[944f0a6e620a371d]::session::Session>::time::<alloc[b31f836c82ae9902]::boxed::Box<dyn core[990e9aac2c65c03a]::any::Any>, rustc_interface[5011239097c2d7af]::passes::start_codegen::{closure#0}>
  27:     0x7fb2b2eb12ec - <rustc_interface[5011239097c2d7af]::passes::QueryContext>::enter::<<rustc_interface[5011239097c2d7af]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[990e9aac2c65c03a]::result::Result<alloc[b31f836c82ae9902]::boxed::Box<dyn core[990e9aac2c65c03a]::any::Any>, rustc_errors[55853087fe44a12]::ErrorGuaranteed>>
  28:     0x7fb2b2ea4b5e - <rustc_interface[5011239097c2d7af]::queries::Queries>::ongoing_codegen
  29:     0x7fb2b2da1f9f - <rustc_interface[5011239097c2d7af]::interface::Compiler>::enter::<rustc_driver[7e8a60621a26c2bc]::run_compiler::{closure#1}::{closure#2}, core[990e9aac2c65c03a]::result::Result<core[990e9aac2c65c03a]::option::Option<rustc_interface[5011239097c2d7af]::queries::Linker>, rustc_errors[55853087fe44a12]::ErrorGuaranteed>>
  30:     0x7fb2b2dfee79 - rustc_span[4929ef0fa1883a40]::with_source_map::<core[990e9aac2c65c03a]::result::Result<(), rustc_errors[55853087fe44a12]::ErrorGuaranteed>, rustc_interface[5011239097c2d7af]::interface::create_compiler_and_run<core[990e9aac2c65c03a]::result::Result<(), rustc_errors[55853087fe44a12]::ErrorGuaranteed>, rustc_driver[7e8a60621a26c2bc]::run_compiler::{closure#1}>::{closure#1}>
  31:     0x7fb2b2da31be - <scoped_tls[56ec63818e29d8c5]::ScopedKey<rustc_span[4929ef0fa1883a40]::SessionGlobals>>::set::<rustc_interface[5011239097c2d7af]::interface::run_compiler<core[990e9aac2c65c03a]::result::Result<(), rustc_errors[55853087fe44a12]::ErrorGuaranteed>, rustc_driver[7e8a60621a26c2bc]::run_compiler::{closure#1}>::{closure#0}, core[990e9aac2c65c03a]::result::Result<(), rustc_errors[55853087fe44a12]::ErrorGuaranteed>>
  32:     0x7fb2b2dfba59 - std[cf5d045d49351115]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[5011239097c2d7af]::util::run_in_thread_pool_with_globals<rustc_interface[5011239097c2d7af]::interface::run_compiler<core[990e9aac2c65c03a]::result::Result<(), rustc_errors[55853087fe44a12]::ErrorGuaranteed>, rustc_driver[7e8a60621a26c2bc]::run_compiler::{closure#1}>::{closure#0}, core[990e9aac2c65c03a]::result::Result<(), rustc_errors[55853087fe44a12]::ErrorGuaranteed>>::{closure#0}, core[990e9aac2c65c03a]::result::Result<(), rustc_errors[55853087fe44a12]::ErrorGuaranteed>>
  33:     0x7fb2b2df3789 - <<std[cf5d045d49351115]::thread::Builder>::spawn_unchecked_<rustc_interface[5011239097c2d7af]::util::run_in_thread_pool_with_globals<rustc_interface[5011239097c2d7af]::interface::run_compiler<core[990e9aac2c65c03a]::result::Result<(), rustc_errors[55853087fe44a12]::ErrorGuaranteed>, rustc_driver[7e8a60621a26c2bc]::run_compiler::{closure#1}>::{closure#0}, core[990e9aac2c65c03a]::result::Result<(), rustc_errors[55853087fe44a12]::ErrorGuaranteed>>::{closure#0}, core[990e9aac2c65c03a]::result::Result<(), rustc_errors[55853087fe44a12]::ErrorGuaranteed>>::{closure#1} as core[990e9aac2c65c03a]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  34:     0x7fb2b228e373 - std::sys::unix::thread::Thread::new::thread_start::hfdb91e9761b815b8
  35:     0x7fb2ac7dd609 - start_thread
  36:     0x7fb2b20f0133 - clone
  37:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.64.0-nightly (2f8088d97 2022-07-09) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type dylib --crate-type rlib -C prefer-dynamic -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -Z unstable-options -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
