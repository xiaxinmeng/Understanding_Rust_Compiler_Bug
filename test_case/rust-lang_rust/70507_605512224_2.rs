
warning: the feature `const_generics` is incomplete and may cause the compiler to crash
 --> main.rs:1:12
  |
1 | #![feature(const_generics)]
  |            ^^^^^^^^^^^^^^
  |
  = note: `#[warn(incomplete_features)]` on by default

error: internal compiler error: unexpected const parent in type_of_def_id(): Expr(expr(HirId { owner: DefIndex(8), local_id: 6 }: R.method::<>()))

error: internal compiler error: mir_const_qualif: MIR had errors
 --> main.rs:8:16
  |
8 |     R.method::<1u8>();
  |                ^^^

error: internal compiler error: PromoteTemps: MIR had errors
 --> main.rs:8:16
  |
8 |     R.method::<1u8>();
  |                ^^^

error: internal compiler error: broken MIR in DefId(0:9 ~ main[317d]::main[0]::{{constant}}[0]) ("return type"): bad type [type error]
 --> main.rs:8:16
  |
8 |     R.method::<1u8>();
  |                ^^^

error: internal compiler error: broken MIR in DefId(0:9 ~ main[317d]::main[0]::{{constant}}[0]) (LocalDecl { mutability: Mut, local_info: Other, internal: false, is_block_tail: None, ty: [type error], user_ty: UserTypeProjections { contents: [] }, source_info: SourceInfo { span: main.rs:8:16: 8:19, scope: scope[0] } }): bad type [type error]
 --> main.rs:8:16
  |
8 |     R.method::<1u8>();
  |                ^^^

thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', src\librustc_errors\lib.rs:346:17
stack backtrace:
   0: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
   1: core::fmt::write
   2: <std::io::IoSliceMut as core::fmt::Debug>::fmt
   3: std::panicking::take_hook
   4: std::panicking::take_hook
   5: rustc_driver::report_ice
   6: std::panicking::rust_panic_with_hook
   7: rustc_errors::annotate_snippet_emitter_writer::AnnotateSnippetEmitterWriter::ui_testing
   8: <rustc_errors::HandlerInner as core::ops::drop::Drop>::drop
   9: rustc_driver::pretty::print_after_hir_lowering
  10: rustc_driver::pretty::print_after_hir_lowering
  11: rustc_driver::pretty::print_after_hir_lowering
  12: rustc_driver::pretty::print_after_hir_lowering
  13: <rustc_span::symbol::SymbolStr as core::fmt::Display>::fmt
  14: <env_logger::filter::inner::Filter as core::fmt::Display>::fmt
  15: <env_logger::filter::inner::Filter as core::fmt::Display>::fmt
  16: _rust_maybe_catch_panic
  17: rustc_driver::pretty::print_after_hir_lowering
  18: ZN244_$LT$std..error..$LT$impl$u20$core..convert..From$LT$alloc..string..String$GT$$u20$for$u20$alloc..boxed..Box$LT$dyn$u20$std..error..Error$u2b$core..marker..Sync$u2b$core..marker..Send$GT$$GT$..from..StringError$u20$as$u20$core..fmt..Display$GT$3fmt17
  19: std::sys::windows::thread::Thread::new
  20: BaseThreadInitThunk
  21: RtlUserThreadStart
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.42.0-nightly (8a87b945b 2020-01-14) running on x86_64-pc-windows-msvc

query stack during panic:
end of query stack
