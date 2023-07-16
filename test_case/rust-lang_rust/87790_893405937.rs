
#0  __GI_raise (sig=sig@entry=6) at ../sysdeps/unix/sysv/linux/raise.c:50
#1  0x00007fffee902859 in __GI_abort () at abort.c:79
#2  0x00007fffeebac457 in std::sys::unix::abort_internal () at library/std/src/sys/unix/mod.rs:254
#3  0x00007fffeeb4f526 in std::process::abort () at library/std/src/process.rs:1957
#4  0x00007ffff481d7a4 in rustc_ast::mut_visit::visit_clobber::{{closure}} () at /home/dwrensha/src/rust/compiler/rustc_ast/src/mut_visit.rs:300
#5  0x00007ffff481ccf1 in core::result::Result<T,E>::unwrap_or_else () at /home/dwrensha/src/rust/library/core/src/result.rs:1083
#6  rustc_ast::mut_visit::visit_clobber () at /home/dwrensha/src/rust/compiler/rustc_ast/src/mut_visit.rs:299
#7  0x00007ffff47bf56d in rustc_ast::ast_like::visit_attrvec () at /home/dwrensha/src/rust/compiler/rustc_ast/src/ast_like.rs:92
#8  <rustc_data_structures::thin_vec::ThinVec<rustc_ast::ast::Attribute> as rustc_ast::ast_like::VecOrAttrVec>::visit ()
    at /home/dwrensha/src/rust/compiler/rustc_ast/src/ast_like.rs:197
#9  <rustc_ast::ast::Expr as rustc_ast::ast_like::AstLike>::visit_attrs () at /home/dwrensha/src/rust/compiler/rustc_ast/src/ast_like.rs:214
#10 <rustc_ast::ptr::P<T> as rustc_ast::ast_like::AstLike>::visit_attrs () at /home/dwrensha/src/rust/compiler/rustc_ast/src/ast_like.rs:34
#11 rustc_expand::config::StripUnconfigured::process_cfg_attrs () at compiler/rustc_expand/src/config.rs:335
#12 rustc_expand::config::StripUnconfigured::configure_expr () at compiler/rustc_expand/src/config.rs:508
#13 0x00007ffff4827ee7 in <rustc_expand::expand::InvocationCollector as rustc_ast::mut_visit::MutVisitor>::visit_expr ()
    at compiler/rustc_expand/src/expand.rs:1157
#14 rustc_ast::mut_visit::noop_visit_anon_const () at /home/dwrensha/src/rust/compiler/rustc_ast/src/mut_visit.rs:1175
#15 0x00007ffff4828afd in <rustc_expand::expand::InvocationCollector as rustc_ast::mut_visit::MutVisitor>::visit_ty ()
    at compiler/rustc_expand/src/expand.rs:1534
#16 rustc_ast::mut_visit::noop_flat_map_field_def () at /home/dwrensha/src/rust/compiler/rustc_ast/src/mut_visit.rs:920
#17 0x00007ffff476f41d in <rustc_expand::expand::InvocationCollector as rustc_ast::mut_visit::MutVisitor>::flat_map_field_def::{{closure}} ()
    at compiler/rustc_expand/src/expand.rs:1245
#18 <rustc_expand::expand::InvocationCollector as rustc_ast::mut_visit::MutVisitor>::flat_map_field_def ()
    at compiler/rustc_expand/src/expand.rs:1245
#19 0x00007ffff47e1f26 in rustc_ast::mut_visit::noop_visit_ty::{{closure}} () at /home/dwrensha/src/rust/compiler/rustc_ast/src/mut_visit.rs:488
#20 <alloc::vec::Vec<T> as rustc_data_structures::map_in_place::MapInPlace<T>>::flat_map_in_place ()
    at /home/dwrensha/src/rust/compiler/rustc_data_structures/src/map_in_place.rs:34
#21 0x00007ffff48269d0 in rustc_ast::mut_visit::noop_visit_variant_data () at /home/dwrensha/src/rust/compiler/rustc_ast/src/mut_visit.rs:889
#22 rustc_ast::mut_visit::MutVisitor::visit_variant_data () at /home/dwrensha/src/rust/compiler/rustc_ast/src/mut_visit.rs:242
#23 rustc_ast::mut_visit::noop_visit_item_kind () at /home/dwrensha/src/rust/compiler/rustc_ast/src/mut_visit.rs:982
#24 0x00007ffff4823e41 in rustc_ast::mut_visit::MutVisitor::visit_item_kind () at /home/dwrensha/src/rust/compiler/rustc_ast/src/mut_visit.rs:110
#25 rustc_ast::mut_visit::noop_flat_map_item () at /home/dwrensha/src/rust/compiler/rustc_ast/src/mut_visit.rs:1093
#26 0x00007ffff4770912 in <rustc_expand::expand::InvocationCollector as rustc_ast::mut_visit::MutVisitor>::flat_map_item::{{closure}} ()
    at compiler/rustc_expand/src/expand.rs:1473
#27 <rustc_expand::expand::InvocationCollector as rustc_ast::mut_visit::MutVisitor>::flat_map_item () at compiler/rustc_expand/src/expand.rs:1473
#28 0x00007ffff47dd4ce in rustc_ast::mut_visit::noop_visit_item_kind::{{closure}} ()
    at /home/dwrensha/src/rust/compiler/rustc_ast/src/mut_visit.rs:966
#29 <alloc::vec::Vec<T> as rustc_data_structures::map_in_place::MapInPlace<T>>::flat_map_in_place ()
    at /home/dwrensha/src/rust/compiler/rustc_data_structures/src/map_in_place.rs:34
#30 0x00007ffff4826462 in rustc_ast::mut_visit::noop_visit_item_kind () at /home/dwrensha/src/rust/compiler/rustc_ast/src/mut_visit.rs:966
#31 0x00007ffff4823e41 in rustc_ast::mut_visit::MutVisitor::visit_item_kind () at /home/dwrensha/src/rust/compiler/rustc_ast/src/mut_visit.rs:110
#32 rustc_ast::mut_visit::noop_flat_map_item () at /home/dwrensha/src/rust/compiler/rustc_ast/src/mut_visit.rs:1093
#33 0x00007ffff47708af in <rustc_expand::expand::InvocationCollector as rustc_ast::mut_visit::MutVisitor>::flat_map_item ()
    at compiler/rustc_expand/src/expand.rs:1475
--Type <RET> for more, q to quit, c to continue without paging--
#34 0x00007ffff4736662 in rustc_expand::expand::AstFragment::mut_visit_with::{{closure}} () at compiler/rustc_expand/src/expand.rs:125
#35 <smallvec::SmallVec<A> as rustc_data_structures::map_in_place::MapInPlace<T>>::flat_map_in_place ()
    at /home/dwrensha/src/rust/compiler/rustc_data_structures/src/map_in_place.rs:80
#36 0x00007ffff476c964 in rustc_expand::expand::AstFragment::mut_visit_with () at compiler/rustc_expand/src/expand.rs:125
#37 rustc_expand::expand::MacroExpander::collect_invocations () at compiler/rustc_expand/src/expand.rs:607
#38 0x00007ffff4767a77 in rustc_expand::expand::MacroExpander::fully_expand_fragment () at compiler/rustc_expand/src/expand.rs:435
#39 0x00007ffff4767112 in rustc_expand::expand::MacroExpander::expand_crate () at compiler/rustc_expand/src/expand.rs:396
#40 0x00007fffefbea1b3 in rustc_interface::passes::configure_and_expand::{{closure}}::{{closure}} () at compiler/rustc_interface/src/passes.rs:334
#41 rustc_data_structures::profiling::VerboseTimingGuard::run () at /home/dwrensha/src/rust/compiler/rustc_data_structures/src/profiling.rs:611
#42 rustc_session::utils::<impl rustc_session::session::Session>::time () at /home/dwrensha/src/rust/compiler/rustc_session/src/utils.rs:16
#43 rustc_interface::passes::configure_and_expand::{{closure}} () at compiler/rustc_interface/src/passes.rs:334
#44 rustc_data_structures::profiling::VerboseTimingGuard::run () at /home/dwrensha/src/rust/compiler/rustc_data_structures/src/profiling.rs:611
#45 rustc_session::utils::<impl rustc_session::session::Session>::time () at /home/dwrensha/src/rust/compiler/rustc_session/src/utils.rs:16
#46 0x00007fffefb62d5b in rustc_interface::passes::configure_and_expand () at compiler/rustc_interface/src/passes.rs:280
#47 0x00007fffefc10d30 in rustc_interface::queries::Queries::expansion::{{closure}}::{{closure}} () at compiler/rustc_interface/src/queries.rs:184
#48 rustc_interface::passes::boxed_resolver::BoxedResolver::access () at compiler/rustc_interface/src/passes.rs:142
#49 0x00007fffefb54deb in rustc_interface::queries::Queries::expansion::{{closure}} () at compiler/rustc_interface/src/queries.rs:183
#50 rustc_interface::queries::Query<T>::compute () at compiler/rustc_interface/src/queries.rs:38
#51 rustc_interface::queries::Queries::expansion () at compiler/rustc_interface/src/queries.rs:172
#52 0x00007fffefa55705 in rustc_driver::run_compiler::{{closure}}::{{closure}} () at compiler/rustc_driver/src/lib.rs:364
#53 rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter ()
    at /home/dwrensha/src/rust/compiler/rustc_interface/src/queries.rs:390
#54 0x00007fffefa0e1af in rustc_driver::run_compiler::{{closure}} () at compiler/rustc_driver/src/lib.rs:312
#55 rustc_interface::interface::create_compiler_and_run::{{closure}} () at /home/dwrensha/src/rust/compiler/rustc_interface/src/interface.rs:209
#56 rustc_span::with_source_map () at /home/dwrensha/src/rust/compiler/rustc_span/src/lib.rs:911
#57 0x00007fffefa56bf8 in rustc_interface::interface::create_compiler_and_run ()
    at /home/dwrensha/src/rust/compiler/rustc_interface/src/interface.rs:203
#58 0x00007fffefa4b5c4 in rustc_interface::interface::run_compiler::{{closure}} ()
    at /home/dwrensha/src/rust/compiler/rustc_interface/src/interface.rs:225
#59 rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals::{{closure}}::{{closure}} ()
    at /home/dwrensha/src/rust/compiler/rustc_interface/src/util.rs:157
#60 scoped_tls::ScopedKey<T>::set () at /home/dwrensha/.cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137
#61 0x00007fffefa105cb in rustc_span::create_session_globals_then () at /home/dwrensha/src/rust/compiler/rustc_span/src/lib.rs:105
#62 rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals::{{closure}} ()
    at /home/dwrensha/src/rust/compiler/rustc_interface/src/util.rs:155
#63 rustc_interface::util::scoped_thread::{{closure}} () at /home/dwrensha/src/rust/compiler/rustc_interface/src/util.rs:130
#64 std::sys_common::backtrace::__rust_begin_short_backtrace () at /home/dwrensha/src/rust/library/std/src/sys_common/backtrace.rs:125
#65 0x00007fffefa00593 in std::thread::Builder::spawn_unchecked::{{closure}}::{{closure}} ()
    at /home/dwrensha/src/rust/library/std/src/thread/mod.rs:476
#66 <core::panic::unwind_safe::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once ()
    at /home/dwrensha/src/rust/library/core/src/panic/unwind_safe.rs:271
#67 std::panicking::try::do_call () at /home/dwrensha/src/rust/library/std/src/panicking.rs:403
#68 std::panicking::try () at /home/dwrensha/src/rust/library/std/src/panicking.rs:367
#69 std::panic::catch_unwind () at /home/dwrensha/src/rust/library/std/src/panic.rs:129
--Type <RET> for more, q to quit, c to continue without paging--
#70 std::thread::Builder::spawn_unchecked::{{closure}} () at /home/dwrensha/src/rust/library/std/src/thread/mod.rs:475
#71 core::ops::function::FnOnce::call_once{{vtable-shim}} () at /home/dwrensha/src/rust/library/core/src/ops/function.rs:227
#72 0x00007fffeeb68b83 in <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once ()
    at /home/dwrensha/src/rust/library/alloc/src/boxed.rs:1572
#73 <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once () at /home/dwrensha/src/rust/library/alloc/src/boxed.rs:1572
#74 std::sys::unix::thread::Thread::new::thread_start () at library/std/src/sys/unix/thread.rs:91
#75 0x00007fffee8bd609 in start_thread (arg=<optimized out>) at pthread_create.c:477
#76 0x00007fffee9ff293 in clone () at ../sysdeps/unix/sysv/linux/x86_64/clone.S:95
