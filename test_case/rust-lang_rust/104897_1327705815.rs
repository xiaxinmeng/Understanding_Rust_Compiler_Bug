
thread 'rustc' panicked at 'Span must not be empty and have no suggestion', /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_errors/src/diagnostic.rs:629:9
stack backtrace:
   0:     0x7f63d3b6baae - std::backtrace_rs::backtrace::libunwind::trace::h9e8eacd2eabf526f
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7f63d3b6baae - std::backtrace_rs::backtrace::trace_unsynchronized::hdde092c806c77301
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7f63d3b6baae - std::sys_common::backtrace::_print_fmt::h436d3f196368eca4
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/sys_common/backtrace.rs:65:5
   3:     0x7f63d3b6baae - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h99c979908d285522
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/sys_common/backtrace.rs:44:22
   4:     0x7f63d3bed3e8 - core::fmt::write::h1efe05f8990e10dd
                               at /home/matthias/vcs/github/rust_debug_assertions/library/core/src/fmt/mod.rs:1208:17
   5:     0x7f63d3b3815f - std::io::Write::write_fmt::hcf0d06405eaea471
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/io/mod.rs:1682:15
   6:     0x7f63d3b6b8a5 - std::sys_common::backtrace::_print::hd2382d70a4a87cf9
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/sys_common/backtrace.rs:47:5
   7:     0x7f63d3b6b8a5 - std::sys_common::backtrace::print::h6cc4dd10bce64e47
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/sys_common/backtrace.rs:34:9
   8:     0x7f63d3b46ca7 - std::panicking::default_hook::{{closure}}::h05ca7f1b94484886
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/panicking.rs:267:22
   9:     0x7f63d3b469b1 - std::panicking::default_hook::h4980d9d9510a271a
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/panicking.rs:286:9
  10:     0x7f63d68e3ca2 - <alloc[c25fbbbd089b3a7d]::boxed::Box<dyn for<'a, 'b> core[54e86a843f8c5207]::ops::function::Fn<(&'a core[54e86a843f8c5207]::panic::panic_info::PanicInfo<'b>,), Output = ()> + core[54e86a843f8c5207]::marker::Sync + core[54e86a843f8c5207]::marker::Send> as core[54e86a843f8c5207]::ops::function::Fn<(&core[54e86a843f8c5207]::panic::panic_info::PanicInfo,)>>::call
                               at /home/matthias/vcs/github/rust_debug_assertions/library/alloc/src/boxed.rs:2024:9
  11:     0x7f63d68e3ca2 - rustc_driver[66522d8468bac70d]::DEFAULT_HOOK::{closure#0}::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_driver/src/lib.rs:1204:13
  12:     0x7f63d3b4775c - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::hd9404e2542de4ad1
                               at /home/matthias/vcs/github/rust_debug_assertions/library/alloc/src/boxed.rs:2024:9
  13:     0x7f63d3b4775c - std::panicking::rust_panic_with_hook::hcb3a82bc5ebe4888
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/panicking.rs:692:13
  14:     0x7f63d3b6bdf1 - std::panicking::begin_panic_handler::{{closure}}::hcbfedaff40f9a99b
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/panicking.rs:577:13
  15:     0x7f63d3b6bbfe - std::sys_common::backtrace::__rust_end_short_backtrace::hd61dca5be05392f4
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/sys_common/backtrace.rs:137:18
  16:     0x7f63d3b47202 - rust_begin_unwind
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/panicking.rs:575:5
  17:     0x7f63d3bf12f3 - core::panicking::panic_fmt::h43c239f62329fe2a
                               at /home/matthias/vcs/github/rust_debug_assertions/library/core/src/panicking.rs:65:14
  18:     0x7f63db9f97c9 - <rustc_errors[35f3f17f09ba1b6d]::diagnostic::Diagnostic>::multipart_suggestion_with_style::<rustc_error_messages[a5eaf4ed1088ede0]::SubdiagnosticMessage>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_errors/src/diagnostic.rs:629:9
  19:     0x7f63dba455db - <rustc_errors[35f3f17f09ba1b6d]::diagnostic::Diagnostic>::multipart_suggestion::<rustc_error_messages[a5eaf4ed1088ede0]::SubdiagnosticMessage>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_errors/src/diagnostic.rs:597:9
  20:     0x7f63dba455db - <rustc_errors[35f3f17f09ba1b6d]::diagnostic_builder::DiagnosticBuilder<()>>::multipart_suggestion::<rustc_error_messages[a5eaf4ed1088ede0]::SubdiagnosticMessage>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_errors/src/diagnostic_builder.rs:375:13
  21:     0x7f63dba455db - <rustc_lint[b673bfe8af2dd270]::unused::UnusedParens as rustc_lint[b673bfe8af2dd270]::unused::UnusedDelimLint>::emit_unused_delims::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/unused.rs:703:17
  22:     0x7f63dba455db - <<rustc_lint[b673bfe8af2dd270]::unused::UnusedParens as rustc_lint[b673bfe8af2dd270]::unused::UnusedDelimLint>::emit_unused_delims::{closure#0} as core[54e86a843f8c5207]::ops::function::FnOnce<(&mut rustc_errors[35f3f17f09ba1b6d]::diagnostic_builder::DiagnosticBuilder<()>,)>>::call_once::{shim:vtable#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/library/core/src/ops/function.rs:422:5
  23:     0x7f63dba30397 - <alloc[c25fbbbd089b3a7d]::boxed::Box<dyn for<'a, 'b> core[54e86a843f8c5207]::ops::function::FnOnce<(&'a mut rustc_errors[35f3f17f09ba1b6d]::diagnostic_builder::DiagnosticBuilder<'b, ()>,), Output = &'a mut rustc_errors[35f3f17f09ba1b6d]::diagnostic_builder::DiagnosticBuilder<'b, ()>>> as core[54e86a843f8c5207]::ops::function::FnOnce<(&mut rustc_errors[35f3f17f09ba1b6d]::diagnostic_builder::DiagnosticBuilder<()>,)>>::call_once
                               at /home/matthias/vcs/github/rust_debug_assertions/library/alloc/src/boxed.rs:1990:9
  24:     0x7f63dba30397 - rustc_middle[913cc66e421cb1d8]::lint::struct_lint_level::struct_lint_level_impl::<rustc_error_messages[a5eaf4ed1088ede0]::DiagnosticMessage>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/lint.rs:466:9
  25:     0x7f63dba27f42 - rustc_middle[913cc66e421cb1d8]::lint::struct_lint_level::<rustc_error_messages[a5eaf4ed1088ede0]::DiagnosticMessage, <rustc_lint[b673bfe8af2dd270]::unused::UnusedParens as rustc_lint[b673bfe8af2dd270]::unused::UnusedDelimLint>::emit_unused_delims::{closure#0}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_middle/src/lint.rs:470:5
  26:     0x7f63db9d09e9 - <rustc_lint[b673bfe8af2dd270]::levels::LintLevelsBuilder<rustc_lint[b673bfe8af2dd270]::levels::TopDown>>::struct_lint::<rustc_error_messages[a5eaf4ed1088ede0]::DiagnosticMessage, <rustc_lint[b673bfe8af2dd270]::unused::UnusedParens as rustc_lint[b673bfe8af2dd270]::unused::UnusedDelimLint>::emit_unused_delims::{closure#0}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/levels.rs:1087:9
  27:     0x7f63db9d2412 - <rustc_lint[b673bfe8af2dd270]::context::EarlyContext as rustc_lint[b673bfe8af2dd270]::context::LintContext>::lookup::<rustc_error_messages[a5eaf4ed1088ede0]::MultiSpan, rustc_error_messages[a5eaf4ed1088ede0]::DiagnosticMessage, <rustc_lint[b673bfe8af2dd270]::unused::UnusedParens as rustc_lint[b673bfe8af2dd270]::unused::UnusedDelimLint>::emit_unused_delims::{closure#0}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/context.rs:1050:9
  28:     0x7f63db9d2412 - <rustc_lint[b673bfe8af2dd270]::context::EarlyContext as rustc_lint[b673bfe8af2dd270]::context::LintContext>::struct_span_lint::<rustc_error_messages[a5eaf4ed1088ede0]::MultiSpan, rustc_error_messages[a5eaf4ed1088ede0]::DiagnosticMessage, <rustc_lint[b673bfe8af2dd270]::unused::UnusedParens as rustc_lint[b673bfe8af2dd270]::unused::UnusedDelimLint>::emit_unused_delims::{closure#0}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/context.rs:923:9
  29:     0x7f63db9d2412 - <rustc_lint[b673bfe8af2dd270]::unused::UnusedParens as rustc_lint[b673bfe8af2dd270]::unused::UnusedDelimLint>::emit_unused_delims
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/unused.rs:681:9
  30:     0x7f63db9d317f - <rustc_lint[b673bfe8af2dd270]::unused::UnusedParens as rustc_lint[b673bfe8af2dd270]::unused::UnusedDelimLint>::emit_unused_delims_expr
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/unused.rs:665:9
  31:     0x7f63db9d317f - <rustc_lint[b673bfe8af2dd270]::unused::UnusedParens as rustc_lint[b673bfe8af2dd270]::unused::UnusedDelimLint>::check_unused_delims_expr
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/unused.rs:874:21
  32:     0x7f63db9d2b07 - <rustc_lint[b673bfe8af2dd270]::unused::UnusedParens as rustc_lint[b673bfe8af2dd270]::unused::UnusedDelimLint>::check_stmt
  33:     0x7f63db9a3b62 - <rustc_lint[b673bfe8af2dd270]::unused::UnusedParens as rustc_lint[b673bfe8af2dd270]::passes::EarlyLintPass>::check_stmt
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/unused.rs:1003:9
  34:     0x7f63db9a3b62 - <rustc_lint[b673bfe8af2dd270]::BuiltinCombinedEarlyLintPass as rustc_lint[b673bfe8af2dd270]::passes::EarlyLintPass>::check_stmt
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/passes.rs:205:13
  35:     0x7f63d6a8dbb5 - <rustc_lint[b673bfe8af2dd270]::early::EarlyContextAndPass<rustc_lint[b673bfe8af2dd270]::BuiltinCombinedEarlyLintPass> as rustc_ast[9914d275d891e42b]::visit::Visitor>::visit_stmt::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:137:13
  36:     0x7f63d6a8dbb5 - <rustc_lint[b673bfe8af2dd270]::early::EarlyContextAndPass<rustc_lint[b673bfe8af2dd270]::BuiltinCombinedEarlyLintPass>>::with_lint_attrs::<<rustc_lint[b673bfe8af2dd270]::early::EarlyContextAndPass<rustc_lint[b673bfe8af2dd270]::BuiltinCombinedEarlyLintPass> as rustc_ast[9914d275d891e42b]::visit::Visitor>::visit_stmt::{closure#0}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:67:9
  37:     0x7f63d6a8dbb5 - <rustc_lint[b673bfe8af2dd270]::early::EarlyContextAndPass<rustc_lint[b673bfe8af2dd270]::BuiltinCombinedEarlyLintPass> as rustc_ast[9914d275d891e42b]::visit::Visitor>::visit_stmt
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:136:9
  38:     0x7f63d6b2dc3c - rustc_ast[9914d275d891e42b]::visit::walk_block::<rustc_lint[b673bfe8af2dd270]::early::EarlyContextAndPass<rustc_lint[b673bfe8af2dd270]::BuiltinCombinedEarlyLintPass>>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_ast/src/visit.rs:706:5
  39:     0x7f63d6b2dc3c - <rustc_lint[b673bfe8af2dd270]::early::EarlyContextAndPass<rustc_lint[b673bfe8af2dd270]::BuiltinCombinedEarlyLintPass> as rustc_ast[9914d275d891e42b]::visit::Visitor>::visit_block
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:201:9
  40:     0x7f63d6b2dc3c - rustc_ast[9914d275d891e42b]::visit::walk_fn::<rustc_lint[b673bfe8af2dd270]::early::EarlyContextAndPass<rustc_lint[b673bfe8af2dd270]::BuiltinCombinedEarlyLintPass>>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_ast/src/visit.rs:657:13
  41:     0x7f63d6b3215a - <rustc_lint[b673bfe8af2dd270]::early::EarlyContextAndPass<rustc_lint[b673bfe8af2dd270]::BuiltinCombinedEarlyLintPass> as rustc_ast[9914d275d891e42b]::visit::Visitor>::visit_fn
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:150:9
  42:     0x7f63d6b3215a - rustc_ast[9914d275d891e42b]::visit::walk_item::<rustc_lint[b673bfe8af2dd270]::early::EarlyContextAndPass<rustc_lint[b673bfe8af2dd270]::BuiltinCombinedEarlyLintPass>>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_ast/src/visit.rs:312:13
  43:     0x7f63d6a8d479 - <rustc_lint[b673bfe8af2dd270]::early::EarlyContextAndPass<rustc_lint[b673bfe8af2dd270]::BuiltinCombinedEarlyLintPass> as rustc_ast[9914d275d891e42b]::visit::Visitor>::visit_item::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:85:13
  44:     0x7f63d6a8d479 - <rustc_lint[b673bfe8af2dd270]::early::EarlyContextAndPass<rustc_lint[b673bfe8af2dd270]::BuiltinCombinedEarlyLintPass>>::with_lint_attrs::<<rustc_lint[b673bfe8af2dd270]::early::EarlyContextAndPass<rustc_lint[b673bfe8af2dd270]::BuiltinCombinedEarlyLintPass> as rustc_ast[9914d275d891e42b]::visit::Visitor>::visit_item::{closure#0}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:67:9
  45:     0x7f63d6a8d479 - <rustc_lint[b673bfe8af2dd270]::early::EarlyContextAndPass<rustc_lint[b673bfe8af2dd270]::BuiltinCombinedEarlyLintPass> as rustc_ast[9914d275d891e42b]::visit::Visitor>::visit_item
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:83:9
  46:     0x7f63d6b281ec - rustc_ast[9914d275d891e42b]::visit::walk_crate::<rustc_lint[b673bfe8af2dd270]::early::EarlyContextAndPass<rustc_lint[b673bfe8af2dd270]::BuiltinCombinedEarlyLintPass>>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_ast/src/visit.rs:263:5
  47:     0x7f63d6a82cb9 - <&rustc_ast[9914d275d891e42b]::ast::Crate as rustc_lint[b673bfe8af2dd270]::early::EarlyCheckNode>::check::<rustc_lint[b673bfe8af2dd270]::BuiltinCombinedEarlyLintPass>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:350:9
  48:     0x7f63d6a82cb9 - rustc_lint[b673bfe8af2dd270]::early::early_lint_node::<rustc_lint[b673bfe8af2dd270]::BuiltinCombinedEarlyLintPass, &rustc_ast[9914d275d891e42b]::ast::Crate>::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:394:66
  49:     0x7f63d6a82cb9 - <rustc_lint[b673bfe8af2dd270]::early::EarlyContextAndPass<rustc_lint[b673bfe8af2dd270]::BuiltinCombinedEarlyLintPass>>::with_lint_attrs::<rustc_lint[b673bfe8af2dd270]::early::early_lint_node<rustc_lint[b673bfe8af2dd270]::BuiltinCombinedEarlyLintPass, &rustc_ast[9914d275d891e42b]::ast::Crate>::{closure#0}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:67:9
  50:     0x7f63d6a82cb9 - rustc_lint[b673bfe8af2dd270]::early::early_lint_node::<rustc_lint[b673bfe8af2dd270]::BuiltinCombinedEarlyLintPass, &rustc_ast[9914d275d891e42b]::ast::Crate>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:394:5
  51:     0x7f63d6a82cb9 - rustc_lint[b673bfe8af2dd270]::early::check_ast_node::<rustc_lint[b673bfe8af2dd270]::BuiltinCombinedEarlyLintPass, &rustc_ast[9914d275d891e42b]::ast::Crate>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:428:20
  52:     0x7f63d6a9bdb4 - rustc_interface[1e37ca5523f5bc3e]::passes::configure_and_expand::{closure#8}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_interface/src/passes.rs:448:9
  53:     0x7f63d6a9bdb4 - <rustc_data_structures[fdce7cfeec0fbdda]::profiling::VerboseTimingGuard>::run::<(), rustc_interface[1e37ca5523f5bc3e]::passes::configure_and_expand::{closure#8}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_data_structures/src/profiling.rs:727:9
  54:     0x7f63d6a9bdb4 - <rustc_session[a15cfc1451fa042e]::session::Session>::time::<(), rustc_interface[1e37ca5523f5bc3e]::passes::configure_and_expand::{closure#8}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_session/src/utils.rs:10:9
  55:     0x7f63d6ad59f3 - rustc_interface[1e37ca5523f5bc3e]::passes::configure_and_expand
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_interface/src/passes.rs:446:5
  56:     0x7f63d6b5a308 - <rustc_interface[1e37ca5523f5bc3e]::queries::Queries>::expansion::{closure#0}::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_interface/src/queries.rs:178:17
  57:     0x7f63d6b5a308 - <rustc_interface[1e37ca5523f5bc3e]::passes::boxed_resolver::BoxedResolver>::access::<<rustc_interface[1e37ca5523f5bc3e]::queries::Queries>::expansion::{closure#0}::{closure#0}, core[54e86a843f8c5207]::result::Result<rustc_ast[9914d275d891e42b]::ast::Crate, rustc_errors[35f3f17f09ba1b6d]::ErrorGuaranteed>>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_interface/src/passes.rs:134:13
  58:     0x7f63d6b5a308 - <rustc_interface[1e37ca5523f5bc3e]::queries::Queries>::expansion::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_interface/src/queries.rs:177:25
  59:     0x7f63d6b5a308 - <core[54e86a843f8c5207]::option::Option<core[54e86a843f8c5207]::result::Result<(alloc[c25fbbbd089b3a7d]::rc::Rc<rustc_ast[9914d275d891e42b]::ast::Crate>, alloc[c25fbbbd089b3a7d]::rc::Rc<core[54e86a843f8c5207]::cell::RefCell<rustc_interface[1e37ca5523f5bc3e]::passes::boxed_resolver::BoxedResolver>>, alloc[c25fbbbd089b3a7d]::rc::Rc<rustc_lint[b673bfe8af2dd270]::context::LintStore>), rustc_errors[35f3f17f09ba1b6d]::ErrorGuaranteed>>>::get_or_insert_with::<<rustc_interface[1e37ca5523f5bc3e]::queries::Queries>::expansion::{closure#0}>
                               at /home/matthias/vcs/github/rust_debug_assertions/library/core/src/option.rs:1546:49
  60:     0x7f63d6b5a308 - <rustc_interface[1e37ca5523f5bc3e]::queries::Query<(alloc[c25fbbbd089b3a7d]::rc::Rc<rustc_ast[9914d275d891e42b]::ast::Crate>, alloc[c25fbbbd089b3a7d]::rc::Rc<core[54e86a843f8c5207]::cell::RefCell<rustc_interface[1e37ca5523f5bc3e]::passes::boxed_resolver::BoxedResolver>>, alloc[c25fbbbd089b3a7d]::rc::Rc<rustc_lint[b673bfe8af2dd270]::context::LintStore>)>>::compute::<<rustc_interface[1e37ca5523f5bc3e]::queries::Queries>::expansion::{closure#0}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_interface/src/queries.rs:36:9
  61:     0x7f63d6b5a308 - <rustc_interface[1e37ca5523f5bc3e]::queries::Queries>::expansion
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_interface/src/queries.rs:166:9
  62:     0x7f63d696bb60 - rustc_driver[66522d8468bac70d]::run_compiler::{closure#1}::{closure#2}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_driver/src/lib.rs:357:13
  63:     0x7f63d696bb60 - <rustc_interface[1e37ca5523f5bc3e]::interface::Compiler>::enter::<rustc_driver[66522d8468bac70d]::run_compiler::{closure#1}::{closure#2}, core[54e86a843f8c5207]::result::Result<core[54e86a843f8c5207]::option::Option<rustc_interface[1e37ca5523f5bc3e]::queries::Linker>, rustc_errors[35f3f17f09ba1b6d]::ErrorGuaranteed>>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_interface/src/queries.rs:377:19
  64:     0x7f63d68e5805 - rustc_driver[66522d8468bac70d]::run_compiler::{closure#1}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_driver/src/lib.rs:308:22
  65:     0x7f63d68e5805 - rustc_interface[1e37ca5523f5bc3e]::interface::run_compiler::<core[54e86a843f8c5207]::result::Result<(), rustc_errors[35f3f17f09ba1b6d]::ErrorGuaranteed>, rustc_driver[66522d8468bac70d]::run_compiler::{closure#1}>::{closure#0}::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_interface/src/interface.rs:327:21
  66:     0x7f63d68e5805 - rustc_span[114d54e89d0228b9]::with_source_map::<core[54e86a843f8c5207]::result::Result<(), rustc_errors[35f3f17f09ba1b6d]::ErrorGuaranteed>, rustc_interface[1e37ca5523f5bc3e]::interface::run_compiler<core[54e86a843f8c5207]::result::Result<(), rustc_errors[35f3f17f09ba1b6d]::ErrorGuaranteed>, rustc_driver[66522d8468bac70d]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_span/src/lib.rs:1007:5
  67:     0x7f63d696c86c - rustc_interface[1e37ca5523f5bc3e]::interface::run_compiler::<core[54e86a843f8c5207]::result::Result<(), rustc_errors[35f3f17f09ba1b6d]::ErrorGuaranteed>, rustc_driver[66522d8468bac70d]::run_compiler::{closure#1}>::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_interface/src/interface.rs:321:13
  68:     0x7f63d696c86c - <scoped_tls[3f7a8f801319db86]::ScopedKey<rustc_span[114d54e89d0228b9]::SessionGlobals>>::set::<rustc_interface[1e37ca5523f5bc3e]::interface::run_compiler<core[54e86a843f8c5207]::result::Result<(), rustc_errors[35f3f17f09ba1b6d]::ErrorGuaranteed>, rustc_driver[66522d8468bac70d]::run_compiler::{closure#1}>::{closure#0}, core[54e86a843f8c5207]::result::Result<(), rustc_errors[35f3f17f09ba1b6d]::ErrorGuaranteed>>
                               at /home/matthias/.cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137:9
  69:     0x7f63d69135d0 - rustc_span[114d54e89d0228b9]::create_session_globals_then::<core[54e86a843f8c5207]::result::Result<(), rustc_errors[35f3f17f09ba1b6d]::ErrorGuaranteed>, rustc_interface[1e37ca5523f5bc3e]::interface::run_compiler<core[54e86a843f8c5207]::result::Result<(), rustc_errors[35f3f17f09ba1b6d]::ErrorGuaranteed>, rustc_driver[66522d8468bac70d]::run_compiler::{closure#1}>::{closure#0}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_span/src/lib.rs:111:5
  70:     0x7f63d69135d0 - rustc_interface[1e37ca5523f5bc3e]::util::run_in_thread_pool_with_globals::<rustc_interface[1e37ca5523f5bc3e]::interface::run_compiler<core[54e86a843f8c5207]::result::Result<(), rustc_errors[35f3f17f09ba1b6d]::ErrorGuaranteed>, rustc_driver[66522d8468bac70d]::run_compiler::{closure#1}>::{closure#0}, core[54e86a843f8c5207]::result::Result<(), rustc_errors[35f3f17f09ba1b6d]::ErrorGuaranteed>>::{closure#0}::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_interface/src/util.rs:145:38
  71:     0x7f63d69135d0 - std[fcf80d85dffcc129]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[1e37ca5523f5bc3e]::util::run_in_thread_pool_with_globals<rustc_interface[1e37ca5523f5bc3e]::interface::run_compiler<core[54e86a843f8c5207]::result::Result<(), rustc_errors[35f3f17f09ba1b6d]::ErrorGuaranteed>, rustc_driver[66522d8468bac70d]::run_compiler::{closure#1}>::{closure#0}, core[54e86a843f8c5207]::result::Result<(), rustc_errors[35f3f17f09ba1b6d]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[54e86a843f8c5207]::result::Result<(), rustc_errors[35f3f17f09ba1b6d]::ErrorGuaranteed>>
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/sys_common/backtrace.rs:121:18
  72:     0x7f63d68f9939 - <std[fcf80d85dffcc129]::thread::Builder>::spawn_unchecked_::<rustc_interface[1e37ca5523f5bc3e]::util::run_in_thread_pool_with_globals<rustc_interface[1e37ca5523f5bc3e]::interface::run_compiler<core[54e86a843f8c5207]::result::Result<(), rustc_errors[35f3f17f09ba1b6d]::ErrorGuaranteed>, rustc_driver[66522d8468bac70d]::run_compiler::{closure#1}>::{closure#0}, core[54e86a843f8c5207]::result::Result<(), rustc_errors[35f3f17f09ba1b6d]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[54e86a843f8c5207]::result::Result<(), rustc_errors[35f3f17f09ba1b6d]::ErrorGuaranteed>>::{closure#1}::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/thread/mod.rs:550:17
  73:     0x7f63d68f9939 - <core[54e86a843f8c5207]::panic::unwind_safe::AssertUnwindSafe<<std[fcf80d85dffcc129]::thread::Builder>::spawn_unchecked_<rustc_interface[1e37ca5523f5bc3e]::util::run_in_thread_pool_with_globals<rustc_interface[1e37ca5523f5bc3e]::interface::run_compiler<core[54e86a843f8c5207]::result::Result<(), rustc_errors[35f3f17f09ba1b6d]::ErrorGuaranteed>, rustc_driver[66522d8468bac70d]::run_compiler::{closure#1}>::{closure#0}, core[54e86a843f8c5207]::result::Result<(), rustc_errors[35f3f17f09ba1b6d]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[54e86a843f8c5207]::result::Result<(), rustc_errors[35f3f17f09ba1b6d]::ErrorGuaranteed>>::{closure#1}::{closure#0}> as core[54e86a843f8c5207]::ops::function::FnOnce<()>>::call_once
                               at /home/matthias/vcs/github/rust_debug_assertions/library/core/src/panic/unwind_safe.rs:271:9
  74:     0x7f63d68f9939 - std[fcf80d85dffcc129]::panicking::try::do_call::<core[54e86a843f8c5207]::panic::unwind_safe::AssertUnwindSafe<<std[fcf80d85dffcc129]::thread::Builder>::spawn_unchecked_<rustc_interface[1e37ca5523f5bc3e]::util::run_in_thread_pool_with_globals<rustc_interface[1e37ca5523f5bc3e]::interface::run_compiler<core[54e86a843f8c5207]::result::Result<(), rustc_errors[35f3f17f09ba1b6d]::ErrorGuaranteed>, rustc_driver[66522d8468bac70d]::run_compiler::{closure#1}>::{closure#0}, core[54e86a843f8c5207]::result::Result<(), rustc_errors[35f3f17f09ba1b6d]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[54e86a843f8c5207]::result::Result<(), rustc_errors[35f3f17f09ba1b6d]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[54e86a843f8c5207]::result::Result<(), rustc_errors[35f3f17f09ba1b6d]::ErrorGuaranteed>>
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/panicking.rs:483:40
  75:     0x7f63d68f9939 - std[fcf80d85dffcc129]::panicking::try::<core[54e86a843f8c5207]::result::Result<(), rustc_errors[35f3f17f09ba1b6d]::ErrorGuaranteed>, core[54e86a843f8c5207]::panic::unwind_safe::AssertUnwindSafe<<std[fcf80d85dffcc129]::thread::Builder>::spawn_unchecked_<rustc_interface[1e37ca5523f5bc3e]::util::run_in_thread_pool_with_globals<rustc_interface[1e37ca5523f5bc3e]::interface::run_compiler<core[54e86a843f8c5207]::result::Result<(), rustc_errors[35f3f17f09ba1b6d]::ErrorGuaranteed>, rustc_driver[66522d8468bac70d]::run_compiler::{closure#1}>::{closure#0}, core[54e86a843f8c5207]::result::Result<(), rustc_errors[35f3f17f09ba1b6d]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[54e86a843f8c5207]::result::Result<(), rustc_errors[35f3f17f09ba1b6d]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/panicking.rs:447:19
  76:     0x7f63d68f9939 - std[fcf80d85dffcc129]::panic::catch_unwind::<core[54e86a843f8c5207]::panic::unwind_safe::AssertUnwindSafe<<std[fcf80d85dffcc129]::thread::Builder>::spawn_unchecked_<rustc_interface[1e37ca5523f5bc3e]::util::run_in_thread_pool_with_globals<rustc_interface[1e37ca5523f5bc3e]::interface::run_compiler<core[54e86a843f8c5207]::result::Result<(), rustc_errors[35f3f17f09ba1b6d]::ErrorGuaranteed>, rustc_driver[66522d8468bac70d]::run_compiler::{closure#1}>::{closure#0}, core[54e86a843f8c5207]::result::Result<(), rustc_errors[35f3f17f09ba1b6d]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[54e86a843f8c5207]::result::Result<(), rustc_errors[35f3f17f09ba1b6d]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[54e86a843f8c5207]::result::Result<(), rustc_errors[35f3f17f09ba1b6d]::ErrorGuaranteed>>
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/panic.rs:137:14
  77:     0x7f63d68f9939 - <std[fcf80d85dffcc129]::thread::Builder>::spawn_unchecked_::<rustc_interface[1e37ca5523f5bc3e]::util::run_in_thread_pool_with_globals<rustc_interface[1e37ca5523f5bc3e]::interface::run_compiler<core[54e86a843f8c5207]::result::Result<(), rustc_errors[35f3f17f09ba1b6d]::ErrorGuaranteed>, rustc_driver[66522d8468bac70d]::run_compiler::{closure#1}>::{closure#0}, core[54e86a843f8c5207]::result::Result<(), rustc_errors[35f3f17f09ba1b6d]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[54e86a843f8c5207]::result::Result<(), rustc_errors[35f3f17f09ba1b6d]::ErrorGuaranteed>>::{closure#1}
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/thread/mod.rs:549:30
  78:     0x7f63d68f9939 - <<std[fcf80d85dffcc129]::thread::Builder>::spawn_unchecked_<rustc_interface[1e37ca5523f5bc3e]::util::run_in_thread_pool_with_globals<rustc_interface[1e37ca5523f5bc3e]::interface::run_compiler<core[54e86a843f8c5207]::result::Result<(), rustc_errors[35f3f17f09ba1b6d]::ErrorGuaranteed>, rustc_driver[66522d8468bac70d]::run_compiler::{closure#1}>::{closure#0}, core[54e86a843f8c5207]::result::Result<(), rustc_errors[35f3f17f09ba1b6d]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[54e86a843f8c5207]::result::Result<(), rustc_errors[35f3f17f09ba1b6d]::ErrorGuaranteed>>::{closure#1} as core[54e86a843f8c5207]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/library/core/src/ops/function.rs:422:5
  79:     0x7f63d3b229a8 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hd41f56d09defd642
                               at /home/matthias/vcs/github/rust_debug_assertions/library/alloc/src/boxed.rs:1990:9
  80:     0x7f63d3b229a8 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h32db6764a310544a
                               at /home/matthias/vcs/github/rust_debug_assertions/library/alloc/src/boxed.rs:1990:9
  81:     0x7f63d3b092f5 - std::sys::unix::thread::Thread::new::thread_start::he99a78b90f8a7820
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/sys/unix/thread.rs:108:17
  82:     0x7f63d3c9f8fd - <unknown>
  83:     0x7f63d3d21a60 - <unknown>
  84:                0x0 - <unknown>

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.67.0-dev running on x86_64-unknown-linux-gnu

query stack during panic:
end of query stack
thread 'rustc' panicked at 'Found a `push` without a `pop`.', compiler/rustc_lint/src/levels.rs:504:9
stack backtrace:
   0:     0x7f63d3b6baae - std::backtrace_rs::backtrace::libunwind::trace::h9e8eacd2eabf526f
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7f63d3b6baae - std::backtrace_rs::backtrace::trace_unsynchronized::hdde092c806c77301
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7f63d3b6baae - std::sys_common::backtrace::_print_fmt::h436d3f196368eca4
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/sys_common/backtrace.rs:65:5
   3:     0x7f63d3b6baae - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h99c979908d285522
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/sys_common/backtrace.rs:44:22
   4:     0x7f63d3bed3e8 - core::fmt::write::h1efe05f8990e10dd
                               at /home/matthias/vcs/github/rust_debug_assertions/library/core/src/fmt/mod.rs:1208:17
   5:     0x7f63d3b3815f - std::io::Write::write_fmt::hcf0d06405eaea471
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/io/mod.rs:1682:15
   6:     0x7f63d3b6b8a5 - std::sys_common::backtrace::_print::hd2382d70a4a87cf9
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/sys_common/backtrace.rs:47:5
   7:     0x7f63d3b6b8a5 - std::sys_common::backtrace::print::h6cc4dd10bce64e47
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/sys_common/backtrace.rs:34:9
   8:     0x7f63d3b46ca7 - std::panicking::default_hook::{{closure}}::h05ca7f1b94484886
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/panicking.rs:267:22
   9:     0x7f63d3b469b1 - std::panicking::default_hook::h4980d9d9510a271a
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/panicking.rs:286:9
  10:     0x7f63d68e3ca2 - <alloc[c25fbbbd089b3a7d]::boxed::Box<dyn for<'a, 'b> core[54e86a843f8c5207]::ops::function::Fn<(&'a core[54e86a843f8c5207]::panic::panic_info::PanicInfo<'b>,), Output = ()> + core[54e86a843f8c5207]::marker::Sync + core[54e86a843f8c5207]::marker::Send> as core[54e86a843f8c5207]::ops::function::Fn<(&core[54e86a843f8c5207]::panic::panic_info::PanicInfo,)>>::call
                               at /home/matthias/vcs/github/rust_debug_assertions/library/alloc/src/boxed.rs:2024:9
  11:     0x7f63d68e3ca2 - rustc_driver[66522d8468bac70d]::DEFAULT_HOOK::{closure#0}::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_driver/src/lib.rs:1204:13
  12:     0x7f63d3b4775c - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::hd9404e2542de4ad1
                               at /home/matthias/vcs/github/rust_debug_assertions/library/alloc/src/boxed.rs:2024:9
  13:     0x7f63d3b4775c - std::panicking::rust_panic_with_hook::hcb3a82bc5ebe4888
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/panicking.rs:692:13
  14:     0x7f63d3b6bdf1 - std::panicking::begin_panic_handler::{{closure}}::hcbfedaff40f9a99b
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/panicking.rs:577:13
  15:     0x7f63d3b6bbfe - std::sys_common::backtrace::__rust_end_short_backtrace::hd61dca5be05392f4
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/sys_common/backtrace.rs:137:18
  16:     0x7f63d3b47202 - rust_begin_unwind
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/panicking.rs:575:5
  17:     0x7f63d3bf12f3 - core::panicking::panic_fmt::h43c239f62329fe2a
                               at /home/matthias/vcs/github/rust_debug_assertions/library/core/src/panicking.rs:65:14
  18:     0x7f63db9cb8c9 - <rustc_lint[b673bfe8af2dd270]::levels::BuilderPush as core[54e86a843f8c5207]::ops::drop::Drop>::drop
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/levels.rs:504:9
  19:     0x7f63d6a8deb0 - <rustc_lint[b673bfe8af2dd270]::early::EarlyContextAndPass<rustc_lint[b673bfe8af2dd270]::BuiltinCombinedEarlyLintPass>>::with_lint_attrs::<<rustc_lint[b673bfe8af2dd270]::early::EarlyContextAndPass<rustc_lint[b673bfe8af2dd270]::BuiltinCombinedEarlyLintPass> as rustc_ast[9914d275d891e42b]::visit::Visitor>::visit_stmt::{closure#0}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:71:5
  20:     0x7f63d6a8deb0 - <rustc_lint[b673bfe8af2dd270]::early::EarlyContextAndPass<rustc_lint[b673bfe8af2dd270]::BuiltinCombinedEarlyLintPass> as rustc_ast[9914d275d891e42b]::visit::Visitor>::visit_stmt
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:136:9
  21:     0x7f63d6b2dc3c - rustc_ast[9914d275d891e42b]::visit::walk_block::<rustc_lint[b673bfe8af2dd270]::early::EarlyContextAndPass<rustc_lint[b673bfe8af2dd270]::BuiltinCombinedEarlyLintPass>>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_ast/src/visit.rs:706:5
  22:     0x7f63d6b2dc3c - <rustc_lint[b673bfe8af2dd270]::early::EarlyContextAndPass<rustc_lint[b673bfe8af2dd270]::BuiltinCombinedEarlyLintPass> as rustc_ast[9914d275d891e42b]::visit::Visitor>::visit_block
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:201:9
  23:     0x7f63d6b2dc3c - rustc_ast[9914d275d891e42b]::visit::walk_fn::<rustc_lint[b673bfe8af2dd270]::early::EarlyContextAndPass<rustc_lint[b673bfe8af2dd270]::BuiltinCombinedEarlyLintPass>>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_ast/src/visit.rs:657:13
  24:     0x7f63d6b3215a - <rustc_lint[b673bfe8af2dd270]::early::EarlyContextAndPass<rustc_lint[b673bfe8af2dd270]::BuiltinCombinedEarlyLintPass> as rustc_ast[9914d275d891e42b]::visit::Visitor>::visit_fn
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:150:9
  25:     0x7f63d6b3215a - rustc_ast[9914d275d891e42b]::visit::walk_item::<rustc_lint[b673bfe8af2dd270]::early::EarlyContextAndPass<rustc_lint[b673bfe8af2dd270]::BuiltinCombinedEarlyLintPass>>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_ast/src/visit.rs:312:13
  26:     0x7f63d6a8d479 - <rustc_lint[b673bfe8af2dd270]::early::EarlyContextAndPass<rustc_lint[b673bfe8af2dd270]::BuiltinCombinedEarlyLintPass> as rustc_ast[9914d275d891e42b]::visit::Visitor>::visit_item::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:85:13
  27:     0x7f63d6a8d479 - <rustc_lint[b673bfe8af2dd270]::early::EarlyContextAndPass<rustc_lint[b673bfe8af2dd270]::BuiltinCombinedEarlyLintPass>>::with_lint_attrs::<<rustc_lint[b673bfe8af2dd270]::early::EarlyContextAndPass<rustc_lint[b673bfe8af2dd270]::BuiltinCombinedEarlyLintPass> as rustc_ast[9914d275d891e42b]::visit::Visitor>::visit_item::{closure#0}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:67:9
  28:     0x7f63d6a8d479 - <rustc_lint[b673bfe8af2dd270]::early::EarlyContextAndPass<rustc_lint[b673bfe8af2dd270]::BuiltinCombinedEarlyLintPass> as rustc_ast[9914d275d891e42b]::visit::Visitor>::visit_item
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:83:9
  29:     0x7f63d6b281ec - rustc_ast[9914d275d891e42b]::visit::walk_crate::<rustc_lint[b673bfe8af2dd270]::early::EarlyContextAndPass<rustc_lint[b673bfe8af2dd270]::BuiltinCombinedEarlyLintPass>>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_ast/src/visit.rs:263:5
  30:     0x7f63d6a82cb9 - <&rustc_ast[9914d275d891e42b]::ast::Crate as rustc_lint[b673bfe8af2dd270]::early::EarlyCheckNode>::check::<rustc_lint[b673bfe8af2dd270]::BuiltinCombinedEarlyLintPass>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:350:9
  31:     0x7f63d6a82cb9 - rustc_lint[b673bfe8af2dd270]::early::early_lint_node::<rustc_lint[b673bfe8af2dd270]::BuiltinCombinedEarlyLintPass, &rustc_ast[9914d275d891e42b]::ast::Crate>::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:394:66
  32:     0x7f63d6a82cb9 - <rustc_lint[b673bfe8af2dd270]::early::EarlyContextAndPass<rustc_lint[b673bfe8af2dd270]::BuiltinCombinedEarlyLintPass>>::with_lint_attrs::<rustc_lint[b673bfe8af2dd270]::early::early_lint_node<rustc_lint[b673bfe8af2dd270]::BuiltinCombinedEarlyLintPass, &rustc_ast[9914d275d891e42b]::ast::Crate>::{closure#0}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:67:9
  33:     0x7f63d6a82cb9 - rustc_lint[b673bfe8af2dd270]::early::early_lint_node::<rustc_lint[b673bfe8af2dd270]::BuiltinCombinedEarlyLintPass, &rustc_ast[9914d275d891e42b]::ast::Crate>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:394:5
  34:     0x7f63d6a82cb9 - rustc_lint[b673bfe8af2dd270]::early::check_ast_node::<rustc_lint[b673bfe8af2dd270]::BuiltinCombinedEarlyLintPass, &rustc_ast[9914d275d891e42b]::ast::Crate>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_lint/src/early.rs:428:20
  35:     0x7f63d6a9bdb4 - rustc_interface[1e37ca5523f5bc3e]::passes::configure_and_expand::{closure#8}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_interface/src/passes.rs:448:9
  36:     0x7f63d6a9bdb4 - <rustc_data_structures[fdce7cfeec0fbdda]::profiling::VerboseTimingGuard>::run::<(), rustc_interface[1e37ca5523f5bc3e]::passes::configure_and_expand::{closure#8}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_data_structures/src/profiling.rs:727:9
  37:     0x7f63d6a9bdb4 - <rustc_session[a15cfc1451fa042e]::session::Session>::time::<(), rustc_interface[1e37ca5523f5bc3e]::passes::configure_and_expand::{closure#8}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_session/src/utils.rs:10:9
  38:     0x7f63d6ad59f3 - rustc_interface[1e37ca5523f5bc3e]::passes::configure_and_expand
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_interface/src/passes.rs:446:5
  39:     0x7f63d6b5a308 - <rustc_interface[1e37ca5523f5bc3e]::queries::Queries>::expansion::{closure#0}::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_interface/src/queries.rs:178:17
  40:     0x7f63d6b5a308 - <rustc_interface[1e37ca5523f5bc3e]::passes::boxed_resolver::BoxedResolver>::access::<<rustc_interface[1e37ca5523f5bc3e]::queries::Queries>::expansion::{closure#0}::{closure#0}, core[54e86a843f8c5207]::result::Result<rustc_ast[9914d275d891e42b]::ast::Crate, rustc_errors[35f3f17f09ba1b6d]::ErrorGuaranteed>>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_interface/src/passes.rs:134:13
  41:     0x7f63d6b5a308 - <rustc_interface[1e37ca5523f5bc3e]::queries::Queries>::expansion::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_interface/src/queries.rs:177:25
  42:     0x7f63d6b5a308 - <core[54e86a843f8c5207]::option::Option<core[54e86a843f8c5207]::result::Result<(alloc[c25fbbbd089b3a7d]::rc::Rc<rustc_ast[9914d275d891e42b]::ast::Crate>, alloc[c25fbbbd089b3a7d]::rc::Rc<core[54e86a843f8c5207]::cell::RefCell<rustc_interface[1e37ca5523f5bc3e]::passes::boxed_resolver::BoxedResolver>>, alloc[c25fbbbd089b3a7d]::rc::Rc<rustc_lint[b673bfe8af2dd270]::context::LintStore>), rustc_errors[35f3f17f09ba1b6d]::ErrorGuaranteed>>>::get_or_insert_with::<<rustc_interface[1e37ca5523f5bc3e]::queries::Queries>::expansion::{closure#0}>
                               at /home/matthias/vcs/github/rust_debug_assertions/library/core/src/option.rs:1546:49
  43:     0x7f63d6b5a308 - <rustc_interface[1e37ca5523f5bc3e]::queries::Query<(alloc[c25fbbbd089b3a7d]::rc::Rc<rustc_ast[9914d275d891e42b]::ast::Crate>, alloc[c25fbbbd089b3a7d]::rc::Rc<core[54e86a843f8c5207]::cell::RefCell<rustc_interface[1e37ca5523f5bc3e]::passes::boxed_resolver::BoxedResolver>>, alloc[c25fbbbd089b3a7d]::rc::Rc<rustc_lint[b673bfe8af2dd270]::context::LintStore>)>>::compute::<<rustc_interface[1e37ca5523f5bc3e]::queries::Queries>::expansion::{closure#0}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_interface/src/queries.rs:36:9
  44:     0x7f63d6b5a308 - <rustc_interface[1e37ca5523f5bc3e]::queries::Queries>::expansion
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_interface/src/queries.rs:166:9
  45:     0x7f63d696bb60 - rustc_driver[66522d8468bac70d]::run_compiler::{closure#1}::{closure#2}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_driver/src/lib.rs:357:13
  46:     0x7f63d696bb60 - <rustc_interface[1e37ca5523f5bc3e]::interface::Compiler>::enter::<rustc_driver[66522d8468bac70d]::run_compiler::{closure#1}::{closure#2}, core[54e86a843f8c5207]::result::Result<core[54e86a843f8c5207]::option::Option<rustc_interface[1e37ca5523f5bc3e]::queries::Linker>, rustc_errors[35f3f17f09ba1b6d]::ErrorGuaranteed>>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_interface/src/queries.rs:377:19
  47:     0x7f63d68e5805 - rustc_driver[66522d8468bac70d]::run_compiler::{closure#1}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_driver/src/lib.rs:308:22
  48:     0x7f63d68e5805 - rustc_interface[1e37ca5523f5bc3e]::interface::run_compiler::<core[54e86a843f8c5207]::result::Result<(), rustc_errors[35f3f17f09ba1b6d]::ErrorGuaranteed>, rustc_driver[66522d8468bac70d]::run_compiler::{closure#1}>::{closure#0}::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_interface/src/interface.rs:327:21
  49:     0x7f63d68e5805 - rustc_span[114d54e89d0228b9]::with_source_map::<core[54e86a843f8c5207]::result::Result<(), rustc_errors[35f3f17f09ba1b6d]::ErrorGuaranteed>, rustc_interface[1e37ca5523f5bc3e]::interface::run_compiler<core[54e86a843f8c5207]::result::Result<(), rustc_errors[35f3f17f09ba1b6d]::ErrorGuaranteed>, rustc_driver[66522d8468bac70d]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_span/src/lib.rs:1007:5
  50:     0x7f63d696c86c - rustc_interface[1e37ca5523f5bc3e]::interface::run_compiler::<core[54e86a843f8c5207]::result::Result<(), rustc_errors[35f3f17f09ba1b6d]::ErrorGuaranteed>, rustc_driver[66522d8468bac70d]::run_compiler::{closure#1}>::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_interface/src/interface.rs:321:13
  51:     0x7f63d696c86c - <scoped_tls[3f7a8f801319db86]::ScopedKey<rustc_span[114d54e89d0228b9]::SessionGlobals>>::set::<rustc_interface[1e37ca5523f5bc3e]::interface::run_compiler<core[54e86a843f8c5207]::result::Result<(), rustc_errors[35f3f17f09ba1b6d]::ErrorGuaranteed>, rustc_driver[66522d8468bac70d]::run_compiler::{closure#1}>::{closure#0}, core[54e86a843f8c5207]::result::Result<(), rustc_errors[35f3f17f09ba1b6d]::ErrorGuaranteed>>
                               at /home/matthias/.cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137:9
  52:     0x7f63d69135d0 - rustc_span[114d54e89d0228b9]::create_session_globals_then::<core[54e86a843f8c5207]::result::Result<(), rustc_errors[35f3f17f09ba1b6d]::ErrorGuaranteed>, rustc_interface[1e37ca5523f5bc3e]::interface::run_compiler<core[54e86a843f8c5207]::result::Result<(), rustc_errors[35f3f17f09ba1b6d]::ErrorGuaranteed>, rustc_driver[66522d8468bac70d]::run_compiler::{closure#1}>::{closure#0}>
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_span/src/lib.rs:111:5
  53:     0x7f63d69135d0 - rustc_interface[1e37ca5523f5bc3e]::util::run_in_thread_pool_with_globals::<rustc_interface[1e37ca5523f5bc3e]::interface::run_compiler<core[54e86a843f8c5207]::result::Result<(), rustc_errors[35f3f17f09ba1b6d]::ErrorGuaranteed>, rustc_driver[66522d8468bac70d]::run_compiler::{closure#1}>::{closure#0}, core[54e86a843f8c5207]::result::Result<(), rustc_errors[35f3f17f09ba1b6d]::ErrorGuaranteed>>::{closure#0}::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/compiler/rustc_interface/src/util.rs:145:38
  54:     0x7f63d69135d0 - std[fcf80d85dffcc129]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[1e37ca5523f5bc3e]::util::run_in_thread_pool_with_globals<rustc_interface[1e37ca5523f5bc3e]::interface::run_compiler<core[54e86a843f8c5207]::result::Result<(), rustc_errors[35f3f17f09ba1b6d]::ErrorGuaranteed>, rustc_driver[66522d8468bac70d]::run_compiler::{closure#1}>::{closure#0}, core[54e86a843f8c5207]::result::Result<(), rustc_errors[35f3f17f09ba1b6d]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[54e86a843f8c5207]::result::Result<(), rustc_errors[35f3f17f09ba1b6d]::ErrorGuaranteed>>
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/sys_common/backtrace.rs:121:18
  55:     0x7f63d68f9939 - <std[fcf80d85dffcc129]::thread::Builder>::spawn_unchecked_::<rustc_interface[1e37ca5523f5bc3e]::util::run_in_thread_pool_with_globals<rustc_interface[1e37ca5523f5bc3e]::interface::run_compiler<core[54e86a843f8c5207]::result::Result<(), rustc_errors[35f3f17f09ba1b6d]::ErrorGuaranteed>, rustc_driver[66522d8468bac70d]::run_compiler::{closure#1}>::{closure#0}, core[54e86a843f8c5207]::result::Result<(), rustc_errors[35f3f17f09ba1b6d]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[54e86a843f8c5207]::result::Result<(), rustc_errors[35f3f17f09ba1b6d]::ErrorGuaranteed>>::{closure#1}::{closure#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/thread/mod.rs:550:17
  56:     0x7f63d68f9939 - <core[54e86a843f8c5207]::panic::unwind_safe::AssertUnwindSafe<<std[fcf80d85dffcc129]::thread::Builder>::spawn_unchecked_<rustc_interface[1e37ca5523f5bc3e]::util::run_in_thread_pool_with_globals<rustc_interface[1e37ca5523f5bc3e]::interface::run_compiler<core[54e86a843f8c5207]::result::Result<(), rustc_errors[35f3f17f09ba1b6d]::ErrorGuaranteed>, rustc_driver[66522d8468bac70d]::run_compiler::{closure#1}>::{closure#0}, core[54e86a843f8c5207]::result::Result<(), rustc_errors[35f3f17f09ba1b6d]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[54e86a843f8c5207]::result::Result<(), rustc_errors[35f3f17f09ba1b6d]::ErrorGuaranteed>>::{closure#1}::{closure#0}> as core[54e86a843f8c5207]::ops::function::FnOnce<()>>::call_once
                               at /home/matthias/vcs/github/rust_debug_assertions/library/core/src/panic/unwind_safe.rs:271:9
  57:     0x7f63d68f9939 - std[fcf80d85dffcc129]::panicking::try::do_call::<core[54e86a843f8c5207]::panic::unwind_safe::AssertUnwindSafe<<std[fcf80d85dffcc129]::thread::Builder>::spawn_unchecked_<rustc_interface[1e37ca5523f5bc3e]::util::run_in_thread_pool_with_globals<rustc_interface[1e37ca5523f5bc3e]::interface::run_compiler<core[54e86a843f8c5207]::result::Result<(), rustc_errors[35f3f17f09ba1b6d]::ErrorGuaranteed>, rustc_driver[66522d8468bac70d]::run_compiler::{closure#1}>::{closure#0}, core[54e86a843f8c5207]::result::Result<(), rustc_errors[35f3f17f09ba1b6d]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[54e86a843f8c5207]::result::Result<(), rustc_errors[35f3f17f09ba1b6d]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[54e86a843f8c5207]::result::Result<(), rustc_errors[35f3f17f09ba1b6d]::ErrorGuaranteed>>
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/panicking.rs:483:40
  58:     0x7f63d68f9939 - std[fcf80d85dffcc129]::panicking::try::<core[54e86a843f8c5207]::result::Result<(), rustc_errors[35f3f17f09ba1b6d]::ErrorGuaranteed>, core[54e86a843f8c5207]::panic::unwind_safe::AssertUnwindSafe<<std[fcf80d85dffcc129]::thread::Builder>::spawn_unchecked_<rustc_interface[1e37ca5523f5bc3e]::util::run_in_thread_pool_with_globals<rustc_interface[1e37ca5523f5bc3e]::interface::run_compiler<core[54e86a843f8c5207]::result::Result<(), rustc_errors[35f3f17f09ba1b6d]::ErrorGuaranteed>, rustc_driver[66522d8468bac70d]::run_compiler::{closure#1}>::{closure#0}, core[54e86a843f8c5207]::result::Result<(), rustc_errors[35f3f17f09ba1b6d]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[54e86a843f8c5207]::result::Result<(), rustc_errors[35f3f17f09ba1b6d]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/panicking.rs:447:19
  59:     0x7f63d68f9939 - std[fcf80d85dffcc129]::panic::catch_unwind::<core[54e86a843f8c5207]::panic::unwind_safe::AssertUnwindSafe<<std[fcf80d85dffcc129]::thread::Builder>::spawn_unchecked_<rustc_interface[1e37ca5523f5bc3e]::util::run_in_thread_pool_with_globals<rustc_interface[1e37ca5523f5bc3e]::interface::run_compiler<core[54e86a843f8c5207]::result::Result<(), rustc_errors[35f3f17f09ba1b6d]::ErrorGuaranteed>, rustc_driver[66522d8468bac70d]::run_compiler::{closure#1}>::{closure#0}, core[54e86a843f8c5207]::result::Result<(), rustc_errors[35f3f17f09ba1b6d]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[54e86a843f8c5207]::result::Result<(), rustc_errors[35f3f17f09ba1b6d]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[54e86a843f8c5207]::result::Result<(), rustc_errors[35f3f17f09ba1b6d]::ErrorGuaranteed>>
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/panic.rs:137:14
  60:     0x7f63d68f9939 - <std[fcf80d85dffcc129]::thread::Builder>::spawn_unchecked_::<rustc_interface[1e37ca5523f5bc3e]::util::run_in_thread_pool_with_globals<rustc_interface[1e37ca5523f5bc3e]::interface::run_compiler<core[54e86a843f8c5207]::result::Result<(), rustc_errors[35f3f17f09ba1b6d]::ErrorGuaranteed>, rustc_driver[66522d8468bac70d]::run_compiler::{closure#1}>::{closure#0}, core[54e86a843f8c5207]::result::Result<(), rustc_errors[35f3f17f09ba1b6d]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[54e86a843f8c5207]::result::Result<(), rustc_errors[35f3f17f09ba1b6d]::ErrorGuaranteed>>::{closure#1}
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/thread/mod.rs:549:30
  61:     0x7f63d68f9939 - <<std[fcf80d85dffcc129]::thread::Builder>::spawn_unchecked_<rustc_interface[1e37ca5523f5bc3e]::util::run_in_thread_pool_with_globals<rustc_interface[1e37ca5523f5bc3e]::interface::run_compiler<core[54e86a843f8c5207]::result::Result<(), rustc_errors[35f3f17f09ba1b6d]::ErrorGuaranteed>, rustc_driver[66522d8468bac70d]::run_compiler::{closure#1}>::{closure#0}, core[54e86a843f8c5207]::result::Result<(), rustc_errors[35f3f17f09ba1b6d]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[54e86a843f8c5207]::result::Result<(), rustc_errors[35f3f17f09ba1b6d]::ErrorGuaranteed>>::{closure#1} as core[54e86a843f8c5207]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
                               at /home/matthias/vcs/github/rust_debug_assertions/library/core/src/ops/function.rs:422:5
  62:     0x7f63d3b229a8 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hd41f56d09defd642
                               at /home/matthias/vcs/github/rust_debug_assertions/library/alloc/src/boxed.rs:1990:9
  63:     0x7f63d3b229a8 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h32db6764a310544a
                               at /home/matthias/vcs/github/rust_debug_assertions/library/alloc/src/boxed.rs:1990:9
  64:     0x7f63d3b092f5 - std::sys::unix::thread::Thread::new::thread_start::he99a78b90f8a7820
                               at /home/matthias/vcs/github/rust_debug_assertions/library/std/src/sys/unix/thread.rs:108:17
  65:     0x7f63d3c9f8fd - <unknown>
  66:     0x7f63d3d21a60 - <unknown>
  67:                0x0 - <unknown>

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.67.0-dev running on x86_64-unknown-linux-gnu

query stack during panic:
end of query stack
thread panicked while panicking. aborting.
[1]    1719852 IOT instruction (core dumped)   ./icemaker_87924.rs
