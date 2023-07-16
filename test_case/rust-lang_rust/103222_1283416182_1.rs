
PS C:\Users\adodson\dev\test-bug> rustc .\src\main.rs
warning: field `execute` is never read
  --> .\src\main.rs:10:5
   |
9  | struct Instruction {
   |        ----------- field in this struct
10 |     execute: fn(),
   |     ^^^^^^^
   |
   = note: `#[warn(dead_code)]` on by default

error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: TyKind::Error constructed but no error reported
  |
  = note: delayed at compiler\rustc_hir_analysis\src\check\expr.rs:352:39

error: internal compiler error: `InferCtxt` incorrectly tainted by errors
  |
  = note: delayed at compiler\rustc_infer\src\infer\mod.rs:1256:27

error: internal compiler error: TyKind::Error constructed but no error reported
  |
  = note: delayed at compiler\rustc_hir_analysis\src\check\coercion.rs:1469:42

error: internal compiler error: TyKind::Error constructed but no error reported
  |
  = note: delayed at compiler\rustc_infer\src\infer\sub.rs:123:31

error: internal compiler error: TyKind::Error constructed but no error reported
  |
  = note: delayed at compiler\rustc_hir_analysis\src\check\fallback.rs:109:58

error: internal compiler error: TyKind::Error constructed but no error reported
  |
  = note: delayed at /rustc/a24a020e6d926dffe6b472fc647978f92269504e\compiler\rustc_middle\src\ty\relate.rs:419:59

error: internal compiler error: TyKind::Error constructed but no error reported
  |
  = note: delayed at compiler\rustc_mir_build\src\build\mod.rs:621:18

error: internal compiler error: PromoteTemps: MIR had errors
  --> .\src\main.rs:13:1
   |
13 | fn main() {
   | ^^^^^^^^^
   |
   = note: delayed at compiler\rustc_const_eval\src\transform\promote_consts.rs:53:22

error: internal compiler error: broken MIR in DefId(0:6 ~ main[2af6]::main) ("return type"): bad type [type error]
  --> .\src\main.rs:13:1
   |
13 | fn main() {
   | ^^^^^^^^^
   |
   = note: delayed at compiler\rustc_borrowck\src\type_check\mod.rs:520:13

error: internal compiler error: TyKind::Error constructed but no error reported
  |
  = note: delayed at compiler\rustc_borrowck\src\type_check\mod.rs:797:20

error: internal compiler error: broken MIR in DefId(0:6 ~ main[2af6]::main) (LocalDecl { mutability: Mut, local_info: None, internal: false, is_block_tail: None, ty: [type error], user_ty: None, source_info: SourceInfo { span: .\src\main.rs:13:1: 13:10 (#0), scope: scope[0] } }): bad type [type error]
  --> .\src\main.rs:13:1
   |
13 | fn main() {
   | ^^^^^^^^^
   |
   = note: delayed at compiler\rustc_borrowck\src\type_check\mod.rs:520:13

error: internal compiler error: PromoteTemps: MIR had errors
  --> .\src\main.rs:15:16
   |
15 |         INST ( || {}; )
   |                ^^
   |
   = note: delayed at compiler\rustc_const_eval\src\transform\promote_consts.rs:53:22

error: internal compiler error: broken MIR in DefId(0:7 ~ main[2af6]::main::{closure#0}) ("return type"): bad type [type error]
  --> .\src\main.rs:15:16
   |
15 |         INST ( || {}; )
   |                ^^
   |
   = note: delayed at compiler\rustc_borrowck\src\type_check\mod.rs:520:13

error: internal compiler error: broken MIR in DefId(0:7 ~ main[2af6]::main::{closure#0}) (LocalDecl { mutability: Mut, local_info: None, internal: false, is_block_tail: None, ty: [type error], user_ty: None, source_info: SourceInfo { span: .\src\main.rs:15:16: 15:18 (#0), scope: scope[0] } }): bad type [type error]
  --> .\src\main.rs:15:16
   |
15 |         INST ( || {}; )
   |                ^^
   |
   = note: delayed at compiler\rustc_borrowck\src\type_check\mod.rs:520:13

thread 'rustc' panicked at 'Box<dyn Any>', compiler\rustc_errors\src\lib.rs:1550:13
stack backtrace:
   0:     0x7ffb00b397f2 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h53c4ea2e5298ee7b
   1:     0x7ffb00b750fb - core::fmt::write::h4929c0534d871047
   2:     0x7ffb00b2be1a - <std::io::IoSlice as core::fmt::Debug>::fmt::hea1586eafdc30c79
   3:     0x7ffb00b3953b - std::sys::common::alloc::realloc_fallback::h441a327114321be3
   4:     0x7ffb00b3ce99 - std::panicking::default_hook::h66f001930739e8fd
   5:     0x7ffb00b3cb1a - std::panicking::default_hook::h66f001930739e8fd
   6:     0x7ffae580086b - rustc_driver[1077fc6396368dcf]::describe_lints
   7:     0x7ffb00b3d7f0 - std::panicking::rust_panic_with_hook::h059540785d16ed47
   8:     0x7ffae7bac033 - <rustc_hir[c703180ab4123f64]::target::Target as rustc_errors[77b34b757f114d60]::diagnostic::IntoDiagnosticArg>::into_diagnostic_arg
   9:     0x7ffae7baa819 - <rustc_hir[c703180ab4123f64]::target::Target as rustc_errors[77b34b757f114d60]::diagnostic::IntoDiagnosticArg>::into_diagnostic_arg
  10:     0x7ffae7b9bec9 - <rustc_errors[77b34b757f114d60]::emitter::DisplaySuggestion as core[a02b6f37981f1b5f]::fmt::Debug>::fmt
  11:     0x7ffae7b93b69 - <getopts[69c649ba10df24e4]::Fail as core[a02b6f37981f1b5f]::fmt::Debug>::fmt
  12:     0x7ffae3ef68e3 - <rustc_errors[77b34b757f114d60]::HandlerInner>::emit_diagnostic
  13:     0x7ffae3ef582d - <rustc_errors[77b34b757f114d60]::Handler>::flush_delayed
  14:     0x7ffae30753d0 - <rustc_interface[69d3caabc375c611]::passes::LintStoreExpandImpl as rustc_expand[c0c33283810bd582]::base::LintStoreExpand>::pre_expansion_lint
  15:     0x7ffae305a47c - <rustc_interface[69d3caabc375c611]::queries::Queries>::ongoing_codegen
  16:     0x7ffae30135d9 - <unknown>
  17:     0x7ffae304788a - <rustc_middle[49f1884e63a0b2fb]::ty::SymbolName as core[a02b6f37981f1b5f]::fmt::Display>::fmt
  18:     0x7ffae3015412 - <unknown>
  19:     0x7ffae30471b7 - rustc_driver[1077fc6396368dcf]::args::arg_expand_all
  20:     0x7ffae3038899 - <unknown>
  21:     0x7ffae3030900 - <unknown>
  22:     0x7ffb00b4e53c - std::sys::windows::thread::Thread::new::h5087fa34a1aa6292
  23:     0x7ffb46107034 - BaseThreadInitThunk
  24:     0x7ffb479826a1 - RtlUserThreadStart

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.66.0-nightly (a24a020e6 2022-10-18) running on x86_64-pc-windows-msvc

query stack during panic:
end of query stack
error: aborting due to 15 previous errors; 1 warning emitted
