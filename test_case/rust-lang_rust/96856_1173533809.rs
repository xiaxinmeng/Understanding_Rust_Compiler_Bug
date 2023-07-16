plain
---- [ui] src/test/ui/mir/ssa-analysis-regression-50041.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/mir/ssa-analysis-regression-50041.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/ssa-analysis-regression-50041" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "mir-opt-level=4" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/ssa-analysis-regression-50041/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: broken MIR in Item(WithOptConstParam { did: DefId(0:27 ~ ssa_analysis_regression_50041[8e9e]::foo), const_param_did: None }) (end of phase transition to Optimized) at bb1[3]:
                                Field projection `((_4.0: Unique<Foo<usize>>).0: NonNull<Foo<usize>>).field[0]` specified type `*const Foo<usize>`, but actual type is *mut Foo<usize>
   |
   |
LL |         Some(vec) => *vec,
   |
   = note: delayed at compiler/rustc_const_eval/src/transform/validate.rs:129:36

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1425:13
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1425:13
stack backtrace:
   0:     0x7f0d2546db3c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h777b0afa2d9bf289
   1:     0x7f0d254d2698 - core::fmt::write::hfcb848612573dce9
   2:     0x7f0d2545e1a1 - std::io::Write::write_fmt::he53eaac35a506cc0
   3:     0x7f0d254709be - std::panicking::default_hook::{{closure}}::h829078e8014b4994
   4:     0x7f0d254706a6 - std::panicking::default_hook::h1c4eff9ee733ba9f
   5:     0x7f0d25e21414 - rustc_driver[ce4c05d70c7f4fff]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f0d25471152 - std::panicking::rust_panic_with_hook::h9e6bf24860102c3e
   7:     0x7f0d28ae9283 - std[28a5b2b2f3247943]::panicking::begin_panic::<rustc_errors[d8b666a88bf9e265]::ExplicitBug>::{closure#0}
   8:     0x7f0d28ae8906 - std[28a5b2b2f3247943]::sys_common::backtrace::__rust_end_short_backtrace::<std[28a5b2b2f3247943]::panicking::begin_panic<rustc_errors[d8b666a88bf9e265]::ExplicitBug>::{closure#0}, !>
   9:     0x7f0d25ddd226 - std[28a5b2b2f3247943]::panicking::begin_panic::<rustc_errors[d8b666a88bf9e265]::ExplicitBug>
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
  10:     0x7f0d28add866 - std[28a5b2b2f3247943]::panic::panic_any::<rustc_errors[d8b666a88bf9e265]::ExplicitBug>
  11:     0x7f0d28ae2a4d - <rustc_errors[d8b666a88bf9e265]::HandlerInner as core[db94ac846a91817a]::ops::drop::Drop>::drop
  12:     0x7f0d25e3c382 - core[db94ac846a91817a]::ptr::drop_in_place::<rustc_session[427c23658a95502a]::parse::ParseSess>
  13:     0x7f0d25e4685d - <alloc[b0ab51bfda518ea6]::rc::Rc<rustc_session[427c23658a95502a]::session::Session> as core[db94ac846a91817a]::ops::drop::Drop>::drop
  14:     0x7f0d25e1412c - core[db94ac846a91817a]::ptr::drop_in_place::<rustc_interface[aec5aa3afc3cd894]::interface::Compiler>
  15:     0x7f0d25e107f9 - rustc_span[1ad2267d673b65c8]::with_source_map::<core[db94ac846a91817a]::result::Result<(), rustc_errors[d8b666a88bf9e265]::ErrorGuaranteed>, rustc_interface[aec5aa3afc3cd894]::interface::create_compiler_and_run<core[db94ac846a91817a]::result::Result<(), rustc_errors[d8b666a88bf9e265]::ErrorGuaranteed>, rustc_driver[ce4c05d70c7f4fff]::run_compiler::{closure#1}>::{closure#1}>
  16:     0x7f0d25e2a839 - rustc_interface[aec5aa3afc3cd894]::interface::create_compiler_and_run::<core[db94ac846a91817a]::result::Result<(), rustc_errors[d8b666a88bf9e265]::ErrorGuaranteed>, rustc_driver[ce4c05d70c7f4fff]::run_compiler::{closure#1}>
  17:     0x7f0d25e0a9af - <scoped_tls[ce793c490feb58c7]::ScopedKey<rustc_span[1ad2267d673b65c8]::SessionGlobals>>::set::<rustc_interface[aec5aa3afc3cd894]::interface::run_compiler<core[db94ac846a91817a]::result::Result<(), rustc_errors[d8b666a88bf9e265]::ErrorGuaranteed>, rustc_driver[ce4c05d70c7f4fff]::run_compiler::{closure#1}>::{closure#0}, core[db94ac846a91817a]::result::Result<(), rustc_errors[d8b666a88bf9e265]::ErrorGuaranteed>>
  18:     0x7f0d25e146e9 - std[28a5b2b2f3247943]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[aec5aa3afc3cd894]::util::run_in_thread_pool_with_globals<rustc_interface[aec5aa3afc3cd894]::interface::run_compiler<core[db94ac846a91817a]::result::Result<(), rustc_errors[d8b666a88bf9e265]::ErrorGuaranteed>, rustc_driver[ce4c05d70c7f4fff]::run_compiler::{closure#1}>::{closure#0}, core[db94ac846a91817a]::result::Result<(), rustc_errors[d8b666a88bf9e265]::ErrorGuaranteed>>::{closure#0}, core[db94ac846a91817a]::result::Result<(), rustc_errors[d8b666a88bf9e265]::ErrorGuaranteed>>
  19:     0x7f0d25e7d1d9 - <<std[28a5b2b2f3247943]::thread::Builder>::spawn_unchecked_<rustc_interface[aec5aa3afc3cd894]::util::run_in_thread_pool_with_globals<rustc_interface[aec5aa3afc3cd894]::interface::run_compiler<core[db94ac846a91817a]::result::Result<(), rustc_errors[d8b666a88bf9e265]::ErrorGuaranteed>, rustc_driver[ce4c05d70c7f4fff]::run_compiler::{closure#1}>::{closure#0}, core[db94ac846a91817a]::result::Result<(), rustc_errors[d8b666a88bf9e265]::ErrorGuaranteed>>::{closure#0}, core[db94ac846a91817a]::result::Result<(), rustc_errors[d8b666a88bf9e265]::ErrorGuaranteed>>::{closure#1} as core[db94ac846a91817a]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  20:     0x7f0d2547c033 - std::sys::unix::thread::Thread::new::thread_start::hd12a00c984da03bc
  21:     0x7f0d1f9ca609 - start_thread
  22:     0x7f0d252dd133 - clone
  23:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.64.0-nightly (7804092bb 2022-07-04) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 -Z mir-opt-level=4
query stack during panic:
end of query stack
------------------------------------------

