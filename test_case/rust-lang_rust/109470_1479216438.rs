plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:24cb9080177205b6e8c946b17badbe402adc938f)
Download action repository 'rust-lang/simpleinfra@master' (SHA:5f3e9487b084c5235556ffd8baa8b183de9eb120)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-16core-64gb)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---
   Compiling memchr v2.5.0
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.87
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: internal compiler error: compiler/rustc_middle/src/ty/subst.rs:821:13: Unexpected parameter Type(Self) when substituting in region 'a (index=0)
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1644:9
stack backtrace:
stack backtrace:
   0:     0x7f624b746275 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h58f3d5f9613fc903
   1:     0x7f624b7b2bd8 - core::fmt::write::hf3b1e4fb936f95f6
   2:     0x7f624b73a581 - std::io::Write::write_fmt::h46e9ff7509b654e1
   3:     0x7f624b746085 - std::sys_common::backtrace::print::h0b48412da775984a
   4:     0x7f624b749274 - std::panicking::default_hook::{{closure}}::h4eaef8437e4e66b7
   5:     0x7f624b748f62 - std::panicking::default_hook::hef252b4dd17d3bdf
   6:     0x7f624c1d2aa5 - rustc_driver_impl[eab11fa56b32282c]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f624b7499a9 - std::panicking::rust_panic_with_hook::hff848c0b52cb878e
   8:     0x7f624ed717c3 - std[e292dd3bffb96032]::panicking::begin_panic::<rustc_errors[b2964f0485bf811b]::ExplicitBug>::{closure#0}
   9:     0x7f624ed6eac6 - std[e292dd3bffb96032]::sys_common::backtrace::__rust_end_short_backtrace::<std[e292dd3bffb96032]::panicking::begin_panic<rustc_errors[b2964f0485bf811b]::ExplicitBug>::{closure#0}, !>
  10:     0x7f624c16e0b6 - std[e292dd3bffb96032]::panicking::begin_panic::<rustc_errors[b2964f0485bf811b]::ExplicitBug>
  11:     0x7f624edd55e6 - std[e292dd3bffb96032]::panic::panic_any::<rustc_errors[b2964f0485bf811b]::ExplicitBug>
  12:     0x7f624edcea9a - <rustc_errors[b2964f0485bf811b]::HandlerInner>::bug::<&alloc[5a8e793bc0d22b6e]::string::String>
  13:     0x7f624edce5d0 - <rustc_errors[b2964f0485bf811b]::Handler>::bug::<&alloc[5a8e793bc0d22b6e]::string::String>
  14:     0x7f624ee18e95 - rustc_middle[287f097903f14c7c]::util::bug::opt_span_bug_fmt::<rustc_span[7fb5d3de1e91b7b5]::span_encoding::Span>::{closure#0}
  15:     0x7f624ee130bc - rustc_middle[287f097903f14c7c]::ty::context::tls::with_opt::<rustc_middle[287f097903f14c7c]::util::bug::opt_span_bug_fmt<rustc_span[7fb5d3de1e91b7b5]::span_encoding::Span>::{closure#0}, !>::{closure#0}
  16:     0x7f624ee1306e - rustc_middle[287f097903f14c7c]::ty::context::tls::with_context_opt::<rustc_middle[287f097903f14c7c]::ty::context::tls::with_opt<rustc_middle[287f097903f14c7c]::util::bug::opt_span_bug_fmt<rustc_span[7fb5d3de1e91b7b5]::span_encoding::Span>::{closure#0}, !>::{closure#0}, !>
  17:     0x7f624ee18dd9 - rustc_middle[287f097903f14c7c]::util::bug::opt_span_bug_fmt::<rustc_span[7fb5d3de1e91b7b5]::span_encoding::Span>
  18:     0x7f624c176a75 - rustc_middle[287f097903f14c7c]::util::bug::bug_fmt
  19:     0x7f624c176681 - <rustc_middle[287f097903f14c7c]::ty::subst::SubstFolder as rustc_type_ir[deda4be755df9f70]::fold::TypeFolder<rustc_middle[287f097903f14c7c]::ty::context::TyCtxt>>::fold_region::region_param_invalid
  20:     0x7f624edec556 - <rustc_middle[287f097903f14c7c]::ty::subst::SubstFolder as rustc_type_ir[deda4be755df9f70]::fold::TypeFolder<rustc_middle[287f097903f14c7c]::ty::context::TyCtxt>>::fold_region
  21:     0x7f624eccddcf - <rustc_middle[287f097903f14c7c]::ty::Ty as rustc_type_ir[deda4be755df9f70]::fold::TypeSuperFoldable<rustc_middle[287f097903f14c7c]::ty::context::TyCtxt>>::super_fold_with::<rustc_middle[287f097903f14c7c]::ty::subst::SubstFolder>
  22:     0x7f624edec79d - <rustc_middle[287f097903f14c7c]::ty::subst::SubstFolder as rustc_type_ir[deda4be755df9f70]::fold::TypeFolder<rustc_middle[287f097903f14c7c]::ty::context::TyCtxt>>::fold_ty
  23:     0x7f624c9c765d - rustc_hir_analysis[12d67aba82b705a0]::check::compare_impl_item::check_type_bounds
  24:     0x7f624c9c0646 - rustc_hir_analysis[12d67aba82b705a0]::check::compare_impl_item::compare_impl_ty
  25:     0x7f624c92ec66 - rustc_hir_analysis[12d67aba82b705a0]::check::check::check_impl_items_against_trait
  26:     0x7f624c92b1f8 - rustc_hir_analysis[12d67aba82b705a0]::check::check::check_item_type
  27:     0x7f624c935cea - rustc_hir_analysis[12d67aba82b705a0]::check::check::check_mod_item_types
  28:     0x7f624de7fc8e - rustc_query_system[9f7ff6c77148f12]::query::plumbing::try_execute_query::<rustc_query_impl[5697ed4679dbf91d]::queries::check_mod_item_types, rustc_query_impl[5697ed4679dbf91d]::plumbing::QueryCtxt>
  29:     0x7f624dc9d8d9 - <rustc_query_impl[5697ed4679dbf91d]::Queries as rustc_middle[287f097903f14c7c]::ty::query::QueryEngine>::check_mod_item_types
  30:     0x7f624c9ce2b7 - <rustc_middle[287f097903f14c7c]::hir::map::Map>::for_each_module::<rustc_hir_analysis[12d67aba82b705a0]::check_crate::{closure#6}::{closure#0}>
  31:     0x7f624c9131f2 - <rustc_session[657c67027fcde92f]::session::Session>::time::<(), rustc_hir_analysis[12d67aba82b705a0]::check_crate::{closure#6}>
  32:     0x7f624ca34b36 - rustc_hir_analysis[12d67aba82b705a0]::check_crate
  33:     0x7f624c296318 - rustc_interface[894d6170faa00a43]::passes::analysis
  34:     0x7f624df35cf3 - rustc_query_system[9f7ff6c77148f12]::query::plumbing::try_execute_query::<rustc_query_impl[5697ed4679dbf91d]::queries::analysis, rustc_query_impl[5697ed4679dbf91d]::plumbing::QueryCtxt>
  35:     0x7f624dc75142 - <rustc_query_impl[5697ed4679dbf91d]::Queries as rustc_middle[287f097903f14c7c]::ty::query::QueryEngine>::analysis
  36:     0x7f624c24379e - <rustc_middle[287f097903f14c7c]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[eab11fa56b32282c]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[7fb5d3de1e91b7b5]::ErrorGuaranteed>>
  37:     0x7f624c21592c - <rustc_interface[894d6170faa00a43]::interface::Compiler>::enter::<rustc_driver_impl[eab11fa56b32282c]::run_compiler::{closure#1}::{closure#2}, core[54ab13d2a06817e1]::result::Result<core[54ab13d2a06817e1]::option::Option<rustc_interface[894d6170faa00a43]::queries::Linker>, rustc_span[7fb5d3de1e91b7b5]::ErrorGuaranteed>>
  38:     0x7f624c230580 - rustc_span[7fb5d3de1e91b7b5]::with_source_map::<core[54ab13d2a06817e1]::result::Result<(), rustc_span[7fb5d3de1e91b7b5]::ErrorGuaranteed>, rustc_interface[894d6170faa00a43]::interface::run_compiler<core[54ab13d2a06817e1]::result::Result<(), rustc_span[7fb5d3de1e91b7b5]::ErrorGuaranteed>, rustc_driver_impl[eab11fa56b32282c]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  39:     0x7f624c1eb855 - std[e292dd3bffb96032]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[894d6170faa00a43]::util::run_in_thread_pool_with_globals<rustc_interface[894d6170faa00a43]::interface::run_compiler<core[54ab13d2a06817e1]::result::Result<(), rustc_span[7fb5d3de1e91b7b5]::ErrorGuaranteed>, rustc_driver_impl[eab11fa56b32282c]::run_compiler::{closure#1}>::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[7fb5d3de1e91b7b5]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[7fb5d3de1e91b7b5]::ErrorGuaranteed>>
  40:     0x7f624c234b26 - std[e292dd3bffb96032]::panicking::try::<core[54ab13d2a06817e1]::result::Result<(), rustc_span[7fb5d3de1e91b7b5]::ErrorGuaranteed>, core[54ab13d2a06817e1]::panic::unwind_safe::AssertUnwindSafe<<std[e292dd3bffb96032]::thread::Builder>::spawn_unchecked_<rustc_interface[894d6170faa00a43]::util::run_in_thread_pool_with_globals<rustc_interface[894d6170faa00a43]::interface::run_compiler<core[54ab13d2a06817e1]::result::Result<(), rustc_span[7fb5d3de1e91b7b5]::ErrorGuaranteed>, rustc_driver_impl[eab11fa56b32282c]::run_compiler::{closure#1}>::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[7fb5d3de1e91b7b5]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[7fb5d3de1e91b7b5]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  41:     0x7f624c1e1344 - <<std[e292dd3bffb96032]::thread::Builder>::spawn_unchecked_<rustc_interface[894d6170faa00a43]::util::run_in_thread_pool_with_globals<rustc_interface[894d6170faa00a43]::interface::run_compiler<core[54ab13d2a06817e1]::result::Result<(), rustc_span[7fb5d3de1e91b7b5]::ErrorGuaranteed>, rustc_driver_impl[eab11fa56b32282c]::run_compiler::{closure#1}>::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[7fb5d3de1e91b7b5]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[7fb5d3de1e91b7b5]::ErrorGuaranteed>>::{closure#1} as core[54ab13d2a06817e1]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  42:     0x7f624b755d2e - std::sys::unix::thread::Thread::new::thread_start::hb98a15cf8f2a3509
  43:     0x7f624b4f3b43 - <unknown>
  44:     0x7f624b585a00 - <unknown>
  45:                0x0 - <unknown>
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.70.0-nightly (9a5d5b82e 2023-03-21) running on x86_64-unknown-linux-gnu


note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [check_mod_item_types] checking item types in module `error`
#1 [analysis] running analysis passes on this crate
error: could not compile `core`
Build completed unsuccessfully in 0:03:13
