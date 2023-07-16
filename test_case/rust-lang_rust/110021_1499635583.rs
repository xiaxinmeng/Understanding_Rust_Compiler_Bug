plain
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 244 tests
.............................................................................i..........  88/244
F.....................................................i...........ii.i.................. 176/244
........................ii...F...i..................................
failures:

---- [mir-opt] tests/mir-opt/const_prop/transmute.rs stdout ----


error: compilation failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/mir-opt/const_prop/transmute.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "-Copt-level=1" "-Zdump-mir=all" "-Zvalidate-mir" "-Zdump-mir-exclude-pass-number" "-Zmir-pretty-relative-line-numbers=yes" "-Zmir-opt-level=0" "-Zmir-enable-passes=+ConstProp" "-Zdump-mir-dir=/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/const_prop/transmute" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/const_prop/transmute" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/const_prop/transmute/auxiliary" "-O" "--crate-type=lib"
stdout: none
--- stderr -------------------------------
  --> /checkout/tests/mir-opt/const_prop/transmute.rs:40:11
   |
   |
39 |     let x: Never = unsafe { transmute(()) };
   |                             ------------- any code following this expression is unreachable
40 |     match x {}
   |           ^ unreachable expression
   |
note: this expression has type `Never`, which is uninhabited
   |
   |
39 |     let x: Never = unsafe { transmute(()) };
   = note: `#[warn(unreachable_code)]` on by default

warning: unused variable: `x`
  --> /checkout/tests/mir-opt/const_prop/transmute.rs:39:9
  --> /checkout/tests/mir-opt/const_prop/transmute.rs:39:9
   |
39 |     let x: Never = unsafe { transmute(()) };
   |         ^ help: if this is intentional, prefix it with an underscore: `_x`
   = note: `#[warn(unused_variables)]` on by default

warning: field `value` is never read
  --> /checkout/tests/mir-opt/const_prop/transmute.rs:33:21
---
   |           field in this union
   |
   = note: `#[warn(dead_code)]` on by default

warning: the type `Never` does not permit zero-initialization
   |
   |
39 |     let x: Never = unsafe { transmute(()) };
   |
   |
note: enums with no inhabited variants have no valid value
   |
   |
61 | enum Never {}
   = note: `#[warn(invalid_value)]` on by default


error: internal compiler error: /checkout/compiler/rustc_codegen_ssa/src/mir/rvalue.rs:897:22: Couldn't translate Aggregate { sized: true } as backend immediate
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1644:9
stack backtrace:
   0:     0x7ff91dd24385 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha3e2b28e4230e23f
   1:     0x7ff91dd92a68 - core::fmt::write::h8267527877742874
   1:     0x7ff91dd92a68 - core::fmt::write::h8267527877742874
   2:     0x7ff91dd185d1 - std::io::Write::write_fmt::hb2390c9174fb4597
   3:     0x7ff91dd24191 - std::sys_common::backtrace::print::hf9009315deed2eb8
   4:     0x7ff91dd27654 - std::panicking::default_hook::{{closure}}::hdca515b285e6c91f
   5:     0x7ff91dd27320 - std::panicking::default_hook::h82397dc786ff0a72
   6:     0x7ff91e8135f5 - rustc_driver_impl[742362009a70431c]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7ff91dd27db1 - std::panicking::rust_panic_with_hook::h1fb3af3dc5983af7
   8:     0x7ff9215acb13 - std[e22443783e85f7c8]::panicking::begin_panic::<rustc_errors[a382a3d855238c0]::ExplicitBug>::{closure#0}
   9:     0x7ff9215a59a6 - std[e22443783e85f7c8]::sys_common::backtrace::__rust_end_short_backtrace::<std[e22443783e85f7c8]::panicking::begin_panic<rustc_errors[a382a3d855238c0]::ExplicitBug>::{closure#0}, !>
  10:     0x7ff91e7b83d6 - std[e22443783e85f7c8]::panicking::begin_panic::<rustc_errors[a382a3d855238c0]::ExplicitBug>
  11:     0x7ff92161d940 - <rustc_errors[a382a3d855238c0]::HandlerInner>::bug::<&alloc[3a3a71201775eca6]::string::String>
  12:     0x7ff92161d5d0 - <rustc_errors[a382a3d855238c0]::Handler>::bug::<&alloc[3a3a71201775eca6]::string::String>
  13:     0x7ff9215e5cfc - rustc_middle[52e642eb40420eda]::util::bug::opt_span_bug_fmt::<rustc_span[1c8ddf94e18f49d9]::span_encoding::Span>::{closure#0}
  14:     0x7ff9215dd01c - rustc_middle[52e642eb40420eda]::ty::context::tls::with_opt::<rustc_middle[52e642eb40420eda]::util::bug::opt_span_bug_fmt<rustc_span[1c8ddf94e18f49d9]::span_encoding::Span>::{closure#0}, !>::{closure#0}
  15:     0x7ff9215dcfaa - rustc_middle[52e642eb40420eda]::ty::context::tls::with_context_opt::<rustc_middle[52e642eb40420eda]::ty::context::tls::with_opt<rustc_middle[52e642eb40420eda]::util::bug::opt_span_bug_fmt<rustc_span[1c8ddf94e18f49d9]::span_encoding::Span>::{closure#0}, !>::{closure#0}, !>
  16:     0x7ff9215e5c39 - rustc_middle[52e642eb40420eda]::util::bug::opt_span_bug_fmt::<rustc_span[1c8ddf94e18f49d9]::span_encoding::Span>
  17:     0x7ff91e7b2bc5 - rustc_middle[52e642eb40420eda]::util::bug::bug_fmt
  18:     0x7ff91eb469a9 - <rustc_codegen_ssa[958d09c1bf53546e]::mir::FunctionCx<rustc_codegen_llvm[ecca59f57bcd7213]::builder::Builder>>::value_kind
  19:     0x7ff91eb46ac5 - <rustc_codegen_ssa[958d09c1bf53546e]::mir::FunctionCx<rustc_codegen_llvm[ecca59f57bcd7213]::builder::Builder>>::rvalue_creates_operand
  20:     0x7ff91ea67792 - rustc_codegen_ssa[958d09c1bf53546e]::mir::analyze::non_ssa_locals::<rustc_codegen_llvm[ecca59f57bcd7213]::builder::Builder>
  21:     0x7ff91eb2e0f1 - rustc_codegen_ssa[958d09c1bf53546e]::mir::codegen_mir::<rustc_codegen_llvm[ecca59f57bcd7213]::builder::Builder>
  22:     0x7ff91eaa7887 - rustc_codegen_ssa[958d09c1bf53546e]::base::codegen_instance::<rustc_codegen_llvm[ecca59f57bcd7213]::builder::Builder>
  23:     0x7ff91eaef715 - <rustc_middle[52e642eb40420eda]::mir::mono::MonoItem as rustc_codegen_ssa[958d09c1bf53546e]::mono_item::MonoItemExt>::define::<rustc_codegen_llvm[ecca59f57bcd7213]::builder::Builder>
  24:     0x7ff91e9decb5 - rustc_codegen_llvm[ecca59f57bcd7213]::base::compile_codegen_unit::module_codegen
  25:     0x7ff91e9e2713 - <rustc_codegen_llvm[ecca59f57bcd7213]::LlvmCodegenBackend as rustc_codegen_ssa[958d09c1bf53546e]::traits::backend::ExtraBackendMethods>::compile_codegen_unit
  26:     0x7ff91eaa6c9d - rustc_codegen_ssa[958d09c1bf53546e]::base::codegen_crate::<rustc_codegen_llvm[ecca59f57bcd7213]::LlvmCodegenBackend>
  27:     0x7ff91e9e75d4 - <rustc_codegen_llvm[ecca59f57bcd7213]::LlvmCodegenBackend as rustc_codegen_ssa[958d09c1bf53546e]::traits::backend::CodegenBackend>::codegen_crate
  28:     0x7ff91e95b87f - <rustc_session[6fe6800af82aa7e3]::session::Session>::time::<alloc[3a3a71201775eca6]::boxed::Box<dyn core[f365564bdbb19c62]::any::Any>, rustc_interface[f7d8d044a3e34b97]::passes::start_codegen::{closure#0}>
  29:     0x7ff91e8d4e78 - rustc_interface[f7d8d044a3e34b97]::passes::start_codegen
  30:     0x7ff91e97bf3f - <std[e22443783e85f7c8]::thread::local::LocalKey<core[f365564bdbb19c62]::cell::Cell<*const ()>>>::with::<rustc_middle[52e642eb40420eda]::ty::context::tls::enter_context<<rustc_middle[52e642eb40420eda]::ty::context::GlobalCtxt>::enter<<rustc_interface[f7d8d044a3e34b97]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[f365564bdbb19c62]::result::Result<alloc[3a3a71201775eca6]::boxed::Box<dyn core[f365564bdbb19c62]::any::Any>, rustc_span[1c8ddf94e18f49d9]::ErrorGuaranteed>>::{closure#0}, core[f365564bdbb19c62]::result::Result<alloc[3a3a71201775eca6]::boxed::Box<dyn core[f365564bdbb19c62]::any::Any>, rustc_span[1c8ddf94e18f49d9]::ErrorGuaranteed>>::{closure#0}, core[f365564bdbb19c62]::result::Result<alloc[3a3a71201775eca6]::boxed::Box<dyn core[f365564bdbb19c62]::any::Any>, rustc_span[1c8ddf94e18f49d9]::ErrorGuaranteed>>
  31:     0x7ff91e8d7aad - <rustc_middle[52e642eb40420eda]::ty::context::GlobalCtxt>::enter::<<rustc_interface[f7d8d044a3e34b97]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[f365564bdbb19c62]::result::Result<alloc[3a3a71201775eca6]::boxed::Box<dyn core[f365564bdbb19c62]::any::Any>, rustc_span[1c8ddf94e18f49d9]::ErrorGuaranteed>>
  32:     0x7ff91e9aa594 - <rustc_interface[f7d8d044a3e34b97]::queries::Queries>::ongoing_codegen
  33:     0x7ff91e852802 - <rustc_interface[f7d8d044a3e34b97]::interface::Compiler>::enter::<rustc_driver_impl[742362009a70431c]::run_compiler::{closure#1}::{closure#2}, core[f365564bdbb19c62]::result::Result<core[f365564bdbb19c62]::option::Option<rustc_interface[f7d8d044a3e34b97]::queries::Linker>, rustc_span[1c8ddf94e18f49d9]::ErrorGuaranteed>>
  34:     0x7ff91e82068e - std[e22443783e85f7c8]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[f7d8d044a3e34b97]::util::run_in_thread_pool_with_globals<rustc_interface[f7d8d044a3e34b97]::interface::run_compiler<core[f365564bdbb19c62]::result::Result<(), rustc_span[1c8ddf94e18f49d9]::ErrorGuaranteed>, rustc_driver_impl[742362009a70431c]::run_compiler::{closure#1}>::{closure#0}, core[f365564bdbb19c62]::result::Result<(), rustc_span[1c8ddf94e18f49d9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[f365564bdbb19c62]::result::Result<(), rustc_span[1c8ddf94e18f49d9]::ErrorGuaranteed>>
  35:     0x7ff91e854568 - std[e22443783e85f7c8]::panicking::try::<core[f365564bdbb19c62]::result::Result<(), rustc_span[1c8ddf94e18f49d9]::ErrorGuaranteed>, core[f365564bdbb19c62]::panic::unwind_safe::AssertUnwindSafe<<std[e22443783e85f7c8]::thread::Builder>::spawn_unchecked_<rustc_interface[f7d8d044a3e34b97]::util::run_in_thread_pool_with_globals<rustc_interface[f7d8d044a3e34b97]::interface::run_compiler<core[f365564bdbb19c62]::result::Result<(), rustc_span[1c8ddf94e18f49d9]::ErrorGuaranteed>, rustc_driver_impl[742362009a70431c]::run_compiler::{closure#1}>::{closure#0}, core[f365564bdbb19c62]::result::Result<(), rustc_span[1c8ddf94e18f49d9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[f365564bdbb19c62]::result::Result<(), rustc_span[1c8ddf94e18f49d9]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  36:     0x7ff91e822a17 - <<std[e22443783e85f7c8]::thread::Builder>::spawn_unchecked_<rustc_interface[f7d8d044a3e34b97]::util::run_in_thread_pool_with_globals<rustc_interface[f7d8d044a3e34b97]::interface::run_compiler<core[f365564bdbb19c62]::result::Result<(), rustc_span[1c8ddf94e18f49d9]::ErrorGuaranteed>, rustc_driver_impl[742362009a70431c]::run_compiler::{closure#1}>::{closure#0}, core[f365564bdbb19c62]::result::Result<(), rustc_span[1c8ddf94e18f49d9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[f365564bdbb19c62]::result::Result<(), rustc_span[1c8ddf94e18f49d9]::ErrorGuaranteed>>::{closure#1} as core[f365564bdbb19c62]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  37:     0x7ff91dd3465e - std::sys::unix::thread::Thread::new::thread_start::hcced726475fdfa01
  38:     0x7ff91dacbb43 - <unknown>
  39:     0x7ff91db5da00 - <unknown>
  40:                0x0 - <unknown>
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.70.0-nightly (2c60df649 2023-04-06) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -C opt-level=1 -Z dump-mir=all -Z validate-mir -Z dump-mir-exclude-pass-number -Z mir-pretty-relative-line-numbers=yes -Z mir-opt-level=0 -Z mir-enable-passes=+ConstProp -Z dump-mir-dir=/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/const_prop/transmute -C prefer-dynamic -C rpath -C debuginfo=0 --crate-type lib
query stack during panic:
end of query stack
error: aborting due to previous error; 4 warnings emitted
------------------------------------------
------------------------------------------


---- [mir-opt] tests/mir-opt/lower_intrinsics.rs stdout ----

error: compilation failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/mir-opt/lower_intrinsics.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "-Copt-level=1" "-Zdump-mir=all" "-Zvalidate-mir" "-Zdump-mir-exclude-pass-number" "-Zmir-pretty-relative-line-numbers=yes" "-Zmir-opt-level=0" "-Zmir-enable-passes=+LowerIntrinsics" "-Zdump-mir-dir=/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/lower_intrinsics" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/lower_intrinsics" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/lower_intrinsics/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: /checkout/compiler/rustc_codegen_ssa/src/mir/rvalue.rs:897:22: Couldn't translate Aggregate { sized: true } as backend immediate
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1644:9
stack backtrace:
   0:     0x7f8371638385 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha3e2b28e4230e23f
   1:     0x7f83716a6a68 - core::fmt::write::h8267527877742874
   1:     0x7f83716a6a68 - core::fmt::write::h8267527877742874
   2:     0x7f837162c5d1 - std::io::Write::write_fmt::hb2390c9174fb4597
   3:     0x7f8371638191 - std::sys_common::backtrace::print::hf9009315deed2eb8
   4:     0x7f837163b654 - std::panicking::default_hook::{{closure}}::hdca515b285e6c91f
   5:     0x7f837163b320 - std::panicking::default_hook::h82397dc786ff0a72
   6:     0x7f83721275f5 - rustc_driver_impl[742362009a70431c]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f837163bdb1 - std::panicking::rust_panic_with_hook::h1fb3af3dc5983af7
   8:     0x7f8374ec0b13 - std[e22443783e85f7c8]::panicking::begin_panic::<rustc_errors[a382a3d855238c0]::ExplicitBug>::{closure#0}
   9:     0x7f8374eb99a6 - std[e22443783e85f7c8]::sys_common::backtrace::__rust_end_short_backtrace::<std[e22443783e85f7c8]::panicking::begin_panic<rustc_errors[a382a3d855238c0]::ExplicitBug>::{closure#0}, !>
  10:     0x7f83720cc3d6 - std[e22443783e85f7c8]::panicking::begin_panic::<rustc_errors[a382a3d855238c0]::ExplicitBug>
  11:     0x7f8374f31940 - <rustc_errors[a382a3d855238c0]::HandlerInner>::bug::<&alloc[3a3a71201775eca6]::string::String>
  12:     0x7f8374f315d0 - <rustc_errors[a382a3d855238c0]::Handler>::bug::<&alloc[3a3a71201775eca6]::string::String>
  13:     0x7f8374ef9cfc - rustc_middle[52e642eb40420eda]::util::bug::opt_span_bug_fmt::<rustc_span[1c8ddf94e18f49d9]::span_encoding::Span>::{closure#0}
  14:     0x7f8374ef101c - rustc_middle[52e642eb40420eda]::ty::context::tls::with_opt::<rustc_middle[52e642eb40420eda]::util::bug::opt_span_bug_fmt<rustc_span[1c8ddf94e18f49d9]::span_encoding::Span>::{closure#0}, !>::{closure#0}
  15:     0x7f8374ef0faa - rustc_middle[52e642eb40420eda]::ty::context::tls::with_context_opt::<rustc_middle[52e642eb40420eda]::ty::context::tls::with_opt<rustc_middle[52e642eb40420eda]::util::bug::opt_span_bug_fmt<rustc_span[1c8ddf94e18f49d9]::span_encoding::Span>::{closure#0}, !>::{closure#0}, !>
  16:     0x7f8374ef9c39 - rustc_middle[52e642eb40420eda]::util::bug::opt_span_bug_fmt::<rustc_span[1c8ddf94e18f49d9]::span_encoding::Span>
  17:     0x7f83720c6bc5 - rustc_middle[52e642eb40420eda]::util::bug::bug_fmt
  18:     0x7f837245a9a9 - <rustc_codegen_ssa[958d09c1bf53546e]::mir::FunctionCx<rustc_codegen_llvm[ecca59f57bcd7213]::builder::Builder>>::value_kind
  19:     0x7f837245aac5 - <rustc_codegen_ssa[958d09c1bf53546e]::mir::FunctionCx<rustc_codegen_llvm[ecca59f57bcd7213]::builder::Builder>>::rvalue_creates_operand
  20:     0x7f837237b792 - rustc_codegen_ssa[958d09c1bf53546e]::mir::analyze::non_ssa_locals::<rustc_codegen_llvm[ecca59f57bcd7213]::builder::Builder>
  21:     0x7f83724420f1 - rustc_codegen_ssa[958d09c1bf53546e]::mir::codegen_mir::<rustc_codegen_llvm[ecca59f57bcd7213]::builder::Builder>
  22:     0x7f83723bb887 - rustc_codegen_ssa[958d09c1bf53546e]::base::codegen_instance::<rustc_codegen_llvm[ecca59f57bcd7213]::builder::Builder>
  23:     0x7f8372403715 - <rustc_middle[52e642eb40420eda]::mir::mono::MonoItem as rustc_codegen_ssa[958d09c1bf53546e]::mono_item::MonoItemExt>::define::<rustc_codegen_llvm[ecca59f57bcd7213]::builder::Builder>
  24:     0x7f83722f2cb5 - rustc_codegen_llvm[ecca59f57bcd7213]::base::compile_codegen_unit::module_codegen
  25:     0x7f83722f6713 - <rustc_codegen_llvm[ecca59f57bcd7213]::LlvmCodegenBackend as rustc_codegen_ssa[958d09c1bf53546e]::traits::backend::ExtraBackendMethods>::compile_codegen_unit
  26:     0x7f83723bac9d - rustc_codegen_ssa[958d09c1bf53546e]::base::codegen_crate::<rustc_codegen_llvm[ecca59f57bcd7213]::LlvmCodegenBackend>
  27:     0x7f83722fb5d4 - <rustc_codegen_llvm[ecca59f57bcd7213]::LlvmCodegenBackend as rustc_codegen_ssa[958d09c1bf53546e]::traits::backend::CodegenBackend>::codegen_crate
  28:     0x7f837226f87f - <rustc_session[6fe6800af82aa7e3]::session::Session>::time::<alloc[3a3a71201775eca6]::boxed::Box<dyn core[f365564bdbb19c62]::any::Any>, rustc_interface[f7d8d044a3e34b97]::passes::start_codegen::{closure#0}>
  29:     0x7f83721e8e78 - rustc_interface[f7d8d044a3e34b97]::passes::start_codegen
  30:     0x7f837228ff3f - <std[e22443783e85f7c8]::thread::local::LocalKey<core[f365564bdbb19c62]::cell::Cell<*const ()>>>::with::<rustc_middle[52e642eb40420eda]::ty::context::tls::enter_context<<rustc_middle[52e642eb40420eda]::ty::context::GlobalCtxt>::enter<<rustc_interface[f7d8d044a3e34b97]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[f365564bdbb19c62]::result::Result<alloc[3a3a71201775eca6]::boxed::Box<dyn core[f365564bdbb19c62]::any::Any>, rustc_span[1c8ddf94e18f49d9]::ErrorGuaranteed>>::{closure#0}, core[f365564bdbb19c62]::result::Result<alloc[3a3a71201775eca6]::boxed::Box<dyn core[f365564bdbb19c62]::any::Any>, rustc_span[1c8ddf94e18f49d9]::ErrorGuaranteed>>::{closure#0}, core[f365564bdbb19c62]::result::Result<alloc[3a3a71201775eca6]::boxed::Box<dyn core[f365564bdbb19c62]::any::Any>, rustc_span[1c8ddf94e18f49d9]::ErrorGuaranteed>>
  31:     0x7f83721ebaad - <rustc_middle[52e642eb40420eda]::ty::context::GlobalCtxt>::enter::<<rustc_interface[f7d8d044a3e34b97]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[f365564bdbb19c62]::result::Result<alloc[3a3a71201775eca6]::boxed::Box<dyn core[f365564bdbb19c62]::any::Any>, rustc_span[1c8ddf94e18f49d9]::ErrorGuaranteed>>
  32:     0x7f83722be594 - <rustc_interface[f7d8d044a3e34b97]::queries::Queries>::ongoing_codegen
  33:     0x7f8372166802 - <rustc_interface[f7d8d044a3e34b97]::interface::Compiler>::enter::<rustc_driver_impl[742362009a70431c]::run_compiler::{closure#1}::{closure#2}, core[f365564bdbb19c62]::result::Result<core[f365564bdbb19c62]::option::Option<rustc_interface[f7d8d044a3e34b97]::queries::Linker>, rustc_span[1c8ddf94e18f49d9]::ErrorGuaranteed>>
  34:     0x7f837213468e - std[e22443783e85f7c8]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[f7d8d044a3e34b97]::util::run_in_thread_pool_with_globals<rustc_interface[f7d8d044a3e34b97]::interface::run_compiler<core[f365564bdbb19c62]::result::Result<(), rustc_span[1c8ddf94e18f49d9]::ErrorGuaranteed>, rustc_driver_impl[742362009a70431c]::run_compiler::{closure#1}>::{closure#0}, core[f365564bdbb19c62]::result::Result<(), rustc_span[1c8ddf94e18f49d9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[f365564bdbb19c62]::result::Result<(), rustc_span[1c8ddf94e18f49d9]::ErrorGuaranteed>>
  35:     0x7f8372168568 - std[e22443783e85f7c8]::panicking::try::<core[f365564bdbb19c62]::result::Result<(), rustc_span[1c8ddf94e18f49d9]::ErrorGuaranteed>, core[f365564bdbb19c62]::panic::unwind_safe::AssertUnwindSafe<<std[e22443783e85f7c8]::thread::Builder>::spawn_unchecked_<rustc_interface[f7d8d044a3e34b97]::util::run_in_thread_pool_with_globals<rustc_interface[f7d8d044a3e34b97]::interface::run_compiler<core[f365564bdbb19c62]::result::Result<(), rustc_span[1c8ddf94e18f49d9]::ErrorGuaranteed>, rustc_driver_impl[742362009a70431c]::run_compiler::{closure#1}>::{closure#0}, core[f365564bdbb19c62]::result::Result<(), rustc_span[1c8ddf94e18f49d9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[f365564bdbb19c62]::result::Result<(), rustc_span[1c8ddf94e18f49d9]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  36:     0x7f8372136a17 - <<std[e22443783e85f7c8]::thread::Builder>::spawn_unchecked_<rustc_interface[f7d8d044a3e34b97]::util::run_in_thread_pool_with_globals<rustc_interface[f7d8d044a3e34b97]::interface::run_compiler<core[f365564bdbb19c62]::result::Result<(), rustc_span[1c8ddf94e18f49d9]::ErrorGuaranteed>, rustc_driver_impl[742362009a70431c]::run_compiler::{closure#1}>::{closure#0}, core[f365564bdbb19c62]::result::Result<(), rustc_span[1c8ddf94e18f49d9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[f365564bdbb19c62]::result::Result<(), rustc_span[1c8ddf94e18f49d9]::ErrorGuaranteed>>::{closure#1} as core[f365564bdbb19c62]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  37:     0x7f837164865e - std::sys::unix::thread::Thread::new::thread_start::hcced726475fdfa01
  38:     0x7f83713dfb43 - <unknown>
  39:     0x7f8371471a00 - <unknown>
  40:                0x0 - <unknown>
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.70.0-nightly (2c60df649 2023-04-06) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -C opt-level=1 -Z dump-mir=all -Z validate-mir -Z dump-mir-exclude-pass-number -Z mir-pretty-relative-line-numbers=yes -Z mir-opt-level=0 -Z mir-enable-passes=+LowerIntrinsics -Z dump-mir-dir=/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/lower_intrinsics -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
error: aborting due to previous error
------------------------------------------
