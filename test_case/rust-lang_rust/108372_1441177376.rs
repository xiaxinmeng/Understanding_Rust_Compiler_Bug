plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:697bea7ddceb6696743da8f159f268aef8bfb3c6)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---
   Compiling chalk-engine v0.87.0
   Compiling gsgdt v0.1.2
   Compiling rustc_index v0.0.0 (/checkout/compiler/rustc_index)
thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `Some(SubstitutionPart { span: compiler/rustc_index/src/interval.rs:226:21: 226:21 (#0), snippet: "" })`,
 right: `None`: Span must not be empty and have no suggestion', /checkout/compiler/rustc_errors/src/diagnostic.rs:627:9
   0:     0x7fe61dc891c6 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h53e61b71c84f668e
   0:     0x7fe61dc891c6 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h53e61b71c84f668e
   1:     0x7fe61dcf5248 - core::fmt::write::h370ceefc764332d1
   2:     0x7fe61dc7a861 - std::io::Write::write_fmt::h29ae07842e655986
   3:     0x7fe61dc88f95 - std::sys_common::backtrace::print::h60a6ac165bdd7b12
   4:     0x7fe61dc8c3d7 - std::panicking::default_hook::{{closure}}::h5edace8ef4ec4fe3
   5:     0x7fe61dc8c0c2 - std::panicking::default_hook::h0f63acebb79ef1ba
   6:     0x7fe61e701335 - rustc_driver_impl[d6863e2e44b1d54b]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fe61dc8ccd0 - std::panicking::rust_panic_with_hook::h190cc8de795c3aef
   8:     0x7fe61dc8ca19 - std::panicking::begin_panic_handler::{{closure}}::hd42ae2c63e2914e5
   9:     0x7fe61dc8973c - std::sys_common::backtrace::__rust_end_short_backtrace::h24f6a86a9a516860
  11:     0x7fe61dc40033 - core::panicking::panic_fmt::ha6d05d75fb2b7d9f
  11:     0x7fe61dc40033 - core::panicking::panic_fmt::ha6d05d75fb2b7d9f
  12:     0x7fe61dc404a9 - core::panicking::assert_failed_inner::hc2e6cee167734903
  13:     0x7fe61e671d8b - core[78b8f9b0dd37335f]::panicking::assert_failed::<core[78b8f9b0dd37335f]::option::Option<&rustc_errors[bbd20863eb2e02a9]::SubstitutionPart>, core[78b8f9b0dd37335f]::option::Option<&rustc_errors[bbd20863eb2e02a9]::SubstitutionPart>>
  14:     0x7fe620d8dd42 - <rustc_errors[bbd20863eb2e02a9]::diagnostic::Diagnostic>::multipart_suggestion_with_style::<rustc_error_messages[472e205dd583dfe]::SubdiagnosticMessage>
  15:     0x7fe620ce148b - <rustc_lint[fe6d62f39905847c]::lints::RefBinopOnCopyTypeSuggestion as rustc_errors[bbd20863eb2e02a9]::diagnostic::AddToDiagnostic>::add_to_diagnostic
  16:     0x7fe620ce605f - <rustc_lint[fe6d62f39905847c]::lints::RefBinopOnCopyTypeDiag as rustc_errors[bbd20863eb2e02a9]::diagnostic::DecorateLint<()>>::decorate_lint
  17:     0x7fe620d01354 - <<rustc_lint[fe6d62f39905847c]::context::LateContext as rustc_lint[fe6d62f39905847c]::context::LintContext>::emit_spanned_lint<rustc_span[965f05b949b72626]::span_encoding::Span, rustc_lint[fe6d62f39905847c]::lints::RefBinopOnCopyTypeDiag>::{closure#0} as core[78b8f9b0dd37335f]::ops::function::FnOnce<(&mut rustc_errors[bbd20863eb2e02a9]::diagnostic_builder::DiagnosticBuilder<()>,)>>::call_once::{shim:vtable#0}
  18:     0x7fe620cff36b - rustc_middle[1be4fd16a1bcb82a]::lint::struct_lint_level::struct_lint_level_impl::<rustc_error_messages[472e205dd583dfe]::DiagnosticMessage>
  19:     0x7fe620cf8a8f - rustc_middle[1be4fd16a1bcb82a]::lint::struct_lint_level::<rustc_error_messages[472e205dd583dfe]::DiagnosticMessage, <rustc_lint[fe6d62f39905847c]::context::LateContext as rustc_lint[fe6d62f39905847c]::context::LintContext>::emit_spanned_lint<rustc_span[965f05b949b72626]::span_encoding::Span, rustc_lint[fe6d62f39905847c]::lints::RefBinopOnCopyTypeDiag>::{closure#0}>
  20:     0x7fe620cc72c7 - <rustc_middle[1be4fd16a1bcb82a]::ty::context::TyCtxt>::struct_span_lint_hir::<rustc_span[965f05b949b72626]::span_encoding::Span, rustc_error_messages[472e205dd583dfe]::DiagnosticMessage, <rustc_lint[fe6d62f39905847c]::context::LateContext as rustc_lint[fe6d62f39905847c]::context::LintContext>::emit_spanned_lint<rustc_span[965f05b949b72626]::span_encoding::Span, rustc_lint[fe6d62f39905847c]::lints::RefBinopOnCopyTypeDiag>::{closure#0}>
  21:     0x7fe620d6fd96 - <rustc_lint[fe6d62f39905847c]::context::LateContext as rustc_lint[fe6d62f39905847c]::context::LintContext>::emit_spanned_lint::<rustc_span[965f05b949b72626]::span_encoding::Span, rustc_lint[fe6d62f39905847c]::lints::RefBinopOnCopyTypeDiag>
  22:     0x7fe620d3e885 - <rustc_lint[fe6d62f39905847c]::builtin::RefBinopOnCopyType as rustc_lint[fe6d62f39905847c]::passes::LateLintPass>::check_expr
  23:     0x7fe620d613e1 - <rustc_lint[fe6d62f39905847c]::BuiltinCombinedModuleLateLintPass as rustc_lint[fe6d62f39905847c]::passes::LateLintPass>::check_expr
  24:     0x7fe620cef0fd - <rustc_lint[fe6d62f39905847c]::late::LateContextAndPass<rustc_lint[fe6d62f39905847c]::BuiltinCombinedModuleLateLintPass> as rustc_hir[dbafcb5a748c6cce]::intravisit::Visitor>::visit_expr
  25:     0x7fe620d7d996 - rustc_hir[dbafcb5a748c6cce]::intravisit::walk_expr::<rustc_lint[fe6d62f39905847c]::late::LateContextAndPass<rustc_lint[fe6d62f39905847c]::BuiltinCombinedModuleLateLintPass>>
  26:     0x7fe620cef108 - <rustc_lint[fe6d62f39905847c]::late::LateContextAndPass<rustc_lint[fe6d62f39905847c]::BuiltinCombinedModuleLateLintPass> as rustc_hir[dbafcb5a748c6cce]::intravisit::Visitor>::visit_expr
  27:     0x7fe620cef108 - <rustc_lint[fe6d62f39905847c]::late::LateContextAndPass<rustc_lint[fe6d62f39905847c]::BuiltinCombinedModuleLateLintPass> as rustc_hir[dbafcb5a748c6cce]::intravisit::Visitor>::visit_expr
  28:     0x7fe620d7d9c5 - rustc_hir[dbafcb5a748c6cce]::intravisit::walk_expr::<rustc_lint[fe6d62f39905847c]::late::LateContextAndPass<rustc_lint[fe6d62f39905847c]::BuiltinCombinedModuleLateLintPass>>
  29:     0x7fe620cef108 - <rustc_lint[fe6d62f39905847c]::late::LateContextAndPass<rustc_lint[fe6d62f39905847c]::BuiltinCombinedModuleLateLintPass> as rustc_hir[dbafcb5a748c6cce]::intravisit::Visitor>::visit_expr
  30:     0x7fe620cefb72 - <rustc_lint[fe6d62f39905847c]::late::LateContextAndPass<rustc_lint[fe6d62f39905847c]::BuiltinCombinedModuleLateLintPass> as rustc_hir[dbafcb5a748c6cce]::intravisit::Visitor>::visit_stmt
  31:     0x7fe620d7db1c - rustc_hir[dbafcb5a748c6cce]::intravisit::walk_expr::<rustc_lint[fe6d62f39905847c]::late::LateContextAndPass<rustc_lint[fe6d62f39905847c]::BuiltinCombinedModuleLateLintPass>>
  32:     0x7fe620cef108 - <rustc_lint[fe6d62f39905847c]::late::LateContextAndPass<rustc_lint[fe6d62f39905847c]::BuiltinCombinedModuleLateLintPass> as rustc_hir[dbafcb5a748c6cce]::intravisit::Visitor>::visit_expr
  33:     0x7fe620d7da3f - rustc_hir[dbafcb5a748c6cce]::intravisit::walk_expr::<rustc_lint[fe6d62f39905847c]::late::LateContextAndPass<rustc_lint[fe6d62f39905847c]::BuiltinCombinedModuleLateLintPass>>
  34:     0x7fe620cef108 - <rustc_lint[fe6d62f39905847c]::late::LateContextAndPass<rustc_lint[fe6d62f39905847c]::BuiltinCombinedModuleLateLintPass> as rustc_hir[dbafcb5a748c6cce]::intravisit::Visitor>::visit_expr
  35:     0x7fe620cefb72 - <rustc_lint[fe6d62f39905847c]::late::LateContextAndPass<rustc_lint[fe6d62f39905847c]::BuiltinCombinedModuleLateLintPass> as rustc_hir[dbafcb5a748c6cce]::intravisit::Visitor>::visit_stmt
  36:     0x7fe620d7d9fc - rustc_hir[dbafcb5a748c6cce]::intravisit::walk_expr::<rustc_lint[fe6d62f39905847c]::late::LateContextAndPass<rustc_lint[fe6d62f39905847c]::BuiltinCombinedModuleLateLintPass>>
  37:     0x7fe620cef108 - <rustc_lint[fe6d62f39905847c]::late::LateContextAndPass<rustc_lint[fe6d62f39905847c]::BuiltinCombinedModuleLateLintPass> as rustc_hir[dbafcb5a748c6cce]::intravisit::Visitor>::visit_expr
  38:     0x7fe620d7da3f - rustc_hir[dbafcb5a748c6cce]::intravisit::walk_expr::<rustc_lint[fe6d62f39905847c]::late::LateContextAndPass<rustc_lint[fe6d62f39905847c]::BuiltinCombinedModuleLateLintPass>>
  39:     0x7fe620cef108 - <rustc_lint[fe6d62f39905847c]::late::LateContextAndPass<rustc_lint[fe6d62f39905847c]::BuiltinCombinedModuleLateLintPass> as rustc_hir[dbafcb5a748c6cce]::intravisit::Visitor>::visit_expr
  40:     0x7fe620cef108 - <rustc_lint[fe6d62f39905847c]::late::LateContextAndPass<rustc_lint[fe6d62f39905847c]::BuiltinCombinedModuleLateLintPass> as rustc_hir[dbafcb5a748c6cce]::intravisit::Visitor>::visit_expr
  41:     0x7fe620cefb72 - <rustc_lint[fe6d62f39905847c]::late::LateContextAndPass<rustc_lint[fe6d62f39905847c]::BuiltinCombinedModuleLateLintPass> as rustc_hir[dbafcb5a748c6cce]::intravisit::Visitor>::visit_stmt
  42:     0x7fe620d7db1c - rustc_hir[dbafcb5a748c6cce]::intravisit::walk_expr::<rustc_lint[fe6d62f39905847c]::late::LateContextAndPass<rustc_lint[fe6d62f39905847c]::BuiltinCombinedModuleLateLintPass>>
  43:     0x7fe620cef108 - <rustc_lint[fe6d62f39905847c]::late::LateContextAndPass<rustc_lint[fe6d62f39905847c]::BuiltinCombinedModuleLateLintPass> as rustc_hir[dbafcb5a748c6cce]::intravisit::Visitor>::visit_expr
  44:     0x7fe620cefec9 - <rustc_lint[fe6d62f39905847c]::late::LateContextAndPass<rustc_lint[fe6d62f39905847c]::BuiltinCombinedModuleLateLintPass> as rustc_hir[dbafcb5a748c6cce]::intravisit::Visitor>::visit_fn
  45:     0x7fe620d76b0a - rustc_hir[dbafcb5a748c6cce]::intravisit::walk_impl_item::<rustc_lint[fe6d62f39905847c]::late::LateContextAndPass<rustc_lint[fe6d62f39905847c]::BuiltinCombinedModuleLateLintPass>>
  46:     0x7fe620ced0bf - <rustc_lint[fe6d62f39905847c]::late::LateContextAndPass<rustc_lint[fe6d62f39905847c]::BuiltinCombinedModuleLateLintPass> as rustc_hir[dbafcb5a748c6cce]::intravisit::Visitor>::visit_nested_impl_item
  47:     0x7fe620d7f5fd - rustc_hir[dbafcb5a748c6cce]::intravisit::walk_item::<rustc_lint[fe6d62f39905847c]::late::LateContextAndPass<rustc_lint[fe6d62f39905847c]::BuiltinCombinedModuleLateLintPass>>
  48:     0x7fe620cecb72 - <rustc_lint[fe6d62f39905847c]::late::LateContextAndPass<rustc_lint[fe6d62f39905847c]::BuiltinCombinedModuleLateLintPass> as rustc_hir[dbafcb5a748c6cce]::intravisit::Visitor>::visit_nested_item
  49:     0x7fe620d7c29c - rustc_hir[dbafcb5a748c6cce]::intravisit::walk_mod::<rustc_lint[fe6d62f39905847c]::late::LateContextAndPass<rustc_lint[fe6d62f39905847c]::BuiltinCombinedModuleLateLintPass>>
  50:     0x7fe620cf2437 - rustc_lint[fe6d62f39905847c]::late::late_lint_mod::<rustc_lint[fe6d62f39905847c]::BuiltinCombinedModuleLateLintPass>
  51:     0x7fe620d5bafd - rustc_lint[fe6d62f39905847c]::lint_mod
  52:     0x7fe620308f91 - rustc_query_system[2b495aa6dab45fba]::query::plumbing::try_execute_query::<rustc_query_impl[384da77d99fd738d]::queries::lint_mod, rustc_query_impl[384da77d99fd738d]::plumbing::QueryCtxt>
  53:     0x7fe6203c5230 - rustc_query_system[2b495aa6dab45fba]::query::plumbing::get_query::<rustc_query_impl[384da77d99fd738d]::queries::lint_mod, rustc_query_impl[384da77d99fd738d]::plumbing::QueryCtxt, rustc_middle[1be4fd16a1bcb82a]::dep_graph::dep_node::DepKind>
  54:     0x7fe61fffc044 - <rustc_query_impl[384da77d99fd738d]::Queries as rustc_middle[1be4fd16a1bcb82a]::ty::query::QueryEngine>::lint_mod
  55:     0x7fe61e80b6b6 - std[f283601f176d5ebe]::panicking::try::<(), core[78b8f9b0dd37335f]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[9292081428f8b355]::sync::par_for_each_in<&[rustc_hir[dbafcb5a748c6cce]::hir_id::OwnerId], <rustc_middle[1be4fd16a1bcb82a]::hir::map::Map>::par_for_each_module<rustc_lint[fe6d62f39905847c]::late::check_crate<rustc_lint[fe6d62f39905847c]::BuiltinCombinedLateLintPass, rustc_interface[255173142009b065]::passes::analysis::{closure#6}::{closure#1}::{closure#2}::{closure#0}::{closure#0}>::{closure#1}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  56:     0x7fe61e7bca77 - rustc_data_structures[9292081428f8b355]::sync::par_for_each_in::<&[rustc_hir[dbafcb5a748c6cce]::hir_id::OwnerId], <rustc_middle[1be4fd16a1bcb82a]::hir::map::Map>::par_for_each_module<rustc_lint[fe6d62f39905847c]::late::check_crate<rustc_lint[fe6d62f39905847c]::BuiltinCombinedLateLintPass, rustc_interface[255173142009b065]::passes::analysis::{closure#6}::{closure#1}::{closure#2}::{closure#0}::{closure#0}>::{closure#1}::{closure#0}::{closure#0}>::{closure#0}>
  57:     0x7fe61e8894db - <rustc_session[10ea56dedbe0c995]::session::Session>::time::<(), rustc_lint[fe6d62f39905847c]::late::check_crate<rustc_lint[fe6d62f39905847c]::BuiltinCombinedLateLintPass, rustc_interface[255173142009b065]::passes::analysis::{closure#6}::{closure#1}::{closure#2}::{closure#0}::{closure#0}>::{closure#1}::{closure#0}>
  58:     0x7fe61e889610 - <rustc_session[10ea56dedbe0c995]::session::Session>::time::<(), rustc_interface[255173142009b065]::passes::analysis::{closure#6}::{closure#1}::{closure#2}::{closure#0}>
  59:     0x7fe61e80b765 - std[f283601f176d5ebe]::panicking::try::<(), core[78b8f9b0dd37335f]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[255173142009b065]::passes::analysis::{closure#6}::{closure#1}::{closure#2}>>
  60:     0x7fe61e7ccece - <core[78b8f9b0dd37335f]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[255173142009b065]::passes::analysis::{closure#6}::{closure#1}> as core[78b8f9b0dd37335f]::ops::function::FnOnce<()>>::call_once
  61:     0x7fe61e80b886 - std[f283601f176d5ebe]::panicking::try::<(), core[78b8f9b0dd37335f]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[255173142009b065]::passes::analysis::{closure#6}::{closure#1}>>
  62:     0x7fe61e88b0b0 - <rustc_session[10ea56dedbe0c995]::session::Session>::time::<(), rustc_interface[255173142009b065]::passes::analysis::{closure#6}>
  63:     0x7fe61e7e0b4c - rustc_interface[255173142009b065]::passes::analysis
  64:     0x7fe62030612c - rustc_query_system[2b495aa6dab45fba]::query::plumbing::try_execute_query::<rustc_query_impl[384da77d99fd738d]::queries::analysis, rustc_query_impl[384da77d99fd738d]::plumbing::QueryCtxt>
  65:     0x7fe6203c4e81 - rustc_query_system[2b495aa6dab45fba]::query::plumbing::get_query::<rustc_query_impl[384da77d99fd738d]::queries::analysis, rustc_query_impl[384da77d99fd738d]::plumbing::QueryCtxt, rustc_middle[1be4fd16a1bcb82a]::dep_graph::dep_node::DepKind>
  66:     0x7fe61ffdd17a - <rustc_query_impl[384da77d99fd738d]::Queries as rustc_middle[1be4fd16a1bcb82a]::ty::query::QueryEngine>::analysis
  67:     0x7fe61e74979c - <rustc_middle[1be4fd16a1bcb82a]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[d6863e2e44b1d54b]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[78b8f9b0dd37335f]::result::Result<(), rustc_errors[bbd20863eb2e02a9]::ErrorGuaranteed>>
  68:     0x7fe61e748993 - <rustc_interface[255173142009b065]::interface::Compiler>::enter::<rustc_driver_impl[d6863e2e44b1d54b]::run_compiler::{closure#1}::{closure#2}, core[78b8f9b0dd37335f]::result::Result<core[78b8f9b0dd37335f]::option::Option<rustc_interface[255173142009b065]::queries::Linker>, rustc_errors[bbd20863eb2e02a9]::ErrorGuaranteed>>
  69:     0x7fe61e772f1a - rustc_span[965f05b949b72626]::with_source_map::<core[78b8f9b0dd37335f]::result::Result<(), rustc_errors[bbd20863eb2e02a9]::ErrorGuaranteed>, rustc_interface[255173142009b065]::interface::run_compiler<core[78b8f9b0dd37335f]::result::Result<(), rustc_errors[bbd20863eb2e02a9]::ErrorGuaranteed>, rustc_driver_impl[d6863e2e44b1d54b]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  70:     0x7fe61e737c1b - <scoped_tls[98e817fa317f4d9a]::ScopedKey<rustc_span[965f05b949b72626]::SessionGlobals>>::set::<rustc_interface[255173142009b065]::interface::run_compiler<core[78b8f9b0dd37335f]::result::Result<(), rustc_errors[bbd20863eb2e02a9]::ErrorGuaranteed>, rustc_driver_impl[d6863e2e44b1d54b]::run_compiler::{closure#1}>::{closure#0}, core[78b8f9b0dd37335f]::result::Result<(), rustc_errors[bbd20863eb2e02a9]::ErrorGuaranteed>>
  71:     0x7fe61e70d46a - std[f283601f176d5ebe]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[255173142009b065]::util::run_in_thread_pool_with_globals<rustc_interface[255173142009b065]::interface::run_compiler<core[78b8f9b0dd37335f]::result::Result<(), rustc_errors[bbd20863eb2e02a9]::ErrorGuaranteed>, rustc_driver_impl[d6863e2e44b1d54b]::run_compiler::{closure#1}>::{closure#0}, core[78b8f9b0dd37335f]::result::Result<(), rustc_errors[bbd20863eb2e02a9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[78b8f9b0dd37335f]::result::Result<(), rustc_errors[bbd20863eb2e02a9]::ErrorGuaranteed>>
  72:     0x7fe61e7337c6 - std[f283601f176d5ebe]::panicking::try::<core[78b8f9b0dd37335f]::result::Result<(), rustc_errors[bbd20863eb2e02a9]::ErrorGuaranteed>, core[78b8f9b0dd37335f]::panic::unwind_safe::AssertUnwindSafe<<std[f283601f176d5ebe]::thread::Builder>::spawn_unchecked_<rustc_interface[255173142009b065]::util::run_in_thread_pool_with_globals<rustc_interface[255173142009b065]::interface::run_compiler<core[78b8f9b0dd37335f]::result::Result<(), rustc_errors[bbd20863eb2e02a9]::ErrorGuaranteed>, rustc_driver_impl[d6863e2e44b1d54b]::run_compiler::{closure#1}>::{closure#0}, core[78b8f9b0dd37335f]::result::Result<(), rustc_errors[bbd20863eb2e02a9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[78b8f9b0dd37335f]::result::Result<(), rustc_errors[bbd20863eb2e02a9]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  73:     0x7fe61e7112d4 - <<std[f283601f176d5ebe]::thread::Builder>::spawn_unchecked_<rustc_interface[255173142009b065]::util::run_in_thread_pool_with_globals<rustc_interface[255173142009b065]::interface::run_compiler<core[78b8f9b0dd37335f]::result::Result<(), rustc_errors[bbd20863eb2e02a9]::ErrorGuaranteed>, rustc_driver_impl[d6863e2e44b1d54b]::run_compiler::{closure#1}>::{closure#0}, core[78b8f9b0dd37335f]::result::Result<(), rustc_errors[bbd20863eb2e02a9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[78b8f9b0dd37335f]::result::Result<(), rustc_errors[bbd20863eb2e02a9]::ErrorGuaranteed>>::{closure#1} as core[78b8f9b0dd37335f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  74:     0x7fe61dc99bfe - std::sys::unix::thread::Thread::new::thread_start::h6a9b2c451f20553b
  75:     0x7fe61da30b43 - <unknown>
  76:     0x7fe61dac2a00 - <unknown>
  77:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.69.0-nightly (01701774b 2023-02-22) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=v0 -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C split-debuginfo=off -Z unstable-options -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -Z binary-dep-depinfo -Z tls-model=initial-exec -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [lint_mod] linting module `interval`
#1 [analysis] running analysis passes on this crate
error: could not compile `rustc_index`
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:05:45
