plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:0fdabd83e1d3faaa8e9cfd7c00031e3a92997344)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---
   Compiling rustc_session v0.0.0 (/checkout/compiler/rustc_session)
   Compiling rustc_attr v0.0.0 (/checkout/compiler/rustc_attr)
   Compiling rustc_query_system v0.0.0 (/checkout/compiler/rustc_query_system)
   Compiling rustc_parse v0.0.0 (/checkout/compiler/rustc_parse)
thread 'rustc' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 30, kind: ReadOnlyFilesystem, message: "Read-only file system" }', compiler/rustc_lint/src/internal.rs:589:14
   0:     0x7f1a415fc316 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h632e586e6b5ff8ad
   0:     0x7f1a415fc316 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h632e586e6b5ff8ad
   1:     0x7f1a4166b938 - core::fmt::write::h370ceefc764332d1
   2:     0x7f1a415efe91 - std::io::Write::write_fmt::he252c6343a326f36
   3:     0x7f1a415fc0e5 - std::sys_common::backtrace::print::hc9ca29e21e351ce2
   4:     0x7f1a415ff527 - std::panicking::default_hook::{{closure}}::h6e9050399229cae4
   5:     0x7f1a415ff212 - std::panicking::default_hook::hd8215bd97bc006bf
   6:     0x7f1a4207a9e5 - rustc_driver_impl[cef884b36dba0828]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f1a415ffe20 - std::panicking::rust_panic_with_hook::hf23b4c1fefb74ed9
   8:     0x7f1a415ffb69 - std::panicking::begin_panic_handler::{{closure}}::h8cb6437bc9f6ef06
   9:     0x7f1a415fc88c - std::sys_common::backtrace::__rust_end_short_backtrace::h30ec68f3e2af9f3f
  10:     0x7f1a415ff832 - rust_begin_unwind
  11:     0x7f1a415b5ff3 - core::panicking::panic_fmt::ha6d05d75fb2b7d9f
  12:     0x7f1a415b64f3 - core::result::unwrap_failed::h1d70961d481f4043
  13:     0x7f1a4469af9d - <core[78b8f9b0dd37335f]::result::Result<std[6bcdc65a9a3a0dbc]::fs::File, std[6bcdc65a9a3a0dbc]::io::error::Error>>::unwrap
  14:     0x7f1a446a2e66 - <rustc_lint[b7ce72301ca3496d]::internal::Diagnostics as rustc_lint[b7ce72301ca3496d]::passes::EarlyLintPass>::check_stmt
  15:     0x7f1a44676ef0 - <rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass as rustc_lint[b7ce72301ca3496d]::passes::EarlyLintPass>::check_stmt
  16:     0x7f1a42197109 - <rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass> as rustc_ast[b107f0e06304be77]::visit::Visitor>::visit_stmt
  17:     0x7f1a421e8e2c - rustc_ast[b107f0e06304be77]::visit::walk_expr::<rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass>>
  18:     0x7f1a42195ed5 - <rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass> as rustc_ast[b107f0e06304be77]::visit::Visitor>::visit_expr
  19:     0x7f1a421e7564 - rustc_ast[b107f0e06304be77]::visit::walk_arm::<rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass>>
  20:     0x7f1a4219cf15 - <rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass> as rustc_ast[b107f0e06304be77]::visit::Visitor>::visit_arm
  21:     0x7f1a421e8d7c - rustc_ast[b107f0e06304be77]::visit::walk_expr::<rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass>>
  22:     0x7f1a42195ed5 - <rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass> as rustc_ast[b107f0e06304be77]::visit::Visitor>::visit_expr
  23:     0x7f1a4219735d - <rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass> as rustc_ast[b107f0e06304be77]::visit::Visitor>::visit_stmt
  24:     0x7f1a421e8b4c - rustc_ast[b107f0e06304be77]::visit::walk_expr::<rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass>>
  25:     0x7f1a42195ed5 - <rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass> as rustc_ast[b107f0e06304be77]::visit::Visitor>::visit_expr
  26:     0x7f1a421e9209 - rustc_ast[b107f0e06304be77]::visit::walk_expr::<rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass>>
  27:     0x7f1a42195ed5 - <rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass> as rustc_ast[b107f0e06304be77]::visit::Visitor>::visit_expr
  28:     0x7f1a421e9209 - rustc_ast[b107f0e06304be77]::visit::walk_expr::<rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass>>
  29:     0x7f1a42195ed5 - <rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass> as rustc_ast[b107f0e06304be77]::visit::Visitor>::visit_expr
  30:     0x7f1a421e9209 - rustc_ast[b107f0e06304be77]::visit::walk_expr::<rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass>>
  31:     0x7f1a42195ed5 - <rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass> as rustc_ast[b107f0e06304be77]::visit::Visitor>::visit_expr
  32:     0x7f1a421e1083 - rustc_ast[b107f0e06304be77]::visit::walk_local::<rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass>>
  33:     0x7f1a42197a95 - <rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass> as rustc_ast[b107f0e06304be77]::visit::Visitor>::visit_local
  34:     0x7f1a4219735d - <rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass> as rustc_ast[b107f0e06304be77]::visit::Visitor>::visit_stmt
  35:     0x7f1a421e8e2c - rustc_ast[b107f0e06304be77]::visit::walk_expr::<rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass>>
  36:     0x7f1a42195ed5 - <rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass> as rustc_ast[b107f0e06304be77]::visit::Visitor>::visit_expr
  37:     0x7f1a4219c90a - <rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass> as rustc_ast[b107f0e06304be77]::visit::Visitor>::visit_fn
  38:     0x7f1a421e8db4 - rustc_ast[b107f0e06304be77]::visit::walk_expr::<rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass>>
  39:     0x7f1a42195ed5 - <rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass> as rustc_ast[b107f0e06304be77]::visit::Visitor>::visit_expr
  40:     0x7f1a421e8a60 - rustc_ast[b107f0e06304be77]::visit::walk_expr::<rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass>>
  41:     0x7f1a42195ed5 - <rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass> as rustc_ast[b107f0e06304be77]::visit::Visitor>::visit_expr
  42:     0x7f1a421e9209 - rustc_ast[b107f0e06304be77]::visit::walk_expr::<rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass>>
  43:     0x7f1a42195ed5 - <rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass> as rustc_ast[b107f0e06304be77]::visit::Visitor>::visit_expr
  44:     0x7f1a421e1083 - rustc_ast[b107f0e06304be77]::visit::walk_local::<rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass>>
  45:     0x7f1a42197a95 - <rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass> as rustc_ast[b107f0e06304be77]::visit::Visitor>::visit_local
  46:     0x7f1a4219735d - <rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass> as rustc_ast[b107f0e06304be77]::visit::Visitor>::visit_stmt
  47:     0x7f1a421e8bec - rustc_ast[b107f0e06304be77]::visit::walk_expr::<rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass>>
  48:     0x7f1a42195ed5 - <rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass> as rustc_ast[b107f0e06304be77]::visit::Visitor>::visit_expr
  49:     0x7f1a4219735d - <rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass> as rustc_ast[b107f0e06304be77]::visit::Visitor>::visit_stmt
  50:     0x7f1a421e617c - rustc_ast[b107f0e06304be77]::visit::walk_fn::<rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass>>
  51:     0x7f1a4219c90a - <rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass> as rustc_ast[b107f0e06304be77]::visit::Visitor>::visit_fn
  52:     0x7f1a421e2c46 - rustc_ast[b107f0e06304be77]::visit::walk_assoc_item::<rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass>>
  53:     0x7f1a4219a933 - <rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass> as rustc_ast[b107f0e06304be77]::visit::Visitor>::visit_assoc_item
  54:     0x7f1a421eabc5 - rustc_ast[b107f0e06304be77]::visit::walk_item::<rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass>>
  55:     0x7f1a421967d5 - <rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass> as rustc_ast[b107f0e06304be77]::visit::Visitor>::visit_item
  56:     0x7f1a421ea7e0 - rustc_ast[b107f0e06304be77]::visit::walk_item::<rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass>>
  57:     0x7f1a421967d5 - <rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass> as rustc_ast[b107f0e06304be77]::visit::Visitor>::visit_item
  58:     0x7f1a421ea7e0 - rustc_ast[b107f0e06304be77]::visit::walk_item::<rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass>>
  59:     0x7f1a421967d5 - <rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass> as rustc_ast[b107f0e06304be77]::visit::Visitor>::visit_item
  60:     0x7f1a421e0d1f - rustc_ast[b107f0e06304be77]::visit::walk_crate::<rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass>>
  61:     0x7f1a42194a37 - rustc_lint[b7ce72301ca3496d]::early::check_ast_node_inner::<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass, &rustc_ast[b107f0e06304be77]::ast::Crate>
  62:     0x7f1a4218ff54 - rustc_lint[b7ce72301ca3496d]::early::check_ast_node::<rustc_lint[b7ce72301ca3496d]::BuiltinCombinedEarlyLintPass, &rustc_ast[b107f0e06304be77]::ast::Crate>
  63:     0x7f1a421effcc - <rustc_session[c0c3bf20e99957b4]::session::Session>::time::<(), rustc_interface[28bf18330852cac6]::passes::configure_and_expand::{closure#8}>
  64:     0x7f1a42152854 - rustc_interface[28bf18330852cac6]::passes::resolver_for_lowering
  65:     0x7f1a43d345a4 - rustc_query_system[4c668902890aadd3]::query::plumbing::try_execute_query::<rustc_query_impl[1d25ef77222e30e]::queries::resolver_for_lowering, rustc_query_impl[1d25ef77222e30e]::plumbing::QueryCtxt>
  66:     0x7f1a43989ce7 - <rustc_query_impl[1d25ef77222e30e]::Queries as rustc_middle[bac06bdcb773b985]::ty::query::QueryEngine>::resolver_for_lowering
  67:     0x7f1a420c3438 - <rustc_middle[bac06bdcb773b985]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[cef884b36dba0828]::run_compiler::{closure#1}::{closure#2}::{closure#2}, &rustc_data_structures[9a08392373d27857]::steal::Steal<(rustc_middle[bac06bdcb773b985]::ty::ResolverAstLowering, alloc[6ebded5921d18d02]::rc::Rc<rustc_ast[b107f0e06304be77]::ast::Crate>)>>
  68:     0x7f1a420c1fa9 - rustc_span[fa9b0981e413c70c]::with_source_map::<core[78b8f9b0dd37335f]::result::Result<(), rustc_span[fa9b0981e413c70c]::ErrorGuaranteed>, rustc_interface[28bf18330852cac6]::interface::run_compiler<core[78b8f9b0dd37335f]::result::Result<(), rustc_span[fa9b0981e413c70c]::ErrorGuaranteed>, rustc_driver_impl[cef884b36dba0828]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  69:     0x7f1a420b139b - <scoped_tls[98e817fa317f4d9a]::ScopedKey<rustc_span[fa9b0981e413c70c]::SessionGlobals>>::set::<rustc_interface[28bf18330852cac6]::interface::run_compiler<core[78b8f9b0dd37335f]::result::Result<(), rustc_span[fa9b0981e413c70c]::ErrorGuaranteed>, rustc_driver_impl[cef884b36dba0828]::run_compiler::{closure#1}>::{closure#0}, core[78b8f9b0dd37335f]::result::Result<(), rustc_span[fa9b0981e413c70c]::ErrorGuaranteed>>
  70:     0x7f1a42086b1a - std[6bcdc65a9a3a0dbc]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[28bf18330852cac6]::util::run_in_thread_pool_with_globals<rustc_interface[28bf18330852cac6]::interface::run_compiler<core[78b8f9b0dd37335f]::result::Result<(), rustc_span[fa9b0981e413c70c]::ErrorGuaranteed>, rustc_driver_impl[cef884b36dba0828]::run_compiler::{closure#1}>::{closure#0}, core[78b8f9b0dd37335f]::result::Result<(), rustc_span[fa9b0981e413c70c]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[78b8f9b0dd37335f]::result::Result<(), rustc_span[fa9b0981e413c70c]::ErrorGuaranteed>>
  71:     0x7f1a420a5cc6 - std[6bcdc65a9a3a0dbc]::panicking::try::<core[78b8f9b0dd37335f]::result::Result<(), rustc_span[fa9b0981e413c70c]::ErrorGuaranteed>, core[78b8f9b0dd37335f]::panic::unwind_safe::AssertUnwindSafe<<std[6bcdc65a9a3a0dbc]::thread::Builder>::spawn_unchecked_<rustc_interface[28bf18330852cac6]::util::run_in_thread_pool_with_globals<rustc_interface[28bf18330852cac6]::interface::run_compiler<core[78b8f9b0dd37335f]::result::Result<(), rustc_span[fa9b0981e413c70c]::ErrorGuaranteed>, rustc_driver_impl[cef884b36dba0828]::run_compiler::{closure#1}>::{closure#0}, core[78b8f9b0dd37335f]::result::Result<(), rustc_span[fa9b0981e413c70c]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[78b8f9b0dd37335f]::result::Result<(), rustc_span[fa9b0981e413c70c]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  72:     0x7f1a4208a984 - <<std[6bcdc65a9a3a0dbc]::thread::Builder>::spawn_unchecked_<rustc_interface[28bf18330852cac6]::util::run_in_thread_pool_with_globals<rustc_interface[28bf18330852cac6]::interface::run_compiler<core[78b8f9b0dd37335f]::result::Result<(), rustc_span[fa9b0981e413c70c]::ErrorGuaranteed>, rustc_driver_impl[cef884b36dba0828]::run_compiler::{closure#1}>::{closure#0}, core[78b8f9b0dd37335f]::result::Result<(), rustc_span[fa9b0981e413c70c]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[78b8f9b0dd37335f]::result::Result<(), rustc_span[fa9b0981e413c70c]::ErrorGuaranteed>>::{closure#1} as core[78b8f9b0dd37335f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  73:     0x7f1a4160cdae - std::sys::unix::thread::Thread::new::thread_start::h3fc22fc7c79eeb9d
  74:     0x7f1a413a6b43 - <unknown>
  75:     0x7f1a41438a00 - <unknown>
  76:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.70.0-nightly (03b6026e7 2023-03-05) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=v0 -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C split-debuginfo=off -Z unstable-options -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -Z binary-dep-depinfo -Z tls-model=initial-exec -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [resolver_for_lowering] getting the resolver for lowering
#0 [resolver_for_lowering] getting the resolver for lowering
end of query stack
thread 'rustc' panicked at 'Found a `push` without a `pop`.', compiler/rustc_lint/src/levels.rs:512:9
   0:     0x7f1a415fc316 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h632e586e6b5ff8ad
   0:     0x7f1a415fc316 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h632e586e6b5ff8ad
   1:     0x7f1a4166b938 - core::fmt::write::h370ceefc764332d1
   2:     0x7f1a415efe91 - std::io::Write::write_fmt::he252c6343a326f36
   3:     0x7f1a415fc0e5 - std::sys_common::backtrace::print::hc9ca29e21e351ce2
   4:     0x7f1a415ff527 - std::panicking::default_hook::{{closure}}::h6e9050399229cae4
   5:     0x7f1a415ff212 - std::panicking::default_hook::hd8215bd97bc006bf
   6:     0x7f1a4207a9e5 - rustc_driver_impl[cef884b36dba0828]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f1a415ffe20 - std::panicking::rust_panic_with_hook::hf23b4c1fefb74ed9
   8:     0x7f1a415ffb22 - std::panicking::begin_panic_handler::{{closure}}::h8cb6437bc9f6ef06
   9:     0x7f1a415fc88c - std::sys_common::backtrace::__rust_end_short_backtrace::h30ec68f3e2af9f3f
  10:     0x7f1a415ff832 - rust_begin_unwind
  11:     0x7f1a415b5ff3 - core::panicking::panic_fmt::ha6d05d75fb2b7d9f
  12:     0x7f1a446ca359 - <rustc_lint[b7ce72301ca3496d]::levels::BuilderPush as core[78b8f9b0dd37335f]::ops::drop::Drop>::drop
  13:     0x7f1a42197470 - <rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass> as rustc_ast[b107f0e06304be77]::visit::Visitor>::visit_stmt
  14:     0x7f1a421e8e2c - rustc_ast[b107f0e06304be77]::visit::walk_expr::<rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass>>
  15:     0x7f1a42195ed5 - <rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass> as rustc_ast[b107f0e06304be77]::visit::Visitor>::visit_expr
  16:     0x7f1a421e7564 - rustc_ast[b107f0e06304be77]::visit::walk_arm::<rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass>>
  17:     0x7f1a4219cf15 - <rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass> as rustc_ast[b107f0e06304be77]::visit::Visitor>::visit_arm
  18:     0x7f1a421e8d7c - rustc_ast[b107f0e06304be77]::visit::walk_expr::<rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass>>
  19:     0x7f1a42195ed5 - <rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass> as rustc_ast[b107f0e06304be77]::visit::Visitor>::visit_expr
  20:     0x7f1a4219735d - <rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass> as rustc_ast[b107f0e06304be77]::visit::Visitor>::visit_stmt
  21:     0x7f1a421e8b4c - rustc_ast[b107f0e06304be77]::visit::walk_expr::<rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass>>
  22:     0x7f1a42195ed5 - <rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass> as rustc_ast[b107f0e06304be77]::visit::Visitor>::visit_expr
  23:     0x7f1a421e9209 - rustc_ast[b107f0e06304be77]::visit::walk_expr::<rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass>>
  24:     0x7f1a42195ed5 - <rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass> as rustc_ast[b107f0e06304be77]::visit::Visitor>::visit_expr
  25:     0x7f1a421e9209 - rustc_ast[b107f0e06304be77]::visit::walk_expr::<rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass>>
  26:     0x7f1a42195ed5 - <rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass> as rustc_ast[b107f0e06304be77]::visit::Visitor>::visit_expr
  27:     0x7f1a421e9209 - rustc_ast[b107f0e06304be77]::visit::walk_expr::<rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass>>
  28:     0x7f1a42195ed5 - <rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass> as rustc_ast[b107f0e06304be77]::visit::Visitor>::visit_expr
  29:     0x7f1a421e1083 - rustc_ast[b107f0e06304be77]::visit::walk_local::<rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass>>
  30:     0x7f1a42197a95 - <rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass> as rustc_ast[b107f0e06304be77]::visit::Visitor>::visit_local
  31:     0x7f1a4219735d - <rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass> as rustc_ast[b107f0e06304be77]::visit::Visitor>::visit_stmt
  32:     0x7f1a421e8e2c - rustc_ast[b107f0e06304be77]::visit::walk_expr::<rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass>>
  33:     0x7f1a42195ed5 - <rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass> as rustc_ast[b107f0e06304be77]::visit::Visitor>::visit_expr
  34:     0x7f1a4219c90a - <rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass> as rustc_ast[b107f0e06304be77]::visit::Visitor>::visit_fn
  35:     0x7f1a421e8db4 - rustc_ast[b107f0e06304be77]::visit::walk_expr::<rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass>>
  36:     0x7f1a42195ed5 - <rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass> as rustc_ast[b107f0e06304be77]::visit::Visitor>::visit_expr
  37:     0x7f1a421e8a60 - rustc_ast[b107f0e06304be77]::visit::walk_expr::<rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass>>
  38:     0x7f1a42195ed5 - <rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass> as rustc_ast[b107f0e06304be77]::visit::Visitor>::visit_expr
  39:     0x7f1a421e9209 - rustc_ast[b107f0e06304be77]::visit::walk_expr::<rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass>>
  40:     0x7f1a42195ed5 - <rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass> as rustc_ast[b107f0e06304be77]::visit::Visitor>::visit_expr
  41:     0x7f1a421e1083 - rustc_ast[b107f0e06304be77]::visit::walk_local::<rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass>>
  42:     0x7f1a42197a95 - <rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass> as rustc_ast[b107f0e06304be77]::visit::Visitor>::visit_local
  43:     0x7f1a4219735d - <rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass> as rustc_ast[b107f0e06304be77]::visit::Visitor>::visit_stmt
  44:     0x7f1a421e8bec - rustc_ast[b107f0e06304be77]::visit::walk_expr::<rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass>>
  45:     0x7f1a42195ed5 - <rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass> as rustc_ast[b107f0e06304be77]::visit::Visitor>::visit_expr
  46:     0x7f1a4219735d - <rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass> as rustc_ast[b107f0e06304be77]::visit::Visitor>::visit_stmt
  47:     0x7f1a421e617c - rustc_ast[b107f0e06304be77]::visit::walk_fn::<rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass>>
  48:     0x7f1a4219c90a - <rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass> as rustc_ast[b107f0e06304be77]::visit::Visitor>::visit_fn
  49:     0x7f1a421e2c46 - rustc_ast[b107f0e06304be77]::visit::walk_assoc_item::<rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass>>
  50:     0x7f1a4219a933 - <rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass> as rustc_ast[b107f0e06304be77]::visit::Visitor>::visit_assoc_item
  51:     0x7f1a421eabc5 - rustc_ast[b107f0e06304be77]::visit::walk_item::<rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass>>
  52:     0x7f1a421967d5 - <rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass> as rustc_ast[b107f0e06304be77]::visit::Visitor>::visit_item
  53:     0x7f1a421ea7e0 - rustc_ast[b107f0e06304be77]::visit::walk_item::<rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass>>
  54:     0x7f1a421967d5 - <rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass> as rustc_ast[b107f0e06304be77]::visit::Visitor>::visit_item
  55:     0x7f1a421ea7e0 - rustc_ast[b107f0e06304be77]::visit::walk_item::<rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass>>
  56:     0x7f1a421967d5 - <rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass> as rustc_ast[b107f0e06304be77]::visit::Visitor>::visit_item
  57:     0x7f1a421e0d1f - rustc_ast[b107f0e06304be77]::visit::walk_crate::<rustc_lint[b7ce72301ca3496d]::early::EarlyContextAndPass<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass>>
  58:     0x7f1a42194a37 - rustc_lint[b7ce72301ca3496d]::early::check_ast_node_inner::<rustc_lint[b7ce72301ca3496d]::early::RuntimeCombinedEarlyLintPass, &rustc_ast[b107f0e06304be77]::ast::Crate>
  59:     0x7f1a4218ff54 - rustc_lint[b7ce72301ca3496d]::early::check_ast_node::<rustc_lint[b7ce72301ca3496d]::BuiltinCombinedEarlyLintPass, &rustc_ast[b107f0e06304be77]::ast::Crate>
  60:     0x7f1a421effcc - <rustc_session[c0c3bf20e99957b4]::session::Session>::time::<(), rustc_interface[28bf18330852cac6]::passes::configure_and_expand::{closure#8}>
  61:     0x7f1a42152854 - rustc_interface[28bf18330852cac6]::passes::resolver_for_lowering
  62:     0x7f1a43d345a4 - rustc_query_system[4c668902890aadd3]::query::plumbing::try_execute_query::<rustc_query_impl[1d25ef77222e30e]::queries::resolver_for_lowering, rustc_query_impl[1d25ef77222e30e]::plumbing::QueryCtxt>
  63:     0x7f1a43989ce7 - <rustc_query_impl[1d25ef77222e30e]::Queries as rustc_middle[bac06bdcb773b985]::ty::query::QueryEngine>::resolver_for_lowering
  64:     0x7f1a420c3438 - <rustc_middle[bac06bdcb773b985]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[cef884b36dba0828]::run_compiler::{closure#1}::{closure#2}::{closure#2}, &rustc_data_structures[9a08392373d27857]::steal::Steal<(rustc_middle[bac06bdcb773b985]::ty::ResolverAstLowering, alloc[6ebded5921d18d02]::rc::Rc<rustc_ast[b107f0e06304be77]::ast::Crate>)>>
  65:     0x7f1a420c1fa9 - rustc_span[fa9b0981e413c70c]::with_source_map::<core[78b8f9b0dd37335f]::result::Result<(), rustc_span[fa9b0981e413c70c]::ErrorGuaranteed>, rustc_interface[28bf18330852cac6]::interface::run_compiler<core[78b8f9b0dd37335f]::result::Result<(), rustc_span[fa9b0981e413c70c]::ErrorGuaranteed>, rustc_driver_impl[cef884b36dba0828]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  66:     0x7f1a420b139b - <scoped_tls[98e817fa317f4d9a]::ScopedKey<rustc_span[fa9b0981e413c70c]::SessionGlobals>>::set::<rustc_interface[28bf18330852cac6]::interface::run_compiler<core[78b8f9b0dd37335f]::result::Result<(), rustc_span[fa9b0981e413c70c]::ErrorGuaranteed>, rustc_driver_impl[cef884b36dba0828]::run_compiler::{closure#1}>::{closure#0}, core[78b8f9b0dd37335f]::result::Result<(), rustc_span[fa9b0981e413c70c]::ErrorGuaranteed>>
  67:     0x7f1a42086b1a - std[6bcdc65a9a3a0dbc]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[28bf18330852cac6]::util::run_in_thread_pool_with_globals<rustc_interface[28bf18330852cac6]::interface::run_compiler<core[78b8f9b0dd37335f]::result::Result<(), rustc_span[fa9b0981e413c70c]::ErrorGuaranteed>, rustc_driver_impl[cef884b36dba0828]::run_compiler::{closure#1}>::{closure#0}, core[78b8f9b0dd37335f]::result::Result<(), rustc_span[fa9b0981e413c70c]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[78b8f9b0dd37335f]::result::Result<(), rustc_span[fa9b0981e413c70c]::ErrorGuaranteed>>
  68:     0x7f1a420a5cc6 - std[6bcdc65a9a3a0dbc]::panicking::try::<core[78b8f9b0dd37335f]::result::Result<(), rustc_span[fa9b0981e413c70c]::ErrorGuaranteed>, core[78b8f9b0dd37335f]::panic::unwind_safe::AssertUnwindSafe<<std[6bcdc65a9a3a0dbc]::thread::Builder>::spawn_unchecked_<rustc_interface[28bf18330852cac6]::util::run_in_thread_pool_with_globals<rustc_interface[28bf18330852cac6]::interface::run_compiler<core[78b8f9b0dd37335f]::result::Result<(), rustc_span[fa9b0981e413c70c]::ErrorGuaranteed>, rustc_driver_impl[cef884b36dba0828]::run_compiler::{closure#1}>::{closure#0}, core[78b8f9b0dd37335f]::result::Result<(), rustc_span[fa9b0981e413c70c]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[78b8f9b0dd37335f]::result::Result<(), rustc_span[fa9b0981e413c70c]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  69:     0x7f1a4208a984 - <<std[6bcdc65a9a3a0dbc]::thread::Builder>::spawn_unchecked_<rustc_interface[28bf18330852cac6]::util::run_in_thread_pool_with_globals<rustc_interface[28bf18330852cac6]::interface::run_compiler<core[78b8f9b0dd37335f]::result::Result<(), rustc_span[fa9b0981e413c70c]::ErrorGuaranteed>, rustc_driver_impl[cef884b36dba0828]::run_compiler::{closure#1}>::{closure#0}, core[78b8f9b0dd37335f]::result::Result<(), rustc_span[fa9b0981e413c70c]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[78b8f9b0dd37335f]::result::Result<(), rustc_span[fa9b0981e413c70c]::ErrorGuaranteed>>::{closure#1} as core[78b8f9b0dd37335f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  70:     0x7f1a4160cdae - std::sys::unix::thread::Thread::new::thread_start::h3fc22fc7c79eeb9d
  71:     0x7f1a413a6b43 - <unknown>
  72:     0x7f1a41438a00 - <unknown>
  73:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.70.0-nightly (03b6026e7 2023-03-05) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=v0 -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C split-debuginfo=off -Z unstable-options -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -Z binary-dep-depinfo -Z tls-model=initial-exec -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [resolver_for_lowering] getting the resolver for lowering
#0 [resolver_for_lowering] getting the resolver for lowering
end of query stack
thread panicked while panicking. aborting.
rustc exited with signal: 6 (SIGABRT) (core dumped)

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_parse --edition=2021 compiler/rustc_parse/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=on -Zunstable-options --check-cfg 'values(feature)' --check-cfg 'names()' --check-cfg 'values()' -C metadata=527f8f9ccc7ecbd4 -C extra-filename=-527f8f9ccc7ecbd4 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-58e73d63c2b388a7.rmeta --extern rustc_ast=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_ast-ecdcdc94c54dd06c.rmeta --extern rustc_ast_pretty=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_ast_pretty-a986291f85aa318c.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-3b4a305ec4604c3b.rmeta --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-8d230c8627106f92.rmeta --extern rustc_feature=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_feature-81a887e242ce2795.rmeta --extern rustc_lexer=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_lexer-2c79123fb8421fdf.rmeta --extern rustc_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/librustc_macros-6a1bd39eff123ebb.so --extern rustc_session=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_session-0f06a62154a39b37.rmeta --extern rustc_span=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_span-cd8f11717f314f47.rmeta --extern thin_vec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libthin_vec-64b7681adf052d22.rmeta --extern tracing=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libtracing-b8bba7b3513922c4.rmeta --extern unicode_normalization=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libunicode_normalization-cfc02ef92abcda8e.rmeta --extern unicode_width=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libunicode_width-74f4b4a0bf8a088e.rmeta -Csymbol-mangling-version=v0 -Zunstable-options '--check-cfg=values(bootstrap)' '--check-cfg=values(parallel_compiler)' '--check-cfg=values(no_btreemap_remove_entry)' '--check-cfg=values(crossbeam_loom)' '--check-cfg=values(span_locations)' '--check-cfg=values(rustix_use_libc)' '--check-cfg=values(emulate_second_only_system)' -Zmacro-backtrace -Clink-args=-Wl,-z,origin '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Csplit-debuginfo=off -Zunstable-options '-Wrustc::internal' -Cprefer-dynamic -Cllvm-args=-import-instr-limit=10 -Z binary-dep-depinfo -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/build/psm-293ea67a0f9b6eb8/out` (exit status: 254)
Build completed unsuccessfully in 0:05:50
