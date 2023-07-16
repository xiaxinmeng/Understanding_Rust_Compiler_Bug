
Copying stage0 rustc from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Assembling stage1 compiler (x86_64-unknown-linux-gnu)
Building stage1 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling cc v1.0.69
   Compiling core v0.0.0 (/home/ben/rust/library/core)
   Compiling libc v0.2.126
   Compiling memchr v2.4.1
   Compiling std v0.0.0 (/home/ben/rust/library/std)
   Compiling compiler_builtins v0.1.73
   Compiling unwind v0.0.0 (/home/ben/rust/library/unwind)
   Compiling rustc-std-workspace-core v1.99.0 (/home/ben/rust/library/rustc-std-workspace-core)
error: internal compiler error: compiler/rustc_lint/src/types.rs:796:13: improper_ctypes: Option nonnull optimization not applied?

thread 'rustc' panicked at 'Box<dyn Any>', /home/ben/rust/compiler/rustc_errors/src/lib.rs:1334:9
stack backtrace:
   0:     0x7f70b8f1b8fd - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::he38c7361b94c5d0c
   1:     0x7f70b8f6443e - core::fmt::write::h1a02af67ba355677
   2:     0x7f70b8ed3081 - std::io::Write::write_fmt::h453053024c73966b
   3:     0x7f70b8ee7e55 - std::panicking::default_hook::{{closure}}::hfdf6d0b1b3859d98
   4:     0x7f70b8ee7ab7 - std::panicking::default_hook::h14232858b308420c
   5:     0x7f70b969d3e6 - rustc_driver[bbe82ef0d8e75ea1]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f70b8ee8634 - std::panicking::rust_panic_with_hook::h5215b735b1ea29b6
   7:     0x7f70bbce39f3 - std[a689b2ed93f5df76]::panicking::begin_panic::<rustc_errors[2825527422c22d3]::ExplicitBug>::{closure#0}
   8:     0x7f70bbce15a6 - std[a689b2ed93f5df76]::sys_common::backtrace::__rust_end_short_backtrace::<std[a689b2ed93f5df76]::panicking::begin_panic<rustc_errors[2825527422c22d3]::ExplicitBug>::{closure#0}, !>
   9:     0x7f70b95db396 - std[a689b2ed93f5df76]::panicking::begin_panic::<rustc_errors[2825527422c22d3]::ExplicitBug>
  10:     0x7f70bbb17c26 - std[a689b2ed93f5df76]::panic::panic_any::<rustc_errors[2825527422c22d3]::ExplicitBug>
  11:     0x7f70bbb166e0 - <rustc_errors[2825527422c22d3]::HandlerInner>::bug::<&alloc[9dfe1f9b2c9bdb6]::string::String>
  12:     0x7f70bbb16330 - <rustc_errors[2825527422c22d3]::Handler>::bug::<&alloc[9dfe1f9b2c9bdb6]::string::String>
  13:     0x7f70bbcf2e5a - rustc_middle[8693c79d52a1849d]::ty::context::tls::with_opt::<rustc_middle[8693c79d52a1849d]::util::bug::opt_span_bug_fmt<rustc_span[9e269396fce057fe]::span_encoding::Span>::{closure#0}, ()>
  14:     0x7f70bbcf2ec9 - rustc_middle[8693c79d52a1849d]::util::bug::opt_span_bug_fmt::<rustc_span[9e269396fce057fe]::span_encoding::Span>
  15:     0x7f70b95e1305 - rustc_middle[8693c79d52a1849d]::util::bug::bug_fmt
  16:     0x7f70bb7e90ed - rustc_lint[90eccce51c55190]::types::repr_nullable_ptr
  17:     0x7f70bb7e9fd8 - <rustc_lint[90eccce51c55190]::types::ImproperCTypesVisitor>::check_type_for_ffi
  18:     0x7f70bb7ea768 - <rustc_lint[90eccce51c55190]::types::ImproperCTypesVisitor>::check_type_for_ffi_and_report_errors
  19:     0x7f70bb7eb158 - <rustc_lint[90eccce51c55190]::types::ImproperCTypesVisitor>::check_foreign_fn
  20:     0x7f70bb7fb39d - <rustc_lint[90eccce51c55190]::late::LateContextAndPass<rustc_lint[90eccce51c55190]::BuiltinCombinedModuleLateLintPass> as rustc_hir[4282039d40c377b4]::intravisit::Visitor>::visit_fn
  21:     0x7f70bb854399 - rustc_hir[4282039d40c377b4]::intravisit::walk_item::<rustc_lint[90eccce51c55190]::late::LateContextAndPass<rustc_lint[90eccce51c55190]::BuiltinCombinedModuleLateLintPass>>
  22:     0x7f70bb7f9293 - <rustc_lint[90eccce51c55190]::late::LateContextAndPass<rustc_lint[90eccce51c55190]::BuiltinCombinedModuleLateLintPass> as rustc_hir[4282039d40c377b4]::intravisit::Visitor>::visit_nested_item
  23:     0x7f70bb7fc92d - rustc_lint[90eccce51c55190]::late::late_lint_mod::<rustc_lint[90eccce51c55190]::BuiltinCombinedModuleLateLintPass>
  24:     0x7f70bae29e2b - rustc_query_system[1523598a8d60b49]::query::plumbing::try_execute_query::<rustc_query_impl[320f40d31780e620]::plumbing::QueryCtxt, rustc_query_system[1523598a8d60b49]::query::caches::DefaultCache<rustc_span[9e269396fce057fe]::def_id::LocalDefId, ()>>
  25:     0x7f70baf1a635 - rustc_query_system[1523598a8d60b49]::query::plumbing::get_query::<rustc_query_impl[320f40d31780e620]::queries::lint_mod, rustc_query_impl[320f40d31780e620]::plumbing::QueryCtxt>
  26:     0x7f70b975b0ff - <rustc_middle[8693c79d52a1849d]::hir::map::Map>::for_each_module::<rustc_lint[90eccce51c55190]::late::check_crate<rustc_lint[90eccce51c55190]::BuiltinCombinedLateLintPass, rustc_interface[fa71196b0e93bb1c]::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}::{closure#0}>::{closure#1}::{closure#0}::{closure#0}>
  27:     0x7f70b97f0a00 - <rustc_session[915c3399c980dc57]::session::Session>::time::<(), rustc_lint[90eccce51c55190]::late::check_crate<rustc_lint[90eccce51c55190]::BuiltinCombinedLateLintPass, rustc_interface[fa71196b0e93bb1c]::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}::{closure#0}>::{closure#1}::{closure#0}>
  28:     0x7f70b97f0bf7 - <rustc_session[915c3399c980dc57]::session::Session>::time::<(), rustc_interface[fa71196b0e93bb1c]::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}>
  29:     0x7f70b97ac004 - <core[4c4fb2cd17933ed9]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[fa71196b0e93bb1c]::passes::analysis::{closure#5}::{closure#1}> as core[4c4fb2cd17933ed9]::ops::function::FnOnce<()>>::call_once
  30:     0x7f70b97f34de - <rustc_session[915c3399c980dc57]::session::Session>::time::<(), rustc_interface[fa71196b0e93bb1c]::passes::analysis::{closure#5}>
  31:     0x7f70b97e601c - rustc_interface[fa71196b0e93bb1c]::passes::analysis
  32:     0x7f70bae7bd09 - rustc_query_system[1523598a8d60b49]::query::plumbing::try_execute_query::<rustc_query_impl[320f40d31780e620]::plumbing::QueryCtxt, rustc_query_system[1523598a8d60b49]::query::caches::DefaultCache<(), core[4c4fb2cd17933ed9]::result::Result<(), rustc_errors[2825527422c22d3]::ErrorGuaranteed>>>
  33:     0x7f70baf1a127 - rustc_query_system[1523598a8d60b49]::query::plumbing::get_query::<rustc_query_impl[320f40d31780e620]::queries::analysis, rustc_query_impl[320f40d31780e620]::plumbing::QueryCtxt>
  34:     0x7f70b964f05b - <rustc_interface[fa71196b0e93bb1c]::passes::QueryContext>::enter::<rustc_driver[bbe82ef0d8e75ea1]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[4c4fb2cd17933ed9]::result::Result<(), rustc_errors[2825527422c22d3]::ErrorGuaranteed>>
  35:     0x7f70b962d4b6 - <rustc_interface[fa71196b0e93bb1c]::interface::Compiler>::enter::<rustc_driver[bbe82ef0d8e75ea1]::run_compiler::{closure#1}::{closure#2}, core[4c4fb2cd17933ed9]::result::Result<core[4c4fb2cd17933ed9]::option::Option<rustc_interface[fa71196b0e93bb1c]::queries::Linker>, rustc_errors[2825527422c22d3]::ErrorGuaranteed>>
  36:     0x7f70b969eef3 - rustc_span[9e269396fce057fe]::with_source_map::<core[4c4fb2cd17933ed9]::result::Result<(), rustc_errors[2825527422c22d3]::ErrorGuaranteed>, rustc_interface[fa71196b0e93bb1c]::interface::create_compiler_and_run<core[4c4fb2cd17933ed9]::result::Result<(), rustc_errors[2825527422c22d3]::ErrorGuaranteed>, rustc_driver[bbe82ef0d8e75ea1]::run_compiler::{closure#1}>::{closure#1}>
  37:     0x7f70b962e27a - <scoped_tls[b383e8bf08376368]::ScopedKey<rustc_span[9e269396fce057fe]::SessionGlobals>>::set::<rustc_interface[fa71196b0e93bb1c]::interface::run_compiler<core[4c4fb2cd17933ed9]::result::Result<(), rustc_errors[2825527422c22d3]::ErrorGuaranteed>, rustc_driver[bbe82ef0d8e75ea1]::run_compiler::{closure#1}>::{closure#0}, core[4c4fb2cd17933ed9]::result::Result<(), rustc_errors[2825527422c22d3]::ErrorGuaranteed>>
  38:     0x7f70b968849f - std[a689b2ed93f5df76]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[fa71196b0e93bb1c]::util::run_in_thread_pool_with_globals<rustc_interface[fa71196b0e93bb1c]::interface::run_compiler<core[4c4fb2cd17933ed9]::result::Result<(), rustc_errors[2825527422c22d3]::ErrorGuaranteed>, rustc_driver[bbe82ef0d8e75ea1]::run_compiler::{closure#1}>::{closure#0}, core[4c4fb2cd17933ed9]::result::Result<(), rustc_errors[2825527422c22d3]::ErrorGuaranteed>>::{closure#0}, core[4c4fb2cd17933ed9]::result::Result<(), rustc_errors[2825527422c22d3]::ErrorGuaranteed>>
  39:     0x7f70b9652a19 - <<std[a689b2ed93f5df76]::thread::Builder>::spawn_unchecked_<rustc_interface[fa71196b0e93bb1c]::util::run_in_thread_pool_with_globals<rustc_interface[fa71196b0e93bb1c]::interface::run_compiler<core[4c4fb2cd17933ed9]::result::Result<(), rustc_errors[2825527422c22d3]::ErrorGuaranteed>, rustc_driver[bbe82ef0d8e75ea1]::run_compiler::{closure#1}>::{closure#0}, core[4c4fb2cd17933ed9]::result::Result<(), rustc_errors[2825527422c22d3]::ErrorGuaranteed>>::{closure#0}, core[4c4fb2cd17933ed9]::result::Result<(), rustc_errors[2825527422c22d3]::ErrorGuaranteed>>::{closure#1} as core[4c4fb2cd17933ed9]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  40:     0x7f70b8ef87a3 - std::sys::unix::thread::Thread::new::thread_start::hed61a4e9d6dc74b3
  41:     0x7f70b8c8c54d - <unknown>
  42:     0x7f70b8d11874 - clone
  43:                0x0 - <unknown>

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.63.0-dev running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=10000 -C debuginfo=0 -Z unstable-options -Z randomize-layout -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -Z unstable-options -C split-debuginfo=off -C prefer-dynamic -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -C panic=abort -Z force-unstable-if-unmarked

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [lint_mod] linting module `int::udiv`
#1 [analysis] running analysis passes on this crate
end of query stack
