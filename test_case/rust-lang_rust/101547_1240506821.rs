plain
.....................i..................i............................................... 264/13492
........................................................................................ 352/13492
........................................................................................ 440/13492
........................................................................................ 528/13492
.....................................................................F...............F.F 616/13492
F..F.........................................................F..F....................... 704/13492
........................................................................i............... 880/13492
........................................................................................ 968/13492
........................................................................................ 1056/13492
........................................................................................ 1144/13492
---
........................................................................................ 6688/13492
...........................................i............................................ 6776/13492
............ii.ii........i....i......................................................... 6864/13492
......i................................................................................. 6952/13492
.............F...F...................................................i...i.............. 7040/13492
...............................i........................................................ 7216/13492
....................................................i................................... 7304/13492
........................................................................................ 7392/13492
........................................................................................ 7480/13492
---
---- [ui] src/test/ui/async-await/async-fn-send-uses-nonsend.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/async-fn-send-uses-nonsend.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-fn-send-uses-nonsend" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "--crate-type" "lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-fn-send-uses-nonsend/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: compiler/rustc_mir_transform/src/generator.rs:1255:14: unexpected generator type [&str; 3]
   |
LL |     println!("{:?} {:?}", non_send(), non_sync());
   |              ^^^^^^^^^^^


thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1398:9
stack backtrace:
   0:     0x7f5bf371e79e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha266d3b4421f7468
   1:     0x7f5bf3787948 - core::fmt::write::hcf80770c462d0907
   2:     0x7f5bf370f0a1 - std::io::Write::write_fmt::h6067fcde5a266ae4
   3:     0x7f5bf37217ae - std::panicking::default_hook::{{closure}}::h8227521fd9887c61
   4:     0x7f5bf3721477 - std::panicking::default_hook::h4f005cd45a35ad85
   5:     0x7f5bf40e4b34 - rustc_driver[511eab029b20559e]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f5bf3721f61 - std::panicking::rust_panic_with_hook::h7ec5ac7188043c5a
   7:     0x7f5bf46a2be3 - std[24b66dceb5e0a55f]::panicking::begin_panic::<rustc_errors[ce972c85f60e45cf]::ExplicitBug>::{closure#0}
   8:     0x7f5bf469f386 - std[24b66dceb5e0a55f]::sys_common::backtrace::__rust_end_short_backtrace::<std[24b66dceb5e0a55f]::panicking::begin_panic<rustc_errors[ce972c85f60e45cf]::ExplicitBug>::{closure#0}, !>
   9:     0x7f5bf3f1a806 - std[24b66dceb5e0a55f]::panicking::begin_panic::<rustc_errors[ce972c85f60e45cf]::ExplicitBug>
  10:     0x7f5bf474abe6 - std[24b66dceb5e0a55f]::panic::panic_any::<rustc_errors[ce972c85f60e45cf]::ExplicitBug>
  11:     0x7f5bf474ab83 - <rustc_errors[ce972c85f60e45cf]::HandlerInner>::span_bug::<rustc_span[c86d7b72459e274]::span_encoding::Span, &alloc[e77f571ce596fd94]::string::String>
  12:     0x7f5bf474a8c0 - <rustc_errors[ce972c85f60e45cf]::Handler>::span_bug::<rustc_span[c86d7b72459e274]::span_encoding::Span, &alloc[e77f571ce596fd94]::string::String>
  13:     0x7f5bf4783ce2 - rustc_middle[2b0e30f1a844f333]::ty::context::tls::with_context_opt::<rustc_middle[2b0e30f1a844f333]::ty::context::tls::with_opt<rustc_middle[2b0e30f1a844f333]::util::bug::opt_span_bug_fmt<rustc_span[c86d7b72459e274]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  14:     0x7f5bf4781d49 - rustc_middle[2b0e30f1a844f333]::util::bug::opt_span_bug_fmt::<rustc_span[c86d7b72459e274]::span_encoding::Span>
  15:     0x7f5bf3f1dfca - rustc_middle[2b0e30f1a844f333]::util::bug::span_bug_fmt::<rustc_span[c86d7b72459e274]::span_encoding::Span>
  16:     0x7f5bf45b57d8 - rustc_mir_transform[3779cdcb05977e58]::generator::state_transform_body_for_generator
  17:     0x7f5bf45c1806 - rustc_mir_transform[3779cdcb05977e58]::run_analysis_to_runtime_passes
  18:     0x7f5bf45c350e - rustc_mir_transform[3779cdcb05977e58]::promoted_mir
  19:     0x7f5bf45a850a - <rustc_mir_transform[3779cdcb05977e58]::provide::{closure#3} as core[b6c6cfde078dfe44]::ops::function::FnOnce<(rustc_middle[2b0e30f1a844f333]::ty::context::TyCtxt, rustc_span[c86d7b72459e274]::def_id::DefId)>>::call_once
  20:     0x7f5bf5cdd413 - rustc_query_system[aa73f8cecf880083]::query::plumbing::try_execute_query::<rustc_query_impl[f78d6cf74d1d25b1]::plumbing::QueryCtxt, rustc_query_system[aa73f8cecf880083]::query::caches::DefaultCache<rustc_span[c86d7b72459e274]::def_id::DefId, &rustc_index[7796512a5b8187f]::vec::IndexVec<rustc_middle[2b0e30f1a844f333]::mir::Promoted, rustc_middle[2b0e30f1a844f333]::mir::Body>>>
  21:     0x7f5bf5d8c8c0 - rustc_query_system[aa73f8cecf880083]::query::plumbing::get_query::<rustc_query_impl[f78d6cf74d1d25b1]::queries::promoted_mir, rustc_query_impl[f78d6cf74d1d25b1]::plumbing::QueryCtxt>
  22:     0x7f5bf5bc7665 - <rustc_query_impl[f78d6cf74d1d25b1]::Queries as rustc_middle[2b0e30f1a844f333]::ty::query::QueryEngine>::promoted_mir
  23:     0x7f5bf6194a57 - <rustc_metadata[25915eeff25cab22]::rmeta::encoder::EncodeContext>::encode_crate_root
  24:     0x7f5bf61a90ed - rustc_metadata[25915eeff25cab22]::rmeta::encoder::encode_metadata_impl
  25:     0x7f5bf62a2410 - rustc_data_structures[a1b3e7ae5b8f85cd]::sync::join::<rustc_metadata[25915eeff25cab22]::rmeta::encoder::encode_metadata::{closure#0}, rustc_metadata[25915eeff25cab22]::rmeta::encoder::encode_metadata::{closure#1}, (), ()>
  26:     0x7f5bf61a8795 - rustc_metadata[25915eeff25cab22]::rmeta::encoder::encode_metadata
  27:     0x7f5bf61db052 - rustc_metadata[25915eeff25cab22]::fs::encode_and_write_metadata
  28:     0x7f5bf4222a6d - <rustc_interface[6492f6541a5bdae5]::passes::QueryContext>::enter::<<rustc_interface[6492f6541a5bdae5]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[b6c6cfde078dfe44]::result::Result<alloc[e77f571ce596fd94]::boxed::Box<dyn core[b6c6cfde078dfe44]::any::Any>, rustc_errors[ce972c85f60e45cf]::ErrorGuaranteed>>
  29:     0x7f5bf42116ed - <rustc_interface[6492f6541a5bdae5]::queries::Queries>::ongoing_codegen
  30:     0x7f5bf40e983e - rustc_interface[6492f6541a5bdae5]::interface::create_compiler_and_run::<core[b6c6cfde078dfe44]::result::Result<(), rustc_errors[ce972c85f60e45cf]::ErrorGuaranteed>, rustc_driver[511eab029b20559e]::run_compiler::{closure#1}>
  31:     0x7f5bf40d0102 - <scoped_tls[ecdf0725d9878678]::ScopedKey<rustc_span[c86d7b72459e274]::SessionGlobals>>::set::<rustc_interface[6492f6541a5bdae5]::interface::run_compiler<core[b6c6cfde078dfe44]::result::Result<(), rustc_errors[ce972c85f60e45cf]::ErrorGuaranteed>, rustc_driver[511eab029b20559e]::run_compiler::{closure#1}>::{closure#0}, core[b6c6cfde078dfe44]::result::Result<(), rustc_errors[ce972c85f60e45cf]::ErrorGuaranteed>>
  32:     0x7f5bf414d65f - std[24b66dceb5e0a55f]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[6492f6541a5bdae5]::util::run_in_thread_pool_with_globals<rustc_interface[6492f6541a5bdae5]::interface::run_compiler<core[b6c6cfde078dfe44]::result::Result<(), rustc_errors[ce972c85f60e45cf]::ErrorGuaranteed>, rustc_driver[511eab029b20559e]::run_compiler::{closure#1}>::{closure#0}, core[b6c6cfde078dfe44]::result::Result<(), rustc_errors[ce972c85f60e45cf]::ErrorGuaranteed>>::{closure#0}, core[b6c6cfde078dfe44]::result::Result<(), rustc_errors[ce972c85f60e45cf]::ErrorGuaranteed>>
  33:     0x7f5bf40e843e - std[24b66dceb5e0a55f]::panic::catch_unwind::<core[b6c6cfde078dfe44]::panic::unwind_safe::AssertUnwindSafe<<std[24b66dceb5e0a55f]::thread::Builder>::spawn_unchecked_<rustc_interface[6492f6541a5bdae5]::util::run_in_thread_pool_with_globals<rustc_interface[6492f6541a5bdae5]::interface::run_compiler<core[b6c6cfde078dfe44]::result::Result<(), rustc_errors[ce972c85f60e45cf]::ErrorGuaranteed>, rustc_driver[511eab029b20559e]::run_compiler::{closure#1}>::{closure#0}, core[b6c6cfde078dfe44]::result::Result<(), rustc_errors[ce972c85f60e45cf]::ErrorGuaranteed>>::{closure#0}, core[b6c6cfde078dfe44]::result::Result<(), rustc_errors[ce972c85f60e45cf]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[b6c6cfde078dfe44]::result::Result<(), rustc_errors[ce972c85f60e45cf]::ErrorGuaranteed>>
  34:     0x7f5bf4150ae2 - <<std[24b66dceb5e0a55f]::thread::Builder>::spawn_unchecked_<rustc_interface[6492f6541a5bdae5]::util::run_in_thread_pool_with_globals<rustc_interface[6492f6541a5bdae5]::interface::run_compiler<core[b6c6cfde078dfe44]::result::Result<(), rustc_errors[ce972c85f60e45cf]::ErrorGuaranteed>, rustc_driver[511eab029b20559e]::run_compiler::{closure#1}>::{closure#0}, core[b6c6cfde078dfe44]::result::Result<(), rustc_errors[ce972c85f60e45cf]::ErrorGuaranteed>>::{closure#0}, core[b6c6cfde078dfe44]::result::Result<(), rustc_errors[ce972c85f60e45cf]::ErrorGuaranteed>>::{closure#1} as core[b6c6cfde078dfe44]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  35:     0x7f5bf372edc5 - std::sys::unix::thread::Thread::new::thread_start::h7617fc35876b6b20
  36:     0x7f5bf34cab43 - <unknown>
  37:     0x7f5bf355ca00 - <unknown>
  38:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.65.0-nightly (0599b7f65 2022-09-08) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 --crate-type lib
query stack during panic:
#0 [promoted_mir] optimizing promoted MIR for `still_send::{closure#0}`
end of query stack
error: aborting due to previous error
error: aborting due to previous error
------------------------------------------


---- [ui] src/test/ui/async-await/async-await.rs#nomiropt stdout ----

error in revision `nomiropt`: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/async-await.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "nomiropt" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-await.nomiropt/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "mir-opt-level=0" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-await.nomiropt/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: compiler/rustc_mir_transform/src/generator.rs:1255:14: unexpected generator type [closure@/checkout/src/test/ui/async-await/async-await.rs:213:9: 213:12]
   |
   |
LL | /         |x| {
LL | |             async move {
LL | |                 async_fn_multiple_args_named_lifetime(x, x).await
LL | |             }
LL | |         },

thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1398:9
stack backtrace:
   0:     0x7f951595179e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha266d3b4421f7468
   0:     0x7f951595179e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha266d3b4421f7468
   1:     0x7f95159ba948 - core::fmt::write::hcf80770c462d0907
   2:     0x7f95159420a1 - std::io::Write::write_fmt::h6067fcde5a266ae4
   3:     0x7f95159547ae - std::panicking::default_hook::{{closure}}::h8227521fd9887c61
   4:     0x7f9515954477 - std::panicking::default_hook::h4f005cd45a35ad85
   5:     0x7f9516317b34 - rustc_driver[511eab029b20559e]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f9515954f61 - std::panicking::rust_panic_with_hook::h7ec5ac7188043c5a
   7:     0x7f95168d5be3 - std[24b66dceb5e0a55f]::panicking::begin_panic::<rustc_errors[ce972c85f60e45cf]::ExplicitBug>::{closure#0}
   8:     0x7f95168d2386 - std[24b66dceb5e0a55f]::sys_common::backtrace::__rust_end_short_backtrace::<std[24b66dceb5e0a55f]::panicking::begin_panic<rustc_errors[ce972c85f60e45cf]::ExplicitBug>::{closure#0}, !>
   9:     0x7f951614d806 - std[24b66dceb5e0a55f]::panicking::begin_panic::<rustc_errors[ce972c85f60e45cf]::ExplicitBug>
  10:     0x7f951697dbe6 - std[24b66dceb5e0a55f]::panic::panic_any::<rustc_errors[ce972c85f60e45cf]::ExplicitBug>
  11:     0x7f951697db83 - <rustc_errors[ce972c85f60e45cf]::HandlerInner>::span_bug::<rustc_span[c86d7b72459e274]::span_encoding::Span, &alloc[e77f571ce596fd94]::string::String>
  12:     0x7f951697d8c0 - <rustc_errors[ce972c85f60e45cf]::Handler>::span_bug::<rustc_span[c86d7b72459e274]::span_encoding::Span, &alloc[e77f571ce596fd94]::string::String>
  13:     0x7f95169b6ce2 - rustc_middle[2b0e30f1a844f333]::ty::context::tls::with_context_opt::<rustc_middle[2b0e30f1a844f333]::ty::context::tls::with_opt<rustc_middle[2b0e30f1a844f333]::util::bug::opt_span_bug_fmt<rustc_span[c86d7b72459e274]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  14:     0x7f95169b4d49 - rustc_middle[2b0e30f1a844f333]::util::bug::opt_span_bug_fmt::<rustc_span[c86d7b72459e274]::span_encoding::Span>
  15:     0x7f9516150fca - rustc_middle[2b0e30f1a844f333]::util::bug::span_bug_fmt::<rustc_span[c86d7b72459e274]::span_encoding::Span>
  16:     0x7f95167e87d8 - rustc_mir_transform[3779cdcb05977e58]::generator::state_transform_body_for_generator
  17:     0x7f95167f4806 - rustc_mir_transform[3779cdcb05977e58]::run_analysis_to_runtime_passes
  18:     0x7f95167f650e - rustc_mir_transform[3779cdcb05977e58]::promoted_mir
  19:     0x7f95167db50a - <rustc_mir_transform[3779cdcb05977e58]::provide::{closure#3} as core[b6c6cfde078dfe44]::ops::function::FnOnce<(rustc_middle[2b0e30f1a844f333]::ty::context::TyCtxt, rustc_span[c86d7b72459e274]::def_id::DefId)>>::call_once
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
  20:     0x7f9517f10413 - rustc_query_system[aa73f8cecf880083]::query::plumbing::try_execute_query::<rustc_query_impl[f78d6cf74d1d25b1]::plumbing::QueryCtxt, rustc_query_system[aa73f8cecf880083]::query::caches::DefaultCache<rustc_span[c86d7b72459e274]::def_id::DefId, &rustc_index[7796512a5b8187f]::vec::IndexVec<rustc_middle[2b0e30f1a844f333]::mir::Promoted, rustc_middle[2b0e30f1a844f333]::mir::Body>>>
  21:     0x7f9517fbf8c0 - rustc_query_system[aa73f8cecf880083]::query::plumbing::get_query::<rustc_query_impl[f78d6cf74d1d25b1]::queries::promoted_mir, rustc_query_impl[f78d6cf74d1d25b1]::plumbing::QueryCtxt>
  22:     0x7f9517dfa665 - <rustc_query_impl[f78d6cf74d1d25b1]::Queries as rustc_middle[2b0e30f1a844f333]::ty::query::QueryEngine>::promoted_mir
  23:     0x7f95178b78dd - <rustc_const_eval[c08325d25416a93]::interpret::eval_context::InterpCx<rustc_const_eval[c08325d25416a93]::const_eval::machine::CompileTimeInterpreter>>::load_mir
  24:     0x7f95179d74c3 - rustc_const_eval[c08325d25416a93]::const_eval::eval_queries::eval_to_allocation_raw_provider
  25:     0x7f9517fe4bc5 - rustc_query_system[aa73f8cecf880083]::query::plumbing::get_query::<rustc_query_impl[f78d6cf74d1d25b1]::queries::eval_to_allocation_raw, rustc_query_impl[f78d6cf74d1d25b1]::plumbing::QueryCtxt>
  26:     0x7f9517e1a7ec - <rustc_query_impl[f78d6cf74d1d25b1]::Queries as rustc_middle[2b0e30f1a844f333]::ty::query::QueryEngine>::eval_to_allocation_raw
  27:     0x7f95179d58b5 - rustc_const_eval[c08325d25416a93]::const_eval::eval_queries::eval_to_const_value_raw_provider
  28:     0x7f9517fec04b - rustc_query_system[aa73f8cecf880083]::query::plumbing::get_query::<rustc_query_impl[f78d6cf74d1d25b1]::queries::eval_to_const_value_raw, rustc_query_impl[f78d6cf74d1d25b1]::plumbing::QueryCtxt>
  29:     0x7f9517e1afac - <rustc_query_impl[f78d6cf74d1d25b1]::Queries as rustc_middle[2b0e30f1a844f333]::ty::query::QueryEngine>::eval_to_const_value_raw
  30:     0x7f95179d555c - rustc_const_eval[c08325d25416a93]::const_eval::eval_queries::eval_to_const_value_raw_provider
  31:     0x7f9517fec04b - rustc_query_system[aa73f8cecf880083]::query::plumbing::get_query::<rustc_query_impl[f78d6cf74d1d25b1]::queries::eval_to_const_value_raw, rustc_query_impl[f78d6cf74d1d25b1]::plumbing::QueryCtxt>
  32:     0x7f9517e1afac - <rustc_query_impl[f78d6cf74d1d25b1]::Queries as rustc_middle[2b0e30f1a844f333]::ty::query::QueryEngine>::eval_to_const_value_raw
  33:     0x7f9518e53bf4 - <rustc_middle[2b0e30f1a844f333]::ty::context::TyCtxt>::const_eval_global_id
  34:     0x7f9518e52aec - <rustc_middle[2b0e30f1a844f333]::ty::context::TyCtxt>::const_eval_resolve
  35:     0x7f9516775424 - <rustc_monomorphize[65cc958de9c9010]::collector::MirNeighborCollector as rustc_middle[2b0e30f1a844f333]::mir::visit::Visitor>::visit_constant
  36:     0x7f951676e19c - <rustc_monomorphize[65cc958de9c9010]::collector::MirNeighborCollector as rustc_middle[2b0e30f1a844f333]::mir::visit::Visitor>::visit_operand
  37:     0x7f951676c3d6 - <rustc_monomorphize[65cc958de9c9010]::collector::MirNeighborCollector as rustc_middle[2b0e30f1a844f333]::mir::visit::Visitor>::visit_rvalue
  38:     0x7f9516778387 - rustc_monomorphize[65cc958de9c9010]::collector::collect_neighbours
  39:     0x7f9516773a09 - rustc_monomorphize[65cc958de9c9010]::collector::collect_items_rec
  40:     0x7f9516774911 - rustc_monomorphize[65cc958de9c9010]::collector::collect_items_rec
  41:     0x7f9516774911 - rustc_monomorphize[65cc958de9c9010]::collector::collect_items_rec
  42:     0x7f9516774911 - rustc_monomorphize[65cc958de9c9010]::collector::collect_items_rec
  43:     0x7f951677b144 - <core[b6c6cfde078dfe44]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[a1b3e7ae5b8f85cd]::sync::par_for_each_in<alloc[e77f571ce596fd94]::vec::Vec<rustc_middle[2b0e30f1a844f333]::mir::mono::MonoItem>, rustc_monomorphize[65cc958de9c9010]::collector::collect_crate_mono_items::{closure#1}::{closure#0}>::{closure#0}::{closure#0}> as core[b6c6cfde078dfe44]::ops::function::FnOnce<()>>::call_once
  44:     0x7f95167b87d5 - std[24b66dceb5e0a55f]::panicking::try::<(), core[b6c6cfde078dfe44]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[a1b3e7ae5b8f85cd]::sync::par_for_each_in<alloc[e77f571ce596fd94]::vec::Vec<rustc_middle[2b0e30f1a844f333]::mir::mono::MonoItem>, rustc_monomorphize[65cc958de9c9010]::collector::collect_crate_mono_items::{closure#1}::{closure#0}>::{closure#0}::{closure#0}>>
  45:     0x7f951678f5e3 - <rustc_session[70efa93bdab11654]::session::Session>::time::<(), rustc_monomorphize[65cc958de9c9010]::collector::collect_crate_mono_items::{closure#1}>
  46:     0x7f95167708ff - rustc_monomorphize[65cc958de9c9010]::collector::collect_crate_mono_items
  47:     0x7f951678427a - rustc_monomorphize[65cc958de9c9010]::partitioning::collect_and_partition_mono_items
  48:     0x7f9517f38ef2 - rustc_query_system[aa73f8cecf880083]::query::plumbing::try_execute_query::<rustc_query_impl[f78d6cf74d1d25b1]::plumbing::QueryCtxt, rustc_query_system[aa73f8cecf880083]::query::caches::DefaultCache<(), (&std[24b66dceb5e0a55f]::collections::hash::set::HashSet<rustc_span[c86d7b72459e274]::def_id::DefId, core[b6c6cfde078dfe44]::hash::BuildHasherDefault<rustc_hash[cffe43c69f8f1eff]::FxHasher>>, &[rustc_middle[2b0e30f1a844f333]::mir::mono::CodegenUnit])>>
  49:     0x7f9518005785 - rustc_query_system[aa73f8cecf880083]::query::plumbing::get_query::<rustc_query_impl[f78d6cf74d1d25b1]::queries::collect_and_partition_mono_items, rustc_query_impl[f78d6cf74d1d25b1]::plumbing::QueryCtxt>
  50:     0x7f9517e57148 - <rustc_query_impl[f78d6cf74d1d25b1]::Queries as rustc_middle[2b0e30f1a844f333]::ty::query::QueryEngine>::collect_and_partition_mono_items
  51:     0x7f951657dd63 - rustc_codegen_ssa[d2586b3b6cdc69b0]::base::codegen_crate::<rustc_codegen_llvm[8276a0420c0bc8c4]::LlvmCodegenBackend>
  52:     0x7f951664ebfd - <rustc_codegen_llvm[8276a0420c0bc8c4]::LlvmCodegenBackend as rustc_codegen_ssa[d2586b3b6cdc69b0]::traits::backend::CodegenBackend>::codegen_crate
  53:     0x7f951648cadb - <rustc_session[70efa93bdab11654]::session::Session>::time::<alloc[e77f571ce596fd94]::boxed::Box<dyn core[b6c6cfde078dfe44]::any::Any>, rustc_interface[6492f6541a5bdae5]::passes::start_codegen::{closure#0}>
  54:     0x7f9516455ae5 - <rustc_interface[6492f6541a5bdae5]::passes::QueryContext>::enter::<<rustc_interface[6492f6541a5bdae5]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[b6c6cfde078dfe44]::result::Result<alloc[e77f571ce596fd94]::boxed::Box<dyn core[b6c6cfde078dfe44]::any::Any>, rustc_errors[ce972c85f60e45cf]::ErrorGuaranteed>>
  55:     0x7f95164446ed - <rustc_interface[6492f6541a5bdae5]::queries::Queries>::ongoing_codegen
  56:     0x7f951631c83e - rustc_interface[6492f6541a5bdae5]::interface::create_compiler_and_run::<core[b6c6cfde078dfe44]::result::Result<(), rustc_errors[ce972c85f60e45cf]::ErrorGuaranteed>, rustc_driver[511eab029b20559e]::run_compiler::{closure#1}>
  57:     0x7f9516303102 - <scoped_tls[ecdf0725d9878678]::ScopedKey<rustc_span[c86d7b72459e274]::SessionGlobals>>::set::<rustc_interface[6492f6541a5bdae5]::interface::run_compiler<core[b6c6cfde078dfe44]::result::Result<(), rustc_errors[ce972c85f60e45cf]::ErrorGuaranteed>, rustc_driver[511eab029b20559e]::run_compiler::{closure#1}>::{closure#0}, core[b6c6cfde078dfe44]::result::Result<(), rustc_errors[ce972c85f60e45cf]::ErrorGuaranteed>>
  58:     0x7f951638065f - std[24b66dceb5e0a55f]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[6492f6541a5bdae5]::util::run_in_thread_pool_with_globals<rustc_interface[6492f6541a5bdae5]::interface::run_compiler<core[b6c6cfde078dfe44]::result::Result<(), rustc_errors[ce972c85f60e45cf]::ErrorGuaranteed>, rustc_driver[511eab029b20559e]::run_compiler::{closure#1}>::{closure#0}, core[b6c6cfde078dfe44]::result::Result<(), rustc_errors[ce972c85f60e45cf]::ErrorGuaranteed>>::{closure#0}, core[b6c6cfde078dfe44]::result::Result<(), rustc_errors[ce972c85f60e45cf]::ErrorGuaranteed>>
  59:     0x7f951631b43e - std[24b66dceb5e0a55f]::panic::catch_unwind::<core[b6c6cfde078dfe44]::panic::unwind_safe::AssertUnwindSafe<<std[24b66dceb5e0a55f]::thread::Builder>::spawn_unchecked_<rustc_interface[6492f6541a5bdae5]::util::run_in_thread_pool_with_globals<rustc_interface[6492f6541a5bdae5]::interface::run_compiler<core[b6c6cfde078dfe44]::result::Result<(), rustc_errors[ce972c85f60e45cf]::ErrorGuaranteed>, rustc_driver[511eab029b20559e]::run_compiler::{closure#1}>::{closure#0}, core[b6c6cfde078dfe44]::result::Result<(), rustc_errors[ce972c85f60e45cf]::ErrorGuaranteed>>::{closure#0}, core[b6c6cfde078dfe44]::result::Result<(), rustc_errors[ce972c85f60e45cf]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[b6c6cfde078dfe44]::result::Result<(), rustc_errors[ce972c85f60e45cf]::ErrorGuaranteed>>
  60:     0x7f9516383ae2 - <<std[24b66dceb5e0a55f]::thread::Builder>::spawn_unchecked_<rustc_interface[6492f6541a5bdae5]::util::run_in_thread_pool_with_globals<rustc_interface[6492f6541a5bdae5]::interface::run_compiler<core[b6c6cfde078dfe44]::result::Result<(), rustc_errors[ce972c85f60e45cf]::ErrorGuaranteed>, rustc_driver[511eab029b20559e]::run_compiler::{closure#1}>::{closure#0}, core[b6c6cfde078dfe44]::result::Result<(), rustc_errors[ce972c85f60e45cf]::ErrorGuaranteed>>::{closure#0}, core[b6c6cfde078dfe44]::result::Result<(), rustc_errors[ce972c85f60e45cf]::ErrorGuaranteed>>::{closure#1} as core[b6c6cfde078dfe44]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  61:     0x7f9515961dc5 - std::sys::unix::thread::Thread::new::thread_start::h7617fc35876b6b20
  62:     0x7f95156fdb43 - <unknown>
  63:     0x7f951578fa00 - <unknown>
  64:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.65.0-nightly (0599b7f65 2022-09-08) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 -Z mir-opt-level=0
query stack during panic:
#0 [promoted_mir] optimizing promoted MIR for `main::{closure#6}::{closure#0}`
#0 [promoted_mir] optimizing promoted MIR for `main::{closure#6}::{closure#0}`
#1 [eval_to_allocation_raw] const-evaluating + checking `main::{closure#6}::{closure#0}::promoted[0]`
#2 [eval_to_const_value_raw] simplifying constant for the type system `main::{closure#6}::{closure#0}::promoted[0]`
#3 [eval_to_const_value_raw] simplifying constant for the type system `main::{closure#6}::{closure#0}::promoted[0]`
#4 [collect_and_partition_mono_items] collect_and_partition_mono_items
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/async-await/async-await.rs#thirunsafeck stdout ----

error in revision `thirunsafeck`: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/async-await.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "thirunsafeck" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-await.thirunsafeck/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zthir-unsafeck" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-await.thirunsafeck/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: compiler/rustc_mir_transform/src/generator.rs:1255:14: unexpected generator type [closure@/checkout/src/test/ui/async-await/async-await.rs:213:9: 213:12]
   |
   |
LL | /         |x| {
LL | |             async move {
LL | |                 async_fn_multiple_args_named_lifetime(x, x).await
LL | |             }
LL | |         },

thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1398:9
stack backtrace:
   0:     0x7f1faed1179e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha266d3b4421f7468
   0:     0x7f1faed1179e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha266d3b4421f7468
   1:     0x7f1faed7a948 - core::fmt::write::hcf80770c462d0907
   2:     0x7f1faed020a1 - std::io::Write::write_fmt::h6067fcde5a266ae4
   3:     0x7f1faed147ae - std::panicking::default_hook::{{closure}}::h8227521fd9887c61
   4:     0x7f1faed14477 - std::panicking::default_hook::h4f005cd45a35ad85
   5:     0x7f1faf6d7b34 - rustc_driver[511eab029b20559e]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f1faed14f61 - std::panicking::rust_panic_with_hook::h7ec5ac7188043c5a
   7:     0x7f1fafc95be3 - std[24b66dceb5e0a55f]::panicking::begin_panic::<rustc_errors[ce972c85f60e45cf]::ExplicitBug>::{closure#0}
   8:     0x7f1fafc92386 - std[24b66dceb5e0a55f]::sys_common::backtrace::__rust_end_short_backtrace::<std[24b66dceb5e0a55f]::panicking::begin_panic<rustc_errors[ce972c85f60e45cf]::ExplicitBug>::{closure#0}, !>
   9:     0x7f1faf50d806 - std[24b66dceb5e0a55f]::panicking::begin_panic::<rustc_errors[ce972c85f60e45cf]::ExplicitBug>
  10:     0x7f1fafd3dbe6 - std[24b66dceb5e0a55f]::panic::panic_any::<rustc_errors[ce972c85f60e45cf]::ExplicitBug>
  11:     0x7f1fafd3db83 - <rustc_errors[ce972c85f60e45cf]::HandlerInner>::span_bug::<rustc_span[c86d7b72459e274]::span_encoding::Span, &alloc[e77f571ce596fd94]::string::String>
  12:     0x7f1fafd3d8c0 - <rustc_errors[ce972c85f60e45cf]::Handler>::span_bug::<rustc_span[c86d7b72459e274]::span_encoding::Span, &alloc[e77f571ce596fd94]::string::String>
  13:     0x7f1fafd76ce2 - rustc_middle[2b0e30f1a844f333]::ty::context::tls::with_context_opt::<rustc_middle[2b0e30f1a844f333]::ty::context::tls::with_opt<rustc_middle[2b0e30f1a844f333]::util::bug::opt_span_bug_fmt<rustc_span[c86d7b72459e274]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  14:     0x7f1fafd74d49 - rustc_middle[2b0e30f1a844f333]::util::bug::opt_span_bug_fmt::<rustc_span[c86d7b72459e274]::span_encoding::Span>
  15:     0x7f1faf510fca - rustc_middle[2b0e30f1a844f333]::util::bug::span_bug_fmt::<rustc_span[c86d7b72459e274]::span_encoding::Span>
  16:     0x7f1fafba87d8 - rustc_mir_transform[3779cdcb05977e58]::generator::state_transform_body_for_generator
  17:     0x7f1fafbb4806 - rustc_mir_transform[3779cdcb05977e58]::run_analysis_to_runtime_passes
  18:     0x7f1fafbb650e - rustc_mir_transform[3779cdcb05977e58]::promoted_mir
  19:     0x7f1fafb9b50a - <rustc_mir_transform[3779cdcb05977e58]::provide::{closure#3} as core[b6c6cfde078dfe44]::ops::function::FnOnce<(rustc_middle[2b0e30f1a844f333]::ty::context::TyCtxt, rustc_span[c86d7b72459e274]::def_id::DefId)>>::call_once
  20:     0x7f1fb12d0413 - rustc_query_system[aa73f8cecf880083]::query::plumbing::try_execute_query::<rustc_query_impl[f78d6cf74d1d25b1]::plumbing::QueryCtxt, rustc_query_system[aa73f8cecf880083]::query::caches::DefaultCache<rustc_span[c86d7b72459e274]::def_id::DefId, &rustc_index[7796512a5b8187f]::vec::IndexVec<rustc_middle[2b0e30f1a844f333]::mir::Promoted, rustc_middle[2b0e30f1a844f333]::mir::Body>>>
  21:     0x7f1fb137f8c0 - rustc_query_system[aa73f8cecf880083]::query::plumbing::get_query::<rustc_query_impl[f78d6cf74d1d25b1]::queries::promoted_mir, rustc_query_impl[f78d6cf74d1d25b1]::plumbing::QueryCtxt>
  22:     0x7f1fb11ba665 - <rustc_query_impl[f78d6cf74d1d25b1]::Queries as rustc_middle[2b0e30f1a844f333]::ty::query::QueryEngine>::promoted_mir
  23:     0x7f1fb0c778dd - <rustc_const_eval[c08325d25416a93]::interpret::eval_context::InterpCx<rustc_const_eval[c08325d25416a93]::const_eval::machine::CompileTimeInterpreter>>::load_mir
  24:     0x7f1fb0d974c3 - rustc_const_eval[c08325d25416a93]::const_eval::eval_queries::eval_to_allocation_raw_provider
  25:     0x7f1fb13a4bc5 - rustc_query_system[aa73f8cecf880083]::query::plumbing::get_query::<rustc_query_impl[f78d6cf74d1d25b1]::queries::eval_to_allocation_raw, rustc_query_impl[f78d6cf74d1d25b1]::plumbing::QueryCtxt>
  26:     0x7f1fb11da7ec - <rustc_query_impl[f78d6cf74d1d25b1]::Queries as rustc_middle[2b0e30f1a844f333]::ty::query::QueryEngine>::eval_to_allocation_raw
  27:     0x7f1fb0d958b5 - rustc_const_eval[c08325d25416a93]::const_eval::eval_queries::eval_to_const_value_raw_provider
  28:     0x7f1fb13ac04b - rustc_query_system[aa73f8cecf880083]::query::plumbing::get_query::<rustc_query_impl[f78d6cf74d1d25b1]::queries::eval_to_const_value_raw, rustc_query_impl[f78d6cf74d1d25b1]::plumbing::QueryCtxt>
  29:     0x7f1fb11dafac - <rustc_query_impl[f78d6cf74d1d25b1]::Queries as rustc_middle[2b0e30f1a844f333]::ty::query::QueryEngine>::eval_to_const_value_raw
  30:     0x7f1fb0d9555c - rustc_const_eval[c08325d25416a93]::const_eval::eval_queries::eval_to_const_value_raw_provider
  31:     0x7f1fb13ac04b - rustc_query_system[aa73f8cecf880083]::query::plumbing::get_query::<rustc_query_impl[f78d6cf74d1d25b1]::queries::eval_to_const_value_raw, rustc_query_impl[f78d6cf74d1d25b1]::plumbing::QueryCtxt>
  32:     0x7f1fb11dafac - <rustc_query_impl[f78d6cf74d1d25b1]::Queries as rustc_middle[2b0e30f1a844f333]::ty::query::QueryEngine>::eval_to_const_value_raw
  33:     0x7f1fb2213bf4 - <rustc_middle[2b0e30f1a844f333]::ty::context::TyCtxt>::const_eval_global_id
  34:     0x7f1fb2212aec - <rustc_middle[2b0e30f1a844f333]::ty::context::TyCtxt>::const_eval_resolve
  35:     0x7f1fafb35424 - <rustc_monomorphize[65cc958de9c9010]::collector::MirNeighborCollector as rustc_middle[2b0e30f1a844f333]::mir::visit::Visitor>::visit_constant
  36:     0x7f1fafb2e19c - <rustc_monomorphize[65cc958de9c9010]::collector::MirNeighborCollector as rustc_middle[2b0e30f1a844f333]::mir::visit::Visitor>::visit_operand
  37:     0x7f1fafb2c3d6 - <rustc_monomorphize[65cc958de9c9010]::collector::MirNeighborCollector as rustc_middle[2b0e30f1a844f333]::mir::visit::Visitor>::visit_rvalue
  38:     0x7f1fafb38387 - rustc_monomorphize[65cc958de9c9010]::collector::collect_neighbours
  39:     0x7f1fafb33a09 - rustc_monomorphize[65cc958de9c9010]::collector::collect_items_rec
  40:     0x7f1fafb34911 - rustc_monomorphize[65cc958de9c9010]::collector::collect_items_rec
  41:     0x7f1fafb34911 - rustc_monomorphize[65cc958de9c9010]::collector::collect_items_rec
  42:     0x7f1fafb34911 - rustc_monomorphize[65cc958de9c9010]::collector::collect_items_rec
  43:     0x7f1fafb3b144 - <core[b6c6cfde078dfe44]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[a1b3e7ae5b8f85cd]::sync::par_for_each_in<alloc[e77f571ce596fd94]::vec::Vec<rustc_middle[2b0e30f1a844f333]::mir::mono::MonoItem>, rustc_monomorphize[65cc958de9c9010]::collector::collect_crate_mono_items::{closure#1}::{closure#0}>::{closure#0}::{closure#0}> as core[b6c6cfde078dfe44]::ops::function::FnOnce<()>>::call_once
  44:     0x7f1fafb787d5 - std[24b66dceb5e0a55f]::panicking::try::<(), core[b6c6cfde078dfe44]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[a1b3e7ae5b8f85cd]::sync::par_for_each_in<alloc[e77f571ce596fd94]::vec::Vec<rustc_middle[2b0e30f1a844f333]::mir::mono::MonoItem>, rustc_monomorphize[65cc958de9c9010]::collector::collect_crate_mono_items::{closure#1}::{closure#0}>::{closure#0}::{closure#0}>>
  45:     0x7f1fafb4f5e3 - <rustc_session[70efa93bdab11654]::session::Session>::time::<(), rustc_monomorphize[65cc958de9c9010]::collector::collect_crate_mono_items::{closure#1}>
  46:     0x7f1fafb308ff - rustc_monomorphize[65cc958de9c9010]::collector::collect_crate_mono_items
  47:     0x7f1fafb4427a - rustc_monomorphize[65cc958de9c9010]::partitioning::collect_and_partition_mono_items
  48:     0x7f1fb12f8ef2 - rustc_query_system[aa73f8cecf880083]::query::plumbing::try_execute_query::<rustc_query_impl[f78d6cf74d1d25b1]::plumbing::QueryCtxt, rustc_query_system[aa73f8cecf880083]::query::caches::DefaultCache<(), (&std[24b66dceb5e0a55f]::collections::hash::set::HashSet<rustc_span[c86d7b72459e274]::def_id::DefId, core[b6c6cfde078dfe44]::hash::BuildHasherDefault<rustc_hash[cffe43c69f8f1eff]::FxHasher>>, &[rustc_middle[2b0e30f1a844f333]::mir::mono::CodegenUnit])>>
  49:     0x7f1fb13c5785 - rustc_query_system[aa73f8cecf880083]::query::plumbing::get_query::<rustc_query_impl[f78d6cf74d1d25b1]::queries::collect_and_partition_mono_items, rustc_query_impl[f78d6cf74d1d25b1]::plumbing::QueryCtxt>
  50:     0x7f1fb1217148 - <rustc_query_impl[f78d6cf74d1d25b1]::Queries as rustc_middle[2b0e30f1a844f333]::ty::query::QueryEngine>::collect_and_partition_mono_items
  51:     0x7f1faf93dd63 - rustc_codegen_ssa[d2586b3b6cdc69b0]::base::codegen_crate::<rustc_codegen_llvm[8276a0420c0bc8c4]::LlvmCodegenBackend>
  52:     0x7f1fafa0ebfd - <rustc_codegen_llvm[8276a0420c0bc8c4]::LlvmCodegenBackend as rustc_codegen_ssa[d2586b3b6cdc69b0]::traits::backend::CodegenBackend>::codegen_crate
  53:     0x7f1faf84cadb - <rustc_session[70efa93bdab11654]::session::Session>::time::<alloc[e77f571ce596fd94]::boxed::Box<dyn core[b6c6cfde078dfe44]::any::Any>, rustc_interface[6492f6541a5bdae5]::passes::start_codegen::{closure#0}>
  54:     0x7f1faf815ae5 - <rustc_interface[6492f6541a5bdae5]::passes::QueryContext>::enter::<<rustc_interface[6492f6541a5bdae5]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[b6c6cfde078dfe44]::result::Result<alloc[e77f571ce596fd94]::boxed::Box<dyn core[b6c6cfde078dfe44]::any::Any>, rustc_errors[ce972c85f60e45cf]::ErrorGuaranteed>>
  55:     0x7f1faf8046ed - <rustc_interface[6492f6541a5bdae5]::queries::Queries>::ongoing_codegen
  56:     0x7f1faf6dc83e - rustc_interface[6492f6541a5bdae5]::interface::create_compiler_and_run::<core[b6c6cfde078dfe44]::result::Result<(), rustc_errors[ce972c85f60e45cf]::ErrorGuaranteed>, rustc_driver[511eab029b20559e]::run_compiler::{closure#1}>
  57:     0x7f1faf6c3102 - <scoped_tls[ecdf0725d9878678]::ScopedKey<rustc_span[c86d7b72459e274]::SessionGlobals>>::set::<rustc_interface[6492f6541a5bdae5]::interface::run_compiler<core[b6c6cfde078dfe44]::result::Result<(), rustc_errors[ce972c85f60e45cf]::ErrorGuaranteed>, rustc_driver[511eab029b20559e]::run_compiler::{closure#1}>::{closure#0}, core[b6c6cfde078dfe44]::result::Result<(), rustc_errors[ce972c85f60e45cf]::ErrorGuaranteed>>
  58:     0x7f1faf74065f - std[24b66dceb5e0a55f]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[6492f6541a5bdae5]::util::run_in_thread_pool_with_globals<rustc_interface[6492f6541a5bdae5]::interface::run_compiler<core[b6c6cfde078dfe44]::result::Result<(), rustc_errors[ce972c85f60e45cf]::ErrorGuaranteed>, rustc_driver[511eab029b20559e]::run_compiler::{closure#1}>::{closure#0}, core[b6c6cfde078dfe44]::result::Result<(), rustc_errors[ce972c85f60e45cf]::ErrorGuaranteed>>::{closure#0}, core[b6c6cfde078dfe44]::result::Result<(), rustc_errors[ce972c85f60e45cf]::ErrorGuaranteed>>
  59:     0x7f1faf6db43e - std[24b66dceb5e0a55f]::panic::catch_unwind::<core[b6c6cfde078dfe44]::panic::unwind_safe::AssertUnwindSafe<<std[24b66dceb5e0a55f]::thread::Builder>::spawn_unchecked_<rustc_interface[6492f6541a5bdae5]::util::run_in_thread_pool_with_globals<rustc_interface[6492f6541a5bdae5]::interface::run_compiler<core[b6c6cfde078dfe44]::result::Result<(), rustc_errors[ce972c85f60e45cf]::ErrorGuaranteed>, rustc_driver[511eab029b20559e]::run_compiler::{closure#1}>::{closure#0}, core[b6c6cfde078dfe44]::result::Result<(), rustc_errors[ce972c85f60e45cf]::ErrorGuaranteed>>::{closure#0}, core[b6c6cfde078dfe44]::result::Result<(), rustc_errors[ce972c85f60e45cf]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[b6c6cfde078dfe44]::result::Result<(), rustc_errors[ce972c85f60e45cf]::ErrorGuaranteed>>
  60:     0x7f1faf743ae2 - <<std[24b66dceb5e0a55f]::thread::Builder>::spawn_unchecked_<rustc_interface[6492f6541a5bdae5]::util::run_in_thread_pool_with_globals<rustc_interface[6492f6541a5bdae5]::interface::run_compiler<core[b6c6cfde078dfe44]::result::Result<(), rustc_errors[ce972c85f60e45cf]::ErrorGuaranteed>, rustc_driver[511eab029b20559e]::run_compiler::{closure#1}>::{closure#0}, core[b6c6cfde078dfe44]::result::Result<(), rustc_errors[ce972c85f60e45cf]::ErrorGuaranteed>>::{closure#0}, core[b6c6cfde078dfe44]::result::Result<(), rustc_errors[ce972c85f60e45cf]::ErrorGuaranteed>>::{closure#1} as core[b6c6cfde078dfe44]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  61:     0x7f1faed21dc5 - std::sys::unix::thread::Thread::new::thread_start::h7617fc35876b6b20
  62:     0x7f1faeabdb43 - <unknown>
  63:     0x7f1faeb4fa00 - <unknown>
  64:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.65.0-nightly (0599b7f65 2022-09-08) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 -Z thir-unsafeck
query stack during panic:
#0 [promoted_mir] optimizing promoted MIR for `main::{closure#6}::{closure#0}`
#0 [promoted_mir] optimizing promoted MIR for `main::{closure#6}::{closure#0}`
#1 [eval_to_allocation_raw] const-evaluating + checking `main::{closure#6}::{closure#0}::promoted[0]`
#2 [eval_to_const_value_raw] simplifying constant for the type system `main::{closure#6}::{closure#0}::promoted[0]`
#3 [eval_to_const_value_raw] simplifying constant for the type system `main::{closure#6}::{closure#0}::promoted[0]`
#4 [collect_and_partition_mono_items] collect_and_partition_mono_items
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/async-await/async-await.rs#default stdout ----

error in revision `default`: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/async-await.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "default" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-await.default/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-await.default/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: compiler/rustc_mir_transform/src/generator.rs:1255:14: unexpected generator type [closure@/checkout/src/test/ui/async-await/async-await.rs:213:9: 213:12]
   |
   |
LL | /         |x| {
LL | |             async move {
LL | |                 async_fn_multiple_args_named_lifetime(x, x).await
LL | |             }
LL | |         },

thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1398:9
stack backtrace:
   0:     0x7f6f2a6e979e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha266d3b4421f7468
   0:     0x7f6f2a6e979e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha266d3b4421f7468
   1:     0x7f6f2a752948 - core::fmt::write::hcf80770c462d0907
   2:     0x7f6f2a6da0a1 - std::io::Write::write_fmt::h6067fcde5a266ae4
   3:     0x7f6f2a6ec7ae - std::panicking::default_hook::{{closure}}::h8227521fd9887c61
   4:     0x7f6f2a6ec477 - std::panicking::default_hook::h4f005cd45a35ad85
   5:     0x7f6f2b0afb34 - rustc_driver[511eab029b20559e]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f6f2a6ecf61 - std::panicking::rust_panic_with_hook::h7ec5ac7188043c5a
   7:     0x7f6f2b66dbe3 - std[24b66dceb5e0a55f]::panicking::begin_panic::<rustc_errors[ce972c85f60e45cf]::ExplicitBug>::{closure#0}
   8:     0x7f6f2b66a386 - std[24b66dceb5e0a55f]::sys_common::backtrace::__rust_end_short_backtrace::<std[24b66dceb5e0a55f]::panicking::begin_panic<rustc_errors[ce972c85f60e45cf]::ExplicitBug>::{closure#0}, !>
   9:     0x7f6f2aee5806 - std[24b66dceb5e0a55f]::panicking::begin_panic::<rustc_errors[ce972c85f60e45cf]::ExplicitBug>
  10:     0x7f6f2b715be6 - std[24b66dceb5e0a55f]::panic::panic_any::<rustc_errors[ce972c85f60e45cf]::ExplicitBug>
  11:     0x7f6f2b715b83 - <rustc_errors[ce972c85f60e45cf]::HandlerInner>::span_bug::<rustc_span[c86d7b72459e274]::span_encoding::Span, &alloc[e77f571ce596fd94]::string::String>
  12:     0x7f6f2b7158c0 - <rustc_errors[ce972c85f60e45cf]::Handler>::span_bug::<rustc_span[c86d7b72459e274]::span_encoding::Span, &alloc[e77f571ce596fd94]::string::String>
  13:     0x7f6f2b74ece2 - rustc_middle[2b0e30f1a844f333]::ty::context::tls::with_context_opt::<rustc_middle[2b0e30f1a844f333]::ty::context::tls::with_opt<rustc_middle[2b0e30f1a844f333]::util::bug::opt_span_bug_fmt<rustc_span[c86d7b72459e274]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  14:     0x7f6f2b74cd49 - rustc_middle[2b0e30f1a844f333]::util::bug::opt_span_bug_fmt::<rustc_span[c86d7b72459e274]::span_encoding::Span>
  15:     0x7f6f2aee8fca - rustc_middle[2b0e30f1a844f333]::util::bug::span_bug_fmt::<rustc_span[c86d7b72459e274]::span_encoding::Span>
  16:     0x7f6f2b5807d8 - rustc_mir_transform[3779cdcb05977e58]::generator::state_transform_body_for_generator
  17:     0x7f6f2b58c806 - rustc_mir_transform[3779cdcb05977e58]::run_analysis_to_runtime_passes
  18:     0x7f6f2b58e50e - rustc_mir_transform[3779cdcb05977e58]::promoted_mir
  19:     0x7f6f2b57350a - <rustc_mir_transform[3779cdcb05977e58]::provide::{closure#3} as core[b6c6cfde078dfe44]::ops::function::FnOnce<(rustc_middle[2b0e30f1a844f333]::ty::context::TyCtxt, rustc_span[c86d7b72459e274]::def_id::DefId)>>::call_once
  20:     0x7f6f2cca8413 - rustc_query_system[aa73f8cecf880083]::query::plumbing::try_execute_query::<rustc_query_impl[f78d6cf74d1d25b1]::plumbing::QueryCtxt, rustc_query_system[aa73f8cecf880083]::query::caches::DefaultCache<rustc_span[c86d7b72459e274]::def_id::DefId, &rustc_index[7796512a5b8187f]::vec::IndexVec<rustc_middle[2b0e30f1a844f333]::mir::Promoted, rustc_middle[2b0e30f1a844f333]::mir::Body>>>
  21:     0x7f6f2cd578c0 - rustc_query_system[aa73f8cecf880083]::query::plumbing::get_query::<rustc_query_impl[f78d6cf74d1d25b1]::queries::promoted_mir, rustc_query_impl[f78d6cf74d1d25b1]::plumbing::QueryCtxt>
  22:     0x7f6f2cb92665 - <rustc_query_impl[f78d6cf74d1d25b1]::Queries as rustc_middle[2b0e30f1a844f333]::ty::query::QueryEngine>::promoted_mir
  23:     0x7f6f2c64f8dd - <rustc_const_eval[c08325d25416a93]::interpret::eval_context::InterpCx<rustc_const_eval[c08325d25416a93]::const_eval::machine::CompileTimeInterpreter>>::load_mir
  24:     0x7f6f2c76f4c3 - rustc_const_eval[c08325d25416a93]::const_eval::eval_queries::eval_to_allocation_raw_provider
  25:     0x7f6f2cd7cbc5 - rustc_query_system[aa73f8cecf880083]::query::plumbing::get_query::<rustc_query_impl[f78d6cf74d1d25b1]::queries::eval_to_allocation_raw, rustc_query_impl[f78d6cf74d1d25b1]::plumbing::QueryCtxt>
  26:     0x7f6f2cbb27ec - <rustc_query_impl[f78d6cf74d1d25b1]::Queries as rustc_middle[2b0e30f1a844f333]::ty::query::QueryEngine>::eval_to_allocation_raw
  27:     0x7f6f2c76d8b5 - rustc_const_eval[c08325d25416a93]::const_eval::eval_queries::eval_to_const_value_raw_provider
  28:     0x7f6f2cd8404b - rustc_query_system[aa73f8cecf880083]::query::plumbing::get_query::<rustc_query_impl[f78d6cf74d1d25b1]::queries::eval_to_const_value_raw, rustc_query_impl[f78d6cf74d1d25b1]::plumbing::QueryCtxt>
  29:     0x7f6f2cbb2fac - <rustc_query_impl[f78d6cf74d1d25b1]::Queries as rustc_middle[2b0e30f1a844f333]::ty::query::QueryEngine>::eval_to_const_value_raw
  30:     0x7f6f2c76d55c - rustc_const_eval[c08325d25416a93]::const_eval::eval_queries::eval_to_const_value_raw_provider
  31:     0x7f6f2cd8404b - rustc_query_system[aa73f8cecf880083]::query::plumbing::get_query::<rustc_query_impl[f78d6cf74d1d25b1]::queries::eval_to_const_value_raw, rustc_query_impl[f78d6cf74d1d25b1]::plumbing::QueryCtxt>
  32:     0x7f6f2cbb2fac - <rustc_query_impl[f78d6cf74d1d25b1]::Queries as rustc_middle[2b0e30f1a844f333]::ty::query::QueryEngine>::eval_to_const_value_raw
  33:     0x7f6f2dbebbf4 - <rustc_middle[2b0e30f1a844f333]::ty::context::TyCtxt>::const_eval_global_id
  34:     0x7f6f2dbeaaec - <rustc_middle[2b0e30f1a844f333]::ty::context::TyCtxt>::const_eval_resolve
  35:     0x7f6f2b50d424 - <rustc_monomorphize[65cc958de9c9010]::collector::MirNeighborCollector as rustc_middle[2b0e30f1a844f333]::mir::visit::Visitor>::visit_constant
  36:     0x7f6f2b50619c - <rustc_monomorphize[65cc958de9c9010]::collector::MirNeighborCollector as rustc_middle[2b0e30f1a844f333]::mir::visit::Visitor>::visit_operand
  37:     0x7f6f2b5043d6 - <rustc_monomorphize[65cc958de9c9010]::collector::MirNeighborCollector as rustc_middle[2b0e30f1a844f333]::mir::visit::Visitor>::visit_rvalue
  38:     0x7f6f2b510387 - rustc_monomorphize[65cc958de9c9010]::collector::collect_neighbours
  39:     0x7f6f2b50ba09 - rustc_monomorphize[65cc958de9c9010]::collector::collect_items_rec
  40:     0x7f6f2b50c911 - rustc_monomorphize[65cc958de9c9010]::collector::collect_items_rec
  41:     0x7f6f2b50c911 - rustc_monomorphize[65cc958de9c9010]::collector::collect_items_rec
  42:     0x7f6f2b50c911 - rustc_monomorphize[65cc958de9c9010]::collector::collect_items_rec
  43:     0x7f6f2b513144 - <core[b6c6cfde078dfe44]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[a1b3e7ae5b8f85cd]::sync::par_for_each_in<alloc[e77f571ce596fd94]::vec::Vec<rustc_middle[2b0e30f1a844f333]::mir::mono::MonoItem>, rustc_monomorphize[65cc958de9c9010]::collector::collect_crate_mono_items::{closure#1}::{closure#0}>::{closure#0}::{closure#0}> as core[b6c6cfde078dfe44]::ops::function::FnOnce<()>>::call_once
  44:     0x7f6f2b5507d5 - std[24b66dceb5e0a55f]::panicking::try::<(), core[b6c6cfde078dfe44]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[a1b3e7ae5b8f85cd]::sync::par_for_each_in<alloc[e77f571ce596fd94]::vec::Vec<rustc_middle[2b0e30f1a844f333]::mir::mono::MonoItem>, rustc_monomorphize[65cc958de9c9010]::collector::collect_crate_mono_items::{closure#1}::{closure#0}>::{closure#0}::{closure#0}>>
  45:     0x7f6f2b5275e3 - <rustc_session[70efa93bdab11654]::session::Session>::time::<(), rustc_monomorphize[65cc958de9c9010]::collector::collect_crate_mono_items::{closure#1}>
  46:     0x7f6f2b5088ff - rustc_monomorphize[65cc958de9c9010]::collector::collect_crate_mono_items
  47:     0x7f6f2b51c27a - rustc_monomorphize[65cc958de9c9010]::partitioning::collect_and_partition_mono_items
  48:     0x7f6f2ccd0ef2 - rustc_query_system[aa73f8cecf880083]::query::plumbing::try_execute_query::<rustc_query_impl[f78d6cf74d1d25b1]::plumbing::QueryCtxt, rustc_query_system[aa73f8cecf880083]::query::caches::DefaultCache<(), (&std[24b66dceb5e0a55f]::collections::hash::set::HashSet<rustc_span[c86d7b72459e274]::def_id::DefId, core[b6c6cfde078dfe44]::hash::BuildHasherDefault<rustc_hash[cffe43c69f8f1eff]::FxHasher>>, &[rustc_middle[2b0e30f1a844f333]::mir::mono::CodegenUnit])>>
  49:     0x7f6f2cd9d785 - rustc_query_system[aa73f8cecf880083]::query::plumbing::get_query::<rustc_query_impl[f78d6cf74d1d25b1]::queries::collect_and_partition_mono_items, rustc_query_impl[f78d6cf74d1d25b1]::plumbing::QueryCtxt>
  50:     0x7f6f2cbef148 - <rustc_query_impl[f78d6cf74d1d25b1]::Queries as rustc_middle[2b0e30f1a844f333]::ty::query::QueryEngine>::collect_and_partition_mono_items
  51:     0x7f6f2b315d63 - rustc_codegen_ssa[d2586b3b6cdc69b0]::base::codegen_crate::<rustc_codegen_llvm[8276a0420c0bc8c4]::LlvmCodegenBackend>
  52:     0x7f6f2b3e6bfd - <rustc_codegen_llvm[8276a0420c0bc8c4]::LlvmCodegenBackend as rustc_codegen_ssa[d2586b3b6cdc69b0]::traits::backend::CodegenBackend>::codegen_crate
  53:     0x7f6f2b224adb - <rustc_session[70efa93bdab11654]::session::Session>::time::<alloc[e77f571ce596fd94]::boxed::Box<dyn core[b6c6cfde078dfe44]::any::Any>, rustc_interface[6492f6541a5bdae5]::passes::start_codegen::{closure#0}>
  54:     0x7f6f2b1edae5 - <rustc_interface[6492f6541a5bdae5]::passes::QueryContext>::enter::<<rustc_interface[6492f6541a5bdae5]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[b6c6cfde078dfe44]::result::Result<alloc[e77f571ce596fd94]::boxed::Box<dyn core[b6c6cfde078dfe44]::any::Any>, rustc_errors[ce972c85f60e45cf]::ErrorGuaranteed>>
  55:     0x7f6f2b1dc6ed - <rustc_interface[6492f6541a5bdae5]::queries::Queries>::ongoing_codegen
  56:     0x7f6f2b0b483e - rustc_interface[6492f6541a5bdae5]::interface::create_compiler_and_run::<core[b6c6cfde078dfe44]::result::Result<(), rustc_errors[ce972c85f60e45cf]::ErrorGuaranteed>, rustc_driver[511eab029b20559e]::run_compiler::{closure#1}>
  57:     0x7f6f2b09b102 - <scoped_tls[ecdf0725d9878678]::ScopedKey<rustc_span[c86d7b72459e274]::SessionGlobals>>::set::<rustc_interface[6492f6541a5bdae5]::interface::run_compiler<core[b6c6cfde078dfe44]::result::Result<(), rustc_errors[ce972c85f60e45cf]::ErrorGuaranteed>, rustc_driver[511eab029b20559e]::run_compiler::{closure#1}>::{closure#0}, core[b6c6cfde078dfe44]::result::Result<(), rustc_errors[ce972c85f60e45cf]::ErrorGuaranteed>>
  58:     0x7f6f2b11865f - std[24b66dceb5e0a55f]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[6492f6541a5bdae5]::util::run_in_thread_pool_with_globals<rustc_interface[6492f6541a5bdae5]::interface::run_compiler<core[b6c6cfde078dfe44]::result::Result<(), rustc_errors[ce972c85f60e45cf]::ErrorGuaranteed>, rustc_driver[511eab029b20559e]::run_compiler::{closure#1}>::{closure#0}, core[b6c6cfde078dfe44]::result::Result<(), rustc_errors[ce972c85f60e45cf]::ErrorGuaranteed>>::{closure#0}, core[b6c6cfde078dfe44]::result::Result<(), rustc_errors[ce972c85f60e45cf]::ErrorGuaranteed>>
  59:     0x7f6f2b0b343e - std[24b66dceb5e0a55f]::panic::catch_unwind::<core[b6c6cfde078dfe44]::panic::unwind_safe::AssertUnwindSafe<<std[24b66dceb5e0a55f]::thread::Builder>::spawn_unchecked_<rustc_interface[6492f6541a5bdae5]::util::run_in_thread_pool_with_globals<rustc_interface[6492f6541a5bdae5]::interface::run_compiler<core[b6c6cfde078dfe44]::result::Result<(), rustc_errors[ce972c85f60e45cf]::ErrorGuaranteed>, rustc_driver[511eab029b20559e]::run_compiler::{closure#1}>::{closure#0}, core[b6c6cfde078dfe44]::result::Result<(), rustc_errors[ce972c85f60e45cf]::ErrorGuaranteed>>::{closure#0}, core[b6c6cfde078dfe44]::result::Result<(), rustc_errors[ce972c85f60e45cf]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[b6c6cfde078dfe44]::result::Result<(), rustc_errors[ce972c85f60e45cf]::ErrorGuaranteed>>
  60:     0x7f6f2b11bae2 - <<std[24b66dceb5e0a55f]::thread::Builder>::spawn_unchecked_<rustc_interface[6492f6541a5bdae5]::util::run_in_thread_pool_with_globals<rustc_interface[6492f6541a5bdae5]::interface::run_compiler<core[b6c6cfde078dfe44]::result::Result<(), rustc_errors[ce972c85f60e45cf]::ErrorGuaranteed>, rustc_driver[511eab029b20559e]::run_compiler::{closure#1}>::{closure#0}, core[b6c6cfde078dfe44]::result::Result<(), rustc_errors[ce972c85f60e45cf]::ErrorGuaranteed>>::{closure#0}, core[b6c6cfde078dfe44]::result::Result<(), rustc_errors[ce972c85f60e45cf]::ErrorGuaranteed>>::{closure#1} as core[b6c6cfde078dfe44]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  61:     0x7f6f2a6f9dc5 - std::sys::unix::thread::Thread::new::thread_start::h7617fc35876b6b20
  62:     0x7f6f2a495b43 - <unknown>
  63:     0x7f6f2a527a00 - <unknown>
  64:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.65.0-nightly (0599b7f65 2022-09-08) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
#0 [promoted_mir] optimizing promoted MIR for `main::{closure#6}::{closure#0}`
#0 [promoted_mir] optimizing promoted MIR for `main::{closure#6}::{closure#0}`
#1 [eval_to_allocation_raw] const-evaluating + checking `main::{closure#6}::{closure#0}::promoted[0]`
#2 [eval_to_const_value_raw] simplifying constant for the type system `main::{closure#6}::{closure#0}::promoted[0]`
#3 [eval_to_const_value_raw] simplifying constant for the type system `main::{closure#6}::{closure#0}::promoted[0]`
#4 [collect_and_partition_mono_items] collect_and_partition_mono_items
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/async-await/await-into-future.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/await-into-future.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/await-into-future/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/await-into-future/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: compiler/rustc_mir_transform/src/generator.rs:1255:14: unexpected generator type i32
   |
   |
LL |     assert_eq!(AwaitMe.await, 41);
   |
   |
   = note: this error: internal compiler error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1398:9
stack backtrace:
   0:     0x7fdfccd2779e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha266d3b4421f7468
   1:     0x7fdfccd90948 - core::fmt::write::hcf80770c462d0907
   1:     0x7fdfccd90948 - core::fmt::write::hcf80770c462d0907
   2:     0x7fdfccd180a1 - std::io::Write::write_fmt::h6067fcde5a266ae4
   3:     0x7fdfccd2a7ae - std::panicking::default_hook::{{closure}}::h8227521fd9887c61
   4:     0x7fdfccd2a477 - std::panicking::default_hook::h4f005cd45a35ad85
   5:     0x7fdfcd6edb34 - rustc_driver[511eab029b20559e]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fdfccd2af61 - std::panicking::rust_panic_with_hook::h7ec5ac7188043c5a
   7:     0x7fdfcdcabbe3 - std[24b66dceb5e0a55f]::panicking::begin_panic::<rustc_errors[ce972c85f60e45cf]::ExplicitBug>::{closure#0}
   8:     0x7fdfcdca8386 - std[24b66dceb5e0a55f]::sys_common::backtrace::__rust_end_short_backtrace::<std[24b66dceb5e0a55f]::panicking::begin_panic<rustc_errors[ce972c85f60e45cf]::ExplicitBug>::{closure#0}, !>
   9:     0x7fdfcd523806 - std[24b66dceb5e0a55f]::panicking::begin_panic::<rustc_errors[ce972c85f60e45cf]::ExplicitBug>
  10:     0x7fdfcdd53be6 - std[24b66dceb5e0a55f]::panic::panic_any::<rustc_errors[ce972c85f60e45cf]::ExplicitBug>
  11:     0x7fdfcdd53b83 - <rustc_errors[ce972c85f60e45cf]::HandlerInner>::span_bug::<rustc_span[c86d7b72459e274]::span_encoding::Span, &alloc[e77f571ce596fd94]::string::String>
  12:     0x7fdfcdd538c0 - <rustc_errors[ce972c85f60e45cf]::Handler>::span_bug::<rustc_span[c86d7b72459e274]::span_encoding::Span, &alloc[e77f571ce596fd94]::string::String>
  13:     0x7fdfcdd8cce2 - rustc_middle[2b0e30f1a844f333]::ty::context::tls::with_context_opt::<rustc_middle[2b0e30f1a844f333]::ty::context::tls::with_opt<rustc_middle[2b0e30f1a844f333]::util::bug::opt_span_bug_fmt<rustc_span[c86d7b72459e274]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  14:     0x7fdfcdd8ad49 - rustc_middle[2b0e30f1a844f333]::util::bug::opt_span_bug_fmt::<rustc_span[c86d7b72459e274]::span_encoding::Span>
  15:     0x7fdfcd526fca - rustc_middle[2b0e30f1a844f333]::util::bug::span_bug_fmt::<rustc_span[c86d7b72459e274]::span_encoding::Span>
  16:     0x7fdfcdbbe7d8 - rustc_mir_transform[3779cdcb05977e58]::generator::state_transform_body_for_generator
  17:     0x7fdfcdbca806 - rustc_mir_transform[3779cdcb05977e58]::run_analysis_to_runtime_passes
  18:     0x7fdfcdbcc50e - rustc_mir_transform[3779cdcb05977e58]::promoted_mir
  19:     0x7fdfcdbb150a - <rustc_mir_transform[3779cdcb05977e58]::provide::{closure#3} as core[b6c6cfde078dfe44]::ops::function::FnOnce<(rustc_middle[2b0e30f1a844f333]::ty::context::TyCtxt, rustc_span[c86d7b72459e274]::def_id::DefId)>>::call_once
  20:     0x7fdfcf2e6413 - rustc_query_system[aa73f8cecf880083]::query::plumbing::try_execute_query::<rustc_query_impl[f78d6cf74d1d25b1]::plumbing::QueryCtxt, rustc_query_system[aa73f8cecf880083]::query::caches::DefaultCache<rustc_span[c86d7b72459e274]::def_id::DefId, &rustc_index[7796512a5b8187f]::vec::IndexVec<rustc_middle[2b0e30f1a844f333]::mir::Promoted, rustc_middle[2b0e30f1a844f333]::mir::Body>>>
  21:     0x7fdfcf3958c0 - rustc_query_system[aa73f8cecf880083]::query::plumbing::get_query::<rustc_query_impl[f78d6cf74d1d25b1]::queries::promoted_mir, rustc_query_impl[f78d6cf74d1d25b1]::plumbing::QueryCtxt>
  22:     0x7fdfcf1d0665 - <rustc_query_impl[f78d6cf74d1d25b1]::Queries as rustc_middle[2b0e30f1a844f333]::ty::query::QueryEngine>::promoted_mir
  23:     0x7fdfcec8d8dd - <rustc_const_eval[c08325d25416a93]::interpret::eval_context::InterpCx<rustc_const_eval[c08325d25416a93]::const_eval::machine::CompileTimeInterpreter>>::load_mir
  24:     0x7fdfcedad4c3 - rustc_const_eval[c08325d25416a93]::const_eval::eval_queries::eval_to_allocation_raw_provider
  25:     0x7fdfcf3babc5 - rustc_query_system[aa73f8cecf880083]::query::plumbing::get_query::<rustc_query_impl[f78d6cf74d1d25b1]::queries::eval_to_allocation_raw, rustc_query_impl[f78d6cf74d1d25b1]::plumbing::QueryCtxt>
  26:     0x7fdfcf1f07ec - <rustc_query_impl[f78d6cf74d1d25b1]::Queries as rustc_middle[2b0e30f1a844f333]::ty::query::QueryEngine>::eval_to_allocation_raw
  27:     0x7fdfcedab8b5 - rustc_const_eval[c08325d25416a93]::const_eval::eval_queries::eval_to_const_value_raw_provider
  28:     0x7fdfcf3c204b - rustc_query_system[aa73f8cecf880083]::query::plumbing::get_query::<rustc_query_impl[f78d6cf74d1d25b1]::queries::eval_to_const_value_raw, rustc_query_impl[f78d6cf74d1d25b1]::plumbing::QueryCtxt>
  29:     0x7fdfcf1f0fac - <rustc_query_impl[f78d6cf74d1d25b1]::Queries as rustc_middle[2b0e30f1a844f333]::ty::query::QueryEngine>::eval_to_const_value_raw
  30:     0x7fdfcedab55c - rustc_const_eval[c08325d25416a93]::const_eval::eval_queries::eval_to_const_value_raw_provider
  31:     0x7fdfcf3c204b - rustc_query_system[aa73f8cecf880083]::query::plumbing::get_query::<rustc_query_impl[f78d6cf74d1d25b1]::queries::eval_to_const_value_raw, rustc_query_impl[f78d6cf74d1d25b1]::plumbing::QueryCtxt>
  32:     0x7fdfcf1f0fac - <rustc_query_impl[f78d6cf74d1d25b1]::Queries as rustc_middle[2b0e30f1a844f333]::ty::query::QueryEngine>::eval_to_const_value_raw
  33:     0x7fdfd0229bf4 - <rustc_middle[2b0e30f1a844f333]::ty::context::TyCtxt>::const_eval_global_id
  34:     0x7fdfd0228aec - <rustc_middle[2b0e30f1a844f333]::ty::context::TyCtxt>::const_eval_resolve
  35:     0x7fdfcdb4b424 - <rustc_monomorphize[65cc958de9c9010]::collector::MirNeighborCollector as rustc_middle[2b0e30f1a844f333]::mir::visit::Visitor>::visit_constant
  36:     0x7fdfcdb4419c - <rustc_monomorphize[65cc958de9c9010]::collector::MirNeighborCollector as rustc_middle[2b0e30f1a844f333]::mir::visit::Visitor>::visit_operand
  37:     0x7fdfcdb423d6 - <rustc_monomorphize[65cc958de9c9010]::collector::MirNeighborCollector as rustc_middle[2b0e30f1a844f333]::mir::visit::Visitor>::visit_rvalue
  38:     0x7fdfcdb4e387 - rustc_monomorphize[65cc958de9c9010]::collector::collect_neighbours
  39:     0x7fdfcdb49a09 - rustc_monomorphize[65cc958de9c9010]::collector::collect_items_rec
  40:     0x7fdfcdb4a911 - rustc_monomorphize[65cc958de9c9010]::collector::collect_items_rec
  41:     0x7fdfcdb4a911 - rustc_monomorphize[65cc958de9c9010]::collector::collect_items_rec
  42:     0x7fdfcdb4a911 - rustc_monomorphize[65cc958de9c9010]::collector::collect_items_rec
  43:     0x7fdfcdb4a911 - rustc_monomorphize[65cc958de9c9010]::collector::collect_items_rec
  44:     0x7fdfcdb51144 - <core[b6c6cfde078dfe44]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[a1b3e7ae5b8f85cd]::sync::par_for_each_in<alloc[e77f571ce596fd94]::vec::Vec<rustc_middle[2b0e30f1a844f333]::mir::mono::MonoItem>, rustc_monomorphize[65cc958de9c9010]::collector::collect_crate_mono_items::{closure#1}::{closure#0}>::{closure#0}::{closure#0}> as core[b6c6cfde078dfe44]::ops::function::FnOnce<()>>::call_once
  45:     0x7fdfcdb8e7d5 - std[24b66dceb5e0a55f]::panicking::try::<(), core[b6c6cfde078dfe44]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[a1b3e7ae5b8f85cd]::sync::par_for_each_in<alloc[e77f571ce596fd94]::vec::Vec<rustc_middle[2b0e30f1a844f333]::mir::mono::MonoItem>, rustc_monomorphize[65cc958de9c9010]::collector::collect_crate_mono_items::{closure#1}::{closure#0}>::{closure#0}::{closure#0}>>
  46:     0x7fdfcdb655e3 - <rustc_session[70efa93bdab11654]::session::Session>::time::<(), rustc_monomorphize[65cc958de9c9010]::collector::collect_crate_mono_items::{closure#1}>
  47:     0x7fdfcdb468ff - rustc_monomorphize[65cc958de9c9010]::collector::collect_crate_mono_items
  48:     0x7fdfcdb5a27a - rustc_monomorphize[65cc958de9c9010]::partitioning::collect_and_partition_mono_items
  49:     0x7fdfcf30eef2 - rustc_query_system[aa73f8cecf880083]::query::plumbing::try_execute_query::<rustc_query_impl[f78d6cf74d1d25b1]::plumbing::QueryCtxt, rustc_query_system[aa73f8cecf880083]::query::caches::DefaultCache<(), (&std[24b66dceb5e0a55f]::collections::hash::set::HashSet<rustc_span[c86d7b72459e274]::def_id::DefId, core[b6c6cfde078dfe44]::hash::BuildHasherDefault<rustc_hash[cffe43c69f8f1eff]::FxHasher>>, &[rustc_middle[2b0e30f1a844f333]::mir::mono::CodegenUnit])>>
  50:     0x7fdfcf3db785 - rustc_query_system[aa73f8cecf880083]::query::plumbing::get_query::<rustc_query_impl[f78d6cf74d1d25b1]::queries::collect_and_partition_mono_items, rustc_query_impl[f78d6cf74d1d25b1]::plumbing::QueryCtxt>
  51:     0x7fdfcf22d148 - <rustc_query_impl[f78d6cf74d1d25b1]::Queries as rustc_middle[2b0e30f1a844f333]::ty::query::QueryEngine>::collect_and_partition_mono_items
  52:     0x7fdfcd953d63 - rustc_codegen_ssa[d2586b3b6cdc69b0]::base::codegen_crate::<rustc_codegen_llvm[8276a0420c0bc8c4]::LlvmCodegenBackend>
  53:     0x7fdfcda24bfd - <rustc_codegen_llvm[8276a0420c0bc8c4]::LlvmCodegenBackend as rustc_codegen_ssa[d2586b3b6cdc69b0]::traits::backend::CodegenBackend>::codegen_crate
  54:     0x7fdfcd862adb - <rustc_session[70efa93bdab11654]::session::Session>::time::<alloc[e77f571ce596fd94]::boxed::Box<dyn core[b6c6cfde078dfe44]::any::Any>, rustc_interface[6492f6541a5bdae5]::passes::start_codegen::{closure#0}>
  55:     0x7fdfcd82bae5 - <rustc_interface[6492f6541a5bdae5]::passes::QueryContext>::enter::<<rustc_interface[6492f6541a5bdae5]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[b6c6cfde078dfe44]::result::Result<alloc[e77f571ce596fd94]::boxed::Box<dyn core[b6c6cfde078dfe44]::any::Any>, rustc_errors[ce972c85f60e45cf]::ErrorGuaranteed>>
  56:     0x7fdfcd81a6ed - <rustc_interface[6492f6541a5bdae5]::queries::Queries>::ongoing_codegen
  57:     0x7fdfcd6f283e - rustc_interface[6492f6541a5bdae5]::interface::create_compiler_and_run::<core[b6c6cfde078dfe44]::result::Result<(), rustc_errors[ce972c85f60e45cf]::ErrorGuaranteed>, rustc_driver[511eab029b20559e]::run_compiler::{closure#1}>
  58:     0x7fdfcd6d9102 - <scoped_tls[ecdf0725d9878678]::ScopedKey<rustc_span[c86d7b72459e274]::SessionGlobals>>::set::<rustc_interface[6492f6541a5bdae5]::interface::run_compiler<core[b6c6cfde078dfe44]::result::Result<(), rustc_errors[ce972c85f60e45cf]::ErrorGuaranteed>, rustc_driver[511eab029b20559e]::run_compiler::{closure#1}>::{closure#0}, core[b6c6cfde078dfe44]::result::Result<(), rustc_errors[ce972c85f60e45cf]::ErrorGuaranteed>>
  59:     0x7fdfcd75665f - std[24b66dceb5e0a55f]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[6492f6541a5bdae5]::util::run_in_thread_pool_with_globals<rustc_interface[6492f6541a5bdae5]::interface::run_compiler<core[b6c6cfde078dfe44]::result::Result<(), rustc_errors[ce972c85f60e45cf]::ErrorGuaranteed>, rustc_driver[511eab029b20559e]::run_compiler::{closure#1}>::{closure#0}, core[b6c6cfde078dfe44]::result::Result<(), rustc_errors[ce972c85f60e45cf]::ErrorGuaranteed>>::{closure#0}, core[b6c6cfde078dfe44]::result::Result<(), rustc_errors[ce972c85f60e45cf]::ErrorGuaranteed>>
  60:     0x7fdfcd6f143e - std[24b66dceb5e0a55f]::panic::catch_unwind::<core[b6c6cfde078dfe44]::panic::unwind_safe::AssertUnwindSafe<<std[24b66dceb5e0a55f]::thread::Builder>::spawn_unchecked_<rustc_interface[6492f6541a5bdae5]::util::run_in_thread_pool_with_globals<rustc_interface[6492f6541a5bdae5]::interface::run_compiler<core[b6c6cfde078dfe44]::result::Result<(), rustc_errors[ce972c85f60e45cf]::ErrorGuaranteed>, rustc_driver[511eab029b20559e]::run_compiler::{closure#1}>::{closure#0}, core[b6c6cfde078dfe44]::result::Result<(), rustc_errors[ce972c85f60e45cf]::ErrorGuaranteed>>::{closure#0}, core[b6c6cfde078dfe44]::result::Result<(), rustc_errors[ce972c85f60e45cf]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[b6c6cfde078dfe44]::result::Result<(), rustc_errors[ce972c85f60e45cf]::ErrorGuaranteed>>
  61:     0x7fdfcd759ae2 - <<std[24b66dceb5e0a55f]::thread::Builder>::spawn_unchecked_<rustc_interface[6492f6541a5bdae5]::util::run_in_thread_pool_with_globals<rustc_interface[6492f6541a5bdae5]::interface::run_compiler<core[b6c6cfde078dfe44]::result::Result<(), rustc_errors[ce972c85f60e45cf]::ErrorGuaranteed>, rustc_driver[511eab029b20559e]::run_compiler::{closure#1}>::{closure#0}, core[b6c6cfde078dfe44]::result::Result<(), rustc_errors[ce972c85f60e45cf]::ErrorGuaranteed>>::{closure#0}, core[b6c6cfde078dfe44]::result::Result<(), rustc_errors[ce972c85f60e45cf]::ErrorGuaranteed>>::{closure#1} as core[b6c6cfde078dfe44]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  62:     0x7fdfccd37dc5 - std::sys::unix::thread::Thread::new::thread_start::h7617fc35876b6b20
  63:     0x7fdfccad3b43 - <unknown>
  64:     0x7fdfccb65a00 - <unknown>
  65:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.65.0-nightly (0599b7f65 2022-09-08) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [promoted_mir] optimizing promoted MIR for `run::{closure#0}`
#1 [eval_to_allocation_raw] const-evaluating + checking `run::{closure#0}::promoted[0]`
#2 [eval_to_const_value_raw] simplifying constant for the type system `run::{closure#0}::promoted[0]`
#3 [eval_to_const_value_raw] simplifying constant for the type system `run::{closure#0}::promoted[0]`
#4 [collect_and_partition_mono_items] collect_and_partition_mono_items
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/async-await/issue-73137.rs#drop-tracking stdout ----

error in revision `drop-tracking`: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issue-73137.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "drop_tracking" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-73137.drop-tracking/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-Zdrop-tracking" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-73137.drop-tracking/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: compiler/rustc_mir_transform/src/generator.rs:1255:14: unexpected generator type usize
   |
   |
LL |         assert_ne!(0usize, unsafe { std::mem::transmute(action.b) });
   |
   = note: this error: internal compiler error originates in the macro `assert_ne` (in Nightly builds, run with -Z macro-backtrace for more info)

