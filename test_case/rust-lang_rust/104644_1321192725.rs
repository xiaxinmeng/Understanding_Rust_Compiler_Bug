plain
........................................................................................ 8712/13880
................................................................................i...ii.. 8800/13880
...........................................................ii........................... 8888/13880
.....................................................................iiii............... 8976/13880
...................................................................................F.... 9064/13880
.......................iF...F...F...............................i....................... 9152/13880
........................................................................................ 9328/13880
.................................................i...................................... 9416/13880
........................................................................................ 9504/13880
............................................................i........................... 9592/13880
---
---- [ui] src/test/ui/allocator/no_std-alloc-error-handler-default.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/allocator/no_std-alloc-error-handler-default.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/no_std-alloc-error-handler-default/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/no_std-alloc-error-handler-default/auxiliary" "-C" "panic=abort"
stdout: none
--- stderr -------------------------------
Block containing LandingPadInst must be jumped to only by the unwind edge of an invoke.
  %147 = landingpad { i8*, i32 }
          cleanup
Block containing LandingPadInst must be jumped to only by the unwind edge of an invoke.
  %163 = landingpad { i8*, i32 }
LLVM ERROR: Broken module found, compilation aborted!
------------------------------------------
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu



---- [ui] src/test/ui/allocator/no_std-alloc-error-handler-custom.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/allocator/no_std-alloc-error-handler-custom.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/no_std-alloc-error-handler-custom/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator/no_std-alloc-error-handler-custom/auxiliary" "-C" "panic=abort"
stdout: none
--- stderr -------------------------------
Block containing LandingPadInst must be jumped to only by the unwind edge of an invoke.
  %147 = landingpad { i8*, i32 }
          cleanup
Block containing LandingPadInst must be jumped to only by the unwind edge of an invoke.
  %163 = landingpad { i8*, i32 }
LLVM ERROR: Broken module found, compilation aborted!
------------------------------------------


---
thread 'main' panicked at 'Box<dyn Any>', /checkout/src/test/ui/drop/dynamic-drop.rs:84:13
thread 'main' panicked at 'Box<dyn Any>', /checkout/src/test/ui/drop/dynamic-drop.rs:47:13
thread 'main' panicked at 'Box<dyn Any>', /checkout/src/test/ui/drop/dynamic-drop.rs:47:13
thread 'main' panicked at 'Box<dyn Any>', /checkout/src/test/ui/drop/dynamic-drop.rs:84:13
thread 'main' panicked at 'missing free: [true, false]', /checkout/src/test/ui/drop/dynamic-drop.rs:30:13


---- [ui] src/test/ui/generator/drop-env.rs#default stdout ----


error in revision `default`: test run failed!
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/drop-env.default/a"
stdout: none
--- stderr -------------------------------
thread 'main' panicked at 'assertion failed: `(left == right)`
---


---- [ui] src/test/ui/generator/smoke.rs#default stdout ----

error in revision `default`: test run failed!
status: signal: 6 (SIGABRT) (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/smoke.default/a"
running 8 tests
------------------------------------------
--- stderr -------------------------------
--- stderr -------------------------------
free(): double free detected in tcache 2
free(): double free detected in tcache 2


---- [ui] src/test/ui/lto/dylib-works.rs stdout ----


error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lto/dylib-works.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lto/dylib-works/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lto/dylib-works/auxiliary"
stdout: none
--- stderr -------------------------------
Block containing LandingPadInst must be jumped to only by the unwind edge of an invoke.
  %24 = landingpad { i8*, i32 }
LLVM ERROR: Broken module found, compilation aborted!
------------------------------------------


---
---- [ui] src/test/ui/panic-runtime/abort.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/panic-runtime/abort.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/abort/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/abort/auxiliary" "-C" "panic=abort"
stdout: none
--- stderr -------------------------------
error: internal compiler error: compiler/rustc_codegen_ssa/src/mir/analyze.rs:308:21: funclet bb7 has 2 parents - bb21 and bb77
   |
LL | / fn main() {
LL | / fn main() {
LL | |     let mut args = env::args_os();
LL | |     let me = args.next().unwrap();
LL | |
...  |
LL | |     assert!(s.unwrap().code() != Some(0));
LL | | }

thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:969:33
stack backtrace:
stack backtrace:
   0:     0x7fa822dff9be - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hfa9c536279f92c1e
   1:     0x7fa822e70308 - core::fmt::write::h6b4f4d25c62b8232
   2:     0x7fa822df1811 - std::io::Write::write_fmt::h534aff8aa4bcb215
   3:     0x7fa822dff7c1 - std::sys_common::backtrace::print::h5a26d90cc226c531
   4:     0x7fa822e02b14 - std::panicking::default_hook::{{closure}}::h886ecc7f329f7d25
   5:     0x7fa822e027d9 - std::panicking::default_hook::h4e30a78e7c4235c3
   6:     0x7fa8238fa004 - rustc_driver[3afbbf468a2719a4]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fa822e03264 - std::panicking::rust_panic_with_hook::he534fe0ef500f57f
   8:     0x7fa825957253 - std[253f16bec9cca3a6]::panicking::begin_panic::<rustc_errors[a06bd71878cb3d2e]::ExplicitBug>::{closure#0}
   9:     0x7fa8259569d6 - std[253f16bec9cca3a6]::sys_common::backtrace::__rust_end_short_backtrace::<std[253f16bec9cca3a6]::panicking::begin_panic<rustc_errors[a06bd71878cb3d2e]::ExplicitBug>::{closure#0}, !>
  10:     0x7fa82377a106 - std[253f16bec9cca3a6]::panicking::begin_panic::<rustc_errors[a06bd71878cb3d2e]::ExplicitBug>
  11:     0x7fa825a015b6 - std[253f16bec9cca3a6]::panic::panic_any::<rustc_errors[a06bd71878cb3d2e]::ExplicitBug>
  12:     0x7fa825a01008 - <rustc_errors[a06bd71878cb3d2e]::HandlerInner>::span_bug::<rustc_span[ff0866a2c64d31da]::span_encoding::Span, &alloc[e2bfebd55119440b]::string::String>
  13:     0x7fa825a009c0 - <rustc_errors[a06bd71878cb3d2e]::Handler>::span_bug::<rustc_span[ff0866a2c64d31da]::span_encoding::Span, &alloc[e2bfebd55119440b]::string::String>
  14:     0x7fa825989f92 - rustc_middle[e58805825ef8070e]::ty::context::tls::with_context_opt::<rustc_middle[e58805825ef8070e]::ty::context::tls::with_opt<rustc_middle[e58805825ef8070e]::util::bug::opt_span_bug_fmt<rustc_span[ff0866a2c64d31da]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  15:     0x7fa825989d69 - rustc_middle[e58805825ef8070e]::util::bug::opt_span_bug_fmt::<rustc_span[ff0866a2c64d31da]::span_encoding::Span>
  16:     0x7fa82377a18a - rustc_middle[e58805825ef8070e]::util::bug::span_bug_fmt::<rustc_span[ff0866a2c64d31da]::span_encoding::Span>
  17:     0x7fa82595dabd - rustc_codegen_ssa[f50447f64831a702]::mir::analyze::cleanup_kinds::propagate::{closure#0}
  18:     0x7fa82595ccbb - rustc_codegen_ssa[f50447f64831a702]::mir::analyze::cleanup_kinds
  19:     0x7fa823bf0503 - rustc_codegen_ssa[f50447f64831a702]::mir::codegen_mir::<rustc_codegen_llvm[db1defb358bde038]::builder::Builder>
  20:     0x7fa823b34a08 - rustc_codegen_ssa[f50447f64831a702]::base::codegen_instance::<rustc_codegen_llvm[db1defb358bde038]::builder::Builder>
  21:     0x7fa823baf873 - rustc_codegen_llvm[db1defb358bde038]::base::compile_codegen_unit::module_codegen
  22:     0x7fa823ac0c77 - <rustc_query_system[3aa5764e6d48ba80]::dep_graph::graph::DepGraph<rustc_middle[e58805825ef8070e]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[e58805825ef8070e]::ty::context::TyCtxt, rustc_span[ff0866a2c64d31da]::symbol::Symbol, rustc_codegen_ssa[f50447f64831a702]::ModuleCodegen<rustc_codegen_llvm[db1defb358bde038]::ModuleLlvm>>
  23:     0x7fa823baecee - rustc_codegen_llvm[db1defb358bde038]::base::compile_codegen_unit
  24:     0x7fa823b33c9c - rustc_codegen_ssa[f50447f64831a702]::base::codegen_crate::<rustc_codegen_llvm[db1defb358bde038]::LlvmCodegenBackend>
  25:     0x7fa823bde0d4 - <rustc_codegen_llvm[db1defb358bde038]::LlvmCodegenBackend as rustc_codegen_ssa[f50447f64831a702]::traits::backend::CodegenBackend>::codegen_crate
  26:     0x7fa8239f2080 - rustc_interface[530b92b4a7443287]::passes::start_codegen
  27:     0x7fa8239ef225 - <rustc_interface[530b92b4a7443287]::passes::QueryContext>::enter::<<rustc_interface[530b92b4a7443287]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[c78c39d29a66721d]::result::Result<alloc[e2bfebd55119440b]::boxed::Box<dyn core[c78c39d29a66721d]::any::Any>, rustc_errors[a06bd71878cb3d2e]::ErrorGuaranteed>>
  28:     0x7fa8239c5572 - <rustc_interface[530b92b4a7443287]::queries::Queries>::ongoing_codegen
  29:     0x7fa82385d55f - <rustc_interface[530b92b4a7443287]::interface::Compiler>::enter::<rustc_driver[3afbbf468a2719a4]::run_compiler::{closure#1}::{closure#2}, core[c78c39d29a66721d]::result::Result<core[c78c39d29a66721d]::option::Option<rustc_interface[530b92b4a7443287]::queries::Linker>, rustc_errors[a06bd71878cb3d2e]::ErrorGuaranteed>>
  30:     0x7fa82387d423 - rustc_span[ff0866a2c64d31da]::with_source_map::<core[c78c39d29a66721d]::result::Result<(), rustc_errors[a06bd71878cb3d2e]::ErrorGuaranteed>, rustc_interface[530b92b4a7443287]::interface::run_compiler<core[c78c39d29a66721d]::result::Result<(), rustc_errors[a06bd71878cb3d2e]::ErrorGuaranteed>, rustc_driver[3afbbf468a2719a4]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  31:     0x7fa8238e101c - <scoped_tls[daba58c055b5a37]::ScopedKey<rustc_span[ff0866a2c64d31da]::SessionGlobals>>::set::<rustc_interface[530b92b4a7443287]::interface::run_compiler<core[c78c39d29a66721d]::result::Result<(), rustc_errors[a06bd71878cb3d2e]::ErrorGuaranteed>, rustc_driver[3afbbf468a2719a4]::run_compiler::{closure#1}>::{closure#0}, core[c78c39d29a66721d]::result::Result<(), rustc_errors[a06bd71878cb3d2e]::ErrorGuaranteed>>
  32:     0x7fa8238d69fa - std[253f16bec9cca3a6]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[530b92b4a7443287]::util::run_in_thread_pool_with_globals<rustc_interface[530b92b4a7443287]::interface::run_compiler<core[c78c39d29a66721d]::result::Result<(), rustc_errors[a06bd71878cb3d2e]::ErrorGuaranteed>, rustc_driver[3afbbf468a2719a4]::run_compiler::{closure#1}>::{closure#0}, core[c78c39d29a66721d]::result::Result<(), rustc_errors[a06bd71878cb3d2e]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[c78c39d29a66721d]::result::Result<(), rustc_errors[a06bd71878cb3d2e]::ErrorGuaranteed>>
  33:     0x7fa8238e24d6 - std[253f16bec9cca3a6]::panicking::try::<core[c78c39d29a66721d]::result::Result<(), rustc_errors[a06bd71878cb3d2e]::ErrorGuaranteed>, core[c78c39d29a66721d]::panic::unwind_safe::AssertUnwindSafe<<std[253f16bec9cca3a6]::thread::Builder>::spawn_unchecked_<rustc_interface[530b92b4a7443287]::util::run_in_thread_pool_with_globals<rustc_interface[530b92b4a7443287]::interface::run_compiler<core[c78c39d29a66721d]::result::Result<(), rustc_errors[a06bd71878cb3d2e]::ErrorGuaranteed>, rustc_driver[3afbbf468a2719a4]::run_compiler::{closure#1}>::{closure#0}, core[c78c39d29a66721d]::result::Result<(), rustc_errors[a06bd71878cb3d2e]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[c78c39d29a66721d]::result::Result<(), rustc_errors[a06bd71878cb3d2e]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  34:     0x7fa82389493a - <<std[253f16bec9cca3a6]::thread::Builder>::spawn_unchecked_<rustc_interface[530b92b4a7443287]::util::run_in_thread_pool_with_globals<rustc_interface[530b92b4a7443287]::interface::run_compiler<core[c78c39d29a66721d]::result::Result<(), rustc_errors[a06bd71878cb3d2e]::ErrorGuaranteed>, rustc_driver[3afbbf468a2719a4]::run_compiler::{closure#1}>::{closure#0}, core[c78c39d29a66721d]::result::Result<(), rustc_errors[a06bd71878cb3d2e]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[c78c39d29a66721d]::result::Result<(), rustc_errors[a06bd71878cb3d2e]::ErrorGuaranteed>>::{closure#1} as core[c78c39d29a66721d]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  35:     0x7fa822e0fbde - std::sys::unix::thread::Thread::new::thread_start::hae46e0d431f44fd6
  36:     0x7fa822ba4b43 - <unknown>
  37:     0x7fa822c36a00 - <unknown>
  38:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.67.0-nightly (053ccfa41 2022-11-20) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C rpath -C debuginfo=0 -C panic=abort
query stack during panic:
end of query stack
error: aborting due to previous error
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/panic-runtime/abort-link-to-unwinding-crates.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/panic-runtime/abort-link-to-unwinding-crates.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/abort-link-to-unwinding-crates/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/abort-link-to-unwinding-crates/auxiliary" "-C" "panic=abort"
stdout: none
--- stderr -------------------------------
error: internal compiler error: compiler/rustc_codegen_ssa/src/mir/analyze.rs:308:21: funclet bb8 has 2 parents - bb22 and bb78
   |
LL | / fn main() {
LL | / fn main() {
LL | |     let mut args = env::args_os();
LL | |     let me = args.next().unwrap();
LL | |
...  |
LL | |     assert!(s.unwrap().code() != Some(0));
LL | | }

thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:969:33
stack backtrace:
stack backtrace:
   0:     0x7f69555cd9be - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hfa9c536279f92c1e
   1:     0x7f695563e308 - core::fmt::write::h6b4f4d25c62b8232
   2:     0x7f69555bf811 - std::io::Write::write_fmt::h534aff8aa4bcb215
   3:     0x7f69555cd7c1 - std::sys_common::backtrace::print::h5a26d90cc226c531
   4:     0x7f69555d0b14 - std::panicking::default_hook::{{closure}}::h886ecc7f329f7d25
   5:     0x7f69555d07d9 - std::panicking::default_hook::h4e30a78e7c4235c3
   6:     0x7f69560c8004 - rustc_driver[3afbbf468a2719a4]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f69555d1264 - std::panicking::rust_panic_with_hook::he534fe0ef500f57f
   8:     0x7f6958125253 - std[253f16bec9cca3a6]::panicking::begin_panic::<rustc_errors[a06bd71878cb3d2e]::ExplicitBug>::{closure#0}
   9:     0x7f69581249d6 - std[253f16bec9cca3a6]::sys_common::backtrace::__rust_end_short_backtrace::<std[253f16bec9cca3a6]::panicking::begin_panic<rustc_errors[a06bd71878cb3d2e]::ExplicitBug>::{closure#0}, !>
  10:     0x7f6955f48106 - std[253f16bec9cca3a6]::panicking::begin_panic::<rustc_errors[a06bd71878cb3d2e]::ExplicitBug>
  11:     0x7f69581cf5b6 - std[253f16bec9cca3a6]::panic::panic_any::<rustc_errors[a06bd71878cb3d2e]::ExplicitBug>
  12:     0x7f69581cf008 - <rustc_errors[a06bd71878cb3d2e]::HandlerInner>::span_bug::<rustc_span[ff0866a2c64d31da]::span_encoding::Span, &alloc[e2bfebd55119440b]::string::String>
  13:     0x7f69581ce9c0 - <rustc_errors[a06bd71878cb3d2e]::Handler>::span_bug::<rustc_span[ff0866a2c64d31da]::span_encoding::Span, &alloc[e2bfebd55119440b]::string::String>
  14:     0x7f6958157f92 - rustc_middle[e58805825ef8070e]::ty::context::tls::with_context_opt::<rustc_middle[e58805825ef8070e]::ty::context::tls::with_opt<rustc_middle[e58805825ef8070e]::util::bug::opt_span_bug_fmt<rustc_span[ff0866a2c64d31da]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  15:     0x7f6958157d69 - rustc_middle[e58805825ef8070e]::util::bug::opt_span_bug_fmt::<rustc_span[ff0866a2c64d31da]::span_encoding::Span>
  16:     0x7f6955f4818a - rustc_middle[e58805825ef8070e]::util::bug::span_bug_fmt::<rustc_span[ff0866a2c64d31da]::span_encoding::Span>
  17:     0x7f695812babd - rustc_codegen_ssa[f50447f64831a702]::mir::analyze::cleanup_kinds::propagate::{closure#0}
  18:     0x7f695812acbb - rustc_codegen_ssa[f50447f64831a702]::mir::analyze::cleanup_kinds
  19:     0x7f69563be503 - rustc_codegen_ssa[f50447f64831a702]::mir::codegen_mir::<rustc_codegen_llvm[db1defb358bde038]::builder::Builder>
  20:     0x7f6956302a08 - rustc_codegen_ssa[f50447f64831a702]::base::codegen_instance::<rustc_codegen_llvm[db1defb358bde038]::builder::Builder>
  21:     0x7f695637d873 - rustc_codegen_llvm[db1defb358bde038]::base::compile_codegen_unit::module_codegen
  22:     0x7f695628ec77 - <rustc_query_system[3aa5764e6d48ba80]::dep_graph::graph::DepGraph<rustc_middle[e58805825ef8070e]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[e58805825ef8070e]::ty::context::TyCtxt, rustc_span[ff0866a2c64d31da]::symbol::Symbol, rustc_codegen_ssa[f50447f64831a702]::ModuleCodegen<rustc_codegen_llvm[db1defb358bde038]::ModuleLlvm>>
  23:     0x7f695637ccee - rustc_codegen_llvm[db1defb358bde038]::base::compile_codegen_unit
  24:     0x7f6956301c9c - rustc_codegen_ssa[f50447f64831a702]::base::codegen_crate::<rustc_codegen_llvm[db1defb358bde038]::LlvmCodegenBackend>
  25:     0x7f69563ac0d4 - <rustc_codegen_llvm[db1defb358bde038]::LlvmCodegenBackend as rustc_codegen_ssa[f50447f64831a702]::traits::backend::CodegenBackend>::codegen_crate
  26:     0x7f69561c0080 - rustc_interface[530b92b4a7443287]::passes::start_codegen
  27:     0x7f69561bd225 - <rustc_interface[530b92b4a7443287]::passes::QueryContext>::enter::<<rustc_interface[530b92b4a7443287]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[c78c39d29a66721d]::result::Result<alloc[e2bfebd55119440b]::boxed::Box<dyn core[c78c39d29a66721d]::any::Any>, rustc_errors[a06bd71878cb3d2e]::ErrorGuaranteed>>
  28:     0x7f6956193572 - <rustc_interface[530b92b4a7443287]::queries::Queries>::ongoing_codegen
  29:     0x7f695602b55f - <rustc_interface[530b92b4a7443287]::interface::Compiler>::enter::<rustc_driver[3afbbf468a2719a4]::run_compiler::{closure#1}::{closure#2}, core[c78c39d29a66721d]::result::Result<core[c78c39d29a66721d]::option::Option<rustc_interface[530b92b4a7443287]::queries::Linker>, rustc_errors[a06bd71878cb3d2e]::ErrorGuaranteed>>
  30:     0x7f695604b423 - rustc_span[ff0866a2c64d31da]::with_source_map::<core[c78c39d29a66721d]::result::Result<(), rustc_errors[a06bd71878cb3d2e]::ErrorGuaranteed>, rustc_interface[530b92b4a7443287]::interface::run_compiler<core[c78c39d29a66721d]::result::Result<(), rustc_errors[a06bd71878cb3d2e]::ErrorGuaranteed>, rustc_driver[3afbbf468a2719a4]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  31:     0x7f69560af01c - <scoped_tls[daba58c055b5a37]::ScopedKey<rustc_span[ff0866a2c64d31da]::SessionGlobals>>::set::<rustc_interface[530b92b4a7443287]::interface::run_compiler<core[c78c39d29a66721d]::result::Result<(), rustc_errors[a06bd71878cb3d2e]::ErrorGuaranteed>, rustc_driver[3afbbf468a2719a4]::run_compiler::{closure#1}>::{closure#0}, core[c78c39d29a66721d]::result::Result<(), rustc_errors[a06bd71878cb3d2e]::ErrorGuaranteed>>
  32:     0x7f69560a49fa - std[253f16bec9cca3a6]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[530b92b4a7443287]::util::run_in_thread_pool_with_globals<rustc_interface[530b92b4a7443287]::interface::run_compiler<core[c78c39d29a66721d]::result::Result<(), rustc_errors[a06bd71878cb3d2e]::ErrorGuaranteed>, rustc_driver[3afbbf468a2719a4]::run_compiler::{closure#1}>::{closure#0}, core[c78c39d29a66721d]::result::Result<(), rustc_errors[a06bd71878cb3d2e]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[c78c39d29a66721d]::result::Result<(), rustc_errors[a06bd71878cb3d2e]::ErrorGuaranteed>>
  33:     0x7f69560b04d6 - std[253f16bec9cca3a6]::panicking::try::<core[c78c39d29a66721d]::result::Result<(), rustc_errors[a06bd71878cb3d2e]::ErrorGuaranteed>, core[c78c39d29a66721d]::panic::unwind_safe::AssertUnwindSafe<<std[253f16bec9cca3a6]::thread::Builder>::spawn_unchecked_<rustc_interface[530b92b4a7443287]::util::run_in_thread_pool_with_globals<rustc_interface[530b92b4a7443287]::interface::run_compiler<core[c78c39d29a66721d]::result::Result<(), rustc_errors[a06bd71878cb3d2e]::ErrorGuaranteed>, rustc_driver[3afbbf468a2719a4]::run_compiler::{closure#1}>::{closure#0}, core[c78c39d29a66721d]::result::Result<(), rustc_errors[a06bd71878cb3d2e]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[c78c39d29a66721d]::result::Result<(), rustc_errors[a06bd71878cb3d2e]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  34:     0x7f695606293a - <<std[253f16bec9cca3a6]::thread::Builder>::spawn_unchecked_<rustc_interface[530b92b4a7443287]::util::run_in_thread_pool_with_globals<rustc_interface[530b92b4a7443287]::interface::run_compiler<core[c78c39d29a66721d]::result::Result<(), rustc_errors[a06bd71878cb3d2e]::ErrorGuaranteed>, rustc_driver[3afbbf468a2719a4]::run_compiler::{closure#1}>::{closure#0}, core[c78c39d29a66721d]::result::Result<(), rustc_errors[a06bd71878cb3d2e]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[c78c39d29a66721d]::result::Result<(), rustc_errors[a06bd71878cb3d2e]::ErrorGuaranteed>>::{closure#1} as core[c78c39d29a66721d]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  35:     0x7f69555ddbde - std::sys::unix::thread::Thread::new::thread_start::hae46e0d431f44fd6
  36:     0x7f6955372b43 - <unknown>
  37:     0x7f6955404a00 - <unknown>
  38:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.67.0-nightly (053ccfa41 2022-11-20) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C rpath -C debuginfo=0 -C panic=abort
query stack during panic:
end of query stack
error: aborting due to previous error
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/panic-runtime/lto-abort.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/panic-runtime/lto-abort.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/lto-abort/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/lto-abort/auxiliary" "-C" "lto" "-C" "panic=abort"
stdout: none
--- stderr -------------------------------
error: internal compiler error: compiler/rustc_codegen_ssa/src/mir/analyze.rs:308:21: funclet bb7 has 2 parents - bb21 and bb77
   |
LL | / fn main() {
LL | / fn main() {
LL | |     let mut args = env::args_os();
LL | |     let me = args.next().unwrap();
LL | |
...  |
LL | |     assert!(s.unwrap().code() != Some(0));
LL | | }

thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:969:33
stack backtrace:
stack backtrace:
   0:     0x7f114fb709be - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hfa9c536279f92c1e
   1:     0x7f114fbe1308 - core::fmt::write::h6b4f4d25c62b8232
   2:     0x7f114fb62811 - std::io::Write::write_fmt::h534aff8aa4bcb215
   3:     0x7f114fb707c1 - std::sys_common::backtrace::print::h5a26d90cc226c531
   4:     0x7f114fb73b14 - std::panicking::default_hook::{{closure}}::h886ecc7f329f7d25
   5:     0x7f114fb737d9 - std::panicking::default_hook::h4e30a78e7c4235c3
   6:     0x7f115066b004 - rustc_driver[3afbbf468a2719a4]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f114fb74264 - std::panicking::rust_panic_with_hook::he534fe0ef500f57f
   8:     0x7f11526c8253 - std[253f16bec9cca3a6]::panicking::begin_panic::<rustc_errors[a06bd71878cb3d2e]::ExplicitBug>::{closure#0}
   9:     0x7f11526c79d6 - std[253f16bec9cca3a6]::sys_common::backtrace::__rust_end_short_backtrace::<std[253f16bec9cca3a6]::panicking::begin_panic<rustc_errors[a06bd71878cb3d2e]::ExplicitBug>::{closure#0}, !>
  10:     0x7f11504eb106 - std[253f16bec9cca3a6]::panicking::begin_panic::<rustc_errors[a06bd71878cb3d2e]::ExplicitBug>
  11:     0x7f11527725b6 - std[253f16bec9cca3a6]::panic::panic_any::<rustc_errors[a06bd71878cb3d2e]::ExplicitBug>
  12:     0x7f1152772008 - <rustc_errors[a06bd71878cb3d2e]::HandlerInner>::span_bug::<rustc_span[ff0866a2c64d31da]::span_encoding::Span, &alloc[e2bfebd55119440b]::string::String>
  13:     0x7f11527719c0 - <rustc_errors[a06bd71878cb3d2e]::Handler>::span_bug::<rustc_span[ff0866a2c64d31da]::span_encoding::Span, &alloc[e2bfebd55119440b]::string::String>
  14:     0x7f11526faf92 - rustc_middle[e58805825ef8070e]::ty::context::tls::with_context_opt::<rustc_middle[e58805825ef8070e]::ty::context::tls::with_opt<rustc_middle[e58805825ef8070e]::util::bug::opt_span_bug_fmt<rustc_span[ff0866a2c64d31da]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  15:     0x7f11526fad69 - rustc_middle[e58805825ef8070e]::util::bug::opt_span_bug_fmt::<rustc_span[ff0866a2c64d31da]::span_encoding::Span>
  16:     0x7f11504eb18a - rustc_middle[e58805825ef8070e]::util::bug::span_bug_fmt::<rustc_span[ff0866a2c64d31da]::span_encoding::Span>
  17:     0x7f11526ceabd - rustc_codegen_ssa[f50447f64831a702]::mir::analyze::cleanup_kinds::propagate::{closure#0}
  18:     0x7f11526cdcbb - rustc_codegen_ssa[f50447f64831a702]::mir::analyze::cleanup_kinds
  19:     0x7f1150961503 - rustc_codegen_ssa[f50447f64831a702]::mir::codegen_mir::<rustc_codegen_llvm[db1defb358bde038]::builder::Builder>
  20:     0x7f11508a5a08 - rustc_codegen_ssa[f50447f64831a702]::base::codegen_instance::<rustc_codegen_llvm[db1defb358bde038]::builder::Builder>
  21:     0x7f1150920873 - rustc_codegen_llvm[db1defb358bde038]::base::compile_codegen_unit::module_codegen
  22:     0x7f1150831c77 - <rustc_query_system[3aa5764e6d48ba80]::dep_graph::graph::DepGraph<rustc_middle[e58805825ef8070e]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[e58805825ef8070e]::ty::context::TyCtxt, rustc_span[ff0866a2c64d31da]::symbol::Symbol, rustc_codegen_ssa[f50447f64831a702]::ModuleCodegen<rustc_codegen_llvm[db1defb358bde038]::ModuleLlvm>>
  23:     0x7f115091fcee - rustc_codegen_llvm[db1defb358bde038]::base::compile_codegen_unit
  24:     0x7f11508a4c9c - rustc_codegen_ssa[f50447f64831a702]::base::codegen_crate::<rustc_codegen_llvm[db1defb358bde038]::LlvmCodegenBackend>
  25:     0x7f115094f0d4 - <rustc_codegen_llvm[db1defb358bde038]::LlvmCodegenBackend as rustc_codegen_ssa[f50447f64831a702]::traits::backend::CodegenBackend>::codegen_crate
  26:     0x7f1150763080 - rustc_interface[530b92b4a7443287]::passes::start_codegen
  27:     0x7f1150760225 - <rustc_interface[530b92b4a7443287]::passes::QueryContext>::enter::<<rustc_interface[530b92b4a7443287]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[c78c39d29a66721d]::result::Result<alloc[e2bfebd55119440b]::boxed::Box<dyn core[c78c39d29a66721d]::any::Any>, rustc_errors[a06bd71878cb3d2e]::ErrorGuaranteed>>
  28:     0x7f1150736572 - <rustc_interface[530b92b4a7443287]::queries::Queries>::ongoing_codegen
  29:     0x7f11505ce55f - <rustc_interface[530b92b4a7443287]::interface::Compiler>::enter::<rustc_driver[3afbbf468a2719a4]::run_compiler::{closure#1}::{closure#2}, core[c78c39d29a66721d]::result::Result<core[c78c39d29a66721d]::option::Option<rustc_interface[530b92b4a7443287]::queries::Linker>, rustc_errors[a06bd71878cb3d2e]::ErrorGuaranteed>>
  30:     0x7f11505ee423 - rustc_span[ff0866a2c64d31da]::with_source_map::<core[c78c39d29a66721d]::result::Result<(), rustc_errors[a06bd71878cb3d2e]::ErrorGuaranteed>, rustc_interface[530b92b4a7443287]::interface::run_compiler<core[c78c39d29a66721d]::result::Result<(), rustc_errors[a06bd71878cb3d2e]::ErrorGuaranteed>, rustc_driver[3afbbf468a2719a4]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  31:     0x7f115065201c - <scoped_tls[daba58c055b5a37]::ScopedKey<rustc_span[ff0866a2c64d31da]::SessionGlobals>>::set::<rustc_interface[530b92b4a7443287]::interface::run_compiler<core[c78c39d29a66721d]::result::Result<(), rustc_errors[a06bd71878cb3d2e]::ErrorGuaranteed>, rustc_driver[3afbbf468a2719a4]::run_compiler::{closure#1}>::{closure#0}, core[c78c39d29a66721d]::result::Result<(), rustc_errors[a06bd71878cb3d2e]::ErrorGuaranteed>>
  32:     0x7f11506479fa - std[253f16bec9cca3a6]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[530b92b4a7443287]::util::run_in_thread_pool_with_globals<rustc_interface[530b92b4a7443287]::interface::run_compiler<core[c78c39d29a66721d]::result::Result<(), rustc_errors[a06bd71878cb3d2e]::ErrorGuaranteed>, rustc_driver[3afbbf468a2719a4]::run_compiler::{closure#1}>::{closure#0}, core[c78c39d29a66721d]::result::Result<(), rustc_errors[a06bd71878cb3d2e]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[c78c39d29a66721d]::result::Result<(), rustc_errors[a06bd71878cb3d2e]::ErrorGuaranteed>>
  33:     0x7f11506534d6 - std[253f16bec9cca3a6]::panicking::try::<core[c78c39d29a66721d]::result::Result<(), rustc_errors[a06bd71878cb3d2e]::ErrorGuaranteed>, core[c78c39d29a66721d]::panic::unwind_safe::AssertUnwindSafe<<std[253f16bec9cca3a6]::thread::Builder>::spawn_unchecked_<rustc_interface[530b92b4a7443287]::util::run_in_thread_pool_with_globals<rustc_interface[530b92b4a7443287]::interface::run_compiler<core[c78c39d29a66721d]::result::Result<(), rustc_errors[a06bd71878cb3d2e]::ErrorGuaranteed>, rustc_driver[3afbbf468a2719a4]::run_compiler::{closure#1}>::{closure#0}, core[c78c39d29a66721d]::result::Result<(), rustc_errors[a06bd71878cb3d2e]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[c78c39d29a66721d]::result::Result<(), rustc_errors[a06bd71878cb3d2e]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  34:     0x7f115060593a - <<std[253f16bec9cca3a6]::thread::Builder>::spawn_unchecked_<rustc_interface[530b92b4a7443287]::util::run_in_thread_pool_with_globals<rustc_interface[530b92b4a7443287]::interface::run_compiler<core[c78c39d29a66721d]::result::Result<(), rustc_errors[a06bd71878cb3d2e]::ErrorGuaranteed>, rustc_driver[3afbbf468a2719a4]::run_compiler::{closure#1}>::{closure#0}, core[c78c39d29a66721d]::result::Result<(), rustc_errors[a06bd71878cb3d2e]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[c78c39d29a66721d]::result::Result<(), rustc_errors[a06bd71878cb3d2e]::ErrorGuaranteed>>::{closure#1} as core[c78c39d29a66721d]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  35:     0x7f114fb80bde - std::sys::unix::thread::Thread::new::thread_start::hae46e0d431f44fd6
  36:     0x7f114f915b43 - <unknown>
  37:     0x7f114f9a7a00 - <unknown>
  38:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.67.0-nightly (053ccfa41 2022-11-20) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C rpath -C debuginfo=0 -C lto -C panic=abort
query stack during panic:
end of query stack
error: aborting due to previous error
------------------------------------------
