plain
failures:

---- [incremental] incremental/delayed_span_bug.rs stdout ----

error in revision `cfail1`: Error: expected failure status (Some(101)) but received status None.
status: signal: 6 (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/delayed_span_bug.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail1" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/delayed_span_bug/delayed_span_bug.inc" "-Z" "incremental-verify-ich" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/delayed_span_bug" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/delayed_span_bug/auxiliary"
stdout: none
--- stderr -------------------------------
warning: Error finalizing incremental compilation session directory `/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/delayed_span_bug/delayed_span_bug.inc/delayed_span_bug-3iuktwy7w0wpu/s-g85jft81jz-fkftbz-working`: No such file or directory (os error 2)
warning: 1 warning emitted


error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: delayed span bug triggered by #[rustc_error(delay_span_bug_from_inside_query)]
   |
LL | fn main() {}
   | ^^^^^^^^^
   |
   |
   = note: delayed at compiler/rustc_middle/src/util/bug.rs:45:14

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1291:13
stack backtrace:
   0:     0x7f317f0d05f2 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hbaf39c7e135a5f29
   1:     0x7f317f1381bf - core::fmt::write::h17fceefd44a8563d
   2:     0x7f317f0bf803 - std::io::Write::write_fmt::hce4aaa025715dd55
   3:     0x7f317f0d3b9c - std::panicking::default_hook::{{closure}}::h710259246eb13b44
   4:     0x7f317f0d36ed - std::panicking::default_hook::h9c7fbd10a3b26c0c
   5:     0x7f317fb7a3b1 - rustc_driver[8ed9b98a7c93dd6b]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f317f0d45ed - std::panicking::rust_panic_with_hook::hb63e1080147ab8fb
   7:     0x7f31823ab313 - std[45702258e1bbc8d]::panicking::begin_panic::<rustc_errors[9bd6aca994b05f94]::ExplicitBug>::{closure#0}
   8:     0x7f31823aa216 - std[45702258e1bbc8d]::sys_common::backtrace::__rust_end_short_backtrace::<std[45702258e1bbc8d]::panicking::begin_panic<rustc_errors[9bd6aca994b05f94]::ExplicitBug>::{closure#0}, !>
   9:     0x7f317fb3138f - std[45702258e1bbc8d]::panicking::begin_panic::<rustc_errors[9bd6aca994b05f94]::ExplicitBug>
  10:     0x7f31823a0996 - std[45702258e1bbc8d]::panic::panic_any::<rustc_errors[9bd6aca994b05f94]::ExplicitBug>
  11:     0x7f31823a4d32 - <rustc_errors[9bd6aca994b05f94]::HandlerInner as core[a65aa9227bae4abc]::ops::drop::Drop>::drop
  12:     0x7f317fb8fea0 - <alloc[d149d8c8d6a713e7]::rc::Rc<rustc_session[d666950372548c5f]::session::Session> as core[a65aa9227bae4abc]::ops::drop::Drop>::drop
  13:     0x7f317fbccf49 - core[a65aa9227bae4abc]::ptr::drop_in_place::<rustc_interface[f56f2f0b26d6c35e]::interface::Compiler>
  14:     0x7f317fbc9e57 - rustc_span[6d3d3d5a9e908e9c]::with_source_map::<core[a65aa9227bae4abc]::result::Result<(), rustc_errors[9bd6aca994b05f94]::ErrorGuaranteed>, rustc_interface[f56f2f0b26d6c35e]::interface::create_compiler_and_run<core[a65aa9227bae4abc]::result::Result<(), rustc_errors[9bd6aca994b05f94]::ErrorGuaranteed>, rustc_driver[8ed9b98a7c93dd6b]::run_compiler::{closure#1}>::{closure#1}>
  15:     0x7f317fbbc0b0 - <scoped_tls[433840387f49a653]::ScopedKey<rustc_span[6d3d3d5a9e908e9c]::SessionGlobals>>::set::<rustc_interface[f56f2f0b26d6c35e]::interface::run_compiler<core[a65aa9227bae4abc]::result::Result<(), rustc_errors[9bd6aca994b05f94]::ErrorGuaranteed>, rustc_driver[8ed9b98a7c93dd6b]::run_compiler::{closure#1}>::{closure#0}, core[a65aa9227bae4abc]::result::Result<(), rustc_errors[9bd6aca994b05f94]::ErrorGuaranteed>>
  16:     0x7f317fb8c249 - std[45702258e1bbc8d]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[f56f2f0b26d6c35e]::util::run_in_thread_pool_with_globals<rustc_interface[f56f2f0b26d6c35e]::interface::run_compiler<core[a65aa9227bae4abc]::result::Result<(), rustc_errors[9bd6aca994b05f94]::ErrorGuaranteed>, rustc_driver[8ed9b98a7c93dd6b]::run_compiler::{closure#1}>::{closure#0}, core[a65aa9227bae4abc]::result::Result<(), rustc_errors[9bd6aca994b05f94]::ErrorGuaranteed>>::{closure#0}, core[a65aa9227bae4abc]::result::Result<(), rustc_errors[9bd6aca994b05f94]::ErrorGuaranteed>>
  17:     0x7f317fbd6a31 - std[45702258e1bbc8d]::panicking::try::<core[a65aa9227bae4abc]::result::Result<(), rustc_errors[9bd6aca994b05f94]::ErrorGuaranteed>, core[a65aa9227bae4abc]::panic::unwind_safe::AssertUnwindSafe<<std[45702258e1bbc8d]::thread::Builder>::spawn_unchecked_<rustc_interface[f56f2f0b26d6c35e]::util::run_in_thread_pool_with_globals<rustc_interface[f56f2f0b26d6c35e]::interface::run_compiler<core[a65aa9227bae4abc]::result::Result<(), rustc_errors[9bd6aca994b05f94]::ErrorGuaranteed>, rustc_driver[8ed9b98a7c93dd6b]::run_compiler::{closure#1}>::{closure#0}, core[a65aa9227bae4abc]::result::Result<(), rustc_errors[9bd6aca994b05f94]::ErrorGuaranteed>>::{closure#0}, core[a65aa9227bae4abc]::result::Result<(), rustc_errors[9bd6aca994b05f94]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  18:     0x7f317fbcf271 - <<std[45702258e1bbc8d]::thread::Builder>::spawn_unchecked_<rustc_interface[f56f2f0b26d6c35e]::util::run_in_thread_pool_with_globals<rustc_interface[f56f2f0b26d6c35e]::interface::run_compiler<core[a65aa9227bae4abc]::result::Result<(), rustc_errors[9bd6aca994b05f94]::ErrorGuaranteed>, rustc_driver[8ed9b98a7c93dd6b]::run_compiler::{closure#1}>::{closure#0}, core[a65aa9227bae4abc]::result::Result<(), rustc_errors[9bd6aca994b05f94]::ErrorGuaranteed>>::{closure#0}, core[a65aa9227bae4abc]::result::Result<(), rustc_errors[9bd6aca994b05f94]::ErrorGuaranteed>>::{closure#1} as core[a65aa9227bae4abc]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  19:     0x7f317f0dfa43 - std::sys::unix::thread::Thread::new::thread_start::hb758e3f4652c5e3b
  20:     0x7f317962b609 - start_thread
  21:     0x7f317ef3e163 - clone
  22:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.61.0-nightly (8ab5939ba 2022-03-22) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C incremental -Z incremental -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
thread 'rustc' panicked at 'panic in a function that cannot unwind', library/core/src/panicking.rs:107:58
stack backtrace:
stack backtrace:
   0:     0x7f317f0d05f2 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hbaf39c7e135a5f29
   1:     0x7f317f1381bf - core::fmt::write::h17fceefd44a8563d
   2:     0x7f317f0bf803 - std::io::Write::write_fmt::hce4aaa025715dd55
   3:     0x7f317f0d3b9c - std::panicking::default_hook::{{closure}}::h710259246eb13b44
   4:     0x7f317f0d36ed - std::panicking::default_hook::h9c7fbd10a3b26c0c
   5:     0x7f317fb7a3b1 - rustc_driver[8ed9b98a7c93dd6b]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f317f0d45ed - std::panicking::rust_panic_with_hook::hb63e1080147ab8fb
   7:     0x7f317f0d4319 - std::panicking::begin_panic_handler::{{closure}}::h28de1e26006d66f9
   8:     0x7f317f0d0b14 - std::sys_common::backtrace::__rust_end_short_backtrace::h26ee2e41c2c2875a
   9:     0x7f317f0d4039 - rust_begin_unwind
  10:     0x7f317f080b7b - core::panicking::panic_no_unwind::hdd8236a91374a97f
  11:     0x7f317fb9038e - <alloc[d149d8c8d6a713e7]::rc::Rc<rustc_session[d666950372548c5f]::session::Session> as core[a65aa9227bae4abc]::ops::drop::Drop>::drop
  12:     0x7f317fbccf49 - core[a65aa9227bae4abc]::ptr::drop_in_place::<rustc_interface[f56f2f0b26d6c35e]::interface::Compiler>
  13:     0x7f317fbc9e57 - rustc_span[6d3d3d5a9e908e9c]::with_source_map::<core[a65aa9227bae4abc]::result::Result<(), rustc_errors[9bd6aca994b05f94]::ErrorGuaranteed>, rustc_interface[f56f2f0b26d6c35e]::interface::create_compiler_and_run<core[a65aa9227bae4abc]::result::Result<(), rustc_errors[9bd6aca994b05f94]::ErrorGuaranteed>, rustc_driver[8ed9b98a7c93dd6b]::run_compiler::{closure#1}>::{closure#1}>
  14:     0x7f317fbbc0b0 - <scoped_tls[433840387f49a653]::ScopedKey<rustc_span[6d3d3d5a9e908e9c]::SessionGlobals>>::set::<rustc_interface[f56f2f0b26d6c35e]::interface::run_compiler<core[a65aa9227bae4abc]::result::Result<(), rustc_errors[9bd6aca994b05f94]::ErrorGuaranteed>, rustc_driver[8ed9b98a7c93dd6b]::run_compiler::{closure#1}>::{closure#0}, core[a65aa9227bae4abc]::result::Result<(), rustc_errors[9bd6aca994b05f94]::ErrorGuaranteed>>
  15:     0x7f317fb8c249 - std[45702258e1bbc8d]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[f56f2f0b26d6c35e]::util::run_in_thread_pool_with_globals<rustc_interface[f56f2f0b26d6c35e]::interface::run_compiler<core[a65aa9227bae4abc]::result::Result<(), rustc_errors[9bd6aca994b05f94]::ErrorGuaranteed>, rustc_driver[8ed9b98a7c93dd6b]::run_compiler::{closure#1}>::{closure#0}, core[a65aa9227bae4abc]::result::Result<(), rustc_errors[9bd6aca994b05f94]::ErrorGuaranteed>>::{closure#0}, core[a65aa9227bae4abc]::result::Result<(), rustc_errors[9bd6aca994b05f94]::ErrorGuaranteed>>
  16:     0x7f317fbd6a31 - std[45702258e1bbc8d]::panicking::try::<core[a65aa9227bae4abc]::result::Result<(), rustc_errors[9bd6aca994b05f94]::ErrorGuaranteed>, core[a65aa9227bae4abc]::panic::unwind_safe::AssertUnwindSafe<<std[45702258e1bbc8d]::thread::Builder>::spawn_unchecked_<rustc_interface[f56f2f0b26d6c35e]::util::run_in_thread_pool_with_globals<rustc_interface[f56f2f0b26d6c35e]::interface::run_compiler<core[a65aa9227bae4abc]::result::Result<(), rustc_errors[9bd6aca994b05f94]::ErrorGuaranteed>, rustc_driver[8ed9b98a7c93dd6b]::run_compiler::{closure#1}>::{closure#0}, core[a65aa9227bae4abc]::result::Result<(), rustc_errors[9bd6aca994b05f94]::ErrorGuaranteed>>::{closure#0}, core[a65aa9227bae4abc]::result::Result<(), rustc_errors[9bd6aca994b05f94]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  17:     0x7f317fbcf271 - <<std[45702258e1bbc8d]::thread::Builder>::spawn_unchecked_<rustc_interface[f56f2f0b26d6c35e]::util::run_in_thread_pool_with_globals<rustc_interface[f56f2f0b26d6c35e]::interface::run_compiler<core[a65aa9227bae4abc]::result::Result<(), rustc_errors[9bd6aca994b05f94]::ErrorGuaranteed>, rustc_driver[8ed9b98a7c93dd6b]::run_compiler::{closure#1}>::{closure#0}, core[a65aa9227bae4abc]::result::Result<(), rustc_errors[9bd6aca994b05f94]::ErrorGuaranteed>>::{closure#0}, core[a65aa9227bae4abc]::result::Result<(), rustc_errors[9bd6aca994b05f94]::ErrorGuaranteed>>::{closure#1} as core[a65aa9227bae4abc]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  18:     0x7f317f0dfa43 - std::sys::unix::thread::Thread::new::thread_start::hb758e3f4652c5e3b
  19:     0x7f317962b609 - start_thread
  20:     0x7f317ef3e163 - clone
  21:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.61.0-nightly (8ab5939ba 2022-03-22) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C incremental -Z incremental -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
thread panicked while panicking. aborting.
------------------------------------------
