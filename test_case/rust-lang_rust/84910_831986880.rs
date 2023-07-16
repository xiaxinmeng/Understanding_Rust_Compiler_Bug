plain
    Checking rustc_middle v0.0.0 (/checkout/compiler/rustc_middle)
    Checking rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
    Checking rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error: internal compiler error: encountered unmarked API: DefId(2:20767 ~ core[f5e5]::num::error::IntErrorKind::PosOverflow::{constructor#0})
  --> compiler/rustc_middle/src/middle/limits.rs:55:25
   |
55 |                         IntErrorKind::PosOverflow => "`limit` is too large",
   |
   = note: delayed at compiler/rustc_middle/src/middle/stability.rs:395:23


error: internal compiler error: encountered unmarked API: DefId(2:20763 ~ core[f5e5]::num::error::IntErrorKind::Empty::{constructor#0})
  --> compiler/rustc_middle/src/middle/limits.rs:56:25
   |
56 |                         IntErrorKind::Empty => "`limit` must be a non-negative integer",
   |
   = note: delayed at compiler/rustc_middle/src/middle/stability.rs:395:23


error: internal compiler error: encountered unmarked API: DefId(2:20765 ~ core[f5e5]::num::error::IntErrorKind::InvalidDigit::{constructor#0})
  --> compiler/rustc_middle/src/middle/limits.rs:57:25
   |
57 |                         IntErrorKind::InvalidDigit => "not a valid integer",
   |
   = note: delayed at compiler/rustc_middle/src/middle/stability.rs:395:23


error: internal compiler error: encountered unmarked API: DefId(2:20769 ~ core[f5e5]::num::error::IntErrorKind::NegOverflow::{constructor#0})
  --> compiler/rustc_middle/src/middle/limits.rs:58:25
   |
58 |                         IntErrorKind::NegOverflow => {
   |
   = note: delayed at compiler/rustc_middle/src/middle/stability.rs:395:23


error: internal compiler error: encountered unmarked API: DefId(2:20771 ~ core[f5e5]::num::error::IntErrorKind::Zero::{constructor#0})
  --> compiler/rustc_middle/src/middle/limits.rs:61:25
   |
61 |                         IntErrorKind::Zero => bug!("zero is a valid `limit`"),
   |
   = note: delayed at compiler/rustc_middle/src/middle/stability.rs:395:23


thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', compiler/rustc_errors/src/lib.rs:1013:13

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.52.0-beta.3 (215738137 2021-04-06) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z macro-backtrace -Z tls-model=initial-exec -Z unstable-options -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic --crate-type lib
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
end of query stack
error: could not compile `rustc_middle`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_driver" "-p" "rustc_typeck" "-p" "rustc_macros" "-p" "rustc_trait_selection" "-p" "rustc_parse_format" "-p" "rustc_lexer" "-p" "rustc_attr" "-p" "rustc_arena" "-p" "rustc_index" "-p" "rustc_infer" "-p" "rustc_graphviz" "-p" "rustc_ty_utils" "-p" "rustc_middle" "-p" "rustc_type_ir" "-p" "rustc_query_system" "-p" "rustc_apfloat" "-p" "rustc_span" "-p" "rustc_parse" "-p" "rustc_mir_build" "-p" "rustc_metadata" "-p" "rustc_expand" "-p" "rustc_lint_defs" "-p" "rustc_ast_passes" "-p" "rustc_data_structures" "-p" "rustc_error_codes" "-p" "rustc_feature" "-p" "rustc_session" "-p" "rustc_fs_util" "-p" "rustc_hir" "-p" "rustc_ast" "-p" "rustc_serialize" "-p" "rustc_lint" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_target" "-p" "rustc_interface" "-p" "rustc_resolve" "-p" "rustc_passes" "-p" "rustc_ast_lowering" "-p" "rustc_query_impl" "-p" "rustc_traits" "-p" "rustc_codegen_llvm" "-p" "rustc_llvm" "-p" "rustc_builtin_macros" "-p" "rustc_symbol_mangling" "-p" "rustc_privacy" "-p" "rustc_incremental" "-p" "rustc_plugin_impl" "-p" "rustc_hir_pretty" "-p" "rustc_errors" "-p" "rustc_save_analysis" "-p" "rustc_ast_pretty" "-p" "rustc_codegen_ssa" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:02:18
