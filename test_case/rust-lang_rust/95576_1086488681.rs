plain
---- [ui] ui/mir/ssa-analysis-regression-50041.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/mir/ssa-analysis-regression-50041.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/ssa-analysis-regression-50041" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "mir-opt-level=4" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/ssa-analysis-regression-50041/auxiliary"
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
stdout: none
--- stderr -------------------------------
error: internal compiler error: /checkout/compiler/rustc_const_eval/src/interpret/eval_context.rs:397:21: expected type differs from actual type.
                                expected: *mut Foo<usize>
                                actual: (*mut Foo<usize>,)
   |
   |
LL | pub fn foo(a: Option<Box<Foo<usize>>>) -> usize {

thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1223:9
stack backtrace:
stack backtrace:
   0:     0x7fe8592ed16c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h5b0681bf800fed3d
   1:     0x7fe85935356f - core::fmt::write::h67e0d4a2696470aa
   2:     0x7fe8592dcab1 - std::io::Write::write_fmt::hddfbba98eefdd1af
   3:     0x7fe8592ecf8b - std::sys_common::backtrace::print::h0a2e002ce284547b
   4:     0x7fe8592f09a4 - std::panicking::default_hook::{{closure}}::h3e78f960e83c2490
   5:     0x7fe8592f055d - std::panicking::default_hook::h007aa76209338927
   6:     0x7fe859d81c81 - rustc_driver[b3bb80e0746ef1e1]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fe8592f108b - std::panicking::rust_panic_with_hook::hb4eddaf1522d9b08
   8:     0x7fe85a33a293 - std[b0a400a388a79038]::panicking::begin_panic::<rustc_errors[c322bfdc5abfa6b3]::ExplicitBug>::{closure#0}
   9:     0x7fe85a3313e6 - std[b0a400a388a79038]::sys_common::backtrace::__rust_end_short_backtrace::<std[b0a400a388a79038]::panicking::begin_panic<rustc_errors[c322bfdc5abfa6b3]::ExplicitBug>::{closure#0}, !>
  10:     0x7fe859a6a8ef - std[b0a400a388a79038]::panicking::begin_panic::<rustc_errors[c322bfdc5abfa6b3]::ExplicitBug>
  11:     0x7fe85a4b959d - std[b0a400a388a79038]::panic::panic_any::<rustc_errors[c322bfdc5abfa6b3]::ExplicitBug>
  12:     0x7fe85a4b796a - <rustc_errors[c322bfdc5abfa6b3]::HandlerInner>::span_bug::<rustc_span[ef02de9b221ac0d0]::span_encoding::Span>
  13:     0x7fe85a4b7780 - <rustc_errors[c322bfdc5abfa6b3]::Handler>::span_bug::<rustc_span[ef02de9b221ac0d0]::span_encoding::Span>
  14:     0x7fe85a477029 - rustc_middle[e9638398fcbbc397]::util::bug::opt_span_bug_fmt::<rustc_span[ef02de9b221ac0d0]::span_encoding::Span>::{closure#0}
  15:     0x7fe85a47710b - rustc_middle[e9638398fcbbc397]::ty::context::tls::with_opt::<rustc_middle[e9638398fcbbc397]::util::bug::opt_span_bug_fmt<rustc_span[ef02de9b221ac0d0]::span_encoding::Span>::{closure#0}, ()>::{closure#0}
  16:     0x7fe85a47420c - rustc_middle[e9638398fcbbc397]::ty::context::tls::with_opt::<rustc_middle[e9638398fcbbc397]::util::bug::opt_span_bug_fmt<rustc_span[ef02de9b221ac0d0]::span_encoding::Span>::{closure#0}, ()>
  17:     0x7fe85a473fb9 - rustc_middle[e9638398fcbbc397]::util::bug::opt_span_bug_fmt::<rustc_span[ef02de9b221ac0d0]::span_encoding::Span>
  18:     0x7fe859a6c043 - rustc_middle[e9638398fcbbc397]::util::bug::span_bug_fmt::<rustc_span[ef02de9b221ac0d0]::span_encoding::Span>
  19:     0x7fe85a42399c - <rustc_const_eval[3264850f551b6304]::interpret::eval_context::InterpCx<rustc_mir_transform[8765ad7c5956c86f]::const_prop::ConstPropMachine>>::access_local
  20:     0x7fe85a425ac8 - <rustc_const_eval[3264850f551b6304]::interpret::eval_context::InterpCx<rustc_mir_transform[8765ad7c5956c86f]::const_prop::ConstPropMachine>>::eval_place_to_op
  21:     0x7fe85a423c61 - <rustc_const_eval[3264850f551b6304]::interpret::eval_context::InterpCx<rustc_mir_transform[8765ad7c5956c86f]::const_prop::ConstPropMachine>>::eval_operand
  22:     0x7fe85a40b3a3 - <rustc_const_eval[3264850f551b6304]::interpret::eval_context::InterpCx<rustc_mir_transform[8765ad7c5956c86f]::const_prop::ConstPropMachine>>::eval_rvalue_into_place
  23:     0x7fe85a2dfbe6 - <rustc_mir_transform[8765ad7c5956c86f]::const_prop::ConstPropagator as rustc_middle[e9638398fcbbc397]::mir::visit::MutVisitor>::visit_statement
  24:     0x7fe85a2df179 - <rustc_mir_transform[8765ad7c5956c86f]::const_prop::ConstPropagator as rustc_middle[e9638398fcbbc397]::mir::visit::MutVisitor>::visit_body
  25:     0x7fe85a2dbc06 - <rustc_mir_transform[8765ad7c5956c86f]::const_prop::ConstProp as rustc_middle[e9638398fcbbc397]::mir::MirPass>::run_pass
  26:     0x7fe85a4adee5 - rustc_mir_transform[8765ad7c5956c86f]::pass_manager::run_passes
  27:     0x7fe85a49364a - rustc_mir_transform[8765ad7c5956c86f]::optimized_mir
  28:     0x7fe85b4578d8 - rustc_query_system[74282cc5d4c6e97f]::query::plumbing::try_execute_query::<rustc_query_impl[e08f0bbe355a447d]::plumbing::QueryCtxt, rustc_query_system[74282cc5d4c6e97f]::query::caches::DefaultCache<rustc_span[ef02de9b221ac0d0]::def_id::DefId, &rustc_middle[e9638398fcbbc397]::mir::Body>>
  29:     0x7fe85b4feac3 - rustc_query_system[74282cc5d4c6e97f]::query::plumbing::get_query::<rustc_query_impl[e08f0bbe355a447d]::queries::optimized_mir, rustc_query_impl[e08f0bbe355a447d]::plumbing::QueryCtxt>
  30:     0x7fe85c4d930c - <rustc_middle[e9638398fcbbc397]::ty::context::TyCtxt>::instance_mir
  31:     0x7fe85a2890f1 - rustc_monomorphize[d26e1feedb776fe3]::collector::collect_neighbours
  32:     0x7fe85a280551 - rustc_monomorphize[d26e1feedb776fe3]::collector::collect_items_rec
  33:     0x7fe85a289f1f - <rustc_session[8fa7df91f64996e8]::session::Session>::time::<(), rustc_monomorphize[d26e1feedb776fe3]::collector::collect_crate_mono_items::{closure#1}>
  34:     0x7fe85a27ed4f - rustc_monomorphize[d26e1feedb776fe3]::collector::collect_crate_mono_items
  35:     0x7fe85a2a62b0 - rustc_monomorphize[d26e1feedb776fe3]::partitioning::collect_and_partition_mono_items
  36:     0x7fe85b479e4a - rustc_query_system[74282cc5d4c6e97f]::query::plumbing::try_execute_query::<rustc_query_impl[e08f0bbe355a447d]::plumbing::QueryCtxt, rustc_query_system[74282cc5d4c6e97f]::query::caches::DefaultCache<(), (&std[b0a400a388a79038]::collections::hash::set::HashSet<rustc_span[ef02de9b221ac0d0]::def_id::DefId, core[15c243b7857cffbc]::hash::BuildHasherDefault<rustc_hash[1dcadffd26666e4a]::FxHasher>>, &[rustc_middle[e9638398fcbbc397]::mir::mono::CodegenUnit])>>
  37:     0x7fe85b55d725 - rustc_query_system[74282cc5d4c6e97f]::query::plumbing::get_query::<rustc_query_impl[e08f0bbe355a447d]::queries::collect_and_partition_mono_items, rustc_query_impl[e08f0bbe355a447d]::plumbing::QueryCtxt>
  38:     0x7fe85b5f6152 - <rustc_query_impl[e08f0bbe355a447d]::Queries as rustc_middle[e9638398fcbbc397]::ty::query::QueryEngine>::collect_and_partition_mono_items
  39:     0x7fe85a063676 - rustc_codegen_ssa[56fb79862b9bddd5]::base::codegen_crate::<rustc_codegen_llvm[c228f9fa6f877d2a]::LlvmCodegenBackend>
  40:     0x7fe85a09f219 - <rustc_codegen_llvm[c228f9fa6f877d2a]::LlvmCodegenBackend as rustc_codegen_ssa[56fb79862b9bddd5]::traits::backend::CodegenBackend>::codegen_crate
  41:     0x7fe859ed7841 - <rustc_session[8fa7df91f64996e8]::session::Session>::time::<alloc[1887a6912fe15a0f]::boxed::Box<dyn core[15c243b7857cffbc]::any::Any>, rustc_interface[e1334eb5893ba77d]::passes::start_codegen::{closure#0}>
  42:     0x7fe859f84aee - <rustc_interface[e1334eb5893ba77d]::passes::QueryContext>::enter::<<rustc_interface[e1334eb5893ba77d]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[15c243b7857cffbc]::result::Result<alloc[1887a6912fe15a0f]::boxed::Box<dyn core[15c243b7857cffbc]::any::Any>, rustc_errors[c322bfdc5abfa6b3]::ErrorGuaranteed>>
  43:     0x7fe859f1d4be - <rustc_interface[e1334eb5893ba77d]::queries::Queries>::ongoing_codegen
  44:     0x7fe859de0978 - <rustc_interface[e1334eb5893ba77d]::interface::Compiler>::enter::<rustc_driver[b3bb80e0746ef1e1]::run_compiler::{closure#1}::{closure#2}, core[15c243b7857cffbc]::result::Result<core[15c243b7857cffbc]::option::Option<rustc_interface[e1334eb5893ba77d]::queries::Linker>, rustc_errors[c322bfdc5abfa6b3]::ErrorGuaranteed>>
  45:     0x7fe859df5c3b - rustc_span[ef02de9b221ac0d0]::with_source_map::<core[15c243b7857cffbc]::result::Result<(), rustc_errors[c322bfdc5abfa6b3]::ErrorGuaranteed>, rustc_interface[e1334eb5893ba77d]::interface::create_compiler_and_run<core[15c243b7857cffbc]::result::Result<(), rustc_errors[c322bfdc5abfa6b3]::ErrorGuaranteed>, rustc_driver[b3bb80e0746ef1e1]::run_compiler::{closure#1}>::{closure#1}>
  46:     0x7fe859de17a3 - <scoped_tls[26d213cf73668105]::ScopedKey<rustc_span[ef02de9b221ac0d0]::SessionGlobals>>::set::<rustc_interface[e1334eb5893ba77d]::interface::run_compiler<core[15c243b7857cffbc]::result::Result<(), rustc_errors[c322bfdc5abfa6b3]::ErrorGuaranteed>, rustc_driver[b3bb80e0746ef1e1]::run_compiler::{closure#1}>::{closure#0}, core[15c243b7857cffbc]::result::Result<(), rustc_errors[c322bfdc5abfa6b3]::ErrorGuaranteed>>
  47:     0x7fe859da8d39 - std[b0a400a388a79038]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[e1334eb5893ba77d]::util::run_in_thread_pool_with_globals<rustc_interface[e1334eb5893ba77d]::interface::run_compiler<core[15c243b7857cffbc]::result::Result<(), rustc_errors[c322bfdc5abfa6b3]::ErrorGuaranteed>, rustc_driver[b3bb80e0746ef1e1]::run_compiler::{closure#1}>::{closure#0}, core[15c243b7857cffbc]::result::Result<(), rustc_errors[c322bfdc5abfa6b3]::ErrorGuaranteed>>::{closure#0}, core[15c243b7857cffbc]::result::Result<(), rustc_errors[c322bfdc5abfa6b3]::ErrorGuaranteed>>
  48:     0x7fe859e0ae59 - <<std[b0a400a388a79038]::thread::Builder>::spawn_unchecked_<rustc_interface[e1334eb5893ba77d]::util::run_in_thread_pool_with_globals<rustc_interface[e1334eb5893ba77d]::interface::run_compiler<core[15c243b7857cffbc]::result::Result<(), rustc_errors[c322bfdc5abfa6b3]::ErrorGuaranteed>, rustc_driver[b3bb80e0746ef1e1]::run_compiler::{closure#1}>::{closure#0}, core[15c243b7857cffbc]::result::Result<(), rustc_errors[c322bfdc5abfa6b3]::ErrorGuaranteed>>::{closure#0}, core[15c243b7857cffbc]::result::Result<(), rustc_errors[c322bfdc5abfa6b3]::ErrorGuaranteed>>::{closure#1} as core[15c243b7857cffbc]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  49:     0x7fe8592fc903 - std::sys::unix::thread::Thread::new::thread_start::h37bf4527ee14e578
  50:     0x7fe85384b609 - start_thread
  51:     0x7fe85915e163 - clone
  52:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.61.0-nightly (ea19fb87d 2022-04-02) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0 -Z mir-opt-level=4
query stack during panic:
#0 [optimized_mir] optimizing MIR for `foo`
#0 [optimized_mir] optimizing MIR for `foo`
#1 [collect_and_partition_mono_items] collect_and_partition_mono_items
error: aborting due to previous error
------------------------------------------


