plain
failures:

---- [ui] src/test/ui/generic-associated-types/issue-91139.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/issue-91139.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-91139" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-91139/auxiliary"
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
stdout: none
--- stderr -------------------------------
error: internal compiler error: compiler/rustc_borrowck/src/universal_regions.rs:810:36: cannot convert `RePlaceholder(Placeholder { universe: U1, name: BrAnon(0) })` to a region vid
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1391:9
stack backtrace:
stack backtrace:
   0:     0x7f03e272b79c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h9b3b8bcb25916d27
   1:     0x7f03e27918b8 - core::fmt::write::hb15aa4388857828a
   2:     0x7f03e271bdd1 - std::io::Write::write_fmt::hfff109ef619797a4
   3:     0x7f03e272e7ae - std::panicking::default_hook::{{closure}}::h95282cce4a7c2844
   4:     0x7f03e272e476 - std::panicking::default_hook::h9671da670356b952
   5:     0x7f03e30e87d4 - rustc_driver[f632b19d4c81d0d4]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f03e272ef62 - std::panicking::rust_panic_with_hook::hc848d3ac10d75bda
   7:     0x7f03e5a74bf3 - std[56b592617f5022a2]::panicking::begin_panic::<rustc_errors[2f6fbd56604b03c1]::ExplicitBug>::{closure#0}
   8:     0x7f03e5a72466 - std[56b592617f5022a2]::sys_common::backtrace::__rust_end_short_backtrace::<std[56b592617f5022a2]::panicking::begin_panic<rustc_errors[2f6fbd56604b03c1]::ExplicitBug>::{closure#0}, !>
   9:     0x7f03e307f156 - std[56b592617f5022a2]::panicking::begin_panic::<rustc_errors[2f6fbd56604b03c1]::ExplicitBug>
  10:     0x7f03e5a6e466 - std[56b592617f5022a2]::panic::panic_any::<rustc_errors[2f6fbd56604b03c1]::ExplicitBug>
  11:     0x7f03e5a6d716 - <rustc_errors[2f6fbd56604b03c1]::HandlerInner>::bug::<&alloc[5d68931393aa826e]::string::String>
  12:     0x7f03e5a6d3d0 - <rustc_errors[2f6fbd56604b03c1]::Handler>::bug::<&alloc[5d68931393aa826e]::string::String>
  13:     0x7f03e5c1ec15 - rustc_middle[f1b3cfaf42ad4fa]::util::bug::opt_span_bug_fmt::<rustc_span[ae121019aa8de976]::span_encoding::Span>::{closure#0}
  14:     0x7f03e5c159fb - rustc_middle[f1b3cfaf42ad4fa]::ty::context::tls::with_opt::<rustc_middle[f1b3cfaf42ad4fa]::util::bug::opt_span_bug_fmt<rustc_span[ae121019aa8de976]::span_encoding::Span>::{closure#0}, ()>::{closure#0}
  15:     0x7f03e5c159a6 - rustc_middle[f1b3cfaf42ad4fa]::ty::context::tls::with_context_opt::<rustc_middle[f1b3cfaf42ad4fa]::ty::context::tls::with_opt<rustc_middle[f1b3cfaf42ad4fa]::util::bug::opt_span_bug_fmt<rustc_span[ae121019aa8de976]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  16:     0x7f03e5c1eb59 - rustc_middle[f1b3cfaf42ad4fa]::util::bug::opt_span_bug_fmt::<rustc_span[ae121019aa8de976]::span_encoding::Span>
  17:     0x7f03e3080715 - rustc_middle[f1b3cfaf42ad4fa]::util::bug::bug_fmt
  18:     0x7f03e42140a8 - <rustc_borrowck[33a6c798be802f14]::universal_regions::UniversalRegionIndices>::to_region_vid::{closure#0}
  19:     0x7f03e420eb53 - <rustc_borrowck[33a6c798be802f14]::universal_regions::UniversalRegions>::to_region_vid
  20:     0x7f03e43a465d - <rustc_borrowck[33a6c798be802f14]::region_infer::RegionInferenceContext>::eval_verify_bound
  21:     0x7f03e43a47e2 - <rustc_borrowck[33a6c798be802f14]::region_infer::RegionInferenceContext>::eval_verify_bound
  22:     0x7f03e43b0199 - <rustc_borrowck[33a6c798be802f14]::region_infer::RegionInferenceContext>::solve
  23:     0x7f03e41d657c - rustc_borrowck[33a6c798be802f14]::nll::compute_regions
  24:     0x7f03e438af70 - rustc_borrowck[33a6c798be802f14]::do_mir_borrowck
  25:     0x7f03e42e2fd0 - <rustc_infer[dec65dfcca68e892]::infer::InferCtxtBuilder>::enter::<rustc_middle[f1b3cfaf42ad4fa]::mir::query::BorrowCheckResult, rustc_borrowck[33a6c798be802f14]::mir_borrowck::{closure#0}>
  26:     0x7f03e4382aa7 - rustc_borrowck[33a6c798be802f14]::mir_borrowck
  27:     0x7f03e43499d6 - <rustc_borrowck[33a6c798be802f14]::provide::{closure#0} as core[bf60d7eb6f92b7b1]::ops::function::FnOnce<(rustc_middle[f1b3cfaf42ad4fa]::ty::context::TyCtxt, rustc_span[ae121019aa8de976]::def_id::LocalDefId)>>::call_once
  28:     0x7f03e4cdbcfd - rustc_query_system[b4ff286a7f9e7aa0]::query::plumbing::try_execute_query::<rustc_query_impl[9249130bb2729ac]::plumbing::QueryCtxt, rustc_query_system[b4ff286a7f9e7aa0]::query::caches::DefaultCache<rustc_span[ae121019aa8de976]::def_id::LocalDefId, &rustc_middle[f1b3cfaf42ad4fa]::mir::query::BorrowCheckResult>>
  29:     0x7f03e4da8408 - rustc_query_system[b4ff286a7f9e7aa0]::query::plumbing::get_query::<rustc_query_impl[9249130bb2729ac]::queries::mir_borrowck, rustc_query_impl[9249130bb2729ac]::plumbing::QueryCtxt>
  30:     0x7f03e48ef344 - <rustc_query_impl[9249130bb2729ac]::Queries as rustc_middle[f1b3cfaf42ad4fa]::ty::query::QueryEngine>::mir_borrowck
  31:     0x7f03e3274e4a - <rustc_middle[f1b3cfaf42ad4fa]::hir::map::Map>::par_body_owners::<rustc_interface[6c8603f7f10f0bc]::passes::analysis::{closure#2}::{closure#0}>
  32:     0x7f03e3257802 - <rustc_session[1389142e47c3b256]::session::Session>::time::<(), rustc_interface[6c8603f7f10f0bc]::passes::analysis::{closure#2}>
  33:     0x7f03e31fe48b - rustc_interface[6c8603f7f10f0bc]::passes::analysis
  34:     0x7f03e4d161a0 - rustc_query_system[b4ff286a7f9e7aa0]::query::plumbing::try_execute_query::<rustc_query_impl[9249130bb2729ac]::plumbing::QueryCtxt, rustc_query_system[b4ff286a7f9e7aa0]::query::caches::DefaultCache<(), core[bf60d7eb6f92b7b1]::result::Result<(), rustc_errors[2f6fbd56604b03c1]::ErrorGuaranteed>>>
  35:     0x7f03e4dfbb12 - rustc_query_system[b4ff286a7f9e7aa0]::query::plumbing::get_query::<rustc_query_impl[9249130bb2729ac]::queries::analysis, rustc_query_impl[9249130bb2729ac]::plumbing::QueryCtxt>
  36:     0x7f03e48d097e - <rustc_query_impl[9249130bb2729ac]::Queries as rustc_middle[f1b3cfaf42ad4fa]::ty::query::QueryEngine>::analysis
  37:     0x7f03e313e235 - <rustc_interface[6c8603f7f10f0bc]::passes::QueryContext>::enter::<rustc_driver[f632b19d4c81d0d4]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[bf60d7eb6f92b7b1]::result::Result<(), rustc_errors[2f6fbd56604b03c1]::ErrorGuaranteed>>
  38:     0x7f03e30efe9c - <rustc_interface[6c8603f7f10f0bc]::interface::Compiler>::enter::<rustc_driver[f632b19d4c81d0d4]::run_compiler::{closure#1}::{closure#2}, core[bf60d7eb6f92b7b1]::result::Result<core[bf60d7eb6f92b7b1]::option::Option<rustc_interface[6c8603f7f10f0bc]::queries::Linker>, rustc_errors[2f6fbd56604b03c1]::ErrorGuaranteed>>
  39:     0x7f03e30d7e88 - rustc_span[ae121019aa8de976]::with_source_map::<core[bf60d7eb6f92b7b1]::result::Result<(), rustc_errors[2f6fbd56604b03c1]::ErrorGuaranteed>, rustc_interface[6c8603f7f10f0bc]::interface::create_compiler_and_run<core[bf60d7eb6f92b7b1]::result::Result<(), rustc_errors[2f6fbd56604b03c1]::ErrorGuaranteed>, rustc_driver[f632b19d4c81d0d4]::run_compiler::{closure#1}>::{closure#1}>
  40:     0x7f03e30f1c79 - rustc_interface[6c8603f7f10f0bc]::interface::create_compiler_and_run::<core[bf60d7eb6f92b7b1]::result::Result<(), rustc_errors[2f6fbd56604b03c1]::ErrorGuaranteed>, rustc_driver[f632b19d4c81d0d4]::run_compiler::{closure#1}>
  41:     0x7f03e30d217f - <scoped_tls[957505d80a35b8ac]::ScopedKey<rustc_span[ae121019aa8de976]::SessionGlobals>>::set::<rustc_interface[6c8603f7f10f0bc]::interface::run_compiler<core[bf60d7eb6f92b7b1]::result::Result<(), rustc_errors[2f6fbd56604b03c1]::ErrorGuaranteed>, rustc_driver[f632b19d4c81d0d4]::run_compiler::{closure#1}>::{closure#0}, core[bf60d7eb6f92b7b1]::result::Result<(), rustc_errors[2f6fbd56604b03c1]::ErrorGuaranteed>>
  42:     0x7f03e30dbbe9 - std[56b592617f5022a2]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[6c8603f7f10f0bc]::util::run_in_thread_pool_with_globals<rustc_interface[6c8603f7f10f0bc]::interface::run_compiler<core[bf60d7eb6f92b7b1]::result::Result<(), rustc_errors[2f6fbd56604b03c1]::ErrorGuaranteed>, rustc_driver[f632b19d4c81d0d4]::run_compiler::{closure#1}>::{closure#0}, core[bf60d7eb6f92b7b1]::result::Result<(), rustc_errors[2f6fbd56604b03c1]::ErrorGuaranteed>>::{closure#0}, core[bf60d7eb6f92b7b1]::result::Result<(), rustc_errors[2f6fbd56604b03c1]::ErrorGuaranteed>>
  43:     0x7f03e3145cc9 - <<std[56b592617f5022a2]::thread::Builder>::spawn_unchecked_<rustc_interface[6c8603f7f10f0bc]::util::run_in_thread_pool_with_globals<rustc_interface[6c8603f7f10f0bc]::interface::run_compiler<core[bf60d7eb6f92b7b1]::result::Result<(), rustc_errors[2f6fbd56604b03c1]::ErrorGuaranteed>, rustc_driver[f632b19d4c81d0d4]::run_compiler::{closure#1}>::{closure#0}, core[bf60d7eb6f92b7b1]::result::Result<(), rustc_errors[2f6fbd56604b03c1]::ErrorGuaranteed>>::{closure#0}, core[bf60d7eb6f92b7b1]::result::Result<(), rustc_errors[2f6fbd56604b03c1]::ErrorGuaranteed>>::{closure#1} as core[bf60d7eb6f92b7b1]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  44:     0x7f03e273a303 - std::sys::unix::thread::Thread::new::thread_start::h547471f99416ecdc
  45:     0x7f03dcc86609 - start_thread
  46:     0x7f03e2599133 - clone
  47:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.64.0-nightly (bd5ae0ed4 2022-07-12) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [mir_borrowck] borrow-checking `foo::{closure#0}`
#1 [analysis] running analysis passes on this crate
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/higher-rank-trait-bounds/issue-59311.rs stdout ----
diff of stderr:

12 LL |     v.t(|| {});
14    |
14    |
-    = note: could not prove `[closure@$DIR/issue-59311.rs:17:9: 17:14] well-formed`
+    = note: could not prove `[closure@$DIR/issue-59311.rs:17:9: 17:11] well-formed`
17 error: higher-ranked lifetime error
18   --> $DIR/issue-59311.rs:17:9



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/higher-rank-trait-bounds/issue-59311/issue-59311.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args higher-rank-trait-bounds/issue-59311.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/higher-rank-trait-bounds/issue-59311.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/higher-rank-trait-bounds/issue-59311" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/higher-rank-trait-bounds/issue-59311/auxiliary"
stdout: none
--- stderr -------------------------------
error: higher-ranked lifetime error
   |
   |
LL |     v.t(|| {});
   |
   |
   = note: could not prove `[closure@/checkout/src/test/ui/higher-rank-trait-bounds/issue-59311.rs:17:9: 17:11] well-formed`
error: higher-ranked lifetime error
  --> /checkout/src/test/ui/higher-rank-trait-bounds/issue-59311.rs:17:5
   |
   |
LL |     v.t(|| {});
   |
   |
   = note: could not prove `[closure@/checkout/src/test/ui/higher-rank-trait-bounds/issue-59311.rs:17:9: 17:11] well-formed`
error: higher-ranked lifetime error
  --> /checkout/src/test/ui/higher-rank-trait-bounds/issue-59311.rs:17:9
   |
   |
LL |     v.t(|| {});
   |
   |
   = note: could not prove `for<'a> &'a V: 'static`
error: aborting due to 3 previous errors
------------------------------------------


