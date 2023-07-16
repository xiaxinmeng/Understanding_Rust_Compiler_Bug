`
error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: bad placeholder type
 --> rd.rs:2:24
  |
2 | const FOO: dyn Fn() -> _ = ""; //~ ERROR E0121
  |                        ^
  |
  = note: delayed at compiler/rustc_hir_analysis/src/collect.rs:403:20

error: internal compiler error: TyKind::Error constructed but no error reported
  |
  = note: delayed at compiler/rustc_hir_typeck/src/coercion.rs:176:49

error: internal compiler error: bad placeholder type
 --> rd.rs:3:25
  |
3 | static BOO: dyn Fn() -> _ = ""; //~ ERROR E0121
  |                         ^
  |
  = note: delayed at compiler/rustc_hir_analysis/src/collect.rs:403:20

error: internal compiler error: TyKind::Error constructed but no error reported
  |
  = note: delayed at compiler/rustc_mir_build/src/build/mod.rs:637:18

error: internal compiler error: mir_const_qualif: MIR had errors
 --> rd.rs:2:1
  |
2 | const FOO: dyn Fn() -> _ = ""; //~ ERROR E0121
  | ^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: delayed at compiler/rustc_mir_transform/src/lib.rs:254:18

error: internal compiler error: broken MIR in DefId(0:3 ~ rd[d0c3]::FOO) ("return type"): bad type [type error]
 --> rd.rs:2:1
  |
2 | const FOO: dyn Fn() -> _ = ""; //~ ERROR E0121
  | ^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: delayed at compiler/rustc_borrowck/src/type_check/mod.rs:520:13

error: internal compiler error: TyKind::Error constructed but no error reported
  |
  = note: delayed at compiler/rustc_borrowck/src/type_check/mod.rs:768:20

error: internal compiler error: broken MIR in DefId(0:3 ~ rd[d0c3]::FOO) (LocalDecl { mutability: Mut, local_info: None, internal: false, is_block_tail: None, ty: [type error], user_ty: None, source_info: SourceInfo { span: rd.rs:2:1: 2:25 (#0), scope: scope[0] } }): bad type [type error]
 --> rd.rs:2:1
  |
2 | const FOO: dyn Fn() -> _ = ""; //~ ERROR E0121
  | ^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: delayed at compiler/rustc_borrowck/src/type_check/mod.rs:520:13

error: internal compiler error: TyKind::Error constructed but no error reported
  |
  = note: delayed at /rustc/5fa44b54641cac7dc47964870d08b4ec82fc8157/compiler/rustc_middle/src/ty/relate.rs:425:59

thread '<unnamed>' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1609:13
stack backtrace:
   0:     0x7fc2eb62658a - std::backtrace_rs::backtrace::libunwind::trace::hc8e994fd046b5ea0
                               at /rustc/5fa44b54641cac7dc47964870d08b4ec82fc8157/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7fc2eb62658a - std::backtrace_rs::backtrace::trace_unsynchronized::h89d598f4e35d7c6d
                               at /rustc/5fa44b54641cac7dc47964870d08b4ec82fc8157/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7fc2eb62658a - std::sys_common::backtrace::_print_fmt::heb8b816127c575ac
                               at /rustc/5fa44b54641cac7dc47964870d08b4ec82fc8157/library/std/src/sys_common/backtrace.rs:65:5
   3:     0x7fc2eb62658a - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hc6d5a81fe9ae4944
                               at /rustc/5fa44b54641cac7dc47964870d08b4ec82fc8157/library/std/src/sys_common/backtrace.rs:44:22
   4:     0x7fc2e75c8bae - core::fmt::write::hcafa8b859a98872c
                               at /rustc/5fa44b54641cac7dc47964870d08b4ec82fc8157/library/core/src/fmt/mod.rs:1208:17
   5:     0x7fc2eb61a855 - std::io::Write::write_fmt::h6673a9b6e958ba8f
                               at /rustc/5fa44b54641cac7dc47964870d08b4ec82fc8157/library/std/src/io/mod.rs:1682:15
   6:     0x7fc2eb626355 - std::sys_common::backtrace::_print::h354a664bdeea14b1
                               at /rustc/5fa44b54641cac7dc47964870d08b4ec82fc8157/library/std/src/sys_common/backtrace.rs:47:5
   7:     0x7fc2eb626355 - std::sys_common::backtrace::print::h4d0bbca313a44919
                               at /rustc/5fa44b54641cac7dc47964870d08b4ec82fc8157/library/std/src/sys_common/backtrace.rs:34:9
   8:     0x7fc2eb6286bf - std::panicking::default_hook::{{closure}}::h7a27de504bd62dbf
                               at /rustc/5fa44b54641cac7dc47964870d08b4ec82fc8157/library/std/src/panicking.rs:267:22
   9:     0x7fc2eb6283fb - std::panicking::default_hook::hbb70baf900f6c2f3
                               at /rustc/5fa44b54641cac7dc47964870d08b4ec82fc8157/library/std/src/panicking.rs:286:9
  10:     0x7fc2ea8692a4 - rustc_driver[3baf39caff1a6c1c]::DEFAULT_HOOK::{closure#0}::{closure#0}
  11:     0x7fc2eb628ebd - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::hfcf54b530484d26c
                               at /rustc/5fa44b54641cac7dc47964870d08b4ec82fc8157/library/alloc/src/boxed.rs:2032:9
  12:     0x7fc2eb628ebd - std::panicking::rust_panic_with_hook::h77d2e8b96839df52
                               at /rustc/5fa44b54641cac7dc47964870d08b4ec82fc8157/library/std/src/panicking.rs:692:13
  13:     0x7fc2ea8a1d51 - std[e181dace32bc9e68]::panicking::begin_panic::<rustc_errors[a8c29d7197cd76e7]::ExplicitBug>::{closure#0}
  14:     0x7fc2ea8a0686 - std[e181dace32bc9e68]::sys_common::backtrace::__rust_end_short_backtrace::<std[e181dace32bc9e68]::panicking::begin_panic<rustc_errors[a8c29d7197cd76e7]::ExplicitBug>::{closure#0}, !>
  15:     0x7fc2ea899276 - std[e181dace32bc9e68]::panicking::begin_panic::<rustc_errors[a8c29d7197cd76e7]::ExplicitBug>
  16:     0x7fc2ea89cae6 - std[e181dace32bc9e68]::panic::panic_any::<rustc_errors[a8c29d7197cd76e7]::ExplicitBug>
  17:     0x7fc2e9d7c426 - <rustc_errors[a8c29d7197cd76e7]::HandlerInner>::flush_delayed::<alloc[6669983853ed71da]::vec::Vec<rustc_errors[a8c29d7197cd76e7]::diagnostic::Diagnostic>, &str>
  18:     0x7fc2e9d7b72b - <rustc_errors[a8c29d7197cd76e7]::HandlerInner as core[272b256b08fe9f3e]::ops::drop::Drop>::drop
  19:     0x55f191ddd4fe - core[272b256b08fe9f3e]::ptr::drop_in_place::<rustc_session[e9234f890d84fb2b]::parse::ParseSess>
  20:     0x55f191dded9f - core[272b256b08fe9f3e]::ptr::drop_in_place::<rustc_session[e9234f890d84fb2b]::session::Session>
  21:     0x55f191a9c06d - core[272b256b08fe9f3e]::ptr::drop_in_place::<rustc_interface[d49f2a515602384d]::interface::Compiler>
  22:     0x55f191a897ea - <scoped_tls[761487f81bd1504c]::ScopedKey<rustc_span[86a07044cd7a2b96]::SessionGlobals>>::set::<rustc_interface[d49f2a515602384d]::interface::run_compiler<core[272b256b08fe9f3e]::result::Result<(), rustc_errors[a8c29d7197cd76e7]::ErrorGuaranteed>, rustdoc[362a79a2860e7de9]::main_args::{closure#1}>::{closure#0}, core[272b256b08fe9f3e]::result::Result<(), rustc_errors[a8c29d7197cd76e7]::ErrorGuaranteed>>
  23:     0x55f191c70ee0 - std[e181dace32bc9e68]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[d49f2a515602384d]::util::run_in_thread_pool_with_globals<rustc_interface[d49f2a515602384d]::interface::run_compiler<core[272b256b08fe9f3e]::result::Result<(), rustc_errors[a8c29d7197cd76e7]::ErrorGuaranteed>, rustdoc[362a79a2860e7de9]::main_args::{closure#1}>::{closure#0}, core[272b256b08fe9f3e]::result::Result<(), rustc_errors[a8c29d7197cd76e7]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[272b256b08fe9f3e]::result::Result<(), rustc_errors[a8c29d7197cd76e7]::ErrorGuaranteed>>
  24:     0x55f191da485d - <<std[e181dace32bc9e68]::thread::Builder>::spawn_unchecked_<rustc_interface[d49f2a515602384d]::util::run_in_thread_pool_with_globals<rustc_interface[d49f2a515602384d]::interface::run_compiler<core[272b256b08fe9f3e]::result::Result<(), rustc_errors[a8c29d7197cd76e7]::ErrorGuaranteed>, rustdoc[362a79a2860e7de9]::main_args::{closure#1}>::{closure#0}, core[272b256b08fe9f3e]::result::Result<(), rustc_errors[a8c29d7197cd76e7]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[272b256b08fe9f3e]::result::Result<(), rustc_errors[a8c29d7197cd76e7]::ErrorGuaranteed>>::{closure#1} as core[272b256b08fe9f3e]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  25:     0x7fc2e75731f3 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h6e99d9d46328d005
                               at /rustc/5fa44b54641cac7dc47964870d08b4ec82fc8157/library/alloc/src/boxed.rs:2000:9
  26:     0x7fc2e75731f3 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hc5c184713c4141ec
                               at /rustc/5fa44b54641cac7dc47964870d08b4ec82fc8157/library/alloc/src/boxed.rs:2000:9
  27:     0x7fc2e75731f3 - std::sys::unix::thread::Thread::new::thread_start::h18a504b023c5c43d
                               at /rustc/5fa44b54641cac7dc47964870d08b4ec82fc8157/library/std/src/sys/unix/thread.rs:108:17
  28:     0x7fc2e72e78fd - <unknown>
  29:     0x7fc2e7369a60 - <unknown>
  30:                0x0 - <unknown>

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.67.0-nightly (5fa44b546 2022-11-26) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z unstable-options

query stack during panic:
end of query stack
