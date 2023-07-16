plain
..................................i.ii.................................................. 13640/13695
.......................................................
failures:

---- [ui] src/test/ui/fuckit/no-borrowck-dead-code.rs stdout ----
error: test compilation failed although it shouldn't!
status: exit status: 101
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/fuckit/no-borrowck-dead-code.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fuckit/no-borrowck-dead-code/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "fuckit" "-Ztreat-err-as-bug=1" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fuckit/no-borrowck-dead-code/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: `InferCtxt` incorrectly tainted by errors

thread 'rustc' panicked at 'aborting due to `-Z treat-err-as-bug=1`', compiler/rustc_errors/src/lib.rs:1575:30
stack backtrace:
   0:     0x7f4224f5a59e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h35d26345d4760e7b
   1:     0x7f4224fc3b68 - core::fmt::write::h5e7c5cba9273c200
   2:     0x7f4224f4bcb1 - std::io::Write::write_fmt::hb17fc873c9ff4c57
   3:     0x7f4224f5a3a1 - std::sys_common::backtrace::print::hf3744bb82f2d3d9b
   4:     0x7f4224f5d524 - std::panicking::default_hook::{{closure}}::h4ac10ceec330b801
   5:     0x7f4224f5d1e9 - std::panicking::default_hook::h457ef78962678c56
   6:     0x7f422594ade4 - rustc_driver[b0342a61dd5b24e3]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f4224f5dc74 - std::panicking::rust_panic_with_hook::h0f8081ef8adce20f
   8:     0x7f4224f5d999 - std::panicking::begin_panic_handler::{{closure}}::h341a6d5f4a5e7f5d
   9:     0x7f4224f5aad4 - std::sys_common::backtrace::__rust_end_short_backtrace::h591be4776cf12bf0
  10:     0x7f4224f5d6a2 - rust_begin_unwind
  11:     0x7f4224f0ea93 - core::panicking::panic_fmt::h1e024dc81227e718
  12:     0x7f422883e9cd - <rustc_errors[7ab7144927b7cf66]::HandlerInner>::panic_if_treat_err_as_bug
  13:     0x7f422883de93 - <rustc_errors[7ab7144927b7cf66]::HandlerInner>::emit_diagnostic
  14:     0x7f42283cafb1 - <rustc_errors[7ab7144927b7cf66]::HandlerInner>::emit_diag_at_span::<rustc_span[3bad8095227657e3]::span_encoding::Span>
  15:     0x7f42283cb0d7 - <rustc_errors[7ab7144927b7cf66]::HandlerInner>::span_bug::<rustc_span[3bad8095227657e3]::span_encoding::Span, &str>
  16:     0x7f42283caa9c - <rustc_errors[7ab7144927b7cf66]::Handler>::delay_span_bug::<rustc_span[3bad8095227657e3]::span_encoding::Span, &str>
  17:     0x7f42284325ee - <rustc_infer[9637bb98626514bb]::infer::InferCtxt>::set_tainted_by_errors
  18:     0x7f4226b0c27f - rustc_borrowck[b61b7e7cec328648]::nll::compute_regions
  19:     0x7f4226cb79ea - rustc_borrowck[b61b7e7cec328648]::do_mir_borrowck
  20:     0x7f4226ca8cf2 - rustc_borrowck[b61b7e7cec328648]::mir_borrowck
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
  21:     0x7f4226c765fe - <rustc_borrowck[b61b7e7cec328648]::provide::{closure#0} as core[c9877688a8040c91]::ops::function::FnOnce<(rustc_middle[1dfec72d19b1298f]::ty::context::TyCtxt, rustc_span[3bad8095227657e3]::def_id::LocalDefId)>>::call_once
  22:     0x7f422763cc89 - rustc_query_system[1c35ff32dd70f131]::query::plumbing::try_execute_query::<rustc_query_impl[e57b3f6133f11961]::plumbing::QueryCtxt, rustc_query_system[1c35ff32dd70f131]::query::caches::DefaultCache<rustc_span[3bad8095227657e3]::def_id::LocalDefId, &rustc_middle[1dfec72d19b1298f]::mir::query::BorrowCheckResult>>
  23:     0x7f4227712cc5 - rustc_query_system[1c35ff32dd70f131]::query::plumbing::get_query::<rustc_query_impl[e57b3f6133f11961]::queries::mir_borrowck, rustc_query_impl[e57b3f6133f11961]::plumbing::QueryCtxt>
  24:     0x7f42272d6c90 - <rustc_query_impl[e57b3f6133f11961]::Queries as rustc_middle[1dfec72d19b1298f]::ty::query::QueryEngine>::mir_borrowck
  25:     0x7f42262dcaac - rustc_monomorphize[934994da5b30697d]::collector::collect_roots
  26:     0x7f422631b117 - <rustc_session[77e9a28020439a76]::session::Session>::time::<alloc[4aedbcc73b32df3b]::vec::Vec<rustc_middle[1dfec72d19b1298f]::mir::mono::MonoItem>, rustc_monomorphize[934994da5b30697d]::collector::collect_crate_mono_items::{closure#0}>
  27:     0x7f42262dbdef - rustc_monomorphize[934994da5b30697d]::collector::collect_crate_mono_items
  28:     0x7f42262f3b3a - rustc_monomorphize[934994da5b30697d]::partitioning::collect_and_partition_mono_items
  29:     0x7f4227689dae - rustc_query_system[1c35ff32dd70f131]::query::plumbing::try_execute_query::<rustc_query_impl[e57b3f6133f11961]::plumbing::QueryCtxt, rustc_query_system[1c35ff32dd70f131]::query::caches::DefaultCache<(), (&std[e8106f6f1ea2fda5]::collections::hash::set::HashSet<rustc_span[3bad8095227657e3]::def_id::DefId, core[c9877688a8040c91]::hash::BuildHasherDefault<rustc_hash[5ae85e398a63f903]::FxHasher>>, &[rustc_middle[1dfec72d19b1298f]::mir::mono::CodegenUnit])>>
  30:     0x7f422775b380 - rustc_query_system[1c35ff32dd70f131]::query::plumbing::get_query::<rustc_query_impl[e57b3f6133f11961]::queries::collect_and_partition_mono_items, rustc_query_impl[e57b3f6133f11961]::plumbing::QueryCtxt>
  31:     0x7f42273171b8 - <rustc_query_impl[e57b3f6133f11961]::Queries as rustc_middle[1dfec72d19b1298f]::ty::query::QueryEngine>::collect_and_partition_mono_items
  32:     0x7f4225be37db - rustc_codegen_ssa[ca06d3675ad7857a]::base::codegen_crate::<rustc_codegen_llvm[1be891754b1ef9d1]::LlvmCodegenBackend>
  33:     0x7f4225cd2a24 - <rustc_codegen_llvm[1be891754b1ef9d1]::LlvmCodegenBackend as rustc_codegen_ssa[ca06d3675ad7857a]::traits::backend::CodegenBackend>::codegen_crate
  34:     0x7f4225a753fb - <rustc_session[77e9a28020439a76]::session::Session>::time::<alloc[4aedbcc73b32df3b]::boxed::Box<dyn core[c9877688a8040c91]::any::Any>, rustc_interface[beb2b4b7675f0f3b]::passes::start_codegen::{closure#0}>
  35:     0x7f4225a9b8c7 - <rustc_interface[beb2b4b7675f0f3b]::passes::QueryContext>::enter::<<rustc_interface[beb2b4b7675f0f3b]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[c9877688a8040c91]::result::Result<alloc[4aedbcc73b32df3b]::boxed::Box<dyn core[c9877688a8040c91]::any::Any>, rustc_errors[7ab7144927b7cf66]::ErrorGuaranteed>>
  36:     0x7f4225a92add - <rustc_interface[beb2b4b7675f0f3b]::queries::Queries>::ongoing_codegen
  37:     0x7f4225a93780 - <rustc_interface[beb2b4b7675f0f3b]::queries::Queries>::linker
  38:     0x7f42259b9ffb - <rustc_interface[beb2b4b7675f0f3b]::interface::Compiler>::enter::<rustc_driver[b0342a61dd5b24e3]::run_compiler::{closure#1}::{closure#2}, core[c9877688a8040c91]::result::Result<core[c9877688a8040c91]::option::Option<rustc_interface[beb2b4b7675f0f3b]::queries::Linker>, rustc_errors[7ab7144927b7cf66]::ErrorGuaranteed>>
  39:     0x7f422594c52e - rustc_span[3bad8095227657e3]::with_source_map::<core[c9877688a8040c91]::result::Result<(), rustc_errors[7ab7144927b7cf66]::ErrorGuaranteed>, rustc_interface[beb2b4b7675f0f3b]::interface::run_compiler<core[c9877688a8040c91]::result::Result<(), rustc_errors[7ab7144927b7cf66]::ErrorGuaranteed>, rustc_driver[b0342a61dd5b24e3]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  40:     0x7f42259ad76c - <scoped_tls[ae44f77365469c1a]::ScopedKey<rustc_span[3bad8095227657e3]::SessionGlobals>>::set::<rustc_interface[beb2b4b7675f0f3b]::interface::run_compiler<core[c9877688a8040c91]::result::Result<(), rustc_errors[7ab7144927b7cf66]::ErrorGuaranteed>, rustc_driver[b0342a61dd5b24e3]::run_compiler::{closure#1}>::{closure#0}, core[c9877688a8040c91]::result::Result<(), rustc_errors[7ab7144927b7cf66]::ErrorGuaranteed>>
  41:     0x7f42259a8299 - std[e8106f6f1ea2fda5]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[beb2b4b7675f0f3b]::util::run_in_thread_pool_with_globals<rustc_interface[beb2b4b7675f0f3b]::interface::run_compiler<core[c9877688a8040c91]::result::Result<(), rustc_errors[7ab7144927b7cf66]::ErrorGuaranteed>, rustc_driver[b0342a61dd5b24e3]::run_compiler::{closure#1}>::{closure#0}, core[c9877688a8040c91]::result::Result<(), rustc_errors[7ab7144927b7cf66]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[c9877688a8040c91]::result::Result<(), rustc_errors[7ab7144927b7cf66]::ErrorGuaranteed>>
  42:     0x7f42259b2a38 - std[e8106f6f1ea2fda5]::panic::catch_unwind::<core[c9877688a8040c91]::panic::unwind_safe::AssertUnwindSafe<<std[e8106f6f1ea2fda5]::thread::Builder>::spawn_unchecked_<rustc_interface[beb2b4b7675f0f3b]::util::run_in_thread_pool_with_globals<rustc_interface[beb2b4b7675f0f3b]::interface::run_compiler<core[c9877688a8040c91]::result::Result<(), rustc_errors[7ab7144927b7cf66]::ErrorGuaranteed>, rustc_driver[b0342a61dd5b24e3]::run_compiler::{closure#1}>::{closure#0}, core[c9877688a8040c91]::result::Result<(), rustc_errors[7ab7144927b7cf66]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[c9877688a8040c91]::result::Result<(), rustc_errors[7ab7144927b7cf66]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[c9877688a8040c91]::result::Result<(), rustc_errors[7ab7144927b7cf66]::ErrorGuaranteed>>
  43:     0x7f422595e60a - <<std[e8106f6f1ea2fda5]::thread::Builder>::spawn_unchecked_<rustc_interface[beb2b4b7675f0f3b]::util::run_in_thread_pool_with_globals<rustc_interface[beb2b4b7675f0f3b]::interface::run_compiler<core[c9877688a8040c91]::result::Result<(), rustc_errors[7ab7144927b7cf66]::ErrorGuaranteed>, rustc_driver[b0342a61dd5b24e3]::run_compiler::{closure#1}>::{closure#0}, core[c9877688a8040c91]::result::Result<(), rustc_errors[7ab7144927b7cf66]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[c9877688a8040c91]::result::Result<(), rustc_errors[7ab7144927b7cf66]::ErrorGuaranteed>>::{closure#1} as core[c9877688a8040c91]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  44:     0x7f4224f6a4e5 - std::sys::unix::thread::Thread::new::thread_start::hdded26f66088c50e
  45:     0x7f4224d04b43 - <unknown>
  46:     0x7f4224d96a00 - <unknown>
  47:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.66.0-nightly (b8fc65dc9 2022-10-21) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 -Z fuckit -Z treat-err-as-bug=1
query stack during panic:
query stack during panic:
#0 [mir_borrowck] borrow-checking `dead_code`
#1 [collect_and_partition_mono_items] collect_and_partition_mono_items
------------------------------------------




failures:
    [ui] src/test/ui/fuckit/no-borrowck-dead-code.rs
test result: FAILED. 13569 passed; 1 failed; 125 ignored; 0 measured; 0 filtered out; finished in 131.12s

Build completed unsuccessfully in 0:13:27
