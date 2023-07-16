plain
   Compiling alloc v0.0.0 (/checkout/library/alloc)
   Compiling cfg-if v0.1.10
   Compiling adler v0.2.3
   Compiling rustc-demangle v0.1.21
error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: `InferCtxt` incorrectly tainted by errors
  = note: delayed at compiler/rustc_infer/src/infer/mod.rs:1247:27

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1425:13
stack backtrace:
stack backtrace:
   0:     0x7f46fd0545bd - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hc4ffe1f8eabd47db
   1:     0x7f46fd0bcbd8 - core::fmt::write::h75736d5168df1a59
   2:     0x7f46fd0456c1 - std::io::Write::write_fmt::h64c00da7b551a7fc
   3:     0x7f46fd05777e - std::panicking::default_hook::{{closure}}::h242d629ae67a0bbd
   4:     0x7f46fd0574e1 - std::panicking::default_hook::hb3c378d91fbba2e0
   5:     0x7f46fdb6dcd4 - rustc_driver[7104356d7b7daba1]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f46fd057ef1 - std::panicking::rust_panic_with_hook::hfb7a27b956b22432
   7:     0x7f47008806a3 - std[3a002778014ff3f7]::panicking::begin_panic::<rustc_errors[2f3ca5e9cc262a90]::ExplicitBug>::{closure#0}
   8:     0x7f470087cce6 - std[3a002778014ff3f7]::sys_common::backtrace::__rust_end_short_backtrace::<std[3a002778014ff3f7]::panicking::begin_panic<rustc_errors[2f3ca5e9cc262a90]::ExplicitBug>::{closure#0}, !>
   9:     0x7f46fdb279b6 - std[3a002778014ff3f7]::panicking::begin_panic::<rustc_errors[2f3ca5e9cc262a90]::ExplicitBug>
  10:     0x7f470088a1f6 - std[3a002778014ff3f7]::panic::panic_any::<rustc_errors[2f3ca5e9cc262a90]::ExplicitBug>
  11:     0x7f470088e7ad - <rustc_errors[2f3ca5e9cc262a90]::HandlerInner as core[25bfd9c2f7020e11]::ops::drop::Drop>::drop
  12:     0x7f46fdb7be52 - core[25bfd9c2f7020e11]::ptr::drop_in_place::<rustc_session[1f00a067afe68450]::parse::ParseSess>
  13:     0x7f46fdb826c5 - <alloc[583b04903d8e2ac7]::rc::Rc<rustc_session[1f00a067afe68450]::session::Session> as core[25bfd9c2f7020e11]::ops::drop::Drop>::drop
  14:     0x7f46fdbe2e1c - core[25bfd9c2f7020e11]::ptr::drop_in_place::<rustc_interface[d32ad7ca6b1da85c]::interface::Compiler>
  15:     0x7f46fdbe0e57 - rustc_span[b4842b5572a45dc7]::with_source_map::<core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>, rustc_interface[d32ad7ca6b1da85c]::interface::create_compiler_and_run<core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>, rustc_driver[7104356d7b7daba1]::run_compiler::{closure#1}>::{closure#1}>
  16:     0x7f46fdb893da - <scoped_tls[63604573309c1f00]::ScopedKey<rustc_span[b4842b5572a45dc7]::SessionGlobals>>::set::<rustc_interface[d32ad7ca6b1da85c]::interface::run_compiler<core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>, rustc_driver[7104356d7b7daba1]::run_compiler::{closure#1}>::{closure#0}, core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>>
  17:     0x7f46fdbdf949 - std[3a002778014ff3f7]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[d32ad7ca6b1da85c]::util::run_in_thread_pool_with_globals<rustc_interface[d32ad7ca6b1da85c]::interface::run_compiler<core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>, rustc_driver[7104356d7b7daba1]::run_compiler::{closure#1}>::{closure#0}, core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>>::{closure#0}, core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>>
  18:     0x7f46fdbd7699 - <<std[3a002778014ff3f7]::thread::Builder>::spawn_unchecked_<rustc_interface[d32ad7ca6b1da85c]::util::run_in_thread_pool_with_globals<rustc_interface[d32ad7ca6b1da85c]::interface::run_compiler<core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>, rustc_driver[7104356d7b7daba1]::run_compiler::{closure#1}>::{closure#0}, core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>>::{closure#0}, core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>>::{closure#1} as core[25bfd9c2f7020e11]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  19:     0x7f46fd064185 - std::sys::unix::thread::Thread::new::thread_start::hc8cc1f68825154bd
  20:     0x7f46f75b3609 - start_thread
  21:     0x7f46fcec6133 - clone
  22:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.65.0-nightly (691e57263 2022-08-10) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -Z unstable-options -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
end of query stack
error: could not compile `adler`
warning: build failed, waiting for other jobs to finish...
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1425:13
stack backtrace:
   0:     0x7fdb1b9e75bd - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hc4ffe1f8eabd47db
   1:     0x7fdb1ba4fbd8 - core::fmt::write::h75736d5168df1a59
   2:     0x7fdb1b9d86c1 - std::io::Write::write_fmt::h64c00da7b551a7fc
   3:     0x7fdb1b9ea77e - std::panicking::default_hook::{{closure}}::h242d629ae67a0bbd
   4:     0x7fdb1b9ea4e1 - std::panicking::default_hook::hb3c378d91fbba2e0
   5:     0x7fdb1c500cd4 - rustc_driver[7104356d7b7daba1]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fdb1b9eaef1 - std::panicking::rust_panic_with_hook::hfb7a27b956b22432
   7:     0x7fdb1f2136a3 - std[3a002778014ff3f7]::panicking::begin_panic::<rustc_errors[2f3ca5e9cc262a90]::ExplicitBug>::{closure#0}
   8:     0x7fdb1f20fce6 - std[3a002778014ff3f7]::sys_common::backtrace::__rust_end_short_backtrace::<std[3a002778014ff3f7]::panicking::begin_panic<rustc_errors[2f3ca5e9cc262a90]::ExplicitBug>::{closure#0}, !>
   9:     0x7fdb1c4ba9b6 - std[3a002778014ff3f7]::panicking::begin_panic::<rustc_errors[2f3ca5e9cc262a90]::ExplicitBug>
  10:     0x7fdb1f21d1f6 - std[3a002778014ff3f7]::panic::panic_any::<rustc_errors[2f3ca5e9cc262a90]::ExplicitBug>
  11:     0x7fdb1f2217ad - <rustc_errors[2f3ca5e9cc262a90]::HandlerInner as core[25bfd9c2f7020e11]::ops::drop::Drop>::drop
  12:     0x7fdb1c50ee52 - core[25bfd9c2f7020e11]::ptr::drop_in_place::<rustc_session[1f00a067afe68450]::parse::ParseSess>
  13:     0x7fdb1c5156c5 - <alloc[583b04903d8e2ac7]::rc::Rc<rustc_session[1f00a067afe68450]::session::Session> as core[25bfd9c2f7020e11]::ops::drop::Drop>::drop
  14:     0x7fdb1c575e1c - core[25bfd9c2f7020e11]::ptr::drop_in_place::<rustc_interface[d32ad7ca6b1da85c]::interface::Compiler>
  15:     0x7fdb1c573e57 - rustc_span[b4842b5572a45dc7]::with_source_map::<core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>, rustc_interface[d32ad7ca6b1da85c]::interface::create_compiler_and_run<core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>, rustc_driver[7104356d7b7daba1]::run_compiler::{closure#1}>::{closure#1}>
  16:     0x7fdb1c51c3da - <scoped_tls[63604573309c1f00]::ScopedKey<rustc_span[b4842b5572a45dc7]::SessionGlobals>>::set::<rustc_interface[d32ad7ca6b1da85c]::interface::run_compiler<core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>, rustc_driver[7104356d7b7daba1]::run_compiler::{closure#1}>::{closure#0}, core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>>
  17:     0x7fdb1c572949 - std[3a002778014ff3f7]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[d32ad7ca6b1da85c]::util::run_in_thread_pool_with_globals<rustc_interface[d32ad7ca6b1da85c]::interface::run_compiler<core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>, rustc_driver[7104356d7b7daba1]::run_compiler::{closure#1}>::{closure#0}, core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>>::{closure#0}, core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>>
  18:     0x7fdb1c56a699 - <<std[3a002778014ff3f7]::thread::Builder>::spawn_unchecked_<rustc_interface[d32ad7ca6b1da85c]::util::run_in_thread_pool_with_globals<rustc_interface[d32ad7ca6b1da85c]::interface::run_compiler<core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>, rustc_driver[7104356d7b7daba1]::run_compiler::{closure#1}>::{closure#0}, core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>>::{closure#0}, core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>>::{closure#1} as core[25bfd9c2f7020e11]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  19:     0x7fdb1b9f7185 - std::sys::unix::thread::Thread::new::thread_start::hc8cc1f68825154bd
  20:     0x7fdb15f46609 - start_thread
  21:     0x7fdb1b859133 - clone
  22:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.65.0-nightly (691e57263 2022-08-10) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -Z unstable-options -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
end of query stack
error: could not compile `unwind`
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1425:13
stack backtrace:
   0:     0x7f94eba885bd - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hc4ffe1f8eabd47db
   1:     0x7f94ebaf0bd8 - core::fmt::write::h75736d5168df1a59
   2:     0x7f94eba796c1 - std::io::Write::write_fmt::h64c00da7b551a7fc
   3:     0x7f94eba8b77e - std::panicking::default_hook::{{closure}}::h242d629ae67a0bbd
   4:     0x7f94eba8b4e1 - std::panicking::default_hook::hb3c378d91fbba2e0
   5:     0x7f94ec5a1cd4 - rustc_driver[7104356d7b7daba1]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f94eba8bef1 - std::panicking::rust_panic_with_hook::hfb7a27b956b22432
   7:     0x7f94ef2b46a3 - std[3a002778014ff3f7]::panicking::begin_panic::<rustc_errors[2f3ca5e9cc262a90]::ExplicitBug>::{closure#0}
   8:     0x7f94ef2b0ce6 - std[3a002778014ff3f7]::sys_common::backtrace::__rust_end_short_backtrace::<std[3a002778014ff3f7]::panicking::begin_panic<rustc_errors[2f3ca5e9cc262a90]::ExplicitBug>::{closure#0}, !>
   9:     0x7f94ec55b9b6 - std[3a002778014ff3f7]::panicking::begin_panic::<rustc_errors[2f3ca5e9cc262a90]::ExplicitBug>
  10:     0x7f94ef2be1f6 - std[3a002778014ff3f7]::panic::panic_any::<rustc_errors[2f3ca5e9cc262a90]::ExplicitBug>
  11:     0x7f94ef2c27ad - <rustc_errors[2f3ca5e9cc262a90]::HandlerInner as core[25bfd9c2f7020e11]::ops::drop::Drop>::drop
  12:     0x7f94ec5afe52 - core[25bfd9c2f7020e11]::ptr::drop_in_place::<rustc_session[1f00a067afe68450]::parse::ParseSess>
  13:     0x7f94ec5b66c5 - <alloc[583b04903d8e2ac7]::rc::Rc<rustc_session[1f00a067afe68450]::session::Session> as core[25bfd9c2f7020e11]::ops::drop::Drop>::drop
  14:     0x7f94ec616e1c - core[25bfd9c2f7020e11]::ptr::drop_in_place::<rustc_interface[d32ad7ca6b1da85c]::interface::Compiler>
  15:     0x7f94ec614e57 - rustc_span[b4842b5572a45dc7]::with_source_map::<core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>, rustc_interface[d32ad7ca6b1da85c]::interface::create_compiler_and_run<core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>, rustc_driver[7104356d7b7daba1]::run_compiler::{closure#1}>::{closure#1}>
  16:     0x7f94ec5bd3da - <scoped_tls[63604573309c1f00]::ScopedKey<rustc_span[b4842b5572a45dc7]::SessionGlobals>>::set::<rustc_interface[d32ad7ca6b1da85c]::interface::run_compiler<core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>, rustc_driver[7104356d7b7daba1]::run_compiler::{closure#1}>::{closure#0}, core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>>
  17:     0x7f94ec613949 - std[3a002778014ff3f7]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[d32ad7ca6b1da85c]::util::run_in_thread_pool_with_globals<rustc_interface[d32ad7ca6b1da85c]::interface::run_compiler<core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>, rustc_driver[7104356d7b7daba1]::run_compiler::{closure#1}>::{closure#0}, core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>>::{closure#0}, core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>>
  18:     0x7f94ec60b699 - <<std[3a002778014ff3f7]::thread::Builder>::spawn_unchecked_<rustc_interface[d32ad7ca6b1da85c]::util::run_in_thread_pool_with_globals<rustc_interface[d32ad7ca6b1da85c]::interface::run_compiler<core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>, rustc_driver[7104356d7b7daba1]::run_compiler::{closure#1}>::{closure#0}, core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>>::{closure#0}, core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>>::{closure#1} as core[25bfd9c2f7020e11]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  19:     0x7f94eba98185 - std::sys::unix::thread::Thread::new::thread_start::hc8cc1f68825154bd
  20:     0x7f94e5fe7609 - start_thread
  21:     0x7f94eb8fa133 - clone
  22:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.65.0-nightly (691e57263 2022-08-10) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -Z unstable-options -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
end of query stack
error: could not compile `libc`
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1425:13
stack backtrace:
   0:     0x7f26da58b5bd - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hc4ffe1f8eabd47db
   1:     0x7f26da5f3bd8 - core::fmt::write::h75736d5168df1a59
   2:     0x7f26da57c6c1 - std::io::Write::write_fmt::h64c00da7b551a7fc
   3:     0x7f26da58e77e - std::panicking::default_hook::{{closure}}::h242d629ae67a0bbd
   4:     0x7f26da58e4e1 - std::panicking::default_hook::hb3c378d91fbba2e0
   5:     0x7f26db0a4cd4 - rustc_driver[7104356d7b7daba1]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f26da58eef1 - std::panicking::rust_panic_with_hook::hfb7a27b956b22432
   7:     0x7f26dddb76a3 - std[3a002778014ff3f7]::panicking::begin_panic::<rustc_errors[2f3ca5e9cc262a90]::ExplicitBug>::{closure#0}
   8:     0x7f26dddb3ce6 - std[3a002778014ff3f7]::sys_common::backtrace::__rust_end_short_backtrace::<std[3a002778014ff3f7]::panicking::begin_panic<rustc_errors[2f3ca5e9cc262a90]::ExplicitBug>::{closure#0}, !>
   9:     0x7f26db05e9b6 - std[3a002778014ff3f7]::panicking::begin_panic::<rustc_errors[2f3ca5e9cc262a90]::ExplicitBug>
  10:     0x7f26dddc11f6 - std[3a002778014ff3f7]::panic::panic_any::<rustc_errors[2f3ca5e9cc262a90]::ExplicitBug>
  11:     0x7f26dddc57ad - <rustc_errors[2f3ca5e9cc262a90]::HandlerInner as core[25bfd9c2f7020e11]::ops::drop::Drop>::drop
  12:     0x7f26db0b2e52 - core[25bfd9c2f7020e11]::ptr::drop_in_place::<rustc_session[1f00a067afe68450]::parse::ParseSess>
  13:     0x7f26db0b96c5 - <alloc[583b04903d8e2ac7]::rc::Rc<rustc_session[1f00a067afe68450]::session::Session> as core[25bfd9c2f7020e11]::ops::drop::Drop>::drop
  14:     0x7f26db119e1c - core[25bfd9c2f7020e11]::ptr::drop_in_place::<rustc_interface[d32ad7ca6b1da85c]::interface::Compiler>
  15:     0x7f26db117e57 - rustc_span[b4842b5572a45dc7]::with_source_map::<core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>, rustc_interface[d32ad7ca6b1da85c]::interface::create_compiler_and_run<core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>, rustc_driver[7104356d7b7daba1]::run_compiler::{closure#1}>::{closure#1}>
  16:     0x7f26db0c03da - <scoped_tls[63604573309c1f00]::ScopedKey<rustc_span[b4842b5572a45dc7]::SessionGlobals>>::set::<rustc_interface[d32ad7ca6b1da85c]::interface::run_compiler<core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>, rustc_driver[7104356d7b7daba1]::run_compiler::{closure#1}>::{closure#0}, core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>>
  17:     0x7f26db116949 - std[3a002778014ff3f7]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[d32ad7ca6b1da85c]::util::run_in_thread_pool_with_globals<rustc_interface[d32ad7ca6b1da85c]::interface::run_compiler<core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>, rustc_driver[7104356d7b7daba1]::run_compiler::{closure#1}>::{closure#0}, core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>>::{closure#0}, core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>>
  18:     0x7f26db10e699 - <<std[3a002778014ff3f7]::thread::Builder>::spawn_unchecked_<rustc_interface[d32ad7ca6b1da85c]::util::run_in_thread_pool_with_globals<rustc_interface[d32ad7ca6b1da85c]::interface::run_compiler<core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>, rustc_driver[7104356d7b7daba1]::run_compiler::{closure#1}>::{closure#0}, core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>>::{closure#0}, core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>>::{closure#1} as core[25bfd9c2f7020e11]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  19:     0x7f26da59b185 - std::sys::unix::thread::Thread::new::thread_start::hc8cc1f68825154bd
  20:     0x7f26d4aea609 - start_thread
  21:     0x7f26da3fd133 - clone
  22:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.65.0-nightly (691e57263 2022-08-10) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=10000 -C debuginfo=0 -Z unstable-options -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -Z unstable-options -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -C panic=abort -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
end of query stack
error: could not compile `compiler_builtins`
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1425:13
stack backtrace:
   0:     0x7f23ff3d35bd - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hc4ffe1f8eabd47db
   1:     0x7f23ff43bbd8 - core::fmt::write::h75736d5168df1a59
   2:     0x7f23ff3c46c1 - std::io::Write::write_fmt::h64c00da7b551a7fc
   3:     0x7f23ff3d677e - std::panicking::default_hook::{{closure}}::h242d629ae67a0bbd
   4:     0x7f23ff3d64e1 - std::panicking::default_hook::hb3c378d91fbba2e0
   5:     0x7f23ffeeccd4 - rustc_driver[7104356d7b7daba1]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f23ff3d6ef1 - std::panicking::rust_panic_with_hook::hfb7a27b956b22432
   7:     0x7f2402bff6a3 - std[3a002778014ff3f7]::panicking::begin_panic::<rustc_errors[2f3ca5e9cc262a90]::ExplicitBug>::{closure#0}
   8:     0x7f2402bfbce6 - std[3a002778014ff3f7]::sys_common::backtrace::__rust_end_short_backtrace::<std[3a002778014ff3f7]::panicking::begin_panic<rustc_errors[2f3ca5e9cc262a90]::ExplicitBug>::{closure#0}, !>
   9:     0x7f23ffea69b6 - std[3a002778014ff3f7]::panicking::begin_panic::<rustc_errors[2f3ca5e9cc262a90]::ExplicitBug>
  10:     0x7f2402c091f6 - std[3a002778014ff3f7]::panic::panic_any::<rustc_errors[2f3ca5e9cc262a90]::ExplicitBug>
  11:     0x7f2402c0d7ad - <rustc_errors[2f3ca5e9cc262a90]::HandlerInner as core[25bfd9c2f7020e11]::ops::drop::Drop>::drop
  12:     0x7f23ffefae52 - core[25bfd9c2f7020e11]::ptr::drop_in_place::<rustc_session[1f00a067afe68450]::parse::ParseSess>
  13:     0x7f23fff016c5 - <alloc[583b04903d8e2ac7]::rc::Rc<rustc_session[1f00a067afe68450]::session::Session> as core[25bfd9c2f7020e11]::ops::drop::Drop>::drop
  14:     0x7f23fff61e1c - core[25bfd9c2f7020e11]::ptr::drop_in_place::<rustc_interface[d32ad7ca6b1da85c]::interface::Compiler>
  15:     0x7f23fff5fe57 - rustc_span[b4842b5572a45dc7]::with_source_map::<core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>, rustc_interface[d32ad7ca6b1da85c]::interface::create_compiler_and_run<core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>, rustc_driver[7104356d7b7daba1]::run_compiler::{closure#1}>::{closure#1}>
  16:     0x7f23fff083da - <scoped_tls[63604573309c1f00]::ScopedKey<rustc_span[b4842b5572a45dc7]::SessionGlobals>>::set::<rustc_interface[d32ad7ca6b1da85c]::interface::run_compiler<core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>, rustc_driver[7104356d7b7daba1]::run_compiler::{closure#1}>::{closure#0}, core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>>
  17:     0x7f23fff5e949 - std[3a002778014ff3f7]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[d32ad7ca6b1da85c]::util::run_in_thread_pool_with_globals<rustc_interface[d32ad7ca6b1da85c]::interface::run_compiler<core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>, rustc_driver[7104356d7b7daba1]::run_compiler::{closure#1}>::{closure#0}, core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>>::{closure#0}, core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>>
  18:     0x7f23fff56699 - <<std[3a002778014ff3f7]::thread::Builder>::spawn_unchecked_<rustc_interface[d32ad7ca6b1da85c]::util::run_in_thread_pool_with_globals<rustc_interface[d32ad7ca6b1da85c]::interface::run_compiler<core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>, rustc_driver[7104356d7b7daba1]::run_compiler::{closure#1}>::{closure#0}, core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>>::{closure#0}, core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>>::{closure#1} as core[25bfd9c2f7020e11]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  19:     0x7f23ff3e3185 - std::sys::unix::thread::Thread::new::thread_start::hc8cc1f68825154bd
  20:     0x7f23f9932609 - start_thread
  21:     0x7f23ff245133 - clone
  22:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.65.0-nightly (691e57263 2022-08-10) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -Z unstable-options -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
end of query stack
error: could not compile `memchr`
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1425:13
stack backtrace:
   0:     0x7ff57e8a55bd - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hc4ffe1f8eabd47db
   1:     0x7ff57e90dbd8 - core::fmt::write::h75736d5168df1a59
   2:     0x7ff57e8966c1 - std::io::Write::write_fmt::h64c00da7b551a7fc
   3:     0x7ff57e8a877e - std::panicking::default_hook::{{closure}}::h242d629ae67a0bbd
   4:     0x7ff57e8a84e1 - std::panicking::default_hook::hb3c378d91fbba2e0
   5:     0x7ff57f3becd4 - rustc_driver[7104356d7b7daba1]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7ff57e8a8ef1 - std::panicking::rust_panic_with_hook::hfb7a27b956b22432
   7:     0x7ff5820d16a3 - std[3a002778014ff3f7]::panicking::begin_panic::<rustc_errors[2f3ca5e9cc262a90]::ExplicitBug>::{closure#0}
   8:     0x7ff5820cdce6 - std[3a002778014ff3f7]::sys_common::backtrace::__rust_end_short_backtrace::<std[3a002778014ff3f7]::panicking::begin_panic<rustc_errors[2f3ca5e9cc262a90]::ExplicitBug>::{closure#0}, !>
   9:     0x7ff57f3789b6 - std[3a002778014ff3f7]::panicking::begin_panic::<rustc_errors[2f3ca5e9cc262a90]::ExplicitBug>
  10:     0x7ff5820db1f6 - std[3a002778014ff3f7]::panic::panic_any::<rustc_errors[2f3ca5e9cc262a90]::ExplicitBug>
  11:     0x7ff5820df7ad - <rustc_errors[2f3ca5e9cc262a90]::HandlerInner as core[25bfd9c2f7020e11]::ops::drop::Drop>::drop
  12:     0x7ff57f3cce52 - core[25bfd9c2f7020e11]::ptr::drop_in_place::<rustc_session[1f00a067afe68450]::parse::ParseSess>
  13:     0x7ff57f3d36c5 - <alloc[583b04903d8e2ac7]::rc::Rc<rustc_session[1f00a067afe68450]::session::Session> as core[25bfd9c2f7020e11]::ops::drop::Drop>::drop
  14:     0x7ff57f433e1c - core[25bfd9c2f7020e11]::ptr::drop_in_place::<rustc_interface[d32ad7ca6b1da85c]::interface::Compiler>
  15:     0x7ff57f431e57 - rustc_span[b4842b5572a45dc7]::with_source_map::<core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>, rustc_interface[d32ad7ca6b1da85c]::interface::create_compiler_and_run<core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>, rustc_driver[7104356d7b7daba1]::run_compiler::{closure#1}>::{closure#1}>
  16:     0x7ff57f3da3da - <scoped_tls[63604573309c1f00]::ScopedKey<rustc_span[b4842b5572a45dc7]::SessionGlobals>>::set::<rustc_interface[d32ad7ca6b1da85c]::interface::run_compiler<core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>, rustc_driver[7104356d7b7daba1]::run_compiler::{closure#1}>::{closure#0}, core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>>
  17:     0x7ff57f430949 - std[3a002778014ff3f7]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[d32ad7ca6b1da85c]::util::run_in_thread_pool_with_globals<rustc_interface[d32ad7ca6b1da85c]::interface::run_compiler<core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>, rustc_driver[7104356d7b7daba1]::run_compiler::{closure#1}>::{closure#0}, core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>>::{closure#0}, core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>>
  18:     0x7ff57f428699 - <<std[3a002778014ff3f7]::thread::Builder>::spawn_unchecked_<rustc_interface[d32ad7ca6b1da85c]::util::run_in_thread_pool_with_globals<rustc_interface[d32ad7ca6b1da85c]::interface::run_compiler<core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>, rustc_driver[7104356d7b7daba1]::run_compiler::{closure#1}>::{closure#0}, core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>>::{closure#0}, core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>>::{closure#1} as core[25bfd9c2f7020e11]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  19:     0x7ff57e8b5185 - std::sys::unix::thread::Thread::new::thread_start::hc8cc1f68825154bd
  20:     0x7ff578e04609 - start_thread
  21:     0x7ff57e717133 - clone
  22:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.65.0-nightly (691e57263 2022-08-10) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -Z unstable-options -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
end of query stack
error: could not compile `rustc-demangle`
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1425:13
stack backtrace:
   0:     0x7fc46c9cf5bd - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hc4ffe1f8eabd47db
   1:     0x7fc46ca37bd8 - core::fmt::write::h75736d5168df1a59
   2:     0x7fc46c9c06c1 - std::io::Write::write_fmt::h64c00da7b551a7fc
   3:     0x7fc46c9d277e - std::panicking::default_hook::{{closure}}::h242d629ae67a0bbd
   4:     0x7fc46c9d24e1 - std::panicking::default_hook::hb3c378d91fbba2e0
   5:     0x7fc46d4e8cd4 - rustc_driver[7104356d7b7daba1]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fc46c9d2ef1 - std::panicking::rust_panic_with_hook::hfb7a27b956b22432
   7:     0x7fc4701fb6a3 - std[3a002778014ff3f7]::panicking::begin_panic::<rustc_errors[2f3ca5e9cc262a90]::ExplicitBug>::{closure#0}
   8:     0x7fc4701f7ce6 - std[3a002778014ff3f7]::sys_common::backtrace::__rust_end_short_backtrace::<std[3a002778014ff3f7]::panicking::begin_panic<rustc_errors[2f3ca5e9cc262a90]::ExplicitBug>::{closure#0}, !>
   9:     0x7fc46d4a29b6 - std[3a002778014ff3f7]::panicking::begin_panic::<rustc_errors[2f3ca5e9cc262a90]::ExplicitBug>
  10:     0x7fc4702051f6 - std[3a002778014ff3f7]::panic::panic_any::<rustc_errors[2f3ca5e9cc262a90]::ExplicitBug>
  11:     0x7fc4702097ad - <rustc_errors[2f3ca5e9cc262a90]::HandlerInner as core[25bfd9c2f7020e11]::ops::drop::Drop>::drop
  12:     0x7fc46d4f6e52 - core[25bfd9c2f7020e11]::ptr::drop_in_place::<rustc_session[1f00a067afe68450]::parse::ParseSess>
  13:     0x7fc46d4fd6c5 - <alloc[583b04903d8e2ac7]::rc::Rc<rustc_session[1f00a067afe68450]::session::Session> as core[25bfd9c2f7020e11]::ops::drop::Drop>::drop
  14:     0x7fc46d55de1c - core[25bfd9c2f7020e11]::ptr::drop_in_place::<rustc_interface[d32ad7ca6b1da85c]::interface::Compiler>
  15:     0x7fc46d55be57 - rustc_span[b4842b5572a45dc7]::with_source_map::<core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>, rustc_interface[d32ad7ca6b1da85c]::interface::create_compiler_and_run<core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>, rustc_driver[7104356d7b7daba1]::run_compiler::{closure#1}>::{closure#1}>
  16:     0x7fc46d5043da - <scoped_tls[63604573309c1f00]::ScopedKey<rustc_span[b4842b5572a45dc7]::SessionGlobals>>::set::<rustc_interface[d32ad7ca6b1da85c]::interface::run_compiler<core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>, rustc_driver[7104356d7b7daba1]::run_compiler::{closure#1}>::{closure#0}, core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>>
  17:     0x7fc46d55a949 - std[3a002778014ff3f7]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[d32ad7ca6b1da85c]::util::run_in_thread_pool_with_globals<rustc_interface[d32ad7ca6b1da85c]::interface::run_compiler<core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>, rustc_driver[7104356d7b7daba1]::run_compiler::{closure#1}>::{closure#0}, core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>>::{closure#0}, core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>>
  18:     0x7fc46d552699 - <<std[3a002778014ff3f7]::thread::Builder>::spawn_unchecked_<rustc_interface[d32ad7ca6b1da85c]::util::run_in_thread_pool_with_globals<rustc_interface[d32ad7ca6b1da85c]::interface::run_compiler<core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>, rustc_driver[7104356d7b7daba1]::run_compiler::{closure#1}>::{closure#0}, core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>>::{closure#0}, core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>>::{closure#1} as core[25bfd9c2f7020e11]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  19:     0x7fc46c9df185 - std::sys::unix::thread::Thread::new::thread_start::hc8cc1f68825154bd
  20:     0x7fc466f2e609 - start_thread
  21:     0x7fc46c841133 - clone
  22:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.65.0-nightly (691e57263 2022-08-10) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -Z unstable-options -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
end of query stack
error: could not compile `alloc`
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1425:13
stack backtrace:
   0:     0x7f2eec0be5bd - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hc4ffe1f8eabd47db
   1:     0x7f2eec126bd8 - core::fmt::write::h75736d5168df1a59
   2:     0x7f2eec0af6c1 - std::io::Write::write_fmt::h64c00da7b551a7fc
   3:     0x7f2eec0c177e - std::panicking::default_hook::{{closure}}::h242d629ae67a0bbd
   4:     0x7f2eec0c14e1 - std::panicking::default_hook::hb3c378d91fbba2e0
   5:     0x7f2eecbd7cd4 - rustc_driver[7104356d7b7daba1]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f2eec0c1ef1 - std::panicking::rust_panic_with_hook::hfb7a27b956b22432
   7:     0x7f2eef8ea6a3 - std[3a002778014ff3f7]::panicking::begin_panic::<rustc_errors[2f3ca5e9cc262a90]::ExplicitBug>::{closure#0}
   8:     0x7f2eef8e6ce6 - std[3a002778014ff3f7]::sys_common::backtrace::__rust_end_short_backtrace::<std[3a002778014ff3f7]::panicking::begin_panic<rustc_errors[2f3ca5e9cc262a90]::ExplicitBug>::{closure#0}, !>
   9:     0x7f2eecb919b6 - std[3a002778014ff3f7]::panicking::begin_panic::<rustc_errors[2f3ca5e9cc262a90]::ExplicitBug>
  10:     0x7f2eef8f41f6 - std[3a002778014ff3f7]::panic::panic_any::<rustc_errors[2f3ca5e9cc262a90]::ExplicitBug>
  11:     0x7f2eef8f87ad - <rustc_errors[2f3ca5e9cc262a90]::HandlerInner as core[25bfd9c2f7020e11]::ops::drop::Drop>::drop
  12:     0x7f2eecbe5e52 - core[25bfd9c2f7020e11]::ptr::drop_in_place::<rustc_session[1f00a067afe68450]::parse::ParseSess>
  13:     0x7f2eecbec6c5 - <alloc[583b04903d8e2ac7]::rc::Rc<rustc_session[1f00a067afe68450]::session::Session> as core[25bfd9c2f7020e11]::ops::drop::Drop>::drop
  14:     0x7f2eecc4ce1c - core[25bfd9c2f7020e11]::ptr::drop_in_place::<rustc_interface[d32ad7ca6b1da85c]::interface::Compiler>
  15:     0x7f2eecc4ae57 - rustc_span[b4842b5572a45dc7]::with_source_map::<core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>, rustc_interface[d32ad7ca6b1da85c]::interface::create_compiler_and_run<core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>, rustc_driver[7104356d7b7daba1]::run_compiler::{closure#1}>::{closure#1}>
  16:     0x7f2eecbf33da - <scoped_tls[63604573309c1f00]::ScopedKey<rustc_span[b4842b5572a45dc7]::SessionGlobals>>::set::<rustc_interface[d32ad7ca6b1da85c]::interface::run_compiler<core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>, rustc_driver[7104356d7b7daba1]::run_compiler::{closure#1}>::{closure#0}, core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>>
  17:     0x7f2eecc49949 - std[3a002778014ff3f7]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[d32ad7ca6b1da85c]::util::run_in_thread_pool_with_globals<rustc_interface[d32ad7ca6b1da85c]::interface::run_compiler<core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>, rustc_driver[7104356d7b7daba1]::run_compiler::{closure#1}>::{closure#0}, core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>>::{closure#0}, core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>>
  18:     0x7f2eecc41699 - <<std[3a002778014ff3f7]::thread::Builder>::spawn_unchecked_<rustc_interface[d32ad7ca6b1da85c]::util::run_in_thread_pool_with_globals<rustc_interface[d32ad7ca6b1da85c]::interface::run_compiler<core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>, rustc_driver[7104356d7b7daba1]::run_compiler::{closure#1}>::{closure#0}, core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>>::{closure#0}, core[25bfd9c2f7020e11]::result::Result<(), rustc_errors[2f3ca5e9cc262a90]::ErrorGuaranteed>>::{closure#1} as core[25bfd9c2f7020e11]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  19:     0x7f2eec0ce185 - std::sys::unix::thread::Thread::new::thread_start::hc8cc1f68825154bd
  20:     0x7f2ee661d609 - start_thread
  21:     0x7f2eebf30133 - clone
  22:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.65.0-nightly (691e57263 2022-08-10) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -Z unstable-options -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
