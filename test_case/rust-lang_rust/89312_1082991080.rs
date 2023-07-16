`
error: internal compiler error: compiler/rustc_trait_selection/src/traits/codegen.rs:98:13: VecMap(
                                    [
                                        (
                                            OpaqueTypeKey {
                                                def_id: DefId(0:7 ~ issue_89312[27c2]::Alias::{opaque#0}),
                                                substs: [
                                                    '_#0r,
                                                ],
                                            },
                                            OpaqueTypeDecl {
                                                hidden_type: OpaqueHiddenType {
                                                    span: no-location (#0),
                                                    ty: &S,
                                                },
                                                origin: TyAlias,
                                            },
                                        ),
                                    ],
                                )

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1279:9
stack backtrace:
   0:     0x7f882ea9d4bd - std::backtrace_rs::backtrace::libunwind::trace::h5518e6be1e7d4fed
                               at /rustc/f132bcf3bdf6d3ff9be7d02e8d0088b99007cd5e/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7f882ea9d4bd - std::backtrace_rs::backtrace::trace_unsynchronized::h15fd2d8832f91324
                               at /rustc/f132bcf3bdf6d3ff9be7d02e8d0088b99007cd5e/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7f882ea9d4bd - std::sys_common::backtrace::_print_fmt::h85c9b6dc9f59e211
                               at /rustc/f132bcf3bdf6d3ff9be7d02e8d0088b99007cd5e/library/std/src/sys_common/backtrace.rs:66:5
   3:     0x7f882ea9d4bd - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h0b5d7dc6ae932a3c
                               at /rustc/f132bcf3bdf6d3ff9be7d02e8d0088b99007cd5e/library/std/src/sys_common/backtrace.rs:45:22
   4:     0x7f882eaf732c - core::fmt::write::h5ceb17dd50b3b2a6
                               at /rustc/f132bcf3bdf6d3ff9be7d02e8d0088b99007cd5e/library/core/src/fmt/mod.rs:1190:17
   5:     0x7f882ea8ea11 - std::io::Write::write_fmt::h3447798faac22fb1
                               at /rustc/f132bcf3bdf6d3ff9be7d02e8d0088b99007cd5e/library/std/src/io/mod.rs:1655:15
   6:     0x7f882eaa05a5 - std::sys_common::backtrace::_print::h9a41a14aa8472d67
                               at /rustc/f132bcf3bdf6d3ff9be7d02e8d0088b99007cd5e/library/std/src/sys_common/backtrace.rs:48:5
   7:     0x7f882eaa05a5 - std::sys_common::backtrace::print::had8dced11e076d04
                               at /rustc/f132bcf3bdf6d3ff9be7d02e8d0088b99007cd5e/library/std/src/sys_common/backtrace.rs:35:9
   8:     0x7f882eaa05a5 - std::panicking::default_hook::{{closure}}::hf5b7507853788630
                               at /rustc/f132bcf3bdf6d3ff9be7d02e8d0088b99007cd5e/library/std/src/panicking.rs:295:22
   9:     0x7f882eaa0259 - std::panicking::default_hook::hae8fcf8cc0c48e73
                               at /rustc/f132bcf3bdf6d3ff9be7d02e8d0088b99007cd5e/library/std/src/panicking.rs:314:9
  10:     0x7f882f2c7181 - rustc_driver[534d81c5b56eff30]::DEFAULT_HOOK::{closure#0}::{closure#0}
  11:     0x7f882eaa0cf0 - std::panicking::rust_panic_with_hook::ha56b4b8e979660f2
                               at /rustc/f132bcf3bdf6d3ff9be7d02e8d0088b99007cd5e/library/std/src/panicking.rs:702:17
  12:     0x7f88303654a1 - std[6abc34ddb203e24a]::panicking::begin_panic::<rustc_errors[60cffa1887773b15]::ExplicitBug>::{closure#0}
  13:     0x7f8830365126 - std[6abc34ddb203e24a]::sys_common::backtrace::__rust_end_short_backtrace::<std[6abc34ddb203e24a]::panicking::begin_panic<rustc_errors[60cffa1887773b15]::ExplicitBug>::{closure#0}, !>
  14:     0x7f883036c32f - std[6abc34ddb203e24a]::panicking::begin_panic::<rustc_errors[60cffa1887773b15]::ExplicitBug>
  15:     0x7f8830378586 - std[6abc34ddb203e24a]::panic::panic_any::<rustc_errors[60cffa1887773b15]::ExplicitBug>
  16:     0x7f883037a683 - <rustc_errors[60cffa1887773b15]::HandlerInner>::bug
  17:     0x7f883037a0e0 - <rustc_errors[60cffa1887773b15]::Handler>::bug
  18:     0x7f88302b9f56 - rustc_middle[2e9ecac3ac57679a]::ty::context::tls::with_opt::<rustc_middle[2e9ecac3ac57679a]::util::bug::opt_span_bug_fmt<rustc_span[c59ac07462d462be]::span_encoding::Span>::{closure#0}, ()>
  19:     0x7f88302bcfe6 - rustc_middle[2e9ecac3ac57679a]::util::bug::opt_span_bug_fmt::<rustc_span[c59ac07462d462be]::span_encoding::Span>
  20:     0x7f88302bcf53 - rustc_middle[2e9ecac3ac57679a]::util::bug::bug_fmt
  21:     0x7f8830e6c6f1 - <rustc_infer[19cbb247616e1fca]::infer::InferCtxtBuilder>::enter::<core[98647bc572ccb271]::result::Result<&rustc_middle[2e9ecac3ac57679a]::traits::ImplSource<()>, rustc_errors[60cffa1887773b15]::ErrorGuaranteed>, rustc_trait_selection[b2f2a884b31fcfb9]::traits::codegen::codegen_fulfill_obligation::{closure#0}>
  22:     0x7f8830ef987e - rustc_trait_selection[b2f2a884b31fcfb9]::traits::codegen::codegen_fulfill_obligation
  23:     0x7f8830bb3b59 - rustc_query_system[4eeaba550dde3146]::query::plumbing::try_execute_query::<rustc_query_impl[b8a580452fca6ad5]::plumbing::QueryCtxt, rustc_query_system[4eeaba550dde3146]::query::caches::DefaultCache<(rustc_middle[2e9ecac3ac57679a]::ty::ParamEnv, rustc_middle[2e9ecac3ac57679a]::ty::sty::Binder<rustc_middle[2e9ecac3ac57679a]::ty::sty::TraitRef>), core[98647bc572ccb271]::result::Result<&rustc_middle[2e9ecac3ac57679a]::traits::ImplSource<()>, rustc_errors[60cffa1887773b15]::ErrorGuaranteed>>>
  24:     0x7f8830c51785 - <rustc_query_impl[b8a580452fca6ad5]::Queries as rustc_middle[2e9ecac3ac57679a]::ty::query::QueryEngine>::codegen_fulfill_obligation
  25:     0x7f88307ada5a - rustc_ty_utils[37785157efa9b39b]::instance::inner_resolve_instance
  26:     0x7f88307acb4f - rustc_ty_utils[37785157efa9b39b]::instance::resolve_instance
  27:     0x7f8830bc52bc - rustc_query_system[4eeaba550dde3146]::query::plumbing::get_query::<rustc_query_impl[b8a580452fca6ad5]::queries::resolve_instance, rustc_query_impl[b8a580452fca6ad5]::plumbing::QueryCtxt>
  28:     0x7f8830c53888 - <rustc_query_impl[b8a580452fca6ad5]::Queries as rustc_middle[2e9ecac3ac57679a]::ty::query::QueryEngine>::resolve_instance
  29:     0x7f8831049393 - <rustc_middle[2e9ecac3ac57679a]::ty::instance::Instance>::resolve
  30:     0x7f8830584dc2 - rustc_monomorphize[617a6ab36e6002f]::collector::collect_neighbours
  31:     0x7f8830580064 - rustc_monomorphize[617a6ab36e6002f]::collector::collect_items_rec
  32:     0x7f88305801c8 - rustc_monomorphize[617a6ab36e6002f]::collector::collect_items_rec
  33:     0x7f88312ed8eb - <rustc_session[c60934eb1ab24f8d]::session::Session>::time::<(), rustc_monomorphize[617a6ab36e6002f]::collector::collect_crate_mono_items::{closure#1}>
  34:     0x7f88312ddc2b - rustc_monomorphize[617a6ab36e6002f]::collector::collect_crate_mono_items
  35:     0x7f88312e4766 - rustc_monomorphize[617a6ab36e6002f]::partitioning::collect_and_partition_mono_items
  36:     0x7f88316397a0 - rustc_query_system[4eeaba550dde3146]::query::plumbing::try_execute_query::<rustc_query_impl[b8a580452fca6ad5]::plumbing::QueryCtxt, rustc_query_system[4eeaba550dde3146]::query::caches::DefaultCache<(), (&std[6abc34ddb203e24a]::collections::hash::set::HashSet<rustc_span[c59ac07462d462be]::def_id::DefId, core[98647bc572ccb271]::hash::BuildHasherDefault<rustc_hash[6368fc160eb0a0c8]::FxHasher>>, &[rustc_middle[2e9ecac3ac57679a]::mir::mono::CodegenUnit])>>
  37:     0x7f883166e7e5 - rustc_query_system[4eeaba550dde3146]::query::plumbing::get_query::<rustc_query_impl[b8a580452fca6ad5]::queries::collect_and_partition_mono_items, rustc_query_impl[b8a580452fca6ad5]::plumbing::QueryCtxt>
  38:     0x7f883171a5a2 - <rustc_query_impl[b8a580452fca6ad5]::Queries as rustc_middle[2e9ecac3ac57679a]::ty::query::QueryEngine>::collect_and_partition_mono_items
  39:     0x7f883118b9b0 - <rustc_codegen_llvm[778abd054f5e5b38]::LlvmCodegenBackend as rustc_codegen_ssa[a5e4443dd651f1d7]::traits::backend::CodegenBackend>::codegen_crate
  40:     0x7f88311708a7 - <rustc_session[c60934eb1ab24f8d]::session::Session>::time::<alloc[6c9db84e1ceb9229]::boxed::Box<dyn core[98647bc572ccb271]::any::Any>, rustc_interface[4f22add5d71eaea7]::passes::start_codegen::{closure#0}>
  41:     0x7f883115f408 - <rustc_interface[4f22add5d71eaea7]::passes::QueryContext>::enter::<<rustc_interface[4f22add5d71eaea7]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[98647bc572ccb271]::result::Result<alloc[6c9db84e1ceb9229]::boxed::Box<dyn core[98647bc572ccb271]::any::Any>, rustc_errors[60cffa1887773b15]::ErrorGuaranteed>>
  42:     0x7f88311550af - <rustc_interface[4f22add5d71eaea7]::queries::Queries>::ongoing_codegen
  43:     0x7f8831119eab - <rustc_interface[4f22add5d71eaea7]::interface::Compiler>::enter::<rustc_driver[534d81c5b56eff30]::run_compiler::{closure#1}::{closure#2}, core[98647bc572ccb271]::result::Result<core[98647bc572ccb271]::option::Option<rustc_interface[4f22add5d71eaea7]::queries::Linker>, rustc_errors[60cffa1887773b15]::ErrorGuaranteed>>
  44:     0x7f883112cebf - rustc_span[c59ac07462d462be]::with_source_map::<core[98647bc572ccb271]::result::Result<(), rustc_errors[60cffa1887773b15]::ErrorGuaranteed>, rustc_interface[4f22add5d71eaea7]::interface::create_compiler_and_run<core[98647bc572ccb271]::result::Result<(), rustc_errors[60cffa1887773b15]::ErrorGuaranteed>, rustc_driver[534d81c5b56eff30]::run_compiler::{closure#1}>::{closure#1}>
  45:     0x7f883111aaf4 - rustc_interface[4f22add5d71eaea7]::interface::create_compiler_and_run::<core[98647bc572ccb271]::result::Result<(), rustc_errors[60cffa1887773b15]::ErrorGuaranteed>, rustc_driver[534d81c5b56eff30]::run_compiler::{closure#1}>
  46:     0x7f8831117862 - <scoped_tls[3f8091782b19ddb8]::ScopedKey<rustc_span[c59ac07462d462be]::SessionGlobals>>::set::<rustc_interface[4f22add5d71eaea7]::interface::run_compiler<core[98647bc572ccb271]::result::Result<(), rustc_errors[60cffa1887773b15]::ErrorGuaranteed>, rustc_driver[534d81c5b56eff30]::run_compiler::{closure#1}>::{closure#0}, core[98647bc572ccb271]::result::Result<(), rustc_errors[60cffa1887773b15]::ErrorGuaranteed>>
  47:     0x7f8831115bbf - std[6abc34ddb203e24a]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[4f22add5d71eaea7]::util::run_in_thread_pool_with_globals<rustc_interface[4f22add5d71eaea7]::interface::run_compiler<core[98647bc572ccb271]::result::Result<(), rustc_errors[60cffa1887773b15]::ErrorGuaranteed>, rustc_driver[534d81c5b56eff30]::run_compiler::{closure#1}>::{closure#0}, core[98647bc572ccb271]::result::Result<(), rustc_errors[60cffa1887773b15]::ErrorGuaranteed>>::{closure#0}, core[98647bc572ccb271]::result::Result<(), rustc_errors[60cffa1887773b15]::ErrorGuaranteed>>
  48:     0x7f883112de52 - <<std[6abc34ddb203e24a]::thread::Builder>::spawn_unchecked_<rustc_interface[4f22add5d71eaea7]::util::run_in_thread_pool_with_globals<rustc_interface[4f22add5d71eaea7]::interface::run_compiler<core[98647bc572ccb271]::result::Result<(), rustc_errors[60cffa1887773b15]::ErrorGuaranteed>, rustc_driver[534d81c5b56eff30]::run_compiler::{closure#1}>::{closure#0}, core[98647bc572ccb271]::result::Result<(), rustc_errors[60cffa1887773b15]::ErrorGuaranteed>>::{closure#0}, core[98647bc572ccb271]::result::Result<(), rustc_errors[60cffa1887773b15]::ErrorGuaranteed>>::{closure#1} as core[98647bc572ccb271]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  49:     0x7f882eaaaee3 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h2886a9bccd9e4f41
                               at /rustc/f132bcf3bdf6d3ff9be7d02e8d0088b99007cd5e/library/alloc/src/boxed.rs:1861:9
  50:     0x7f882eaaaee3 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h24fe0a3ccb1a9186
                               at /rustc/f132bcf3bdf6d3ff9be7d02e8d0088b99007cd5e/library/alloc/src/boxed.rs:1861:9
  51:     0x7f882eaaaee3 - std::sys::unix::thread::Thread::new::thread_start::h6e8defb8d96e41f9
                               at /rustc/f132bcf3bdf6d3ff9be7d02e8d0088b99007cd5e/library/std/src/sys/unix/thread.rs:108:17
  52:     0x7f882e8835c2 - start_thread
  53:     0x7f882e908584 - __clone
  54:                0x0 - <unknown>

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.61.0-nightly (f132bcf3b 2022-03-30) running on x86_64-unknown-linux-gnu

query stack during panic:
#0 [codegen_fulfill_obligation] checking if `core::ops::function::Fn` fulfills its obligations
#1 [resolve_instance] resolving instance `<[closure@./src/test/ui/impl-trait/issues/issue-89312.rs:23:19: 23:25] as core::ops::function::Fn<(&S,)>>::call`
#2 [collect_and_partition_mono_items] collect_and_partition_mono_items
end of query stack
error: aborting due to previous error
