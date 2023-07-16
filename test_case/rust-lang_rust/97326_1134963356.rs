plain
   Compiling remove_dir_all v0.5.3
   Compiling arrayvec v0.7.0
   Compiling tinystr v0.3.4
   Compiling rustc_graphviz v0.0.0 (/checkout/compiler/rustc_graphviz)
error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: no region-bound-pairs for HirId { owner: DefId(0:106 ~ smallvec[643d]::Drain), local_id: 0 }
    |
    |
383 | impl<'a, T: 'a + Array> Drop for Drain<'a, T> {
    |
    |
    = note: delayed at compiler/rustc_infer/src/infer/outlives/obligations.rs:182:31
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1368:13
stack backtrace:
   0:     0x7f6a20dc9cc2 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hed5e82c7553650f8
   1:     0x7f6a20e314c8 - core::fmt::write::h1fc55f25f1da25e2
   1:     0x7f6a20e314c8 - core::fmt::write::h1fc55f25f1da25e2
   2:     0x7f6a20dba031 - std::io::Write::write_fmt::h31d176ee87802dac
   3:     0x7f6a20dccff9 - std::panicking::default_hook::{{closure}}::h4b6a498b15d19255
   4:     0x7f6a20dccc9a - std::panicking::default_hook::hdfde1f7a50c48f5a
   5:     0x7f6a2192fa41 - rustc_driver[4e648c9c312606c5]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f6a20dcd85f - std::panicking::rust_panic_with_hook::hd859ce74a35190e4
   7:     0x7f6a24431263 - std[f015e8aa67e3adc2]::panicking::begin_panic::<rustc_errors[a2a383fcc09f534c]::ExplicitBug>::{closure#0}
   8:     0x7f6a2442e356 - std[f015e8aa67e3adc2]::sys_common::backtrace::__rust_end_short_backtrace::<std[f015e8aa67e3adc2]::panicking::begin_panic<rustc_errors[a2a383fcc09f534c]::ExplicitBug>::{closure#0}, !>
   9:     0x7f6a21874e06 - std[f015e8aa67e3adc2]::panicking::begin_panic::<rustc_errors[a2a383fcc09f534c]::ExplicitBug>
  10:     0x7f6a24477156 - std[f015e8aa67e3adc2]::panic::panic_any::<rustc_errors[a2a383fcc09f534c]::ExplicitBug>
  11:     0x7f6a2447b937 - <rustc_errors[a2a383fcc09f534c]::HandlerInner as core[13e0a73518ff7a6a]::ops::drop::Drop>::drop
  12:     0x7f6a218b84d2 - core[13e0a73518ff7a6a]::ptr::drop_in_place::<rustc_session[49c68779a48a494]::parse::ParseSess>
  13:     0x7f6a218bb7b5 - <alloc[8b6d570f41c2f666]::rc::Rc<rustc_session[49c68779a48a494]::session::Session> as core[13e0a73518ff7a6a]::ops::drop::Drop>::drop
  14:     0x7f6a218abbfc - core[13e0a73518ff7a6a]::ptr::drop_in_place::<rustc_interface[3d76fa0422dd3719]::interface::Compiler>
  15:     0x7f6a218a8cb4 - rustc_span[4dd9eb67e91c4c01]::with_source_map::<core[13e0a73518ff7a6a]::result::Result<(), rustc_errors[a2a383fcc09f534c]::ErrorGuaranteed>, rustc_interface[3d76fa0422dd3719]::interface::create_compiler_and_run<core[13e0a73518ff7a6a]::result::Result<(), rustc_errors[a2a383fcc09f534c]::ErrorGuaranteed>, rustc_driver[4e648c9c312606c5]::run_compiler::{closure#1}>::{closure#1}>
  16:     0x7f6a218c73cf - <scoped_tls[cfe6876fdb8dd56a]::ScopedKey<rustc_span[4dd9eb67e91c4c01]::SessionGlobals>>::set::<rustc_interface[3d76fa0422dd3719]::interface::run_compiler<core[13e0a73518ff7a6a]::result::Result<(), rustc_errors[a2a383fcc09f534c]::ErrorGuaranteed>, rustc_driver[4e648c9c312606c5]::run_compiler::{closure#1}>::{closure#0}, core[13e0a73518ff7a6a]::result::Result<(), rustc_errors[a2a383fcc09f534c]::ErrorGuaranteed>>
  17:     0x7f6a2191c5f9 - std[f015e8aa67e3adc2]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[3d76fa0422dd3719]::util::run_in_thread_pool_with_globals<rustc_interface[3d76fa0422dd3719]::interface::run_compiler<core[13e0a73518ff7a6a]::result::Result<(), rustc_errors[a2a383fcc09f534c]::ErrorGuaranteed>, rustc_driver[4e648c9c312606c5]::run_compiler::{closure#1}>::{closure#0}, core[13e0a73518ff7a6a]::result::Result<(), rustc_errors[a2a383fcc09f534c]::ErrorGuaranteed>>::{closure#0}, core[13e0a73518ff7a6a]::result::Result<(), rustc_errors[a2a383fcc09f534c]::ErrorGuaranteed>>
  18:     0x7f6a218dadb1 - std[f015e8aa67e3adc2]::panicking::try::<core[13e0a73518ff7a6a]::result::Result<(), rustc_errors[a2a383fcc09f534c]::ErrorGuaranteed>, core[13e0a73518ff7a6a]::panic::unwind_safe::AssertUnwindSafe<<std[f015e8aa67e3adc2]::thread::Builder>::spawn_unchecked_<rustc_interface[3d76fa0422dd3719]::util::run_in_thread_pool_with_globals<rustc_interface[3d76fa0422dd3719]::interface::run_compiler<core[13e0a73518ff7a6a]::result::Result<(), rustc_errors[a2a383fcc09f534c]::ErrorGuaranteed>, rustc_driver[4e648c9c312606c5]::run_compiler::{closure#1}>::{closure#0}, core[13e0a73518ff7a6a]::result::Result<(), rustc_errors[a2a383fcc09f534c]::ErrorGuaranteed>>::{closure#0}, core[13e0a73518ff7a6a]::result::Result<(), rustc_errors[a2a383fcc09f534c]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  19:     0x7f6a2191f952 - <<std[f015e8aa67e3adc2]::thread::Builder>::spawn_unchecked_<rustc_interface[3d76fa0422dd3719]::util::run_in_thread_pool_with_globals<rustc_interface[3d76fa0422dd3719]::interface::run_compiler<core[13e0a73518ff7a6a]::result::Result<(), rustc_errors[a2a383fcc09f534c]::ErrorGuaranteed>, rustc_driver[4e648c9c312606c5]::run_compiler::{closure#1}>::{closure#0}, core[13e0a73518ff7a6a]::result::Result<(), rustc_errors[a2a383fcc09f534c]::ErrorGuaranteed>>::{closure#0}, core[13e0a73518ff7a6a]::result::Result<(), rustc_errors[a2a383fcc09f534c]::ErrorGuaranteed>>::{closure#1} as core[13e0a73518ff7a6a]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  20:     0x7f6a20dda203 - std::sys::unix::thread::Thread::new::thread_start::h79b5570210b990ad
  21:     0x7f6a1b32a609 - start_thread
  22:     0x7f6a20c3d133 - clone
  23:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.63.0-nightly (cb2b0873e 2022-05-23) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=on -C symbol-mangling-version=v0 -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -Z unstable-options -C split-debuginfo=off -Z tls-model=initial-exec -Z unstable-options -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
end of query stack
error: could not compile `smallvec`
warning: build failed, waiting for other jobs to finish...
error: internal compiler error: no region-bound-pairs for HirId { owner: DefId(0:242 ~ arrayvec[5fef]::arrayvec::Drain), local_id: 0 }
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/arrayvec-0.7.0/src/arrayvec.rs:931:13
    |
931 | impl<'a, T: 'a, const CAP: usize> Drop for Drain<'a, T, CAP> {
    |
    |
    = note: delayed at compiler/rustc_infer/src/infer/outlives/obligations.rs:182:31
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1368:13
stack backtrace:
   0:     0x7fd297f2ccc2 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hed5e82c7553650f8
   1:     0x7fd297f944c8 - core::fmt::write::h1fc55f25f1da25e2
   1:     0x7fd297f944c8 - core::fmt::write::h1fc55f25f1da25e2
   2:     0x7fd297f1d031 - std::io::Write::write_fmt::h31d176ee87802dac
   3:     0x7fd297f2fff9 - std::panicking::default_hook::{{closure}}::h4b6a498b15d19255
   4:     0x7fd297f2fc9a - std::panicking::default_hook::hdfde1f7a50c48f5a
   5:     0x7fd298a92a41 - rustc_driver[4e648c9c312606c5]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fd297f3085f - std::panicking::rust_panic_with_hook::hd859ce74a35190e4
   7:     0x7fd29b594263 - std[f015e8aa67e3adc2]::panicking::begin_panic::<rustc_errors[a2a383fcc09f534c]::ExplicitBug>::{closure#0}
   8:     0x7fd29b591356 - std[f015e8aa67e3adc2]::sys_common::backtrace::__rust_end_short_backtrace::<std[f015e8aa67e3adc2]::panicking::begin_panic<rustc_errors[a2a383fcc09f534c]::ExplicitBug>::{closure#0}, !>
   9:     0x7fd2989d7e06 - std[f015e8aa67e3adc2]::panicking::begin_panic::<rustc_errors[a2a383fcc09f534c]::ExplicitBug>
  10:     0x7fd29b5da156 - std[f015e8aa67e3adc2]::panic::panic_any::<rustc_errors[a2a383fcc09f534c]::ExplicitBug>
  11:     0x7fd29b5de937 - <rustc_errors[a2a383fcc09f534c]::HandlerInner as core[13e0a73518ff7a6a]::ops::drop::Drop>::drop
  12:     0x7fd298a1b4d2 - core[13e0a73518ff7a6a]::ptr::drop_in_place::<rustc_session[49c68779a48a494]::parse::ParseSess>
  13:     0x7fd298a1e7b5 - <alloc[8b6d570f41c2f666]::rc::Rc<rustc_session[49c68779a48a494]::session::Session> as core[13e0a73518ff7a6a]::ops::drop::Drop>::drop
  14:     0x7fd298a0ebfc - core[13e0a73518ff7a6a]::ptr::drop_in_place::<rustc_interface[3d76fa0422dd3719]::interface::Compiler>
  15:     0x7fd298a0bcb4 - rustc_span[4dd9eb67e91c4c01]::with_source_map::<core[13e0a73518ff7a6a]::result::Result<(), rustc_errors[a2a383fcc09f534c]::ErrorGuaranteed>, rustc_interface[3d76fa0422dd3719]::interface::create_compiler_and_run<core[13e0a73518ff7a6a]::result::Result<(), rustc_errors[a2a383fcc09f534c]::ErrorGuaranteed>, rustc_driver[4e648c9c312606c5]::run_compiler::{closure#1}>::{closure#1}>
  16:     0x7fd298a2a3cf - <scoped_tls[cfe6876fdb8dd56a]::ScopedKey<rustc_span[4dd9eb67e91c4c01]::SessionGlobals>>::set::<rustc_interface[3d76fa0422dd3719]::interface::run_compiler<core[13e0a73518ff7a6a]::result::Result<(), rustc_errors[a2a383fcc09f534c]::ErrorGuaranteed>, rustc_driver[4e648c9c312606c5]::run_compiler::{closure#1}>::{closure#0}, core[13e0a73518ff7a6a]::result::Result<(), rustc_errors[a2a383fcc09f534c]::ErrorGuaranteed>>
  17:     0x7fd298a7f5f9 - std[f015e8aa67e3adc2]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[3d76fa0422dd3719]::util::run_in_thread_pool_with_globals<rustc_interface[3d76fa0422dd3719]::interface::run_compiler<core[13e0a73518ff7a6a]::result::Result<(), rustc_errors[a2a383fcc09f534c]::ErrorGuaranteed>, rustc_driver[4e648c9c312606c5]::run_compiler::{closure#1}>::{closure#0}, core[13e0a73518ff7a6a]::result::Result<(), rustc_errors[a2a383fcc09f534c]::ErrorGuaranteed>>::{closure#0}, core[13e0a73518ff7a6a]::result::Result<(), rustc_errors[a2a383fcc09f534c]::ErrorGuaranteed>>
  18:     0x7fd298a3ddb1 - std[f015e8aa67e3adc2]::panicking::try::<core[13e0a73518ff7a6a]::result::Result<(), rustc_errors[a2a383fcc09f534c]::ErrorGuaranteed>, core[13e0a73518ff7a6a]::panic::unwind_safe::AssertUnwindSafe<<std[f015e8aa67e3adc2]::thread::Builder>::spawn_unchecked_<rustc_interface[3d76fa0422dd3719]::util::run_in_thread_pool_with_globals<rustc_interface[3d76fa0422dd3719]::interface::run_compiler<core[13e0a73518ff7a6a]::result::Result<(), rustc_errors[a2a383fcc09f534c]::ErrorGuaranteed>, rustc_driver[4e648c9c312606c5]::run_compiler::{closure#1}>::{closure#0}, core[13e0a73518ff7a6a]::result::Result<(), rustc_errors[a2a383fcc09f534c]::ErrorGuaranteed>>::{closure#0}, core[13e0a73518ff7a6a]::result::Result<(), rustc_errors[a2a383fcc09f534c]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  19:     0x7fd298a82952 - <<std[f015e8aa67e3adc2]::thread::Builder>::spawn_unchecked_<rustc_interface[3d76fa0422dd3719]::util::run_in_thread_pool_with_globals<rustc_interface[3d76fa0422dd3719]::interface::run_compiler<core[13e0a73518ff7a6a]::result::Result<(), rustc_errors[a2a383fcc09f534c]::ErrorGuaranteed>, rustc_driver[4e648c9c312606c5]::run_compiler::{closure#1}>::{closure#0}, core[13e0a73518ff7a6a]::result::Result<(), rustc_errors[a2a383fcc09f534c]::ErrorGuaranteed>>::{closure#0}, core[13e0a73518ff7a6a]::result::Result<(), rustc_errors[a2a383fcc09f534c]::ErrorGuaranteed>>::{closure#1} as core[13e0a73518ff7a6a]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  20:     0x7fd297f3d203 - std::sys::unix::thread::Thread::new::thread_start::h79b5570210b990ad
  21:     0x7fd29248d609 - start_thread
  22:     0x7fd297da0133 - clone
  23:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.63.0-nightly (cb2b0873e 2022-05-23) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=on -C symbol-mangling-version=v0 -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -Z unstable-options -C split-debuginfo=off -Z tls-model=initial-exec -Z unstable-options -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
