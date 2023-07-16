
error: internal compiler error: bad_placeholder_type
 --> main.rs:1:24
  |
1 | pub const FOO: fn() -> _ = 1;
  |                        ^
  |
  = note: delayed at /rustc/04488afe34512aa4c33566eb16d8c912a3ae04f9/src/librustc_session/session.rs:436:27

error: internal compiler error: TyKind::Error constructed but no error reported
  |
  = note: delayed at /rustc/04488afe34512aa4c33566eb16d8c912a3ae04f9/src/librustc_session/session.rs:436:27

error: internal compiler error: mir_const_qualif: MIR had errors
 --> main.rs:1:1
  |
1 | pub const FOO: fn() -> _ = 1;
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: delayed at /rustc/04488afe34512aa4c33566eb16d8c912a3ae04f9/src/librustc_session/session.rs:436:27

error: internal compiler error: PromoteTemps: MIR had errors
 --> main.rs:1:1
  |
1 | pub const FOO: fn() -> _ = 1;
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: delayed at /rustc/04488afe34512aa4c33566eb16d8c912a3ae04f9/src/librustc_session/session.rs:436:27

error: internal compiler error: broken MIR in DefId(0:3 ~ main[317d]::FOO[0]) ("return type"): bad type [type error]
 --> main.rs:1:1
  |
1 | pub const FOO: fn() -> _ = 1;
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: delayed at src/librustc_mir/borrow_check/type_check/mod.rs:258:27

error: internal compiler error: broken MIR in DefId(0:3 ~ main[317d]::FOO[0]) (LocalDecl { mutability: Mut, local_info: None, internal: false, is_block_tail: None, ty: [type error], user_ty: None, source_info: SourceInfo { span: main.rs:1:1: 1:30 (#0), scope: scope[0] } }): bad type [type error]
 --> main.rs:1:1
  |
1 | pub const FOO: fn() -> _ = 1;
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: delayed at src/librustc_mir/borrow_check/type_check/mod.rs:258:27

thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:367:17
stack backtrace:
   0:     0x7f8122b8c925 - backtrace::backtrace::libunwind::trace::h14d338b30b3ea0a7
                               at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.46/src/backtrace/libunwind.rs:86
   1:     0x7f8122b8c925 - backtrace::backtrace::trace_unsynchronized::h73ea91d74a3fd67f
                               at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.46/src/backtrace/mod.rs:66
   2:     0x7f8122b8c925 - std::sys_common::backtrace::_print_fmt::hd42948c952866e12
                               at src/libstd/sys_common/backtrace.rs:78
   3:     0x7f8122b8c925 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha8f928866ff7571e
                               at src/libstd/sys_common/backtrace.rs:59
   4:     0x7f8122bc93ac - core::fmt::write::he0c1e5f7426d2718
                               at src/libcore/fmt/mod.rs:1076
   5:     0x7f8122b7e6a2 - std::io::Write::write_fmt::hf3afc6cfd57d0033
                               at src/libstd/io/mod.rs:1537
   6:     0x7f8122b917c0 - std::sys_common::backtrace::_print::hfc0110703f3696fd
                               at src/libstd/sys_common/backtrace.rs:62
   7:     0x7f8122b917c0 - std::sys_common::backtrace::print::h3f77c6990ddfaa22
                               at src/libstd/sys_common/backtrace.rs:49
   8:     0x7f8122b917c0 - std::panicking::default_hook::{{closure}}::heae49580a8d62d75
                               at src/libstd/panicking.rs:198
   9:     0x7f8122b9150c - std::panicking::default_hook::hecc34e3f729e213c
                               at src/libstd/panicking.rs:217
  10:     0x7f81232fc2e9 - rustc_driver::report_ice::h3be30985e3a52e6d
  11:     0x7f8122b91f38 - std::panicking::rust_panic_with_hook::he82f5d0644692441
                               at src/libstd/panicking.rs:530
  12:     0x7f8125e4d49e - std::panicking::begin_panic::h58c374fa2c754653
  13:     0x7f8125e811f2 - <rustc_errors::HandlerInner as core::ops::drop::Drop>::drop::hd1d9bbded6f728fa
  14:     0x7f812333e4f6 - core::ptr::drop_in_place::h9f5d87bb2860130d
  15:     0x7f81233451a6 - <alloc::rc::Rc<T> as core::ops::drop::Drop>::drop::h4621d3ee5e7200ec
  16:     0x7f812335a56d - core::ptr::drop_in_place::h475f04886ae24548
  17:     0x7f8123355abb - rustc_span::with_source_map::h8932c000c7b08879
  18:     0x7f81232b7826 - rustc_interface::interface::create_compiler_and_run::h442b6cbb52362b36
  19:     0x7f81232e1f1d - scoped_tls::ScopedKey<T>::set::h8938bb49c90cd057
  20:     0x7f8123308272 - std::sys_common::backtrace::__rust_begin_short_backtrace::hde285eb3c6f4aaa5
  21:     0x7f81232c570e - core::ops::function::FnOnce::call_once{{vtable.shim}}::hbbf6021168227a97
  22:     0x7f8122ba0cfa - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::hd2b3bc04af94a84f
                               at /rustc/04488afe34512aa4c33566eb16d8c912a3ae04f9/src/liballoc/boxed.rs:1081
  23:     0x7f8122ba0cfa - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::h1044417e186e567a
                               at /rustc/04488afe34512aa4c33566eb16d8c912a3ae04f9/src/liballoc/boxed.rs:1081
  24:     0x7f8122ba0cfa - std::sys::unix::thread::Thread::new::thread_start::h276e6ca033938925
                               at src/libstd/sys/unix/thread.rs:87
  25:     0x7f8122ae1fa3 - start_thread
                               at /build/glibc-vjB4T1/glibc-2.28/nptl/pthread_create.c:486
  26:     0x7f8122a034cf - clone
                               at ../sysdeps/unix/sysv/linux/x86_64/clone.S:95
  27:                0x0 - <unknown>

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.46.0 (04488afe3 2020-08-24) running on x86_64-unknown-linux-gnu

query stack during panic:
end of query stack
