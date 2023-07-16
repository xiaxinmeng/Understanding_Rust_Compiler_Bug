`
warning: 2 warnings emitted

error: internal compiler error: bad_placeholder_type
  --> ice.rs:11:64
   |
11 | static CALLBACKS: HashMap<*const dyn WidgetExt, dyn FnMut(&mut _) + 'static> = HashMap::new();
   |                                                                ^
   |
   = note: delayed at compiler/rustc_typeck/src/collect.rs:390:20

error: internal compiler error: TyKind::Error constructed but no error reported
  |
  = note: delayed at compiler/rustc_typeck/src/astconv/mod.rs:532:48

error: internal compiler error: TyKind::Error constructed but no error reported
  |
  = note: delayed at /rustc/b7404c898a1a6933b71c72428a6dce551bcc1be7/compiler/rustc_middle/src/ty/relate.rs:382:59

error: internal compiler error: TyKind::Error constructed but no error reported
  |
  = note: delayed at compiler/rustc_infer/src/infer/sub.rs:121:31

error: internal compiler error: TyKind::Error constructed but no error reported
  |
  = note: delayed at compiler/rustc_typeck/src/check/coercion.rs:157:49

error: internal compiler error: TyKind::Error constructed but no error reported
  |
  = note: delayed at compiler/rustc_typeck/src/check/fallback.rs:89:58

error: internal compiler error: cat_expr Errd
  --> ice.rs:11:80
   |
11 | static CALLBACKS: HashMap<*const dyn WidgetExt, dyn FnMut(&mut _) + 'static> = HashMap::new();
   |                                                                                ^^^^^^^^^^^^^^
   |
   = note: delayed at compiler/rustc_typeck/src/check/regionck.rs:408:31

error: internal compiler error: cat_expr Errd
  --> ice.rs:11:80
   |
11 | static CALLBACKS: HashMap<*const dyn WidgetExt, dyn FnMut(&mut _) + 'static> = HashMap::new();
   |                                                                                ^^^^^^^^^^^^
   |
   = note: delayed at compiler/rustc_typeck/src/check/regionck.rs:408:31

error: internal compiler error: generic static must be rejected
  --> ice.rs:11:1
   |
11 | static CALLBACKS: HashMap<*const dyn WidgetExt, dyn FnMut(&mut _) + 'static> = HashMap::new();
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: delayed at compiler/rustc_typeck/src/check/check.rs:424:22

error: internal compiler error: TyKind::Error constructed but no error reported
  |
  = note: delayed at compiler/rustc_mir_build/src/build/mod.rs:716:18

error: internal compiler error: mir_const_qualif: MIR had errors
  --> ice.rs:11:1
   |
11 | static CALLBACKS: HashMap<*const dyn WidgetExt, dyn FnMut(&mut _) + 'static> = HashMap::new();
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: delayed at compiler/rustc_mir/src/transform/mod.rs:238:18

error: internal compiler error: PromoteTemps: MIR had errors
  --> ice.rs:11:1
   |
11 | static CALLBACKS: HashMap<*const dyn WidgetExt, dyn FnMut(&mut _) + 'static> = HashMap::new();
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: delayed at compiler/rustc_mir/src/transform/promote_consts.rs:51:22

error: internal compiler error: broken MIR in DefId(0:10 ~ ice[9cac]::CALLBACKS) ("return type"): bad type [type error]
  --> ice.rs:11:1
   |
11 | static CALLBACKS: HashMap<*const dyn WidgetExt, dyn FnMut(&mut _) + 'static> = HashMap::new();
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: delayed at compiler/rustc_mir/src/borrow_check/type_check/mod.rs:308:27

error: internal compiler error: TyKind::Error constructed but no error reported
  |
  = note: delayed at compiler/rustc_mir/src/borrow_check/type_check/mod.rs:777:20

error: internal compiler error: broken MIR in DefId(0:10 ~ ice[9cac]::CALLBACKS) (LocalDecl { mutability: Mut, local_info: None, internal: false, is_block_tail: None, ty: [type error], user_ty: None, source_info: SourceInfo { span: ice.rs:11:1: 11:95 (#0), scope: scope[0] } }): bad type [type error]
  --> ice.rs:11:1
   |
11 | static CALLBACKS: HashMap<*const dyn WidgetExt, dyn FnMut(&mut _) + 'static> = HashMap::new();
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: delayed at compiler/rustc_mir/src/borrow_check/type_check/mod.rs:308:27

thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', compiler/rustc_errors/src/lib.rs:1166:13
stack backtrace:
   0:     0x7efc859b74dc - std::backtrace_rs::backtrace::libunwind::trace::h788b2853b7016c32
                               at /rustc/b7404c898a1a6933b71c72428a6dce551bcc1be7/library/std/src/../../backtrace/src/backtrace/libunwind.rs:90:5
   1:     0x7efc859b74dc - std::backtrace_rs::backtrace::trace_unsynchronized::h3626590e16510efa
                               at /rustc/b7404c898a1a6933b71c72428a6dce551bcc1be7/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7efc859b74dc - std::sys_common::backtrace::_print_fmt::ha76294ed367b5eb6
                               at /rustc/b7404c898a1a6933b71c72428a6dce551bcc1be7/library/std/src/sys_common/backtrace.rs:67:5
   3:     0x7efc859b74dc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h4244b134876ede81
                               at /rustc/b7404c898a1a6933b71c72428a6dce551bcc1be7/library/std/src/sys_common/backtrace.rs:46:22
   4:     0x7efc85a151ec - core::fmt::write::h9a6d9c74526a6c1b
                               at /rustc/b7404c898a1a6933b71c72428a6dce551bcc1be7/library/core/src/fmt/mod.rs:1150:17
   5:     0x7efc859a8415 - std::io::Write::write_fmt::h7f8a2ef72f011ad9
                               at /rustc/b7404c898a1a6933b71c72428a6dce551bcc1be7/library/std/src/io/mod.rs:1667:15
   6:     0x7efc859baa40 - std::sys_common::backtrace::_print::h4b3c9553c91f7522
                               at /rustc/b7404c898a1a6933b71c72428a6dce551bcc1be7/library/std/src/sys_common/backtrace.rs:49:5
   7:     0x7efc859baa40 - std::sys_common::backtrace::print::h36fb46a493801fb8
                               at /rustc/b7404c898a1a6933b71c72428a6dce551bcc1be7/library/std/src/sys_common/backtrace.rs:36:9
   8:     0x7efc859baa40 - std::panicking::default_hook::{{closure}}::hf28f6810f0e04677
                               at /rustc/b7404c898a1a6933b71c72428a6dce551bcc1be7/library/std/src/panicking.rs:210:50
   9:     0x7efc859ba5f7 - std::panicking::default_hook::hadb819fa279f9d7b
                               at /rustc/b7404c898a1a6933b71c72428a6dce551bcc1be7/library/std/src/panicking.rs:227:9
  10:     0x7efc8618e7b1 - rustc_driver::DEFAULT_HOOK::{{closure}}::{{closure}}::hf7804370e2af68fc
  11:     0x7efc859bb259 - std::panicking::rust_panic_with_hook::h015085c4aa271d26
                               at /rustc/b7404c898a1a6933b71c72428a6dce551bcc1be7/library/std/src/panicking.rs:628:17
  12:     0x7efc859bad10 - std::panicking::begin_panic_handler::{{closure}}::h15a8a2888dd1ba59
                               at /rustc/b7404c898a1a6933b71c72428a6dce551bcc1be7/library/std/src/panicking.rs:521:13
  13:     0x7efc859b7984 - std::sys_common::backtrace::__rust_end_short_backtrace::h673e204498e49379
                               at /rustc/b7404c898a1a6933b71c72428a6dce551bcc1be7/library/std/src/sys_common/backtrace.rs:141:18
  14:     0x7efc859bac79 - rust_begin_unwind
                               at /rustc/b7404c898a1a6933b71c72428a6dce551bcc1be7/library/std/src/panicking.rs:517:5
  15:     0x7efc8597f7fb - std::panicking::begin_panic_fmt::hc0c200f2fa4ef986
                               at /rustc/b7404c898a1a6933b71c72428a6dce551bcc1be7/library/std/src/panicking.rs:460:5
  16:     0x7efc886f109e - rustc_errors::HandlerInner::flush_delayed::hfa770e96c4015d8d
  17:     0x7efc886ef91d - <rustc_errors::HandlerInner as core::ops::drop::Drop>::drop::hc2269e7aaf41b216
  18:     0x7efc87cb1976 - core::ptr::drop_in_place<rustc_session::parse::ParseSess>::h72dc1432dfff01af
  19:     0x7efc87cb3a1e - <alloc::rc::Rc<T> as core::ops::drop::Drop>::drop::h644fed3d75172bde
  20:     0x7efc87c858cd - core::ptr::drop_in_place<rustc_interface::interface::Compiler>::hebdcf30a25eaa413
  21:     0x7efc87c855bc - rustc_span::with_source_map::h0c688eed363cc6b3
  22:     0x7efc87c976ac - scoped_tls::ScopedKey<T>::set::h3412736d21354023
  23:     0x7efc87c8681b - std::sys_common::backtrace::__rust_begin_short_backtrace::h2eadc68631dd2604
  24:     0x7efc87c839e5 - core::ops::function::FnOnce::call_once{{vtable.shim}}::h05dec0a08a1dc2b4
  25:     0x7efc859c78a3 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h334de5469f492d6e
                               at /rustc/b7404c898a1a6933b71c72428a6dce551bcc1be7/library/alloc/src/boxed.rs:1636:9
  26:     0x7efc859c78a3 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h24abb7dc135c5761
                               at /rustc/b7404c898a1a6933b71c72428a6dce551bcc1be7/library/alloc/src/boxed.rs:1636:9
  27:     0x7efc859c78a3 - std::sys::unix::thread::Thread::new::thread_start::h09d0eb3bf5e9d59a
                               at /rustc/b7404c898a1a6933b71c72428a6dce551bcc1be7/library/std/src/sys/unix/thread.rs:106:17
  28:     0x7efc858cf259 - start_thread
  29:     0x7efc857e45e3 - __GI___clone
  30:                0x0 - <unknown>

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.56.0-nightly (b7404c898 2021-09-03) running on x86_64-unknown-linux-gnu

query stack during panic:
end of query stack
thread panicked while panicking. aborting.
[2]    3704753 illegal hardware instruction (core dumped)  rustc ice.rs
