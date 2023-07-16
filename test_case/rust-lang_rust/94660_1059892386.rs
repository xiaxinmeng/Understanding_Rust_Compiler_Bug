plain
...................................i................................................................ 1300/12700
.....................................................................i.............................. 1400/12700
.................................................................................................... 1500/12700
.................................................................................................... 1600/12700
.........................................F.F.F.....iF............................................... 1700/12700
....................................................................i............................... 1900/12700
.................................................................................................... 2000/12700
.................................................................................................... 2100/12700
.................................................................................................... 2200/12700
---
---- [ui] ui/cmse-nonsecure/cmse-nonsecure-call/params-on-registers.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/cmse-nonsecure/cmse-nonsecure-call/params-on-registers.rs" "-Zthreads=1" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cmse-nonsecure/cmse-nonsecure-call/params-on-registers" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target" "thumbv8m.main-none-eabi" "--crate-type" "lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cmse-nonsecure/cmse-nonsecure-call/params-on-registers/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: broken MIR in DefId(0:7 ~ params_on_registers[6840]::test) (NoSolution): could not prove Binder(TraitPredicate(<u32 as Copy>, polarity:Positive), [])
   = note: delayed at compiler/rustc_borrowck/src/type_check/canonical.rs:150:13


error: internal compiler error: broken MIR in DefId(0:7 ~ params_on_registers[6840]::test) (NoSolution): could not prove Binder(TraitPredicate(<u32 as Copy>, polarity:Positive), [])
   = note: delayed at compiler/rustc_borrowck/src/type_check/canonical.rs:150:13


error: internal compiler error: broken MIR in DefId(0:7 ~ params_on_registers[6840]::test) (NoSolution): could not prove Binder(TraitPredicate(<u32 as Copy>, polarity:Positive), [])
   = note: delayed at compiler/rustc_borrowck/src/type_check/canonical.rs:150:13


error: internal compiler error: broken MIR in DefId(0:7 ~ params_on_registers[6840]::test) (NoSolution): could not prove Binder(TraitPredicate(<u32 as Copy>, polarity:Positive), [])
   = note: delayed at compiler/rustc_borrowck/src/type_check/canonical.rs:150:13
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1266:13
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1266:13
stack backtrace:
   0:     0x7f83158c05dc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h44e6c69af6c2f06f
   1:     0x7f8315927a0f - core::fmt::write::h5f4b6e359e7e435a
   2:     0x7f83158afaa1 - std::io::Write::write_fmt::h4607a7c48291320a
   3:     0x7f83158c03fb - std::sys_common::backtrace::print::hf27fd1825ee66778
   4:     0x7f83158c3b34 - std::panicking::default_hook::{{closure}}::he1a59f991ce1d27b
   5:     0x7f83158c36ed - std::panicking::default_hook::hfced6c661d290326
   6:     0x7f831633a811 - rustc_driver[b560162ccb61062d]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f83158c421a - std::panicking::rust_panic_with_hook::h9abdd2ae546e474c
   8:     0x7f8318b969c3 - std[94a519590adbd16d]::panicking::begin_panic::<rustc_errors[1da165d4f1c6a1c0]::ExplicitBug>::{closure#0}
   9:     0x7f8318b96776 - std[94a519590adbd16d]::sys_common::backtrace::__rust_end_short_backtrace::<std[94a519590adbd16d]::panicking::begin_panic<rustc_errors[1da165d4f1c6a1c0]::ExplicitBug>::{closure#0}, !>
  10:     0x7f83162f995f - std[94a519590adbd16d]::panicking::begin_panic::<rustc_errors[1da165d4f1c6a1c0]::ExplicitBug>
  11:     0x7f8318bc8736 - std[94a519590adbd16d]::panic::panic_any::<rustc_errors[1da165d4f1c6a1c0]::ExplicitBug>
  12:     0x7f8318bcd706 - <rustc_errors[1da165d4f1c6a1c0]::HandlerInner as core[d8f8c7b14b601550]::ops::drop::Drop>::drop
  13:     0x7f83163524b2 - core[d8f8c7b14b601550]::ptr::drop_in_place::<rustc_session[e22b767dbbfd943c]::parse::ParseSess>
  14:     0x7f8316357aaa - <alloc[1b7dfdc50e21c679]::rc::Rc<rustc_session[e22b767dbbfd943c]::session::Session> as core[d8f8c7b14b601550]::ops::drop::Drop>::drop
  15:     0x7f83163a976c - core[d8f8c7b14b601550]::ptr::drop_in_place::<rustc_interface[a2eba84e67d3cf00]::interface::Compiler>
  16:     0x7f83163995d9 - rustc_interface[a2eba84e67d3cf00]::interface::create_compiler_and_run::<core[d8f8c7b14b601550]::result::Result<(), rustc_errors[1da165d4f1c6a1c0]::ErrorGuaranteed>, rustc_driver[b560162ccb61062d]::run_compiler::{closure#1}>
  17:     0x7f831635fddc - std[94a519590adbd16d]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[a2eba84e67d3cf00]::util::run_in_thread_pool_with_globals<rustc_interface[a2eba84e67d3cf00]::interface::run_compiler<core[d8f8c7b14b601550]::result::Result<(), rustc_errors[1da165d4f1c6a1c0]::ErrorGuaranteed>, rustc_driver[b560162ccb61062d]::run_compiler::{closure#1}>::{closure#0}, core[d8f8c7b14b601550]::result::Result<(), rustc_errors[1da165d4f1c6a1c0]::ErrorGuaranteed>>::{closure#0}, core[d8f8c7b14b601550]::result::Result<(), rustc_errors[1da165d4f1c6a1c0]::ErrorGuaranteed>>
  18:     0x7f83163b63d1 - std[94a519590adbd16d]::panic::catch_unwind::<core[d8f8c7b14b601550]::panic::unwind_safe::AssertUnwindSafe<<std[94a519590adbd16d]::thread::Builder>::spawn_unchecked_<rustc_interface[a2eba84e67d3cf00]::util::run_in_thread_pool_with_globals<rustc_interface[a2eba84e67d3cf00]::interface::run_compiler<core[d8f8c7b14b601550]::result::Result<(), rustc_errors[1da165d4f1c6a1c0]::ErrorGuaranteed>, rustc_driver[b560162ccb61062d]::run_compiler::{closure#1}>::{closure#0}, core[d8f8c7b14b601550]::result::Result<(), rustc_errors[1da165d4f1c6a1c0]::ErrorGuaranteed>>::{closure#0}, core[d8f8c7b14b601550]::result::Result<(), rustc_errors[1da165d4f1c6a1c0]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[d8f8c7b14b601550]::result::Result<(), rustc_errors[1da165d4f1c6a1c0]::ErrorGuaranteed>>
  19:     0x7f8316361b22 - <<std[94a519590adbd16d]::thread::Builder>::spawn_unchecked_<rustc_interface[a2eba84e67d3cf00]::util::run_in_thread_pool_with_globals<rustc_interface[a2eba84e67d3cf00]::interface::run_compiler<core[d8f8c7b14b601550]::result::Result<(), rustc_errors[1da165d4f1c6a1c0]::ErrorGuaranteed>, rustc_driver[b560162ccb61062d]::run_compiler::{closure#1}>::{closure#0}, core[d8f8c7b14b601550]::result::Result<(), rustc_errors[1da165d4f1c6a1c0]::ErrorGuaranteed>>::{closure#0}, core[d8f8c7b14b601550]::result::Result<(), rustc_errors[1da165d4f1c6a1c0]::ErrorGuaranteed>>::{closure#1} as core[d8f8c7b14b601550]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  20:     0x7f83158cfa03 - std::sys::unix::thread::Thread::new::thread_start::h065355bb226194c3
  21:     0x7f830fc3c609 - start_thread
  22:     0x7f8315733293 - clone
  23:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.61.0-nightly (b446fac71 2022-03-06) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0 --crate-type lib
query stack during panic:
end of query stack
------------------------------------------



---- [ui] ui/cmse-nonsecure/cmse-nonsecure-call/params-on-stack.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/cmse-nonsecure/cmse-nonsecure-call/params-on-stack.rs" "-Zthreads=1" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cmse-nonsecure/cmse-nonsecure-call/params-on-stack" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target" "thumbv8m.main-none-eabi" "--crate-type" "lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cmse-nonsecure/cmse-nonsecure-call/params-on-stack/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: broken MIR in DefId(0:7 ~ params_on_stack[8dc7]::test) (NoSolution): could not prove Binder(TraitPredicate(<u32 as Copy>, polarity:Positive), [])
   = note: delayed at compiler/rustc_borrowck/src/type_check/canonical.rs:150:13


error: internal compiler error: broken MIR in DefId(0:7 ~ params_on_stack[8dc7]::test) (NoSolution): could not prove Binder(TraitPredicate(<u32 as Copy>, polarity:Positive), [])
   = note: delayed at compiler/rustc_borrowck/src/type_check/canonical.rs:150:13


error: internal compiler error: broken MIR in DefId(0:7 ~ params_on_stack[8dc7]::test) (NoSolution): could not prove Binder(TraitPredicate(<u32 as Copy>, polarity:Positive), [])
   = note: delayed at compiler/rustc_borrowck/src/type_check/canonical.rs:150:13


error: internal compiler error: broken MIR in DefId(0:7 ~ params_on_stack[8dc7]::test) (NoSolution): could not prove Binder(TraitPredicate(<u32 as Copy>, polarity:Positive), [])
   = note: delayed at compiler/rustc_borrowck/src/type_check/canonical.rs:150:13


error: internal compiler error: broken MIR in DefId(0:7 ~ params_on_stack[8dc7]::test) (NoSolution): could not prove Binder(TraitPredicate(<u32 as Copy>, polarity:Positive), [])
   = note: delayed at compiler/rustc_borrowck/src/type_check/canonical.rs:150:13

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1266:13
stack backtrace:
stack backtrace:
   0:     0x7f23a855a5dc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h44e6c69af6c2f06f
   1:     0x7f23a85c1a0f - core::fmt::write::h5f4b6e359e7e435a
   2:     0x7f23a8549aa1 - std::io::Write::write_fmt::h4607a7c48291320a
   3:     0x7f23a855a3fb - std::sys_common::backtrace::print::hf27fd1825ee66778
   4:     0x7f23a855db34 - std::panicking::default_hook::{{closure}}::he1a59f991ce1d27b
   5:     0x7f23a855d6ed - std::panicking::default_hook::hfced6c661d290326
   6:     0x7f23a8fd4811 - rustc_driver[b560162ccb61062d]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f23a855e21a - std::panicking::rust_panic_with_hook::h9abdd2ae546e474c
   8:     0x7f23ab8309c3 - std[94a519590adbd16d]::panicking::begin_panic::<rustc_errors[1da165d4f1c6a1c0]::ExplicitBug>::{closure#0}
   9:     0x7f23ab830776 - std[94a519590adbd16d]::sys_common::backtrace::__rust_end_short_backtrace::<std[94a519590adbd16d]::panicking::begin_panic<rustc_errors[1da165d4f1c6a1c0]::ExplicitBug>::{closure#0}, !>
  10:     0x7f23a8f9395f - std[94a519590adbd16d]::panicking::begin_panic::<rustc_errors[1da165d4f1c6a1c0]::ExplicitBug>
  11:     0x7f23ab862736 - std[94a519590adbd16d]::panic::panic_any::<rustc_errors[1da165d4f1c6a1c0]::ExplicitBug>
  12:     0x7f23ab867706 - <rustc_errors[1da165d4f1c6a1c0]::HandlerInner as core[d8f8c7b14b601550]::ops::drop::Drop>::drop
  13:     0x7f23a8fec4b2 - core[d8f8c7b14b601550]::ptr::drop_in_place::<rustc_session[e22b767dbbfd943c]::parse::ParseSess>
  14:     0x7f23a8ff1aaa - <alloc[1b7dfdc50e21c679]::rc::Rc<rustc_session[e22b767dbbfd943c]::session::Session> as core[d8f8c7b14b601550]::ops::drop::Drop>::drop
  15:     0x7f23a904376c - core[d8f8c7b14b601550]::ptr::drop_in_place::<rustc_interface[a2eba84e67d3cf00]::interface::Compiler>
  16:     0x7f23a90335d9 - rustc_interface[a2eba84e67d3cf00]::interface::create_compiler_and_run::<core[d8f8c7b14b601550]::result::Result<(), rustc_errors[1da165d4f1c6a1c0]::ErrorGuaranteed>, rustc_driver[b560162ccb61062d]::run_compiler::{closure#1}>
  17:     0x7f23a8ff9ddc - std[94a519590adbd16d]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[a2eba84e67d3cf00]::util::run_in_thread_pool_with_globals<rustc_interface[a2eba84e67d3cf00]::interface::run_compiler<core[d8f8c7b14b601550]::result::Result<(), rustc_errors[1da165d4f1c6a1c0]::ErrorGuaranteed>, rustc_driver[b560162ccb61062d]::run_compiler::{closure#1}>::{closure#0}, core[d8f8c7b14b601550]::result::Result<(), rustc_errors[1da165d4f1c6a1c0]::ErrorGuaranteed>>::{closure#0}, core[d8f8c7b14b601550]::result::Result<(), rustc_errors[1da165d4f1c6a1c0]::ErrorGuaranteed>>
  18:     0x7f23a90503d1 - std[94a519590adbd16d]::panic::catch_unwind::<core[d8f8c7b14b601550]::panic::unwind_safe::AssertUnwindSafe<<std[94a519590adbd16d]::thread::Builder>::spawn_unchecked_<rustc_interface[a2eba84e67d3cf00]::util::run_in_thread_pool_with_globals<rustc_interface[a2eba84e67d3cf00]::interface::run_compiler<core[d8f8c7b14b601550]::result::Result<(), rustc_errors[1da165d4f1c6a1c0]::ErrorGuaranteed>, rustc_driver[b560162ccb61062d]::run_compiler::{closure#1}>::{closure#0}, core[d8f8c7b14b601550]::result::Result<(), rustc_errors[1da165d4f1c6a1c0]::ErrorGuaranteed>>::{closure#0}, core[d8f8c7b14b601550]::result::Result<(), rustc_errors[1da165d4f1c6a1c0]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[d8f8c7b14b601550]::result::Result<(), rustc_errors[1da165d4f1c6a1c0]::ErrorGuaranteed>>
  19:     0x7f23a8ffbb22 - <<std[94a519590adbd16d]::thread::Builder>::spawn_unchecked_<rustc_interface[a2eba84e67d3cf00]::util::run_in_thread_pool_with_globals<rustc_interface[a2eba84e67d3cf00]::interface::run_compiler<core[d8f8c7b14b601550]::result::Result<(), rustc_errors[1da165d4f1c6a1c0]::ErrorGuaranteed>, rustc_driver[b560162ccb61062d]::run_compiler::{closure#1}>::{closure#0}, core[d8f8c7b14b601550]::result::Result<(), rustc_errors[1da165d4f1c6a1c0]::ErrorGuaranteed>>::{closure#0}, core[d8f8c7b14b601550]::result::Result<(), rustc_errors[1da165d4f1c6a1c0]::ErrorGuaranteed>>::{closure#1} as core[d8f8c7b14b601550]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  20:     0x7f23a8569a03 - std::sys::unix::thread::Thread::new::thread_start::h065355bb226194c3
  21:     0x7f23a28d6609 - start_thread
  22:     0x7f23a83cd293 - clone
  23:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.61.0-nightly (b446fac71 2022-03-06) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0 --crate-type lib
query stack during panic:
end of query stack
------------------------------------------



---- [ui] ui/cmse-nonsecure/cmse-nonsecure-entry/params-on-stack.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/cmse-nonsecure/cmse-nonsecure-entry/params-on-stack.rs" "-Zthreads=1" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cmse-nonsecure/cmse-nonsecure-entry/params-on-stack" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target" "thumbv8m.main-none-eabi" "--crate-type" "lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cmse-nonsecure/cmse-nonsecure-entry/params-on-stack/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: broken MIR in DefId(0:3 ~ params_on_stack[8dc7]::entry_function) (NoSolution): could not prove Binder(TraitPredicate(<u32 as Copy>, polarity:Positive), [])
   = note: delayed at compiler/rustc_borrowck/src/type_check/canonical.rs:150:13

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1266:13
stack backtrace:
stack backtrace:
   0:     0x7f878040d5dc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h44e6c69af6c2f06f
   1:     0x7f8780474a0f - core::fmt::write::h5f4b6e359e7e435a
   2:     0x7f87803fcaa1 - std::io::Write::write_fmt::h4607a7c48291320a
   3:     0x7f878040d3fb - std::sys_common::backtrace::print::hf27fd1825ee66778
   4:     0x7f8780410b34 - std::panicking::default_hook::{{closure}}::he1a59f991ce1d27b
   5:     0x7f87804106ed - std::panicking::default_hook::hfced6c661d290326
   6:     0x7f8780e87811 - rustc_driver[b560162ccb61062d]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f878041121a - std::panicking::rust_panic_with_hook::h9abdd2ae546e474c
   8:     0x7f87836e39c3 - std[94a519590adbd16d]::panicking::begin_panic::<rustc_errors[1da165d4f1c6a1c0]::ExplicitBug>::{closure#0}
   9:     0x7f87836e3776 - std[94a519590adbd16d]::sys_common::backtrace::__rust_end_short_backtrace::<std[94a519590adbd16d]::panicking::begin_panic<rustc_errors[1da165d4f1c6a1c0]::ExplicitBug>::{closure#0}, !>
  10:     0x7f8780e4695f - std[94a519590adbd16d]::panicking::begin_panic::<rustc_errors[1da165d4f1c6a1c0]::ExplicitBug>
  11:     0x7f8783715736 - std[94a519590adbd16d]::panic::panic_any::<rustc_errors[1da165d4f1c6a1c0]::ExplicitBug>
  12:     0x7f878371a706 - <rustc_errors[1da165d4f1c6a1c0]::HandlerInner as core[d8f8c7b14b601550]::ops::drop::Drop>::drop
  13:     0x7f8780e9f4b2 - core[d8f8c7b14b601550]::ptr::drop_in_place::<rustc_session[e22b767dbbfd943c]::parse::ParseSess>
  14:     0x7f8780ea4aaa - <alloc[1b7dfdc50e21c679]::rc::Rc<rustc_session[e22b767dbbfd943c]::session::Session> as core[d8f8c7b14b601550]::ops::drop::Drop>::drop
  15:     0x7f8780ef676c - core[d8f8c7b14b601550]::ptr::drop_in_place::<rustc_interface[a2eba84e67d3cf00]::interface::Compiler>
  16:     0x7f8780ee65d9 - rustc_interface[a2eba84e67d3cf00]::interface::create_compiler_and_run::<core[d8f8c7b14b601550]::result::Result<(), rustc_errors[1da165d4f1c6a1c0]::ErrorGuaranteed>, rustc_driver[b560162ccb61062d]::run_compiler::{closure#1}>
  17:     0x7f8780eacddc - std[94a519590adbd16d]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[a2eba84e67d3cf00]::util::run_in_thread_pool_with_globals<rustc_interface[a2eba84e67d3cf00]::interface::run_compiler<core[d8f8c7b14b601550]::result::Result<(), rustc_errors[1da165d4f1c6a1c0]::ErrorGuaranteed>, rustc_driver[b560162ccb61062d]::run_compiler::{closure#1}>::{closure#0}, core[d8f8c7b14b601550]::result::Result<(), rustc_errors[1da165d4f1c6a1c0]::ErrorGuaranteed>>::{closure#0}, core[d8f8c7b14b601550]::result::Result<(), rustc_errors[1da165d4f1c6a1c0]::ErrorGuaranteed>>
  18:     0x7f8780f033d1 - std[94a519590adbd16d]::panic::catch_unwind::<core[d8f8c7b14b601550]::panic::unwind_safe::AssertUnwindSafe<<std[94a519590adbd16d]::thread::Builder>::spawn_unchecked_<rustc_interface[a2eba84e67d3cf00]::util::run_in_thread_pool_with_globals<rustc_interface[a2eba84e67d3cf00]::interface::run_compiler<core[d8f8c7b14b601550]::result::Result<(), rustc_errors[1da165d4f1c6a1c0]::ErrorGuaranteed>, rustc_driver[b560162ccb61062d]::run_compiler::{closure#1}>::{closure#0}, core[d8f8c7b14b601550]::result::Result<(), rustc_errors[1da165d4f1c6a1c0]::ErrorGuaranteed>>::{closure#0}, core[d8f8c7b14b601550]::result::Result<(), rustc_errors[1da165d4f1c6a1c0]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[d8f8c7b14b601550]::result::Result<(), rustc_errors[1da165d4f1c6a1c0]::ErrorGuaranteed>>
  19:     0x7f8780eaeb22 - <<std[94a519590adbd16d]::thread::Builder>::spawn_unchecked_<rustc_interface[a2eba84e67d3cf00]::util::run_in_thread_pool_with_globals<rustc_interface[a2eba84e67d3cf00]::interface::run_compiler<core[d8f8c7b14b601550]::result::Result<(), rustc_errors[1da165d4f1c6a1c0]::ErrorGuaranteed>, rustc_driver[b560162ccb61062d]::run_compiler::{closure#1}>::{closure#0}, core[d8f8c7b14b601550]::result::Result<(), rustc_errors[1da165d4f1c6a1c0]::ErrorGuaranteed>>::{closure#0}, core[d8f8c7b14b601550]::result::Result<(), rustc_errors[1da165d4f1c6a1c0]::ErrorGuaranteed>>::{closure#1} as core[d8f8c7b14b601550]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  20:     0x7f878041ca03 - std::sys::unix::thread::Thread::new::thread_start::h065355bb226194c3
  21:     0x7f877a789609 - start_thread
  22:     0x7f8780280293 - clone
  23:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.61.0-nightly (b446fac71 2022-03-06) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0 --crate-type lib
query stack during panic:
end of query stack
------------------------------------------



---- [ui] ui/cmse-nonsecure/cmse-nonsecure-entry/params-on-registers.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/cmse-nonsecure/cmse-nonsecure-entry/params-on-registers.rs" "-Zthreads=1" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cmse-nonsecure/cmse-nonsecure-entry/params-on-registers" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target" "thumbv8m.main-none-eabi" "--crate-type" "lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cmse-nonsecure/cmse-nonsecure-entry/params-on-registers/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: broken MIR in DefId(0:3 ~ params_on_registers[6840]::entry_function) (NoSolution): could not prove Binder(TraitPredicate(<u32 as Copy>, polarity:Positive), [])
   = note: delayed at compiler/rustc_borrowck/src/type_check/canonical.rs:150:13

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1266:13
stack backtrace:
stack backtrace:
   0:     0x7fd53dc9a5dc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h44e6c69af6c2f06f
   1:     0x7fd53dd01a0f - core::fmt::write::h5f4b6e359e7e435a
   2:     0x7fd53dc89aa1 - std::io::Write::write_fmt::h4607a7c48291320a
   3:     0x7fd53dc9a3fb - std::sys_common::backtrace::print::hf27fd1825ee66778
   4:     0x7fd53dc9db34 - std::panicking::default_hook::{{closure}}::he1a59f991ce1d27b
   5:     0x7fd53dc9d6ed - std::panicking::default_hook::hfced6c661d290326
   6:     0x7fd53e714811 - rustc_driver[b560162ccb61062d]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fd53dc9e21a - std::panicking::rust_panic_with_hook::h9abdd2ae546e474c
   8:     0x7fd540f709c3 - std[94a519590adbd16d]::panicking::begin_panic::<rustc_errors[1da165d4f1c6a1c0]::ExplicitBug>::{closure#0}
   9:     0x7fd540f70776 - std[94a519590adbd16d]::sys_common::backtrace::__rust_end_short_backtrace::<std[94a519590adbd16d]::panicking::begin_panic<rustc_errors[1da165d4f1c6a1c0]::ExplicitBug>::{closure#0}, !>
  10:     0x7fd53e6d395f - std[94a519590adbd16d]::panicking::begin_panic::<rustc_errors[1da165d4f1c6a1c0]::ExplicitBug>
  11:     0x7fd540fa2736 - std[94a519590adbd16d]::panic::panic_any::<rustc_errors[1da165d4f1c6a1c0]::ExplicitBug>
  12:     0x7fd540fa7706 - <rustc_errors[1da165d4f1c6a1c0]::HandlerInner as core[d8f8c7b14b601550]::ops::drop::Drop>::drop
  13:     0x7fd53e72c4b2 - core[d8f8c7b14b601550]::ptr::drop_in_place::<rustc_session[e22b767dbbfd943c]::parse::ParseSess>
  14:     0x7fd53e731aaa - <alloc[1b7dfdc50e21c679]::rc::Rc<rustc_session[e22b767dbbfd943c]::session::Session> as core[d8f8c7b14b601550]::ops::drop::Drop>::drop
  15:     0x7fd53e78376c - core[d8f8c7b14b601550]::ptr::drop_in_place::<rustc_interface[a2eba84e67d3cf00]::interface::Compiler>
  16:     0x7fd53e7735d9 - rustc_interface[a2eba84e67d3cf00]::interface::create_compiler_and_run::<core[d8f8c7b14b601550]::result::Result<(), rustc_errors[1da165d4f1c6a1c0]::ErrorGuaranteed>, rustc_driver[b560162ccb61062d]::run_compiler::{closure#1}>
  17:     0x7fd53e739ddc - std[94a519590adbd16d]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[a2eba84e67d3cf00]::util::run_in_thread_pool_with_globals<rustc_interface[a2eba84e67d3cf00]::interface::run_compiler<core[d8f8c7b14b601550]::result::Result<(), rustc_errors[1da165d4f1c6a1c0]::ErrorGuaranteed>, rustc_driver[b560162ccb61062d]::run_compiler::{closure#1}>::{closure#0}, core[d8f8c7b14b601550]::result::Result<(), rustc_errors[1da165d4f1c6a1c0]::ErrorGuaranteed>>::{closure#0}, core[d8f8c7b14b601550]::result::Result<(), rustc_errors[1da165d4f1c6a1c0]::ErrorGuaranteed>>
  18:     0x7fd53e7903d1 - std[94a519590adbd16d]::panic::catch_unwind::<core[d8f8c7b14b601550]::panic::unwind_safe::AssertUnwindSafe<<std[94a519590adbd16d]::thread::Builder>::spawn_unchecked_<rustc_interface[a2eba84e67d3cf00]::util::run_in_thread_pool_with_globals<rustc_interface[a2eba84e67d3cf00]::interface::run_compiler<core[d8f8c7b14b601550]::result::Result<(), rustc_errors[1da165d4f1c6a1c0]::ErrorGuaranteed>, rustc_driver[b560162ccb61062d]::run_compiler::{closure#1}>::{closure#0}, core[d8f8c7b14b601550]::result::Result<(), rustc_errors[1da165d4f1c6a1c0]::ErrorGuaranteed>>::{closure#0}, core[d8f8c7b14b601550]::result::Result<(), rustc_errors[1da165d4f1c6a1c0]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[d8f8c7b14b601550]::result::Result<(), rustc_errors[1da165d4f1c6a1c0]::ErrorGuaranteed>>
  19:     0x7fd53e73bb22 - <<std[94a519590adbd16d]::thread::Builder>::spawn_unchecked_<rustc_interface[a2eba84e67d3cf00]::util::run_in_thread_pool_with_globals<rustc_interface[a2eba84e67d3cf00]::interface::run_compiler<core[d8f8c7b14b601550]::result::Result<(), rustc_errors[1da165d4f1c6a1c0]::ErrorGuaranteed>, rustc_driver[b560162ccb61062d]::run_compiler::{closure#1}>::{closure#0}, core[d8f8c7b14b601550]::result::Result<(), rustc_errors[1da165d4f1c6a1c0]::ErrorGuaranteed>>::{closure#0}, core[d8f8c7b14b601550]::result::Result<(), rustc_errors[1da165d4f1c6a1c0]::ErrorGuaranteed>>::{closure#1} as core[d8f8c7b14b601550]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  20:     0x7fd53dca9a03 - std::sys::unix::thread::Thread::new::thread_start::h065355bb226194c3
  21:     0x7fd538016609 - start_thread
  22:     0x7fd53db0d293 - clone
  23:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.61.0-nightly (b446fac71 2022-03-06) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0 --crate-type lib
query stack during panic:
end of query stack
------------------------------------------

