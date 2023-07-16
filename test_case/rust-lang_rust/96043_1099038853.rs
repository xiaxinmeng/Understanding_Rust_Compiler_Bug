plain
   Compiling rustc_serialize v0.0.0 (/checkout/compiler/rustc_serialize)
   Compiling petgraph v0.5.1
   Compiling gimli v0.26.1
   Compiling object v0.28.1
error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: No `move_errors` should be allowed in MIR borrowck
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/aho-corasick-0.7.18/src/nfa.rs:419:5
    |
419 | /     fn set_next_state(&mut self, input: u8, next: S) {
420 | |         match *self {
421 | |             Transitions::Sparse(ref mut sparse) => {
422 | |                 match sparse.binary_search_by_key(&input, |&(b, _)| b) {
430 | |         }
431 | |     }
    | |_____^
    |
    |
    = note: delayed at compiler/rustc_mir_transform/src/elaborate_drops.rs:34:26

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1383:13
stack backtrace:
   0:     0x7faea77f2452 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h7a88a4dc96b6b8f8
   1:     0x7faea7857a18 - core::fmt::write::h42234c3e51154f4c
   2:     0x7faea77e2871 - std::io::Write::write_fmt::h5505b6313bdcb0f3
   3:     0x7faea77f5786 - std::panicking::default_hook::{{closure}}::hbe1bb29927e8c85b
   4:     0x7faea77f5385 - std::panicking::default_hook::h30c665989e20cb24
   5:     0x7faea830f631 - rustc_driver[2b15ae7948b6e616]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7faea77f60ad - std::panicking::rust_panic_with_hook::hab898bc6064aa4ee
   7:     0x7faeaad5da43 - std[e4dc215d72d9f73d]::panicking::begin_panic::<rustc_errors[3b46f01f9753e7d]::ExplicitBug>::{closure#0}
   8:     0x7faeaad5c736 - std[e4dc215d72d9f73d]::sys_common::backtrace::__rust_end_short_backtrace::<std[e4dc215d72d9f73d]::panicking::begin_panic<rustc_errors[3b46f01f9753e7d]::ExplicitBug>::{closure#0}, !>
   9:     0x7faea825c56f - std[e4dc215d72d9f73d]::panicking::begin_panic::<rustc_errors[3b46f01f9753e7d]::ExplicitBug>
  10:     0x7faeaacfc516 - std[e4dc215d72d9f73d]::panic::panic_any::<rustc_errors[3b46f01f9753e7d]::ExplicitBug>
  11:     0x7faeaad00e7a - <rustc_errors[3b46f01f9753e7d]::HandlerInner as core[10878fb91fc84a80]::ops::drop::Drop>::drop
  12:     0x7faea8292e52 - core[10878fb91fc84a80]::ptr::drop_in_place::<rustc_session[490bd8b11d3080dc]::parse::ParseSess>
  13:     0x7faea8299445 - <alloc[4b492a408420e30b]::rc::Rc<rustc_session[490bd8b11d3080dc]::session::Session> as core[10878fb91fc84a80]::ops::drop::Drop>::drop
  14:     0x7faea828962c - core[10878fb91fc84a80]::ptr::drop_in_place::<rustc_interface[1e8edbf255833c1]::interface::Compiler>
  15:     0x7faea8287a44 - rustc_span[e033c2886c1ea87]::with_source_map::<core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>, rustc_interface[1e8edbf255833c1]::interface::create_compiler_and_run<core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>, rustc_driver[2b15ae7948b6e616]::run_compiler::{closure#1}>::{closure#1}>
  16:     0x7faea82b6ee7 - rustc_interface[1e8edbf255833c1]::interface::create_compiler_and_run::<core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>, rustc_driver[2b15ae7948b6e616]::run_compiler::{closure#1}>
  17:     0x7faea82bb63f - <scoped_tls[5ed4b67c3b198af5]::ScopedKey<rustc_span[e033c2886c1ea87]::SessionGlobals>>::set::<rustc_interface[1e8edbf255833c1]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>, rustc_driver[2b15ae7948b6e616]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>>
  18:     0x7faea8302fb9 - std[e4dc215d72d9f73d]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[1e8edbf255833c1]::util::run_in_thread_pool_with_globals<rustc_interface[1e8edbf255833c1]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>, rustc_driver[2b15ae7948b6e616]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>>
  19:     0x7faea82bd771 - std[e4dc215d72d9f73d]::panicking::try::<core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>, core[10878fb91fc84a80]::panic::unwind_safe::AssertUnwindSafe<<std[e4dc215d72d9f73d]::thread::Builder>::spawn_unchecked_<rustc_interface[1e8edbf255833c1]::util::run_in_thread_pool_with_globals<rustc_interface[1e8edbf255833c1]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>, rustc_driver[2b15ae7948b6e616]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  20:     0x7faea82fe012 - <<std[e4dc215d72d9f73d]::thread::Builder>::spawn_unchecked_<rustc_interface[1e8edbf255833c1]::util::run_in_thread_pool_with_globals<rustc_interface[1e8edbf255833c1]::interface::run_compiler<core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>, rustc_driver[2b15ae7948b6e616]::run_compiler::{closure#1}>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>>::{closure#0}, core[10878fb91fc84a80]::result::Result<(), rustc_errors[3b46f01f9753e7d]::ErrorGuaranteed>>::{closure#1} as core[10878fb91fc84a80]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  21:     0x7faea7802863 - std::sys::unix::thread::Thread::new::thread_start::h2f9ecc8966c8b525
  22:     0x7faea1d52609 - start_thread
  23:     0x7faea7665163 - clone
  24:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.62.0-nightly (de23b6d5a 2022-04-14) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=on -C symbol-mangling-version=v0 -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -Z tls-model=initial-exec -Z unstable-options -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
