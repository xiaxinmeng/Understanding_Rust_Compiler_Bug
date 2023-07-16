
   --> /Users/nlopes/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:490:1
    |
490 | pub unsafe fn drop_in_place<T: ?Sized>(to_drop: *mut T) {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: delayed at compiler/rustc_const_eval/src/transform/validate.rs:88:36

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1594:13
stack backtrace:
   0:        0x1018277b8 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h86f81dea2431b255
   1:        0x101878fa0 - core::fmt::write::h99cc000a520a5f0a
   2:        0x10181acbc - std::io::Write::write_fmt::h6dfe77e87020576e
   3:        0x1018275cc - std::sys_common::backtrace::print::h212135278ba731fd
   4:        0x10182a074 - std::panicking::default_hook::{{closure}}::he39bf9e88678c8e4
   5:        0x101829dcc - std::panicking::default_hook::hecb628a57e2b11ab
   6:        0x1099348f0 - rustc_driver[20b538c9ae0efabc]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:        0x10182a774 - std::panicking::rust_panic_with_hook::h281ab4d4a0a242eb
   8:        0x10d898238 - std[8354aa8059e9bd7]::panicking::begin_panic::<rustc_errors[498705a636aa56c4]::ExplicitBug>::{closure#0}
   9:        0x10d896938 - std[8354aa8059e9bd7]::sys_common::backtrace::__rust_end_short_backtrace::<std[8354aa8059e9bd7]::panicking::begin_panic<rustc_errors[498705a636aa56c4]::ExplicitBug>::{closure#0}, !>
  10:        0x10dc0b128 - std[8354aa8059e9bd7]::panicking::begin_panic::<rustc_errors[498705a636aa56c4]::ExplicitBug>
  11:        0x10d889240 - std[8354aa8059e9bd7]::panic::panic_any::<rustc_errors[498705a636aa56c4]::ExplicitBug>
  12:        0x10d88ff2c - <rustc_errors[498705a636aa56c4]::HandlerInner>::flush_delayed::<alloc[43fad65f40ecac15]::vec::Vec<rustc_errors[498705a636aa56c4]::diagnostic::Diagnostic>, &str>
  13:        0x10d88c86c - <rustc_errors[498705a636aa56c4]::HandlerInner as core[c1799017b12a3ae4]::ops::drop::Drop>::drop
  14:        0x1098c7ab4 - core[c1799017b12a3ae4]::ptr::drop_in_place::<rustc_session[d3c18a1d8780d037]::parse::ParseSess>
  15:        0x1098c8f50 - core[c1799017b12a3ae4]::ptr::drop_in_place::<rustc_session[d3c18a1d8780d037]::session::Session>
  16:        0x109923cfc - core[c1799017b12a3ae4]::ptr::drop_in_place::<rustc_interface[c57057a1c61fa02]::interface::Compiler>
  17:        0x1099102e0 - rustc_span[a0522e4eb001f528]::with_source_map::<core[c1799017b12a3ae4]::result::Result<(), rustc_errors[498705a636aa56c4]::ErrorGuaranteed>, rustc_interface[c57057a1c61fa02]::interface::run_compiler<core[c1799017b12a3ae4]::result::Result<(), rustc_errors[498705a636aa56c4]::ErrorGuaranteed>, rustc_driver[20b538c9ae0efabc]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  18:        0x1099082e0 - <scoped_tls[6e629ea917078905]::ScopedKey<rustc_span[a0522e4eb001f528]::SessionGlobals>>::set::<rustc_interface[c57057a1c61fa02]::interface::run_compiler<core[c1799017b12a3ae4]::result::Result<(), rustc_errors[498705a636aa56c4]::ErrorGuaranteed>, rustc_driver[20b538c9ae0efabc]::run_compiler::{closure#1}>::{closure#0}, core[c1799017b12a3ae4]::result::Result<(), rustc_errors[498705a636aa56c4]::ErrorGuaranteed>>
  19:        0x1098d6770 - std[8354aa8059e9bd7]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[c57057a1c61fa02]::util::run_in_thread_pool_with_globals<rustc_interface[c57057a1c61fa02]::interface::run_compiler<core[c1799017b12a3ae4]::result::Result<(), rustc_errors[498705a636aa56c4]::ErrorGuaranteed>, rustc_driver[20b538c9ae0efabc]::run_compiler::{closure#1}>::{closure#0}, core[c1799017b12a3ae4]::result::Result<(), rustc_errors[498705a636aa56c4]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[c1799017b12a3ae4]::result::Result<(), rustc_errors[498705a636aa56c4]::ErrorGuaranteed>>
  20:        0x1098bca94 - <<std[8354aa8059e9bd7]::thread::Builder>::spawn_unchecked_<rustc_interface[c57057a1c61fa02]::util::run_in_thread_pool_with_globals<rustc_interface[c57057a1c61fa02]::interface::run_compiler<core[c1799017b12a3ae4]::result::Result<(), rustc_errors[498705a636aa56c4]::ErrorGuaranteed>, rustc_driver[20b538c9ae0efabc]::run_compiler::{closure#1}>::{closure#0}, core[c1799017b12a3ae4]::result::Result<(), rustc_errors[498705a636aa56c4]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[c1799017b12a3ae4]::result::Result<(), rustc_errors[498705a636aa56c4]::ErrorGuaranteed>>::{closure#1} as core[c1799017b12a3ae4]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  21:        0x101832ab4 - std::sys::unix::thread::Thread::new::thread_start::hcb03e9b32fbe8c31
  22:        0x19eaca06c - __pthread_deallocate

note: the compiler unexpectedly panicked. this is a bug.
