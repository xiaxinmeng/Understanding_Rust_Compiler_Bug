
C:/Users/Caio/.cargo/bin/cargo.exe run --package nespera --bin nespera
   Compiling nespera v0.1.0 (C:\Users\Caio\projects\nespera)
error: internal compiler error: cat_expr Errd
 --> src\main.rs:7:14
  |
7 |       fn bar() {
  |  ______________^
8 | |         Self(1u8);
9 | |     }
  | |_____^

error: internal compiler error: cat_expr Errd
 --> src\main.rs:8:9
  |
8 |         Self(1u8);
  |         ^^^^^^^^^

error: internal compiler error: cat_expr Errd
 --> src\main.rs:8:9
  |
8 |         Self(1u8);
  |         ^^^^

error: internal compiler error: QualifyAndPromoteConstants: Mir had errors
 --> src\main.rs:7:5
  |
7 | /     fn bar() {
8 | |         Self(1u8);
9 | |     }
  | |_____^

error: internal compiler error: broken MIR in DefId(0/0:5 ~ nespera[1ed1]::{{impl}}[0]::bar[0]) ("return type"): bad type [type error]
 --> src\main.rs:7:5
  |
7 | /     fn bar() {
8 | |         Self(1u8);
9 | |     }
  | |_____^

error: internal compiler error: broken MIR in DefId(0/0:5 ~ nespera[1ed1]::{{impl}}[0]::bar[0]) (LocalDecl { mutability: Mut, is_user_variable: None, internal: false, is_block_tail: None, ty: [type error], user_ty: UserTypeProjections { contents: [] }, name: None, source_info: SourceInfo { span: src\main.rs:7:5: 9:6, scope: scope[0] }, visibility_scope: scope[0] }): bad type [type error]
 --> src\main.rs:7:5
  |
7 | /     fn bar() {
8 | |         Self(1u8);
9 | |     }
  | |_____^

thread 'main' panicked at 'no errors encountered even though `delay_span_bug` issued', src\librustc_errors\lib.rs:334:17
stack backtrace:
   0: std::sys_common::alloc::realloc_fallback
   1: std::panicking::take_hook
   2: std::panicking::take_hook
   3: rustc::ty::structural_impls::<impl rustc::ty::context::Lift<'tcx> for rustc::ty::instance::InstanceDef<'a>>::lift_to_tcx
   4: std::panicking::rust_panic_with_hook
   5: <rustc_errors::diagnostic::SubDiagnostic as core::fmt::Debug>::fmt
   6: <rustc_errors::Handler as core::ops::drop::Drop>::drop
   7: <humantime::duration::Error as std::error::Error>::cause
   8: <humantime::duration::Error as std::error::Error>::cause
   9: <humantime::duration::Error as std::error::Error>::cause
  10: <rustc_driver::derive_registrar::Finder as rustc::hir::itemlikevisit::ItemLikeVisitor<'v>>::visit_item
  11: _rust_maybe_catch_panic
  12: rustc_driver::profile::dump
  13: rustc_driver::main
  14: <unknown>
  15: std::panicking::update_panic_count
  16: _rust_maybe_catch_panic
  17: std::rt::lang_start_internal
  18: <unknown>
  19: <unknown>
  20: BaseThreadInitThunk
  21: RtlUserThreadStart
query stack during panic:
end of query stack

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.32.0-nightly (1f57e4841 2018-11-23) running on x86_64-pc-windows-msvc

note: compiler flags: -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

error: Could not compile `nespera`.

To learn more, run the command again with --verbose.

Process finished with exit code 101

