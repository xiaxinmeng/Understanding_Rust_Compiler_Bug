plain
---- [ui] src/test/ui/trait-bounds/issue-94680.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/trait-bounds/issue-94680.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/trait-bounds/issue-94680" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/trait-bounds/issue-94680/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: Encountered error `Unimplemented` selecting `Binder(<&mut () as std::clone::Clone>, [])` during codegen
   = note: delayed at compiler/rustc_trait_selection/src/traits/codegen.rs:65:43

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1347:13
stack backtrace:
stack backtrace:
   0:     0x7f3e7d5c39ac - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::he79ffb09e8b8e035
   1:     0x7f3e7d629ff8 - core::fmt::write::h035d7b490ff26b4c
   2:     0x7f3e7d5b3811 - std::io::Write::write_fmt::he5b8713360e03a00
   3:     0x7f3e7d5c69de - std::panicking::default_hook::{{closure}}::hc39a4a7068837970
   4:     0x7f3e7d5c660c - std::panicking::default_hook::h786685b4cb9e7e2f
   5:     0x7f3e7e11f701 - rustc_driver[d2bc6b22aefc2e08]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f3e7d5c723e - std::panicking::rust_panic_with_hook::h7bc736bf008aebcd
   7:     0x7f3e80c01e13 - std[26c616c8f821e00]::panicking::begin_panic::<rustc_errors[4311cf657190ca4f]::ExplicitBug>::{closure#0}
   8:     0x7f3e80c00a66 - std[26c616c8f821e00]::sys_common::backtrace::__rust_end_short_backtrace::<std[26c616c8f821e00]::panicking::begin_panic<rustc_errors[4311cf657190ca4f]::ExplicitBug>::{closure#0}, !>
   9:     0x7f3e7e05c39f - std[26c616c8f821e00]::panicking::begin_panic::<rustc_errors[4311cf657190ca4f]::ExplicitBug>
  10:     0x7f3e80c08126 - std[26c616c8f821e00]::panic::panic_any::<rustc_errors[4311cf657190ca4f]::ExplicitBug>
  11:     0x7f3e80c0c6e5 - <rustc_errors[4311cf657190ca4f]::HandlerInner as core[184437b00b3715b2]::ops::drop::Drop>::drop
  12:     0x7f3e7e0a9bc2 - core[184437b00b3715b2]::ptr::drop_in_place::<rustc_session[b7f8b9dce94a9819]::parse::ParseSess>
  13:     0x7f3e7e0af548 - <alloc[d4fe1911169e2e6a]::rc::Rc<rustc_session[b7f8b9dce94a9819]::session::Session> as core[184437b00b3715b2]::ops::drop::Drop>::drop
  14:     0x7f3e7e0960fc - core[184437b00b3715b2]::ptr::drop_in_place::<rustc_interface[6f102b65a1a509e8]::interface::Compiler>
  15:     0x7f3e7e093e14 - rustc_span[fd2f1606d087ff6f]::with_source_map::<core[184437b00b3715b2]::result::Result<(), rustc_errors[4311cf657190ca4f]::ErrorGuaranteed>, rustc_interface[6f102b65a1a509e8]::interface::create_compiler_and_run<core[184437b00b3715b2]::result::Result<(), rustc_errors[4311cf657190ca4f]::ErrorGuaranteed>, rustc_driver[d2bc6b22aefc2e08]::run_compiler::{closure#1}>::{closure#1}>
  16:     0x7f3e7e0b21b9 - <scoped_tls[32310630fbc94439]::ScopedKey<rustc_span[fd2f1606d087ff6f]::SessionGlobals>>::set::<rustc_interface[6f102b65a1a509e8]::interface::run_compiler<core[184437b00b3715b2]::result::Result<(), rustc_errors[4311cf657190ca4f]::ErrorGuaranteed>, rustc_driver[d2bc6b22aefc2e08]::run_compiler::{closure#1}>::{closure#0}, core[184437b00b3715b2]::result::Result<(), rustc_errors[4311cf657190ca4f]::ErrorGuaranteed>>
  17:     0x7f3e7e1066f9 - std[26c616c8f821e00]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[6f102b65a1a509e8]::util::run_in_thread_pool_with_globals<rustc_interface[6f102b65a1a509e8]::interface::run_compiler<core[184437b00b3715b2]::result::Result<(), rustc_errors[4311cf657190ca4f]::ErrorGuaranteed>, rustc_driver[d2bc6b22aefc2e08]::run_compiler::{closure#1}>::{closure#0}, core[184437b00b3715b2]::result::Result<(), rustc_errors[4311cf657190ca4f]::ErrorGuaranteed>>::{closure#0}, core[184437b00b3715b2]::result::Result<(), rustc_errors[4311cf657190ca4f]::ErrorGuaranteed>>
  18:     0x7f3e7e0c65b1 - std[26c616c8f821e00]::panicking::try::<core[184437b00b3715b2]::result::Result<(), rustc_errors[4311cf657190ca4f]::ErrorGuaranteed>, core[184437b00b3715b2]::panic::unwind_safe::AssertUnwindSafe<<std[26c616c8f821e00]::thread::Builder>::spawn_unchecked_<rustc_interface[6f102b65a1a509e8]::util::run_in_thread_pool_with_globals<rustc_interface[6f102b65a1a509e8]::interface::run_compiler<core[184437b00b3715b2]::result::Result<(), rustc_errors[4311cf657190ca4f]::ErrorGuaranteed>, rustc_driver[d2bc6b22aefc2e08]::run_compiler::{closure#1}>::{closure#0}, core[184437b00b3715b2]::result::Result<(), rustc_errors[4311cf657190ca4f]::ErrorGuaranteed>>::{closure#0}, core[184437b00b3715b2]::result::Result<(), rustc_errors[4311cf657190ca4f]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  19:     0x7f3e7e107412 - <<std[26c616c8f821e00]::thread::Builder>::spawn_unchecked_<rustc_interface[6f102b65a1a509e8]::util::run_in_thread_pool_with_globals<rustc_interface[6f102b65a1a509e8]::interface::run_compiler<core[184437b00b3715b2]::result::Result<(), rustc_errors[4311cf657190ca4f]::ErrorGuaranteed>, rustc_driver[d2bc6b22aefc2e08]::run_compiler::{closure#1}>::{closure#0}, core[184437b00b3715b2]::result::Result<(), rustc_errors[4311cf657190ca4f]::ErrorGuaranteed>>::{closure#0}, core[184437b00b3715b2]::result::Result<(), rustc_errors[4311cf657190ca4f]::ErrorGuaranteed>>::{closure#1} as core[184437b00b3715b2]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  20:     0x7f3e7d5d2593 - std::sys::unix::thread::Thread::new::thread_start::h8a8826562b08a72e
  21:     0x7f3e77b25609 - start_thread
  22:     0x7f3e7d438163 - clone
  23:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.62.0-nightly (887cca3cf 2022-04-26) running on x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
------------------------------------------



---- [ui] src/test/ui/trait-bounds/issue-94999.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/trait-bounds/issue-94999.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/trait-bounds/issue-94999" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/trait-bounds/issue-94999/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: Encountered error `Unimplemented` selecting `Binder(<S as std::clone::Clone>, [])` during codegen
   = note: delayed at compiler/rustc_trait_selection/src/traits/codegen.rs:65:43

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1347:13
stack backtrace:
stack backtrace:
   0:     0x7f912d5199ac - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::he79ffb09e8b8e035
   1:     0x7f912d57fff8 - core::fmt::write::h035d7b490ff26b4c
   2:     0x7f912d509811 - std::io::Write::write_fmt::he5b8713360e03a00
   3:     0x7f912d51c9de - std::panicking::default_hook::{{closure}}::hc39a4a7068837970
   4:     0x7f912d51c60c - std::panicking::default_hook::h786685b4cb9e7e2f
   5:     0x7f912e075701 - rustc_driver[d2bc6b22aefc2e08]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f912d51d23e - std::panicking::rust_panic_with_hook::h7bc736bf008aebcd
   7:     0x7f9130b57e13 - std[26c616c8f821e00]::panicking::begin_panic::<rustc_errors[4311cf657190ca4f]::ExplicitBug>::{closure#0}
   8:     0x7f9130b56a66 - std[26c616c8f821e00]::sys_common::backtrace::__rust_end_short_backtrace::<std[26c616c8f821e00]::panicking::begin_panic<rustc_errors[4311cf657190ca4f]::ExplicitBug>::{closure#0}, !>
   9:     0x7f912dfb239f - std[26c616c8f821e00]::panicking::begin_panic::<rustc_errors[4311cf657190ca4f]::ExplicitBug>
  10:     0x7f9130b5e126 - std[26c616c8f821e00]::panic::panic_any::<rustc_errors[4311cf657190ca4f]::ExplicitBug>
  11:     0x7f9130b626e5 - <rustc_errors[4311cf657190ca4f]::HandlerInner as core[184437b00b3715b2]::ops::drop::Drop>::drop
  12:     0x7f912dfffbc2 - core[184437b00b3715b2]::ptr::drop_in_place::<rustc_session[b7f8b9dce94a9819]::parse::ParseSess>
  13:     0x7f912e005548 - <alloc[d4fe1911169e2e6a]::rc::Rc<rustc_session[b7f8b9dce94a9819]::session::Session> as core[184437b00b3715b2]::ops::drop::Drop>::drop
  14:     0x7f912dfec0fc - core[184437b00b3715b2]::ptr::drop_in_place::<rustc_interface[6f102b65a1a509e8]::interface::Compiler>
  15:     0x7f912dfe9e14 - rustc_span[fd2f1606d087ff6f]::with_source_map::<core[184437b00b3715b2]::result::Result<(), rustc_errors[4311cf657190ca4f]::ErrorGuaranteed>, rustc_interface[6f102b65a1a509e8]::interface::create_compiler_and_run<core[184437b00b3715b2]::result::Result<(), rustc_errors[4311cf657190ca4f]::ErrorGuaranteed>, rustc_driver[d2bc6b22aefc2e08]::run_compiler::{closure#1}>::{closure#1}>
  16:     0x7f912e0081b9 - <scoped_tls[32310630fbc94439]::ScopedKey<rustc_span[fd2f1606d087ff6f]::SessionGlobals>>::set::<rustc_interface[6f102b65a1a509e8]::interface::run_compiler<core[184437b00b3715b2]::result::Result<(), rustc_errors[4311cf657190ca4f]::ErrorGuaranteed>, rustc_driver[d2bc6b22aefc2e08]::run_compiler::{closure#1}>::{closure#0}, core[184437b00b3715b2]::result::Result<(), rustc_errors[4311cf657190ca4f]::ErrorGuaranteed>>
  17:     0x7f912e05c6f9 - std[26c616c8f821e00]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[6f102b65a1a509e8]::util::run_in_thread_pool_with_globals<rustc_interface[6f102b65a1a509e8]::interface::run_compiler<core[184437b00b3715b2]::result::Result<(), rustc_errors[4311cf657190ca4f]::ErrorGuaranteed>, rustc_driver[d2bc6b22aefc2e08]::run_compiler::{closure#1}>::{closure#0}, core[184437b00b3715b2]::result::Result<(), rustc_errors[4311cf657190ca4f]::ErrorGuaranteed>>::{closure#0}, core[184437b00b3715b2]::result::Result<(), rustc_errors[4311cf657190ca4f]::ErrorGuaranteed>>
  18:     0x7f912e01c5b1 - std[26c616c8f821e00]::panicking::try::<core[184437b00b3715b2]::result::Result<(), rustc_errors[4311cf657190ca4f]::ErrorGuaranteed>, core[184437b00b3715b2]::panic::unwind_safe::AssertUnwindSafe<<std[26c616c8f821e00]::thread::Builder>::spawn_unchecked_<rustc_interface[6f102b65a1a509e8]::util::run_in_thread_pool_with_globals<rustc_interface[6f102b65a1a509e8]::interface::run_compiler<core[184437b00b3715b2]::result::Result<(), rustc_errors[4311cf657190ca4f]::ErrorGuaranteed>, rustc_driver[d2bc6b22aefc2e08]::run_compiler::{closure#1}>::{closure#0}, core[184437b00b3715b2]::result::Result<(), rustc_errors[4311cf657190ca4f]::ErrorGuaranteed>>::{closure#0}, core[184437b00b3715b2]::result::Result<(), rustc_errors[4311cf657190ca4f]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  19:     0x7f912e05d412 - <<std[26c616c8f821e00]::thread::Builder>::spawn_unchecked_<rustc_interface[6f102b65a1a509e8]::util::run_in_thread_pool_with_globals<rustc_interface[6f102b65a1a509e8]::interface::run_compiler<core[184437b00b3715b2]::result::Result<(), rustc_errors[4311cf657190ca4f]::ErrorGuaranteed>, rustc_driver[d2bc6b22aefc2e08]::run_compiler::{closure#1}>::{closure#0}, core[184437b00b3715b2]::result::Result<(), rustc_errors[4311cf657190ca4f]::ErrorGuaranteed>>::{closure#0}, core[184437b00b3715b2]::result::Result<(), rustc_errors[4311cf657190ca4f]::ErrorGuaranteed>>::{closure#1} as core[184437b00b3715b2]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  20:     0x7f912d528593 - std::sys::unix::thread::Thread::new::thread_start::h8a8826562b08a72e
  21:     0x7f9127a7b609 - start_thread
  22:     0x7f912d38e163 - clone
  23:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.62.0-nightly (887cca3cf 2022-04-26) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
------------------------------------------

