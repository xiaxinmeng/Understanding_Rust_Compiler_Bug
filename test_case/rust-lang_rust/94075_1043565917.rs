plain
Set({test::compiler/rustc_interface}) not skipped for "bootstrap::test::CrateLibrustc" -- not in [src/tools/tidy]
 finished in 6.044 seconds
Testing rustc_interface stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling rustc_interface v0.0.0 (/checkout/compiler/rustc_interface)
error: internal compiler error: /checkout/compiler/rustc_const_eval/src/interpret/operand.rs:78:42: Got a scalar pair where a scalar was expected
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1160:9
stack backtrace:
stack backtrace:
   0:     0x7fe4239cc71c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hd2a4427bc194f90c
   1:     0x7fe423a3b11e - core::fmt::write::h04f3cb9c5bd3369c
   2:     0x7fe4239bb6c1 - std::io::Write::write_fmt::h33bb4b10b956f418
   3:     0x7fe4239cc54b - std::sys_common::backtrace::print::h498d219314722bf2
   4:     0x7fe4239d0d14 - std::panicking::default_hook::{{closure}}::h953e478500a4611d
   5:     0x7fe4239d08f6 - std::panicking::default_hook::h8a205d841121ef74
   6:     0x7fe4244a3de1 - rustc_driver[9b70d8386d30c36e]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fe4239d1433 - std::panicking::rust_panic_with_hook::h25cd42f1b126a642
   8:     0x7fe426d67163 - std[3e634f71eb1fd3c7]::panicking::begin_panic::<rustc_errors[e2f3c60da98b9720]::ExplicitBug>::{closure#0}
   9:     0x7fe426d66fc6 - std[3e634f71eb1fd3c7]::sys_common::backtrace::__rust_end_short_backtrace::<std[3e634f71eb1fd3c7]::panicking::begin_panic<rustc_errors[e2f3c60da98b9720]::ExplicitBug>::{closure#0}, !>
  10:     0x7fe4243e0baf - std[3e634f71eb1fd3c7]::panicking::begin_panic::<rustc_errors[e2f3c60da98b9720]::ExplicitBug>
  11:     0x7fe426d9d41d - std[3e634f71eb1fd3c7]::panic::panic_any::<rustc_errors[e2f3c60da98b9720]::ExplicitBug>
  12:     0x7fe426d99578 - <rustc_errors[e2f3c60da98b9720]::HandlerInner>::bug
  13:     0x7fe426d972f0 - <rustc_errors[e2f3c60da98b9720]::Handler>::bug
  14:     0x7fe426a18743 - rustc_middle[2734fb24384d6128]::ty::context::tls::with_opt::<rustc_middle[2734fb24384d6128]::util::bug::opt_span_bug_fmt<rustc_span[59fba679e0fd44db]::span_encoding::Span>::{closure#0}, ()>
  15:     0x7fe426a18fb3 - rustc_middle[2734fb24384d6128]::util::bug::opt_span_bug_fmt::<rustc_span[59fba679e0fd44db]::span_encoding::Span>
  16:     0x7fe4243c069c - rustc_middle[2734fb24384d6128]::util::bug::bug_fmt
  17:     0x7fe4249b8029 - <rustc_const_eval[4348eb560319beab]::interpret::eval_context::InterpCx<rustc_mir_transform[429c201da36571ce]::const_prop::ConstPropMachine>>::read_scalar
  18:     0x7fe4248f7ce3 - <rustc_const_eval[4348eb560319beab]::interpret::validity::ValidityVisitor<rustc_mir_transform[429c201da36571ce]::const_prop::ConstPropMachine>>::read_scalar
  19:     0x7fe4248f7fb4 - <rustc_const_eval[4348eb560319beab]::interpret::validity::ValidityVisitor<rustc_mir_transform[429c201da36571ce]::const_prop::ConstPropMachine>>::try_visit_primitive
  20:     0x7fe4248fa8ba - <rustc_const_eval[4348eb560319beab]::interpret::validity::ValidityVisitor<rustc_mir_transform[429c201da36571ce]::const_prop::ConstPropMachine> as rustc_const_eval[4348eb560319beab]::interpret::visitor::ValueVisitor<rustc_mir_transform[429c201da36571ce]::const_prop::ConstPropMachine>>::visit_value
  21:     0x7fe4248ff2b0 - <rustc_const_eval[4348eb560319beab]::interpret::validity::ValidityVisitor<rustc_mir_transform[429c201da36571ce]::const_prop::ConstPropMachine> as rustc_const_eval[4348eb560319beab]::interpret::visitor::ValueVisitor<rustc_mir_transform[429c201da36571ce]::const_prop::ConstPropMachine>>::walk_value
  22:     0x7fe4248fa948 - <rustc_const_eval[4348eb560319beab]::interpret::validity::ValidityVisitor<rustc_mir_transform[429c201da36571ce]::const_prop::ConstPropMachine> as rustc_const_eval[4348eb560319beab]::interpret::visitor::ValueVisitor<rustc_mir_transform[429c201da36571ce]::const_prop::ConstPropMachine>>::visit_value
  23:     0x7fe4248ff2b0 - <rustc_const_eval[4348eb560319beab]::interpret::validity::ValidityVisitor<rustc_mir_transform[429c201da36571ce]::const_prop::ConstPropMachine> as rustc_const_eval[4348eb560319beab]::interpret::visitor::ValueVisitor<rustc_mir_transform[429c201da36571ce]::const_prop::ConstPropMachine>>::walk_value
  24:     0x7fe4248fa948 - <rustc_const_eval[4348eb560319beab]::interpret::validity::ValidityVisitor<rustc_mir_transform[429c201da36571ce]::const_prop::ConstPropMachine> as rustc_const_eval[4348eb560319beab]::interpret::visitor::ValueVisitor<rustc_mir_transform[429c201da36571ce]::const_prop::ConstPropMachine>>::visit_value
  25:     0x7fe4249009fd - <rustc_const_eval[4348eb560319beab]::interpret::validity::ValidityVisitor<rustc_mir_transform[429c201da36571ce]::const_prop::ConstPropMachine> as rustc_const_eval[4348eb560319beab]::interpret::visitor::ValueVisitor<rustc_mir_transform[429c201da36571ce]::const_prop::ConstPropMachine>>::walk_value
  26:     0x7fe4249cb4aa - <rustc_const_eval[4348eb560319beab]::interpret::eval_context::InterpCx<rustc_mir_transform[429c201da36571ce]::const_prop::ConstPropMachine>>::validate_operand_internal
  27:     0x7fe424977769 - <rustc_mir_transform[429c201da36571ce]::const_prop::ConstPropagator as rustc_middle[2734fb24384d6128]::mir::visit::MutVisitor>::visit_statement
  28:     0x7fe42497578f - <rustc_mir_transform[429c201da36571ce]::const_prop::ConstPropagator as rustc_middle[2734fb24384d6128]::mir::visit::MutVisitor>::visit_body
  29:     0x7fe42496e50c - <rustc_mir_transform[429c201da36571ce]::const_prop::ConstProp as rustc_middle[2734fb24384d6128]::mir::MirPass>::run_pass
  30:     0x7fe424a01515 - rustc_mir_transform[429c201da36571ce]::pass_manager::run_passes
  31:     0x7fe4249f6024 - rustc_mir_transform[429c201da36571ce]::optimized_mir
  32:     0x7fe4259bc6f5 - rustc_query_system[25acd371db95127e]::query::plumbing::try_execute_query::<rustc_query_impl[30e11d024c162bc1]::plumbing::QueryCtxt, rustc_query_system[25acd371db95127e]::query::caches::DefaultCache<rustc_span[59fba679e0fd44db]::def_id::DefId, &rustc_middle[2734fb24384d6128]::mir::Body>>
  33:     0x7fe425a5d4eb - rustc_query_system[25acd371db95127e]::query::plumbing::get_query::<rustc_query_impl[30e11d024c162bc1]::queries::optimized_mir, rustc_query_impl[30e11d024c162bc1]::plumbing::QueryCtxt>
  34:     0x7fe426aa942f - <rustc_middle[2734fb24384d6128]::ty::context::TyCtxt>::instance_mir
  35:     0x7fe424851e11 - rustc_monomorphize[f9586b9544bcf8a6]::collector::collect_neighbours
  36:     0x7fe424859b96 - rustc_data_structures[72ccbd85b4981db0]::stack::ensure_sufficient_stack::<(), rustc_monomorphize[f9586b9544bcf8a6]::collector::collect_items_rec::{closure#0}>
  37:     0x7fe424848887 - rustc_monomorphize[f9586b9544bcf8a6]::collector::collect_items_rec
  38:     0x7fe424848b3a - rustc_monomorphize[f9586b9544bcf8a6]::collector::collect_items_rec
  39:     0x7fe424848b3a - rustc_monomorphize[f9586b9544bcf8a6]::collector::collect_items_rec
  40:     0x7fe424865ec2 - <rustc_session[68dd9ce62d21586]::session::Session>::time::<(), rustc_monomorphize[f9586b9544bcf8a6]::collector::collect_crate_mono_items::{closure#1}>
  41:     0x7fe4248471bb - rustc_monomorphize[f9586b9544bcf8a6]::collector::collect_crate_mono_items
  42:     0x7fe4248863d3 - rustc_monomorphize[f9586b9544bcf8a6]::partitioning::collect_and_partition_mono_items
  43:     0x7fe425b8119b - rustc_data_structures[72ccbd85b4981db0]::stack::ensure_sufficient_stack::<(&std[3e634f71eb1fd3c7]::collections::hash::set::HashSet<rustc_span[59fba679e0fd44db]::def_id::DefId, core[ba5cb6be30a93795]::hash::BuildHasherDefault<rustc_hash[6f7973c56968f131]::FxHasher>>, &[rustc_middle[2734fb24384d6128]::mir::mono::CodegenUnit]), rustc_query_system[25acd371db95127e]::query::plumbing::execute_job<rustc_query_impl[30e11d024c162bc1]::plumbing::QueryCtxt, (), (&std[3e634f71eb1fd3c7]::collections::hash::set::HashSet<rustc_span[59fba679e0fd44db]::def_id::DefId, core[ba5cb6be30a93795]::hash::BuildHasherDefault<rustc_hash[6f7973c56968f131]::FxHasher>>, &[rustc_middle[2734fb24384d6128]::mir::mono::CodegenUnit])>::{closure#0}>
  44:     0x7fe4259da2f1 - rustc_query_system[25acd371db95127e]::query::plumbing::try_execute_query::<rustc_query_impl[30e11d024c162bc1]::plumbing::QueryCtxt, rustc_query_system[25acd371db95127e]::query::caches::DefaultCache<(), (&std[3e634f71eb1fd3c7]::collections::hash::set::HashSet<rustc_span[59fba679e0fd44db]::def_id::DefId, core[ba5cb6be30a93795]::hash::BuildHasherDefault<rustc_hash[6f7973c56968f131]::FxHasher>>, &[rustc_middle[2734fb24384d6128]::mir::mono::CodegenUnit])>>
  45:     0x7fe425ab0563 - rustc_query_system[25acd371db95127e]::query::plumbing::get_query::<rustc_query_impl[30e11d024c162bc1]::queries::collect_and_partition_mono_items, rustc_query_impl[30e11d024c162bc1]::plumbing::QueryCtxt>
  46:     0x7fe425b95a7e - <rustc_query_impl[30e11d024c162bc1]::Queries as rustc_middle[2734fb24384d6128]::ty::query::QueryEngine>::collect_and_partition_mono_items
  47:     0x7fe42469a768 - <rustc_codegen_llvm[94be95b1df3f112e]::LlvmCodegenBackend as rustc_codegen_ssa[99e0bc141d794c15]::traits::backend::CodegenBackend>::codegen_crate
  48:     0x7fe4245b51e8 - <rustc_session[68dd9ce62d21586]::session::Session>::time::<alloc[73103391425eea9e]::boxed::Box<dyn core[ba5cb6be30a93795]::any::Any>, rustc_interface[4208f5f86dbe46f0]::passes::start_codegen::{closure#0}>
  49:     0x7fe4246510cf - <rustc_interface[4208f5f86dbe46f0]::passes::QueryContext>::enter::<<rustc_interface[4208f5f86dbe46f0]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[ba5cb6be30a93795]::result::Result<alloc[73103391425eea9e]::boxed::Box<dyn core[ba5cb6be30a93795]::any::Any>, rustc_errors[e2f3c60da98b9720]::ErrorReported>>
  50:     0x7fe42462ef7e - <rustc_interface[4208f5f86dbe46f0]::queries::Queries>::ongoing_codegen
  51:     0x7fe424455345 - <rustc_interface[4208f5f86dbe46f0]::interface::Compiler>::enter::<rustc_driver[9b70d8386d30c36e]::run_compiler::{closure#1}::{closure#2}, core[ba5cb6be30a93795]::result::Result<core[ba5cb6be30a93795]::option::Option<rustc_interface[4208f5f86dbe46f0]::queries::Linker>, rustc_errors[e2f3c60da98b9720]::ErrorReported>>
  52:     0x7fe424492bb9 - rustc_span[59fba679e0fd44db]::with_source_map::<core[ba5cb6be30a93795]::result::Result<(), rustc_errors[e2f3c60da98b9720]::ErrorReported>, rustc_interface[4208f5f86dbe46f0]::interface::create_compiler_and_run<core[ba5cb6be30a93795]::result::Result<(), rustc_errors[e2f3c60da98b9720]::ErrorReported>, rustc_driver[9b70d8386d30c36e]::run_compiler::{closure#1}>::{closure#1}>
  53:     0x7fe42445360a - rustc_interface[4208f5f86dbe46f0]::interface::create_compiler_and_run::<core[ba5cb6be30a93795]::result::Result<(), rustc_errors[e2f3c60da98b9720]::ErrorReported>, rustc_driver[9b70d8386d30c36e]::run_compiler::{closure#1}>
  54:     0x7fe42443074f - <scoped_tls[9ef8146b5f47b671]::ScopedKey<rustc_span[59fba679e0fd44db]::SessionGlobals>>::set::<rustc_interface[4208f5f86dbe46f0]::interface::run_compiler<core[ba5cb6be30a93795]::result::Result<(), rustc_errors[e2f3c60da98b9720]::ErrorReported>, rustc_driver[9b70d8386d30c36e]::run_compiler::{closure#1}>::{closure#0}, core[ba5cb6be30a93795]::result::Result<(), rustc_errors[e2f3c60da98b9720]::ErrorReported>>
  55:     0x7fe42442df39 - std[3e634f71eb1fd3c7]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[4208f5f86dbe46f0]::util::run_in_thread_pool_with_globals<rustc_interface[4208f5f86dbe46f0]::interface::run_compiler<core[ba5cb6be30a93795]::result::Result<(), rustc_errors[e2f3c60da98b9720]::ErrorReported>, rustc_driver[9b70d8386d30c36e]::run_compiler::{closure#1}>::{closure#0}, core[ba5cb6be30a93795]::result::Result<(), rustc_errors[e2f3c60da98b9720]::ErrorReported>>::{closure#0}, core[ba5cb6be30a93795]::result::Result<(), rustc_errors[e2f3c60da98b9720]::ErrorReported>>
  56:     0x7fe424495f21 - std[3e634f71eb1fd3c7]::panicking::try::<core[ba5cb6be30a93795]::result::Result<(), rustc_errors[e2f3c60da98b9720]::ErrorReported>, core[ba5cb6be30a93795]::panic::unwind_safe::AssertUnwindSafe<<std[3e634f71eb1fd3c7]::thread::Builder>::spawn_unchecked_<rustc_interface[4208f5f86dbe46f0]::util::run_in_thread_pool_with_globals<rustc_interface[4208f5f86dbe46f0]::interface::run_compiler<core[ba5cb6be30a93795]::result::Result<(), rustc_errors[e2f3c60da98b9720]::ErrorReported>, rustc_driver[9b70d8386d30c36e]::run_compiler::{closure#1}>::{closure#0}, core[ba5cb6be30a93795]::result::Result<(), rustc_errors[e2f3c60da98b9720]::ErrorReported>>::{closure#0}, core[ba5cb6be30a93795]::result::Result<(), rustc_errors[e2f3c60da98b9720]::ErrorReported>>::{closure#1}::{closure#0}>>
  57:     0x7fe424430422 - <<std[3e634f71eb1fd3c7]::thread::Builder>::spawn_unchecked_<rustc_interface[4208f5f86dbe46f0]::util::run_in_thread_pool_with_globals<rustc_interface[4208f5f86dbe46f0]::interface::run_compiler<core[ba5cb6be30a93795]::result::Result<(), rustc_errors[e2f3c60da98b9720]::ErrorReported>, rustc_driver[9b70d8386d30c36e]::run_compiler::{closure#1}>::{closure#0}, core[ba5cb6be30a93795]::result::Result<(), rustc_errors[e2f3c60da98b9720]::ErrorReported>>::{closure#0}, core[ba5cb6be30a93795]::result::Result<(), rustc_errors[e2f3c60da98b9720]::ErrorReported>>::{closure#1} as core[ba5cb6be30a93795]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  58:     0x7fe4239e0493 - std::sys::unix::thread::Thread::new::thread_start::ha58e33afaf9bcd1a
  59:     0x7fe41dd52609 - start_thread
  60:     0x7fe423849293 - clone
  61:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.60.0-nightly (8e0e5ce39 2022-02-17) running on x86_64-unknown-linux-gnu

note: compiler flags: -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=on -C symbol-mangling-version=v0 -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -Z tls-model=initial-exec -Z unstable-options -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [optimized_mir] optimizing MIR for `tests::test_native_libs_tracking_hash_different_order`
#0 [optimized_mir] optimizing MIR for `tests::test_native_libs_tracking_hash_different_order`
#1 [collect_and_partition_mono_items] collect_and_partition_mono_items
error: could not compile `rustc_interface`
Build completed unsuccessfully in 0:24:42
