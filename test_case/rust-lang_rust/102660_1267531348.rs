plain
   Compiling object v0.26.2
   Compiling std_detect v0.1.5 (/checkout/library/stdarch/crates/std_detect)
   Compiling miniz_oxide v0.4.0
   Compiling addr2line v0.16.0
error: internal compiler error: compiler/rustc_middle/src/ty/inhabitedness/mod.rs:121:14: unexpected TyKind, use `Ty::uninhabited_predicate`
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1502:9
stack backtrace:
stack backtrace:
   0:     0x7f0ab85331bb - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h5990f0b80572247e
   1:     0x7f0ab8598138 - core::fmt::write::h6d6e96066401bc0f
   2:     0x7f0ab8524771 - std::io::Write::write_fmt::hc028ec08a989af23
   3:     0x7f0ab8536148 - std::panicking::default_hook::{{closure}}::h1250db69b47083ad
   4:     0x7f0ab8535ea5 - std::panicking::default_hook::hde2c59d7684931fe
   5:     0x7f0ab8e6aaa4 - rustc_driver[49417d61fdddd109]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f0ab85368f0 - std::panicking::rust_panic_with_hook::h0977d771efbcb7aa
   7:     0x7f0abb7f2133 - std[6b2f5401d24e360d]::panicking::begin_panic::<rustc_errors[7bb1601a2f308775]::ExplicitBug>::{closure#0}
   8:     0x7f0abb7ebec6 - std[6b2f5401d24e360d]::sys_common::backtrace::__rust_end_short_backtrace::<std[6b2f5401d24e360d]::panicking::begin_panic<rustc_errors[7bb1601a2f308775]::ExplicitBug>::{closure#0}, !>
   9:     0x7f0ab8e18e66 - std[6b2f5401d24e360d]::panicking::begin_panic::<rustc_errors[7bb1601a2f308775]::ExplicitBug>
  10:     0x7f0abb81bcb6 - std[6b2f5401d24e360d]::panic::panic_any::<rustc_errors[7bb1601a2f308775]::ExplicitBug>
  11:     0x7f0abb816503 - <rustc_errors[7bb1601a2f308775]::HandlerInner>::bug::<&alloc[e7e82c908fb9c817]::string::String>
  12:     0x7f0abb816010 - <rustc_errors[7bb1601a2f308775]::Handler>::bug::<&alloc[e7e82c908fb9c817]::string::String>
  13:     0x7f0abb85e040 - rustc_middle[e33c4a6161842a61]::ty::context::tls::with_context_opt::<rustc_middle[e33c4a6161842a61]::ty::context::tls::with_opt<rustc_middle[e33c4a6161842a61]::util::bug::opt_span_bug_fmt<rustc_span[680edb1c21557d56]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  14:     0x7f0abb867b09 - rustc_middle[e33c4a6161842a61]::util::bug::opt_span_bug_fmt::<rustc_span[680edb1c21557d56]::span_encoding::Span>
  15:     0x7f0ab8e202d5 - rustc_middle[e33c4a6161842a61]::util::bug::bug_fmt
  16:     0x7f0abb7be052 - rustc_middle[e33c4a6161842a61]::ty::inhabitedness::uninhabited_predicate
  17:     0x7f0abaa8a84e - rustc_query_system[6a643ac45a3e1242]::query::plumbing::get_query::<rustc_query_impl[5fcc1ee5e32b6aa6]::queries::uninhabited_predicate, rustc_query_impl[5fcc1ee5e32b6aa6]::plumbing::QueryCtxt>
  18:     0x7f0aba634e5f - <rustc_query_impl[5fcc1ee5e32b6aa6]::Queries as rustc_middle[e33c4a6161842a61]::ty::query::QueryEngine>::uninhabited_predicate
  19:     0x7f0abb287104 - <rustc_lint[b4c8bb4ec987291]::builtin::InvalidValue as rustc_lint[b4c8bb4ec987291]::passes::LateLintPass>::check_expr::tys_inhabited::<core[69c2305d6fa5d54f]::iter::adapters::map::Map<core[69c2305d6fa5d54f]::slice::iter::Iter<rustc_middle[e33c4a6161842a61]::ty::FieldDef>, <rustc_lint[b4c8bb4ec987291]::builtin::InvalidValue as rustc_lint[b4c8bb4ec987291]::passes::LateLintPass>::check_expr::ty_find_init_error::{closure#0}::{closure#0}>>
  20:     0x7f0abb27dd63 - <core[69c2305d6fa5d54f]::slice::iter::Iter<rustc_middle[e33c4a6161842a61]::ty::VariantDef> as core[69c2305d6fa5d54f]::iter::traits::iterator::Iterator>::find_map::<(&rustc_middle[e33c4a6161842a61]::ty::VariantDef, bool), &mut <rustc_lint[b4c8bb4ec987291]::builtin::InvalidValue as rustc_lint[b4c8bb4ec987291]::passes::LateLintPass>::check_expr::ty_find_init_error::{closure#0}>
  21:     0x7f0abb287ebc - <rustc_lint[b4c8bb4ec987291]::builtin::InvalidValue as rustc_lint[b4c8bb4ec987291]::passes::LateLintPass>::check_expr::ty_find_init_error
  22:     0x7f0abb287291 - <rustc_lint[b4c8bb4ec987291]::builtin::InvalidValue as rustc_lint[b4c8bb4ec987291]::passes::LateLintPass>::check_expr::variant_find_init_error
  23:     0x7f0abb287ba2 - <rustc_lint[b4c8bb4ec987291]::builtin::InvalidValue as rustc_lint[b4c8bb4ec987291]::passes::LateLintPass>::check_expr::ty_find_init_error
  24:     0x7f0abb286e7f - <rustc_lint[b4c8bb4ec987291]::builtin::InvalidValue as rustc_lint[b4c8bb4ec987291]::passes::LateLintPass>::check_expr
  25:     0x7f0abb2c6188 - <rustc_lint[b4c8bb4ec987291]::BuiltinCombinedModuleLateLintPass as rustc_lint[b4c8bb4ec987291]::passes::LateLintPass>::check_expr
  26:     0x7f0abb31a389 - <rustc_lint[b4c8bb4ec987291]::late::LateContextAndPass<rustc_lint[b4c8bb4ec987291]::BuiltinCombinedModuleLateLintPass> as rustc_hir[b5a1179931e60c33]::intravisit::Visitor>::visit_expr
  27:     0x7f0abb2eaa92 - rustc_hir[b5a1179931e60c33]::intravisit::walk_local::<rustc_lint[b4c8bb4ec987291]::late::LateContextAndPass<rustc_lint[b4c8bb4ec987291]::BuiltinCombinedModuleLateLintPass>>
  28:     0x7f0abb31caf4 - <rustc_lint[b4c8bb4ec987291]::late::LateContextAndPass<rustc_lint[b4c8bb4ec987291]::BuiltinCombinedModuleLateLintPass> as rustc_hir[b5a1179931e60c33]::intravisit::Visitor>::visit_local
  29:     0x7f0abb31adbf - <rustc_lint[b4c8bb4ec987291]::late::LateContextAndPass<rustc_lint[b4c8bb4ec987291]::BuiltinCombinedModuleLateLintPass> as rustc_hir[b5a1179931e60c33]::intravisit::Visitor>::visit_stmt
  30:     0x7f0abb2f386c - rustc_hir[b5a1179931e60c33]::intravisit::walk_expr::<rustc_lint[b4c8bb4ec987291]::late::LateContextAndPass<rustc_lint[b4c8bb4ec987291]::BuiltinCombinedModuleLateLintPass>>
  31:     0x7f0abb31a394 - <rustc_lint[b4c8bb4ec987291]::late::LateContextAndPass<rustc_lint[b4c8bb4ec987291]::BuiltinCombinedModuleLateLintPass> as rustc_hir[b5a1179931e60c33]::intravisit::Visitor>::visit_expr
  32:     0x7f0abb31a394 - <rustc_lint[b4c8bb4ec987291]::late::LateContextAndPass<rustc_lint[b4c8bb4ec987291]::BuiltinCombinedModuleLateLintPass> as rustc_hir[b5a1179931e60c33]::intravisit::Visitor>::visit_expr
  33:     0x7f0abb31a394 - <rustc_lint[b4c8bb4ec987291]::late::LateContextAndPass<rustc_lint[b4c8bb4ec987291]::BuiltinCombinedModuleLateLintPass> as rustc_hir[b5a1179931e60c33]::intravisit::Visitor>::visit_expr
  34:     0x7f0abb3192b3 - <rustc_lint[b4c8bb4ec987291]::late::LateContextAndPass<rustc_lint[b4c8bb4ec987291]::BuiltinCombinedModuleLateLintPass> as rustc_hir[b5a1179931e60c33]::intravisit::Visitor>::visit_nested_body
  35:     0x7f0abb31b0c0 - <rustc_lint[b4c8bb4ec987291]::late::LateContextAndPass<rustc_lint[b4c8bb4ec987291]::BuiltinCombinedModuleLateLintPass> as rustc_hir[b5a1179931e60c33]::intravisit::Visitor>::visit_fn
  36:     0x7f0abb2f505b - rustc_hir[b5a1179931e60c33]::intravisit::walk_item::<rustc_lint[b4c8bb4ec987291]::late::LateContextAndPass<rustc_lint[b4c8bb4ec987291]::BuiltinCombinedModuleLateLintPass>>
  37:     0x7f0abb317908 - <rustc_lint[b4c8bb4ec987291]::late::LateContextAndPass<rustc_lint[b4c8bb4ec987291]::BuiltinCombinedModuleLateLintPass> as rustc_hir[b5a1179931e60c33]::intravisit::Visitor>::visit_nested_item
  38:     0x7f0abb2f192c - rustc_hir[b5a1179931e60c33]::intravisit::walk_mod::<rustc_lint[b4c8bb4ec987291]::late::LateContextAndPass<rustc_lint[b4c8bb4ec987291]::BuiltinCombinedModuleLateLintPass>>
  39:     0x7f0abb31d37c - rustc_lint[b4c8bb4ec987291]::late::late_lint_mod::<rustc_lint[b4c8bb4ec987291]::BuiltinCombinedModuleLateLintPass>
  40:     0x7f0abb2c171d - rustc_lint[b4c8bb4ec987291]::lint_mod
  41:     0x7f0aba993d89 - rustc_query_system[6a643ac45a3e1242]::query::plumbing::try_execute_query::<rustc_query_impl[5fcc1ee5e32b6aa6]::plumbing::QueryCtxt, rustc_query_system[6a643ac45a3e1242]::query::caches::DefaultCache<rustc_span[680edb1c21557d56]::def_id::LocalDefId, ()>>
  42:     0x7f0abaab6cec - rustc_query_system[6a643ac45a3e1242]::query::plumbing::get_query::<rustc_query_impl[5fcc1ee5e32b6aa6]::queries::lint_mod, rustc_query_impl[5fcc1ee5e32b6aa6]::plumbing::QueryCtxt>
  43:     0x7f0aba600474 - <rustc_query_impl[5fcc1ee5e32b6aa6]::Queries as rustc_middle[e33c4a6161842a61]::ty::query::QueryEngine>::lint_mod
  44:     0x7f0ab8ffc964 - <core[69c2305d6fa5d54f]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[4b37639fdaa3743b]::sync::par_for_each_in<&[rustc_hir[b5a1179931e60c33]::hir_id::OwnerId], <rustc_middle[e33c4a6161842a61]::hir::map::Map>::par_for_each_module<rustc_lint[b4c8bb4ec987291]::late::check_crate<rustc_lint[b4c8bb4ec987291]::BuiltinCombinedLateLintPass, rustc_interface[3bb86903555d04ac]::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}::{closure#0}>::{closure#1}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[69c2305d6fa5d54f]::ops::function::FnOnce<()>>::call_once
  45:     0x7f0ab90389a9 - std[6b2f5401d24e360d]::panicking::try::<(), core[69c2305d6fa5d54f]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[4b37639fdaa3743b]::sync::par_for_each_in<&[rustc_hir[b5a1179931e60c33]::hir_id::OwnerId], <rustc_middle[e33c4a6161842a61]::hir::map::Map>::par_for_each_module<rustc_lint[b4c8bb4ec987291]::late::check_crate<rustc_lint[b4c8bb4ec987291]::BuiltinCombinedLateLintPass, rustc_interface[3bb86903555d04ac]::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}::{closure#0}>::{closure#1}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  46:     0x7f0ab9007fe7 - rustc_data_structures[4b37639fdaa3743b]::sync::par_for_each_in::<&[rustc_hir[b5a1179931e60c33]::hir_id::OwnerId], <rustc_middle[e33c4a6161842a61]::hir::map::Map>::par_for_each_module<rustc_lint[b4c8bb4ec987291]::late::check_crate<rustc_lint[b4c8bb4ec987291]::BuiltinCombinedLateLintPass, rustc_interface[3bb86903555d04ac]::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}::{closure#0}>::{closure#1}::{closure#0}::{closure#0}>::{closure#0}>
  47:     0x7f0ab8f7422c - <rustc_session[596cdfd9d719c75a]::session::Session>::time::<(), rustc_lint[b4c8bb4ec987291]::late::check_crate<rustc_lint[b4c8bb4ec987291]::BuiltinCombinedLateLintPass, rustc_interface[3bb86903555d04ac]::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}::{closure#0}>::{closure#1}::{closure#0}>
  48:     0x7f0ab8f74380 - <rustc_session[596cdfd9d719c75a]::session::Session>::time::<(), rustc_interface[3bb86903555d04ac]::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}>
  49:     0x7f0ab9038a98 - std[6b2f5401d24e360d]::panicking::try::<(), core[69c2305d6fa5d54f]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[3bb86903555d04ac]::passes::analysis::{closure#5}::{closure#1}::{closure#2}>>
  50:     0x7f0ab8ffdda3 - <core[69c2305d6fa5d54f]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[3bb86903555d04ac]::passes::analysis::{closure#5}::{closure#1}> as core[69c2305d6fa5d54f]::ops::function::FnOnce<()>>::call_once
  51:     0x7f0ab9038bb9 - std[6b2f5401d24e360d]::panicking::try::<(), core[69c2305d6fa5d54f]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[3bb86903555d04ac]::passes::analysis::{closure#5}::{closure#1}>>
  52:     0x7f0ab8f760a5 - <rustc_session[596cdfd9d719c75a]::session::Session>::time::<(), rustc_interface[3bb86903555d04ac]::passes::analysis::{closure#5}>
  53:     0x7f0ab8f9c8bc - rustc_interface[3bb86903555d04ac]::passes::analysis
  54:     0x7f0aba9d7c50 - rustc_query_system[6a643ac45a3e1242]::query::plumbing::try_execute_query::<rustc_query_impl[5fcc1ee5e32b6aa6]::plumbing::QueryCtxt, rustc_query_system[6a643ac45a3e1242]::query::caches::DefaultCache<(), core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[7bb1601a2f308775]::ErrorGuaranteed>>>
  55:     0x7f0abaab6810 - rustc_query_system[6a643ac45a3e1242]::query::plumbing::get_query::<rustc_query_impl[5fcc1ee5e32b6aa6]::queries::analysis, rustc_query_impl[5fcc1ee5e32b6aa6]::plumbing::QueryCtxt>
  56:     0x7f0aba5e3f6a - <rustc_query_impl[5fcc1ee5e32b6aa6]::Queries as rustc_middle[e33c4a6161842a61]::ty::query::QueryEngine>::analysis
  57:     0x7f0ab8ecfe28 - <rustc_interface[3bb86903555d04ac]::passes::QueryContext>::enter::<rustc_driver[49417d61fdddd109]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[7bb1601a2f308775]::ErrorGuaranteed>>
  58:     0x7f0ab8e6eba9 - rustc_interface[3bb86903555d04ac]::interface::create_compiler_and_run::<core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[7bb1601a2f308775]::ErrorGuaranteed>, rustc_driver[49417d61fdddd109]::run_compiler::{closure#1}>
  59:     0x7f0ab8ed3c5d - std[6b2f5401d24e360d]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[3bb86903555d04ac]::util::run_in_thread_pool_with_globals<rustc_interface[3bb86903555d04ac]::interface::run_compiler<core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[7bb1601a2f308775]::ErrorGuaranteed>, rustc_driver[49417d61fdddd109]::run_compiler::{closure#1}>::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[7bb1601a2f308775]::ErrorGuaranteed>>::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[7bb1601a2f308775]::ErrorGuaranteed>>
  60:     0x7f0ab8e81b2e - std[6b2f5401d24e360d]::panic::catch_unwind::<core[69c2305d6fa5d54f]::panic::unwind_safe::AssertUnwindSafe<<std[6b2f5401d24e360d]::thread::Builder>::spawn_unchecked_<rustc_interface[3bb86903555d04ac]::util::run_in_thread_pool_with_globals<rustc_interface[3bb86903555d04ac]::interface::run_compiler<core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[7bb1601a2f308775]::ErrorGuaranteed>, rustc_driver[49417d61fdddd109]::run_compiler::{closure#1}>::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[7bb1601a2f308775]::ErrorGuaranteed>>::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[7bb1601a2f308775]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[7bb1601a2f308775]::ErrorGuaranteed>>
  61:     0x7f0ab8ed70ef - <<std[6b2f5401d24e360d]::thread::Builder>::spawn_unchecked_<rustc_interface[3bb86903555d04ac]::util::run_in_thread_pool_with_globals<rustc_interface[3bb86903555d04ac]::interface::run_compiler<core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[7bb1601a2f308775]::ErrorGuaranteed>, rustc_driver[49417d61fdddd109]::run_compiler::{closure#1}>::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[7bb1601a2f308775]::ErrorGuaranteed>>::{closure#0}, core[69c2305d6fa5d54f]::result::Result<(), rustc_errors[7bb1601a2f308775]::ErrorGuaranteed>>::{closure#1} as core[69c2305d6fa5d54f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  62:     0x7f0ab8542555 - std::sys::unix::thread::Thread::new::thread_start::ha211867b314ecdb6
  63:     0x7f0ab82e4b43 - <unknown>
  64:     0x7f0ab8376a00 - <unknown>
  65:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.66.0-nightly (5a4a6147a 2022-10-04) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type dylib --crate-type rlib -C prefer-dynamic -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [uninhabited_predicate] computing the uninhabited predicate of `extern "C" fn()`
#1 [lint_mod] linting module `sys::unix::stack_overflow::imp`
#2 [analysis] running analysis passes on this crate
error: could not compile `std`
Build completed unsuccessfully in 0:04:31
