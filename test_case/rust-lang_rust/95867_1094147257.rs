plain
[RUSTC-TIMING] rustc_std_workspace_core test:false 0.073
error[E0282]: type annotations needed
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/compiler_builtins-0.1.70/src/float/conv.rs:32:25
   |
32 |             x.cast() << (f_sd - exp - 1),
   |                         ^^^^^^^^^^^^^^^^ cannot infer type for type `{integer}`

thread 'rustc' panicked at 'Failed to get parent for DefId(0:0 ~ compiler_builtins[5e6d])', compiler/rustc_middle/src/traits/specialization_graph.rs:45:52
stack backtrace:
   0:     0x7f77b9cf3ffd - std::backtrace_rs::backtrace::libunwind::trace::he82d420584fababe
                               at /rustc/15d46b8154c4bad80167c6cbf595c14757a5d34d/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7f77b9cf3ffd - std::backtrace_rs::backtrace::trace_unsynchronized::h32e7d2468e45dba3
                               at /rustc/15d46b8154c4bad80167c6cbf595c14757a5d34d/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7f77b9cf3ffd - std::sys_common::backtrace::_print_fmt::h592247c5117ee813
                               at /rustc/15d46b8154c4bad80167c6cbf595c14757a5d34d/library/std/src/sys_common/backtrace.rs:66:5
   3:     0x7f77b9cf3ffd - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h433eb4d018df726f
                               at /rustc/15d46b8154c4bad80167c6cbf595c14757a5d34d/library/std/src/sys_common/backtrace.rs:45:22
   4:     0x7f77b9d4dd1c - core::fmt::write::h047481f0716cc3e7
                               at /rustc/15d46b8154c4bad80167c6cbf595c14757a5d34d/library/core/src/fmt/mod.rs:1194:17
   5:     0x7f77b9ce5821 - std::io::Write::write_fmt::h9d7fef21064260e6
                               at /rustc/15d46b8154c4bad80167c6cbf595c14757a5d34d/library/std/src/io/mod.rs:1655:15
   6:     0x7f77b9cf6f15 - std::sys_common::backtrace::_print::hbdd7703d4ed4f702
                               at /rustc/15d46b8154c4bad80167c6cbf595c14757a5d34d/library/std/src/sys_common/backtrace.rs:48:5
   7:     0x7f77b9cf6f15 - std::sys_common::backtrace::print::h612c74123aa21d26
                               at /rustc/15d46b8154c4bad80167c6cbf595c14757a5d34d/library/std/src/sys_common/backtrace.rs:35:9
   8:     0x7f77b9cf6f15 - std::panicking::default_hook::{{closure}}::h2b2c7128b4a12306
                               at /rustc/15d46b8154c4bad80167c6cbf595c14757a5d34d/library/std/src/panicking.rs:295:22
   9:     0x7f77b9cf6b89 - std::panicking::default_hook::hf50ad1ed27ad018d
                               at /rustc/15d46b8154c4bad80167c6cbf595c14757a5d34d/library/std/src/panicking.rs:314:9
  10:     0x7f77ba644d69 - rustc_driver[963d8b28c9e32bce]::DEFAULT_HOOK::{closure#0}::{closure#0}
  11:     0x7f77b9cf76b0 - std::panicking::rust_panic_with_hook::h4d5445299cf86ef1
                               at /rustc/15d46b8154c4bad80167c6cbf595c14757a5d34d/library/std/src/panicking.rs:702:17
  12:     0x7f77b9cf74e7 - std::panicking::begin_panic_handler::{{closure}}::h00e400154add04ec
                               at /rustc/15d46b8154c4bad80167c6cbf595c14757a5d34d/library/std/src/panicking.rs:588:13
  13:     0x7f77b9cf44b4 - std::sys_common::backtrace::__rust_end_short_backtrace::h29dca1fe55960ae1
                               at /rustc/15d46b8154c4bad80167c6cbf595c14757a5d34d/library/std/src/sys_common/backtrace.rs:138:18
  14:     0x7f77b9cf7219 - rust_begin_unwind
                               at /rustc/15d46b8154c4bad80167c6cbf595c14757a5d34d/library/std/src/panicking.rs:584:5
  15:     0x7f77b9cbb763 - core::panicking::panic_fmt::hf1439381799f2864
                               at /rustc/15d46b8154c4bad80167c6cbf595c14757a5d34d/library/core/src/panicking.rs:142:14
  16:     0x7f77bcbddd3e - <rustc_middle[4499fc7b7472eef]::traits::specialization_graph::Graph>::parent
  17:     0x7f77bcbde8d0 - <rustc_middle[4499fc7b7472eef]::traits::specialization_graph::Ancestors>::leaf_def
  18:     0x7f77bc7b80da - rustc_trait_selection[537040e93e7a58fe]::traits::project::assoc_def
  19:     0x7f77bc7604ef - <rustc_infer[fb4585239d5b9492]::infer::InferCtxt>::commit_if_ok::<(), (), rustc_trait_selection[537040e93e7a58fe]::traits::project::assemble_candidates_from_impls::{closure#0}>
  20:     0x7f77bc7bd53a - rustc_trait_selection[537040e93e7a58fe]::traits::project::project
  21:     0x7f77bc7baf3b - rustc_trait_selection[537040e93e7a58fe]::traits::project::opt_normalize_projection_type
  22:     0x7f77bc7b5e23 - rustc_trait_selection[537040e93e7a58fe]::traits::project::normalize_projection_type
  23:     0x7f77bc7b45e7 - <rustc_trait_selection[537040e93e7a58fe]::traits::project::AssocTypeNormalizer as rustc_middle[4499fc7b7472eef]::ty::fold::TypeFolder>::fold_ty
  24:     0x7f77bc8913f9 - <rustc_middle[4499fc7b7472eef]::ty::Ty as rustc_middle[4499fc7b7472eef]::ty::fold::TypeFoldable>::super_fold_with::<rustc_trait_selection[537040e93e7a58fe]::traits::project::AssocTypeNormalizer>
  25:     0x7f77bc7b3faf - <rustc_trait_selection[537040e93e7a58fe]::traits::project::AssocTypeNormalizer as rustc_middle[4499fc7b7472eef]::ty::fold::TypeFolder>::fold_ty
  26:     0x7f77baf122bb - rustc_middle[4499fc7b7472eef]::ty::util::fold_list::<rustc_trait_selection[537040e93e7a58fe]::traits::project::AssocTypeNormalizer, rustc_middle[4499fc7b7472eef]::ty::Ty, <&rustc_middle[4499fc7b7472eef]::ty::list::List<rustc_middle[4499fc7b7472eef]::ty::Ty> as rustc_middle[4499fc7b7472eef]::ty::fold::TypeFoldable>::try_super_fold_with<rustc_trait_selection[537040e93e7a58fe]::traits::project::AssocTypeNormalizer>::{closure#0}>
  27:     0x7f77bafd733b - <rustc_trait_selection[537040e93e7a58fe]::traits::project::AssocTypeNormalizer>::fold::<rustc_middle[4499fc7b7472eef]::ty::sty::FnSig>
  28:     0x7f77bafec1f8 - rustc_trait_selection[537040e93e7a58fe]::traits::project::normalize::<rustc_middle[4499fc7b7472eef]::ty::sty::FnSig>
  29:     0x7f77bae007a0 - <rustc_typeck[9eafc09810d5126a]::check::fn_ctxt::FnCtxt>::construct_obligation_for_trait
  30:     0x7f77bae2870b - <rustc_typeck[9eafc09810d5126a]::check::fn_ctxt::FnCtxt>::lookup_method_in_trait
  31:     0x7f77bae1d15a - <rustc_typeck[9eafc09810d5126a]::check::fn_ctxt::FnCtxt>::check_expr_kind
  32:     0x7f77badd1b01 - <rustc_typeck[9eafc09810d5126a]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  33:     0x7f77bae1e1d7 - <rustc_typeck[9eafc09810d5126a]::check::fn_ctxt::FnCtxt>::check_expr_kind
  34:     0x7f77badd1b01 - <rustc_typeck[9eafc09810d5126a]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  35:     0x7f77bade5fec - <rustc_typeck[9eafc09810d5126a]::check::fn_ctxt::FnCtxt>::check_stmt
  36:     0x7f77bade6753 - <rustc_typeck[9eafc09810d5126a]::check::fn_ctxt::FnCtxt>::check_block_with_expected
  37:     0x7f77bae1cd32 - <rustc_typeck[9eafc09810d5126a]::check::fn_ctxt::FnCtxt>::check_expr_kind
  38:     0x7f77badd1b01 - <rustc_typeck[9eafc09810d5126a]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  39:     0x7f77baf3a4ec - <rustc_infer[fb4585239d5b9492]::infer::InferCtxtBuilder>::enter::<&rustc_middle[4499fc7b7472eef]::ty::context::TypeckResults, <rustc_typeck[9eafc09810d5126a]::check::inherited::InheritedBuilder>::enter<rustc_typeck[9eafc09810d5126a]::check::typeck_with_fallback<rustc_typeck[9eafc09810d5126a]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[4499fc7b7472eef]::ty::context::TypeckResults>::{closure#0}>
  40:     0x7f77baeba641 - rustc_typeck[9eafc09810d5126a]::check::typeck
  41:     0x7f77bbaec820 - rustc_query_system[10a64a31f4f52c65]::query::plumbing::try_execute_query::<rustc_query_impl[6c9a745188c71205]::plumbing::QueryCtxt, rustc_query_system[10a64a31f4f52c65]::query::caches::DefaultCache<rustc_span[e92932c11291754d]::def_id::LocalDefId, &rustc_middle[4499fc7b7472eef]::ty::context::TypeckResults>>
  42:     0x7f77bbbd92e8 - rustc_query_system[10a64a31f4f52c65]::query::plumbing::get_query::<rustc_query_impl[6c9a745188c71205]::queries::typeck, rustc_query_impl[6c9a745188c71205]::plumbing::QueryCtxt>
  43:     0x7f77bafa0d78 - <rustc_middle[4499fc7b7472eef]::hir::map::Map>::par_body_owners::<rustc_typeck[9eafc09810d5126a]::check::typeck_item_bodies::{closure#0}>
  44:     0x7f77baebf6ed - rustc_typeck[9eafc09810d5126a]::check::typeck_item_bodies
  45:     0x7f77bbb52387 - rustc_query_system[10a64a31f4f52c65]::query::plumbing::try_execute_query::<rustc_query_impl[6c9a745188c71205]::plumbing::QueryCtxt, rustc_query_system[10a64a31f4f52c65]::query::caches::DefaultCache<(), ()>>
  46:     0x7f77bbbb83fa - rustc_query_system[10a64a31f4f52c65]::query::plumbing::get_query::<rustc_query_impl[6c9a745188c71205]::queries::typeck_item_bodies, rustc_query_impl[6c9a745188c71205]::plumbing::QueryCtxt>
  47:     0x7f77baefe341 - rustc_typeck[9eafc09810d5126a]::check_crate
  48:     0x7f77ba730a21 - rustc_interface[510514e2c7202ceb]::passes::analysis
  49:     0x7f77bbb45699 - rustc_query_system[10a64a31f4f52c65]::query::plumbing::try_execute_query::<rustc_query_impl[6c9a745188c71205]::plumbing::QueryCtxt, rustc_query_system[10a64a31f4f52c65]::query::caches::DefaultCache<(), core[7b24a5c2c811bf3d]::result::Result<(), rustc_errors[15b2ceb35a572425]::ErrorGuaranteed>>>
  50:     0x7f77bbbd9677 - rustc_query_system[10a64a31f4f52c65]::query::plumbing::get_query::<rustc_query_impl[6c9a745188c71205]::queries::analysis, rustc_query_impl[6c9a745188c71205]::plumbing::QueryCtxt>
  51:     0x7f77ba60233b - <rustc_interface[510514e2c7202ceb]::passes::QueryContext>::enter::<rustc_driver[963d8b28c9e32bce]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[7b24a5c2c811bf3d]::result::Result<(), rustc_errors[15b2ceb35a572425]::ErrorGuaranteed>>
  52:     0x7f77ba5de051 - <rustc_interface[510514e2c7202ceb]::interface::Compiler>::enter::<rustc_driver[963d8b28c9e32bce]::run_compiler::{closure#1}::{closure#2}, core[7b24a5c2c811bf3d]::result::Result<core[7b24a5c2c811bf3d]::option::Option<rustc_interface[510514e2c7202ceb]::queries::Linker>, rustc_errors[15b2ceb35a572425]::ErrorGuaranteed>>
  53:     0x7f77ba64e2e3 - rustc_span[e92932c11291754d]::with_source_map::<core[7b24a5c2c811bf3d]::result::Result<(), rustc_errors[15b2ceb35a572425]::ErrorGuaranteed>, rustc_interface[510514e2c7202ceb]::interface::create_compiler_and_run<core[7b24a5c2c811bf3d]::result::Result<(), rustc_errors[15b2ceb35a572425]::ErrorGuaranteed>, rustc_driver[963d8b28c9e32bce]::run_compiler::{closure#1}>::{closure#1}>
  54:     0x7f77ba5def41 - rustc_interface[510514e2c7202ceb]::interface::create_compiler_and_run::<core[7b24a5c2c811bf3d]::result::Result<(), rustc_errors[15b2ceb35a572425]::ErrorGuaranteed>, rustc_driver[963d8b28c9e32bce]::run_compiler::{closure#1}>
  55:     0x7f77ba5fbbc2 - <scoped_tls[8ecdc8189a2436d4]::ScopedKey<rustc_span[e92932c11291754d]::SessionGlobals>>::set::<rustc_interface[510514e2c7202ceb]::interface::run_compiler<core[7b24a5c2c811bf3d]::result::Result<(), rustc_errors[15b2ceb35a572425]::ErrorGuaranteed>, rustc_driver[963d8b28c9e32bce]::run_compiler::{closure#1}>::{closure#0}, core[7b24a5c2c811bf3d]::result::Result<(), rustc_errors[15b2ceb35a572425]::ErrorGuaranteed>>
  56:     0x7f77ba6057ff - std[b270603ab51ba841]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[510514e2c7202ceb]::util::run_in_thread_pool_with_globals<rustc_interface[510514e2c7202ceb]::interface::run_compiler<core[7b24a5c2c811bf3d]::result::Result<(), rustc_errors[15b2ceb35a572425]::ErrorGuaranteed>, rustc_driver[963d8b28c9e32bce]::run_compiler::{closure#1}>::{closure#0}, core[7b24a5c2c811bf3d]::result::Result<(), rustc_errors[15b2ceb35a572425]::ErrorGuaranteed>>::{closure#0}, core[7b24a5c2c811bf3d]::result::Result<(), rustc_errors[15b2ceb35a572425]::ErrorGuaranteed>>
  57:     0x7f77ba608b39 - <<std[b270603ab51ba841]::thread::Builder>::spawn_unchecked_<rustc_interface[510514e2c7202ceb]::util::run_in_thread_pool_with_globals<rustc_interface[510514e2c7202ceb]::interface::run_compiler<core[7b24a5c2c811bf3d]::result::Result<(), rustc_errors[15b2ceb35a572425]::ErrorGuaranteed>, rustc_driver[963d8b28c9e32bce]::run_compiler::{closure#1}>::{closure#0}, core[7b24a5c2c811bf3d]::result::Result<(), rustc_errors[15b2ceb35a572425]::ErrorGuaranteed>>::{closure#0}, core[7b24a5c2c811bf3d]::result::Result<(), rustc_errors[15b2ceb35a572425]::ErrorGuaranteed>>::{closure#1} as core[7b24a5c2c811bf3d]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  58:     0x7f77b9d01813 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hc769d35e5ff1556a
                               at /rustc/15d46b8154c4bad80167c6cbf595c14757a5d34d/library/alloc/src/boxed.rs:1858:9
  59:     0x7f77b9d01813 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h0f1a8717e1c9f2ce
                               at /rustc/15d46b8154c4bad80167c6cbf595c14757a5d34d/library/alloc/src/boxed.rs:1858:9
  60:     0x7f77b9d01813 - std::sys::unix::thread::Thread::new::thread_start::hfc9cf8ccb9230662
                               at /rustc/15d46b8154c4bad80167c6cbf595c14757a5d34d/library/std/src/sys/unix/thread.rs:108:17
  61:     0x7f77b983c8ca - start_thread
  62:     0x7f77b939aabd - clone
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.62.0-nightly (15d46b815 2022-04-09) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=10000 -C debuginfo=1 -C linker=clang -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -Z save-analysis -C prefer-dynamic -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -C panic=abort -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [typeck] type-checking `int::<impl at /cargo/registry/src/github.com-1ecc6299db9ec823/compiler_builtins-0.1.70/src/int/mod.rs:206:9: 229:10>::FUZZ_LENGTHS`
#1 [typeck_item_bodies] type-checking all item bodies
#2 [analysis] running analysis passes on this crate
For more information about this error, try `rustc --explain E0282`.
For more information about this error, try `rustc --explain E0282`.
thread 'rustc' panicked at 'Failed to get parent for DefId(0:0 ~ libc[c2e9])', compiler/rustc_middle/src/traits/specialization_graph.rs:45:52
[RUSTC-TIMING] compiler_builtins test:false 0.349
error: could not compile `compiler_builtins` due to previous error
warning: build failed, waiting for other jobs to finish...
warning: build failed, waiting for other jobs to finish...
   0:     0x7fe69bf37ffd - std::backtrace_rs::backtrace::libunwind::trace::he82d420584fababe
                               at /rustc/15d46b8154c4bad80167c6cbf595c14757a5d34d/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7fe69bf37ffd - std::backtrace_rs::backtrace::trace_unsynchronized::h32e7d2468e45dba3
                               at /rustc/15d46b8154c4bad80167c6cbf595c14757a5d34d/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7fe69bf37ffd - std::sys_common::backtrace::_print_fmt::h592247c5117ee813
                               at /rustc/15d46b8154c4bad80167c6cbf595c14757a5d34d/library/std/src/sys_common/backtrace.rs:66:5
   3:     0x7fe69bf37ffd - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h433eb4d018df726f
                               at /rustc/15d46b8154c4bad80167c6cbf595c14757a5d34d/library/std/src/sys_common/backtrace.rs:45:22
   4:     0x7fe69bf91d1c - core::fmt::write::h047481f0716cc3e7
                               at /rustc/15d46b8154c4bad80167c6cbf595c14757a5d34d/library/core/src/fmt/mod.rs:1194:17
   5:     0x7fe69bf29821 - std::io::Write::write_fmt::h9d7fef21064260e6
                               at /rustc/15d46b8154c4bad80167c6cbf595c14757a5d34d/library/std/src/io/mod.rs:1655:15
   6:     0x7fe69bf3af15 - std::sys_common::backtrace::_print::hbdd7703d4ed4f702
                               at /rustc/15d46b8154c4bad80167c6cbf595c14757a5d34d/library/std/src/sys_common/backtrace.rs:48:5
   7:     0x7fe69bf3af15 - std::sys_common::backtrace::print::h612c74123aa21d26
                               at /rustc/15d46b8154c4bad80167c6cbf595c14757a5d34d/library/std/src/sys_common/backtrace.rs:35:9
   8:     0x7fe69bf3af15 - std::panicking::default_hook::{{closure}}::h2b2c7128b4a12306
                               at /rustc/15d46b8154c4bad80167c6cbf595c14757a5d34d/library/std/src/panicking.rs:295:22
   9:     0x7fe69bf3ab89 - std::panicking::default_hook::hf50ad1ed27ad018d
                               at /rustc/15d46b8154c4bad80167c6cbf595c14757a5d34d/library/std/src/panicking.rs:314:9
  10:     0x7fe69c888d69 - rustc_driver[963d8b28c9e32bce]::DEFAULT_HOOK::{closure#0}::{closure#0}
  11:     0x7fe69bf3b6b0 - std::panicking::rust_panic_with_hook::h4d5445299cf86ef1
                               at /rustc/15d46b8154c4bad80167c6cbf595c14757a5d34d/library/std/src/panicking.rs:702:17
  12:     0x7fe69bf3b4e7 - std::panicking::begin_panic_handler::{{closure}}::h00e400154add04ec
                               at /rustc/15d46b8154c4bad80167c6cbf595c14757a5d34d/library/std/src/panicking.rs:588:13
  13:     0x7fe69bf384b4 - std::sys_common::backtrace::__rust_end_short_backtrace::h29dca1fe55960ae1
                               at /rustc/15d46b8154c4bad80167c6cbf595c14757a5d34d/library/std/src/sys_common/backtrace.rs:138:18
  14:     0x7fe69bf3b219 - rust_begin_unwind
                               at /rustc/15d46b8154c4bad80167c6cbf595c14757a5d34d/library/std/src/panicking.rs:584:5
  15:     0x7fe69beff763 - core::panicking::panic_fmt::hf1439381799f2864
                               at /rustc/15d46b8154c4bad80167c6cbf595c14757a5d34d/library/core/src/panicking.rs:142:14
  16:     0x7fe69ee21d3e - <rustc_middle[4499fc7b7472eef]::traits::specialization_graph::Graph>::parent
  17:     0x7fe69ee228d0 - <rustc_middle[4499fc7b7472eef]::traits::specialization_graph::Ancestors>::leaf_def
  18:     0x7fe69e9fc0da - rustc_trait_selection[537040e93e7a58fe]::traits::project::assoc_def
  19:     0x7fe69e9a44ef - <rustc_infer[fb4585239d5b9492]::infer::InferCtxt>::commit_if_ok::<(), (), rustc_trait_selection[537040e93e7a58fe]::traits::project::assemble_candidates_from_impls::{closure#0}>
  20:     0x7fe69ea0153a - rustc_trait_selection[537040e93e7a58fe]::traits::project::project
  21:     0x7fe69e9fef3b - rustc_trait_selection[537040e93e7a58fe]::traits::project::opt_normalize_projection_type
  22:     0x7fe69e9f9e23 - rustc_trait_selection[537040e93e7a58fe]::traits::project::normalize_projection_type
  23:     0x7fe69e9f85e7 - <rustc_trait_selection[537040e93e7a58fe]::traits::project::AssocTypeNormalizer as rustc_middle[4499fc7b7472eef]::ty::fold::TypeFolder>::fold_ty
  24:     0x7fe69ead53f9 - <rustc_middle[4499fc7b7472eef]::ty::Ty as rustc_middle[4499fc7b7472eef]::ty::fold::TypeFoldable>::super_fold_with::<rustc_trait_selection[537040e93e7a58fe]::traits::project::AssocTypeNormalizer>
  25:     0x7fe69e9f7faf - <rustc_trait_selection[537040e93e7a58fe]::traits::project::AssocTypeNormalizer as rustc_middle[4499fc7b7472eef]::ty::fold::TypeFolder>::fold_ty
  26:     0x7fe69d1562bb - rustc_middle[4499fc7b7472eef]::ty::util::fold_list::<rustc_trait_selection[537040e93e7a58fe]::traits::project::AssocTypeNormalizer, rustc_middle[4499fc7b7472eef]::ty::Ty, <&rustc_middle[4499fc7b7472eef]::ty::list::List<rustc_middle[4499fc7b7472eef]::ty::Ty> as rustc_middle[4499fc7b7472eef]::ty::fold::TypeFoldable>::try_super_fold_with<rustc_trait_selection[537040e93e7a58fe]::traits::project::AssocTypeNormalizer>::{closure#0}>
  27:     0x7fe69d21b33b - <rustc_trait_selection[537040e93e7a58fe]::traits::project::AssocTypeNormalizer>::fold::<rustc_middle[4499fc7b7472eef]::ty::sty::FnSig>
  28:     0x7fe69d2301f8 - rustc_trait_selection[537040e93e7a58fe]::traits::project::normalize::<rustc_middle[4499fc7b7472eef]::ty::sty::FnSig>
  29:     0x7fe69d0447a0 - <rustc_typeck[9eafc09810d5126a]::check::fn_ctxt::FnCtxt>::construct_obligation_for_trait
  30:     0x7fe69d06c70b - <rustc_typeck[9eafc09810d5126a]::check::fn_ctxt::FnCtxt>::lookup_method_in_trait
  31:     0x7fe69d06115a - <rustc_typeck[9eafc09810d5126a]::check::fn_ctxt::FnCtxt>::check_expr_kind
  32:     0x7fe69d015b01 - <rustc_typeck[9eafc09810d5126a]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  33:     0x7fe69d0622ba - <rustc_typeck[9eafc09810d5126a]::check::fn_ctxt::FnCtxt>::check_expr_kind
  34:     0x7fe69d015b01 - <rustc_typeck[9eafc09810d5126a]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  35:     0x7fe69d026443 - <rustc_typeck[9eafc09810d5126a]::check::fn_ctxt::FnCtxt>::check_argument_types
  36:     0x7fe69d0013b0 - <rustc_typeck[9eafc09810d5126a]::check::fn_ctxt::FnCtxt>::confirm_builtin_call
  37:     0x7fe69cffe45b - <rustc_typeck[9eafc09810d5126a]::check::fn_ctxt::FnCtxt>::check_call
  38:     0x7fe69d060957 - <rustc_typeck[9eafc09810d5126a]::check::fn_ctxt::FnCtxt>::check_expr_kind
  39:     0x7fe69d015b01 - <rustc_typeck[9eafc09810d5126a]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  40:     0x7fe69d0461a4 - <rustc_typeck[9eafc09810d5126a]::check::fn_ctxt::FnCtxt>::check_overloaded_binop
  41:     0x7fe69d045f92 - <rustc_typeck[9eafc09810d5126a]::check::fn_ctxt::FnCtxt>::check_binop
  42:     0x7fe69d0618f0 - <rustc_typeck[9eafc09810d5126a]::check::fn_ctxt::FnCtxt>::check_expr_kind
  43:     0x7fe69d015b01 - <rustc_typeck[9eafc09810d5126a]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  44:     0x7fe69d029b85 - <rustc_typeck[9eafc09810d5126a]::check::fn_ctxt::FnCtxt>::check_decl_initializer
  45:     0x7fe69d029c7e - <rustc_typeck[9eafc09810d5126a]::check::fn_ctxt::FnCtxt>::check_decl
  46:     0x7fe69d029e30 - <rustc_typeck[9eafc09810d5126a]::check::fn_ctxt::FnCtxt>::check_stmt
  47:     0x7fe69d02a753 - <rustc_typeck[9eafc09810d5126a]::check::fn_ctxt::FnCtxt>::check_block_with_expected
  48:     0x7fe69d060d32 - <rustc_typeck[9eafc09810d5126a]::check::fn_ctxt::FnCtxt>::check_expr_kind
  49:     0x7fe69d015b01 - <rustc_typeck[9eafc09810d5126a]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  50:     0x7fe69d0165c5 - <rustc_typeck[9eafc09810d5126a]::check::fn_ctxt::FnCtxt>::check_return_expr
  51:     0x7fe69d2cdf4f - rustc_typeck[9eafc09810d5126a]::check::check::check_fn
  52:     0x7fe69d17e15d - <rustc_infer[fb4585239d5b9492]::infer::InferCtxtBuilder>::enter::<&rustc_middle[4499fc7b7472eef]::ty::context::TypeckResults, <rustc_typeck[9eafc09810d5126a]::check::inherited::InheritedBuilder>::enter<rustc_typeck[9eafc09810d5126a]::check::typeck_with_fallback<rustc_typeck[9eafc09810d5126a]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[4499fc7b7472eef]::ty::context::TypeckResults>::{closure#0}>
  53:     0x7fe69d0fe641 - rustc_typeck[9eafc09810d5126a]::check::typeck
  54:     0x7fe69dd30820 - rustc_query_system[10a64a31f4f52c65]::query::plumbing::try_execute_query::<rustc_query_impl[6c9a745188c71205]::plumbing::QueryCtxt, rustc_query_system[10a64a31f4f52c65]::query::caches::DefaultCache<rustc_span[e92932c11291754d]::def_id::LocalDefId, &rustc_middle[4499fc7b7472eef]::ty::context::TypeckResults>>
  55:     0x7fe69de1d2e8 - rustc_query_system[10a64a31f4f52c65]::query::plumbing::get_query::<rustc_query_impl[6c9a745188c71205]::queries::typeck, rustc_query_impl[6c9a745188c71205]::plumbing::QueryCtxt>
  56:     0x7fe69d1e4d78 - <rustc_middle[4499fc7b7472eef]::hir::map::Map>::par_body_owners::<rustc_typeck[9eafc09810d5126a]::check::typeck_item_bodies::{closure#0}>
  57:     0x7fe69d1036ed - rustc_typeck[9eafc09810d5126a]::check::typeck_item_bodies
  58:     0x7fe69dd96387 - rustc_query_system[10a64a31f4f52c65]::query::plumbing::try_execute_query::<rustc_query_impl[6c9a745188c71205]::plumbing::QueryCtxt, rustc_query_system[10a64a31f4f52c65]::query::caches::DefaultCache<(), ()>>
  59:     0x7fe69ddfc3fa - rustc_query_system[10a64a31f4f52c65]::query::plumbing::get_query::<rustc_query_impl[6c9a745188c71205]::queries::typeck_item_bodies, rustc_query_impl[6c9a745188c71205]::plumbing::QueryCtxt>
  60:     0x7fe69d142341 - rustc_typeck[9eafc09810d5126a]::check_crate
  61:     0x7fe69c974a21 - rustc_interface[510514e2c7202ceb]::passes::analysis
  62:     0x7fe69dd89699 - rustc_query_system[10a64a31f4f52c65]::query::plumbing::try_execute_query::<rustc_query_impl[6c9a745188c71205]::plumbing::QueryCtxt, rustc_query_system[10a64a31f4f52c65]::query::caches::DefaultCache<(), core[7b24a5c2c811bf3d]::result::Result<(), rustc_errors[15b2ceb35a572425]::ErrorGuaranteed>>>
  63:     0x7fe69de1d677 - rustc_query_system[10a64a31f4f52c65]::query::plumbing::get_query::<rustc_query_impl[6c9a745188c71205]::queries::analysis, rustc_query_impl[6c9a745188c71205]::plumbing::QueryCtxt>
  64:     0x7fe69c84633b - <rustc_interface[510514e2c7202ceb]::passes::QueryContext>::enter::<rustc_driver[963d8b28c9e32bce]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[7b24a5c2c811bf3d]::result::Result<(), rustc_errors[15b2ceb35a572425]::ErrorGuaranteed>>
  65:     0x7fe69c822051 - <rustc_interface[510514e2c7202ceb]::interface::Compiler>::enter::<rustc_driver[963d8b28c9e32bce]::run_compiler::{closure#1}::{closure#2}, core[7b24a5c2c811bf3d]::result::Result<core[7b24a5c2c811bf3d]::option::Option<rustc_interface[510514e2c7202ceb]::queries::Linker>, rustc_errors[15b2ceb35a572425]::ErrorGuaranteed>>
  66:     0x7fe69c8922e3 - rustc_span[e92932c11291754d]::with_source_map::<core[7b24a5c2c811bf3d]::result::Result<(), rustc_errors[15b2ceb35a572425]::ErrorGuaranteed>, rustc_interface[510514e2c7202ceb]::interface::create_compiler_and_run<core[7b24a5c2c811bf3d]::result::Result<(), rustc_errors[15b2ceb35a572425]::ErrorGuaranteed>, rustc_driver[963d8b28c9e32bce]::run_compiler::{closure#1}>::{closure#1}>
  67:     0x7fe69c822f41 - rustc_interface[510514e2c7202ceb]::interface::create_compiler_and_run::<core[7b24a5c2c811bf3d]::result::Result<(), rustc_errors[15b2ceb35a572425]::ErrorGuaranteed>, rustc_driver[963d8b28c9e32bce]::run_compiler::{closure#1}>
  68:     0x7fe69c83fbc2 - <scoped_tls[8ecdc8189a2436d4]::ScopedKey<rustc_span[e92932c11291754d]::SessionGlobals>>::set::<rustc_interface[510514e2c7202ceb]::interface::run_compiler<core[7b24a5c2c811bf3d]::result::Result<(), rustc_errors[15b2ceb35a572425]::ErrorGuaranteed>, rustc_driver[963d8b28c9e32bce]::run_compiler::{closure#1}>::{closure#0}, core[7b24a5c2c811bf3d]::result::Result<(), rustc_errors[15b2ceb35a572425]::ErrorGuaranteed>>
  69:     0x7fe69c8497ff - std[b270603ab51ba841]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[510514e2c7202ceb]::util::run_in_thread_pool_with_globals<rustc_interface[510514e2c7202ceb]::interface::run_compiler<core[7b24a5c2c811bf3d]::result::Result<(), rustc_errors[15b2ceb35a572425]::ErrorGuaranteed>, rustc_driver[963d8b28c9e32bce]::run_compiler::{closure#1}>::{closure#0}, core[7b24a5c2c811bf3d]::result::Result<(), rustc_errors[15b2ceb35a572425]::ErrorGuaranteed>>::{closure#0}, core[7b24a5c2c811bf3d]::result::Result<(), rustc_errors[15b2ceb35a572425]::ErrorGuaranteed>>
  70:     0x7fe69c84cb39 - <<std[b270603ab51ba841]::thread::Builder>::spawn_unchecked_<rustc_interface[510514e2c7202ceb]::util::run_in_thread_pool_with_globals<rustc_interface[510514e2c7202ceb]::interface::run_compiler<core[7b24a5c2c811bf3d]::result::Result<(), rustc_errors[15b2ceb35a572425]::ErrorGuaranteed>, rustc_driver[963d8b28c9e32bce]::run_compiler::{closure#1}>::{closure#0}, core[7b24a5c2c811bf3d]::result::Result<(), rustc_errors[15b2ceb35a572425]::ErrorGuaranteed>>::{closure#0}, core[7b24a5c2c811bf3d]::result::Result<(), rustc_errors[15b2ceb35a572425]::ErrorGuaranteed>>::{closure#1} as core[7b24a5c2c811bf3d]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  71:     0x7fe69bf45813 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hc769d35e5ff1556a
                               at /rustc/15d46b8154c4bad80167c6cbf595c14757a5d34d/library/alloc/src/boxed.rs:1858:9
  72:     0x7fe69bf45813 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h0f1a8717e1c9f2ce
                               at /rustc/15d46b8154c4bad80167c6cbf595c14757a5d34d/library/alloc/src/boxed.rs:1858:9
  73:     0x7fe69bf45813 - std::sys::unix::thread::Thread::new::thread_start::hfc9cf8ccb9230662
                               at /rustc/15d46b8154c4bad80167c6cbf595c14757a5d34d/library/std/src/sys/unix/thread.rs:108:17
  74:     0x7fe69ba808ca - start_thread
  75:     0x7fe69b5deabd - clone
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.62.0-nightly (15d46b815 2022-04-09) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=1 -C linker=clang -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -Z save-analysis -C prefer-dynamic -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [typeck] type-checking `unix::linux_like::FD_CLR`
#1 [typeck_item_bodies] type-checking all item bodies
#2 [analysis] running analysis passes on this crate
end of query stack
error: internal compiler error: no errors encountered even though `delay_span_bug` issued
error: internal compiler error: TyKind::Error constructed but no error reported
  |
  = note: delayed at compiler/rustc_trait_selection/src/traits/project.rs:1991:37


error: internal compiler error: TyKind::Error constructed but no error reported
  |
  = note: delayed at /rustc/15d46b8154c4bad80167c6cbf595c14757a5d34d/compiler/rustc_middle/src/ty/relate.rs:414:59
error: internal compiler error: TyKind::Error constructed but no error reported
  |
  = note: delayed at compiler/rustc_infer/src/infer/sub.rs:122:31


error: internal compiler error: TyKind::Error constructed but no error reported
  |
  = note: delayed at compiler/rustc_typeck/src/check/coercion.rs:160:49

error: internal compiler error: cat_expr Errd
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.121/src/unix/linux_like/linux/mod.rs:2993:28
     |
2993 | pub const ABS_CNT: usize = ABS_MAX as usize + 1;
     |
     = note: delayed at compiler/rustc_typeck/src/check/regionck.rs:463:31

error: internal compiler error: TyKind::Error constructed but no error reported
error: internal compiler error: TyKind::Error constructed but no error reported
  |
  = note: delayed at compiler/rustc_mir_build/src/build/mod.rs:725:18

error: internal compiler error: mir_const_qualif: MIR had errors
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.121/src/unix/linux_like/linux/mod.rs:2993:1
     |
2993 | pub const ABS_CNT: usize = ABS_MAX as usize + 1;
     |
     = note: delayed at compiler/rustc_mir_transform/src/lib.rs:193:18


error: internal compiler error: PromoteTemps: MIR had errors
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.121/src/unix/linux_like/linux/mod.rs:2993:1
     |
2993 | pub const ABS_CNT: usize = ABS_MAX as usize + 1;
     |
     = note: delayed at compiler/rustc_const_eval/src/transform/promote_consts.rs:53:22


error: internal compiler error: broken MIR in DefId(0:3410 ~ libc[c2e9]::unix::linux_like::linux::ABS_CNT) ("return type"): bad type [type error]
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.121/src/unix/linux_like/linux/mod.rs:2993:1
     |
2993 | pub const ABS_CNT: usize = ABS_MAX as usize + 1;
     |
     = note: delayed at compiler/rustc_borrowck/src/type_check/mod.rs:540:13

error: internal compiler error: TyKind::Error constructed but no error reported
error: internal compiler error: TyKind::Error constructed but no error reported
  |
  = note: delayed at compiler/rustc_borrowck/src/type_check/mod.rs:795:20

error: internal compiler error: broken MIR in DefId(0:3410 ~ libc[c2e9]::unix::linux_like::linux::ABS_CNT) (LocalDecl { mutability: Mut, local_info: None, internal: false, is_block_tail: None, ty: [type error], user_ty: None, source_info: SourceInfo { span: /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.121/src/unix/linux_like/linux/mod.rs:2993:1: 2993:49 (#0), scope: scope[0] } }): bad type [type error]
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.121/src/unix/linux_like/linux/mod.rs:2993:1
     |
2993 | pub const ABS_CNT: usize = ABS_MAX as usize + 1;
     |
     = note: delayed at compiler/rustc_borrowck/src/type_check/mod.rs:540:13

error: internal compiler error: ty::ConstKind::Error constructed but no error reported
error: internal compiler error: ty::ConstKind::Error constructed but no error reported
  |
  = note: delayed at /rustc/15d46b8154c4bad80167c6cbf595c14757a5d34d/compiler/rustc_middle/src/ty/consts.rs:267:52

error: internal compiler error: cat_expr Errd
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.121/src/unix/linux_like/linux/mod.rs:2091:41
     |
2091 | pub const NFNL_MSG_BATCH_END: ::c_int = NLMSG_MIN_TYPE + 1;
     |
     = note: delayed at compiler/rustc_typeck/src/check/regionck.rs:463:31


error: internal compiler error: cat_expr Errd
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.121/src/unix/linux_like/linux/mod.rs:2979:27
     |
2979 | pub const FF_CNT: usize = FF_MAX as usize + 1;
     |
     = note: delayed at compiler/rustc_typeck/src/check/regionck.rs:463:31


error: internal compiler error: cat_expr Errd
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.121/src/unix/linux_like/linux/mod.rs:2983:35
     |
2983 | pub const INPUT_PROP_CNT: usize = INPUT_PROP_MAX as usize + 1;
     |
     = note: delayed at compiler/rustc_typeck/src/check/regionck.rs:463:31


error: internal compiler error: cat_expr Errd
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.121/src/unix/linux_like/linux/mod.rs:2985:27
     |
2985 | pub const EV_CNT: usize = EV_MAX as usize + 1;
     |
     = note: delayed at compiler/rustc_typeck/src/check/regionck.rs:463:31


error: internal compiler error: cat_expr Errd
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.121/src/unix/linux_like/linux/mod.rs:2987:28
     |
2987 | pub const SYN_CNT: usize = SYN_MAX as usize + 1;
     |
     = note: delayed at compiler/rustc_typeck/src/check/regionck.rs:463:31


error: internal compiler error: cat_expr Errd
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.121/src/unix/linux_like/linux/mod.rs:2989:28
     |
2989 | pub const KEY_CNT: usize = KEY_MAX as usize + 1;
     |
     = note: delayed at compiler/rustc_typeck/src/check/regionck.rs:463:31


error: internal compiler error: cat_expr Errd
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.121/src/unix/linux_like/linux/mod.rs:2991:28
     |
2991 | pub const REL_CNT: usize = REL_MAX as usize + 1;
     |
     = note: delayed at compiler/rustc_typeck/src/check/regionck.rs:463:31


error: internal compiler error: cat_expr Errd
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.121/src/unix/linux_like/linux/mod.rs:2995:27
     |
2995 | pub const SW_CNT: usize = SW_MAX as usize + 1;
     |
     = note: delayed at compiler/rustc_typeck/src/check/regionck.rs:463:31


error: internal compiler error: cat_expr Errd
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.121/src/unix/linux_like/linux/mod.rs:2997:28
     |
2997 | pub const MSC_CNT: usize = MSC_MAX as usize + 1;
     |
     = note: delayed at compiler/rustc_typeck/src/check/regionck.rs:463:31


error: internal compiler error: cat_expr Errd
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.121/src/unix/linux_like/linux/mod.rs:2999:28
     |
2999 | pub const LED_CNT: usize = LED_MAX as usize + 1;
     |
     = note: delayed at compiler/rustc_typeck/src/check/regionck.rs:463:31


error: internal compiler error: cat_expr Errd
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.121/src/unix/linux_like/linux/mod.rs:3001:28
     |
3001 | pub const REP_CNT: usize = REP_MAX as usize + 1;
     |
     = note: delayed at compiler/rustc_typeck/src/check/regionck.rs:463:31


error: internal compiler error: cat_expr Errd
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.121/src/unix/linux_like/linux/mod.rs:3003:28
     |
3003 | pub const SND_CNT: usize = SND_MAX as usize + 1;
     |
     = note: delayed at compiler/rustc_typeck/src/check/regionck.rs:463:31


error: internal compiler error: cat_expr Errd
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.121/src/unix/linux_like/linux/gnu/mod.rs:931:40
    |
931 | pub const GENL_ID_VFS_DQUOT: ::c_int = ::NLMSG_MIN_TYPE + 1;
    |
    = note: delayed at compiler/rustc_typeck/src/check/regionck.rs:463:31


error: internal compiler error: cat_expr Errd
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.121/src/unix/linux_like/linux/gnu/mod.rs:932:38
    |
932 | pub const GENL_ID_PMCRAID: ::c_int = ::NLMSG_MIN_TYPE + 2;
    |
    = note: delayed at compiler/rustc_typeck/src/check/regionck.rs:463:31

error: internal compiler error: TyKind::Error constructed but no error reported
---
  = note: delayed at compiler/rustc_typeck/src/check/coercion.rs:930:53

error: internal compiler error: TyKind::Error constructed but no error reported
  |
  = note: delayed at compiler/rustc_typeck/src/check/fn_ctxt/checks.rs:902:27
error: internal compiler error: TyKind::Error constructed but no error reported
  |
  = note: delayed at compiler/rustc_typeck/src/check/coercion.rs:1350:42


error: internal compiler error: TyKind::Error constructed but no error reported
  |
  = note: delayed at compiler/rustc_typeck/src/check/fallback.rs:110:58

error: internal compiler error: expected fullfillment errors
  = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:210:23


error: internal compiler error: cat_expr Errd
     |
267  |  /         macro_rules! const_fn {
267  |  /         macro_rules! const_fn {
268  |  |             ($($(#[$attr:meta])* $({$constness:ident})* fn $i:ident(
269  |  |                         $($arg:ident: $argty:ty),*
270  |  |             ) -> $ret:ty {
276  |  |                 ) -> $ret {
     |  |___________________________^
     |  |___________________________^
277  | ||                     $($body);*
     | ||_________________^
279  |  |             )*)
280  |  |         }
     |  |_________- in this expansion of `const_fn!`
     |  |_________- in this expansion of `const_fn!`
     |
    ::: /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.121/src/unix/linux_like/mod.rs:1486:1
     |
1486 | /  const_fn! {
1487 | |      {const} fn CMSG_ALIGN(len: usize) -> usize {
1488 | |          len + ::mem::size_of::<usize>() - 1 & !(::mem::size_of::<usize>() - 1)
1490 | |  }
     | |__- in this macro invocation
     |
     = note: delayed at compiler/rustc_typeck/src/check/regionck.rs:463:31
     = note: delayed at compiler/rustc_typeck/src/check/regionck.rs:463:31

error: internal compiler error: cat_expr Errd
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.121/src/unix/linux_like/mod.rs:1488:9
     |
1488 |         len + ::mem::size_of::<usize>() - 1 & !(::mem::size_of::<usize>() - 1)
     |
     = note: delayed at compiler/rustc_typeck/src/check/regionck.rs:463:31


error: internal compiler error: cat_expr Errd
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.121/src/unix/linux_like/mod.rs:1488:47
     |
1488 |         len + ::mem::size_of::<usize>() - 1 & !(::mem::size_of::<usize>() - 1)
     |
     = note: delayed at compiler/rustc_typeck/src/check/regionck.rs:463:31


error: internal compiler error: cat_expr Errd
    --> /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.121/src/unix/linux_like/mod.rs:1488:48
     |
1488 |         len + ::mem::size_of::<usize>() - 1 & !(::mem::size_of::<usize>() - 1)
     |
     = note: delayed at compiler/rustc_typeck/src/check/regionck.rs:463:31

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1383:13
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1383:13
stack backtrace:
   0:     0x7fe69bf37ffd - std::backtrace_rs::backtrace::libunwind::trace::he82d420584fababe
                               at /rustc/15d46b8154c4bad80167c6cbf595c14757a5d34d/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7fe69bf37ffd - std::backtrace_rs::backtrace::trace_unsynchronized::h32e7d2468e45dba3
                               at /rustc/15d46b8154c4bad80167c6cbf595c14757a5d34d/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7fe69bf37ffd - std::sys_common::backtrace::_print_fmt::h592247c5117ee813
                               at /rustc/15d46b8154c4bad80167c6cbf595c14757a5d34d/library/std/src/sys_common/backtrace.rs:66:5
   3:     0x7fe69bf37ffd - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h433eb4d018df726f
                               at /rustc/15d46b8154c4bad80167c6cbf595c14757a5d34d/library/std/src/sys_common/backtrace.rs:45:22
   4:     0x7fe69bf91d1c - core::fmt::write::h047481f0716cc3e7
                               at /rustc/15d46b8154c4bad80167c6cbf595c14757a5d34d/library/core/src/fmt/mod.rs:1194:17
   5:     0x7fe69bf29821 - std::io::Write::write_fmt::h9d7fef21064260e6
                               at /rustc/15d46b8154c4bad80167c6cbf595c14757a5d34d/library/std/src/io/mod.rs:1655:15
   6:     0x7fe69bf3af15 - std::sys_common::backtrace::_print::hbdd7703d4ed4f702
                               at /rustc/15d46b8154c4bad80167c6cbf595c14757a5d34d/library/std/src/sys_common/backtrace.rs:48:5
   7:     0x7fe69bf3af15 - std::sys_common::backtrace::print::h612c74123aa21d26
                               at /rustc/15d46b8154c4bad80167c6cbf595c14757a5d34d/library/std/src/sys_common/backtrace.rs:35:9
   8:     0x7fe69bf3af15 - std::panicking::default_hook::{{closure}}::h2b2c7128b4a12306
                               at /rustc/15d46b8154c4bad80167c6cbf595c14757a5d34d/library/std/src/panicking.rs:295:22
   9:     0x7fe69bf3ab89 - std::panicking::default_hook::hf50ad1ed27ad018d
                               at /rustc/15d46b8154c4bad80167c6cbf595c14757a5d34d/library/std/src/panicking.rs:314:9
  10:     0x7fe69c888d69 - rustc_driver[963d8b28c9e32bce]::DEFAULT_HOOK::{closure#0}::{closure#0}
  11:     0x7fe69bf3b6b0 - std::panicking::rust_panic_with_hook::h4d5445299cf86ef1
                               at /rustc/15d46b8154c4bad80167c6cbf595c14757a5d34d/library/std/src/panicking.rs:702:17
  12:     0x7fe69efb1df3 - std[b270603ab51ba841]::panicking::begin_panic::<rustc_errors[15b2ceb35a572425]::ExplicitBug>::{closure#0}
  13:     0x7fe69efb12a6 - std[b270603ab51ba841]::sys_common::backtrace::__rust_end_short_backtrace::<std[b270603ab51ba841]::panicking::begin_panic<rustc_errors[15b2ceb35a572425]::ExplicitBug>::{closure#0}, !>
  14:     0x7fe69c7f948f - std[b270603ab51ba841]::panicking::begin_panic::<rustc_errors[15b2ceb35a572425]::ExplicitBug>
  15:     0x7fe69effb186 - std[b270603ab51ba841]::panic::panic_any::<rustc_errors[15b2ceb35a572425]::ExplicitBug>
  16:     0x7fe69effeed4 - <rustc_errors[15b2ceb35a572425]::HandlerInner as core[7b24a5c2c811bf3d]::ops::drop::Drop>::drop
  17:     0x7fe69c817ad8 - core[7b24a5c2c811bf3d]::ptr::drop_in_place::<rustc_session[50343810cbd57bc8]::parse::ParseSess>
  18:     0x7fe69c81fb5e - <alloc[54dacad8c5cbd630]::rc::Rc<rustc_session[50343810cbd57bc8]::session::Session> as core[7b24a5c2c811bf3d]::ops::drop::Drop>::drop
  19:     0x7fe69c8944bd - core[7b24a5c2c811bf3d]::ptr::drop_in_place::<rustc_interface[510514e2c7202ceb]::interface::Compiler>
  20:     0x7fe69c8929a8 - rustc_span[e92932c11291754d]::with_source_map::<core[7b24a5c2c811bf3d]::result::Result<(), rustc_errors[15b2ceb35a572425]::ErrorGuaranteed>, rustc_interface[510514e2c7202ceb]::interface::create_compiler_and_run<core[7b24a5c2c811bf3d]::result::Result<(), rustc_errors[15b2ceb35a572425]::ErrorGuaranteed>, rustc_driver[963d8b28c9e32bce]::run_compiler::{closure#1}>::{closure#1}>
  21:     0x7fe69c822f41 - rustc_interface[510514e2c7202ceb]::interface::create_compiler_and_run::<core[7b24a5c2c811bf3d]::result::Result<(), rustc_errors[15b2ceb35a572425]::ErrorGuaranteed>, rustc_driver[963d8b28c9e32bce]::run_compiler::{closure#1}>
  22:     0x7fe69c83fbc2 - <scoped_tls[8ecdc8189a2436d4]::ScopedKey<rustc_span[e92932c11291754d]::SessionGlobals>>::set::<rustc_interface[510514e2c7202ceb]::interface::run_compiler<core[7b24a5c2c811bf3d]::result::Result<(), rustc_errors[15b2ceb35a572425]::ErrorGuaranteed>, rustc_driver[963d8b28c9e32bce]::run_compiler::{closure#1}>::{closure#0}, core[7b24a5c2c811bf3d]::result::Result<(), rustc_errors[15b2ceb35a572425]::ErrorGuaranteed>>
  23:     0x7fe69c8497ff - std[b270603ab51ba841]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[510514e2c7202ceb]::util::run_in_thread_pool_with_globals<rustc_interface[510514e2c7202ceb]::interface::run_compiler<core[7b24a5c2c811bf3d]::result::Result<(), rustc_errors[15b2ceb35a572425]::ErrorGuaranteed>, rustc_driver[963d8b28c9e32bce]::run_compiler::{closure#1}>::{closure#0}, core[7b24a5c2c811bf3d]::result::Result<(), rustc_errors[15b2ceb35a572425]::ErrorGuaranteed>>::{closure#0}, core[7b24a5c2c811bf3d]::result::Result<(), rustc_errors[15b2ceb35a572425]::ErrorGuaranteed>>
  24:     0x7fe69c84cb39 - <<std[b270603ab51ba841]::thread::Builder>::spawn_unchecked_<rustc_interface[510514e2c7202ceb]::util::run_in_thread_pool_with_globals<rustc_interface[510514e2c7202ceb]::interface::run_compiler<core[7b24a5c2c811bf3d]::result::Result<(), rustc_errors[15b2ceb35a572425]::ErrorGuaranteed>, rustc_driver[963d8b28c9e32bce]::run_compiler::{closure#1}>::{closure#0}, core[7b24a5c2c811bf3d]::result::Result<(), rustc_errors[15b2ceb35a572425]::ErrorGuaranteed>>::{closure#0}, core[7b24a5c2c811bf3d]::result::Result<(), rustc_errors[15b2ceb35a572425]::ErrorGuaranteed>>::{closure#1} as core[7b24a5c2c811bf3d]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  25:     0x7fe69bf45813 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hc769d35e5ff1556a
                               at /rustc/15d46b8154c4bad80167c6cbf595c14757a5d34d/library/alloc/src/boxed.rs:1858:9
  26:     0x7fe69bf45813 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h0f1a8717e1c9f2ce
                               at /rustc/15d46b8154c4bad80167c6cbf595c14757a5d34d/library/alloc/src/boxed.rs:1858:9
  27:     0x7fe69bf45813 - std::sys::unix::thread::Thread::new::thread_start::hfc9cf8ccb9230662
                               at /rustc/15d46b8154c4bad80167c6cbf595c14757a5d34d/library/std/src/sys/unix/thread.rs:108:17
  28:     0x7fe69ba808ca - start_thread
  29:     0x7fe69b5deabd - clone
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.62.0-nightly (15d46b815 2022-04-09) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=1 -C linker=clang -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -Z save-analysis -C prefer-dynamic -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
end of query stack
thread panicked while panicking. aborting.
[RUSTC-TIMING] libc test:false 0.614
rustc exited with signal: 6 (core dumped)
error: could not compile `libc`
Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name libc /cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.121/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=1 --cfg 'feature="align"' --cfg 'feature="rustc-dep-of-std"' --cfg 'feature="rustc-std-workspace-core"' -C metadata=020914c5936c5f85 -C extra-filename=-020914c5936c5f85 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -C linker=clang -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps --extern rustc_std_workspace_core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_std_workspace_core-2a6a2797f7a73818.rmeta --cap-lints allow -Csymbol-mangling-version=legacy -Zunstable-options -Zunstable-options '--check-cfg=names()' '--check-cfg=values()' '--check-cfg=values(bootstrap)' '--check-cfg=values(stdarch_intel_sde)' '--check-cfg=values(no_fp_fmt_parse)' '--check-cfg=values(no_global_oom_handling)' '--check-cfg=values(freebsd12)' '--check-cfg=values(backtrace_in_libstd)' '--check-cfg=values(target_env,"libnx")' '--check-cfg=values(target_os,"watchos")' '--check-cfg=values(target_arch,"asmjs","spirv","nvptx","nvptx64","le32","xtensa")' '--check-cfg=values(dont_compile_me)' -Zmacro-backtrace -Clink-args=-Wl,-z,origin '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Zsave-analysis -Cprefer-dynamic -Cembed-bitcode=yes '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/")' -Z binary-dep-depinfo --cfg freebsd11 --cfg libc_priv_mod_use --cfg libc_union --cfg libc_const_size_of --cfg libc_align --cfg libc_core_cvoid --cfg libc_packedN --cfg libc_cfg_target_vendor --cfg libc_non_exhaustive --cfg libc_ptr_addr_of --cfg libc_thread_local` (exit status: 254)
Build completed unsuccessfully in 0:11:33
