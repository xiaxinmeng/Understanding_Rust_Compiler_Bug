
warning: the feature `const_generics` is incomplete and may cause the compiler to crash
 --> src/main.rs:1:12
  |
1 | #![feature(const_generics)]
  |            ^^^^^^^^^^^^^^

thread 'rustc' panicked at 'index out of bounds: the len is 1 but the index is 1', /rustc/6afcb5628523b0baae5704ad34ac1aba8ba10de6/src/libcore/slice/mod.rs:2695:10
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
             at src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:39
   1: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:71
   2: std::panicking::default_hook::{{closure}}
             at src/libstd/sys_common/backtrace.rs:59
             at src/libstd/panicking.rs:197
   3: std::panicking::default_hook
             at src/libstd/panicking.rs:211
   4: rustc::util::common::panic_hook
   5: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:478
   6: std::panicking::continue_panic_fmt
             at src/libstd/panicking.rs:381
   7: rust_begin_unwind
             at src/libstd/panicking.rs:308
   8: core::panicking::panic_fmt
             at src/libcore/panicking.rs:85
   9: core::panicking::panic_bounds_check
             at src/libcore/panicking.rs:61
  10: rustc_mir::borrow_check::nll::type_check::type_check
  11: rustc_mir::borrow_check::nll::compute_regions
  12: rustc_mir::borrow_check::do_mir_borrowck
  13: rustc::ty::context::GlobalCtxt::enter_local
  14: rustc_mir::borrow_check::mir_borrowck
  15: rustc::ty::query::__query_compute::mir_borrowck
  16: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::mir_borrowck>::compute
  17: rustc::dep_graph::graph::DepGraph::with_task_impl
  18: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  19: rustc::ty::<impl rustc::ty::context::TyCtxt>::par_body_owners
  20: rustc::util::common::time
  21: rustc_interface::passes::analysis
  22: rustc::ty::query::__query_compute::analysis
  23: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::analysis>::compute
  24: rustc::dep_graph::graph::DepGraph::with_task_impl
  25: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  26: rustc::ty::context::tls::enter_global
  27: rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}
  28: rustc_interface::passes::create_global_ctxt::{{closure}}
  29: rustc_interface::interface::run_compiler_in_existing_thread_pool
  30: std::thread::local::LocalKey<T>::with
  31: scoped_tls::ScopedKey<T>::set
  32: syntax::with_globals
query stack during panic:
#0 [mir_borrowck] processing `<S<T, N> as std::cmp::PartialEq>::eq`
#1 [analysis] running analysis passes on this crate
end of query stack
error: internal compiler error: array length could not be evaluated

error: internal compiler error: cat_expr Errd
 --> src/main.rs:3:10
  |
3 | #[derive(PartialEq)]
  |          ^^^^^^^^^

error: internal compiler error: cat_expr Errd
 --> src/main.rs:4:29
  |
4 | struct S<T, const N: usize>([T; N]);
  |                             ^^^^^^

error: internal compiler error: QualifyAndPromoteConstants: Mir had errors
 --> src/main.rs:3:10
  |
3 | #[derive(PartialEq)]
  |          ^^^^^^^^^

error: internal compiler error: broken MIR in DefId(0:23 ~ test2rust[4429]::{{impl}}[0]::eq[0]) ("return type"): bad type [type error]
 --> src/main.rs:3:10
  |
3 | #[derive(PartialEq)]
  |          ^^^^^^^^^

error: internal compiler error: broken MIR in DefId(0:23 ~ test2rust[4429]::{{impl}}[0]::eq[0]) (LocalDecl { mutability: Mut, is_user_variable: None, internal: false, is_block_tail: None, ty: [type error], user_ty: UserTypeProjections { contents: [] }, name: None, source_info: SourceInfo { span: src/main.rs:3:10: 3:19, scope: scope[0] }, visibility_scope: scope[0] }): bad type [type error]
 --> src/main.rs:3:10
  |
3 | #[derive(PartialEq)]
  |          ^^^^^^^^^

thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:356:17
stack backtrace:
   0:     0x7fe075754b53 - std::sys::unix::backtrace::tracing::imp::unwind_backtrace::he0fa00e90b88cf41
                               at src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:39
   1:     0x7fe07574c9ab - std::sys_common::backtrace::_print::h04cbc2776ed466ce
                               at src/libstd/sys_common/backtrace.rs:71
   2:     0x7fe075750d86 - std::panicking::default_hook::{{closure}}::hc52ea54cb1555d62
                               at src/libstd/sys_common/backtrace.rs:59
                               at src/libstd/panicking.rs:197
   3:     0x7fe075750b19 - std::panicking::default_hook::h4c942d7c5edb1023
                               at src/libstd/panicking.rs:211
   4:     0x7fe073484a70 - rustc::util::common::panic_hook::h2ba263e88c3777c0
   5:     0x7fe075751578 - std::panicking::rust_panic_with_hook::he42004b73d9971e9
                               at src/libstd/panicking.rs:478
   6:     0x7fe0720cbcb4 - std::panicking::begin_panic::hefb6ff22ee5b155d
   7:     0x7fe0720eed24 - <rustc_errors::Handler as core::ops::drop::Drop>::drop::hc46550d8e7a81487
   8:     0x7fe075a86265 - core::ptr::real_drop_in_place::h74c340c83e03d59b
   9:     0x7fe075a86b84 - core::ptr::real_drop_in_place::h8508895491c70819
  10:     0x7fe075a8e863 - <alloc::rc::Rc<T> as core::ops::drop::Drop>::drop::h85186abd0d4c034e
  11:     0x7fe075a45ceb - core::ptr::real_drop_in_place::he79d810282490551
  12:     0x7fe075a3d3e6 - rustc_interface::interface::run_compiler_in_existing_thread_pool::h1f087613a37194c6
  13:     0x7fe075abb1d5 - std::thread::local::LocalKey<T>::with::h0656e36a98139a4f
  14:     0x7fe075a5de74 - scoped_tls::ScopedKey<T>::set::hdb899c3a7d167a06
  15:     0x7fe075aa9d81 - syntax::with_globals::h68df3686302d6411
  16:     0x7fe075a0b214 - std::sys_common::backtrace::__rust_begin_short_backtrace::h5fb6b9bffd514a08
  17:     0x7fe075762619 - __rust_maybe_catch_panic
                               at src/libpanic_unwind/lib.rs:85
  18:     0x7fe075a36a78 - core::ops::function::FnOnce::call_once{{vtable.shim}}::h65c7eb30f335d157
  19:     0x7fe0757335ae - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::h34602ed91fba4f13
                               at /rustc/6afcb5628523b0baae5704ad34ac1aba8ba10de6/src/liballoc/boxed.rs:704
  20:     0x7fe07576128f - std::sys::unix::thread::Thread::new::thread_start::h549f61312ca41869
                               at /rustc/6afcb5628523b0baae5704ad34ac1aba8ba10de6/src/liballoc/boxed.rs:704
                               at src/libstd/sys_common/thread.rs:13
                               at src/libstd/sys/unix/thread.rs:79
  21:     0x7fe075697fa7 - start_thread
                               at /builddir/glibc-2.29/nptl/pthread_create.c:486
  22:     0x7fe0755b9a1e - __GI___clone
                               at ../sysdeps/unix/sysv/linux/x86_64/clone.S:95
  23:                0x0 - <unknown>
query stack during panic:
end of query stack
thread panicked while panicking. aborting.
error: Could not compile `test2rust`.

Caused by:
  process didn't exit successfully: `rustc --edition=2018 --crate-name test2rust src/main.rs --color always --crate-type bin --emit=dep-info,metadata -C debuginfo=2 -C metadata=9cfb5b45880c2b00 -C extra-filename=-9cfb5b45880c2b00 --out-dir /home/carado/tmp/test2rust/target/debug/deps -C incremental=/home/carado/tmp/test2rust/target/debug/incremental -L dependency=/home/carado/tmp/test2rust/target/debug/deps` (signal: 4, SIGILL: illegal instruction)
