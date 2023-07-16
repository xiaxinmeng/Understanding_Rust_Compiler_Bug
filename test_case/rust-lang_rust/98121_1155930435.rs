plain
..........................................................................i............. 1936/13068
........................................................................................ 2024/13068
........................................................................................ 2112/13068
........................................................................................ 2200/13068
.................................................................................F...F.. 2288/13068
........................................................................................ 2464/13068
........................................................................................ 2552/13068
........................................................................................ 2640/13068
........................................................................................ 2728/13068
---
failures:

---- [ui] src/test/ui/const-generics/occurs-check/unused-substs-3.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/occurs-check/unused-substs-3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/occurs-check/unused-substs-3" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/occurs-check/unused-substs-3/auxiliary"
stdout: none
--- stderr -------------------------------
  left: `7`,
  left: `7`,
 right: `10633823966279326983230456482242756615`: Scalar value 0x8000000000000000000000000000007 exceeds size of 8 bytes', compiler/rustc_middle/src/ty/consts/int.rs:177:9
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   0:     0x7f5633eb6cfc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h82616d2a02b95547
   0:     0x7f5633eb6cfc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h82616d2a02b95547
   1:     0x7f5633f1de58 - core::fmt::write::hb0c658a2cfd96fc4
   2:     0x7f5633ea6ae1 - std::io::Write::write_fmt::h54c5ac5d2e976cb3
   3:     0x7f5633eb9d2e - std::panicking::default_hook::{{closure}}::hb23da20a35556c6d
   4:     0x7f5633eb9a19 - std::panicking::default_hook::h9d2e97522f694923
   5:     0x7f56349e01a1 - rustc_driver[39665097b4ed30e4]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f5633eba492 - std::panicking::rust_panic_with_hook::h5dab4f648715c4a0
   7:     0x7f5633eba2c7 - std::panicking::begin_panic_handler::{{closure}}::ha82f8a91de2b59b2
   8:     0x7f5633eb7224 - std::sys_common::backtrace::__rust_end_short_backtrace::h52959e2d89f01605
   9:     0x7f5633eb9fa9 - rust_begin_unwind
  10:     0x7f5633e6e013 - core::panicking::panic_fmt::h4c9dfce10852636f
  11:     0x7f5633f1a49e - core::panicking::assert_failed_inner::h03f0940274c2ca18
  12:     0x7f563496087b - core[71408e98247a1ba3]::panicking::assert_failed::<u128, u128>
  13:     0x7f56374a8f8f - <rustc_middle[50fd58bc93f4b907]::ty::consts::int::ScalarInt>::to_bits
  14:     0x7f56374a91ce - <rustc_middle[50fd58bc93f4b907]::ty::consts::valtree::ValTree>::try_to_machine_usize
  15:     0x7f5637235023 - rustc_middle[50fd58bc93f4b907]::ty::relate::super_relate_tys::<rustc_infer[c164df521a26f6]::infer::combine::Generalizer>
  16:     0x7f56371e857a - <rustc_infer[c164df521a26f6]::infer::combine::Generalizer as rustc_middle[50fd58bc93f4b907]::ty::relate::TypeRelation>::tys
  17:     0x7f56371edf6c - <rustc_infer[c164df521a26f6]::infer::combine::CombineFields>::instantiate
  18:     0x7f563722780b - <rustc_infer[c164df521a26f6]::infer::sub::Sub as rustc_middle[50fd58bc93f4b907]::ty::relate::TypeRelation>::tys
  19:     0x7f5635522880 - <rustc_infer[c164df521a26f6]::infer::InferCtxt>::commit_if_ok::<rustc_infer[c164df521a26f6]::infer::InferOk<()>, rustc_middle[50fd58bc93f4b907]::ty::error::TypeError, <rustc_infer[c164df521a26f6]::infer::at::Trace>::sub<rustc_middle[50fd58bc93f4b907]::ty::Ty>::{closure#0}>
  20:     0x7f56354d9de3 - <rustc_infer[c164df521a26f6]::infer::at::At>::sub_exp::<rustc_middle[50fd58bc93f4b907]::ty::Ty>
  21:     0x7f563551fab7 - <rustc_infer[c164df521a26f6]::infer::InferCtxt>::commit_if_ok::<rustc_infer[c164df521a26f6]::infer::InferOk<rustc_middle[50fd58bc93f4b907]::ty::Ty>, rustc_middle[50fd58bc93f4b907]::ty::error::TypeError, <rustc_typeck[87f5f0e1435a037b]::check::coercion::Coerce>::unify::{closure#0}>
  22:     0x7f5635566d02 - <rustc_typeck[87f5f0e1435a037b]::check::coercion::Coerce>::unify
  23:     0x7f563557c6f1 - <rustc_typeck[87f5f0e1435a037b]::check::coercion::Coerce>::coerce
  24:     0x7f5635520197 - <rustc_infer[c164df521a26f6]::infer::InferCtxt>::commit_if_ok::<rustc_infer[c164df521a26f6]::infer::InferOk<(alloc[7d46d0994c758d7d]::vec::Vec<rustc_middle[50fd58bc93f4b907]::ty::adjustment::Adjustment>, rustc_middle[50fd58bc93f4b907]::ty::Ty)>, rustc_middle[50fd58bc93f4b907]::ty::error::TypeError, <rustc_typeck[87f5f0e1435a037b]::check::fn_ctxt::FnCtxt>::try_find_coercion_lub<&rustc_hir[d468c7651af76b9f]::hir::Expr>::{closure#7}>
  25:     0x7f563521bba3 - <rustc_typeck[87f5f0e1435a037b]::check::fn_ctxt::FnCtxt>::try_coerce
  26:     0x7f5635285ecc - <rustc_typeck[87f5f0e1435a037b]::check::fn_ctxt::FnCtxt>::demand_coerce_diag
  27:     0x7f5635288594 - <rustc_typeck[87f5f0e1435a037b]::check::fn_ctxt::FnCtxt>::check_expr_kind
  28:     0x7f563522c9eb - <rustc_typeck[87f5f0e1435a037b]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  29:     0x7f5635286949 - <rustc_typeck[87f5f0e1435a037b]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  30:     0x7f5635246f71 - <rustc_typeck[87f5f0e1435a037b]::check::fn_ctxt::FnCtxt>::check_stmt
  31:     0x7f5635247514 - <rustc_typeck[87f5f0e1435a037b]::check::fn_ctxt::FnCtxt>::check_block_with_expected
  32:     0x7f5635287e5c - <rustc_typeck[87f5f0e1435a037b]::check::fn_ctxt::FnCtxt>::check_expr_kind
  33:     0x7f563522c9eb - <rustc_typeck[87f5f0e1435a037b]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  34:     0x7f5635286949 - <rustc_typeck[87f5f0e1435a037b]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  35:     0x7f563522deb7 - <rustc_typeck[87f5f0e1435a037b]::check::fn_ctxt::FnCtxt>::check_return_expr
  36:     0x7f56353608b1 - rustc_typeck[87f5f0e1435a037b]::check::check::check_fn
  37:     0x7f563551058e - <rustc_infer[c164df521a26f6]::infer::InferCtxtBuilder>::enter::<&rustc_middle[50fd58bc93f4b907]::ty::context::TypeckResults, <rustc_typeck[87f5f0e1435a037b]::check::inherited::InheritedBuilder>::enter<rustc_typeck[87f5f0e1435a037b]::check::typeck_with_fallback<rustc_typeck[87f5f0e1435a037b]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[50fd58bc93f4b907]::ty::context::TypeckResults>::{closure#0}>
  38:     0x7f563534c0de - <rustc_typeck[87f5f0e1435a037b]::check::inherited::InheritedBuilder>::enter::<rustc_typeck[87f5f0e1435a037b]::check::typeck_with_fallback<rustc_typeck[87f5f0e1435a037b]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[50fd58bc93f4b907]::ty::context::TypeckResults>
  39:     0x7f56353cf39e - rustc_typeck[87f5f0e1435a037b]::check::typeck
  40:     0x7f563651d986 - rustc_query_system[fc5aa2a14ec399c6]::query::plumbing::try_execute_query::<rustc_query_impl[3ed6196958178b69]::plumbing::QueryCtxt, rustc_query_system[fc5aa2a14ec399c6]::query::caches::DefaultCache<rustc_span[97139491182189eb]::def_id::LocalDefId, &rustc_middle[50fd58bc93f4b907]::ty::context::TypeckResults>>
  41:     0x7f5636638457 - rustc_query_system[fc5aa2a14ec399c6]::query::plumbing::get_query::<rustc_query_impl[3ed6196958178b69]::queries::typeck, rustc_query_impl[3ed6196958178b69]::plumbing::QueryCtxt>
  42:     0x7f563618cbc4 - <rustc_query_impl[3ed6196958178b69]::Queries as rustc_middle[50fd58bc93f4b907]::ty::query::QueryEngine>::typeck
  43:     0x7f56355c221b - <rustc_middle[50fd58bc93f4b907]::hir::map::Map>::par_body_owners::<rustc_typeck[87f5f0e1435a037b]::check::typeck_item_bodies::{closure#0}>
  44:     0x7f56353d42ad - rustc_typeck[87f5f0e1435a037b]::check::typeck_item_bodies
  45:     0x7f563656610a - rustc_query_system[fc5aa2a14ec399c6]::query::plumbing::try_execute_query::<rustc_query_impl[3ed6196958178b69]::plumbing::QueryCtxt, rustc_query_system[fc5aa2a14ec399c6]::query::caches::DefaultCache<(), ()>>
  46:     0x7f56365fc095 - rustc_query_system[fc5aa2a14ec399c6]::query::plumbing::get_query::<rustc_query_impl[3ed6196958178b69]::queries::typeck_item_bodies, rustc_query_impl[3ed6196958178b69]::plumbing::QueryCtxt>
  47:     0x7f563618c66e - <rustc_query_impl[3ed6196958178b69]::Queries as rustc_middle[50fd58bc93f4b907]::ty::query::QueryEngine>::typeck_item_bodies
  48:     0x7f563554889a - <rustc_session[4211bfba78cbf5fb]::session::Session>::time::<(), rustc_typeck[87f5f0e1435a037b]::check_crate::{closure#7}>
  49:     0x7f56354926e8 - rustc_typeck[87f5f0e1435a037b]::check_crate
  50:     0x7f5634afe631 - rustc_interface[4d778361d2f35779]::passes::analysis
  51:     0x7f5636559ade - rustc_query_system[fc5aa2a14ec399c6]::query::plumbing::try_execute_query::<rustc_query_impl[3ed6196958178b69]::plumbing::QueryCtxt, rustc_query_system[fc5aa2a14ec399c6]::query::caches::DefaultCache<(), core[71408e98247a1ba3]::result::Result<(), rustc_errors[b963431c8540357]::ErrorGuaranteed>>>
  52:     0x7f5636638832 - rustc_query_system[fc5aa2a14ec399c6]::query::plumbing::get_query::<rustc_query_impl[3ed6196958178b69]::queries::analysis, rustc_query_impl[3ed6196958178b69]::plumbing::QueryCtxt>
  53:     0x7f5636170ade - <rustc_query_impl[3ed6196958178b69]::Queries as rustc_middle[50fd58bc93f4b907]::ty::query::QueryEngine>::analysis
  54:     0x7f5634a510f4 - <rustc_interface[4d778361d2f35779]::passes::QueryContext>::enter::<rustc_driver[39665097b4ed30e4]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[71408e98247a1ba3]::result::Result<(), rustc_errors[b963431c8540357]::ErrorGuaranteed>>
  55:     0x7f56349fd9c5 - <rustc_interface[4d778361d2f35779]::interface::Compiler>::enter::<rustc_driver[39665097b4ed30e4]::run_compiler::{closure#1}::{closure#2}, core[71408e98247a1ba3]::result::Result<core[71408e98247a1ba3]::option::Option<rustc_interface[4d778361d2f35779]::queries::Linker>, rustc_errors[b963431c8540357]::ErrorGuaranteed>>
  56:     0x7f56349e1b1b - rustc_span[97139491182189eb]::with_source_map::<core[71408e98247a1ba3]::result::Result<(), rustc_errors[b963431c8540357]::ErrorGuaranteed>, rustc_interface[4d778361d2f35779]::interface::create_compiler_and_run<core[71408e98247a1ba3]::result::Result<(), rustc_errors[b963431c8540357]::ErrorGuaranteed>, rustc_driver[39665097b4ed30e4]::run_compiler::{closure#1}>::{closure#1}>
  57:     0x7f56349feb94 - <scoped_tls[4ba69821ac6c74bd]::ScopedKey<rustc_span[97139491182189eb]::SessionGlobals>>::set::<rustc_interface[4d778361d2f35779]::interface::run_compiler<core[71408e98247a1ba3]::result::Result<(), rustc_errors[b963431c8540357]::ErrorGuaranteed>, rustc_driver[39665097b4ed30e4]::run_compiler::{closure#1}>::{closure#0}, core[71408e98247a1ba3]::result::Result<(), rustc_errors[b963431c8540357]::ErrorGuaranteed>>
  58:     0x7f5634a5cb89 - std[aa180c2ff93fe481]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[4d778361d2f35779]::util::run_in_thread_pool_with_globals<rustc_interface[4d778361d2f35779]::interface::run_compiler<core[71408e98247a1ba3]::result::Result<(), rustc_errors[b963431c8540357]::ErrorGuaranteed>, rustc_driver[39665097b4ed30e4]::run_compiler::{closure#1}>::{closure#0}, core[71408e98247a1ba3]::result::Result<(), rustc_errors[b963431c8540357]::ErrorGuaranteed>>::{closure#0}, core[71408e98247a1ba3]::result::Result<(), rustc_errors[b963431c8540357]::ErrorGuaranteed>>
  59:     0x7f5634a54fd9 - <<std[aa180c2ff93fe481]::thread::Builder>::spawn_unchecked_<rustc_interface[4d778361d2f35779]::util::run_in_thread_pool_with_globals<rustc_interface[4d778361d2f35779]::interface::run_compiler<core[71408e98247a1ba3]::result::Result<(), rustc_errors[b963431c8540357]::ErrorGuaranteed>, rustc_driver[39665097b4ed30e4]::run_compiler::{closure#1}>::{closure#0}, core[71408e98247a1ba3]::result::Result<(), rustc_errors[b963431c8540357]::ErrorGuaranteed>>::{closure#0}, core[71408e98247a1ba3]::result::Result<(), rustc_errors[b963431c8540357]::ErrorGuaranteed>>::{closure#1} as core[71408e98247a1ba3]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  60:     0x7f5633ec58b3 - std::sys::unix::thread::Thread::new::thread_start::h09854d189bc130f5
  61:     0x7f562e418609 - start_thread
  62:     0x7f5633d2b133 - clone
  63:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.63.0-nightly (13df7102b 2022-06-15) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `main`
#1 [typeck_item_bodies] type-checking all item bodies
#2 [analysis] running analysis passes on this crate
------------------------------------------


---- [ui] src/test/ui/const-generics/occurs-check/unused-substs-4.rs stdout ----
---- [ui] src/test/ui/const-generics/occurs-check/unused-substs-4.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/occurs-check/unused-substs-4.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/occurs-check/unused-substs-4" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/occurs-check/unused-substs-4/auxiliary"
stdout: none
--- stderr -------------------------------
  left: `7`,
  left: `7`,
 right: `10633823966279326983230456482242756615`: Scalar value 0x8000000000000000000000000000007 exceeds size of 8 bytes', compiler/rustc_middle/src/ty/consts/int.rs:177:9
stack backtrace:
   0:     0x7fb095678cfc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h82616d2a02b95547
   1:     0x7fb0956dfe58 - core::fmt::write::hb0c658a2cfd96fc4
   2:     0x7fb095668ae1 - std::io::Write::write_fmt::h54c5ac5d2e976cb3
   3:     0x7fb09567bd2e - std::panicking::default_hook::{{closure}}::hb23da20a35556c6d
   4:     0x7fb09567ba19 - std::panicking::default_hook::h9d2e97522f694923
   5:     0x7fb0961a21a1 - rustc_driver[39665097b4ed30e4]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fb09567c492 - std::panicking::rust_panic_with_hook::h5dab4f648715c4a0
   7:     0x7fb09567c2c7 - std::panicking::begin_panic_handler::{{closure}}::ha82f8a91de2b59b2
   8:     0x7fb095679224 - std::sys_common::backtrace::__rust_end_short_backtrace::h52959e2d89f01605
   9:     0x7fb09567bfa9 - rust_begin_unwind
  10:     0x7fb095630013 - core::panicking::panic_fmt::h4c9dfce10852636f
  11:     0x7fb0956dc49e - core::panicking::assert_failed_inner::h03f0940274c2ca18
  12:     0x7fb09612287b - core[71408e98247a1ba3]::panicking::assert_failed::<u128, u128>
  13:     0x7fb098c6af8f - <rustc_middle[50fd58bc93f4b907]::ty::consts::int::ScalarInt>::to_bits
  14:     0x7fb098c6b1ce - <rustc_middle[50fd58bc93f4b907]::ty::consts::valtree::ValTree>::try_to_machine_usize
  15:     0x7fb0989f4fef - rustc_middle[50fd58bc93f4b907]::ty::relate::super_relate_tys::<rustc_infer[c164df521a26f6]::infer::sub::Sub>
  16:     0x7fb09895c6b1 - <rustc_infer[c164df521a26f6]::infer::InferCtxt>::super_combine_tys::<rustc_infer[c164df521a26f6]::infer::sub::Sub>
  17:     0x7fb0989e9a0e - <rustc_infer[c164df521a26f6]::infer::sub::Sub as rustc_middle[50fd58bc93f4b907]::ty::relate::TypeRelation>::tys
  18:     0x7fb096ce4880 - <rustc_infer[c164df521a26f6]::infer::InferCtxt>::commit_if_ok::<rustc_infer[c164df521a26f6]::infer::InferOk<()>, rustc_middle[50fd58bc93f4b907]::ty::error::TypeError, <rustc_infer[c164df521a26f6]::infer::at::Trace>::sub<rustc_middle[50fd58bc93f4b907]::ty::Ty>::{closure#0}>
  19:     0x7fb096c9bde3 - <rustc_infer[c164df521a26f6]::infer::at::At>::sub_exp::<rustc_middle[50fd58bc93f4b907]::ty::Ty>
  20:     0x7fb096ce1ab7 - <rustc_infer[c164df521a26f6]::infer::InferCtxt>::commit_if_ok::<rustc_infer[c164df521a26f6]::infer::InferOk<rustc_middle[50fd58bc93f4b907]::ty::Ty>, rustc_middle[50fd58bc93f4b907]::ty::error::TypeError, <rustc_typeck[87f5f0e1435a037b]::check::coercion::Coerce>::unify::{closure#0}>
  21:     0x7fb096d28d02 - <rustc_typeck[87f5f0e1435a037b]::check::coercion::Coerce>::unify
  22:     0x7fb096d3e6f1 - <rustc_typeck[87f5f0e1435a037b]::check::coercion::Coerce>::coerce
  23:     0x7fb096ce2197 - <rustc_infer[c164df521a26f6]::infer::InferCtxt>::commit_if_ok::<rustc_infer[c164df521a26f6]::infer::InferOk<(alloc[7d46d0994c758d7d]::vec::Vec<rustc_middle[50fd58bc93f4b907]::ty::adjustment::Adjustment>, rustc_middle[50fd58bc93f4b907]::ty::Ty)>, rustc_middle[50fd58bc93f4b907]::ty::error::TypeError, <rustc_typeck[87f5f0e1435a037b]::check::fn_ctxt::FnCtxt>::try_find_coercion_lub<&rustc_hir[d468c7651af76b9f]::hir::Expr>::{closure#7}>
  24:     0x7fb0969ddba3 - <rustc_typeck[87f5f0e1435a037b]::check::fn_ctxt::FnCtxt>::try_coerce
  25:     0x7fb096a47ecc - <rustc_typeck[87f5f0e1435a037b]::check::fn_ctxt::FnCtxt>::demand_coerce_diag
  26:     0x7fb096a4a594 - <rustc_typeck[87f5f0e1435a037b]::check::fn_ctxt::FnCtxt>::check_expr_kind
  27:     0x7fb0969ee9eb - <rustc_typeck[87f5f0e1435a037b]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  28:     0x7fb096a48949 - <rustc_typeck[87f5f0e1435a037b]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  29:     0x7fb096a08f71 - <rustc_typeck[87f5f0e1435a037b]::check::fn_ctxt::FnCtxt>::check_stmt
  30:     0x7fb096a09514 - <rustc_typeck[87f5f0e1435a037b]::check::fn_ctxt::FnCtxt>::check_block_with_expected
  31:     0x7fb096a49e5c - <rustc_typeck[87f5f0e1435a037b]::check::fn_ctxt::FnCtxt>::check_expr_kind
  32:     0x7fb0969ee9eb - <rustc_typeck[87f5f0e1435a037b]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  33:     0x7fb096a48949 - <rustc_typeck[87f5f0e1435a037b]::check::fn_ctxt::FnCtxt>::check_expr_with_expectation
  34:     0x7fb0969efeb7 - <rustc_typeck[87f5f0e1435a037b]::check::fn_ctxt::FnCtxt>::check_return_expr
  35:     0x7fb096b228b1 - rustc_typeck[87f5f0e1435a037b]::check::check::check_fn
  36:     0x7fb096cd258e - <rustc_infer[c164df521a26f6]::infer::InferCtxtBuilder>::enter::<&rustc_middle[50fd58bc93f4b907]::ty::context::TypeckResults, <rustc_typeck[87f5f0e1435a037b]::check::inherited::InheritedBuilder>::enter<rustc_typeck[87f5f0e1435a037b]::check::typeck_with_fallback<rustc_typeck[87f5f0e1435a037b]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[50fd58bc93f4b907]::ty::context::TypeckResults>::{closure#0}>
  37:     0x7fb096b0e0de - <rustc_typeck[87f5f0e1435a037b]::check::inherited::InheritedBuilder>::enter::<rustc_typeck[87f5f0e1435a037b]::check::typeck_with_fallback<rustc_typeck[87f5f0e1435a037b]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[50fd58bc93f4b907]::ty::context::TypeckResults>
  38:     0x7fb096b9139e - rustc_typeck[87f5f0e1435a037b]::check::typeck
  39:     0x7fb097cdf986 - rustc_query_system[fc5aa2a14ec399c6]::query::plumbing::try_execute_query::<rustc_query_impl[3ed6196958178b69]::plumbing::QueryCtxt, rustc_query_system[fc5aa2a14ec399c6]::query::caches::DefaultCache<rustc_span[97139491182189eb]::def_id::LocalDefId, &rustc_middle[50fd58bc93f4b907]::ty::context::TypeckResults>>
  40:     0x7fb097dfa457 - rustc_query_system[fc5aa2a14ec399c6]::query::plumbing::get_query::<rustc_query_impl[3ed6196958178b69]::queries::typeck, rustc_query_impl[3ed6196958178b69]::plumbing::QueryCtxt>
  41:     0x7fb09794ebc4 - <rustc_query_impl[3ed6196958178b69]::Queries as rustc_middle[50fd58bc93f4b907]::ty::query::QueryEngine>::typeck
  42:     0x7fb096d8421b - <rustc_middle[50fd58bc93f4b907]::hir::map::Map>::par_body_owners::<rustc_typeck[87f5f0e1435a037b]::check::typeck_item_bodies::{closure#0}>
  43:     0x7fb096b962ad - rustc_typeck[87f5f0e1435a037b]::check::typeck_item_bodies
  44:     0x7fb097d2810a - rustc_query_system[fc5aa2a14ec399c6]::query::plumbing::try_execute_query::<rustc_query_impl[3ed6196958178b69]::plumbing::QueryCtxt, rustc_query_system[fc5aa2a14ec399c6]::query::caches::DefaultCache<(), ()>>
  45:     0x7fb097dbe095 - rustc_query_system[fc5aa2a14ec399c6]::query::plumbing::get_query::<rustc_query_impl[3ed6196958178b69]::queries::typeck_item_bodies, rustc_query_impl[3ed6196958178b69]::plumbing::QueryCtxt>
  46:     0x7fb09794e66e - <rustc_query_impl[3ed6196958178b69]::Queries as rustc_middle[50fd58bc93f4b907]::ty::query::QueryEngine>::typeck_item_bodies
  47:     0x7fb096d0a89a - <rustc_session[4211bfba78cbf5fb]::session::Session>::time::<(), rustc_typeck[87f5f0e1435a037b]::check_crate::{closure#7}>
  48:     0x7fb096c546e8 - rustc_typeck[87f5f0e1435a037b]::check_crate
  49:     0x7fb0962c0631 - rustc_interface[4d778361d2f35779]::passes::analysis
  50:     0x7fb097d1bade - rustc_query_system[fc5aa2a14ec399c6]::query::plumbing::try_execute_query::<rustc_query_impl[3ed6196958178b69]::plumbing::QueryCtxt, rustc_query_system[fc5aa2a14ec399c6]::query::caches::DefaultCache<(), core[71408e98247a1ba3]::result::Result<(), rustc_errors[b963431c8540357]::ErrorGuaranteed>>>
  51:     0x7fb097dfa832 - rustc_query_system[fc5aa2a14ec399c6]::query::plumbing::get_query::<rustc_query_impl[3ed6196958178b69]::queries::analysis, rustc_query_impl[3ed6196958178b69]::plumbing::QueryCtxt>
  52:     0x7fb097932ade - <rustc_query_impl[3ed6196958178b69]::Queries as rustc_middle[50fd58bc93f4b907]::ty::query::QueryEngine>::analysis
  53:     0x7fb0962130f4 - <rustc_interface[4d778361d2f35779]::passes::QueryContext>::enter::<rustc_driver[39665097b4ed30e4]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[71408e98247a1ba3]::result::Result<(), rustc_errors[b963431c8540357]::ErrorGuaranteed>>
  54:     0x7fb0961bf9c5 - <rustc_interface[4d778361d2f35779]::interface::Compiler>::enter::<rustc_driver[39665097b4ed30e4]::run_compiler::{closure#1}::{closure#2}, core[71408e98247a1ba3]::result::Result<core[71408e98247a1ba3]::option::Option<rustc_interface[4d778361d2f35779]::queries::Linker>, rustc_errors[b963431c8540357]::ErrorGuaranteed>>
  55:     0x7fb0961a3b1b - rustc_span[97139491182189eb]::with_source_map::<core[71408e98247a1ba3]::result::Result<(), rustc_errors[b963431c8540357]::ErrorGuaranteed>, rustc_interface[4d778361d2f35779]::interface::create_compiler_and_run<core[71408e98247a1ba3]::result::Result<(), rustc_errors[b963431c8540357]::ErrorGuaranteed>, rustc_driver[39665097b4ed30e4]::run_compiler::{closure#1}>::{closure#1}>
  56:     0x7fb0961c0b94 - <scoped_tls[4ba69821ac6c74bd]::ScopedKey<rustc_span[97139491182189eb]::SessionGlobals>>::set::<rustc_interface[4d778361d2f35779]::interface::run_compiler<core[71408e98247a1ba3]::result::Result<(), rustc_errors[b963431c8540357]::ErrorGuaranteed>, rustc_driver[39665097b4ed30e4]::run_compiler::{closure#1}>::{closure#0}, core[71408e98247a1ba3]::result::Result<(), rustc_errors[b963431c8540357]::ErrorGuaranteed>>
  57:     0x7fb09621eb89 - std[aa180c2ff93fe481]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[4d778361d2f35779]::util::run_in_thread_pool_with_globals<rustc_interface[4d778361d2f35779]::interface::run_compiler<core[71408e98247a1ba3]::result::Result<(), rustc_errors[b963431c8540357]::ErrorGuaranteed>, rustc_driver[39665097b4ed30e4]::run_compiler::{closure#1}>::{closure#0}, core[71408e98247a1ba3]::result::Result<(), rustc_errors[b963431c8540357]::ErrorGuaranteed>>::{closure#0}, core[71408e98247a1ba3]::result::Result<(), rustc_errors[b963431c8540357]::ErrorGuaranteed>>
  58:     0x7fb096216fd9 - <<std[aa180c2ff93fe481]::thread::Builder>::spawn_unchecked_<rustc_interface[4d778361d2f35779]::util::run_in_thread_pool_with_globals<rustc_interface[4d778361d2f35779]::interface::run_compiler<core[71408e98247a1ba3]::result::Result<(), rustc_errors[b963431c8540357]::ErrorGuaranteed>, rustc_driver[39665097b4ed30e4]::run_compiler::{closure#1}>::{closure#0}, core[71408e98247a1ba3]::result::Result<(), rustc_errors[b963431c8540357]::ErrorGuaranteed>>::{closure#0}, core[71408e98247a1ba3]::result::Result<(), rustc_errors[b963431c8540357]::ErrorGuaranteed>>::{closure#1} as core[71408e98247a1ba3]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  59:     0x7fb0956878b3 - std::sys::unix::thread::Thread::new::thread_start::h09854d189bc130f5
  60:     0x7fb08fbda609 - start_thread
  61:     0x7fb0954ed133 - clone
  62:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.63.0-nightly (13df7102b 2022-06-15) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `main`
#1 [typeck_item_bodies] type-checking all item bodies
#2 [analysis] running analysis passes on this crate
------------------------------------------



