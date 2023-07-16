
warning: Error finalizing incremental compilation session directory `/home/luiz/code/tcp-server/target/debug/incremental/server-15un5yx1wtpo7/s-g87sgl5zk4-16n2tdn-working`: No such file or directory (os error 2)

error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: broken MIR in DefId(0:186 ~ server[4c72]::streams::client::bar::{closure#0}) (((*((*_1).0: &i32)).0: i32)): can't project out of PlaceTy { ty: i32, variant_index: None }
  --> server/src/streams/client.rs:72:22
   |
72 |         let Foo::Foo(baz) = foo;
   |                      ^^^
   |
   = note: delayed at compiler/rustc_borrowck/src/type_check/mod.rs:870:31

error: internal compiler error: TyKind::Error constructed but no error reported
  |
  = note: delayed at compiler/rustc_borrowck/src/type_check/mod.rs:805:20

error: internal compiler error: TyKind::Error constructed but no error reported
  |
  = note: delayed at /rustc/9f4dc0b4db892271cd0dada6e072775b5b5d6b1e/compiler/rustc_middle/src/ty/relate.rs:390:59

error: internal compiler error: broken MIR in DefId(0:185 ~ server[4c72]::streams::client::bar) ((_1.0: i32)): can't project out of PlaceTy { ty: streams::client::Foo, variant_index: None }
  --> server/src/streams/client.rs:70:5
   |
70 | /     || {
71 | |         // `let foo = foo;` makes the ICE disappear
72 | |         let Foo::Foo(baz) = foo;
73 | |     };
   | |_____^
   |
   = note: delayed at compiler/rustc_borrowck/src/type_check/mod.rs:870:31

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1291:13
stack backtrace:
   0:     0x7fc7f49d380d - std::backtrace_rs::backtrace::libunwind::trace::he1bc69317b314d2e
                               at /rustc/9f4dc0b4db892271cd0dada6e072775b5b5d6b1e/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7fc7f49d380d - std::backtrace_rs::backtrace::trace_unsynchronized::h0be7bfa7bff7f915
                               at /rustc/9f4dc0b4db892271cd0dada6e072775b5b5d6b1e/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7fc7f49d380d - std::sys_common::backtrace::_print_fmt::h3daff2ba590cbc63
                               at /rustc/9f4dc0b4db892271cd0dada6e072775b5b5d6b1e/library/std/src/sys_common/backtrace.rs:66:5
   3:     0x7fc7f49d380d - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h82b295b0737f92f8
                               at /rustc/9f4dc0b4db892271cd0dada6e072775b5b5d6b1e/library/std/src/sys_common/backtrace.rs:45:22
   4:     0x7fc7f4a2d80c - core::fmt::write::hc116e04c8b781f0a
                               at /rustc/9f4dc0b4db892271cd0dada6e072775b5b5d6b1e/library/core/src/fmt/mod.rs:1190:17
   5:     0x7fc7f49c4e01 - std::io::Write::write_fmt::hdf2c5cf46a74c282
                               at /rustc/9f4dc0b4db892271cd0dada6e072775b5b5d6b1e/library/std/src/io/mod.rs:1655:15
   6:     0x7fc7f49d68f5 - std::sys_common::backtrace::_print::ha38935cfbe56cf33
                               at /rustc/9f4dc0b4db892271cd0dada6e072775b5b5d6b1e/library/std/src/sys_common/backtrace.rs:48:5
   7:     0x7fc7f49d68f5 - std::sys_common::backtrace::print::h5a555af4c956e490
                               at /rustc/9f4dc0b4db892271cd0dada6e072775b5b5d6b1e/library/std/src/sys_common/backtrace.rs:35:9
   8:     0x7fc7f49d68f5 - std::panicking::default_hook::{{closure}}::h7210e8b2ea136c6a
                               at /rustc/9f4dc0b4db892271cd0dada6e072775b5b5d6b1e/library/std/src/panicking.rs:295:22
   9:     0x7fc7f49d65a9 - std::panicking::default_hook::he0b6748cad6a9b81
                               at /rustc/9f4dc0b4db892271cd0dada6e072775b5b5d6b1e/library/std/src/panicking.rs:314:9
  10:     0x7fc7f51722b1 - rustc_driver[15b9156f6b24bb3]::DEFAULT_HOOK::{closure#0}::{closure#0}
  11:     0x7fc7e508a263 - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::h005d1be707c1bc91
                               at /rustc/9f4dc0b4db892271cd0dada6e072775b5b5d6b1e/library/alloc/src/boxed.rs:1875:9
  12:     0x7fc7e50909fc - proc_macro::bridge::client::<impl proc_macro::bridge::Bridge>::enter::{{closure}}::{{closure}}::h684827b3cee6988a
                               at /rustc/9f4dc0b4db892271cd0dada6e072775b5b5d6b1e/library/proc_macro/src/bridge/client.rs:319:21
  13:     0x7fc7e50ab450 - std::panicking::update_hook::{{closure}}::hff6f21b944627197
                               at /rustc/9f4dc0b4db892271cd0dada6e072775b5b5d6b1e/library/std/src/panicking.rs:258:41
  14:     0x7fc7f49d7040 - std::panicking::rust_panic_with_hook::h7d90789c41819678
                               at /rustc/9f4dc0b4db892271cd0dada6e072775b5b5d6b1e/library/std/src/panicking.rs:702:17
  15:     0x7fc7f6213c81 - std[7af56082c5a9adab]::panicking::begin_panic::<rustc_errors[afdb89a768619fcf]::ExplicitBug>::{closure#0}
  16:     0x7fc7f6212cc6 - std[7af56082c5a9adab]::sys_common::backtrace::__rust_end_short_backtrace::<std[7af56082c5a9adab]::panicking::begin_panic<rustc_errors[afdb89a768619fcf]::ExplicitBug>::{closure#0}, !>
  17:     0x7fc7f6218d4f - std[7af56082c5a9adab]::panicking::begin_panic::<rustc_errors[afdb89a768619fcf]::ExplicitBug>
  18:     0x7fc7f6225386 - std[7af56082c5a9adab]::panic::panic_any::<rustc_errors[afdb89a768619fcf]::ExplicitBug>
  19:     0x7fc7f78b7e6c - <rustc_errors[afdb89a768619fcf]::HandlerInner as core[ec6d764e948e0586]::ops::drop::Drop>::drop
  20:     0x7fc7f6ff5c98 - core[ec6d764e948e0586]::ptr::drop_in_place::<rustc_session[6f79c38a04c315ff]::parse::ParseSess>
  21:     0x7fc7f6ff841a - <alloc[c291d8ee38d8ad41]::rc::Rc<rustc_session[6f79c38a04c315ff]::session::Session> as core[ec6d764e948e0586]::ops::drop::Drop>::drop
  22:     0x7fc7f6fe117d - core[ec6d764e948e0586]::ptr::drop_in_place::<rustc_interface[f459bdc7c39711c]::interface::Compiler>
  23:     0x7fc7f6fe0734 - rustc_span[37126335ebbd5d87]::with_source_map::<core[ec6d764e948e0586]::result::Result<(), rustc_errors[afdb89a768619fcf]::ErrorGuaranteed>, rustc_interface[f459bdc7c39711c]::interface::create_compiler_and_run<core[ec6d764e948e0586]::result::Result<(), rustc_errors[afdb89a768619fcf]::ErrorGuaranteed>, rustc_driver[15b9156f6b24bb3]::run_compiler::{closure#1}>::{closure#1}>
  24:     0x7fc7f6fce184 - rustc_interface[f459bdc7c39711c]::interface::create_compiler_and_run::<core[ec6d764e948e0586]::result::Result<(), rustc_errors[afdb89a768619fcf]::ErrorGuaranteed>, rustc_driver[15b9156f6b24bb3]::run_compiler::{closure#1}>
  25:     0x7fc7f6fcaf02 - <scoped_tls[838f77cdbb45d46c]::ScopedKey<rustc_span[37126335ebbd5d87]::SessionGlobals>>::set::<rustc_interface[f459bdc7c39711c]::interface::run_compiler<core[ec6d764e948e0586]::result::Result<(), rustc_errors[afdb89a768619fcf]::ErrorGuaranteed>, rustc_driver[15b9156f6b24bb3]::run_compiler::{closure#1}>::{closure#0}, core[ec6d764e948e0586]::result::Result<(), rustc_errors[afdb89a768619fcf]::ErrorGuaranteed>>
  26:     0x7fc7f6fc925f - std[7af56082c5a9adab]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[f459bdc7c39711c]::util::run_in_thread_pool_with_globals<rustc_interface[f459bdc7c39711c]::interface::run_compiler<core[ec6d764e948e0586]::result::Result<(), rustc_errors[afdb89a768619fcf]::ErrorGuaranteed>, rustc_driver[15b9156f6b24bb3]::run_compiler::{closure#1}>::{closure#0}, core[ec6d764e948e0586]::result::Result<(), rustc_errors[afdb89a768619fcf]::ErrorGuaranteed>>::{closure#0}, core[ec6d764e948e0586]::result::Result<(), rustc_errors[afdb89a768619fcf]::ErrorGuaranteed>>
  27:     0x7fc7f6fe14f2 - <<std[7af56082c5a9adab]::thread::Builder>::spawn_unchecked_<rustc_interface[f459bdc7c39711c]::util::run_in_thread_pool_with_globals<rustc_interface[f459bdc7c39711c]::interface::run_compiler<core[ec6d764e948e0586]::result::Result<(), rustc_errors[afdb89a768619fcf]::ErrorGuaranteed>, rustc_driver[15b9156f6b24bb3]::run_compiler::{closure#1}>::{closure#0}, core[ec6d764e948e0586]::result::Result<(), rustc_errors[afdb89a768619fcf]::ErrorGuaranteed>>::{closure#0}, core[ec6d764e948e0586]::result::Result<(), rustc_errors[afdb89a768619fcf]::ErrorGuaranteed>>::{closure#1} as core[ec6d764e948e0586]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  28:     0x7fc7f49e1233 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h547c45aa46469d29
                               at /rustc/9f4dc0b4db892271cd0dada6e072775b5b5d6b1e/library/alloc/src/boxed.rs:1861:9
  29:     0x7fc7f49e1233 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::he0a17443e7e0f241
                               at /rustc/9f4dc0b4db892271cd0dada6e072775b5b5d6b1e/library/alloc/src/boxed.rs:1861:9
  30:     0x7fc7f49e1233 - std::sys::unix::thread::Thread::new::thread_start::hd0dd84768f409ab9
                               at /rustc/9f4dc0b4db892271cd0dada6e072775b5b5d6b1e/library/std/src/sys/unix/thread.rs:108:17
  31:     0x7fc7f477a947 - start_thread
                               at ./nptl/./nptl/pthread_create.c:435:8
  32:     0x7fc7f480aa44 - __GI___clone
                               at ./misc/../sysdeps/unix/sysv/linux/x86_64/clone.S:100
  33:                0x0 - <unknown>

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.61.0-nightly (9f4dc0b4d 2022-03-23) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type bin -C embed-bitcode=no -C debuginfo=2 -C incremental

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
warning: `server` (bin "server") generated 5 warnings
error: could not compile `server`; 5 warnings emitted
