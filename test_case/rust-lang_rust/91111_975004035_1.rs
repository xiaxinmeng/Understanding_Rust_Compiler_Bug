
error: unexpected token: `{
    impl std::ops::Neg for i8 { }
}`
  --> /home/bdl/projects/contrib/rust/src/test/ui/attributes/issue-90873.rs:1:6
   |
LL | #![a={impl std::ops::Neg for i8 {}}]
   |      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: cannot find attribute `a` in this scope
  --> /home/bdl/projects/contrib/rust/src/test/ui/attributes/issue-90873.rs:1:4
   |
LL | #![a={impl std::ops::Neg for i8 {}}]
   |    ^

error[E0601]: `main` function not found in crate `issue_90873`
  --> /home/bdl/projects/contrib/rust/src/test/ui/attributes/issue-90873.rs:1:1
   |
LL | #![a={impl std::ops::Neg for i8 {}}]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ consider adding a `main` function to `/home/bdl/projects/contrib/rust/src/test/ui/attributes/issue-90873.rs`

thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', /home/bdl/projects/contrib/rust/compiler/rustc_middle/src/hir/map/mod.rs:210:52
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.58.0-dev running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0

query stack during panic:
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', compiler/rustc_middle/src/hir/map/mod.rs:210:52
stack backtrace:
   0:     0x7f42ce406bd3 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h78f5b37598192a4b
   1:     0x7f42ce470a2c - core::fmt::write::h334e8a7dff01ee85
   2:     0x7f42ce425982 - std::io::Write::write_fmt::hc32e08861cda7f55
   3:     0x7f42ce406a5f - std::sys_common::backtrace::print::h0b51c866bf8392d4
   4:     0x7f42ce3e67b7 - std::panicking::default_hook::{{closure}}::hbf375a7b7d563750
   5:     0x7f42ce3e6583 - std::panicking::default_hook::hdc671b6e5bc7b725
   6:     0x7f42cf6d8fb1 - rustc_driver[d41804075018f620]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f42ce3e6c83 - std::panicking::rust_panic_with_hook::h420f3750019b761b
   8:     0x7f42ce4116c2 - std::panicking::begin_panic_handler::{{closure}}::h7c98fec52596dfda
   9:     0x7f42ce411634 - std::sys_common::backtrace::__rust_end_short_backtrace::h15a6240fa53d2b35
  10:     0x7f42ce3e6882 - rust_begin_unwind
  11:     0x7f42ce4772e1 - core::panicking::panic_fmt::h32c0e6c0f26ccedd
  12:     0x7f42ce4771ad - core::panicking::panic::h7d1997b8d9b7dc1f
  13:     0x7f42d188dc05 - <rustc_middle[10090f0afbd31705]::hir::map::Map>::span_if_local
  14:     0x7f42d1727f8e - <rustc_middle[10090f0afbd31705]::hir::provide::{closure#7} as core[f357e379bd850c3a]::ops::function::FnOnce<(rustc_middle[10090f0afbd31705]::ty::context::TyCtxt, rustc_span[40f78271cd095b52]::def_id::DefId)>>::call_once
  15:     0x7f42d0959ea6 - rustc_query_system[dce725971ba33794]::query::plumbing::try_execute_query::<rustc_query_impl[bf6d60d32eceeeec]::plumbing::QueryCtxt, rustc_query_system[dce725971ba33794]::query::caches::DefaultCache<rustc_span[40f78271cd095b52]::def_id::DefId, rustc_span[40f78271cd095b52]::span_encoding::Span>>
  16:     0x7f42d0a7c472 - rustc_query_system[dce725971ba33794]::query::plumbing::get_query::<rustc_query_impl[bf6d60d32eceeeec]::queries::def_span, rustc_query_impl[bf6d60d32eceeeec]::plumbing::QueryCtxt>
  17:     0x7f42d1835783 - <rustc_middle[10090f0afbd31705]::ty::print::pretty::FmtPrinter<&mut alloc[c5437b0a12fc4a30]::string::String> as rustc_middle[10090f0afbd31705]::ty::print::Printer>::print_def_path
  18:     0x7f42d17fcbf3 - <rustc_middle[10090f0afbd31705]::ty::context::TyCtxt>::def_path_str_with_substs
  19:     0x7f42d17fc994 - <rustc_middle[10090f0afbd31705]::ty::context::TyCtxt>::def_path_str
  20:     0x7f42d08c69af - <std[fb89b8483b16d21]::thread::local::LocalKey<core[f357e379bd850c3a]::cell::Cell<bool>>>::with::<rustc_middle[10090f0afbd31705]::ty::print::pretty::with_no_trimmed_paths<<rustc_query_impl[bf6d60d32eceeeec]::queries::impl_trait_ref as rustc_query_system[dce725971ba33794]::query::config::QueryDescription<rustc_query_impl[bf6d60d32eceeeec]::plumbing::QueryCtxt>>::describe::{closure#0}, alloc[c5437b0a12fc4a30]::string::String>::{closure#0}, alloc[c5437b0a12fc4a30]::string::String>
  21:     0x7f42d0ac24c9 - <rustc_query_impl[bf6d60d32eceeeec]::queries::impl_trait_ref as rustc_query_system[dce725971ba33794]::query::config::QueryDescription<rustc_query_impl[bf6d60d32eceeeec]::plumbing::QueryCtxt>>::describe
  22:     0x7f42d08d52aa - <std[fb89b8483b16d21]::thread::local::LocalKey<core[f357e379bd850c3a]::cell::Cell<bool>>>::with::<rustc_middle[10090f0afbd31705]::ty::print::pretty::with_no_visible_paths<rustc_query_impl[bf6d60d32eceeeec]::make_query::impl_trait_ref::{closure#0}, alloc[c5437b0a12fc4a30]::string::String>::{closure#0}, alloc[c5437b0a12fc4a30]::string::String>
  23:     0x7f42d0884729 - rustc_query_impl[bf6d60d32eceeeec]::make_query::impl_trait_ref
  24:     0x7f42d090760e - <rustc_query_system[dce725971ba33794]::query::plumbing::QueryState<rustc_middle[10090f0afbd31705]::dep_graph::dep_node::DepKind, rustc_span[40f78271cd095b52]::def_id::DefId>>::try_collect_active_jobs::<rustc_query_impl[bf6d60d32eceeeec]::plumbing::QueryCtxt>
  25:     0x7f42d0b555d6 - <rustc_query_impl[bf6d60d32eceeeec]::Queries>::try_collect_active_jobs
  26:     0x7f42d0ab7e61 - rustc_query_system[dce725971ba33794]::query::job::print_query_stack::<rustc_query_impl[bf6d60d32eceeeec]::plumbing::QueryCtxt>
  27:     0x7f42cf7f1132 - rustc_interface[f04650f84469ddcf]::interface::try_print_query_stack
  28:     0x7f42cf6d9865 - rustc_driver[d41804075018f620]::report_ice
  29:     0x7f42ce3e6c83 - std::panicking::rust_panic_with_hook::h420f3750019b761b
  30:     0x7f42ce4116c2 - std::panicking::begin_panic_handler::{{closure}}::h7c98fec52596dfda
  31:     0x7f42ce411634 - std::sys_common::backtrace::__rust_end_short_backtrace::h15a6240fa53d2b35
  32:     0x7f42ce3e6882 - rust_begin_unwind
  33:     0x7f42ce4772e1 - core::panicking::panic_fmt::h32c0e6c0f26ccedd
  34:     0x7f42ce4771ad - core::panicking::panic::h7d1997b8d9b7dc1f
  35:     0x7f42cfe7b341 - rustc_typeck[2410166cb868bf25]::collect::impl_trait_ref
  36:     0x7f42d09512a7 - rustc_query_system[dce725971ba33794]::query::plumbing::try_execute_query::<rustc_query_impl[bf6d60d32eceeeec]::plumbing::QueryCtxt, rustc_query_system[dce725971ba33794]::query::caches::DefaultCache<rustc_span[40f78271cd095b52]::def_id::DefId, core[f357e379bd850c3a]::option::Option<rustc_middle[10090f0afbd31705]::ty::sty::TraitRef>>>
  37:     0x7f42d0a29d8f - rustc_query_system[dce725971ba33794]::query::plumbing::get_query::<rustc_query_impl[bf6d60d32eceeeec]::queries::impl_trait_ref, rustc_query_impl[bf6d60d32eceeeec]::plumbing::QueryCtxt>
  38:     0x7f42cfcb6216 - rustc_typeck[2410166cb868bf25]::coherence::orphan::orphan_check_crate
  39:     0x7f42d0983f65 - rustc_query_system[dce725971ba33794]::query::plumbing::try_execute_query::<rustc_query_impl[bf6d60d32eceeeec]::plumbing::QueryCtxt, rustc_query_system[dce725971ba33794]::query::caches::DefaultCache<(), &[rustc_span[40f78271cd095b52]::def_id::LocalDefId]>>
  40:     0x7f42d0a40e32 - rustc_query_system[dce725971ba33794]::query::plumbing::get_query::<rustc_query_impl[bf6d60d32eceeeec]::queries::orphan_check_crate, rustc_query_impl[bf6d60d32eceeeec]::plumbing::QueryCtxt>
  41:     0x7f42cfd4f02f - rustc_typeck[2410166cb868bf25]::coherence::check_coherence
  42:     0x7f42cfe6b64d - <rustc_session[d4984f95fe528ebf]::session::Session>::track_errors::<rustc_typeck[2410166cb868bf25]::check_crate::{closure#3}, ()>
  43:     0x7f42cfdf451e - rustc_typeck[2410166cb868bf25]::check_crate
  44:     0x7f42cf7ac931 - rustc_interface[f04650f84469ddcf]::passes::analysis
  45:     0x7f42d097e82f - rustc_query_system[dce725971ba33794]::query::plumbing::try_execute_query::<rustc_query_impl[bf6d60d32eceeeec]::plumbing::QueryCtxt, rustc_query_system[dce725971ba33794]::query::caches::DefaultCache<(), core[f357e379bd850c3a]::result::Result<(), rustc_errors[d7239617424b38c3]::ErrorReported>>>
  46:     0x7f42d0a7c11c - rustc_query_system[dce725971ba33794]::query::plumbing::get_query::<rustc_query_impl[bf6d60d32eceeeec]::queries::analysis, rustc_query_impl[bf6d60d32eceeeec]::plumbing::QueryCtxt>
  47:     0x7f42cf72b517 - <rustc_interface[f04650f84469ddcf]::passes::QueryContext>::enter::<rustc_driver[d41804075018f620]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[f357e379bd850c3a]::result::Result<(), rustc_errors[d7239617424b38c3]::ErrorReported>>
  48:     0x7f42cf6431fb - <rustc_interface[f04650f84469ddcf]::interface::Compiler>::enter::<rustc_driver[d41804075018f620]::run_compiler::{closure#1}::{closure#2}, core[f357e379bd850c3a]::result::Result<core[f357e379bd850c3a]::option::Option<rustc_interface[f04650f84469ddcf]::queries::Linker>, rustc_errors[d7239617424b38c3]::ErrorReported>>
  49:     0x7f42cf6de31b - rustc_span[40f78271cd095b52]::with_source_map::<core[f357e379bd850c3a]::result::Result<(), rustc_errors[d7239617424b38c3]::ErrorReported>, rustc_interface[f04650f84469ddcf]::interface::create_compiler_and_run<core[f357e379bd850c3a]::result::Result<(), rustc_errors[d7239617424b38c3]::ErrorReported>, rustc_driver[d41804075018f620]::run_compiler::{closure#1}>::{closure#1}>
  50:     0x7f42cf643dba - rustc_interface[f04650f84469ddcf]::interface::create_compiler_and_run::<core[f357e379bd850c3a]::result::Result<(), rustc_errors[d7239617424b38c3]::ErrorReported>, rustc_driver[d41804075018f620]::run_compiler::{closure#1}>
  51:     0x7f42cf64a9fe - <scoped_tls[bf13be3bdb46ebf7]::ScopedKey<rustc_span[40f78271cd095b52]::SessionGlobals>>::set::<rustc_interface[f04650f84469ddcf]::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface[f04650f84469ddcf]::interface::run_compiler<core[f357e379bd850c3a]::result::Result<(), rustc_errors[d7239617424b38c3]::ErrorReported>, rustc_driver[d41804075018f620]::run_compiler::{closure#1}>::{closure#0}, core[f357e379bd850c3a]::result::Result<(), rustc_errors[d7239617424b38c3]::ErrorReported>>::{closure#0}::{closure#0}, core[f357e379bd850c3a]::result::Result<(), rustc_errors[d7239617424b38c3]::ErrorReported>>
  52:     0x7f42cf731775 - std[fb89b8483b16d21]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[f04650f84469ddcf]::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface[f04650f84469ddcf]::interface::run_compiler<core[f357e379bd850c3a]::result::Result<(), rustc_errors[d7239617424b38c3]::ErrorReported>, rustc_driver[d41804075018f620]::run_compiler::{closure#1}>::{closure#0}, core[f357e379bd850c3a]::result::Result<(), rustc_errors[d7239617424b38c3]::ErrorReported>>::{closure#0}, core[f357e379bd850c3a]::result::Result<(), rustc_errors[d7239617424b38c3]::ErrorReported>>
  53:     0x7f42cf6dd0ce - std[fb89b8483b16d21]::panicking::try::<core[f357e379bd850c3a]::result::Result<(), rustc_errors[d7239617424b38c3]::ErrorReported>, core[f357e379bd850c3a]::panic::unwind_safe::AssertUnwindSafe<<std[fb89b8483b16d21]::thread::Builder>::spawn_unchecked<rustc_interface[f04650f84469ddcf]::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface[f04650f84469ddcf]::interface::run_compiler<core[f357e379bd850c3a]::result::Result<(), rustc_errors[d7239617424b38c3]::ErrorReported>, rustc_driver[d41804075018f620]::run_compiler::{closure#1}>::{closure#0}, core[f357e379bd850c3a]::result::Result<(), rustc_errors[d7239617424b38c3]::ErrorReported>>::{closure#0}, core[f357e379bd850c3a]::result::Result<(), rustc_errors[d7239617424b38c3]::ErrorReported>>::{closure#1}::{closure#0}>>
  54:     0x7f42cf647fc8 - <<std[fb89b8483b16d21]::thread::Builder>::spawn_unchecked<rustc_interface[f04650f84469ddcf]::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface[f04650f84469ddcf]::interface::run_compiler<core[f357e379bd850c3a]::result::Result<(), rustc_errors[d7239617424b38c3]::ErrorReported>, rustc_driver[d41804075018f620]::run_compiler::{closure#1}>::{closure#0}, core[f357e379bd850c3a]::result::Result<(), rustc_errors[d7239617424b38c3]::ErrorReported>>::{closure#0}, core[f357e379bd850c3a]::result::Result<(), rustc_errors[d7239617424b38c3]::ErrorReported>>::{closure#1} as core[f357e379bd850c3a]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  55:     0x7f42ce3e5cc8 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::he2cb6e0d2e44ae07
  56:     0x7f42ce3f2497 - std::sys::unix::thread::Thread::new::thread_start::h715c8a327323cfc7
  57:     0x7f42c9223ea7 - start_thread
                               at ./nptl/./nptl/pthread_create.c:477:8
  58:     0x7f42ce236def - clone
                               at ./misc/../sysdeps/unix/sysv/linux/x86_64/clone.S:95
  59:                0x0 - <unknown>

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.58.0-dev running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0

query stack during panic:
end of query stack
thread panicked while panicking. aborting.

