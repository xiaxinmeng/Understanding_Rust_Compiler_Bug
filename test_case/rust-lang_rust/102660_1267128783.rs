plain
[RUSTC-TIMING] addr2line test:false 0.527
[RUSTC-TIMING] gimli test:false 4.639
[RUSTC-TIMING] object test:false 4.650
[RUSTC-TIMING] core test:false 25.893
error: internal compiler error: compiler/rustc_middle/src/ty/inhabitedness/mod.rs:229:14: unexpected TyKind
thread 'rustc' panicked at 'Box<dyn Any>', /rustc/b6d0f43b6b1f1c9b4e24f9364b3747d49fd0ac97/compiler/rustc_errors/src/lib.rs:1502:9
stack backtrace:
stack backtrace:
   0:     0x7f35d3832220 - std::backtrace_rs::backtrace::libunwind::trace::h0d2af8cdbee73aad
                               at /rustc/b6d0f43b6b1f1c9b4e24f9364b3747d49fd0ac97/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   1:     0x7f35d3832220 - std::backtrace_rs::backtrace::trace_unsynchronized::h490f7a50b18336a0
                               at /rustc/b6d0f43b6b1f1c9b4e24f9364b3747d49fd0ac97/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7f35d3832220 - std::sys_common::backtrace::_print_fmt::h32e83ed2eedc9f9e
                               at /rustc/b6d0f43b6b1f1c9b4e24f9364b3747d49fd0ac97/library/std/src/sys_common/backtrace.rs:66:5
   3:     0x7f35d3832220 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h7f1c393dafcf63fe
                               at /rustc/b6d0f43b6b1f1c9b4e24f9364b3747d49fd0ac97/library/std/src/sys_common/backtrace.rs:45:22
   4:     0x7f35d388cfee - core::fmt::write::h861db7d15f3ad9bc
                               at /rustc/b6d0f43b6b1f1c9b4e24f9364b3747d49fd0ac97/library/core/src/fmt/mod.rs:1209:17
   5:     0x7f35d3822ea5 - std::io::Write::write_fmt::hb26930ff0767d3e2
                               at /rustc/b6d0f43b6b1f1c9b4e24f9364b3747d49fd0ac97/library/std/src/io/mod.rs:1679:15
   6:     0x7f35d3834fc3 - std::sys_common::backtrace::_print::h41f659671a3d0d78
                               at /rustc/b6d0f43b6b1f1c9b4e24f9364b3747d49fd0ac97/library/std/src/sys_common/backtrace.rs:48:5
   7:     0x7f35d3834fc3 - std::sys_common::backtrace::print::h34a7f6cdc3c94db7
                               at /rustc/b6d0f43b6b1f1c9b4e24f9364b3747d49fd0ac97/library/std/src/sys_common/backtrace.rs:35:9
   8:     0x7f35d3834fc3 - std::panicking::default_hook::{{closure}}::h9911fb04eab34ac5
                               at /rustc/b6d0f43b6b1f1c9b4e24f9364b3747d49fd0ac97/library/std/src/panicking.rs:267:22
   9:     0x7f35d3834c9a - std::panicking::default_hook::h22317dbaafe5f111
                               at /rustc/b6d0f43b6b1f1c9b4e24f9364b3747d49fd0ac97/library/std/src/panicking.rs:286:9
  10:     0x7f35d0bda266 - rustc_driver[e6df3c77098fb729]::DEFAULT_HOOK::{closure#0}::{closure#0}
  11:     0x7f35d38357fb - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::h484ee9b7ff9bd422
                               at /rustc/b6d0f43b6b1f1c9b4e24f9364b3747d49fd0ac97/library/alloc/src/boxed.rs:1952:9
  12:     0x7f35d38357fb - std::panicking::rust_panic_with_hook::h765f741a773bb961
                               at /rustc/b6d0f43b6b1f1c9b4e24f9364b3747d49fd0ac97/library/std/src/panicking.rs:673:13
  13:     0x7f35d302c313 - std[aebb45f19603b3b]::panicking::begin_panic::<rustc_errors[a385eb38ab49152]::ExplicitBug>::{closure#0}
  14:     0x7f35d3026c96 - std[aebb45f19603b3b]::sys_common::backtrace::__rust_end_short_backtrace::<std[aebb45f19603b3b]::panicking::begin_panic<rustc_errors[a385eb38ab49152]::ExplicitBug>::{closure#0}, !>
  15:     0x7f35d3026ac6 - std[aebb45f19603b3b]::panicking::begin_panic::<rustc_errors[a385eb38ab49152]::ExplicitBug>
  16:     0x7f35d3026ab6 - std[aebb45f19603b3b]::panic::panic_any::<rustc_errors[a385eb38ab49152]::ExplicitBug>
  17:     0x7f35d301c7a3 - <rustc_errors[a385eb38ab49152]::HandlerInner>::bug::<&alloc[436baeb795ae8640]::string::String>
  18:     0x7f35d301bfb0 - <rustc_errors[a385eb38ab49152]::Handler>::bug::<&alloc[436baeb795ae8640]::string::String>
  19:     0x7f35d30a84f0 - rustc_middle[35a2341265cc919f]::ty::context::tls::with_context_opt::<rustc_middle[35a2341265cc919f]::ty::context::tls::with_opt<rustc_middle[35a2341265cc919f]::util::bug::opt_span_bug_fmt<rustc_span[a05d955188b2abc7]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  20:     0x7f35d30a92c9 - rustc_middle[35a2341265cc919f]::util::bug::opt_span_bug_fmt::<rustc_span[a05d955188b2abc7]::span_encoding::Span>
  21:     0x7f35d30a9245 - rustc_middle[35a2341265cc919f]::util::bug::bug_fmt
  22:     0x7f35d30ed1b9 - rustc_middle[35a2341265cc919f]::ty::inhabitedness::type_uninhabited_from
  23:     0x7f35d211499d - rustc_query_system[95385a2e0f52738e]::query::plumbing::try_execute_query::<rustc_query_impl[d4f1b3433e4ccd2c]::plumbing::QueryCtxt, rustc_query_system[95385a2e0f52738e]::query::caches::DefaultCache<rustc_middle[35a2341265cc919f]::ty::ParamEnvAnd<rustc_middle[35a2341265cc919f]::ty::Ty>, rustc_middle[35a2341265cc919f]::ty::inhabitedness::def_id_forest::DefIdForest>>
  24:     0x7f35d2243ae4 - rustc_query_system[95385a2e0f52738e]::query::plumbing::get_query::<rustc_query_impl[d4f1b3433e4ccd2c]::queries::type_uninhabited_from, rustc_query_impl[d4f1b3433e4ccd2c]::plumbing::QueryCtxt>
  25:     0x7f35d2448e63 - <rustc_query_impl[d4f1b3433e4ccd2c]::Queries as rustc_middle[35a2341265cc919f]::ty::query::QueryEngine>::type_uninhabited_from
  26:     0x7f35d2c67a20 - <rustc_lint[adf3804abbfdeb66]::builtin::InvalidValue as rustc_lint[adf3804abbfdeb66]::passes::LateLintPass>::check_expr::tys_inhabited::<core[6134bbfab4c35619]::iter::adapters::map::Map<core[6134bbfab4c35619]::slice::iter::Iter<rustc_middle[35a2341265cc919f]::ty::FieldDef>, <rustc_lint[adf3804abbfdeb66]::builtin::InvalidValue as rustc_lint[adf3804abbfdeb66]::passes::LateLintPass>::check_expr::ty_find_init_error::{closure#0}::{closure#0}>>
  27:     0x7f35d2c5e28b - <core[6134bbfab4c35619]::slice::iter::Iter<rustc_middle[35a2341265cc919f]::ty::VariantDef> as core[6134bbfab4c35619]::iter::traits::iterator::Iterator>::find_map::<(&rustc_middle[35a2341265cc919f]::ty::VariantDef, bool), &mut <rustc_lint[adf3804abbfdeb66]::builtin::InvalidValue as rustc_lint[adf3804abbfdeb66]::passes::LateLintPass>::check_expr::ty_find_init_error::{closure#0}>
  28:     0x7f35d2c68b0f - <rustc_lint[adf3804abbfdeb66]::builtin::InvalidValue as rustc_lint[adf3804abbfdeb66]::passes::LateLintPass>::check_expr::ty_find_init_error
  29:     0x7f35d2c67bbf - <rustc_lint[adf3804abbfdeb66]::builtin::InvalidValue as rustc_lint[adf3804abbfdeb66]::passes::LateLintPass>::check_expr::variant_find_init_error
  30:     0x7f35d2c687e1 - <rustc_lint[adf3804abbfdeb66]::builtin::InvalidValue as rustc_lint[adf3804abbfdeb66]::passes::LateLintPass>::check_expr::ty_find_init_error
  31:     0x7f35d2c67584 - <rustc_lint[adf3804abbfdeb66]::builtin::InvalidValue as rustc_lint[adf3804abbfdeb66]::passes::LateLintPass>::check_expr
  32:     0x7f35d2bda1f2 - <rustc_lint[adf3804abbfdeb66]::BuiltinCombinedModuleLateLintPass as rustc_lint[adf3804abbfdeb66]::passes::LateLintPass>::check_expr
  33:     0x7f35d2bffa4b - rustc_hir[854dfe8484ba389]::intravisit::walk_local::<rustc_lint[adf3804abbfdeb66]::late::LateContextAndPass<rustc_lint[adf3804abbfdeb66]::BuiltinCombinedModuleLateLintPass>>
  34:     0x7f35d2c0ebb1 - rustc_hir[854dfe8484ba389]::intravisit::walk_stmt::<rustc_lint[adf3804abbfdeb66]::late::LateContextAndPass<rustc_lint[adf3804abbfdeb66]::BuiltinCombinedModuleLateLintPass>>
  35:     0x7f35d2bff641 - rustc_hir[854dfe8484ba389]::intravisit::walk_block::<rustc_lint[adf3804abbfdeb66]::late::LateContextAndPass<rustc_lint[adf3804abbfdeb66]::BuiltinCombinedModuleLateLintPass>>
  36:     0x7f35d2c52f39 - <rustc_lint[adf3804abbfdeb66]::late::LateContextAndPass<rustc_lint[adf3804abbfdeb66]::BuiltinCombinedModuleLateLintPass> as rustc_hir[854dfe8484ba389]::intravisit::Visitor>::visit_expr
  37:     0x7f35d2bff69a - rustc_hir[854dfe8484ba389]::intravisit::walk_block::<rustc_lint[adf3804abbfdeb66]::late::LateContextAndPass<rustc_lint[adf3804abbfdeb66]::BuiltinCombinedModuleLateLintPass>>
  38:     0x7f35d2c0a856 - rustc_hir[854dfe8484ba389]::intravisit::walk_body::<rustc_lint[adf3804abbfdeb66]::late::LateContextAndPass<rustc_lint[adf3804abbfdeb66]::BuiltinCombinedModuleLateLintPass>>
  39:     0x7f35d2c52af3 - <rustc_lint[adf3804abbfdeb66]::late::LateContextAndPass<rustc_lint[adf3804abbfdeb66]::BuiltinCombinedModuleLateLintPass> as rustc_hir[854dfe8484ba389]::intravisit::Visitor>::visit_nested_body
  40:     0x7f35d2c5347d - <rustc_lint[adf3804abbfdeb66]::late::LateContextAndPass<rustc_lint[adf3804abbfdeb66]::BuiltinCombinedModuleLateLintPass> as rustc_hir[854dfe8484ba389]::intravisit::Visitor>::visit_fn
  41:     0x7f35d2c0d53a - rustc_hir[854dfe8484ba389]::intravisit::walk_item::<rustc_lint[adf3804abbfdeb66]::late::LateContextAndPass<rustc_lint[adf3804abbfdeb66]::BuiltinCombinedModuleLateLintPass>>
  42:     0x7f35d2c51c98 - <rustc_lint[adf3804abbfdeb66]::late::LateContextAndPass<rustc_lint[adf3804abbfdeb66]::BuiltinCombinedModuleLateLintPass> as rustc_hir[854dfe8484ba389]::intravisit::Visitor>::visit_nested_item
  43:     0x7f35d2c546bd - rustc_lint[adf3804abbfdeb66]::late::late_lint_mod::<rustc_lint[adf3804abbfdeb66]::BuiltinCombinedModuleLateLintPass>
  44:     0x7f35d2bd405d - rustc_lint[adf3804abbfdeb66]::lint_mod
  45:     0x7f35d215b5c0 - rustc_query_system[95385a2e0f52738e]::query::plumbing::try_execute_query::<rustc_query_impl[d4f1b3433e4ccd2c]::plumbing::QueryCtxt, rustc_query_system[95385a2e0f52738e]::query::caches::DefaultCache<rustc_span[a05d955188b2abc7]::def_id::LocalDefId, ()>>
  46:     0x7f35d224b8bb - rustc_query_system[95385a2e0f52738e]::query::plumbing::get_query::<rustc_query_impl[d4f1b3433e4ccd2c]::queries::lint_mod, rustc_query_impl[d4f1b3433e4ccd2c]::plumbing::QueryCtxt>
  47:     0x7f35d0c76f7b - rustc_data_structures[7fad5114b2a72fe2]::sync::join::<rustc_lint[adf3804abbfdeb66]::late::check_crate<rustc_lint[adf3804abbfdeb66]::BuiltinCombinedLateLintPass, rustc_interface[3a5ae1b4f89bc2a4]::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}::{closure#0}>::{closure#0}, rustc_lint[adf3804abbfdeb66]::late::check_crate<rustc_lint[adf3804abbfdeb66]::BuiltinCombinedLateLintPass, rustc_interface[3a5ae1b4f89bc2a4]::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}::{closure#0}>::{closure#1}, (), ()>
  48:     0x7f35d0c69450 - <rustc_session[a7029a46616df422]::session::Session>::time::<(), rustc_interface[3a5ae1b4f89bc2a4]::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}>
  49:     0x7f35d0cbd09e - <core[6134bbfab4c35619]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[3a5ae1b4f89bc2a4]::passes::analysis::{closure#5}::{closure#1}> as core[6134bbfab4c35619]::ops::function::FnOnce<()>>::call_once
  50:     0x7f35d0c6b962 - <rustc_session[a7029a46616df422]::session::Session>::time::<(), rustc_interface[3a5ae1b4f89bc2a4]::passes::analysis::{closure#5}>
  51:     0x7f35d0ca9b9c - rustc_interface[3a5ae1b4f89bc2a4]::passes::analysis
  52:     0x7f35d21d0587 - rustc_query_system[95385a2e0f52738e]::query::plumbing::try_execute_query::<rustc_query_impl[d4f1b3433e4ccd2c]::plumbing::QueryCtxt, rustc_query_system[95385a2e0f52738e]::query::caches::DefaultCache<(), core[6134bbfab4c35619]::result::Result<(), rustc_errors[a385eb38ab49152]::ErrorGuaranteed>>>
  53:     0x7f35d224b3f0 - rustc_query_system[95385a2e0f52738e]::query::plumbing::get_query::<rustc_query_impl[d4f1b3433e4ccd2c]::queries::analysis, rustc_query_impl[d4f1b3433e4ccd2c]::plumbing::QueryCtxt>
  54:     0x7f35d0bc3b33 - <rustc_interface[3a5ae1b4f89bc2a4]::passes::QueryContext>::enter::<rustc_driver[e6df3c77098fb729]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[6134bbfab4c35619]::result::Result<(), rustc_errors[a385eb38ab49152]::ErrorGuaranteed>>
  55:     0x7f35d0b55202 - rustc_interface[3a5ae1b4f89bc2a4]::interface::create_compiler_and_run::<core[6134bbfab4c35619]::result::Result<(), rustc_errors[a385eb38ab49152]::ErrorGuaranteed>, rustc_driver[e6df3c77098fb729]::run_compiler::{closure#1}>
  56:     0x7f35d0bc3282 - <scoped_tls[10d8a9c768e14d1c]::ScopedKey<rustc_span[a05d955188b2abc7]::SessionGlobals>>::set::<rustc_interface[3a5ae1b4f89bc2a4]::interface::run_compiler<core[6134bbfab4c35619]::result::Result<(), rustc_errors[a385eb38ab49152]::ErrorGuaranteed>, rustc_driver[e6df3c77098fb729]::run_compiler::{closure#1}>::{closure#0}, core[6134bbfab4c35619]::result::Result<(), rustc_errors[a385eb38ab49152]::ErrorGuaranteed>>
  57:     0x7f35d0b8c060 - std[aebb45f19603b3b]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[3a5ae1b4f89bc2a4]::util::run_in_thread_pool_with_globals<rustc_interface[3a5ae1b4f89bc2a4]::interface::run_compiler<core[6134bbfab4c35619]::result::Result<(), rustc_errors[a385eb38ab49152]::ErrorGuaranteed>, rustc_driver[e6df3c77098fb729]::run_compiler::{closure#1}>::{closure#0}, core[6134bbfab4c35619]::result::Result<(), rustc_errors[a385eb38ab49152]::ErrorGuaranteed>>::{closure#0}, core[6134bbfab4c35619]::result::Result<(), rustc_errors[a385eb38ab49152]::ErrorGuaranteed>>
  58:     0x7f35d0b8ee5c - <<std[aebb45f19603b3b]::thread::Builder>::spawn_unchecked_<rustc_interface[3a5ae1b4f89bc2a4]::util::run_in_thread_pool_with_globals<rustc_interface[3a5ae1b4f89bc2a4]::interface::run_compiler<core[6134bbfab4c35619]::result::Result<(), rustc_errors[a385eb38ab49152]::ErrorGuaranteed>, rustc_driver[e6df3c77098fb729]::run_compiler::{closure#1}>::{closure#0}, core[6134bbfab4c35619]::result::Result<(), rustc_errors[a385eb38ab49152]::ErrorGuaranteed>>::{closure#0}, core[6134bbfab4c35619]::result::Result<(), rustc_errors[a385eb38ab49152]::ErrorGuaranteed>>::{closure#1} as core[6134bbfab4c35619]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  59:     0x7f35d383f2d3 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hc857ea950c71bdf1
                               at /rustc/b6d0f43b6b1f1c9b4e24f9364b3747d49fd0ac97/library/alloc/src/boxed.rs:1938:9
  60:     0x7f35d383f2d3 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hf072b24df241b0e4
                               at /rustc/b6d0f43b6b1f1c9b4e24f9364b3747d49fd0ac97/library/alloc/src/boxed.rs:1938:9
  61:     0x7f35d383f2d3 - std::sys::unix::thread::Thread::new::thread_start::hb03e8df5386483e7
                               at /rustc/b6d0f43b6b1f1c9b4e24f9364b3747d49fd0ac97/library/std/src/sys/unix/thread.rs:108:17
  62:     0x7f35cf4e3ea5 - start_thread
  63:     0x7f35cf20cb0d - clone
  64:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.66.0-nightly (b6d0f43b6 2022-10-04) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type dylib --crate-type rlib -C prefer-dynamic -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=1 -Z unstable-options -C linker=clang -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C link-args=-fuse-ld=lld -C split-debuginfo=off -Z save-analysis -C prefer-dynamic -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [type_uninhabited_from] computing the inhabitedness of `ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: UserFacing, constness: NotConst }, value: extern "C" fn() }`
#1 [lint_mod] linting module `sys::unix::stack_overflow::imp`
#2 [analysis] running analysis passes on this crate
[RUSTC-TIMING] std test:false 3.555
error: could not compile `std`
Build completed unsuccessfully in 0:09:50
