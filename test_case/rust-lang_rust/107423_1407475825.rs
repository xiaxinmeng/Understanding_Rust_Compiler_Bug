`
....
error[E0405]: cannot find trait `A` in this scope
 --> /home/matthias/vcs/github/glacier2/fixed/94629.rs:1:6
  |
1 | impl A for B {
  |      ^ not found in this scope

error[E0412]: cannot find type `B` in this scope
 --> /home/matthias/vcs/github/glacier2/fixed/94629.rs:1:12
  |
1 | impl A for B {
  |            ^ not found in this scope

error[E0658]: this is an internal attribute that will never be stable
 --> /home/matthias/vcs/github/glacier2/fixed/94629.rs:5:1
  |
5 | #[rustc_legacy_const_generics(2)]
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = help: add `#![feature(rustc_attrs)]` to the crate attributes to enable

thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `Some(SubstitutionPart { span: /home/matthias/vcs/github/glacier2/fixed/94629.rs:2:36: 2:36 (#0), snippet: "" })`,
 right: `None`: Span must not be empty and have no suggestion', /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_errors/src/diagnostic.rs:640:9
stack backtrace:
   0:     0x7fa524f746b1 - std::backtrace_rs::backtrace::libunwind::trace::h57c700f2e7064cc9
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7fa524f746b1 - std::backtrace_rs::backtrace::trace_unsynchronized::h3bd6f8115b3aef84
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7fa524f746b1 - std::sys_common::backtrace::_print_fmt::h4e3cd4a1386ea18b
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/sys_common/backtrace.rs:65:5
   3:     0x7fa524f746b1 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha4399f941a901f68
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/sys_common/backtrace.rs:44:22
   4:     0x7fa524ff2558 - core::fmt::write::h3693e9c93ce31d95
                               at /home/matthias/vcs/github/rust_debug_assertions/library/core/src/fmt/mod.rs:1232:17
   5:     0x7fa524f3455f - std::io::Write::write_fmt::h7cd08f6496a9e03d
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/io/mod.rs:1682:15
   6:     0x7fa524f744a5 - std::sys_common::backtrace::_print::h1ea264f371a6489b
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/sys_common/backtrace.rs:47:5
   7:     0x7fa524f744a5 - std::sys_common::backtrace::print::h43e663fef4567fcf
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/sys_common/backtrace.rs:34:9
   8:     0x7fa524f38f54 - std::panicking::default_hook::{{closure}}::h94496c5e3b8314dc
   9:     0x7fa524f38bc1 - std::panicking::default_hook::h62ce584520972394
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/panicking.rs:286:9
  10:     0x7fa527bf2932 - <alloc[88ef48cbf1494855]::boxed::Box<dyn for<'a, 'b> core[3a7799eb5b53fe42]::ops::function::Fn<(&'a core[3a7799eb5b53fe42]::panic::panic_info::PanicInfo<'b>,), Output = ()> + core[3a7799eb5b53fe42]::marker::Sync + core[3a7799eb5b53fe42]::marker::Send> as core[3a7799eb5b53fe42]::ops::function::Fn<(&core[3a7799eb5b53fe42]::panic::panic_info::PanicInfo,)>>::call
                               at /home/matthias/vcs/github/rust_debug_assertions/library/alloc/src/boxed.rs:2002:9
  11:     0x7fa527bf2932 - rustc_driver[aa86919b4ce94ddd]::DEFAULT_HOOK::{closure#0}::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_driver/src/lib.rs:1165:17
  12:     0x7fa524f3991f - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::h20060c80854d9f26
                               at /home/matthias/vcs/github/rust_debug_assertions/library/alloc/src/boxed.rs:2002:9
  13:     0x7fa524f3991f - std::panicking::rust_panic_with_hook::hf3310e5853e10814
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/panicking.rs:692:13
  14:     0x7fa524f75b39 - std::panicking::begin_panic_handler::{{closure}}::hb76bc4aa6a51e144
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/panicking.rs:579:13
  15:     0x7fa524f747fe - std::sys_common::backtrace::__rust_end_short_backtrace::h76df651326f0b61a
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/sys_common/backtrace.rs:137:18
  16:     0x7fa524f39432 - rust_begin_unwind
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/panicking.rs:575:5
  17:     0x7fa524ff6263 - core::panicking::panic_fmt::ha9472d71642ed08e
                               at /home/matthias/vcs/github/rust_debug_assertions/library/core/src/panicking.rs:64:14
  18:     0x7fa524ff6843 - core::panicking::assert_failed_inner::h08d1d4ab99760b32
  19:     0x7fa528a2ad69 - core[3a7799eb5b53fe42]::panicking::assert_failed::<core[3a7799eb5b53fe42]::option::Option<&rustc_errors[9e83615d379720b2]::SubstitutionPart>, core[3a7799eb5b53fe42]::option::Option<&rustc_errors[9e83615d379720b2]::SubstitutionPart>>
                               at /home/matthias/vcs/github/rust_debug_assertions/library/core/src/panicking.rs:212:5
  20:     0x7fa528a89422 - <rustc_errors[9e83615d379720b2]::diagnostic::Diagnostic>::multipart_suggestion_with_style::<rustc_error_messages[87ef96e77df7413c]::SubdiagnosticMessage>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_errors/src/diagnostic.rs:640:9
  21:     0x7fa5289c9da7 - <rustc_lint[967e662febf2ed6]::lints::UnusedDelimSuggestion as rustc_errors[9e83615d379720b2]::diagnostic::AddToDiagnostic>::add_to_diagnostic_with::<<rustc_lint[967e662febf2ed6]::lints::UnusedDelimSuggestion as rustc_errors[9e83615d379720b2]::diagnostic::AddToDiagnostic>::add_to_diagnostic::{closure#0}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/lints.rs:1456:10
  22:     0x7fa5289c9da7 - <rustc_lint[967e662febf2ed6]::lints::UnusedDelimSuggestion as rustc_errors[9e83615d379720b2]::diagnostic::AddToDiagnostic>::add_to_diagnostic
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_errors/src/diagnostic.rs:80:9
  23:     0x7fa5289b7b05 - <rustc_errors[9e83615d379720b2]::diagnostic::Diagnostic>::subdiagnostic::<rustc_lint[967e662febf2ed6]::lints::UnusedDelimSuggestion>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_errors/src/diagnostic.rs:916:9
  24:     0x7fa5289b7b05 - <rustc_errors[9e83615d379720b2]::diagnostic_builder::DiagnosticBuilder<()>>::subdiagnostic::<rustc_lint[967e662febf2ed6]::lints::UnusedDelimSuggestion>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_errors/src/diagnostic_builder.rs:428:13
  25:     0x7fa5289b7b05 - <rustc_lint[967e662febf2ed6]::lints::UnusedDelim as rustc_errors[9e83615d379720b2]::diagnostic::DecorateLint<()>>::decorate_lint
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/lints.rs:1447:10
  26:     0x7fa5289b7b05 - <rustc_lint[967e662febf2ed6]::context::EarlyContext as rustc_lint[967e662febf2ed6]::context::LintContext>::emit_spanned_lint::<rustc_error_messages[87ef96e77df7413c]::MultiSpan, rustc_lint[967e662febf2ed6]::lints::UnusedDelim>::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/context.rs:917:63
  27:     0x7fa5289b7b05 - <<rustc_lint[967e662febf2ed6]::context::EarlyContext as rustc_lint[967e662febf2ed6]::context::LintContext>::emit_spanned_lint<rustc_error_messages[87ef96e77df7413c]::MultiSpan, rustc_lint[967e662febf2ed6]::lints::UnusedDelim>::{closure#0} as core[3a7799eb5b53fe42]::ops::function::FnOnce<(&mut rustc_errors[9e83615d379720b2]::diagnostic_builder::DiagnosticBuilder<()>,)>>::call_once::{shim:vtable#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/library/core/src/ops/function.rs:250:5
  28:     0x7fa5289acb17 - <alloc[88ef48cbf1494855]::boxed::Box<dyn for<'a, 'b> core[3a7799eb5b53fe42]::ops::function::FnOnce<(&'a mut rustc_errors[9e83615d379720b2]::diagnostic_builder::DiagnosticBuilder<'b, ()>,), Output = &'a mut rustc_errors[9e83615d379720b2]::diagnostic_builder::DiagnosticBuilder<'b, ()>>> as core[3a7799eb5b53fe42]::ops::function::FnOnce<(&mut rustc_errors[9e83615d379720b2]::diagnostic_builder::DiagnosticBuilder<()>,)>>::call_once
                               at /home/matthias/vcs/github/rust_debug_assertions/library/alloc/src/boxed.rs:1988:9
  29:     0x7fa5289acb17 - rustc_middle[90e84a49c4f353b4]::lint::struct_lint_level::struct_lint_level_impl::<rustc_error_messages[87ef96e77df7413c]::DiagnosticMessage>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/lint.rs:457:9
  30:     0x7fa5289a898e - rustc_middle[90e84a49c4f353b4]::lint::struct_lint_level::<rustc_error_messages[87ef96e77df7413c]::DiagnosticMessage, <rustc_lint[967e662febf2ed6]::context::EarlyContext as rustc_lint[967e662febf2ed6]::context::LintContext>::emit_spanned_lint<rustc_error_messages[87ef96e77df7413c]::MultiSpan, rustc_lint[967e662febf2ed6]::lints::UnusedDelim>::{closure#0}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/lint.rs:461:5
  31:     0x7fa528a42807 - <rustc_lint[967e662febf2ed6]::levels::LintLevelsBuilder<rustc_lint[967e662febf2ed6]::levels::TopDown>>::struct_lint::<rustc_error_messages[87ef96e77df7413c]::DiagnosticMessage, <rustc_lint[967e662febf2ed6]::context::EarlyContext as rustc_lint[967e662febf2ed6]::context::LintContext>::emit_spanned_lint<rustc_error_messages[87ef96e77df7413c]::MultiSpan, rustc_lint[967e662febf2ed6]::lints::UnusedDelim>::{closure#0}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/levels.rs:1019:9
  32:     0x7fa52898e8d4 - <rustc_lint[967e662febf2ed6]::context::EarlyContext as rustc_lint[967e662febf2ed6]::context::LintContext>::lookup::<rustc_error_messages[87ef96e77df7413c]::MultiSpan, rustc_error_messages[87ef96e77df7413c]::DiagnosticMessage, <rustc_lint[967e662febf2ed6]::context::EarlyContext as rustc_lint[967e662febf2ed6]::context::LintContext>::emit_spanned_lint<rustc_error_messages[87ef96e77df7413c]::MultiSpan, rustc_lint[967e662febf2ed6]::lints::UnusedDelim>::{closure#0}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/context.rs:1065:9
  33:     0x7fa52898e8d4 - <rustc_lint[967e662febf2ed6]::context::EarlyContext as rustc_lint[967e662febf2ed6]::context::LintContext>::emit_spanned_lint::<rustc_error_messages[87ef96e77df7413c]::MultiSpan, rustc_lint[967e662febf2ed6]::lints::UnusedDelim>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/context.rs:917:9
  34:     0x7fa52898e8d4 - <rustc_lint[967e662febf2ed6]::unused::UnusedBraces as rustc_lint[967e662febf2ed6]::unused::UnusedDelimLint>::emit_unused_delims
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/unused.rs:692:9
  35:     0x7fa52898e8d4 - <rustc_lint[967e662febf2ed6]::unused::UnusedBraces as rustc_lint[967e662febf2ed6]::unused::UnusedDelimLint>::emit_unused_delims_expr
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/unused.rs:652:9
  36:     0x7fa5289911bd - <rustc_lint[967e662febf2ed6]::unused::UnusedBraces as rustc_lint[967e662febf2ed6]::unused::UnusedDelimLint>::check_unused_delims_expr
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/unused.rs:1148:29
  37:     0x7fa528a4963d - <rustc_lint[967e662febf2ed6]::unused::UnusedBraces as rustc_lint[967e662febf2ed6]::unused::UnusedDelimLint>::check_stmt
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/unused.rs:777:17
  38:     0x7fa528a4963d - <rustc_lint[967e662febf2ed6]::unused::UnusedBraces as rustc_lint[967e662febf2ed6]::passes::EarlyLintPass>::check_stmt
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/unused.rs:1170:9
  39:     0x7fa528a4963d - <rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass as rustc_lint[967e662febf2ed6]::passes::EarlyLintPass>::check_stmt
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/passes.rs:205:13
  40:     0x7fa528825887 - <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_stmt::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:146:13
  41:     0x7fa528825887 - <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>::with_lint_attrs::<<rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_stmt::{closure#0}>::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:76:36
  42:     0x7fa528825887 - stacker[12027ce31fbb92f0]::maybe_grow::<(), <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>::with_lint_attrs<<rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_stmt::{closure#0}>::{closure#0}>
                               at /home/matthias/.cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.15/src/lib.rs:55:9
  43:     0x7fa528825887 - rustc_data_structures[3154e4d7ef9f97f8]::stack::ensure_sufficient_stack::<(), <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>::with_lint_attrs<<rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_stmt::{closure#0}>::{closure#0}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_data_structures/src/stack.rs:17:5
  44:     0x7fa528825887 - <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>::with_lint_attrs::<<rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_stmt::{closure#0}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:76:9
  45:     0x7fa528825887 - <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_stmt
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:145:9
  46:     0x7fa528825d1c - rustc_ast[67da8ea2e7591207]::visit::walk_block::<rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_ast/src/visit.rs:709:5
  47:     0x7fa528825d1c - <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_block
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:210:9
  48:     0x7fa52895df28 - rustc_ast[67da8ea2e7591207]::visit::walk_expr::<rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>
  49:     0x7fa5288244cb - <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_expr::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:126:13
  50:     0x7fa5288244cb - <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>::with_lint_attrs::<<rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_expr::{closure#0}>::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:76:36
  51:     0x7fa5288244cb - stacker[12027ce31fbb92f0]::maybe_grow::<(), <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>::with_lint_attrs<<rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_expr::{closure#0}>::{closure#0}>
                               at /home/matthias/.cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.15/src/lib.rs:55:9
  52:     0x7fa5288244cb - rustc_data_structures[3154e4d7ef9f97f8]::stack::ensure_sufficient_stack::<(), <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>::with_lint_attrs<<rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_expr::{closure#0}>::{closure#0}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_data_structures/src/stack.rs:17:5
  53:     0x7fa5288244cb - <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>::with_lint_attrs::<<rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_expr::{closure#0}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:76:9
  54:     0x7fa5288244cb - <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_expr
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:124:9
  55:     0x7fa52895e141 - rustc_ast[67da8ea2e7591207]::visit::walk_expr::<rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>
  56:     0x7fa5288244cb - <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_expr::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:126:13
  57:     0x7fa5288244cb - <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>::with_lint_attrs::<<rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_expr::{closure#0}>::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:76:36
  58:     0x7fa5288244cb - stacker[12027ce31fbb92f0]::maybe_grow::<(), <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>::with_lint_attrs<<rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_expr::{closure#0}>::{closure#0}>
                               at /home/matthias/.cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.15/src/lib.rs:55:9
  59:     0x7fa5288244cb - rustc_data_structures[3154e4d7ef9f97f8]::stack::ensure_sufficient_stack::<(), <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>::with_lint_attrs<<rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_expr::{closure#0}>::{closure#0}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_data_structures/src/stack.rs:17:5
  60:     0x7fa5288244cb - <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>::with_lint_attrs::<<rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_expr::{closure#0}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:76:9
  61:     0x7fa5288244cb - <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_expr
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:124:9
  62:     0x7fa528825b1b - <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_stmt
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:153:9
  63:     0x7fa528825d1c - rustc_ast[67da8ea2e7591207]::visit::walk_block::<rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_ast/src/visit.rs:709:5
  64:     0x7fa528825d1c - <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_block
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:210:9
  65:     0x7fa52895df28 - rustc_ast[67da8ea2e7591207]::visit::walk_expr::<rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>
  66:     0x7fa5288244cb - <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_expr::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:126:13
  67:     0x7fa5288244cb - <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>::with_lint_attrs::<<rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_expr::{closure#0}>::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:76:36
  68:     0x7fa5288244cb - stacker[12027ce31fbb92f0]::maybe_grow::<(), <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>::with_lint_attrs<<rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_expr::{closure#0}>::{closure#0}>
                               at /home/matthias/.cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.15/src/lib.rs:55:9
  69:     0x7fa5288244cb - rustc_data_structures[3154e4d7ef9f97f8]::stack::ensure_sufficient_stack::<(), <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>::with_lint_attrs<<rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_expr::{closure#0}>::{closure#0}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_data_structures/src/stack.rs:17:5
  70:     0x7fa5288244cb - <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>::with_lint_attrs::<<rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_expr::{closure#0}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:76:9
  71:     0x7fa5288244cb - <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_expr
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:124:9
  72:     0x7fa528825b1b - <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_stmt
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:153:9
  73:     0x7fa528825d1c - rustc_ast[67da8ea2e7591207]::visit::walk_block::<rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_ast/src/visit.rs:709:5
  74:     0x7fa528825d1c - <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_block
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:210:9
  75:     0x7fa52895df28 - rustc_ast[67da8ea2e7591207]::visit::walk_expr::<rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>
  76:     0x7fa5288244cb - <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_expr::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:126:13
  77:     0x7fa5288244cb - <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>::with_lint_attrs::<<rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_expr::{closure#0}>::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:76:36
  78:     0x7fa5288244cb - stacker[12027ce31fbb92f0]::maybe_grow::<(), <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>::with_lint_attrs<<rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_expr::{closure#0}>::{closure#0}>
                               at /home/matthias/.cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.15/src/lib.rs:55:9
  79:     0x7fa5288244cb - rustc_data_structures[3154e4d7ef9f97f8]::stack::ensure_sufficient_stack::<(), <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>::with_lint_attrs<<rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_expr::{closure#0}>::{closure#0}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_data_structures/src/stack.rs:17:5
  80:     0x7fa5288244cb - <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>::with_lint_attrs::<<rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_expr::{closure#0}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:76:9
  81:     0x7fa5288244cb - <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_expr
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:124:9
  82:     0x7fa528825b1b - <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_stmt
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:153:9
  83:     0x7fa528825d1c - rustc_ast[67da8ea2e7591207]::visit::walk_block::<rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_ast/src/visit.rs:709:5
  84:     0x7fa528825d1c - <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_block
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:210:9
  85:     0x7fa52895df28 - rustc_ast[67da8ea2e7591207]::visit::walk_expr::<rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>
  86:     0x7fa5288244cb - <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_expr::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:126:13
  87:     0x7fa5288244cb - <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>::with_lint_attrs::<<rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_expr::{closure#0}>::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:76:36
  88:     0x7fa5288244cb - stacker[12027ce31fbb92f0]::maybe_grow::<(), <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>::with_lint_attrs<<rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_expr::{closure#0}>::{closure#0}>
                               at /home/matthias/.cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.15/src/lib.rs:55:9
  89:     0x7fa5288244cb - rustc_data_structures[3154e4d7ef9f97f8]::stack::ensure_sufficient_stack::<(), <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>::with_lint_attrs<<rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_expr::{closure#0}>::{closure#0}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_data_structures/src/stack.rs:17:5
  90:     0x7fa5288244cb - <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>::with_lint_attrs::<<rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_expr::{closure#0}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:76:9
  91:     0x7fa5288244cb - <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_expr
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:124:9
  92:     0x7fa528825b1b - <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_stmt
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:153:9
  93:     0x7fa528825d1c - rustc_ast[67da8ea2e7591207]::visit::walk_block::<rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_ast/src/visit.rs:709:5
  94:     0x7fa528825d1c - <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_block
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:210:9
  95:     0x7fa528955b43 - <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_fn
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:159:9
  96:     0x7fa528955b43 - rustc_ast[67da8ea2e7591207]::visit::walk_assoc_item::<rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_ast/src/visit.rs:682:13
  97:     0x7fa528829616 - <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_assoc_item::{closure#0}
  98:     0x7fa528829616 - <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>::with_lint_attrs::<<rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_assoc_item::{closure#0}>::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:76:36
  99:     0x7fa528829616 - stacker[12027ce31fbb92f0]::maybe_grow::<(), <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>::with_lint_attrs<<rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_assoc_item::{closure#0}>::{closure#0}>
                               at /home/matthias/.cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.15/src/lib.rs:55:9
 100:     0x7fa528829616 - rustc_data_structures[3154e4d7ef9f97f8]::stack::ensure_sufficient_stack::<(), <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>::with_lint_attrs<<rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_assoc_item::{closure#0}>::{closure#0}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_data_structures/src/stack.rs:17:5
 101:     0x7fa528829616 - <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>::with_lint_attrs::<<rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_assoc_item::{closure#0}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:76:9
 102:     0x7fa528829616 - <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_assoc_item
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:262:9
 103:     0x7fa52895fff1 - rustc_ast[67da8ea2e7591207]::visit::walk_item::<rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_ast/src/visit.rs:349:13
 104:     0x7fa528824e7b - <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_item::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:94:13
 105:     0x7fa528824e7b - <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>::with_lint_attrs::<<rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_item::{closure#0}>::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:76:36
 106:     0x7fa528824e7b - stacker[12027ce31fbb92f0]::maybe_grow::<(), <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>::with_lint_attrs<<rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_item::{closure#0}>::{closure#0}>
                               at /home/matthias/.cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.15/src/lib.rs:55:9
 107:     0x7fa528824e7b - rustc_data_structures[3154e4d7ef9f97f8]::stack::ensure_sufficient_stack::<(), <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>::with_lint_attrs<<rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_item::{closure#0}>::{closure#0}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_data_structures/src/stack.rs:17:5
 108:     0x7fa528824e7b - <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>::with_lint_attrs::<<rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_item::{closure#0}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:76:9
 109:     0x7fa528824e7b - <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_item
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:92:9
 110:     0x7fa528952fdc - rustc_ast[67da8ea2e7591207]::visit::walk_crate::<rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_ast/src/visit.rs:266:5
 111:     0x7fa5288151b0 - <&rustc_ast[67da8ea2e7591207]::ast::Crate as rustc_lint[967e662febf2ed6]::early::EarlyCheckNode>::check::<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:359:9
 112:     0x7fa5288151b0 - rustc_lint[967e662febf2ed6]::early::check_ast_node_inner::<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass, &rustc_ast[67da8ea2e7591207]::ast::Crate>::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:423:66
 113:     0x7fa5288151b0 - <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>::with_lint_attrs::<rustc_lint[967e662febf2ed6]::early::check_ast_node_inner<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass, &rustc_ast[67da8ea2e7591207]::ast::Crate>::{closure#0}>::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:76:36
 114:     0x7fa5288151b0 - stacker[12027ce31fbb92f0]::maybe_grow::<(), <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>::with_lint_attrs<rustc_lint[967e662febf2ed6]::early::check_ast_node_inner<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass, &rustc_ast[67da8ea2e7591207]::ast::Crate>::{closure#0}>::{closure#0}>
                               at /home/matthias/.cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.15/src/lib.rs:55:9
 115:     0x7fa5288151b0 - rustc_data_structures[3154e4d7ef9f97f8]::stack::ensure_sufficient_stack::<(), <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>::with_lint_attrs<rustc_lint[967e662febf2ed6]::early::check_ast_node_inner<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass, &rustc_ast[67da8ea2e7591207]::ast::Crate>::{closure#0}>::{closure#0}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_data_structures/src/stack.rs:17:5
 116:     0x7fa5288151b0 - <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>::with_lint_attrs::<rustc_lint[967e662febf2ed6]::early::check_ast_node_inner<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass, &rustc_ast[67da8ea2e7591207]::ast::Crate>::{closure#0}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:76:9
 117:     0x7fa5288151b0 - rustc_lint[967e662febf2ed6]::early::check_ast_node_inner::<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass, &rustc_ast[67da8ea2e7591207]::ast::Crate>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:423:5
 118:     0x7fa5288151b0 - rustc_lint[967e662febf2ed6]::early::check_ast_node::<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass, &rustc_ast[67da8ea2e7591207]::ast::Crate>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:406:9
 119:     0x7fa5288dbe3d - rustc_interface[df52044673a04e8b]::passes::configure_and_expand::{closure#8}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_interface/src/passes.rs:453:9
 120:     0x7fa5288dbe3d - <rustc_data_structures[3154e4d7ef9f97f8]::profiling::VerboseTimingGuard>::run::<(), rustc_interface[df52044673a04e8b]::passes::configure_and_expand::{closure#8}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_data_structures/src/profiling.rs:723:9
 121:     0x7fa5288dbe3d - <rustc_session[c28864ed605f8943]::session::Session>::time::<(), rustc_interface[df52044673a04e8b]::passes::configure_and_expand::{closure#8}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_session/src/utils.rs:10:50
 122:     0x7fa52886ff8f - rustc_interface[df52044673a04e8b]::passes::configure_and_expand
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_interface/src/passes.rs:451:5
 123:     0x7fa52888dcd8 - <rustc_interface[df52044673a04e8b]::queries::Queries>::expansion::{closure#0}::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_interface/src/queries.rs:188:17
 124:     0x7fa52888dcd8 - <rustc_interface[df52044673a04e8b]::passes::boxed_resolver::BoxedResolver>::access::<<rustc_interface[df52044673a04e8b]::queries::Queries>::expansion::{closure#0}::{closure#0}, core[3a7799eb5b53fe42]::result::Result<rustc_ast[67da8ea2e7591207]::ast::Crate, rustc_errors[9e83615d379720b2]::ErrorGuaranteed>>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_interface/src/passes.rs:136:13
 125:     0x7fa52888dcd8 - <rustc_interface[df52044673a04e8b]::queries::Queries>::expansion::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_interface/src/queries.rs:187:25
 126:     0x7fa52888dcd8 - <rustc_interface[df52044673a04e8b]::queries::Query<(alloc[88ef48cbf1494855]::rc::Rc<rustc_ast[67da8ea2e7591207]::ast::Crate>, alloc[88ef48cbf1494855]::rc::Rc<core[3a7799eb5b53fe42]::cell::RefCell<rustc_interface[df52044673a04e8b]::passes::boxed_resolver::BoxedResolver>>, alloc[88ef48cbf1494855]::rc::Rc<rustc_lint[967e662febf2ed6]::context::LintStore>)>>::compute::<<rustc_interface[df52044673a04e8b]::queries::Queries>::expansion::{closure#0}>::{closure#0}::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_interface/src/queries.rs:43:41
 127:     0x7fa52888dcd8 - <core[3a7799eb5b53fe42]::option::Option<core[3a7799eb5b53fe42]::result::Result<rustc_data_structures[3154e4d7ef9f97f8]::steal::Steal<(alloc[88ef48cbf1494855]::rc::Rc<rustc_ast[67da8ea2e7591207]::ast::Crate>, alloc[88ef48cbf1494855]::rc::Rc<core[3a7799eb5b53fe42]::cell::RefCell<rustc_interface[df52044673a04e8b]::passes::boxed_resolver::BoxedResolver>>, alloc[88ef48cbf1494855]::rc::Rc<rustc_lint[967e662febf2ed6]::context::LintStore>)>, rustc_errors[9e83615d379720b2]::ErrorGuaranteed>>>::get_or_insert_with::<<rustc_interface[df52044673a04e8b]::queries::Query<(alloc[88ef48cbf1494855]::rc::Rc<rustc_ast[67da8ea2e7591207]::ast::Crate>, alloc[88ef48cbf1494855]::rc::Rc<core[3a7799eb5b53fe42]::cell::RefCell<rustc_interface[df52044673a04e8b]::passes::boxed_resolver::BoxedResolver>>, alloc[88ef48cbf1494855]::rc::Rc<rustc_lint[967e662febf2ed6]::context::LintStore>)>>::compute<<rustc_interface[df52044673a04e8b]::queries::Queries>::expansion::{closure#0}>::{closure#0}::{closure#0}>
                               at /home/matthias/vcs/github/rust_debug_assertions/library/core/src/option.rs:1591:49
 128:     0x7fa52888dcd8 - <rustc_interface[df52044673a04e8b]::queries::Query<(alloc[88ef48cbf1494855]::rc::Rc<rustc_ast[67da8ea2e7591207]::ast::Crate>, alloc[88ef48cbf1494855]::rc::Rc<core[3a7799eb5b53fe42]::cell::RefCell<rustc_interface[df52044673a04e8b]::passes::boxed_resolver::BoxedResolver>>, alloc[88ef48cbf1494855]::rc::Rc<rustc_lint[967e662febf2ed6]::context::LintStore>)>>::compute::<<rustc_interface[df52044673a04e8b]::queries::Queries>::expansion::{closure#0}>::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_interface/src/queries.rs:43:17
 129:     0x7fa52888dcd8 - <core[3a7799eb5b53fe42]::cell::RefMut<core[3a7799eb5b53fe42]::option::Option<core[3a7799eb5b53fe42]::result::Result<rustc_data_structures[3154e4d7ef9f97f8]::steal::Steal<(alloc[88ef48cbf1494855]::rc::Rc<rustc_ast[67da8ea2e7591207]::ast::Crate>, alloc[88ef48cbf1494855]::rc::Rc<core[3a7799eb5b53fe42]::cell::RefCell<rustc_interface[df52044673a04e8b]::passes::boxed_resolver::BoxedResolver>>, alloc[88ef48cbf1494855]::rc::Rc<rustc_lint[967e662febf2ed6]::context::LintStore>)>, rustc_errors[9e83615d379720b2]::ErrorGuaranteed>>>>::filter_map::<rustc_data_structures[3154e4d7ef9f97f8]::steal::Steal<(alloc[88ef48cbf1494855]::rc::Rc<rustc_ast[67da8ea2e7591207]::ast::Crate>, alloc[88ef48cbf1494855]::rc::Rc<core[3a7799eb5b53fe42]::cell::RefCell<rustc_interface[df52044673a04e8b]::passes::boxed_resolver::BoxedResolver>>, alloc[88ef48cbf1494855]::rc::Rc<rustc_lint[967e662febf2ed6]::context::LintStore>)>, <rustc_interface[df52044673a04e8b]::queries::Query<(alloc[88ef48cbf1494855]::rc::Rc<rustc_ast[67da8ea2e7591207]::ast::Crate>, alloc[88ef48cbf1494855]::rc::Rc<core[3a7799eb5b53fe42]::cell::RefCell<rustc_interface[df52044673a04e8b]::passes::boxed_resolver::BoxedResolver>>, alloc[88ef48cbf1494855]::rc::Rc<rustc_lint[967e662febf2ed6]::context::LintStore>)>>::compute<<rustc_interface[df52044673a04e8b]::queries::Queries>::expansion::{closure#0}>::{closure#0}>
                               at /home/matthias/vcs/github/rust_debug_assertions/library/core/src/cell.rs:1588:15
 130:     0x7fa52888dcd8 - <rustc_interface[df52044673a04e8b]::queries::Query<(alloc[88ef48cbf1494855]::rc::Rc<rustc_ast[67da8ea2e7591207]::ast::Crate>, alloc[88ef48cbf1494855]::rc::Rc<core[3a7799eb5b53fe42]::cell::RefCell<rustc_interface[df52044673a04e8b]::passes::boxed_resolver::BoxedResolver>>, alloc[88ef48cbf1494855]::rc::Rc<rustc_lint[967e662febf2ed6]::context::LintStore>)>>::compute::<<rustc_interface[df52044673a04e8b]::queries::Queries>::expansion::{closure#0}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_interface/src/queries.rs:40:9
 131:     0x7fa52888dcd8 - <rustc_interface[df52044673a04e8b]::queries::Queries>::expansion
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_interface/src/queries.rs:176:9
 132:     0x7fa52888e36e - <rustc_interface[df52044673a04e8b]::queries::Queries>::global_ctxt::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_interface/src/queries.rs:213:49
 133:     0x7fa52888e36e - <rustc_interface[df52044673a04e8b]::queries::Query<rustc_interface[df52044673a04e8b]::passes::QueryContext>>::compute::<<rustc_interface[df52044673a04e8b]::queries::Queries>::global_ctxt::{closure#0}>::{closure#0}::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_interface/src/queries.rs:43:41
 134:     0x7fa52888e36e - <core[3a7799eb5b53fe42]::option::Option<core[3a7799eb5b53fe42]::result::Result<rustc_data_structures[3154e4d7ef9f97f8]::steal::Steal<rustc_interface[df52044673a04e8b]::passes::QueryContext>, rustc_errors[9e83615d379720b2]::ErrorGuaranteed>>>::get_or_insert_with::<<rustc_interface[df52044673a04e8b]::queries::Query<rustc_interface[df52044673a04e8b]::passes::QueryContext>>::compute<<rustc_interface[df52044673a04e8b]::queries::Queries>::global_ctxt::{closure#0}>::{closure#0}::{closure#0}>
                               at /home/matthias/vcs/github/rust_debug_assertions/library/core/src/option.rs:1591:49
 135:     0x7fa52888e36e - <rustc_interface[df52044673a04e8b]::queries::Query<rustc_interface[df52044673a04e8b]::passes::QueryContext>>::compute::<<rustc_interface[df52044673a04e8b]::queries::Queries>::global_ctxt::{closure#0}>::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_interface/src/queries.rs:43:17
 136:     0x7fa52888e36e - <core[3a7799eb5b53fe42]::cell::RefMut<core[3a7799eb5b53fe42]::option::Option<core[3a7799eb5b53fe42]::result::Result<rustc_data_structures[3154e4d7ef9f97f8]::steal::Steal<rustc_interface[df52044673a04e8b]::passes::QueryContext>, rustc_errors[9e83615d379720b2]::ErrorGuaranteed>>>>::filter_map::<rustc_data_structures[3154e4d7ef9f97f8]::steal::Steal<rustc_interface[df52044673a04e8b]::passes::QueryContext>, <rustc_interface[df52044673a04e8b]::queries::Query<rustc_interface[df52044673a04e8b]::passes::QueryContext>>::compute<<rustc_interface[df52044673a04e8b]::queries::Queries>::global_ctxt::{closure#0}>::{closure#0}>
                               at /home/matthias/vcs/github/rust_debug_assertions/library/core/src/cell.rs:1588:15
 137:     0x7fa52888e36e - <rustc_interface[df52044673a04e8b]::queries::Query<rustc_interface[df52044673a04e8b]::passes::QueryContext>>::compute::<<rustc_interface[df52044673a04e8b]::queries::Queries>::global_ctxt::{closure#0}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_interface/src/queries.rs:40:9
 138:     0x7fa52888e36e - <rustc_interface[df52044673a04e8b]::queries::Queries>::global_ctxt
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_interface/src/queries.rs:211:9
 139:     0x7fa527c89449 - rustc_driver[aa86919b4ce94ddd]::run_compiler::{closure#1}::{closure#2}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_driver/src/lib.rs:330:29
 140:     0x7fa527c89449 - <rustc_interface[df52044673a04e8b]::interface::Compiler>::enter::<rustc_driver[aa86919b4ce94ddd]::run_compiler::{closure#1}::{closure#2}, core[3a7799eb5b53fe42]::result::Result<core[3a7799eb5b53fe42]::option::Option<rustc_interface[df52044673a04e8b]::queries::Linker>, rustc_errors[9e83615d379720b2]::ErrorGuaranteed>>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_interface/src/queries.rs:386:19
 141:     0x7fa527bd85a5 - rustc_driver[aa86919b4ce94ddd]::run_compiler::{closure#1}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_driver/src/lib.rs:293:22
 142:     0x7fa527bd85a5 - rustc_interface[df52044673a04e8b]::interface::run_compiler::<core[3a7799eb5b53fe42]::result::Result<(), rustc_errors[9e83615d379720b2]::ErrorGuaranteed>, rustc_driver[aa86919b4ce94ddd]::run_compiler::{closure#1}>::{closure#0}::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_interface/src/interface.rs:300:21
 143:     0x7fa527bd85a5 - rustc_span[bb644a7833b5c27d]::with_source_map::<core[3a7799eb5b53fe42]::result::Result<(), rustc_errors[9e83615d379720b2]::ErrorGuaranteed>, rustc_interface[df52044673a04e8b]::interface::run_compiler<core[3a7799eb5b53fe42]::result::Result<(), rustc_errors[9e83615d379720b2]::ErrorGuaranteed>, rustc_driver[aa86919b4ce94ddd]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_span/src/lib.rs:1026:5
 144:     0x7fa527c77f69 - rustc_interface[df52044673a04e8b]::interface::run_compiler::<core[3a7799eb5b53fe42]::result::Result<(), rustc_errors[9e83615d379720b2]::ErrorGuaranteed>, rustc_driver[aa86919b4ce94ddd]::run_compiler::{closure#1}>::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_interface/src/interface.rs:294:13
 145:     0x7fa527c77f69 - <scoped_tls[3e8267cb6b5069b8]::ScopedKey<rustc_span[bb644a7833b5c27d]::SessionGlobals>>::set::<rustc_interface[df52044673a04e8b]::interface::run_compiler<core[3a7799eb5b53fe42]::result::Result<(), rustc_errors[9e83615d379720b2]::ErrorGuaranteed>, rustc_driver[aa86919b4ce94ddd]::run_compiler::{closure#1}>::{closure#0}, core[3a7799eb5b53fe42]::result::Result<(), rustc_errors[9e83615d379720b2]::ErrorGuaranteed>>
                               at /home/matthias/.cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137:9
 146:     0x7fa527c08e7f - rustc_span[bb644a7833b5c27d]::create_session_globals_then::<core[3a7799eb5b53fe42]::result::Result<(), rustc_errors[9e83615d379720b2]::ErrorGuaranteed>, rustc_interface[df52044673a04e8b]::interface::run_compiler<core[3a7799eb5b53fe42]::result::Result<(), rustc_errors[9e83615d379720b2]::ErrorGuaranteed>, rustc_driver[aa86919b4ce94ddd]::run_compiler::{closure#1}>::{closure#0}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_span/src/lib.rs:111:5
 147:     0x7fa527c08e7f - rustc_interface[df52044673a04e8b]::util::run_in_thread_pool_with_globals::<rustc_interface[df52044673a04e8b]::interface::run_compiler<core[3a7799eb5b53fe42]::result::Result<(), rustc_errors[9e83615d379720b2]::ErrorGuaranteed>, rustc_driver[aa86919b4ce94ddd]::run_compiler::{closure#1}>::{closure#0}, core[3a7799eb5b53fe42]::result::Result<(), rustc_errors[9e83615d379720b2]::ErrorGuaranteed>>::{closure#0}::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_interface/src/util.rs:146:38
 148:     0x7fa527c08e7f - std[a3e1a1d40e09fad4]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[df52044673a04e8b]::util::run_in_thread_pool_with_globals<rustc_interface[df52044673a04e8b]::interface::run_compiler<core[3a7799eb5b53fe42]::result::Result<(), rustc_errors[9e83615d379720b2]::ErrorGuaranteed>, rustc_driver[aa86919b4ce94ddd]::run_compiler::{closure#1}>::{closure#0}, core[3a7799eb5b53fe42]::result::Result<(), rustc_errors[9e83615d379720b2]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[3a7799eb5b53fe42]::result::Result<(), rustc_errors[9e83615d379720b2]::ErrorGuaranteed>>
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/sys_common/backtrace.rs:121:18
 149:     0x7fa527c01353 - <std[a3e1a1d40e09fad4]::thread::Builder>::spawn_unchecked_::<rustc_interface[df52044673a04e8b]::util::run_in_thread_pool_with_globals<rustc_interface[df52044673a04e8b]::interface::run_compiler<core[3a7799eb5b53fe42]::result::Result<(), rustc_errors[9e83615d379720b2]::ErrorGuaranteed>, rustc_driver[aa86919b4ce94ddd]::run_compiler::{closure#1}>::{closure#0}, core[3a7799eb5b53fe42]::result::Result<(), rustc_errors[9e83615d379720b2]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[3a7799eb5b53fe42]::result::Result<(), rustc_errors[9e83615d379720b2]::ErrorGuaranteed>>::{closure#1}::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/thread/mod.rs:558:17
 150:     0x7fa527c01353 - <core[3a7799eb5b53fe42]::panic::unwind_safe::AssertUnwindSafe<<std[a3e1a1d40e09fad4]::thread::Builder>::spawn_unchecked_<rustc_interface[df52044673a04e8b]::util::run_in_thread_pool_with_globals<rustc_interface[df52044673a04e8b]::interface::run_compiler<core[3a7799eb5b53fe42]::result::Result<(), rustc_errors[9e83615d379720b2]::ErrorGuaranteed>, rustc_driver[aa86919b4ce94ddd]::run_compiler::{closure#1}>::{closure#0}, core[3a7799eb5b53fe42]::result::Result<(), rustc_errors[9e83615d379720b2]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[3a7799eb5b53fe42]::result::Result<(), rustc_errors[9e83615d379720b2]::ErrorGuaranteed>>::{closure#1}::{closure#0}> as core[3a7799eb5b53fe42]::ops::function::FnOnce<()>>::call_once
                               at /home/matthias/vcs/github/rust_debug_assertions/library/core/src/panic/unwind_safe.rs:271:9
 151:     0x7fa527c01353 - std[a3e1a1d40e09fad4]::panicking::try::do_call::<core[3a7799eb5b53fe42]::panic::unwind_safe::AssertUnwindSafe<<std[a3e1a1d40e09fad4]::thread::Builder>::spawn_unchecked_<rustc_interface[df52044673a04e8b]::util::run_in_thread_pool_with_globals<rustc_interface[df52044673a04e8b]::interface::run_compiler<core[3a7799eb5b53fe42]::result::Result<(), rustc_errors[9e83615d379720b2]::ErrorGuaranteed>, rustc_driver[aa86919b4ce94ddd]::run_compiler::{closure#1}>::{closure#0}, core[3a7799eb5b53fe42]::result::Result<(), rustc_errors[9e83615d379720b2]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[3a7799eb5b53fe42]::result::Result<(), rustc_errors[9e83615d379720b2]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[3a7799eb5b53fe42]::result::Result<(), rustc_errors[9e83615d379720b2]::ErrorGuaranteed>>
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/panicking.rs:483:40
 152:     0x7fa527c01353 - std[a3e1a1d40e09fad4]::panicking::try::<core[3a7799eb5b53fe42]::result::Result<(), rustc_errors[9e83615d379720b2]::ErrorGuaranteed>, core[3a7799eb5b53fe42]::panic::unwind_safe::AssertUnwindSafe<<std[a3e1a1d40e09fad4]::thread::Builder>::spawn_unchecked_<rustc_interface[df52044673a04e8b]::util::run_in_thread_pool_with_globals<rustc_interface[df52044673a04e8b]::interface::run_compiler<core[3a7799eb5b53fe42]::result::Result<(), rustc_errors[9e83615d379720b2]::ErrorGuaranteed>, rustc_driver[aa86919b4ce94ddd]::run_compiler::{closure#1}>::{closure#0}, core[3a7799eb5b53fe42]::result::Result<(), rustc_errors[9e83615d379720b2]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[3a7799eb5b53fe42]::result::Result<(), rustc_errors[9e83615d379720b2]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/panicking.rs:447:19
 153:     0x7fa527c01353 - std[a3e1a1d40e09fad4]::panic::catch_unwind::<core[3a7799eb5b53fe42]::panic::unwind_safe::AssertUnwindSafe<<std[a3e1a1d40e09fad4]::thread::Builder>::spawn_unchecked_<rustc_interface[df52044673a04e8b]::util::run_in_thread_pool_with_globals<rustc_interface[df52044673a04e8b]::interface::run_compiler<core[3a7799eb5b53fe42]::result::Result<(), rustc_errors[9e83615d379720b2]::ErrorGuaranteed>, rustc_driver[aa86919b4ce94ddd]::run_compiler::{closure#1}>::{closure#0}, core[3a7799eb5b53fe42]::result::Result<(), rustc_errors[9e83615d379720b2]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[3a7799eb5b53fe42]::result::Result<(), rustc_errors[9e83615d379720b2]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[3a7799eb5b53fe42]::result::Result<(), rustc_errors[9e83615d379720b2]::ErrorGuaranteed>>
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/panic.rs:140:14
 154:     0x7fa527c01353 - <std[a3e1a1d40e09fad4]::thread::Builder>::spawn_unchecked_::<rustc_interface[df52044673a04e8b]::util::run_in_thread_pool_with_globals<rustc_interface[df52044673a04e8b]::interface::run_compiler<core[3a7799eb5b53fe42]::result::Result<(), rustc_errors[9e83615d379720b2]::ErrorGuaranteed>, rustc_driver[aa86919b4ce94ddd]::run_compiler::{closure#1}>::{closure#0}, core[3a7799eb5b53fe42]::result::Result<(), rustc_errors[9e83615d379720b2]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[3a7799eb5b53fe42]::result::Result<(), rustc_errors[9e83615d379720b2]::ErrorGuaranteed>>::{closure#1}
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/thread/mod.rs:557:30
 155:     0x7fa527c01353 - <<std[a3e1a1d40e09fad4]::thread::Builder>::spawn_unchecked_<rustc_interface[df52044673a04e8b]::util::run_in_thread_pool_with_globals<rustc_interface[df52044673a04e8b]::interface::run_compiler<core[3a7799eb5b53fe42]::result::Result<(), rustc_errors[9e83615d379720b2]::ErrorGuaranteed>, rustc_driver[aa86919b4ce94ddd]::run_compiler::{closure#1}>::{closure#0}, core[3a7799eb5b53fe42]::result::Result<(), rustc_errors[9e83615d379720b2]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[3a7799eb5b53fe42]::result::Result<(), rustc_errors[9e83615d379720b2]::ErrorGuaranteed>>::{closure#1} as core[3a7799eb5b53fe42]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/library/core/src/ops/function.rs:250:5
 156:     0x7fa524f24e58 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h1de6d10e235e916d
                               at /home/matthias/vcs/github/rust_debug_assertions/library/alloc/src/boxed.rs:1988:9
 157:     0x7fa524f24e58 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h7d484493755f58f7
                               at /home/matthias/vcs/github/rust_debug_assertions/library/alloc/src/boxed.rs:1988:9
 158:     0x7fa524f08505 - std::sys::unix::thread::Thread::new::thread_start::hc0cec0ea00f343dd
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/sys/unix/thread.rs:108:17
 159:     0x7fa524c9f8fd - <unknown>
 160:     0x7fa524d21d20 - <unknown>
 161:                0x0 - <unknown>

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.69.0-dev running on x86_64-unknown-linux-gnu

query stack during panic:
end of query stack
thread 'rustc' panicked at 'Found a `push` without a `pop`.', compiler/rustc_lint/src/levels.rs:509:9
stack backtrace:
   0:     0x7fa524f746b1 - std::backtrace_rs::backtrace::libunwind::trace::h57c700f2e7064cc9
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7fa524f746b1 - std::backtrace_rs::backtrace::trace_unsynchronized::h3bd6f8115b3aef84
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7fa524f746b1 - std::sys_common::backtrace::_print_fmt::h4e3cd4a1386ea18b
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/sys_common/backtrace.rs:65:5
   3:     0x7fa524f746b1 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha4399f941a901f68
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/sys_common/backtrace.rs:44:22
   4:     0x7fa524ff2558 - core::fmt::write::h3693e9c93ce31d95
                               at /home/matthias/vcs/github/rust_debug_assertions/library/core/src/fmt/mod.rs:1232:17
   5:     0x7fa524f3455f - std::io::Write::write_fmt::h7cd08f6496a9e03d
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/io/mod.rs:1682:15
   6:     0x7fa524f744a5 - std::sys_common::backtrace::_print::h1ea264f371a6489b
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/sys_common/backtrace.rs:47:5
   7:     0x7fa524f744a5 - std::sys_common::backtrace::print::h43e663fef4567fcf
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/sys_common/backtrace.rs:34:9
   8:     0x7fa524f38f54 - std::panicking::default_hook::{{closure}}::h94496c5e3b8314dc
   9:     0x7fa524f38bc1 - std::panicking::default_hook::h62ce584520972394
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/panicking.rs:286:9
  10:     0x7fa527bf2932 - <alloc[88ef48cbf1494855]::boxed::Box<dyn for<'a, 'b> core[3a7799eb5b53fe42]::ops::function::Fn<(&'a core[3a7799eb5b53fe42]::panic::panic_info::PanicInfo<'b>,), Output = ()> + core[3a7799eb5b53fe42]::marker::Sync + core[3a7799eb5b53fe42]::marker::Send> as core[3a7799eb5b53fe42]::ops::function::Fn<(&core[3a7799eb5b53fe42]::panic::panic_info::PanicInfo,)>>::call
                               at /home/matthias/vcs/github/rust_debug_assertions/library/alloc/src/boxed.rs:2002:9
  11:     0x7fa527bf2932 - rustc_driver[aa86919b4ce94ddd]::DEFAULT_HOOK::{closure#0}::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_driver/src/lib.rs:1165:17
  12:     0x7fa524f3991f - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::h20060c80854d9f26
                               at /home/matthias/vcs/github/rust_debug_assertions/library/alloc/src/boxed.rs:2002:9
  13:     0x7fa524f3991f - std::panicking::rust_panic_with_hook::hf3310e5853e10814
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/panicking.rs:692:13
  14:     0x7fa524f75af2 - std::panicking::begin_panic_handler::{{closure}}::hb76bc4aa6a51e144
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/panicking.rs:577:13
  15:     0x7fa524f747fe - std::sys_common::backtrace::__rust_end_short_backtrace::h76df651326f0b61a
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/sys_common/backtrace.rs:137:18
  16:     0x7fa524f39432 - rust_begin_unwind
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/panicking.rs:575:5
  17:     0x7fa524ff6263 - core::panicking::panic_fmt::ha9472d71642ed08e
                               at /home/matthias/vcs/github/rust_debug_assertions/library/core/src/panicking.rs:64:14
  18:     0x7fa528a3d6b9 - <rustc_lint[967e662febf2ed6]::levels::BuilderPush as core[3a7799eb5b53fe42]::ops::drop::Drop>::drop
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/levels.rs:509:9
  19:     0x7fa528814627 - core[3a7799eb5b53fe42]::ptr::drop_in_place::<rustc_lint[967e662febf2ed6]::levels::BuilderPush>
                               at /home/matthias/vcs/github/rust_debug_assertions/library/core/src/ptr/mod.rs:490:1
  20:     0x7fa528825c96 - <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>::with_lint_attrs::<<rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_stmt::{closure#0}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:80:5
  21:     0x7fa528825c96 - <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_stmt
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:145:9
  22:     0x7fa528825d1c - rustc_ast[67da8ea2e7591207]::visit::walk_block::<rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_ast/src/visit.rs:709:5
  23:     0x7fa528825d1c - <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_block
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:210:9
  24:     0x7fa52895df28 - rustc_ast[67da8ea2e7591207]::visit::walk_expr::<rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>
  25:     0x7fa5288244cb - <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_expr::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:126:13
  26:     0x7fa5288244cb - <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>::with_lint_attrs::<<rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_expr::{closure#0}>::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:76:36
  27:     0x7fa5288244cb - stacker[12027ce31fbb92f0]::maybe_grow::<(), <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>::with_lint_attrs<<rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_expr::{closure#0}>::{closure#0}>
                               at /home/matthias/.cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.15/src/lib.rs:55:9
  28:     0x7fa5288244cb - rustc_data_structures[3154e4d7ef9f97f8]::stack::ensure_sufficient_stack::<(), <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>::with_lint_attrs<<rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_expr::{closure#0}>::{closure#0}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_data_structures/src/stack.rs:17:5
  29:     0x7fa5288244cb - <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>::with_lint_attrs::<<rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_expr::{closure#0}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:76:9
  30:     0x7fa5288244cb - <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_expr
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:124:9
  31:     0x7fa52895e141 - rustc_ast[67da8ea2e7591207]::visit::walk_expr::<rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>
  32:     0x7fa5288244cb - <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_expr::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:126:13
  33:     0x7fa5288244cb - <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>::with_lint_attrs::<<rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_expr::{closure#0}>::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:76:36
  34:     0x7fa5288244cb - stacker[12027ce31fbb92f0]::maybe_grow::<(), <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>::with_lint_attrs<<rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_expr::{closure#0}>::{closure#0}>
                               at /home/matthias/.cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.15/src/lib.rs:55:9
  35:     0x7fa5288244cb - rustc_data_structures[3154e4d7ef9f97f8]::stack::ensure_sufficient_stack::<(), <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>::with_lint_attrs<<rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_expr::{closure#0}>::{closure#0}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_data_structures/src/stack.rs:17:5
  36:     0x7fa5288244cb - <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>::with_lint_attrs::<<rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_expr::{closure#0}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:76:9
  37:     0x7fa5288244cb - <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_expr
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:124:9
  38:     0x7fa528825b1b - <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_stmt
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:153:9
  39:     0x7fa528825d1c - rustc_ast[67da8ea2e7591207]::visit::walk_block::<rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_ast/src/visit.rs:709:5
  40:     0x7fa528825d1c - <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_block
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:210:9
  41:     0x7fa52895df28 - rustc_ast[67da8ea2e7591207]::visit::walk_expr::<rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>
  42:     0x7fa5288244cb - <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_expr::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:126:13
  43:     0x7fa5288244cb - <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>::with_lint_attrs::<<rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_expr::{closure#0}>::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:76:36
  44:     0x7fa5288244cb - stacker[12027ce31fbb92f0]::maybe_grow::<(), <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>::with_lint_attrs<<rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_expr::{closure#0}>::{closure#0}>
                               at /home/matthias/.cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.15/src/lib.rs:55:9
  45:     0x7fa5288244cb - rustc_data_structures[3154e4d7ef9f97f8]::stack::ensure_sufficient_stack::<(), <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>::with_lint_attrs<<rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_expr::{closure#0}>::{closure#0}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_data_structures/src/stack.rs:17:5
  46:     0x7fa5288244cb - <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>::with_lint_attrs::<<rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_expr::{closure#0}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:76:9
  47:     0x7fa5288244cb - <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_expr
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:124:9
  48:     0x7fa528825b1b - <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_stmt
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:153:9
  49:     0x7fa528825d1c - rustc_ast[67da8ea2e7591207]::visit::walk_block::<rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_ast/src/visit.rs:709:5
  50:     0x7fa528825d1c - <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_block
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:210:9
  51:     0x7fa52895df28 - rustc_ast[67da8ea2e7591207]::visit::walk_expr::<rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>
  52:     0x7fa5288244cb - <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_expr::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:126:13
  53:     0x7fa5288244cb - <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>::with_lint_attrs::<<rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_expr::{closure#0}>::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:76:36
  54:     0x7fa5288244cb - stacker[12027ce31fbb92f0]::maybe_grow::<(), <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>::with_lint_attrs<<rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_expr::{closure#0}>::{closure#0}>
                               at /home/matthias/.cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.15/src/lib.rs:55:9
  55:     0x7fa5288244cb - rustc_data_structures[3154e4d7ef9f97f8]::stack::ensure_sufficient_stack::<(), <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>::with_lint_attrs<<rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_expr::{closure#0}>::{closure#0}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_data_structures/src/stack.rs:17:5
  56:     0x7fa5288244cb - <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>::with_lint_attrs::<<rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_expr::{closure#0}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:76:9
  57:     0x7fa5288244cb - <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_expr
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:124:9
  58:     0x7fa528825b1b - <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_stmt
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:153:9
  59:     0x7fa528825d1c - rustc_ast[67da8ea2e7591207]::visit::walk_block::<rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_ast/src/visit.rs:709:5
  60:     0x7fa528825d1c - <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_block
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:210:9
  61:     0x7fa52895df28 - rustc_ast[67da8ea2e7591207]::visit::walk_expr::<rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>
  62:     0x7fa5288244cb - <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_expr::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:126:13
  63:     0x7fa5288244cb - <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>::with_lint_attrs::<<rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_expr::{closure#0}>::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:76:36
  64:     0x7fa5288244cb - stacker[12027ce31fbb92f0]::maybe_grow::<(), <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>::with_lint_attrs<<rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_expr::{closure#0}>::{closure#0}>
                               at /home/matthias/.cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.15/src/lib.rs:55:9
  65:     0x7fa5288244cb - rustc_data_structures[3154e4d7ef9f97f8]::stack::ensure_sufficient_stack::<(), <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>::with_lint_attrs<<rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_expr::{closure#0}>::{closure#0}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_data_structures/src/stack.rs:17:5
  66:     0x7fa5288244cb - <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>::with_lint_attrs::<<rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_expr::{closure#0}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:76:9
  67:     0x7fa5288244cb - <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_expr
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:124:9
  68:     0x7fa528825b1b - <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_stmt
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:153:9
  69:     0x7fa528825d1c - rustc_ast[67da8ea2e7591207]::visit::walk_block::<rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_ast/src/visit.rs:709:5
  70:     0x7fa528825d1c - <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_block
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:210:9
  71:     0x7fa528955b43 - <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_fn
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:159:9
  72:     0x7fa528955b43 - rustc_ast[67da8ea2e7591207]::visit::walk_assoc_item::<rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_ast/src/visit.rs:682:13
  73:     0x7fa528829616 - <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_assoc_item::{closure#0}
  74:     0x7fa528829616 - <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>::with_lint_attrs::<<rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_assoc_item::{closure#0}>::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:76:36
  75:     0x7fa528829616 - stacker[12027ce31fbb92f0]::maybe_grow::<(), <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>::with_lint_attrs<<rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_assoc_item::{closure#0}>::{closure#0}>
                               at /home/matthias/.cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.15/src/lib.rs:55:9
  76:     0x7fa528829616 - rustc_data_structures[3154e4d7ef9f97f8]::stack::ensure_sufficient_stack::<(), <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>::with_lint_attrs<<rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_assoc_item::{closure#0}>::{closure#0}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_data_structures/src/stack.rs:17:5
  77:     0x7fa528829616 - <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>::with_lint_attrs::<<rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_assoc_item::{closure#0}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:76:9
  78:     0x7fa528829616 - <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_assoc_item
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:262:9
  79:     0x7fa52895fff1 - rustc_ast[67da8ea2e7591207]::visit::walk_item::<rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_ast/src/visit.rs:349:13
  80:     0x7fa528824e7b - <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_item::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:94:13
  81:     0x7fa528824e7b - <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>::with_lint_attrs::<<rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_item::{closure#0}>::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:76:36
  82:     0x7fa528824e7b - stacker[12027ce31fbb92f0]::maybe_grow::<(), <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>::with_lint_attrs<<rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_item::{closure#0}>::{closure#0}>
                               at /home/matthias/.cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.15/src/lib.rs:55:9
  83:     0x7fa528824e7b - rustc_data_structures[3154e4d7ef9f97f8]::stack::ensure_sufficient_stack::<(), <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>::with_lint_attrs<<rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_item::{closure#0}>::{closure#0}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_data_structures/src/stack.rs:17:5
  84:     0x7fa528824e7b - <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>::with_lint_attrs::<<rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_item::{closure#0}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:76:9
  85:     0x7fa528824e7b - <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass> as rustc_ast[67da8ea2e7591207]::visit::Visitor>::visit_item
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:92:9
  86:     0x7fa528952fdc - rustc_ast[67da8ea2e7591207]::visit::walk_crate::<rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_ast/src/visit.rs:266:5
  87:     0x7fa5288151b0 - <&rustc_ast[67da8ea2e7591207]::ast::Crate as rustc_lint[967e662febf2ed6]::early::EarlyCheckNode>::check::<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:359:9
  88:     0x7fa5288151b0 - rustc_lint[967e662febf2ed6]::early::check_ast_node_inner::<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass, &rustc_ast[67da8ea2e7591207]::ast::Crate>::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:423:66
  89:     0x7fa5288151b0 - <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>::with_lint_attrs::<rustc_lint[967e662febf2ed6]::early::check_ast_node_inner<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass, &rustc_ast[67da8ea2e7591207]::ast::Crate>::{closure#0}>::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:76:36
  90:     0x7fa5288151b0 - stacker[12027ce31fbb92f0]::maybe_grow::<(), <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>::with_lint_attrs<rustc_lint[967e662febf2ed6]::early::check_ast_node_inner<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass, &rustc_ast[67da8ea2e7591207]::ast::Crate>::{closure#0}>::{closure#0}>
                               at /home/matthias/.cargo/registry/src/github.com-1ecc6299db9ec823/stacker-0.1.15/src/lib.rs:55:9
  91:     0x7fa5288151b0 - rustc_data_structures[3154e4d7ef9f97f8]::stack::ensure_sufficient_stack::<(), <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>::with_lint_attrs<rustc_lint[967e662febf2ed6]::early::check_ast_node_inner<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass, &rustc_ast[67da8ea2e7591207]::ast::Crate>::{closure#0}>::{closure#0}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_data_structures/src/stack.rs:17:5
  92:     0x7fa5288151b0 - <rustc_lint[967e662febf2ed6]::early::EarlyContextAndPass<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass>>::with_lint_attrs::<rustc_lint[967e662febf2ed6]::early::check_ast_node_inner<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass, &rustc_ast[67da8ea2e7591207]::ast::Crate>::{closure#0}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:76:9
  93:     0x7fa5288151b0 - rustc_lint[967e662febf2ed6]::early::check_ast_node_inner::<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass, &rustc_ast[67da8ea2e7591207]::ast::Crate>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:423:5
  94:     0x7fa5288151b0 - rustc_lint[967e662febf2ed6]::early::check_ast_node::<rustc_lint[967e662febf2ed6]::BuiltinCombinedEarlyLintPass, &rustc_ast[67da8ea2e7591207]::ast::Crate>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:406:9
  95:     0x7fa5288dbe3d - rustc_interface[df52044673a04e8b]::passes::configure_and_expand::{closure#8}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_interface/src/passes.rs:453:9
  96:     0x7fa5288dbe3d - <rustc_data_structures[3154e4d7ef9f97f8]::profiling::VerboseTimingGuard>::run::<(), rustc_interface[df52044673a04e8b]::passes::configure_and_expand::{closure#8}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_data_structures/src/profiling.rs:723:9
  97:     0x7fa5288dbe3d - <rustc_session[c28864ed605f8943]::session::Session>::time::<(), rustc_interface[df52044673a04e8b]::passes::configure_and_expand::{closure#8}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_session/src/utils.rs:10:50
  98:     0x7fa52886ff8f - rustc_interface[df52044673a04e8b]::passes::configure_and_expand
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_interface/src/passes.rs:451:5
  99:     0x7fa52888dcd8 - <rustc_interface[df52044673a04e8b]::queries::Queries>::expansion::{closure#0}::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_interface/src/queries.rs:188:17
 100:     0x7fa52888dcd8 - <rustc_interface[df52044673a04e8b]::passes::boxed_resolver::BoxedResolver>::access::<<rustc_interface[df52044673a04e8b]::queries::Queries>::expansion::{closure#0}::{closure#0}, core[3a7799eb5b53fe42]::result::Result<rustc_ast[67da8ea2e7591207]::ast::Crate, rustc_errors[9e83615d379720b2]::ErrorGuaranteed>>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_interface/src/passes.rs:136:13
 101:     0x7fa52888dcd8 - <rustc_interface[df52044673a04e8b]::queries::Queries>::expansion::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_interface/src/queries.rs:187:25
 102:     0x7fa52888dcd8 - <rustc_interface[df52044673a04e8b]::queries::Query<(alloc[88ef48cbf1494855]::rc::Rc<rustc_ast[67da8ea2e7591207]::ast::Crate>, alloc[88ef48cbf1494855]::rc::Rc<core[3a7799eb5b53fe42]::cell::RefCell<rustc_interface[df52044673a04e8b]::passes::boxed_resolver::BoxedResolver>>, alloc[88ef48cbf1494855]::rc::Rc<rustc_lint[967e662febf2ed6]::context::LintStore>)>>::compute::<<rustc_interface[df52044673a04e8b]::queries::Queries>::expansion::{closure#0}>::{closure#0}::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_interface/src/queries.rs:43:41
 103:     0x7fa52888dcd8 - <core[3a7799eb5b53fe42]::option::Option<core[3a7799eb5b53fe42]::result::Result<rustc_data_structures[3154e4d7ef9f97f8]::steal::Steal<(alloc[88ef48cbf1494855]::rc::Rc<rustc_ast[67da8ea2e7591207]::ast::Crate>, alloc[88ef48cbf1494855]::rc::Rc<core[3a7799eb5b53fe42]::cell::RefCell<rustc_interface[df52044673a04e8b]::passes::boxed_resolver::BoxedResolver>>, alloc[88ef48cbf1494855]::rc::Rc<rustc_lint[967e662febf2ed6]::context::LintStore>)>, rustc_errors[9e83615d379720b2]::ErrorGuaranteed>>>::get_or_insert_with::<<rustc_interface[df52044673a04e8b]::queries::Query<(alloc[88ef48cbf1494855]::rc::Rc<rustc_ast[67da8ea2e7591207]::ast::Crate>, alloc[88ef48cbf1494855]::rc::Rc<core[3a7799eb5b53fe42]::cell::RefCell<rustc_interface[df52044673a04e8b]::passes::boxed_resolver::BoxedResolver>>, alloc[88ef48cbf1494855]::rc::Rc<rustc_lint[967e662febf2ed6]::context::LintStore>)>>::compute<<rustc_interface[df52044673a04e8b]::queries::Queries>::expansion::{closure#0}>::{closure#0}::{closure#0}>
                               at /home/matthias/vcs/github/rust_debug_assertions/library/core/src/option.rs:1591:49
 104:     0x7fa52888dcd8 - <rustc_interface[df52044673a04e8b]::queries::Query<(alloc[88ef48cbf1494855]::rc::Rc<rustc_ast[67da8ea2e7591207]::ast::Crate>, alloc[88ef48cbf1494855]::rc::Rc<core[3a7799eb5b53fe42]::cell::RefCell<rustc_interface[df52044673a04e8b]::passes::boxed_resolver::BoxedResolver>>, alloc[88ef48cbf1494855]::rc::Rc<rustc_lint[967e662febf2ed6]::context::LintStore>)>>::compute::<<rustc_interface[df52044673a04e8b]::queries::Queries>::expansion::{closure#0}>::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_interface/src/queries.rs:43:17
 105:     0x7fa52888dcd8 - <core[3a7799eb5b53fe42]::cell::RefMut<core[3a7799eb5b53fe42]::option::Option<core[3a7799eb5b53fe42]::result::Result<rustc_data_structures[3154e4d7ef9f97f8]::steal::Steal<(alloc[88ef48cbf1494855]::rc::Rc<rustc_ast[67da8ea2e7591207]::ast::Crate>, alloc[88ef48cbf1494855]::rc::Rc<core[3a7799eb5b53fe42]::cell::RefCell<rustc_interface[df52044673a04e8b]::passes::boxed_resolver::BoxedResolver>>, alloc[88ef48cbf1494855]::rc::Rc<rustc_lint[967e662febf2ed6]::context::LintStore>)>, rustc_errors[9e83615d379720b2]::ErrorGuaranteed>>>>::filter_map::<rustc_data_structures[3154e4d7ef9f97f8]::steal::Steal<(alloc[88ef48cbf1494855]::rc::Rc<rustc_ast[67da8ea2e7591207]::ast::Crate>, alloc[88ef48cbf1494855]::rc::Rc<core[3a7799eb5b53fe42]::cell::RefCell<rustc_interface[df52044673a04e8b]::passes::boxed_resolver::BoxedResolver>>, alloc[88ef48cbf1494855]::rc::Rc<rustc_lint[967e662febf2ed6]::context::LintStore>)>, <rustc_interface[df52044673a04e8b]::queries::Query<(alloc[88ef48cbf1494855]::rc::Rc<rustc_ast[67da8ea2e7591207]::ast::Crate>, alloc[88ef48cbf1494855]::rc::Rc<core[3a7799eb5b53fe42]::cell::RefCell<rustc_interface[df52044673a04e8b]::passes::boxed_resolver::BoxedResolver>>, alloc[88ef48cbf1494855]::rc::Rc<rustc_lint[967e662febf2ed6]::context::LintStore>)>>::compute<<rustc_interface[df52044673a04e8b]::queries::Queries>::expansion::{closure#0}>::{closure#0}>
                               at /home/matthias/vcs/github/rust_debug_assertions/library/core/src/cell.rs:1588:15
 106:     0x7fa52888dcd8 - <rustc_interface[df52044673a04e8b]::queries::Query<(alloc[88ef48cbf1494855]::rc::Rc<rustc_ast[67da8ea2e7591207]::ast::Crate>, alloc[88ef48cbf1494855]::rc::Rc<core[3a7799eb5b53fe42]::cell::RefCell<rustc_interface[df52044673a04e8b]::passes::boxed_resolver::BoxedResolver>>, alloc[88ef48cbf1494855]::rc::Rc<rustc_lint[967e662febf2ed6]::context::LintStore>)>>::compute::<<rustc_interface[df52044673a04e8b]::queries::Queries>::expansion::{closure#0}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_interface/src/queries.rs:40:9
 107:     0x7fa52888dcd8 - <rustc_interface[df52044673a04e8b]::queries::Queries>::expansion
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_interface/src/queries.rs:176:9
 108:     0x7fa52888e36e - <rustc_interface[df52044673a04e8b]::queries::Queries>::global_ctxt::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_interface/src/queries.rs:213:49
 109:     0x7fa52888e36e - <rustc_interface[df52044673a04e8b]::queries::Query<rustc_interface[df52044673a04e8b]::passes::QueryContext>>::compute::<<rustc_interface[df52044673a04e8b]::queries::Queries>::global_ctxt::{closure#0}>::{closure#0}::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_interface/src/queries.rs:43:41
 110:     0x7fa52888e36e - <core[3a7799eb5b53fe42]::option::Option<core[3a7799eb5b53fe42]::result::Result<rustc_data_structures[3154e4d7ef9f97f8]::steal::Steal<rustc_interface[df52044673a04e8b]::passes::QueryContext>, rustc_errors[9e83615d379720b2]::ErrorGuaranteed>>>::get_or_insert_with::<<rustc_interface[df52044673a04e8b]::queries::Query<rustc_interface[df52044673a04e8b]::passes::QueryContext>>::compute<<rustc_interface[df52044673a04e8b]::queries::Queries>::global_ctxt::{closure#0}>::{closure#0}::{closure#0}>
                               at /home/matthias/vcs/github/rust_debug_assertions/library/core/src/option.rs:1591:49
 111:     0x7fa52888e36e - <rustc_interface[df52044673a04e8b]::queries::Query<rustc_interface[df52044673a04e8b]::passes::QueryContext>>::compute::<<rustc_interface[df52044673a04e8b]::queries::Queries>::global_ctxt::{closure#0}>::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_interface/src/queries.rs:43:17
 112:     0x7fa52888e36e - <core[3a7799eb5b53fe42]::cell::RefMut<core[3a7799eb5b53fe42]::option::Option<core[3a7799eb5b53fe42]::result::Result<rustc_data_structures[3154e4d7ef9f97f8]::steal::Steal<rustc_interface[df52044673a04e8b]::passes::QueryContext>, rustc_errors[9e83615d379720b2]::ErrorGuaranteed>>>>::filter_map::<rustc_data_structures[3154e4d7ef9f97f8]::steal::Steal<rustc_interface[df52044673a04e8b]::passes::QueryContext>, <rustc_interface[df52044673a04e8b]::queries::Query<rustc_interface[df52044673a04e8b]::passes::QueryContext>>::compute<<rustc_interface[df52044673a04e8b]::queries::Queries>::global_ctxt::{closure#0}>::{closure#0}>
                               at /home/matthias/vcs/github/rust_debug_assertions/library/core/src/cell.rs:1588:15
 113:     0x7fa52888e36e - <rustc_interface[df52044673a04e8b]::queries::Query<rustc_interface[df52044673a04e8b]::passes::QueryContext>>::compute::<<rustc_interface[df52044673a04e8b]::queries::Queries>::global_ctxt::{closure#0}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_interface/src/queries.rs:40:9
 114:     0x7fa52888e36e - <rustc_interface[df52044673a04e8b]::queries::Queries>::global_ctxt
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_interface/src/queries.rs:211:9
 115:     0x7fa527c89449 - rustc_driver[aa86919b4ce94ddd]::run_compiler::{closure#1}::{closure#2}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_driver/src/lib.rs:330:29
 116:     0x7fa527c89449 - <rustc_interface[df52044673a04e8b]::interface::Compiler>::enter::<rustc_driver[aa86919b4ce94ddd]::run_compiler::{closure#1}::{closure#2}, core[3a7799eb5b53fe42]::result::Result<core[3a7799eb5b53fe42]::option::Option<rustc_interface[df52044673a04e8b]::queries::Linker>, rustc_errors[9e83615d379720b2]::ErrorGuaranteed>>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_interface/src/queries.rs:386:19
 117:     0x7fa527bd85a5 - rustc_driver[aa86919b4ce94ddd]::run_compiler::{closure#1}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_driver/src/lib.rs:293:22
 118:     0x7fa527bd85a5 - rustc_interface[df52044673a04e8b]::interface::run_compiler::<core[3a7799eb5b53fe42]::result::Result<(), rustc_errors[9e83615d379720b2]::ErrorGuaranteed>, rustc_driver[aa86919b4ce94ddd]::run_compiler::{closure#1}>::{closure#0}::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_interface/src/interface.rs:300:21
 119:     0x7fa527bd85a5 - rustc_span[bb644a7833b5c27d]::with_source_map::<core[3a7799eb5b53fe42]::result::Result<(), rustc_errors[9e83615d379720b2]::ErrorGuaranteed>, rustc_interface[df52044673a04e8b]::interface::run_compiler<core[3a7799eb5b53fe42]::result::Result<(), rustc_errors[9e83615d379720b2]::ErrorGuaranteed>, rustc_driver[aa86919b4ce94ddd]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_span/src/lib.rs:1026:5
 120:     0x7fa527c77f69 - rustc_interface[df52044673a04e8b]::interface::run_compiler::<core[3a7799eb5b53fe42]::result::Result<(), rustc_errors[9e83615d379720b2]::ErrorGuaranteed>, rustc_driver[aa86919b4ce94ddd]::run_compiler::{closure#1}>::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_interface/src/interface.rs:294:13
 121:     0x7fa527c77f69 - <scoped_tls[3e8267cb6b5069b8]::ScopedKey<rustc_span[bb644a7833b5c27d]::SessionGlobals>>::set::<rustc_interface[df52044673a04e8b]::interface::run_compiler<core[3a7799eb5b53fe42]::result::Result<(), rustc_errors[9e83615d379720b2]::ErrorGuaranteed>, rustc_driver[aa86919b4ce94ddd]::run_compiler::{closure#1}>::{closure#0}, core[3a7799eb5b53fe42]::result::Result<(), rustc_errors[9e83615d379720b2]::ErrorGuaranteed>>
                               at /home/matthias/.cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137:9
 122:     0x7fa527c08e7f - rustc_span[bb644a7833b5c27d]::create_session_globals_then::<core[3a7799eb5b53fe42]::result::Result<(), rustc_errors[9e83615d379720b2]::ErrorGuaranteed>, rustc_interface[df52044673a04e8b]::interface::run_compiler<core[3a7799eb5b53fe42]::result::Result<(), rustc_errors[9e83615d379720b2]::ErrorGuaranteed>, rustc_driver[aa86919b4ce94ddd]::run_compiler::{closure#1}>::{closure#0}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_span/src/lib.rs:111:5
 123:     0x7fa527c08e7f - rustc_interface[df52044673a04e8b]::util::run_in_thread_pool_with_globals::<rustc_interface[df52044673a04e8b]::interface::run_compiler<core[3a7799eb5b53fe42]::result::Result<(), rustc_errors[9e83615d379720b2]::ErrorGuaranteed>, rustc_driver[aa86919b4ce94ddd]::run_compiler::{closure#1}>::{closure#0}, core[3a7799eb5b53fe42]::result::Result<(), rustc_errors[9e83615d379720b2]::ErrorGuaranteed>>::{closure#0}::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_interface/src/util.rs:146:38
 124:     0x7fa527c08e7f - std[a3e1a1d40e09fad4]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[df52044673a04e8b]::util::run_in_thread_pool_with_globals<rustc_interface[df52044673a04e8b]::interface::run_compiler<core[3a7799eb5b53fe42]::result::Result<(), rustc_errors[9e83615d379720b2]::ErrorGuaranteed>, rustc_driver[aa86919b4ce94ddd]::run_compiler::{closure#1}>::{closure#0}, core[3a7799eb5b53fe42]::result::Result<(), rustc_errors[9e83615d379720b2]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[3a7799eb5b53fe42]::result::Result<(), rustc_errors[9e83615d379720b2]::ErrorGuaranteed>>
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/sys_common/backtrace.rs:121:18
 125:     0x7fa527c01353 - <std[a3e1a1d40e09fad4]::thread::Builder>::spawn_unchecked_::<rustc_interface[df52044673a04e8b]::util::run_in_thread_pool_with_globals<rustc_interface[df52044673a04e8b]::interface::run_compiler<core[3a7799eb5b53fe42]::result::Result<(), rustc_errors[9e83615d379720b2]::ErrorGuaranteed>, rustc_driver[aa86919b4ce94ddd]::run_compiler::{closure#1}>::{closure#0}, core[3a7799eb5b53fe42]::result::Result<(), rustc_errors[9e83615d379720b2]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[3a7799eb5b53fe42]::result::Result<(), rustc_errors[9e83615d379720b2]::ErrorGuaranteed>>::{closure#1}::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/thread/mod.rs:558:17
 126:     0x7fa527c01353 - <core[3a7799eb5b53fe42]::panic::unwind_safe::AssertUnwindSafe<<std[a3e1a1d40e09fad4]::thread::Builder>::spawn_unchecked_<rustc_interface[df52044673a04e8b]::util::run_in_thread_pool_with_globals<rustc_interface[df52044673a04e8b]::interface::run_compiler<core[3a7799eb5b53fe42]::result::Result<(), rustc_errors[9e83615d379720b2]::ErrorGuaranteed>, rustc_driver[aa86919b4ce94ddd]::run_compiler::{closure#1}>::{closure#0}, core[3a7799eb5b53fe42]::result::Result<(), rustc_errors[9e83615d379720b2]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[3a7799eb5b53fe42]::result::Result<(), rustc_errors[9e83615d379720b2]::ErrorGuaranteed>>::{closure#1}::{closure#0}> as core[3a7799eb5b53fe42]::ops::function::FnOnce<()>>::call_once
                               at /home/matthias/vcs/github/rust_debug_assertions/library/core/src/panic/unwind_safe.rs:271:9
 127:     0x7fa527c01353 - std[a3e1a1d40e09fad4]::panicking::try::do_call::<core[3a7799eb5b53fe42]::panic::unwind_safe::AssertUnwindSafe<<std[a3e1a1d40e09fad4]::thread::Builder>::spawn_unchecked_<rustc_interface[df52044673a04e8b]::util::run_in_thread_pool_with_globals<rustc_interface[df52044673a04e8b]::interface::run_compiler<core[3a7799eb5b53fe42]::result::Result<(), rustc_errors[9e83615d379720b2]::ErrorGuaranteed>, rustc_driver[aa86919b4ce94ddd]::run_compiler::{closure#1}>::{closure#0}, core[3a7799eb5b53fe42]::result::Result<(), rustc_errors[9e83615d379720b2]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[3a7799eb5b53fe42]::result::Result<(), rustc_errors[9e83615d379720b2]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[3a7799eb5b53fe42]::result::Result<(), rustc_errors[9e83615d379720b2]::ErrorGuaranteed>>
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/panicking.rs:483:40
 128:     0x7fa527c01353 - std[a3e1a1d40e09fad4]::panicking::try::<core[3a7799eb5b53fe42]::result::Result<(), rustc_errors[9e83615d379720b2]::ErrorGuaranteed>, core[3a7799eb5b53fe42]::panic::unwind_safe::AssertUnwindSafe<<std[a3e1a1d40e09fad4]::thread::Builder>::spawn_unchecked_<rustc_interface[df52044673a04e8b]::util::run_in_thread_pool_with_globals<rustc_interface[df52044673a04e8b]::interface::run_compiler<core[3a7799eb5b53fe42]::result::Result<(), rustc_errors[9e83615d379720b2]::ErrorGuaranteed>, rustc_driver[aa86919b4ce94ddd]::run_compiler::{closure#1}>::{closure#0}, core[3a7799eb5b53fe42]::result::Result<(), rustc_errors[9e83615d379720b2]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[3a7799eb5b53fe42]::result::Result<(), rustc_errors[9e83615d379720b2]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/panicking.rs:447:19
 129:     0x7fa527c01353 - std[a3e1a1d40e09fad4]::panic::catch_unwind::<core[3a7799eb5b53fe42]::panic::unwind_safe::AssertUnwindSafe<<std[a3e1a1d40e09fad4]::thread::Builder>::spawn_unchecked_<rustc_interface[df52044673a04e8b]::util::run_in_thread_pool_with_globals<rustc_interface[df52044673a04e8b]::interface::run_compiler<core[3a7799eb5b53fe42]::result::Result<(), rustc_errors[9e83615d379720b2]::ErrorGuaranteed>, rustc_driver[aa86919b4ce94ddd]::run_compiler::{closure#1}>::{closure#0}, core[3a7799eb5b53fe42]::result::Result<(), rustc_errors[9e83615d379720b2]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[3a7799eb5b53fe42]::result::Result<(), rustc_errors[9e83615d379720b2]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[3a7799eb5b53fe42]::result::Result<(), rustc_errors[9e83615d379720b2]::ErrorGuaranteed>>
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/panic.rs:140:14
 130:     0x7fa527c01353 - <std[a3e1a1d40e09fad4]::thread::Builder>::spawn_unchecked_::<rustc_interface[df52044673a04e8b]::util::run_in_thread_pool_with_globals<rustc_interface[df52044673a04e8b]::interface::run_compiler<core[3a7799eb5b53fe42]::result::Result<(), rustc_errors[9e83615d379720b2]::ErrorGuaranteed>, rustc_driver[aa86919b4ce94ddd]::run_compiler::{closure#1}>::{closure#0}, core[3a7799eb5b53fe42]::result::Result<(), rustc_errors[9e83615d379720b2]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[3a7799eb5b53fe42]::result::Result<(), rustc_errors[9e83615d379720b2]::ErrorGuaranteed>>::{closure#1}
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/thread/mod.rs:557:30
 131:     0x7fa527c01353 - <<std[a3e1a1d40e09fad4]::thread::Builder>::spawn_unchecked_<rustc_interface[df52044673a04e8b]::util::run_in_thread_pool_with_globals<rustc_interface[df52044673a04e8b]::interface::run_compiler<core[3a7799eb5b53fe42]::result::Result<(), rustc_errors[9e83615d379720b2]::ErrorGuaranteed>, rustc_driver[aa86919b4ce94ddd]::run_compiler::{closure#1}>::{closure#0}, core[3a7799eb5b53fe42]::result::Result<(), rustc_errors[9e83615d379720b2]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[3a7799eb5b53fe42]::result::Result<(), rustc_errors[9e83615d379720b2]::ErrorGuaranteed>>::{closure#1} as core[3a7799eb5b53fe42]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/library/core/src/ops/function.rs:250:5
 132:     0x7fa524f24e58 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h1de6d10e235e916d
                               at /home/matthias/vcs/github/rust_debug_assertions/library/alloc/src/boxed.rs:1988:9
 133:     0x7fa524f24e58 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h7d484493755f58f7
                               at /home/matthias/vcs/github/rust_debug_assertions/library/alloc/src/boxed.rs:1988:9
 134:     0x7fa524f08505 - std::sys::unix::thread::Thread::new::thread_start::hc0cec0ea00f343dd
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/sys/unix/thread.rs:108:17
 135:     0x7fa524c9f8fd - <unknown>
 136:     0x7fa524d21d20 - <unknown>
 137:                0x0 - <unknown>

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.69.0-dev running on x86_64-unknown-linux-gnu

query stack during panic:
end of query stack
thread panicked while panicking. aborting.
[1]    999190 IOT instruction (core dumped)  ~/.rustup/toolchains/local-debug-assertions/bin/rustc
