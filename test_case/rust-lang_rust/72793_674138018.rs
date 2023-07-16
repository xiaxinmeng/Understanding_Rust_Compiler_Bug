`
error: internal compiler error: Encountered error `Unimplemented` selecting `Binder(<impl Fn(Alias<'_>) as std::ops::Fn<(impl T,)>>)` during codegen
  |
  = note: delayed at /home/matthias/vcs/github/rust/src/librustc_session/session.rs:441:27

thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:366:17
stack backtrace:
   0:     0x7f6662e2ae31 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h9a65342185cb7022
   1:     0x7f6662e999dd - core::fmt::write::hc3de054a6627b208
   2:     0x7f6662e0ab53 - std::io::Write::write_fmt::h64bd8d8dca95fd5c
   3:     0x7f6662e03bb0 - std::panicking::default_hook::{{closure}}::ha9c6947a872dc513
   4:     0x7f6662e037ed - std::panicking::default_hook::h3e5e46d076456208
   5:     0x7f6663ef0269 - rustc_driver::report_ice::he0572c9ff71e0a8c
   6:     0x7f6662e041f6 - std::panicking::rust_panic_with_hook::h30c558028146672c
   7:     0x7f6666991a56 - std::panicking::begin_panic::{{closure}}::h62cde3015c32ea83
   8:     0x7f666699191e - std::sys_common::backtrace::__rust_end_short_backtrace::h8d34e3f6f4e20c59
   9:     0x7f666696859c - std::panicking::begin_panic::h6e7f1f8f159dddb1
  10:     0x7f6666971dae - <rustc_errors::HandlerInner as core::ops::drop::Drop>::drop::h5013360fb564f32c
  11:     0x7f6663d4a3d6 - core::ptr::drop_in_place::h4e9bf5d5a9a1b0b3
  12:     0x7f6663d6dad0 - <alloc::rc::Rc<T> as core::ops::drop::Drop>::drop::h0ba476d8189f3dd5
  13:     0x7f6663ec6a4d - core::ptr::drop_in_place::hb83cdb4088f009cf
  14:     0x7f6663eacc92 - rustc_span::with_source_map::hdfa547cd4d1341a2
  15:     0x7f6663d24038 - rustc_interface::interface::create_compiler_and_run::h10b475d531695c0e
  16:     0x7f6663eb7bfd - std::sys_common::backtrace::__rust_begin_short_backtrace::h4c9162f49c0dc935
  17:     0x7f6663eba9be - core::ops::function::FnOnce::call_once{{vtable.shim}}::hbba71a1842b71242
  18:     0x7f6662e05cda - std::sys::unix::thread::Thread::new::thread_start::hdc96d124dc384f80
  19:     0x7f665fc04422 - start_thread
  20:     0x7f6662c07bf3 - __clone
  21:                0x0 - <unknown>

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.47.0-dev running on x86_64-unknown-linux-gnu

note: compiler flags: -Z mir-opt-level=2

query stack during panic:
end of query stack
