plain
   Compiling object v0.26.2
   Compiling hashbrown v0.12.3
   Compiling miniz_oxide v0.4.0
   Compiling addr2line v0.16.0
error: internal compiler error: compiler/rustc_middle/src/ty/inhabitedness/mod.rs:229:14: unexpected TyKind
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1502:9
stack backtrace:
stack backtrace:
   0:     0x7fb8d3ad61bb - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h5990f0b80572247e
   1:     0x7fb8d3b3b138 - core::fmt::write::h6d6e96066401bc0f
   2:     0x7fb8d3ac7771 - std::io::Write::write_fmt::hc028ec08a989af23
   3:     0x7fb8d3ad9148 - std::panicking::default_hook::{{closure}}::h1250db69b47083ad
   4:     0x7fb8d3ad8ea5 - std::panicking::default_hook::hde2c59d7684931fe
   5:     0x7fb8d440cab4 - rustc_driver[49417d61fdddd109]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fb8d3ad98f0 - std::panicking::rust_panic_with_hook::h0977d771efbcb7aa
   7:     0x7fb8d6d66323 - std[6b2f5401d24e360d]::panicking::begin_panic::<rustc_errors[7bb1601a2f308775]::ExplicitBug>::{closure#0}
   8:     0x7fb8d6d5f4c6 - std[6b2f5401d24e360d]::sys_common::backtrace::__rust_end_short_backtrace::<std[6b2f5401d24e360d]::panicking::begin_panic<rustc_errors[7bb1601a2f308775]::ExplicitBug>::{closure#0}, !>
   9:     0x7fb8d43bfdf6 - std[6b2f5401d24e360d]::panicking::begin_panic::<rustc_errors[7bb1601a2f308775]::ExplicitBug>
  10:     0x7fb8d6cbbd96 - std[6b2f5401d24e360d]::panic::panic_any::<rustc_errors[7bb1601a2f308775]::ExplicitBug>
  11:     0x7fb8d6cb65d3 - <rustc_errors[7bb1601a2f308775]::HandlerInner>::bug::<&alloc[e7e82c908fb9c817]::string::String>
  12:     0x7fb8d6cb60e0 - <rustc_errors[7bb1601a2f308775]::Handler>::bug::<&alloc[e7e82c908fb9c817]::string::String>
  13:     0x7fb8d6ddeb10 - rustc_middle[e33c4a6161842a61]::ty::context::tls::with_context_opt::<rustc_middle[e33c4a6161842a61]::ty::context::tls::with_opt<rustc_middle[e33c4a6161842a61]::util::bug::opt_span_bug_fmt<rustc_span[680edb1c21557d56]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  14:     0x7fb8d6de85e9 - rustc_middle[e33c4a6161842a61]::util::bug::opt_span_bug_fmt::<rustc_span[680edb1c21557d56]::span_encoding::Span>
  15:     0x7fb8d43c22e5 - rustc_middle[e33c4a6161842a61]::util::bug::bug_fmt
  16:     0x7fb8d6d05ee5 - rustc_middle[e33c4a6161842a61]::ty::inhabitedness::type_uninhabited_from
  17:     0x7fb8d602b76d - rustc_query_system[6a643ac45a3e1242]::query::plumbing::get_query::<rustc_query_impl[5fcc1ee5e32b6aa6]::queries::type_uninhabited_from, rustc_query_impl[5fcc1ee5e32b6aa6]::plumbing::QueryCtxt>
  18:     0x7fb8d5bd5f68 - <rustc_query_impl[5fcc1ee5e32b6aa6]::Queries as rustc_middle[e33c4a6161842a61]::ty::query::QueryEngine>::type_uninhabited_from
  19:     0x7fb8d68ae1dd - <rustc_lint[b4c8bb4ec987291]::builtin::InvalidValue as rustc_lint[b4c8bb4ec987291]::passes::LateLintPass>::check_expr::tys_inhabited::<core[69c2305d6fa5d54f]::iter::adapters::map::Map<core[69c2305d6fa5d54f]::slice::iter::Iter<rustc_middle[e33c4a6161842a61]::ty::FieldDef>, <rustc_lint[b4c8bb4ec987291]::builtin::InvalidValue as rustc_lint[b4c8bb4ec987291]::passes::LateLintPass>::check_expr::ty_find_init_error::{closure#0}::{closure#0}>>
  20:     0x7fb8d68a4df3 - <core[69c2305d6fa5d54f]::slice::iter::Iter<rustc_middle[e33c4a6161842a61]::ty::VariantDef> as core[69c2305d6fa5d54f]::iter::traits::iterator::Iterator>::find_map::<(&rustc_middle[e33c4a6161842a61]::ty::VariantDef, bool), &mut <rustc_lint[b4c8bb4ec987291]::builtin::InvalidValue as rustc_lint[b4c8bb4ec987291]::passes::LateLintPass>::check_expr::ty_find_init_error::{closure#0}>
  21:     0x7fb8d68aefdc - <rustc_lint[b4c8bb4ec987291]::builtin::InvalidValue as rustc_lint[b4c8bb4ec987291]::passes::LateLintPass>::check_expr::ty_find_init_error
  22:     0x7fb8d68ae3b1 - <rustc_lint[b4c8bb4ec987291]::builtin::InvalidValue as rustc_lint[b4c8bb4ec987291]::passes::LateLintPass>::check_expr::variant_find_init_error
  23:     0x7fb8d68aecc2 - <rustc_lint[b4c8bb4ec987291]::builtin::InvalidValue as rustc_lint[b4c8bb4ec987291]::passes::LateLintPass>::check_expr::ty_find_init_error
  24:     0x7fb8d68adeff - <rustc_lint[b4c8bb4ec987291]::builtin::InvalidValue as rustc_lint[b4c8bb4ec987291]::passes::LateLintPass>::check_expr
  25:     0x7fb8d6827ed8 - <rustc_lint[b4c8bb4ec987291]::BuiltinCombinedModuleLateLintPass as rustc_lint[b4c8bb4ec987291]::passes::LateLintPass>::check_expr
  26:     0x7fb8d687d439 - <rustc_lint[b4c8bb4ec987291]::late::LateContextAndPass<rustc_lint[b4c8bb4ec987291]::BuiltinCombinedModuleLateLintPass> as rustc_hir[b5a1179931e60c33]::intravisit::Visitor>::visit_expr
  27:     0x7fb8d68c4bb2 - rustc_hir[b5a1179931e60c33]::intravisit::walk_local::<rustc_lint[b4c8bb4ec987291]::late::LateContextAndPass<rustc_lint[b4c8bb4ec987291]::BuiltinCombinedModuleLateLintPass>>
  28:     0x7fb8d687fba4 - <rustc_lint[b4c8bb4ec987291]::late::LateContextAndPass<rustc_lint[b4c8bb4ec987291]::BuiltinCombinedModuleLateLintPass> as rustc_hir[b5a1179931e60c33]::intravisit::Visitor>::visit_local
  29:     0x7fb8d687de6f - <rustc_lint[b4c8bb4ec987291]::late::LateContextAndPass<rustc_lint[b4c8bb4ec987291]::BuiltinCombinedModuleLateLintPass> as rustc_hir[b5a1179931e60c33]::intravisit::Visitor>::visit_stmt
  30:     0x7fb8d68cd98c - rustc_hir[b5a1179931e60c33]::intravisit::walk_expr::<rustc_lint[b4c8bb4ec987291]::late::LateContextAndPass<rustc_lint[b4c8bb4ec987291]::BuiltinCombinedModuleLateLintPass>>
  31:     0x7fb8d687d444 - <rustc_lint[b4c8bb4ec987291]::late::LateContextAndPass<rustc_lint[b4c8bb4ec987291]::BuiltinCombinedModuleLateLintPass> as rustc_hir[b5a1179931e60c33]::intravisit::Visitor>::visit_expr
  32:     0x7fb8d687d444 - <rustc_lint[b4c8bb4ec987291]::late::LateContextAndPass<rustc_lint[b4c8bb4ec987291]::BuiltinCombinedModuleLateLintPass> as rustc_hir[b5a1179931e60c33]::intravisit::Visitor>::visit_expr
  33:     0x7fb8d687d444 - <rustc_lint[b4c8bb4ec987291]::late::LateContextAndPass<rustc_lint[b4c8bb4ec987291]::BuiltinCombinedModuleLateLintPass> as rustc_hir[b5a1179931e60c33]::intravisit::Visitor>::visit_expr
  34:     0x7fb8d687c363 - <rustc_lint[b4c8bb4ec987291]::late::LateContextAndPass<rustc_lint[b4c8bb4ec987291]::BuiltinCombinedModuleLateLintPass> as rustc_hir[b5a1179931e60c33]::intravisit::Visitor>::visit_nested_body
  35:     0x7fb8d687e170 - <rustc_lint[b4c8bb4ec987291]::late::LateContextAndPass<rustc_lint[b4c8bb4ec987291]::BuiltinCombinedModuleLateLintPass> as rustc_hir[b5a1179931e60c33]::intravisit::Visitor>::visit_fn
  36:     0x7fb8d68cf17b - rustc_hir[b5a1179931e60c33]::intravisit::walk_item::<rustc_lint[b4c8bb4ec987291]::late::LateContextAndPass<rustc_lint[b4c8bb4ec987291]::BuiltinCombinedModuleLateLintPass>>
  37:     0x7fb8d687a9b8 - <rustc_lint[b4c8bb4ec987291]::late::LateContextAndPass<rustc_lint[b4c8bb4ec987291]::BuiltinCombinedModuleLateLintPass> as rustc_hir[b5a1179931e60c33]::intravisit::Visitor>::visit_nested_item
  38:     0x7fb8d68cba4c - rustc_hir[b5a1179931e60c33]::intravisit::walk_mod::<rustc_lint[b4c8bb4ec987291]::late::LateContextAndPass<rustc_lint[b4c8bb4ec987291]::BuiltinCombinedModuleLateLintPass>>
  39:     0x7fb8d688042c - rustc_lint[b4c8bb4ec987291]::late::late_lint_mod::<rustc_lint[b4c8bb4ec987291]::BuiltinCombinedModuleLateLintPass>
  40:     0x7fb8d682352d - rustc_lint[b4c8bb4ec987291]::lint_mod
  41:     0x7fb8d5f34ba9 - rustc_query_system[6a643ac45a3e1242]::query::plumbing::try_execute_query::<rustc_query_impl[5fcc1ee5e32b6aa6]::plumbing::QueryCtxt, rustc_query_system[6a643ac45a3e1242]::query::caches::DefaultCache<rustc_span[680edb1c21557d56]::def_id::LocalDefId, ()>>
  42:     0x7fb8d6057d0c - rustc_query_system[6a643ac45a3e1242]::query::plumbing::get_query::<rustc_query_impl[5fcc1ee5e32b6aa6]::queries::lint_mod, rustc_query_impl[5fcc1ee5e32b6aa6]::plumbing::QueryCtxt>
  43:     0x7fb8d5ba1564 - <rustc_query_impl[5fcc1ee5e32b6aa6]::Queries as rustc_middle[e33c4a6161842a61]::ty::query::QueryEngine>::lint_mod
  44:     0x7fb8d459e894 - <core[69c2305d6fa5d54f]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[4b37639fdaa3743b]::sync::par_for_each_in<&[rustc_hir[b5a1179931e60c33]::hir_id::OwnerId], <rustc_middle[e33c4a6161842a61]::hir::map::Map>::par_for_each_module<rustc_lint[b4c8bb4ec987291]::late::check_crate<rustc_lint[b4c8bb4ec987291]::BuiltinCombinedLateLintPass, rustc_interface[3bb86903555d04ac]::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}::{closure#0}>::{closure#1}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[69c2305d6fa5d54f]::ops::function::FnOnce<()>>::call_once
  45:     0x7fb8d45da8d9 - std[6b2f5401d24e360d]::panicking::try::<(), core[69c2305d6fa5d54f]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[4b37639fdaa3743b]::sync::par_for_each_in<&[rustc_hir[b5a1179931e60c33]::hir_id::OwnerId], <rustc_middle[e33c4a6161842a61]::hir::map::Map>::par_for_each_module<rustc_lint[b4c8bb4ec987291]::late::check_crate<rustc_lint[b4c8bb4ec987291]::BuiltinCombinedLateLintPass, rustc_interface[3bb86903555d04ac]::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}::{closure#0}>::{closure#1}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  46:     0x7fb8d45a9f17 - rustc_data_structures[4b37639fdaa3743b]::sync::par_for_each_in::<&[rustc_hir[b5a1179931e60c33]::hir_id::OwnerId], <rustc_middle[e33c4a6161842a61]::hir::map::Map>::par_for_each_module<rustc_lint[b4c8bb4ec987291]::late::check_crate<rustc_lint[b4c8bb4ec987291]::BuiltinCombinedLateLintPass, rustc_interface[3bb86903555d04ac]::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}::{closure#0}>::{closure#1}::{closure#0}::{closure#0}>::{closure#0}>
  47:     0x7fb8d451623c - <rustc_session[596cdfd9d719c75a]::session::Session>::time::<(), rustc_lint[b4c8bb4ec987291]::late::check_crate<rustc_lint[b4c8bb4ec987291]::BuiltinCombinedLateLintPass, rustc_interface[3bb86903555d04ac]::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}::{closure#0}>::{closure#1}::{closure#0}>
  48:     0x7fb8d4516390 - <rustc_session[596cdfd9d719c75a]::session::Session>::time::<(), rustc_interface[3bb86903555d04ac]::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}>
  49:     0x7fb8d45da9c8 - std[6b2f5401d24e360d]::panicking::try::<(), core[69c2305d6fa5d54f]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[3bb86903555d04ac]::passes::analysis::{closure#5}::{closure#1}::{closure#2}>>
  50:     0x7fb8d459fcd3 - <core[69c2305d6fa5d54f]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[3bb86903555d04ac]::passes::analysis::{closure#5}::{closure#1}> as core[69c2305d6fa5d54f]::ops::function::FnOnce<()>>::call_once
  51:     0x7fb8d45daae9 - std[6b2f5401d24e360d]::panicking::try::<(), core[69c2305d6fa5d54f]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[3bb86903555d04ac]::passes::analysis::{closure#5}::{closure#1}>>
  52:     0x7fb8d45180b5 - <rustc_session[596cdfd9d719c75a]::session::Session>::time::<(), rustc_interface[3bb86903555d04ac]::passes::analysis::{closure#5}>
  53:     0x7fb8d453e8cc - rustc_interface[3bb86903555d04ac]::passes::analysis
  54:     0x7fb8d5f78a70 - rustc_query_system[6a643ac45a3e1242]::query::plumbing::try_execute_query::<rustc_query_impl[5fcc1ee5e32b6aa6]::plumbing::QueryCtxt, rustc_query_system[6a643ac45a3e1242]::query::caches::DefaultCache<(), core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[7bb1601a2f308775]::ErrorGuaranteed>>>
  55:     0x7fb8d6057830 - rustc_query_system[6a643ac45a3e1242]::query::plumbing::get_query::<rustc_query_impl[5fcc1ee5e32b6aa6]::queries::analysis, rustc_query_impl[5fcc1ee5e32b6aa6]::plumbing::QueryCtxt>
  56:     0x7fb8d5b8505a - <rustc_query_impl[5fcc1ee5e32b6aa6]::Queries as rustc_middle[e33c4a6161842a61]::ty::query::QueryEngine>::analysis
  57:     0x7fb8d4471e38 - <rustc_interface[3bb86903555d04ac]::passes::QueryContext>::enter::<rustc_driver[49417d61fdddd109]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[7bb1601a2f308775]::ErrorGuaranteed>>
  58:     0x7fb8d4410bb9 - rustc_interface[3bb86903555d04ac]::interface::create_compiler_and_run::<core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[7bb1601a2f308775]::ErrorGuaranteed>, rustc_driver[49417d61fdddd109]::run_compiler::{closure#1}>
  59:     0x7fb8d4475c6d - std[6b2f5401d24e360d]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[3bb86903555d04ac]::util::run_in_thread_pool_with_globals<rustc_interface[3bb86903555d04ac]::interface::run_compiler<core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[7bb1601a2f308775]::ErrorGuaranteed>, rustc_driver[49417d61fdddd109]::run_compiler::{closure#1}>::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[7bb1601a2f308775]::ErrorGuaranteed>>::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[7bb1601a2f308775]::ErrorGuaranteed>>
  60:     0x7fb8d4423b3e - std[6b2f5401d24e360d]::panic::catch_unwind::<core[69c2305d6fa5d54f]::panic::unwind_safe::AssertUnwindSafe<<std[6b2f5401d24e360d]::thread::Builder>::spawn_unchecked_<rustc_interface[3bb86903555d04ac]::util::run_in_thread_pool_with_globals<rustc_interface[3bb86903555d04ac]::interface::run_compiler<core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[7bb1601a2f308775]::ErrorGuaranteed>, rustc_driver[49417d61fdddd109]::run_compiler::{closure#1}>::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[7bb1601a2f308775]::ErrorGuaranteed>>::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[7bb1601a2f308775]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[7bb1601a2f308775]::ErrorGuaranteed>>
  61:     0x7fb8d44790ff - <<std[6b2f5401d24e360d]::thread::Builder>::spawn_unchecked_<rustc_interface[3bb86903555d04ac]::util::run_in_thread_pool_with_globals<rustc_interface[3bb86903555d04ac]::interface::run_compiler<core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[7bb1601a2f308775]::ErrorGuaranteed>, rustc_driver[49417d61fdddd109]::run_compiler::{closure#1}>::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[7bb1601a2f308775]::ErrorGuaranteed>>::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[7bb1601a2f308775]::ErrorGuaranteed>>::{closure#1} as core[69c2305d6fa5d54f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  62:     0x7fb8d3ae5555 - std::sys::unix::thread::Thread::new::thread_start::ha211867b314ecdb6
  63:     0x7fb8d3887b43 - <unknown>
  64:     0x7fb8d3919a00 - <unknown>
  65:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.66.0-nightly (e36fc3eb6 2022-10-04) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type dylib --crate-type rlib -C prefer-dynamic -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [type_uninhabited_from] computing the inhabitedness of `ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: UserFacing, constness: NotConst }, value: extern "C" fn() }`
#1 [lint_mod] linting module `sys::unix::stack_overflow::imp`
#2 [analysis] running analysis passes on this crate
error: could not compile `std`
Build completed unsuccessfully in 0:03:35
