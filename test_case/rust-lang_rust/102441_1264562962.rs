plain
iii.ii...................i..................i........................................... 264/13618
........................................................................................ 352/13618
........................................................................................ 440/13618
........................................................................................ 528/13618
.........................................................F.F............................ 616/13618
................................................................................i....... 792/13618
........................................................................................ 880/13618
i....................................................................................... 968/13618
........................................................................................ 1056/13618
........................................................................................ 1056/13618
........................................................................................ 1144/13618
........................................................................................ 1232/13618
..................................................................................i..... 1320/13618
........................................................................................ 1408/13618
.......................................................i................................ 1496/13618
........................................................................................ 1584/13618
........................................................................................ 1672/13618
......................................................F........F..F.....F............... 1760/13618
........................................................................................ 1936/13618
...........................................................................i............ 2024/13618
........................................................................................ 2112/13618
........................................................................................ 2200/13618
---
failures:

---- [ui] src/test/ui/associated-types/substs-ppaux.rs#verbose stdout ----

error in revision `verbose`: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/substs-ppaux.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "verbose" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/substs-ppaux.verbose" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "verbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/substs-ppaux.verbose/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: compiler/rustc_middle/src/hir/map/mod.rs:872:18: expected expr, found local let x: () = <i8 as Foo<'static, 'static,  u8>>::bar::<'static, char>; (hir_id=HirId { owner: OwnerId { def_id: DefId(0:17 ~ substs_ppaux[26cf]::foo) }, local_id: 15 })
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1502:9
stack backtrace:
   0:     0x7f847925872e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::he9cf1d92f30cd7c4
   1:     0x7f84792c1588 - core::fmt::write::hb71f33d6dcbbaae1
   1:     0x7f84792c1588 - core::fmt::write::hb71f33d6dcbbaae1
   2:     0x7f8479249651 - std::io::Write::write_fmt::hba03ba06b18d9e62
   3:     0x7f847925b7de - std::panicking::default_hook::{{closure}}::hdef4e1c9c8ad451e
   4:     0x7f847925b499 - std::panicking::default_hook::hc35395d68df0c59f
   5:     0x7f8479c15874 - rustc_driver[f414b4652d8f0863]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f847925bf9d - std::panicking::rust_panic_with_hook::hee5baad676042f5e
   7:     0x7f847c67ccd3 - std[2e8a54ede7eda140]::panicking::begin_panic::<rustc_errors[215d8a3c2f9dfb2d]::ExplicitBug>::{closure#0}
   8:     0x7f847c679476 - std[2e8a54ede7eda140]::sys_common::backtrace::__rust_end_short_backtrace::<std[2e8a54ede7eda140]::panicking::begin_panic<rustc_errors[215d8a3c2f9dfb2d]::ExplicitBug>::{closure#0}, !>
   9:     0x7f8479bb8376 - std[2e8a54ede7eda140]::panicking::begin_panic::<rustc_errors[215d8a3c2f9dfb2d]::ExplicitBug>
  10:     0x7f847c66daa6 - std[2e8a54ede7eda140]::panic::panic_any::<rustc_errors[215d8a3c2f9dfb2d]::ExplicitBug>
  11:     0x7f847c6666b9 - <rustc_errors[215d8a3c2f9dfb2d]::HandlerInner>::bug::<&alloc[13583fc3d0f3e700]::string::String>
  12:     0x7f847c666190 - <rustc_errors[215d8a3c2f9dfb2d]::Handler>::bug::<&alloc[13583fc3d0f3e700]::string::String>
  13:     0x7f847c7f2bfe - rustc_middle[d24c4156abdc8924]::ty::context::tls::with_context_opt::<rustc_middle[d24c4156abdc8924]::ty::context::tls::with_opt<rustc_middle[d24c4156abdc8924]::util::bug::opt_span_bug_fmt<rustc_span[435a2de3bfa5601f]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  14:     0x7f847c7ff4f9 - rustc_middle[d24c4156abdc8924]::util::bug::opt_span_bug_fmt::<rustc_span[435a2de3bfa5601f]::span_encoding::Span>
  15:     0x7f8479bc27b8 - rustc_middle[d24c4156abdc8924]::util::bug::bug_fmt
  16:     0x7f847c72ac03 - <rustc_middle[d24c4156abdc8924]::hir::map::Map>::expect_expr
  17:     0x7f847a0cfe62 - <rustc_hir_analysis[d8ce6c1405752847]::check::fn_ctxt::FnCtxt>::suggest_deref_ref_or_into
  18:     0x7f847a09c8f0 - <rustc_hir_analysis[d8ce6c1405752847]::check::fn_ctxt::FnCtxt>::emit_coerce_suggestions
  19:     0x7f847a10e755 - <rustc_hir_analysis[d8ce6c1405752847]::check::fn_ctxt::FnCtxt>::demand_coerce_diag
  20:     0x7f847a0a29d8 - <rustc_hir_analysis[d8ce6c1405752847]::check::fn_ctxt::FnCtxt>::demand_coerce
  21:     0x7f847a0c7c9c - <rustc_hir_analysis[d8ce6c1405752847]::check::fn_ctxt::FnCtxt>::check_decl
  22:     0x7f847a0c805b - <rustc_hir_analysis[d8ce6c1405752847]::check::fn_ctxt::FnCtxt>::check_stmt
  23:     0x7f847a0c8774 - <rustc_hir_analysis[d8ce6c1405752847]::check::fn_ctxt::FnCtxt>::check_block_with_expected
  24:     0x7f847a10fbdb - <rustc_hir_analysis[d8ce6c1405752847]::check::fn_ctxt::FnCtxt>::check_expr_kind
  25:     0x7f847a0a8dac - <rustc_hir_analysis[d8ce6c1405752847]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  26:     0x7f847a10ec82 - <rustc_hir_analysis[d8ce6c1405752847]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  27:     0x7f847a0aa5ac - <rustc_hir_analysis[d8ce6c1405752847]::check::fn_ctxt::FnCtxt>::check_return_expr
  28:     0x7f847a361cbb - rustc_hir_analysis[d8ce6c1405752847]::check::check::check_fn
  29:     0x7f847a355d63 - <rustc_hir_analysis[d8ce6c1405752847]::check::inherited::InheritedBuilder>::enter::<rustc_hir_analysis[d8ce6c1405752847]::check::typeck_with_fallback<rustc_hir_analysis[d8ce6c1405752847]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[d24c4156abdc8924]::ty::context::TypeckResults>
  30:     0x7f847a21323b - rustc_hir_analysis[d8ce6c1405752847]::check::typeck
  31:     0x7f847b83d71c - rustc_query_system[f5f779109e483cf5]::query::plumbing::try_execute_query::<rustc_query_impl[3cf691212940497a]::plumbing::QueryCtxt, rustc_query_system[f5f779109e483cf5]::query::caches::DefaultCache<rustc_span[435a2de3bfa5601f]::def_id::LocalDefId, &rustc_middle[d24c4156abdc8924]::ty::context::TypeckResults>>
  32:     0x7f847b969ebd - rustc_query_system[f5f779109e483cf5]::query::plumbing::get_query::<rustc_query_impl[3cf691212940497a]::queries::typeck, rustc_query_impl[3cf691212940497a]::plumbing::QueryCtxt>
  33:     0x7f847b47c600 - <rustc_query_impl[3cf691212940497a]::Queries as rustc_middle[d24c4156abdc8924]::ty::query::QueryEngine>::typeck
  34:     0x7f847a20a5e6 - <core[93d0d218afed051e]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[8b8c5aa8aa14aa0c]::sync::par_for_each_in<&[rustc_span[435a2de3bfa5601f]::def_id::LocalDefId], <rustc_middle[d24c4156abdc8924]::hir::map::Map>::par_body_owners<rustc_hir_analysis[d8ce6c1405752847]::check::typeck_item_bodies::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[93d0d218afed051e]::ops::function::FnOnce<()>>::call_once
  35:     0x7f847a41aa99 - std[2e8a54ede7eda140]::panicking::try::<(), core[93d0d218afed051e]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[8b8c5aa8aa14aa0c]::sync::par_for_each_in<&[rustc_span[435a2de3bfa5601f]::def_id::LocalDefId], <rustc_middle[d24c4156abdc8924]::hir::map::Map>::par_body_owners<rustc_hir_analysis[d8ce6c1405752847]::check::typeck_item_bodies::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  36:     0x7f847a13788d - rustc_data_structures[8b8c5aa8aa14aa0c]::sync::par_for_each_in::<&[rustc_span[435a2de3bfa5601f]::def_id::LocalDefId], <rustc_middle[d24c4156abdc8924]::hir::map::Map>::par_body_owners<rustc_hir_analysis[d8ce6c1405752847]::check::typeck_item_bodies::{closure#0}>::{closure#0}>
  37:     0x7f847a217bb7 - rustc_hir_analysis[d8ce6c1405752847]::check::typeck_item_bodies
  38:     0x7f847b892247 - rustc_query_system[f5f779109e483cf5]::query::plumbing::try_execute_query::<rustc_query_impl[3cf691212940497a]::plumbing::QueryCtxt, rustc_query_system[f5f779109e483cf5]::query::caches::DefaultCache<(), ()>>
  39:     0x7f847b9314eb - rustc_query_system[f5f779109e483cf5]::query::plumbing::get_query::<rustc_query_impl[3cf691212940497a]::queries::typeck_item_bodies, rustc_query_impl[3cf691212940497a]::plumbing::QueryCtxt>
  40:     0x7f847b47beaa - <rustc_query_impl[3cf691212940497a]::Queries as rustc_middle[d24c4156abdc8924]::ty::query::QueryEngine>::typeck_item_bodies
  41:     0x7f847a1bf5db - <rustc_session[16cf70e3063e72cf]::session::Session>::time::<(), rustc_hir_analysis[d8ce6c1405752847]::check_crate::{closure#7}>
  42:     0x7f847a44f202 - rustc_hir_analysis[d8ce6c1405752847]::check_crate
  43:     0x7f8479d70081 - rustc_interface[6df570514bc600c5]::passes::analysis
  44:     0x7f847b886cf8 - rustc_query_system[f5f779109e483cf5]::query::plumbing::try_execute_query::<rustc_query_impl[3cf691212940497a]::plumbing::QueryCtxt, rustc_query_system[f5f779109e483cf5]::query::caches::DefaultCache<(), core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>>>
  45:     0x7f847b96a26b - rustc_query_system[f5f779109e483cf5]::query::plumbing::get_query::<rustc_query_impl[3cf691212940497a]::queries::analysis, rustc_query_impl[3cf691212940497a]::plumbing::QueryCtxt>
  46:     0x7f847b4544fa - <rustc_query_impl[3cf691212940497a]::Queries as rustc_middle[d24c4156abdc8924]::ty::query::QueryEngine>::analysis
  47:     0x7f8479c76886 - <rustc_interface[6df570514bc600c5]::passes::QueryContext>::enter::<rustc_driver[f414b4652d8f0863]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>>
  48:     0x7f8479c1991c - rustc_interface[6df570514bc600c5]::interface::create_compiler_and_run::<core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>, rustc_driver[f414b4652d8f0863]::run_compiler::{closure#1}>
  49:     0x7f8479c84f4f - <scoped_tls[799d6291841bce05]::ScopedKey<rustc_span[435a2de3bfa5601f]::SessionGlobals>>::set::<rustc_interface[6df570514bc600c5]::interface::run_compiler<core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>, rustc_driver[f414b4652d8f0863]::run_compiler::{closure#1}>::{closure#0}, core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>>
  50:     0x7f8479c7a49f - std[2e8a54ede7eda140]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[6df570514bc600c5]::util::run_in_thread_pool_with_globals<rustc_interface[6df570514bc600c5]::interface::run_compiler<core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>, rustc_driver[f414b4652d8f0863]::run_compiler::{closure#1}>::{closure#0}, core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>>::{closure#0}, core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>>
  51:     0x7f8479c18191 - std[2e8a54ede7eda140]::panic::catch_unwind::<core[93d0d218afed051e]::panic::unwind_safe::AssertUnwindSafe<<std[2e8a54ede7eda140]::thread::Builder>::spawn_unchecked_<rustc_interface[6df570514bc600c5]::util::run_in_thread_pool_with_globals<rustc_interface[6df570514bc600c5]::interface::run_compiler<core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>, rustc_driver[f414b4652d8f0863]::run_compiler::{closure#1}>::{closure#0}, core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>>::{closure#0}, core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>>
  52:     0x7f8479c7e090 - <<std[2e8a54ede7eda140]::thread::Builder>::spawn_unchecked_<rustc_interface[6df570514bc600c5]::util::run_in_thread_pool_with_globals<rustc_interface[6df570514bc600c5]::interface::run_compiler<core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>, rustc_driver[f414b4652d8f0863]::run_compiler::{closure#1}>::{closure#0}, core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>>::{closure#0}, core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>>::{closure#1} as core[93d0d218afed051e]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  53:     0x7f8479268b35 - std::sys::unix::thread::Thread::new::thread_start::h781ed01b585563bf
  54:     0x7f8479003b43 - <unknown>
  55:     0x7f8479095a00 - <unknown>
  56:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.66.0-nightly (7f7051991 2022-10-02) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 -Z verbose
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `foo` [typeck]
#1 [typeck_item_bodies] type-checking all item bodies [typeck_item_bodies]
#2 [analysis] running analysis passes on this crate [analysis]
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/associated-types/substs-ppaux.rs#normal stdout ----

error in revision `normal`: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/substs-ppaux.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "normal" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/substs-ppaux.normal" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/substs-ppaux.normal/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: compiler/rustc_middle/src/hir/map/mod.rs:872:18: expected expr, found local let x: () = <i8 as Foo<'static, 'static,  u8>>::bar::<'static, char>; (hir_id=HirId { owner: OwnerId { def_id: DefId(0:17 ~ substs_ppaux[26cf]::foo) }, local_id: 15 })
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1502:9
stack backtrace:
   0:     0x7f0c98ffb72e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::he9cf1d92f30cd7c4
   1:     0x7f0c99064588 - core::fmt::write::hb71f33d6dcbbaae1
   1:     0x7f0c99064588 - core::fmt::write::hb71f33d6dcbbaae1
   2:     0x7f0c98fec651 - std::io::Write::write_fmt::hba03ba06b18d9e62
   3:     0x7f0c98ffe7de - std::panicking::default_hook::{{closure}}::hdef4e1c9c8ad451e
   4:     0x7f0c98ffe499 - std::panicking::default_hook::hc35395d68df0c59f
   5:     0x7f0c999b8874 - rustc_driver[f414b4652d8f0863]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f0c98ffef9d - std::panicking::rust_panic_with_hook::hee5baad676042f5e
   7:     0x7f0c9c41fcd3 - std[2e8a54ede7eda140]::panicking::begin_panic::<rustc_errors[215d8a3c2f9dfb2d]::ExplicitBug>::{closure#0}
   8:     0x7f0c9c41c476 - std[2e8a54ede7eda140]::sys_common::backtrace::__rust_end_short_backtrace::<std[2e8a54ede7eda140]::panicking::begin_panic<rustc_errors[215d8a3c2f9dfb2d]::ExplicitBug>::{closure#0}, !>
   9:     0x7f0c9995b376 - std[2e8a54ede7eda140]::panicking::begin_panic::<rustc_errors[215d8a3c2f9dfb2d]::ExplicitBug>
  10:     0x7f0c9c410aa6 - std[2e8a54ede7eda140]::panic::panic_any::<rustc_errors[215d8a3c2f9dfb2d]::ExplicitBug>
  11:     0x7f0c9c4096b9 - <rustc_errors[215d8a3c2f9dfb2d]::HandlerInner>::bug::<&alloc[13583fc3d0f3e700]::string::String>
  12:     0x7f0c9c409190 - <rustc_errors[215d8a3c2f9dfb2d]::Handler>::bug::<&alloc[13583fc3d0f3e700]::string::String>
  13:     0x7f0c9c595bfe - rustc_middle[d24c4156abdc8924]::ty::context::tls::with_context_opt::<rustc_middle[d24c4156abdc8924]::ty::context::tls::with_opt<rustc_middle[d24c4156abdc8924]::util::bug::opt_span_bug_fmt<rustc_span[435a2de3bfa5601f]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  14:     0x7f0c9c5a24f9 - rustc_middle[d24c4156abdc8924]::util::bug::opt_span_bug_fmt::<rustc_span[435a2de3bfa5601f]::span_encoding::Span>
  15:     0x7f0c999657b8 - rustc_middle[d24c4156abdc8924]::util::bug::bug_fmt
  16:     0x7f0c9c4cdc03 - <rustc_middle[d24c4156abdc8924]::hir::map::Map>::expect_expr
  17:     0x7f0c99e72e62 - <rustc_hir_analysis[d8ce6c1405752847]::check::fn_ctxt::FnCtxt>::suggest_deref_ref_or_into
  18:     0x7f0c99e3f8f0 - <rustc_hir_analysis[d8ce6c1405752847]::check::fn_ctxt::FnCtxt>::emit_coerce_suggestions
  19:     0x7f0c99eb1755 - <rustc_hir_analysis[d8ce6c1405752847]::check::fn_ctxt::FnCtxt>::demand_coerce_diag
  20:     0x7f0c99e459d8 - <rustc_hir_analysis[d8ce6c1405752847]::check::fn_ctxt::FnCtxt>::demand_coerce
  21:     0x7f0c99e6ac9c - <rustc_hir_analysis[d8ce6c1405752847]::check::fn_ctxt::FnCtxt>::check_decl
  22:     0x7f0c99e6b05b - <rustc_hir_analysis[d8ce6c1405752847]::check::fn_ctxt::FnCtxt>::check_stmt
  23:     0x7f0c99e6b774 - <rustc_hir_analysis[d8ce6c1405752847]::check::fn_ctxt::FnCtxt>::check_block_with_expected
  24:     0x7f0c99eb2bdb - <rustc_hir_analysis[d8ce6c1405752847]::check::fn_ctxt::FnCtxt>::check_expr_kind
  25:     0x7f0c99e4bdac - <rustc_hir_analysis[d8ce6c1405752847]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  26:     0x7f0c99eb1c82 - <rustc_hir_analysis[d8ce6c1405752847]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  27:     0x7f0c99e4d5ac - <rustc_hir_analysis[d8ce6c1405752847]::check::fn_ctxt::FnCtxt>::check_return_expr
  28:     0x7f0c9a104cbb - rustc_hir_analysis[d8ce6c1405752847]::check::check::check_fn
  29:     0x7f0c9a0f8d63 - <rustc_hir_analysis[d8ce6c1405752847]::check::inherited::InheritedBuilder>::enter::<rustc_hir_analysis[d8ce6c1405752847]::check::typeck_with_fallback<rustc_hir_analysis[d8ce6c1405752847]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[d24c4156abdc8924]::ty::context::TypeckResults>
  30:     0x7f0c99fb623b - rustc_hir_analysis[d8ce6c1405752847]::check::typeck
  31:     0x7f0c9b5e071c - rustc_query_system[f5f779109e483cf5]::query::plumbing::try_execute_query::<rustc_query_impl[3cf691212940497a]::plumbing::QueryCtxt, rustc_query_system[f5f779109e483cf5]::query::caches::DefaultCache<rustc_span[435a2de3bfa5601f]::def_id::LocalDefId, &rustc_middle[d24c4156abdc8924]::ty::context::TypeckResults>>
  32:     0x7f0c9b70cebd - rustc_query_system[f5f779109e483cf5]::query::plumbing::get_query::<rustc_query_impl[3cf691212940497a]::queries::typeck, rustc_query_impl[3cf691212940497a]::plumbing::QueryCtxt>
  33:     0x7f0c9b21f600 - <rustc_query_impl[3cf691212940497a]::Queries as rustc_middle[d24c4156abdc8924]::ty::query::QueryEngine>::typeck
  34:     0x7f0c99fad5e6 - <core[93d0d218afed051e]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[8b8c5aa8aa14aa0c]::sync::par_for_each_in<&[rustc_span[435a2de3bfa5601f]::def_id::LocalDefId], <rustc_middle[d24c4156abdc8924]::hir::map::Map>::par_body_owners<rustc_hir_analysis[d8ce6c1405752847]::check::typeck_item_bodies::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[93d0d218afed051e]::ops::function::FnOnce<()>>::call_once
  35:     0x7f0c9a1bda99 - std[2e8a54ede7eda140]::panicking::try::<(), core[93d0d218afed051e]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[8b8c5aa8aa14aa0c]::sync::par_for_each_in<&[rustc_span[435a2de3bfa5601f]::def_id::LocalDefId], <rustc_middle[d24c4156abdc8924]::hir::map::Map>::par_body_owners<rustc_hir_analysis[d8ce6c1405752847]::check::typeck_item_bodies::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  36:     0x7f0c99eda88d - rustc_data_structures[8b8c5aa8aa14aa0c]::sync::par_for_each_in::<&[rustc_span[435a2de3bfa5601f]::def_id::LocalDefId], <rustc_middle[d24c4156abdc8924]::hir::map::Map>::par_body_owners<rustc_hir_analysis[d8ce6c1405752847]::check::typeck_item_bodies::{closure#0}>::{closure#0}>
  37:     0x7f0c99fbabb7 - rustc_hir_analysis[d8ce6c1405752847]::check::typeck_item_bodies
  38:     0x7f0c9b635247 - rustc_query_system[f5f779109e483cf5]::query::plumbing::try_execute_query::<rustc_query_impl[3cf691212940497a]::plumbing::QueryCtxt, rustc_query_system[f5f779109e483cf5]::query::caches::DefaultCache<(), ()>>
  39:     0x7f0c9b6d44eb - rustc_query_system[f5f779109e483cf5]::query::plumbing::get_query::<rustc_query_impl[3cf691212940497a]::queries::typeck_item_bodies, rustc_query_impl[3cf691212940497a]::plumbing::QueryCtxt>
  40:     0x7f0c9b21eeaa - <rustc_query_impl[3cf691212940497a]::Queries as rustc_middle[d24c4156abdc8924]::ty::query::QueryEngine>::typeck_item_bodies
  41:     0x7f0c99f625db - <rustc_session[16cf70e3063e72cf]::session::Session>::time::<(), rustc_hir_analysis[d8ce6c1405752847]::check_crate::{closure#7}>
  42:     0x7f0c9a1f2202 - rustc_hir_analysis[d8ce6c1405752847]::check_crate
  43:     0x7f0c99b13081 - rustc_interface[6df570514bc600c5]::passes::analysis
  44:     0x7f0c9b629cf8 - rustc_query_system[f5f779109e483cf5]::query::plumbing::try_execute_query::<rustc_query_impl[3cf691212940497a]::plumbing::QueryCtxt, rustc_query_system[f5f779109e483cf5]::query::caches::DefaultCache<(), core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>>>
  45:     0x7f0c9b70d26b - rustc_query_system[f5f779109e483cf5]::query::plumbing::get_query::<rustc_query_impl[3cf691212940497a]::queries::analysis, rustc_query_impl[3cf691212940497a]::plumbing::QueryCtxt>
  46:     0x7f0c9b1f74fa - <rustc_query_impl[3cf691212940497a]::Queries as rustc_middle[d24c4156abdc8924]::ty::query::QueryEngine>::analysis
  47:     0x7f0c99a19886 - <rustc_interface[6df570514bc600c5]::passes::QueryContext>::enter::<rustc_driver[f414b4652d8f0863]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>>
  48:     0x7f0c999bc91c - rustc_interface[6df570514bc600c5]::interface::create_compiler_and_run::<core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>, rustc_driver[f414b4652d8f0863]::run_compiler::{closure#1}>
  49:     0x7f0c99a27f4f - <scoped_tls[799d6291841bce05]::ScopedKey<rustc_span[435a2de3bfa5601f]::SessionGlobals>>::set::<rustc_interface[6df570514bc600c5]::interface::run_compiler<core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>, rustc_driver[f414b4652d8f0863]::run_compiler::{closure#1}>::{closure#0}, core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>>
  50:     0x7f0c99a1d49f - std[2e8a54ede7eda140]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[6df570514bc600c5]::util::run_in_thread_pool_with_globals<rustc_interface[6df570514bc600c5]::interface::run_compiler<core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>, rustc_driver[f414b4652d8f0863]::run_compiler::{closure#1}>::{closure#0}, core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>>::{closure#0}, core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>>
  51:     0x7f0c999bb191 - std[2e8a54ede7eda140]::panic::catch_unwind::<core[93d0d218afed051e]::panic::unwind_safe::AssertUnwindSafe<<std[2e8a54ede7eda140]::thread::Builder>::spawn_unchecked_<rustc_interface[6df570514bc600c5]::util::run_in_thread_pool_with_globals<rustc_interface[6df570514bc600c5]::interface::run_compiler<core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>, rustc_driver[f414b4652d8f0863]::run_compiler::{closure#1}>::{closure#0}, core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>>::{closure#0}, core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>>
  52:     0x7f0c99a21090 - <<std[2e8a54ede7eda140]::thread::Builder>::spawn_unchecked_<rustc_interface[6df570514bc600c5]::util::run_in_thread_pool_with_globals<rustc_interface[6df570514bc600c5]::interface::run_compiler<core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>, rustc_driver[f414b4652d8f0863]::run_compiler::{closure#1}>::{closure#0}, core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>>::{closure#0}, core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>>::{closure#1} as core[93d0d218afed051e]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  53:     0x7f0c9900bb35 - std::sys::unix::thread::Thread::new::thread_start::h781ed01b585563bf
  54:     0x7f0c98da6b43 - <unknown>
  55:     0x7f0c98e38a00 - <unknown>
  56:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.66.0-nightly (7f7051991 2022-10-02) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `foo`
#1 [typeck_item_bodies] type-checking all item bodies
#2 [analysis] running analysis passes on this crate
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/closures/issue-90871.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/closures/issue-90871.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/issue-90871" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/issue-90871/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0412]: cannot find type `n` in this scope
   |
   |
LL |     2: n([u8; || 1])
   |        ^ expecting a type here because of type ascription

error: internal compiler error: compiler/rustc_middle/src/hir/map/mod.rs:872:18: expected expr, found const || 1 (hir_id=HirId { owner: OwnerId { def_id: DefId(0:3 ~ issue_90871[d52d]::main) }, local_id: 5 })
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1502:9
stack backtrace:
   0:     0x7f929a3d472e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::he9cf1d92f30cd7c4
   1:     0x7f929a43d588 - core::fmt::write::hb71f33d6dcbbaae1
   1:     0x7f929a43d588 - core::fmt::write::hb71f33d6dcbbaae1
   2:     0x7f929a3c5651 - std::io::Write::write_fmt::hba03ba06b18d9e62
   3:     0x7f929a3d77de - std::panicking::default_hook::{{closure}}::hdef4e1c9c8ad451e
   4:     0x7f929a3d7499 - std::panicking::default_hook::hc35395d68df0c59f
   5:     0x7f929ad91874 - rustc_driver[f414b4652d8f0863]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f929a3d7f9d - std::panicking::rust_panic_with_hook::hee5baad676042f5e
   7:     0x7f929d7f8cd3 - std[2e8a54ede7eda140]::panicking::begin_panic::<rustc_errors[215d8a3c2f9dfb2d]::ExplicitBug>::{closure#0}
   8:     0x7f929d7f5476 - std[2e8a54ede7eda140]::sys_common::backtrace::__rust_end_short_backtrace::<std[2e8a54ede7eda140]::panicking::begin_panic<rustc_errors[215d8a3c2f9dfb2d]::ExplicitBug>::{closure#0}, !>
   9:     0x7f929ad34376 - std[2e8a54ede7eda140]::panicking::begin_panic::<rustc_errors[215d8a3c2f9dfb2d]::ExplicitBug>
  10:     0x7f929d7e9aa6 - std[2e8a54ede7eda140]::panic::panic_any::<rustc_errors[215d8a3c2f9dfb2d]::ExplicitBug>
  11:     0x7f929d7e26b9 - <rustc_errors[215d8a3c2f9dfb2d]::HandlerInner>::bug::<&alloc[13583fc3d0f3e700]::string::String>
  12:     0x7f929d7e2190 - <rustc_errors[215d8a3c2f9dfb2d]::Handler>::bug::<&alloc[13583fc3d0f3e700]::string::String>
  13:     0x7f929d96ebfe - rustc_middle[d24c4156abdc8924]::ty::context::tls::with_context_opt::<rustc_middle[d24c4156abdc8924]::ty::context::tls::with_opt<rustc_middle[d24c4156abdc8924]::util::bug::opt_span_bug_fmt<rustc_span[435a2de3bfa5601f]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  14:     0x7f929d97b4f9 - rustc_middle[d24c4156abdc8924]::util::bug::opt_span_bug_fmt::<rustc_span[435a2de3bfa5601f]::span_encoding::Span>
  15:     0x7f929ad3e7b8 - rustc_middle[d24c4156abdc8924]::util::bug::bug_fmt
  16:     0x7f929d8a6c03 - <rustc_middle[d24c4156abdc8924]::hir::map::Map>::expect_expr
  17:     0x7f929b24be62 - <rustc_hir_analysis[d8ce6c1405752847]::check::fn_ctxt::FnCtxt>::suggest_deref_ref_or_into
  18:     0x7f929b2188f0 - <rustc_hir_analysis[d8ce6c1405752847]::check::fn_ctxt::FnCtxt>::emit_coerce_suggestions
  19:     0x7f929b28a755 - <rustc_hir_analysis[d8ce6c1405752847]::check::fn_ctxt::FnCtxt>::demand_coerce_diag
  20:     0x7f929b21e9d8 - <rustc_hir_analysis[d8ce6c1405752847]::check::fn_ctxt::FnCtxt>::demand_coerce
  21:     0x7f929b4d1f1e - <rustc_hir_analysis[d8ce6c1405752847]::check::inherited::InheritedBuilder>::enter::<rustc_hir_analysis[d8ce6c1405752847]::check::typeck_with_fallback<rustc_hir_analysis[d8ce6c1405752847]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[d24c4156abdc8924]::ty::context::TypeckResults>
  22:     0x7f929b38f23b - rustc_hir_analysis[d8ce6c1405752847]::check::typeck
  23:     0x7f929c9b971c - rustc_query_system[f5f779109e483cf5]::query::plumbing::try_execute_query::<rustc_query_impl[3cf691212940497a]::plumbing::QueryCtxt, rustc_query_system[f5f779109e483cf5]::query::caches::DefaultCache<rustc_span[435a2de3bfa5601f]::def_id::LocalDefId, &rustc_middle[d24c4156abdc8924]::ty::context::TypeckResults>>
  24:     0x7f929cae5ebd - rustc_query_system[f5f779109e483cf5]::query::plumbing::get_query::<rustc_query_impl[3cf691212940497a]::queries::typeck, rustc_query_impl[3cf691212940497a]::plumbing::QueryCtxt>
  25:     0x7f929c5f8600 - <rustc_query_impl[3cf691212940497a]::Queries as rustc_middle[d24c4156abdc8924]::ty::query::QueryEngine>::typeck
  26:     0x7f929b3865e6 - <core[93d0d218afed051e]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[8b8c5aa8aa14aa0c]::sync::par_for_each_in<&[rustc_span[435a2de3bfa5601f]::def_id::LocalDefId], <rustc_middle[d24c4156abdc8924]::hir::map::Map>::par_body_owners<rustc_hir_analysis[d8ce6c1405752847]::check::typeck_item_bodies::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[93d0d218afed051e]::ops::function::FnOnce<()>>::call_once
  27:     0x7f929b596a99 - std[2e8a54ede7eda140]::panicking::try::<(), core[93d0d218afed051e]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[8b8c5aa8aa14aa0c]::sync::par_for_each_in<&[rustc_span[435a2de3bfa5601f]::def_id::LocalDefId], <rustc_middle[d24c4156abdc8924]::hir::map::Map>::par_body_owners<rustc_hir_analysis[d8ce6c1405752847]::check::typeck_item_bodies::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  28:     0x7f929b2b388d - rustc_data_structures[8b8c5aa8aa14aa0c]::sync::par_for_each_in::<&[rustc_span[435a2de3bfa5601f]::def_id::LocalDefId], <rustc_middle[d24c4156abdc8924]::hir::map::Map>::par_body_owners<rustc_hir_analysis[d8ce6c1405752847]::check::typeck_item_bodies::{closure#0}>::{closure#0}>
  29:     0x7f929b393bb7 - rustc_hir_analysis[d8ce6c1405752847]::check::typeck_item_bodies
  30:     0x7f929ca0e247 - rustc_query_system[f5f779109e483cf5]::query::plumbing::try_execute_query::<rustc_query_impl[3cf691212940497a]::plumbing::QueryCtxt, rustc_query_system[f5f779109e483cf5]::query::caches::DefaultCache<(), ()>>
  31:     0x7f929caad4eb - rustc_query_system[f5f779109e483cf5]::query::plumbing::get_query::<rustc_query_impl[3cf691212940497a]::queries::typeck_item_bodies, rustc_query_impl[3cf691212940497a]::plumbing::QueryCtxt>
  32:     0x7f929c5f7eaa - <rustc_query_impl[3cf691212940497a]::Queries as rustc_middle[d24c4156abdc8924]::ty::query::QueryEngine>::typeck_item_bodies
  33:     0x7f929b33b5db - <rustc_session[16cf70e3063e72cf]::session::Session>::time::<(), rustc_hir_analysis[d8ce6c1405752847]::check_crate::{closure#7}>
  34:     0x7f929b5cb202 - rustc_hir_analysis[d8ce6c1405752847]::check_crate
  35:     0x7f929aeec081 - rustc_interface[6df570514bc600c5]::passes::analysis
  36:     0x7f929ca02cf8 - rustc_query_system[f5f779109e483cf5]::query::plumbing::try_execute_query::<rustc_query_impl[3cf691212940497a]::plumbing::QueryCtxt, rustc_query_system[f5f779109e483cf5]::query::caches::DefaultCache<(), core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>>>
  37:     0x7f929cae626b - rustc_query_system[f5f779109e483cf5]::query::plumbing::get_query::<rustc_query_impl[3cf691212940497a]::queries::analysis, rustc_query_impl[3cf691212940497a]::plumbing::QueryCtxt>
  38:     0x7f929c5d04fa - <rustc_query_impl[3cf691212940497a]::Queries as rustc_middle[d24c4156abdc8924]::ty::query::QueryEngine>::analysis
  39:     0x7f929adf2886 - <rustc_interface[6df570514bc600c5]::passes::QueryContext>::enter::<rustc_driver[f414b4652d8f0863]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>>
  40:     0x7f929ad9591c - rustc_interface[6df570514bc600c5]::interface::create_compiler_and_run::<core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>, rustc_driver[f414b4652d8f0863]::run_compiler::{closure#1}>
  41:     0x7f929ae00f4f - <scoped_tls[799d6291841bce05]::ScopedKey<rustc_span[435a2de3bfa5601f]::SessionGlobals>>::set::<rustc_interface[6df570514bc600c5]::interface::run_compiler<core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>, rustc_driver[f414b4652d8f0863]::run_compiler::{closure#1}>::{closure#0}, core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>>
  42:     0x7f929adf649f - std[2e8a54ede7eda140]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[6df570514bc600c5]::util::run_in_thread_pool_with_globals<rustc_interface[6df570514bc600c5]::interface::run_compiler<core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>, rustc_driver[f414b4652d8f0863]::run_compiler::{closure#1}>::{closure#0}, core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>>::{closure#0}, core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>>
  43:     0x7f929ad94191 - std[2e8a54ede7eda140]::panic::catch_unwind::<core[93d0d218afed051e]::panic::unwind_safe::AssertUnwindSafe<<std[2e8a54ede7eda140]::thread::Builder>::spawn_unchecked_<rustc_interface[6df570514bc600c5]::util::run_in_thread_pool_with_globals<rustc_interface[6df570514bc600c5]::interface::run_compiler<core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>, rustc_driver[f414b4652d8f0863]::run_compiler::{closure#1}>::{closure#0}, core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>>::{closure#0}, core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>>
  44:     0x7f929adfa090 - <<std[2e8a54ede7eda140]::thread::Builder>::spawn_unchecked_<rustc_interface[6df570514bc600c5]::util::run_in_thread_pool_with_globals<rustc_interface[6df570514bc600c5]::interface::run_compiler<core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>, rustc_driver[f414b4652d8f0863]::run_compiler::{closure#1}>::{closure#0}, core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>>::{closure#0}, core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>>::{closure#1} as core[93d0d218afed051e]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  45:     0x7f929a3e4b35 - std::sys::unix::thread::Thread::new::thread_start::h781ed01b585563bf
  46:     0x7f929a17fb43 - <unknown>
  47:     0x7f929a211a00 - <unknown>
  48:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.66.0-nightly (7f7051991 2022-10-02) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `main::{constant#0}`
#1 [typeck_item_bodies] type-checking all item bodies
#2 [analysis] running analysis passes on this crate
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0412`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/closures/print/closure-print-generic-2.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/closures/print/closure-print-generic-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/print/closure-print-generic-2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/print/closure-print-generic-2/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: compiler/rustc_middle/src/hir/map/mod.rs:872:18: expected expr, found local let c1: () = c; (hir_id=HirId { owner: OwnerId { def_id: DefId(0:4 ~ closure_print_generic_2[7906]::mod1::f) }, local_id: 60 })
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1502:9
stack backtrace:
   0:     0x7faaa4c4472e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::he9cf1d92f30cd7c4
   1:     0x7faaa4cad588 - core::fmt::write::hb71f33d6dcbbaae1
   1:     0x7faaa4cad588 - core::fmt::write::hb71f33d6dcbbaae1
   2:     0x7faaa4c35651 - std::io::Write::write_fmt::hba03ba06b18d9e62
   3:     0x7faaa4c477de - std::panicking::default_hook::{{closure}}::hdef4e1c9c8ad451e
   4:     0x7faaa4c47499 - std::panicking::default_hook::hc35395d68df0c59f
   5:     0x7faaa5601874 - rustc_driver[f414b4652d8f0863]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7faaa4c47f9d - std::panicking::rust_panic_with_hook::hee5baad676042f5e
   7:     0x7faaa8068cd3 - std[2e8a54ede7eda140]::panicking::begin_panic::<rustc_errors[215d8a3c2f9dfb2d]::ExplicitBug>::{closure#0}
   8:     0x7faaa8065476 - std[2e8a54ede7eda140]::sys_common::backtrace::__rust_end_short_backtrace::<std[2e8a54ede7eda140]::panicking::begin_panic<rustc_errors[215d8a3c2f9dfb2d]::ExplicitBug>::{closure#0}, !>
   9:     0x7faaa55a4376 - std[2e8a54ede7eda140]::panicking::begin_panic::<rustc_errors[215d8a3c2f9dfb2d]::ExplicitBug>
  10:     0x7faaa8059aa6 - std[2e8a54ede7eda140]::panic::panic_any::<rustc_errors[215d8a3c2f9dfb2d]::ExplicitBug>
  11:     0x7faaa80526b9 - <rustc_errors[215d8a3c2f9dfb2d]::HandlerInner>::bug::<&alloc[13583fc3d0f3e700]::string::String>
  12:     0x7faaa8052190 - <rustc_errors[215d8a3c2f9dfb2d]::Handler>::bug::<&alloc[13583fc3d0f3e700]::string::String>
  13:     0x7faaa81debfe - rustc_middle[d24c4156abdc8924]::ty::context::tls::with_context_opt::<rustc_middle[d24c4156abdc8924]::ty::context::tls::with_opt<rustc_middle[d24c4156abdc8924]::util::bug::opt_span_bug_fmt<rustc_span[435a2de3bfa5601f]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  14:     0x7faaa81eb4f9 - rustc_middle[d24c4156abdc8924]::util::bug::opt_span_bug_fmt::<rustc_span[435a2de3bfa5601f]::span_encoding::Span>
  15:     0x7faaa55ae7b8 - rustc_middle[d24c4156abdc8924]::util::bug::bug_fmt
  16:     0x7faaa8116c03 - <rustc_middle[d24c4156abdc8924]::hir::map::Map>::expect_expr
  17:     0x7faaa5abbe62 - <rustc_hir_analysis[d8ce6c1405752847]::check::fn_ctxt::FnCtxt>::suggest_deref_ref_or_into
  18:     0x7faaa5a888f0 - <rustc_hir_analysis[d8ce6c1405752847]::check::fn_ctxt::FnCtxt>::emit_coerce_suggestions
  19:     0x7faaa5afa755 - <rustc_hir_analysis[d8ce6c1405752847]::check::fn_ctxt::FnCtxt>::demand_coerce_diag
  20:     0x7faaa5a8e9d8 - <rustc_hir_analysis[d8ce6c1405752847]::check::fn_ctxt::FnCtxt>::demand_coerce
  21:     0x7faaa5ab3c9c - <rustc_hir_analysis[d8ce6c1405752847]::check::fn_ctxt::FnCtxt>::check_decl
  22:     0x7faaa5ab405b - <rustc_hir_analysis[d8ce6c1405752847]::check::fn_ctxt::FnCtxt>::check_stmt
  23:     0x7faaa5ab4774 - <rustc_hir_analysis[d8ce6c1405752847]::check::fn_ctxt::FnCtxt>::check_block_with_expected
  24:     0x7faaa5afbbdb - <rustc_hir_analysis[d8ce6c1405752847]::check::fn_ctxt::FnCtxt>::check_expr_kind
  25:     0x7faaa5a94dac - <rustc_hir_analysis[d8ce6c1405752847]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  26:     0x7faaa5afac82 - <rustc_hir_analysis[d8ce6c1405752847]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  27:     0x7faaa5a965ac - <rustc_hir_analysis[d8ce6c1405752847]::check::fn_ctxt::FnCtxt>::check_return_expr
  28:     0x7faaa5d4dcbb - rustc_hir_analysis[d8ce6c1405752847]::check::check::check_fn
  29:     0x7faaa5d41d63 - <rustc_hir_analysis[d8ce6c1405752847]::check::inherited::InheritedBuilder>::enter::<rustc_hir_analysis[d8ce6c1405752847]::check::typeck_with_fallback<rustc_hir_analysis[d8ce6c1405752847]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[d24c4156abdc8924]::ty::context::TypeckResults>
  30:     0x7faaa5bff23b - rustc_hir_analysis[d8ce6c1405752847]::check::typeck
  31:     0x7faaa722971c - rustc_query_system[f5f779109e483cf5]::query::plumbing::try_execute_query::<rustc_query_impl[3cf691212940497a]::plumbing::QueryCtxt, rustc_query_system[f5f779109e483cf5]::query::caches::DefaultCache<rustc_span[435a2de3bfa5601f]::def_id::LocalDefId, &rustc_middle[d24c4156abdc8924]::ty::context::TypeckResults>>
  32:     0x7faaa7355ebd - rustc_query_system[f5f779109e483cf5]::query::plumbing::get_query::<rustc_query_impl[3cf691212940497a]::queries::typeck, rustc_query_impl[3cf691212940497a]::plumbing::QueryCtxt>
  33:     0x7faaa6e68600 - <rustc_query_impl[3cf691212940497a]::Queries as rustc_middle[d24c4156abdc8924]::ty::query::QueryEngine>::typeck
  34:     0x7faaa5bf65e6 - <core[93d0d218afed051e]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[8b8c5aa8aa14aa0c]::sync::par_for_each_in<&[rustc_span[435a2de3bfa5601f]::def_id::LocalDefId], <rustc_middle[d24c4156abdc8924]::hir::map::Map>::par_body_owners<rustc_hir_analysis[d8ce6c1405752847]::check::typeck_item_bodies::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[93d0d218afed051e]::ops::function::FnOnce<()>>::call_once
  35:     0x7faaa5e06a99 - std[2e8a54ede7eda140]::panicking::try::<(), core[93d0d218afed051e]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[8b8c5aa8aa14aa0c]::sync::par_for_each_in<&[rustc_span[435a2de3bfa5601f]::def_id::LocalDefId], <rustc_middle[d24c4156abdc8924]::hir::map::Map>::par_body_owners<rustc_hir_analysis[d8ce6c1405752847]::check::typeck_item_bodies::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  36:     0x7faaa5b2388d - rustc_data_structures[8b8c5aa8aa14aa0c]::sync::par_for_each_in::<&[rustc_span[435a2de3bfa5601f]::def_id::LocalDefId], <rustc_middle[d24c4156abdc8924]::hir::map::Map>::par_body_owners<rustc_hir_analysis[d8ce6c1405752847]::check::typeck_item_bodies::{closure#0}>::{closure#0}>
  37:     0x7faaa5c03bb7 - rustc_hir_analysis[d8ce6c1405752847]::check::typeck_item_bodies
  38:     0x7faaa727e247 - rustc_query_system[f5f779109e483cf5]::query::plumbing::try_execute_query::<rustc_query_impl[3cf691212940497a]::plumbing::QueryCtxt, rustc_query_system[f5f779109e483cf5]::query::caches::DefaultCache<(), ()>>
  39:     0x7faaa731d4eb - rustc_query_system[f5f779109e483cf5]::query::plumbing::get_query::<rustc_query_impl[3cf691212940497a]::queries::typeck_item_bodies, rustc_query_impl[3cf691212940497a]::plumbing::QueryCtxt>
  40:     0x7faaa6e67eaa - <rustc_query_impl[3cf691212940497a]::Queries as rustc_middle[d24c4156abdc8924]::ty::query::QueryEngine>::typeck_item_bodies
  41:     0x7faaa5bab5db - <rustc_session[16cf70e3063e72cf]::session::Session>::time::<(), rustc_hir_analysis[d8ce6c1405752847]::check_crate::{closure#7}>
  42:     0x7faaa5e3b202 - rustc_hir_analysis[d8ce6c1405752847]::check_crate
  43:     0x7faaa575c081 - rustc_interface[6df570514bc600c5]::passes::analysis
  44:     0x7faaa7272cf8 - rustc_query_system[f5f779109e483cf5]::query::plumbing::try_execute_query::<rustc_query_impl[3cf691212940497a]::plumbing::QueryCtxt, rustc_query_system[f5f779109e483cf5]::query::caches::DefaultCache<(), core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>>>
  45:     0x7faaa735626b - rustc_query_system[f5f779109e483cf5]::query::plumbing::get_query::<rustc_query_impl[3cf691212940497a]::queries::analysis, rustc_query_impl[3cf691212940497a]::plumbing::QueryCtxt>
  46:     0x7faaa6e404fa - <rustc_query_impl[3cf691212940497a]::Queries as rustc_middle[d24c4156abdc8924]::ty::query::QueryEngine>::analysis
  47:     0x7faaa5662886 - <rustc_interface[6df570514bc600c5]::passes::QueryContext>::enter::<rustc_driver[f414b4652d8f0863]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>>
  48:     0x7faaa560591c - rustc_interface[6df570514bc600c5]::interface::create_compiler_and_run::<core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>, rustc_driver[f414b4652d8f0863]::run_compiler::{closure#1}>
  49:     0x7faaa5670f4f - <scoped_tls[799d6291841bce05]::ScopedKey<rustc_span[435a2de3bfa5601f]::SessionGlobals>>::set::<rustc_interface[6df570514bc600c5]::interface::run_compiler<core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>, rustc_driver[f414b4652d8f0863]::run_compiler::{closure#1}>::{closure#0}, core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>>
  50:     0x7faaa566649f - std[2e8a54ede7eda140]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[6df570514bc600c5]::util::run_in_thread_pool_with_globals<rustc_interface[6df570514bc600c5]::interface::run_compiler<core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>, rustc_driver[f414b4652d8f0863]::run_compiler::{closure#1}>::{closure#0}, core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>>::{closure#0}, core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>>
  51:     0x7faaa5604191 - std[2e8a54ede7eda140]::panic::catch_unwind::<core[93d0d218afed051e]::panic::unwind_safe::AssertUnwindSafe<<std[2e8a54ede7eda140]::thread::Builder>::spawn_unchecked_<rustc_interface[6df570514bc600c5]::util::run_in_thread_pool_with_globals<rustc_interface[6df570514bc600c5]::interface::run_compiler<core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>, rustc_driver[f414b4652d8f0863]::run_compiler::{closure#1}>::{closure#0}, core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>>::{closure#0}, core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>>
  52:     0x7faaa566a090 - <<std[2e8a54ede7eda140]::thread::Builder>::spawn_unchecked_<rustc_interface[6df570514bc600c5]::util::run_in_thread_pool_with_globals<rustc_interface[6df570514bc600c5]::interface::run_compiler<core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>, rustc_driver[f414b4652d8f0863]::run_compiler::{closure#1}>::{closure#0}, core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>>::{closure#0}, core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>>::{closure#1} as core[93d0d218afed051e]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  53:     0x7faaa4c54b35 - std::sys::unix::thread::Thread::new::thread_start::h781ed01b585563bf
  54:     0x7faaa49efb43 - <unknown>
  55:     0x7faaa4a81a00 - <unknown>
  56:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.66.0-nightly (7f7051991 2022-10-02) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `mod1::f`
#1 [typeck_item_bodies] type-checking all item bodies
#2 [analysis] running analysis passes on this crate
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/closures/print/closure-print-generic-trim-off-verbose-2.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/closures/print/closure-print-generic-trim-off-verbose-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/print/closure-print-generic-trim-off-verbose-2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Ztrim-diagnostic-paths=off" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/print/closure-print-generic-trim-off-verbose-2/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: compiler/rustc_middle/src/hir/map/mod.rs:872:18: expected expr, found local let c1 : () = c; (hir_id=HirId { owner: OwnerId { def_id: DefId(0:4 ~ closure_print_generic_trim_off_verbose_2[0eb7]::mod1::f) }, local_id: 60 })
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1502:9
stack backtrace:
   0:     0x7f9ca50f572e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::he9cf1d92f30cd7c4
   0:     0x7f9ca50f572e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::he9cf1d92f30cd7c4
   1:     0x7f9ca515e588 - core::fmt::write::hb71f33d6dcbbaae1
   2:     0x7f9ca50e6651 - std::io::Write::write_fmt::hba03ba06b18d9e62
   3:     0x7f9ca50f87de - std::panicking::default_hook::{{closure}}::hdef4e1c9c8ad451e
   4:     0x7f9ca50f8499 - std::panicking::default_hook::hc35395d68df0c59f
   5:     0x7f9ca5ab2874 - rustc_driver[f414b4652d8f0863]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f9ca50f8f9d - std::panicking::rust_panic_with_hook::hee5baad676042f5e
   7:     0x7f9ca8519cd3 - std[2e8a54ede7eda140]::panicking::begin_panic::<rustc_errors[215d8a3c2f9dfb2d]::ExplicitBug>::{closure#0}
   8:     0x7f9ca8516476 - std[2e8a54ede7eda140]::sys_common::backtrace::__rust_end_short_backtrace::<std[2e8a54ede7eda140]::panicking::begin_panic<rustc_errors[215d8a3c2f9dfb2d]::ExplicitBug>::{closure#0}, !>
   9:     0x7f9ca5a55376 - std[2e8a54ede7eda140]::panicking::begin_panic::<rustc_errors[215d8a3c2f9dfb2d]::ExplicitBug>
  10:     0x7f9ca850aaa6 - std[2e8a54ede7eda140]::panic::panic_any::<rustc_errors[215d8a3c2f9dfb2d]::ExplicitBug>
  11:     0x7f9ca85036b9 - <rustc_errors[215d8a3c2f9dfb2d]::HandlerInner>::bug::<&alloc[13583fc3d0f3e700]::string::String>
  12:     0x7f9ca8503190 - <rustc_errors[215d8a3c2f9dfb2d]::Handler>::bug::<&alloc[13583fc3d0f3e700]::string::String>
  13:     0x7f9ca868fbfe - rustc_middle[d24c4156abdc8924]::ty::context::tls::with_context_opt::<rustc_middle[d24c4156abdc8924]::ty::context::tls::with_opt<rustc_middle[d24c4156abdc8924]::util::bug::opt_span_bug_fmt<rustc_span[435a2de3bfa5601f]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  14:     0x7f9ca869c4f9 - rustc_middle[d24c4156abdc8924]::util::bug::opt_span_bug_fmt::<rustc_span[435a2de3bfa5601f]::span_encoding::Span>
  15:     0x7f9ca5a5f7b8 - rustc_middle[d24c4156abdc8924]::util::bug::bug_fmt
  16:     0x7f9ca85c7c03 - <rustc_middle[d24c4156abdc8924]::hir::map::Map>::expect_expr
  17:     0x7f9ca5f6ce62 - <rustc_hir_analysis[d8ce6c1405752847]::check::fn_ctxt::FnCtxt>::suggest_deref_ref_or_into
  18:     0x7f9ca5f398f0 - <rustc_hir_analysis[d8ce6c1405752847]::check::fn_ctxt::FnCtxt>::emit_coerce_suggestions
  19:     0x7f9ca5fab755 - <rustc_hir_analysis[d8ce6c1405752847]::check::fn_ctxt::FnCtxt>::demand_coerce_diag
  20:     0x7f9ca5f3f9d8 - <rustc_hir_analysis[d8ce6c1405752847]::check::fn_ctxt::FnCtxt>::demand_coerce
  21:     0x7f9ca5f64c9c - <rustc_hir_analysis[d8ce6c1405752847]::check::fn_ctxt::FnCtxt>::check_decl
  22:     0x7f9ca5f6505b - <rustc_hir_analysis[d8ce6c1405752847]::check::fn_ctxt::FnCtxt>::check_stmt
  23:     0x7f9ca5f65774 - <rustc_hir_analysis[d8ce6c1405752847]::check::fn_ctxt::FnCtxt>::check_block_with_expected
  24:     0x7f9ca5facbdb - <rustc_hir_analysis[d8ce6c1405752847]::check::fn_ctxt::FnCtxt>::check_expr_kind
  25:     0x7f9ca5f45dac - <rustc_hir_analysis[d8ce6c1405752847]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  26:     0x7f9ca5fabc82 - <rustc_hir_analysis[d8ce6c1405752847]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  27:     0x7f9ca5f475ac - <rustc_hir_analysis[d8ce6c1405752847]::check::fn_ctxt::FnCtxt>::check_return_expr
  28:     0x7f9ca61fecbb - rustc_hir_analysis[d8ce6c1405752847]::check::check::check_fn
  29:     0x7f9ca61f2d63 - <rustc_hir_analysis[d8ce6c1405752847]::check::inherited::InheritedBuilder>::enter::<rustc_hir_analysis[d8ce6c1405752847]::check::typeck_with_fallback<rustc_hir_analysis[d8ce6c1405752847]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[d24c4156abdc8924]::ty::context::TypeckResults>
  30:     0x7f9ca60b023b - rustc_hir_analysis[d8ce6c1405752847]::check::typeck
  31:     0x7f9ca76da71c - rustc_query_system[f5f779109e483cf5]::query::plumbing::try_execute_query::<rustc_query_impl[3cf691212940497a]::plumbing::QueryCtxt, rustc_query_system[f5f779109e483cf5]::query::caches::DefaultCache<rustc_span[435a2de3bfa5601f]::def_id::LocalDefId, &rustc_middle[d24c4156abdc8924]::ty::context::TypeckResults>>
  32:     0x7f9ca7806ebd - rustc_query_system[f5f779109e483cf5]::query::plumbing::get_query::<rustc_query_impl[3cf691212940497a]::queries::typeck, rustc_query_impl[3cf691212940497a]::plumbing::QueryCtxt>
  33:     0x7f9ca7319600 - <rustc_query_impl[3cf691212940497a]::Queries as rustc_middle[d24c4156abdc8924]::ty::query::QueryEngine>::typeck
  34:     0x7f9ca60a75e6 - <core[93d0d218afed051e]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[8b8c5aa8aa14aa0c]::sync::par_for_each_in<&[rustc_span[435a2de3bfa5601f]::def_id::LocalDefId], <rustc_middle[d24c4156abdc8924]::hir::map::Map>::par_body_owners<rustc_hir_analysis[d8ce6c1405752847]::check::typeck_item_bodies::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[93d0d218afed051e]::ops::function::FnOnce<()>>::call_once
  35:     0x7f9ca62b7a99 - std[2e8a54ede7eda140]::panicking::try::<(), core[93d0d218afed051e]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[8b8c5aa8aa14aa0c]::sync::par_for_each_in<&[rustc_span[435a2de3bfa5601f]::def_id::LocalDefId], <rustc_middle[d24c4156abdc8924]::hir::map::Map>::par_body_owners<rustc_hir_analysis[d8ce6c1405752847]::check::typeck_item_bodies::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  36:     0x7f9ca5fd488d - rustc_data_structures[8b8c5aa8aa14aa0c]::sync::par_for_each_in::<&[rustc_span[435a2de3bfa5601f]::def_id::LocalDefId], <rustc_middle[d24c4156abdc8924]::hir::map::Map>::par_body_owners<rustc_hir_analysis[d8ce6c1405752847]::check::typeck_item_bodies::{closure#0}>::{closure#0}>
  37:     0x7f9ca60b4bb7 - rustc_hir_analysis[d8ce6c1405752847]::check::typeck_item_bodies
  38:     0x7f9ca772f247 - rustc_query_system[f5f779109e483cf5]::query::plumbing::try_execute_query::<rustc_query_impl[3cf691212940497a]::plumbing::QueryCtxt, rustc_query_system[f5f779109e483cf5]::query::caches::DefaultCache<(), ()>>
  39:     0x7f9ca77ce4eb - rustc_query_system[f5f779109e483cf5]::query::plumbing::get_query::<rustc_query_impl[3cf691212940497a]::queries::typeck_item_bodies, rustc_query_impl[3cf691212940497a]::plumbing::QueryCtxt>
  40:     0x7f9ca7318eaa - <rustc_query_impl[3cf691212940497a]::Queries as rustc_middle[d24c4156abdc8924]::ty::query::QueryEngine>::typeck_item_bodies
  41:     0x7f9ca605c5db - <rustc_session[16cf70e3063e72cf]::session::Session>::time::<(), rustc_hir_analysis[d8ce6c1405752847]::check_crate::{closure#7}>
  42:     0x7f9ca62ec202 - rustc_hir_analysis[d8ce6c1405752847]::check_crate
  43:     0x7f9ca5c0d081 - rustc_interface[6df570514bc600c5]::passes::analysis
  44:     0x7f9ca7723cf8 - rustc_query_system[f5f779109e483cf5]::query::plumbing::try_execute_query::<rustc_query_impl[3cf691212940497a]::plumbing::QueryCtxt, rustc_query_system[f5f779109e483cf5]::query::caches::DefaultCache<(), core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>>>
  45:     0x7f9ca780726b - rustc_query_system[f5f779109e483cf5]::query::plumbing::get_query::<rustc_query_impl[3cf691212940497a]::queries::analysis, rustc_query_impl[3cf691212940497a]::plumbing::QueryCtxt>
  46:     0x7f9ca72f14fa - <rustc_query_impl[3cf691212940497a]::Queries as rustc_middle[d24c4156abdc8924]::ty::query::QueryEngine>::analysis
  47:     0x7f9ca5b13886 - <rustc_interface[6df570514bc600c5]::passes::QueryContext>::enter::<rustc_driver[f414b4652d8f0863]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>>
  48:     0x7f9ca5ab691c - rustc_interface[6df570514bc600c5]::interface::create_compiler_and_run::<core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>, rustc_driver[f414b4652d8f0863]::run_compiler::{closure#1}>
  49:     0x7f9ca5b21f4f - <scoped_tls[799d6291841bce05]::ScopedKey<rustc_span[435a2de3bfa5601f]::SessionGlobals>>::set::<rustc_interface[6df570514bc600c5]::interface::run_compiler<core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>, rustc_driver[f414b4652d8f0863]::run_compiler::{closure#1}>::{closure#0}, core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>>
  50:     0x7f9ca5b1749f - std[2e8a54ede7eda140]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[6df570514bc600c5]::util::run_in_thread_pool_with_globals<rustc_interface[6df570514bc600c5]::interface::run_compiler<core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>, rustc_driver[f414b4652d8f0863]::run_compiler::{closure#1}>::{closure#0}, core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>>::{closure#0}, core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>>
  51:     0x7f9ca5ab5191 - std[2e8a54ede7eda140]::panic::catch_unwind::<core[93d0d218afed051e]::panic::unwind_safe::AssertUnwindSafe<<std[2e8a54ede7eda140]::thread::Builder>::spawn_unchecked_<rustc_interface[6df570514bc600c5]::util::run_in_thread_pool_with_globals<rustc_interface[6df570514bc600c5]::interface::run_compiler<core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>, rustc_driver[f414b4652d8f0863]::run_compiler::{closure#1}>::{closure#0}, core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>>::{closure#0}, core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>>
  52:     0x7f9ca5b1b090 - <<std[2e8a54ede7eda140]::thread::Builder>::spawn_unchecked_<rustc_interface[6df570514bc600c5]::util::run_in_thread_pool_with_globals<rustc_interface[6df570514bc600c5]::interface::run_compiler<core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>, rustc_driver[f414b4652d8f0863]::run_compiler::{closure#1}>::{closure#0}, core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>>::{closure#0}, core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>>::{closure#1} as core[93d0d218afed051e]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  53:     0x7f9ca5105b35 - std::sys::unix::thread::Thread::new::thread_start::h781ed01b585563bf
  54:     0x7f9ca4ea0b43 - <unknown>
  55:     0x7f9ca4f32a00 - <unknown>
  56:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.66.0-nightly (7f7051991 2022-10-02) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 -Z trim-diagnostic-paths=off -Z verbose
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `mod1::f` [typeck]
#1 [typeck_item_bodies] type-checking all item bodies [typeck_item_bodies]
#2 [analysis] running analysis passes on this crate [analysis]
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/closures/print/closure-print-generic-verbose-2.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/closures/print/closure-print-generic-verbose-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/print/closure-print-generic-verbose-2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/print/closure-print-generic-verbose-2/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: compiler/rustc_middle/src/hir/map/mod.rs:872:18: expected expr, found local let c1 : () = c; (hir_id=HirId { owner: OwnerId { def_id: DefId(0:4 ~ closure_print_generic_verbose_2[b771]::mod1::f) }, local_id: 60 })
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1502:9
stack backtrace:
   0:     0x7efcb96d872e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::he9cf1d92f30cd7c4
   0:     0x7efcb96d872e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::he9cf1d92f30cd7c4
   1:     0x7efcb9741588 - core::fmt::write::hb71f33d6dcbbaae1
   2:     0x7efcb96c9651 - std::io::Write::write_fmt::hba03ba06b18d9e62
   3:     0x7efcb96db7de - std::panicking::default_hook::{{closure}}::hdef4e1c9c8ad451e
   4:     0x7efcb96db499 - std::panicking::default_hook::hc35395d68df0c59f
   5:     0x7efcba095874 - rustc_driver[f414b4652d8f0863]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7efcb96dbf9d - std::panicking::rust_panic_with_hook::hee5baad676042f5e
   7:     0x7efcbcafccd3 - std[2e8a54ede7eda140]::panicking::begin_panic::<rustc_errors[215d8a3c2f9dfb2d]::ExplicitBug>::{closure#0}
   8:     0x7efcbcaf9476 - std[2e8a54ede7eda140]::sys_common::backtrace::__rust_end_short_backtrace::<std[2e8a54ede7eda140]::panicking::begin_panic<rustc_errors[215d8a3c2f9dfb2d]::ExplicitBug>::{closure#0}, !>
   9:     0x7efcba038376 - std[2e8a54ede7eda140]::panicking::begin_panic::<rustc_errors[215d8a3c2f9dfb2d]::ExplicitBug>
  10:     0x7efcbcaedaa6 - std[2e8a54ede7eda140]::panic::panic_any::<rustc_errors[215d8a3c2f9dfb2d]::ExplicitBug>
  11:     0x7efcbcae66b9 - <rustc_errors[215d8a3c2f9dfb2d]::HandlerInner>::bug::<&alloc[13583fc3d0f3e700]::string::String>
  12:     0x7efcbcae6190 - <rustc_errors[215d8a3c2f9dfb2d]::Handler>::bug::<&alloc[13583fc3d0f3e700]::string::String>
  13:     0x7efcbcc72bfe - rustc_middle[d24c4156abdc8924]::ty::context::tls::with_context_opt::<rustc_middle[d24c4156abdc8924]::ty::context::tls::with_opt<rustc_middle[d24c4156abdc8924]::util::bug::opt_span_bug_fmt<rustc_span[435a2de3bfa5601f]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  14:     0x7efcbcc7f4f9 - rustc_middle[d24c4156abdc8924]::util::bug::opt_span_bug_fmt::<rustc_span[435a2de3bfa5601f]::span_encoding::Span>
  15:     0x7efcba0427b8 - rustc_middle[d24c4156abdc8924]::util::bug::bug_fmt
  16:     0x7efcbcbaac03 - <rustc_middle[d24c4156abdc8924]::hir::map::Map>::expect_expr
  17:     0x7efcba54fe62 - <rustc_hir_analysis[d8ce6c1405752847]::check::fn_ctxt::FnCtxt>::suggest_deref_ref_or_into
  18:     0x7efcba51c8f0 - <rustc_hir_analysis[d8ce6c1405752847]::check::fn_ctxt::FnCtxt>::emit_coerce_suggestions
  19:     0x7efcba58e755 - <rustc_hir_analysis[d8ce6c1405752847]::check::fn_ctxt::FnCtxt>::demand_coerce_diag
  20:     0x7efcba5229d8 - <rustc_hir_analysis[d8ce6c1405752847]::check::fn_ctxt::FnCtxt>::demand_coerce
  21:     0x7efcba547c9c - <rustc_hir_analysis[d8ce6c1405752847]::check::fn_ctxt::FnCtxt>::check_decl
  22:     0x7efcba54805b - <rustc_hir_analysis[d8ce6c1405752847]::check::fn_ctxt::FnCtxt>::check_stmt
  23:     0x7efcba548774 - <rustc_hir_analysis[d8ce6c1405752847]::check::fn_ctxt::FnCtxt>::check_block_with_expected
  24:     0x7efcba58fbdb - <rustc_hir_analysis[d8ce6c1405752847]::check::fn_ctxt::FnCtxt>::check_expr_kind
  25:     0x7efcba528dac - <rustc_hir_analysis[d8ce6c1405752847]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  26:     0x7efcba58ec82 - <rustc_hir_analysis[d8ce6c1405752847]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  27:     0x7efcba52a5ac - <rustc_hir_analysis[d8ce6c1405752847]::check::fn_ctxt::FnCtxt>::check_return_expr
  28:     0x7efcba7e1cbb - rustc_hir_analysis[d8ce6c1405752847]::check::check::check_fn
  29:     0x7efcba7d5d63 - <rustc_hir_analysis[d8ce6c1405752847]::check::inherited::InheritedBuilder>::enter::<rustc_hir_analysis[d8ce6c1405752847]::check::typeck_with_fallback<rustc_hir_analysis[d8ce6c1405752847]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[d24c4156abdc8924]::ty::context::TypeckResults>
  30:     0x7efcba69323b - rustc_hir_analysis[d8ce6c1405752847]::check::typeck
  31:     0x7efcbbcbd71c - rustc_query_system[f5f779109e483cf5]::query::plumbing::try_execute_query::<rustc_query_impl[3cf691212940497a]::plumbing::QueryCtxt, rustc_query_system[f5f779109e483cf5]::query::caches::DefaultCache<rustc_span[435a2de3bfa5601f]::def_id::LocalDefId, &rustc_middle[d24c4156abdc8924]::ty::context::TypeckResults>>
  32:     0x7efcbbde9ebd - rustc_query_system[f5f779109e483cf5]::query::plumbing::get_query::<rustc_query_impl[3cf691212940497a]::queries::typeck, rustc_query_impl[3cf691212940497a]::plumbing::QueryCtxt>
  33:     0x7efcbb8fc600 - <rustc_query_impl[3cf691212940497a]::Queries as rustc_middle[d24c4156abdc8924]::ty::query::QueryEngine>::typeck
  34:     0x7efcba68a5e6 - <core[93d0d218afed051e]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[8b8c5aa8aa14aa0c]::sync::par_for_each_in<&[rustc_span[435a2de3bfa5601f]::def_id::LocalDefId], <rustc_middle[d24c4156abdc8924]::hir::map::Map>::par_body_owners<rustc_hir_analysis[d8ce6c1405752847]::check::typeck_item_bodies::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[93d0d218afed051e]::ops::function::FnOnce<()>>::call_once
  35:     0x7efcba89aa99 - std[2e8a54ede7eda140]::panicking::try::<(), core[93d0d218afed051e]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[8b8c5aa8aa14aa0c]::sync::par_for_each_in<&[rustc_span[435a2de3bfa5601f]::def_id::LocalDefId], <rustc_middle[d24c4156abdc8924]::hir::map::Map>::par_body_owners<rustc_hir_analysis[d8ce6c1405752847]::check::typeck_item_bodies::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  36:     0x7efcba5b788d - rustc_data_structures[8b8c5aa8aa14aa0c]::sync::par_for_each_in::<&[rustc_span[435a2de3bfa5601f]::def_id::LocalDefId], <rustc_middle[d24c4156abdc8924]::hir::map::Map>::par_body_owners<rustc_hir_analysis[d8ce6c1405752847]::check::typeck_item_bodies::{closure#0}>::{closure#0}>
  37:     0x7efcba697bb7 - rustc_hir_analysis[d8ce6c1405752847]::check::typeck_item_bodies
  38:     0x7efcbbd12247 - rustc_query_system[f5f779109e483cf5]::query::plumbing::try_execute_query::<rustc_query_impl[3cf691212940497a]::plumbing::QueryCtxt, rustc_query_system[f5f779109e483cf5]::query::caches::DefaultCache<(), ()>>
  39:     0x7efcbbdb14eb - rustc_query_system[f5f779109e483cf5]::query::plumbing::get_query::<rustc_query_impl[3cf691212940497a]::queries::typeck_item_bodies, rustc_query_impl[3cf691212940497a]::plumbing::QueryCtxt>
  40:     0x7efcbb8fbeaa - <rustc_query_impl[3cf691212940497a]::Queries as rustc_middle[d24c4156abdc8924]::ty::query::QueryEngine>::typeck_item_bodies
  41:     0x7efcba63f5db - <rustc_session[16cf70e3063e72cf]::session::Session>::time::<(), rustc_hir_analysis[d8ce6c1405752847]::check_crate::{closure#7}>
  42:     0x7efcba8cf202 - rustc_hir_analysis[d8ce6c1405752847]::check_crate
  43:     0x7efcba1f0081 - rustc_interface[6df570514bc600c5]::passes::analysis
  44:     0x7efcbbd06cf8 - rustc_query_system[f5f779109e483cf5]::query::plumbing::try_execute_query::<rustc_query_impl[3cf691212940497a]::plumbing::QueryCtxt, rustc_query_system[f5f779109e483cf5]::query::caches::DefaultCache<(), core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>>>
  45:     0x7efcbbdea26b - rustc_query_system[f5f779109e483cf5]::query::plumbing::get_query::<rustc_query_impl[3cf691212940497a]::queries::analysis, rustc_query_impl[3cf691212940497a]::plumbing::QueryCtxt>
  46:     0x7efcbb8d44fa - <rustc_query_impl[3cf691212940497a]::Queries as rustc_middle[d24c4156abdc8924]::ty::query::QueryEngine>::analysis
  47:     0x7efcba0f6886 - <rustc_interface[6df570514bc600c5]::passes::QueryContext>::enter::<rustc_driver[f414b4652d8f0863]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>>
  48:     0x7efcba09991c - rustc_interface[6df570514bc600c5]::interface::create_compiler_and_run::<core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>, rustc_driver[f414b4652d8f0863]::run_compiler::{closure#1}>
  49:     0x7efcba104f4f - <scoped_tls[799d6291841bce05]::ScopedKey<rustc_span[435a2de3bfa5601f]::SessionGlobals>>::set::<rustc_interface[6df570514bc600c5]::interface::run_compiler<core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>, rustc_driver[f414b4652d8f0863]::run_compiler::{closure#1}>::{closure#0}, core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>>
  50:     0x7efcba0fa49f - std[2e8a54ede7eda140]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[6df570514bc600c5]::util::run_in_thread_pool_with_globals<rustc_interface[6df570514bc600c5]::interface::run_compiler<core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>, rustc_driver[f414b4652d8f0863]::run_compiler::{closure#1}>::{closure#0}, core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>>::{closure#0}, core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>>
  51:     0x7efcba098191 - std[2e8a54ede7eda140]::panic::catch_unwind::<core[93d0d218afed051e]::panic::unwind_safe::AssertUnwindSafe<<std[2e8a54ede7eda140]::thread::Builder>::spawn_unchecked_<rustc_interface[6df570514bc600c5]::util::run_in_thread_pool_with_globals<rustc_interface[6df570514bc600c5]::interface::run_compiler<core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>, rustc_driver[f414b4652d8f0863]::run_compiler::{closure#1}>::{closure#0}, core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>>::{closure#0}, core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>>
  52:     0x7efcba0fe090 - <<std[2e8a54ede7eda140]::thread::Builder>::spawn_unchecked_<rustc_interface[6df570514bc600c5]::util::run_in_thread_pool_with_globals<rustc_interface[6df570514bc600c5]::interface::run_compiler<core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>, rustc_driver[f414b4652d8f0863]::run_compiler::{closure#1}>::{closure#0}, core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>>::{closure#0}, core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>>::{closure#1} as core[93d0d218afed051e]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  53:     0x7efcb96e8b35 - std::sys::unix::thread::Thread::new::thread_start::h781ed01b585563bf
  54:     0x7efcb9483b43 - <unknown>
  55:     0x7efcb9515a00 - <unknown>
  56:                0x0 - <unknown>
---
LL | |     }
   | |_____^
help: you might have meant to use the following enum variant
   |
LL |     let _: E = E::Unit;
help: alternatively, the following enum variants are also available
   |
   |
LL |     let _: E = (E::Fn(/* fields */));
   |                ~~~~~~~~~~~~~~~~~~~~~
LL |     let _: E = (E::Struct { /* fields */ });
help: a function with a similar name exists
   |
   |
LL |     let _: E = m::f;
help: consider importing one of these items instead
   |
LL | use std::f32::consts::E;
   |
   |
LL | use std::f64::consts::E;
   |
help: if you import `E`, refer to it directly
   |
LL -     let _: E = m::E;
LL +     let _: E = E;


error[E0423]: expected value, found struct variant `m::E::Struct`
   |
LL | /         Struct {
LL | |             s: u8,
LL | |         },
LL | |         },
   | |_________- `m::E::Struct` defined here
...
LL |       let _: E = m::E::Struct;
   |                  ^^^^^^^^^^^^ help: use struct literal syntax instead: `m::E::Struct { s: val }`
error[E0423]: expected value, found enum `E`
  --> /checkout/src/test/ui/resolve/privacy-enum-ctor.rs:49:16
   |
   |
LL |     let _: E = E;
   |
note: the enum is defined here
  --> /checkout/src/test/ui/resolve/privacy-enum-ctor.rs:2:5
   |
---
LL | |     }
   | |_____^
help: you might have meant to use the following enum variant
   |
LL |     let _: E = E::Unit;
help: alternatively, the following enum variants are also available
   |
   |
LL |     let _: E = (E::Fn(/* fields */));
   |                ~~~~~~~~~~~~~~~~~~~~~
LL |     let _: E = (E::Struct { /* fields */ });
help: consider importing one of these items instead
   |
LL | use std::f32::consts::E;
   |
   |
LL | use std::f64::consts::E;
   |

error[E0423]: expected value, found struct variant `E::Struct`
  --> /checkout/src/test/ui/resolve/privacy-enum-ctor.rs:53:16
   |
LL | /         Struct {
LL | |             s: u8,
LL | |         },
   | |_________- `E::Struct` defined here
...
LL |       let _: E = E::Struct;
   |                  ^^^^^^^^^ help: use struct literal syntax instead: `E::Struct { s: val }`
error[E0412]: cannot find type `Z` in this scope
  --> /checkout/src/test/ui/resolve/privacy-enum-ctor.rs:57:12
   |
LL |     pub enum E {
LL |     pub enum E {
   |     ---------- similarly named enum `E` defined here
...
LL |     let _: Z = m::n::Z;
   |            ^ help: an enum with a similar name exists: `E`
   |
note: enum `m::Z` exists but is inaccessible
   |
   |
LL |         pub(in m) enum Z {


error[E0423]: expected value, found enum `m::n::Z`
   |
   |
LL |     let _: Z = m::n::Z;
   |
note: the enum is defined here
  --> /checkout/src/test/ui/resolve/privacy-enum-ctor.rs:11:9
   |
   |
LL | /         pub(in m) enum Z {
LL | |             Fn(u8),
LL | |             Struct {
LL | |                 s: u8,
LL | |             },
LL | |             Unit,
LL | |         }
help: you might have meant to use the following enum variant
   |
   |
LL |     let _: Z = m::Z::Unit;
help: alternatively, the following enum variants are also available
   |
   |
LL |     let _: Z = (m::Z::Fn(/* fields */));
   |                ~~~~~~~~~~~~~~~~~~~~~~~~
LL |     let _: Z = (m::Z::Struct { /* fields */ });

error[E0412]: cannot find type `Z` in this scope
  --> /checkout/src/test/ui/resolve/privacy-enum-ctor.rs:61:12
   |
   |
LL |     pub enum E {
   |     ---------- similarly named enum `E` defined here
...
LL |     let _: Z = m::n::Z::Fn;
   |            ^ help: an enum with a similar name exists: `E`
   |
note: enum `m::Z` exists but is inaccessible
   |
   |
LL |         pub(in m) enum Z {

error[E0412]: cannot find type `Z` in this scope
  --> /checkout/src/test/ui/resolve/privacy-enum-ctor.rs:64:12
   |
   |
LL |     pub enum E {
   |     ---------- similarly named enum `E` defined here
...
LL |     let _: Z = m::n::Z::Struct;
   |            ^ help: an enum with a similar name exists: `E`
   |
note: enum `m::Z` exists but is inaccessible
   |
   |
LL |         pub(in m) enum Z {


error[E0423]: expected value, found struct variant `m::n::Z::Struct`
   |
LL | /             Struct {
LL | |                 s: u8,
LL | |             },
LL | |             },
   | |_____________- `m::n::Z::Struct` defined here
...
LL |       let _: Z = m::n::Z::Struct;
   |                  ^^^^^^^^^^^^^^^ help: use struct literal syntax instead: `m::n::Z::Struct { s: val }`
error[E0412]: cannot find type `Z` in this scope
  --> /checkout/src/test/ui/resolve/privacy-enum-ctor.rs:68:12
   |
LL |     pub enum E {
LL |     pub enum E {
   |     ---------- similarly named enum `E` defined here
...
LL |     let _: Z = m::n::Z::Unit {};
   |            ^ help: an enum with a similar name exists: `E`
   |
note: enum `m::Z` exists but is inaccessible
   |
   |
LL |         pub(in m) enum Z {


error[E0603]: enum `Z` is private
   |
   |
LL |     let _: Z = m::n::Z;
   |                      ^ private enum
   |
note: the enum `Z` is defined here
   |
   |
LL |         pub(in m) enum Z {


error[E0603]: enum `Z` is private
   |
   |
LL |     let _: Z = m::n::Z::Fn;
   |                      ^ private enum
   |
note: the enum `Z` is defined here
   |
   |
LL |         pub(in m) enum Z {


error[E0603]: enum `Z` is private
   |
   |
LL |     let _: Z = m::n::Z::Struct;
   |                      ^ private enum
   |
note: the enum `Z` is defined here
   |
   |
LL |         pub(in m) enum Z {


error[E0603]: enum `Z` is private
   |
   |
LL |     let _: Z = m::n::Z::Unit {};
   |                      ^ private enum
   |
note: the enum `Z` is defined here
   |
   |
LL |         pub(in m) enum Z {


error: internal compiler error: compiler/rustc_middle/src/hir/map/mod.rs:872:18: expected expr, found local let _: Z = Z::Fn; (hir_id=HirId { owner: OwnerId { def_id: DefId(0:24 ~ privacy_enum_ctor[66ef]::m::f) }, local_id: 14 })
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1502:9
stack backtrace:
   0:     0x7fba1263572e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::he9cf1d92f30cd7c4
   0:     0x7fba1263572e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::he9cf1d92f30cd7c4
   1:     0x7fba1269e588 - core::fmt::write::hb71f33d6dcbbaae1
   2:     0x7fba12626651 - std::io::Write::write_fmt::hba03ba06b18d9e62
   3:     0x7fba126387de - std::panicking::default_hook::{{closure}}::hdef4e1c9c8ad451e
   4:     0x7fba12638499 - std::panicking::default_hook::hc35395d68df0c59f
   5:     0x7fba12ff2874 - rustc_driver[f414b4652d8f0863]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fba12638f9d - std::panicking::rust_panic_with_hook::hee5baad676042f5e
   7:     0x7fba15a59cd3 - std[2e8a54ede7eda140]::panicking::begin_panic::<rustc_errors[215d8a3c2f9dfb2d]::ExplicitBug>::{closure#0}
   8:     0x7fba15a56476 - std[2e8a54ede7eda140]::sys_common::backtrace::__rust_end_short_backtrace::<std[2e8a54ede7eda140]::panicking::begin_panic<rustc_errors[215d8a3c2f9dfb2d]::ExplicitBug>::{closure#0}, !>
   9:     0x7fba12f95376 - std[2e8a54ede7eda140]::panicking::begin_panic::<rustc_errors[215d8a3c2f9dfb2d]::ExplicitBug>
  10:     0x7fba15a4aaa6 - std[2e8a54ede7eda140]::panic::panic_any::<rustc_errors[215d8a3c2f9dfb2d]::ExplicitBug>
  11:     0x7fba15a436b9 - <rustc_errors[215d8a3c2f9dfb2d]::HandlerInner>::bug::<&alloc[13583fc3d0f3e700]::string::String>
  12:     0x7fba15a43190 - <rustc_errors[215d8a3c2f9dfb2d]::Handler>::bug::<&alloc[13583fc3d0f3e700]::string::String>
  13:     0x7fba15bcfbfe - rustc_middle[d24c4156abdc8924]::ty::context::tls::with_context_opt::<rustc_middle[d24c4156abdc8924]::ty::context::tls::with_opt<rustc_middle[d24c4156abdc8924]::util::bug::opt_span_bug_fmt<rustc_span[435a2de3bfa5601f]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  14:     0x7fba15bdc4f9 - rustc_middle[d24c4156abdc8924]::util::bug::opt_span_bug_fmt::<rustc_span[435a2de3bfa5601f]::span_encoding::Span>
  15:     0x7fba12f9f7b8 - rustc_middle[d24c4156abdc8924]::util::bug::bug_fmt
  16:     0x7fba15b07c03 - <rustc_middle[d24c4156abdc8924]::hir::map::Map>::expect_expr
  17:     0x7fba134ace62 - <rustc_hir_analysis[d8ce6c1405752847]::check::fn_ctxt::FnCtxt>::suggest_deref_ref_or_into
  18:     0x7fba134798f0 - <rustc_hir_analysis[d8ce6c1405752847]::check::fn_ctxt::FnCtxt>::emit_coerce_suggestions
  19:     0x7fba134eb755 - <rustc_hir_analysis[d8ce6c1405752847]::check::fn_ctxt::FnCtxt>::demand_coerce_diag
  20:     0x7fba1347f9d8 - <rustc_hir_analysis[d8ce6c1405752847]::check::fn_ctxt::FnCtxt>::demand_coerce
  21:     0x7fba134a4c9c - <rustc_hir_analysis[d8ce6c1405752847]::check::fn_ctxt::FnCtxt>::check_decl
  22:     0x7fba134a505b - <rustc_hir_analysis[d8ce6c1405752847]::check::fn_ctxt::FnCtxt>::check_stmt
  23:     0x7fba134a5774 - <rustc_hir_analysis[d8ce6c1405752847]::check::fn_ctxt::FnCtxt>::check_block_with_expected
  24:     0x7fba134ecbdb - <rustc_hir_analysis[d8ce6c1405752847]::check::fn_ctxt::FnCtxt>::check_expr_kind
  25:     0x7fba13485dac - <rustc_hir_analysis[d8ce6c1405752847]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  26:     0x7fba134ebc82 - <rustc_hir_analysis[d8ce6c1405752847]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  27:     0x7fba134875ac - <rustc_hir_analysis[d8ce6c1405752847]::check::fn_ctxt::FnCtxt>::check_return_expr
  28:     0x7fba1373ecbb - rustc_hir_analysis[d8ce6c1405752847]::check::check::check_fn
  29:     0x7fba13732d63 - <rustc_hir_analysis[d8ce6c1405752847]::check::inherited::InheritedBuilder>::enter::<rustc_hir_analysis[d8ce6c1405752847]::check::typeck_with_fallback<rustc_hir_analysis[d8ce6c1405752847]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[d24c4156abdc8924]::ty::context::TypeckResults>
  30:     0x7fba135f023b - rustc_hir_analysis[d8ce6c1405752847]::check::typeck
  31:     0x7fba14c1a71c - rustc_query_system[f5f779109e483cf5]::query::plumbing::try_execute_query::<rustc_query_impl[3cf691212940497a]::plumbing::QueryCtxt, rustc_query_system[f5f779109e483cf5]::query::caches::DefaultCache<rustc_span[435a2de3bfa5601f]::def_id::LocalDefId, &rustc_middle[d24c4156abdc8924]::ty::context::TypeckResults>>
  32:     0x7fba14d46ebd - rustc_query_system[f5f779109e483cf5]::query::plumbing::get_query::<rustc_query_impl[3cf691212940497a]::queries::typeck, rustc_query_impl[3cf691212940497a]::plumbing::QueryCtxt>
  33:     0x7fba14859600 - <rustc_query_impl[3cf691212940497a]::Queries as rustc_middle[d24c4156abdc8924]::ty::query::QueryEngine>::typeck
  34:     0x7fba135e75e6 - <core[93d0d218afed051e]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[8b8c5aa8aa14aa0c]::sync::par_for_each_in<&[rustc_span[435a2de3bfa5601f]::def_id::LocalDefId], <rustc_middle[d24c4156abdc8924]::hir::map::Map>::par_body_owners<rustc_hir_analysis[d8ce6c1405752847]::check::typeck_item_bodies::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[93d0d218afed051e]::ops::function::FnOnce<()>>::call_once
  35:     0x7fba137f7a99 - std[2e8a54ede7eda140]::panicking::try::<(), core[93d0d218afed051e]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[8b8c5aa8aa14aa0c]::sync::par_for_each_in<&[rustc_span[435a2de3bfa5601f]::def_id::LocalDefId], <rustc_middle[d24c4156abdc8924]::hir::map::Map>::par_body_owners<rustc_hir_analysis[d8ce6c1405752847]::check::typeck_item_bodies::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  36:     0x7fba1351488d - rustc_data_structures[8b8c5aa8aa14aa0c]::sync::par_for_each_in::<&[rustc_span[435a2de3bfa5601f]::def_id::LocalDefId], <rustc_middle[d24c4156abdc8924]::hir::map::Map>::par_body_owners<rustc_hir_analysis[d8ce6c1405752847]::check::typeck_item_bodies::{closure#0}>::{closure#0}>
  37:     0x7fba135f4bb7 - rustc_hir_analysis[d8ce6c1405752847]::check::typeck_item_bodies
  38:     0x7fba14c6f247 - rustc_query_system[f5f779109e483cf5]::query::plumbing::try_execute_query::<rustc_query_impl[3cf691212940497a]::plumbing::QueryCtxt, rustc_query_system[f5f779109e483cf5]::query::caches::DefaultCache<(), ()>>
  39:     0x7fba14d0e4eb - rustc_query_system[f5f779109e483cf5]::query::plumbing::get_query::<rustc_query_impl[3cf691212940497a]::queries::typeck_item_bodies, rustc_query_impl[3cf691212940497a]::plumbing::QueryCtxt>
  40:     0x7fba14858eaa - <rustc_query_impl[3cf691212940497a]::Queries as rustc_middle[d24c4156abdc8924]::ty::query::QueryEngine>::typeck_item_bodies
  41:     0x7fba1359c5db - <rustc_session[16cf70e3063e72cf]::session::Session>::time::<(), rustc_hir_analysis[d8ce6c1405752847]::check_crate::{closure#7}>
  42:     0x7fba1382c202 - rustc_hir_analysis[d8ce6c1405752847]::check_crate
  43:     0x7fba1314d081 - rustc_interface[6df570514bc600c5]::passes::analysis
  44:     0x7fba14c63cf8 - rustc_query_system[f5f779109e483cf5]::query::plumbing::try_execute_query::<rustc_query_impl[3cf691212940497a]::plumbing::QueryCtxt, rustc_query_system[f5f779109e483cf5]::query::caches::DefaultCache<(), core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>>>
  45:     0x7fba14d4726b - rustc_query_system[f5f779109e483cf5]::query::plumbing::get_query::<rustc_query_impl[3cf691212940497a]::queries::analysis, rustc_query_impl[3cf691212940497a]::plumbing::QueryCtxt>
  46:     0x7fba148314fa - <rustc_query_impl[3cf691212940497a]::Queries as rustc_middle[d24c4156abdc8924]::ty::query::QueryEngine>::analysis
  47:     0x7fba13053886 - <rustc_interface[6df570514bc600c5]::passes::QueryContext>::enter::<rustc_driver[f414b4652d8f0863]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>>
  48:     0x7fba12ff691c - rustc_interface[6df570514bc600c5]::interface::create_compiler_and_run::<core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>, rustc_driver[f414b4652d8f0863]::run_compiler::{closure#1}>
  49:     0x7fba13061f4f - <scoped_tls[799d6291841bce05]::ScopedKey<rustc_span[435a2de3bfa5601f]::SessionGlobals>>::set::<rustc_interface[6df570514bc600c5]::interface::run_compiler<core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>, rustc_driver[f414b4652d8f0863]::run_compiler::{closure#1}>::{closure#0}, core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>>
  50:     0x7fba1305749f - std[2e8a54ede7eda140]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[6df570514bc600c5]::util::run_in_thread_pool_with_globals<rustc_interface[6df570514bc600c5]::interface::run_compiler<core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>, rustc_driver[f414b4652d8f0863]::run_compiler::{closure#1}>::{closure#0}, core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>>::{closure#0}, core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>>
  51:     0x7fba12ff5191 - std[2e8a54ede7eda140]::panic::catch_unwind::<core[93d0d218afed051e]::panic::unwind_safe::AssertUnwindSafe<<std[2e8a54ede7eda140]::thread::Builder>::spawn_unchecked_<rustc_interface[6df570514bc600c5]::util::run_in_thread_pool_with_globals<rustc_interface[6df570514bc600c5]::interface::run_compiler<core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>, rustc_driver[f414b4652d8f0863]::run_compiler::{closure#1}>::{closure#0}, core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>>::{closure#0}, core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>>
  52:     0x7fba1305b090 - <<std[2e8a54ede7eda140]::thread::Builder>::spawn_unchecked_<rustc_interface[6df570514bc600c5]::util::run_in_thread_pool_with_globals<rustc_interface[6df570514bc600c5]::interface::run_compiler<core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>, rustc_driver[f414b4652d8f0863]::run_compiler::{closure#1}>::{closure#0}, core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>>::{closure#0}, core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>>::{closure#1} as core[93d0d218afed051e]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  53:     0x7fba12645b35 - std::sys::unix::thread::Thread::new::thread_start::h781ed01b585563bf
  54:     0x7fba123e0b43 - <unknown>
  55:     0x7fba12472a00 - <unknown>
  56:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.66.0-nightly (7f7051991 2022-10-02) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `m::f`
#1 [typeck_item_bodies] type-checking all item bodies
#2 [analysis] running analysis passes on this crate
end of query stack
error: internal compiler error: compiler/rustc_middle/src/hir/map/mod.rs:872:18: expected expr, found local let _: E = m::E::Fn; (hir_id=HirId { owner: OwnerId { def_id: DefId(0:28 ~ privacy_enum_ctor[66ef]::main) }, local_id: 16 })
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1502:9
stack backtrace:
   0:     0x7fba1263572e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::he9cf1d92f30cd7c4
   0:     0x7fba1263572e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::he9cf1d92f30cd7c4
   1:     0x7fba1269e588 - core::fmt::write::hb71f33d6dcbbaae1
   2:     0x7fba12626651 - std::io::Write::write_fmt::hba03ba06b18d9e62
   3:     0x7fba126387de - std::panicking::default_hook::{{closure}}::hdef4e1c9c8ad451e
   4:     0x7fba12638499 - std::panicking::default_hook::hc35395d68df0c59f
   5:     0x7fba12ff2874 - rustc_driver[f414b4652d8f0863]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fba12638f9d - std::panicking::rust_panic_with_hook::hee5baad676042f5e
   7:     0x7fba15a59cd3 - std[2e8a54ede7eda140]::panicking::begin_panic::<rustc_errors[215d8a3c2f9dfb2d]::ExplicitBug>::{closure#0}
   8:     0x7fba15a56476 - std[2e8a54ede7eda140]::sys_common::backtrace::__rust_end_short_backtrace::<std[2e8a54ede7eda140]::panicking::begin_panic<rustc_errors[215d8a3c2f9dfb2d]::ExplicitBug>::{closure#0}, !>
   9:     0x7fba12f95376 - std[2e8a54ede7eda140]::panicking::begin_panic::<rustc_errors[215d8a3c2f9dfb2d]::ExplicitBug>
  10:     0x7fba15a4aaa6 - std[2e8a54ede7eda140]::panic::panic_any::<rustc_errors[215d8a3c2f9dfb2d]::ExplicitBug>
  11:     0x7fba15a436b9 - <rustc_errors[215d8a3c2f9dfb2d]::HandlerInner>::bug::<&alloc[13583fc3d0f3e700]::string::String>
  12:     0x7fba15a43190 - <rustc_errors[215d8a3c2f9dfb2d]::Handler>::bug::<&alloc[13583fc3d0f3e700]::string::String>
  13:     0x7fba15bcfbfe - rustc_middle[d24c4156abdc8924]::ty::context::tls::with_context_opt::<rustc_middle[d24c4156abdc8924]::ty::context::tls::with_opt<rustc_middle[d24c4156abdc8924]::util::bug::opt_span_bug_fmt<rustc_span[435a2de3bfa5601f]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  14:     0x7fba15bdc4f9 - rustc_middle[d24c4156abdc8924]::util::bug::opt_span_bug_fmt::<rustc_span[435a2de3bfa5601f]::span_encoding::Span>
  15:     0x7fba12f9f7b8 - rustc_middle[d24c4156abdc8924]::util::bug::bug_fmt
  16:     0x7fba15b07c03 - <rustc_middle[d24c4156abdc8924]::hir::map::Map>::expect_expr
  17:     0x7fba134ace62 - <rustc_hir_analysis[d8ce6c1405752847]::check::fn_ctxt::FnCtxt>::suggest_deref_ref_or_into
  18:     0x7fba134798f0 - <rustc_hir_analysis[d8ce6c1405752847]::check::fn_ctxt::FnCtxt>::emit_coerce_suggestions
  19:     0x7fba134eb755 - <rustc_hir_analysis[d8ce6c1405752847]::check::fn_ctxt::FnCtxt>::demand_coerce_diag
  20:     0x7fba1347f9d8 - <rustc_hir_analysis[d8ce6c1405752847]::check::fn_ctxt::FnCtxt>::demand_coerce
  21:     0x7fba134a4c9c - <rustc_hir_analysis[d8ce6c1405752847]::check::fn_ctxt::FnCtxt>::check_decl
  22:     0x7fba134a505b - <rustc_hir_analysis[d8ce6c1405752847]::check::fn_ctxt::FnCtxt>::check_stmt
  23:     0x7fba134a5774 - <rustc_hir_analysis[d8ce6c1405752847]::check::fn_ctxt::FnCtxt>::check_block_with_expected
  24:     0x7fba134ecbdb - <rustc_hir_analysis[d8ce6c1405752847]::check::fn_ctxt::FnCtxt>::check_expr_kind
  25:     0x7fba13485dac - <rustc_hir_analysis[d8ce6c1405752847]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  26:     0x7fba134ebc82 - <rustc_hir_analysis[d8ce6c1405752847]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  27:     0x7fba134875ac - <rustc_hir_analysis[d8ce6c1405752847]::check::fn_ctxt::FnCtxt>::check_return_expr
  28:     0x7fba1373ecbb - rustc_hir_analysis[d8ce6c1405752847]::check::check::check_fn
  29:     0x7fba13732d63 - <rustc_hir_analysis[d8ce6c1405752847]::check::inherited::InheritedBuilder>::enter::<rustc_hir_analysis[d8ce6c1405752847]::check::typeck_with_fallback<rustc_hir_analysis[d8ce6c1405752847]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[d24c4156abdc8924]::ty::context::TypeckResults>
  30:     0x7fba135f023b - rustc_hir_analysis[d8ce6c1405752847]::check::typeck
  31:     0x7fba14c1a71c - rustc_query_system[f5f779109e483cf5]::query::plumbing::try_execute_query::<rustc_query_impl[3cf691212940497a]::plumbing::QueryCtxt, rustc_query_system[f5f779109e483cf5]::query::caches::DefaultCache<rustc_span[435a2de3bfa5601f]::def_id::LocalDefId, &rustc_middle[d24c4156abdc8924]::ty::context::TypeckResults>>
  32:     0x7fba14d46ebd - rustc_query_system[f5f779109e483cf5]::query::plumbing::get_query::<rustc_query_impl[3cf691212940497a]::queries::typeck, rustc_query_impl[3cf691212940497a]::plumbing::QueryCtxt>
  33:     0x7fba14859600 - <rustc_query_impl[3cf691212940497a]::Queries as rustc_middle[d24c4156abdc8924]::ty::query::QueryEngine>::typeck
  34:     0x7fba135e75e6 - <core[93d0d218afed051e]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[8b8c5aa8aa14aa0c]::sync::par_for_each_in<&[rustc_span[435a2de3bfa5601f]::def_id::LocalDefId], <rustc_middle[d24c4156abdc8924]::hir::map::Map>::par_body_owners<rustc_hir_analysis[d8ce6c1405752847]::check::typeck_item_bodies::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[93d0d218afed051e]::ops::function::FnOnce<()>>::call_once
  35:     0x7fba137f7a99 - std[2e8a54ede7eda140]::panicking::try::<(), core[93d0d218afed051e]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[8b8c5aa8aa14aa0c]::sync::par_for_each_in<&[rustc_span[435a2de3bfa5601f]::def_id::LocalDefId], <rustc_middle[d24c4156abdc8924]::hir::map::Map>::par_body_owners<rustc_hir_analysis[d8ce6c1405752847]::check::typeck_item_bodies::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  36:     0x7fba1351488d - rustc_data_structures[8b8c5aa8aa14aa0c]::sync::par_for_each_in::<&[rustc_span[435a2de3bfa5601f]::def_id::LocalDefId], <rustc_middle[d24c4156abdc8924]::hir::map::Map>::par_body_owners<rustc_hir_analysis[d8ce6c1405752847]::check::typeck_item_bodies::{closure#0}>::{closure#0}>
  37:     0x7fba135f4bb7 - rustc_hir_analysis[d8ce6c1405752847]::check::typeck_item_bodies
  38:     0x7fba14c6f247 - rustc_query_system[f5f779109e483cf5]::query::plumbing::try_execute_query::<rustc_query_impl[3cf691212940497a]::plumbing::QueryCtxt, rustc_query_system[f5f779109e483cf5]::query::caches::DefaultCache<(), ()>>
  39:     0x7fba14d0e4eb - rustc_query_system[f5f779109e483cf5]::query::plumbing::get_query::<rustc_query_impl[3cf691212940497a]::queries::typeck_item_bodies, rustc_query_impl[3cf691212940497a]::plumbing::QueryCtxt>
  40:     0x7fba14858eaa - <rustc_query_impl[3cf691212940497a]::Queries as rustc_middle[d24c4156abdc8924]::ty::query::QueryEngine>::typeck_item_bodies
  41:     0x7fba1359c5db - <rustc_session[16cf70e3063e72cf]::session::Session>::time::<(), rustc_hir_analysis[d8ce6c1405752847]::check_crate::{closure#7}>
  42:     0x7fba1382c202 - rustc_hir_analysis[d8ce6c1405752847]::check_crate
  43:     0x7fba1314d081 - rustc_interface[6df570514bc600c5]::passes::analysis
  44:     0x7fba14c63cf8 - rustc_query_system[f5f779109e483cf5]::query::plumbing::try_execute_query::<rustc_query_impl[3cf691212940497a]::plumbing::QueryCtxt, rustc_query_system[f5f779109e483cf5]::query::caches::DefaultCache<(), core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>>>
  45:     0x7fba14d4726b - rustc_query_system[f5f779109e483cf5]::query::plumbing::get_query::<rustc_query_impl[3cf691212940497a]::queries::analysis, rustc_query_impl[3cf691212940497a]::plumbing::QueryCtxt>
  46:     0x7fba148314fa - <rustc_query_impl[3cf691212940497a]::Queries as rustc_middle[d24c4156abdc8924]::ty::query::QueryEngine>::analysis
  47:     0x7fba13053886 - <rustc_interface[6df570514bc600c5]::passes::QueryContext>::enter::<rustc_driver[f414b4652d8f0863]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>>
  48:     0x7fba12ff691c - rustc_interface[6df570514bc600c5]::interface::create_compiler_and_run::<core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>, rustc_driver[f414b4652d8f0863]::run_compiler::{closure#1}>
  49:     0x7fba13061f4f - <scoped_tls[799d6291841bce05]::ScopedKey<rustc_span[435a2de3bfa5601f]::SessionGlobals>>::set::<rustc_interface[6df570514bc600c5]::interface::run_compiler<core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>, rustc_driver[f414b4652d8f0863]::run_compiler::{closure#1}>::{closure#0}, core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>>
  50:     0x7fba1305749f - std[2e8a54ede7eda140]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[6df570514bc600c5]::util::run_in_thread_pool_with_globals<rustc_interface[6df570514bc600c5]::interface::run_compiler<core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>, rustc_driver[f414b4652d8f0863]::run_compiler::{closure#1}>::{closure#0}, core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>>::{closure#0}, core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>>
  51:     0x7fba12ff5191 - std[2e8a54ede7eda140]::panic::catch_unwind::<core[93d0d218afed051e]::panic::unwind_safe::AssertUnwindSafe<<std[2e8a54ede7eda140]::thread::Builder>::spawn_unchecked_<rustc_interface[6df570514bc600c5]::util::run_in_thread_pool_with_globals<rustc_interface[6df570514bc600c5]::interface::run_compiler<core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>, rustc_driver[f414b4652d8f0863]::run_compiler::{closure#1}>::{closure#0}, core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>>::{closure#0}, core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>>
  52:     0x7fba1305b090 - <<std[2e8a54ede7eda140]::thread::Builder>::spawn_unchecked_<rustc_interface[6df570514bc600c5]::util::run_in_thread_pool_with_globals<rustc_interface[6df570514bc600c5]::interface::run_compiler<core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>, rustc_driver[f414b4652d8f0863]::run_compiler::{closure#1}>::{closure#0}, core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>>::{closure#0}, core[93d0d218afed051e]::result::Result<(), rustc_errors[215d8a3c2f9dfb2d]::ErrorGuaranteed>>::{closure#1} as core[93d0d218afed051e]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  53:     0x7fba12645b35 - std::sys::unix::thread::Thread::new::thread_start::h781ed01b585563bf
  54:     0x7fba123e0b43 - <unknown>
  55:     0x7fba12472a00 - <unknown>
  56:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.66.0-nightly (7f7051991 2022-10-02) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `main`
#1 [typeck_item_bodies] type-checking all item bodies
#2 [analysis] running analysis passes on this crate
error: aborting due to 19 previous errors

Some errors have detailed explanations: E0412, E0423, E0603.
For more information about an error, try `rustc --explain E0412`.
For more information about an error, try `rustc --explain E0412`.
------------------------------------------


---- [ui] src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2497-if-let-chains/disallowed-positions" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2497-if-let-chains/disallowed-positions/auxiliary"
stdout: none
--- stderr -------------------------------
error: expected expression, found `let` statement
   |
   |
LL |     if (let 0 = 1) {}

error: expected expression, found `let` statement
  --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:33:11
   |
   |
LL |     if (((let 0 = 1))) {}

error: expected expression, found `let` statement
  --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:37:9
   |
   |
LL |     if (let 0 = 1) && true {}

error: expected expression, found `let` statement
  --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:41:17
   |
   |
LL |     if true && (let 0 = 1) {}

error: expected expression, found `let` statement
  --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:45:9
   |
   |
LL |     if (let 0 = 1) && (let 0 = 1) {}

error: expected expression, found `let` statement
  --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:45:24
   |
   |
LL |     if (let 0 = 1) && (let 0 = 1) {}

error: expected expression, found `let` statement
  --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:51:35
   |
   |
LL |     if let 0 = 1 && let 1 = 2 && (let 2 = 3 && let 3 = 4 && let 4 = 5) {}

error: expected expression, found `let` statement
  --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:51:48
   |
   |
LL |     if let 0 = 1 && let 1 = 2 && (let 2 = 3 && let 3 = 4 && let 4 = 5) {}

error: expected expression, found `let` statement
  --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:51:61
   |
   |
LL |     if let 0 = 1 && let 1 = 2 && (let 2 = 3 && let 3 = 4 && let 4 = 5) {}

error: expected expression, found `let` statement
  --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:61:12
   |
   |
LL |     while (let 0 = 1) {}

error: expected expression, found `let` statement
  --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:65:14
   |
   |
LL |     while (((let 0 = 1))) {}

error: expected expression, found `let` statement
  --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:69:12
   |
   |
LL |     while (let 0 = 1) && true {}

error: expected expression, found `let` statement
  --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:73:20
   |
   |
LL |     while true && (let 0 = 1) {}

error: expected expression, found `let` statement
  --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:77:12
   |
   |
LL |     while (let 0 = 1) && (let 0 = 1) {}

error: expected expression, found `let` statement
  --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:77:27
   |
   |
LL |     while (let 0 = 1) && (let 0 = 1) {}

error: expected expression, found `let` statement
  --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:83:38
   |
   |
LL |     while let 0 = 1 && let 1 = 2 && (let 2 = 3 && let 3 = 4 && let 4 = 5) {}

error: expected expression, found `let` statement
  --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:83:51
   |
   |
LL |     while let 0 = 1 && let 1 = 2 && (let 2 = 3 && let 3 = 4 && let 4 = 5) {}

error: expected expression, found `let` statement
  --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:83:64
   |
   |
LL |     while let 0 = 1 && let 1 = 2 && (let 2 = 3 && let 3 = 4 && let 4 = 5) {}

error: expected expression, found `let` statement
  --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:110:9
   |
   |
LL |     if &let 0 = 0 {}

error: expected expression, found `let` statement
  --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:115:9
   |
   |
LL |     if !let 0 = 0 {}

error: expected expression, found `let` statement
  --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:118:9
   |
   |
LL |     if *let 0 = 0 {}

error: expected expression, found `let` statement
  --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:122:9
   |
   |
LL |     if -let 0 = 0 {}

error: expected expression, found `let` statement
  --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:132:9
   |
   |
LL |     if (let 0 = 0)? {}

error: expected expression, found `let` statement
  --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:138:16
   |
   |
LL |     if true || let 0 = 0 {}

error: expected expression, found `let` statement
  --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:141:17
   |
   |
LL |     if (true || let 0 = 0) {}

error: expected expression, found `let` statement
  --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:144:25
   |
   |
LL |     if true && (true || let 0 = 0) {}

error: expected expression, found `let` statement
  --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:147:25
   |
   |
---

error: expected expression, found `let` statement
  --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:368:25
   |
LL |         let x = true && let y = 1;

error: expected expression, found `let` statement
  --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:374:19
   |
   |
LL |         [1, 2, 3][let _ = ()]

error: expected expression, found `let` statement
  --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:379:6
   |
   |
LL |     &let 0 = 0

error: expected expression, found `let` statement
  --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:391:17
   |
   |
LL |         true && let 1 = 1

error: expected expression, found `let` statement
  --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:397:17
   |
   |
LL |         true && let 1 = 1

error: expected expression, found `let` statement
  --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:403:17
   |
   |
LL |         true && let 1 = 1

error: expected expression, found `let` statement
  --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:415:17
   |
   |
LL |         true && let 1 = 1


error: expressions must be enclosed in braces to be used as const generic arguments
   |
   |
LL |         true && let 1 = 1
   |
help: enclose the `const` expression in braces
   |
   |
LL |         { true && let 1 = 1 }

error: expected expression, found `let` statement
  --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:425:9
   |
   |
LL |     if (let Some(a) = opt && true) {

error: expected expression, found `let` statement
  --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:430:9
   |
   |
LL |     if (let Some(a) = opt) && true {

error: expected expression, found `let` statement
  --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:434:9
   |
   |
LL |     if (let Some(a) = opt) && (let Some(b) = a) {

error: expected expression, found `let` statement
  --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:434:32
   |
   |
LL |     if (let Some(a) = opt) && (let Some(b) = a) {

error: expected expression, found `let` statement
  --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:443:9
   |
   |
LL |     if (let Some(a) = opt && (let Some(b) = a)) && b == 1 {

error: expected expression, found `let` statement
  --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:443:31
   |
   |
LL |     if (let Some(a) = opt && (let Some(b) = a)) && b == 1 {

error: expected expression, found `let` statement
  --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:449:9
   |
   |
LL |     if (let Some(a) = opt && (let Some(b) = a)) && true {

error: expected expression, found `let` statement
  --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:449:31
   |
   |
LL |     if (let Some(a) = opt && (let Some(b) = a)) && true {

error: expected expression, found `let` statement
  --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:455:9
   |
   |
LL |     if (let Some(a) = opt && (true)) && true {

error: expected expression, found `let` statement
  --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:472:22
   |
   |
LL |     let x = (true && let y = 1);

error: expected expression, found `let` statement
  --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:477:20
   |
   |
LL |         ([1, 2, 3][let _ = ()])

error: expected expression, found `let` statement
  --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:99:16
   |
   |
LL |     use_expr!((let 0 = 1 && 0 == 0));

error: expected expression, found `let` statement
  --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:103:16
   |
   |
LL |     use_expr!((let 0 = 1));

error: `let` expressions are not supported here
  --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:29:9
   |
   |
LL |     if (let 0 = 1) {}
   |
   |
   = note: only supported directly in conditions of `if` and `while` expressions
note: `let`s wrapped in parentheses are not supported in a context with let chains
   |
   |
LL |     if (let 0 = 1) {}

error: `let` expressions are not supported here
  --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:33:11
   |
   |
LL |     if (((let 0 = 1))) {}
   |
   |
   = note: only supported directly in conditions of `if` and `while` expressions
note: `let`s wrapped in parentheses are not supported in a context with let chains
   |
   |
LL |     if (((let 0 = 1))) {}

error: `let` expressions are not supported here
  --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:37:9
   |
   |
LL |     if (let 0 = 1) && true {}
   |
   |
   = note: only supported directly in conditions of `if` and `while` expressions
note: `let`s wrapped in parentheses are not supported in a context with let chains
   |
   |
LL |     if (let 0 = 1) && true {}

error: `let` expressions are not supported here
  --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:41:17
   |
   |
LL |     if true && (let 0 = 1) {}
   |
   |
   = note: only supported directly in conditions of `if` and `while` expressions
note: `let`s wrapped in parentheses are not supported in a context with let chains
   |
   |
LL |     if true && (let 0 = 1) {}

error: `let` expressions are not supported here
  --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:45:9
   |
   |
LL |     if (let 0 = 1) && (let 0 = 1) {}
   |
   |
   = note: only supported directly in conditions of `if` and `while` expressions
note: `let`s wrapped in parentheses are not supported in a context with let chains
   |
   |
LL |     if (let 0 = 1) && (let 0 = 1) {}

error: `let` expressions are not supported here
  --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:45:24
   |
   |
LL |     if (let 0 = 1) && (let 0 = 1) {}
   |
   |
   = note: only supported directly in conditions of `if` and `while` expressions
note: `let`s wrapped in parentheses are not supported in a context with let chains
   |
   |
LL |     if (let 0 = 1) && (let 0 = 1) {}

error: `let` expressions are not supported here
  --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:51:35
   |
   |
LL |     if let 0 = 1 && let 1 = 2 && (let 2 = 3 && let 3 = 4 && let 4 = 5) {}
   |
   |
   = note: only supported directly in conditions of `if` and `while` expressions
note: `let`s wrapped in parentheses are not supported in a context with let chains
   |
   |
LL |     if let 0 = 1 && let 1 = 2 && (let 2 = 3 && let 3 = 4 && let 4 = 5) {}

error: `let` expressions are not supported here
  --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:51:48
   |
   |
LL |     if let 0 = 1 && let 1 = 2 && (let 2 = 3 && let 3 = 4 && let 4 = 5) {}
   |
   |
   = note: only supported directly in conditions of `if` and `while` expressions
note: `let`s wrapped in parentheses are not supported in a context with let chains
   |
   |
LL |     if let 0 = 1 && let 1 = 2 && (let 2 = 3 && let 3 = 4 && let 4 = 5) {}

error: `let` expressions are not supported here
  --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:51:61
   |
   |
LL |     if let 0 = 1 && let 1 = 2 && (let 2 = 3 && let 3 = 4 && let 4 = 5) {}
   |
   |
   = note: only supported directly in conditions of `if` and `while` expressions
note: `let`s wrapped in parentheses are not supported in a context with let chains
   |
   |
LL |     if let 0 = 1 && let 1 = 2 && (let 2 = 3 && let 3 = 4 && let 4 = 5) {}

error: `let` expressions are not supported here
  --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:61:12
   |
   |
LL |     while (let 0 = 1) {}
   |
   |
   = note: only supported directly in conditions of `if` and `while` expressions
note: `let`s wrapped in parentheses are not supported in a context with let chains
   |
   |
LL |     while (let 0 = 1) {}

error: `let` expressions are not supported here
  --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:65:14
   |
   |
LL |     while (((let 0 = 1))) {}
   |
   |
   = note: only supported directly in conditions of `if` and `while` expressions
note: `let`s wrapped in parentheses are not supported in a context with let chains
   |
   |
LL |     while (((let 0 = 1))) {}

error: `let` expressions are not supported here
  --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:69:12
   |
   |
LL |     while (let 0 = 1) && true {}
   |
   |
   = note: only supported directly in conditions of `if` and `while` expressions
note: `let`s wrapped in parentheses are not supported in a context with let chains
   |
   |
LL |     while (let 0 = 1) && true {}

error: `let` expressions are not supported here
  --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:73:20
   |
   |
LL |     while true && (let 0 = 1) {}
   |
   |
   = note: only supported directly in conditions of `if` and `while` expressions
note: `let`s wrapped in parentheses are not supported in a context with let chains
   |
   |
LL |     while true && (let 0 = 1) {}

error: `let` expressions are not supported here
  --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:77:12
   |
   |
LL |     while (let 0 = 1) && (let 0 = 1) {}
   |
   |
   = note: only supported directly in conditions of `if` and `while` expressions
note: `let`s wrapped in parentheses are not supported in a context with let chains
   |
   |
LL |     while (let 0 = 1) && (let 0 = 1) {}

error: `let` expressions are not supported here
  --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:77:27
   |
   |
LL |     while (let 0 = 1) && (let 0 = 1) {}
   |
   |
   = note: only supported directly in conditions of `if` and `while` expressions
note: `let`s wrapped in parentheses are not supported in a context with let chains
   |
   |
LL |     while (let 0 = 1) && (let 0 = 1) {}

error: `let` expressions are not supported here
  --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:83:38
   |
   |
LL |     while let 0 = 1 && let 1 = 2 && (let 2 = 3 && let 3 = 4 && let 4 = 5) {}
   |
   |
   = note: only supported directly in conditions of `if` and `while` expressions
note: `let`s wrapped in parentheses are not supported in a context with let chains
   |
   |
LL |     while let 0 = 1 && let 1 = 2 && (let 2 = 3 && let 3 = 4 && let 4 = 5) {}

error: `let` expressions are not supported here
  --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:83:51
   |
   |
LL |     while let 0 = 1 && let 1 = 2 && (let 2 = 3 && let 3 = 4 && let 4 = 5) {}
   |
   |
   = note: only supported directly in conditions of `if` and `while` expressions
note: `let`s wrapped in parentheses are not supported in a context with let chains
   |
   |
LL |     while let 0 = 1 && let 1 = 2 && (let 2 = 3 && let 3 = 4 && let 4 = 5) {}

error: `let` expressions are not supported here
  --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:83:64
   |
   |
LL |     while let 0 = 1 && let 1 = 2 && (let 2 = 3 && let 3 = 4 && let 4 = 5) {}
   |
   |
   = note: only supported directly in conditions of `if` and `while` expressions
note: `let`s wrapped in parentheses are not supported in a context with let chains
   |
   |
LL |     while let 0 = 1 && let 1 = 2 && (let 2 = 3 && let 3 = 4 && let 4 = 5) {}

error: `let` expressions are not supported here
  --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:99:16
   |
   |
LL |     use_expr!((let 0 = 1 && 0 == 0));
   |
   |
   = note: only supported directly in conditions of `if` and `while` expressions
note: `let`s wrapped in parentheses are not supported in a context with let chains
   |
   |
LL |     use_expr!((let 0 = 1 && 0 == 0));

error: `let` expressions are not supported here
  --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:99:16
   |
   |
LL |     use_expr!((let 0 = 1 && 0 == 0));
   |
   |
   = note: only supported directly in conditions of `if` and `while` expressions
note: `let`s wrapped in parentheses are not supported in a context with let chains
   |
   |
LL |     use_expr!((let 0 = 1 && 0 == 0));

error: `let` expressions are not supported here
  --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:103:16
   |
   |
LL |     use_expr!((let 0 = 1));
   |
   |
   = note: only supported directly in conditions of `if` and `while` expressions
note: `let`s wrapped in parentheses are not supported in a context with let chains
   |
   |
LL |     use_expr!((let 0 = 1));

error: `let` expressions are not supported here
  --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:103:16
   |
   |
LL |     use_expr!((let 0 = 1));
   |
   |
   = note: only supported directly in conditions of `if` and `while` expressions
note: `let`s wrapped in parentheses are not supported in a context with let chains
   |
   |
LL |     use_expr!((let 0 = 1));

error: `let` expressions are not supported here
  --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:110:9
   |
   |
LL |     if &let 0 = 0 {}
   |
   |
   = note: only supported directly in conditions of `if` and `while` expressions
error: `let` expressions are not supported here
  --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:115:9
   |
   |
LL |     if !let 0 = 0 {}
   |
   |
   = note: only supported directly in conditions of `if` and `while` expressions
error: `let` expressions are not supported here
  --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:118:9
   |
   |
LL |     if *let 0 = 0 {}
   |
   |
   = note: only supported directly in conditions of `if` and `while` expressions
error: `let` expressions are not supported here
  --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:122:9
   |
   |
LL |     if -let 0 = 0 {}
   |
   |
   = note: only supported directly in conditions of `if` and `while` expressions
error: `let` expressions are not supported here
  --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:132:9
   |
   |
LL |     if (let 0 = 0)? {}
   |
   |
   = note: only supported directly in conditions of `if` and `while` expressions
note: `let`s wrapped in parentheses are not supported in a context with let chains
   |
   |
LL |     if (let 0 = 0)? {}

error: `let` expressions are not supported here
  --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:138:16
   |
   |
LL |     if true || let 0 = 0 {}
   |
   |
   = note: only supported directly in conditions of `if` and `while` expressions
note: `||` operators are not supported in let chain expressions
   |
   |
LL |     if true || let 0 = 0 {}

error: `let` expressions are not supported here
  --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:141:17
   |
   |
LL |     if (true || let 0 = 0) {}
   |
   |
   = note: only supported directly in conditions of `if` and `while` expressions
note: `||` operators are not supported in let chain expressions
   |
   |
LL |     if (true || let 0 = 0) {}

error: `let` expressions are not supported here
  --> /checkout/src/test/ui/rfc-2497-if-let-chains/disallowed-positions.rs:144:25
   |
   |
LL |     if true && (true || let 0 = 0) {}
   |
   |
   = note: only supported directly in conditions of `if` and `while` expressions
note: `||` operators are not supported in let chain expressions
   |
   |
LL |     if true && (true || let 0 = 0) {}
