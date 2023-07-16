plain
---- [pretty] src/test/pretty/anonymous-types.rs stdout ----

error: pretty-printing failed in round 0 revision None
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/pretty/anonymous-types.rs" "-Z" "unpretty=normal" "--target" "x86_64-unknown-linux-gnu" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/pretty/anonymous-types/auxiliary.pretty" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers"
// Test for issue 85480
// Pretty print anonymous struct and union types


// pp-exact

struct Foo {
    _: union  {
    _: union  {
        _: struct  {
            a: u8,
            b: u16,
        c: u32,
    },
    d: u64,
    e: f32,
---
    };

fn main() {}
------------------------------------------
--- stderr -------------------------------
error: internal compiler error: no errors encountered even though `delay_span_bug` issued
error: internal compiler error: expected type, found keyword `struct`
  --> /checkout/src/test/pretty/anonymous-types.rs:20:2
   |
20 |  struct {
20 |  struct {
   |  ^^^^^^ expected type

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1426:13
stack backtrace:
   0:     0x7f096382ad2c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h8f51349cd9845f35
   1:     0x7f0963891a08 - core::fmt::write::h3eab1d68c9be7914
   2:     0x7f096381b391 - std::io::Write::write_fmt::he7b28504b0b57f1e
   3:     0x7f096382dcde - std::panicking::default_hook::{{closure}}::hb2b67964cf71a513
   4:     0x7f096382d9a6 - std::panicking::default_hook::h8d598a816bd77a40
   5:     0x7f09641e1384 - rustc_driver[a487d662d491ed7a]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f096382e492 - std::panicking::rust_panic_with_hook::hc4aa9a8a627ed352
   7:     0x7f0966e4d243 - std[4a2a80dd7afb0540]::panicking::begin_panic::<rustc_errors[105efaf57a0dac8a]::ExplicitBug>::{closure#0}
   8:     0x7f0966e4ce06 - std[4a2a80dd7afb0540]::sys_common::backtrace::__rust_end_short_backtrace::<std[4a2a80dd7afb0540]::panicking::begin_panic<rustc_errors[105efaf57a0dac8a]::ExplicitBug>::{closure#0}, !>
   9:     0x7f09641a1bf6 - std[4a2a80dd7afb0540]::panicking::begin_panic::<rustc_errors[105efaf57a0dac8a]::ExplicitBug>
  10:     0x7f0966e910c6 - std[4a2a80dd7afb0540]::panic::panic_any::<rustc_errors[105efaf57a0dac8a]::ExplicitBug>
  11:     0x7f0966e95e6c - <rustc_errors[105efaf57a0dac8a]::HandlerInner as core[c001877e96235ee8]::ops::drop::Drop>::drop
  12:     0x7f09642018e2 - core[c001877e96235ee8]::ptr::drop_in_place::<rustc_session[35fe54be19709116]::parse::ParseSess>
  13:     0x7f0964209ab8 - <alloc[90583d39ca30ccef]::rc::Rc<rustc_session[35fe54be19709116]::session::Session> as core[c001877e96235ee8]::ops::drop::Drop>::drop
  14:     0x7f09642030ac - core[c001877e96235ee8]::ptr::drop_in_place::<rustc_interface[56c72d32e7a2fc8f]::interface::Compiler>
  15:     0x7f09641f9449 - rustc_span[b2329d8fed91f0a8]::with_source_map::<core[c001877e96235ee8]::result::Result<(), rustc_errors[105efaf57a0dac8a]::ErrorGuaranteed>, rustc_interface[56c72d32e7a2fc8f]::interface::create_compiler_and_run<core[c001877e96235ee8]::result::Result<(), rustc_errors[105efaf57a0dac8a]::ErrorGuaranteed>, rustc_driver[a487d662d491ed7a]::run_compiler::{closure#1}>::{closure#1}>
  16:     0x7f09641e5a3a - rustc_interface[56c72d32e7a2fc8f]::interface::create_compiler_and_run::<core[c001877e96235ee8]::result::Result<(), rustc_errors[105efaf57a0dac8a]::ErrorGuaranteed>, rustc_driver[a487d662d491ed7a]::run_compiler::{closure#1}>
  17:     0x7f09641cc8cf - <scoped_tls[9fa244cf078a9f7f]::ScopedKey<rustc_span[b2329d8fed91f0a8]::SessionGlobals>>::set::<rustc_interface[56c72d32e7a2fc8f]::interface::run_compiler<core[c001877e96235ee8]::result::Result<(), rustc_errors[105efaf57a0dac8a]::ErrorGuaranteed>, rustc_driver[a487d662d491ed7a]::run_compiler::{closure#1}>::{closure#0}, core[c001877e96235ee8]::result::Result<(), rustc_errors[105efaf57a0dac8a]::ErrorGuaranteed>>
  18:     0x7f09641d4cd9 - std[4a2a80dd7afb0540]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[56c72d32e7a2fc8f]::util::run_in_thread_pool_with_globals<rustc_interface[56c72d32e7a2fc8f]::interface::run_compiler<core[c001877e96235ee8]::result::Result<(), rustc_errors[105efaf57a0dac8a]::ErrorGuaranteed>, rustc_driver[a487d662d491ed7a]::run_compiler::{closure#1}>::{closure#0}, core[c001877e96235ee8]::result::Result<(), rustc_errors[105efaf57a0dac8a]::ErrorGuaranteed>>::{closure#0}, core[c001877e96235ee8]::result::Result<(), rustc_errors[105efaf57a0dac8a]::ErrorGuaranteed>>
  19:     0x7f0964241379 - <<std[4a2a80dd7afb0540]::thread::Builder>::spawn_unchecked_<rustc_interface[56c72d32e7a2fc8f]::util::run_in_thread_pool_with_globals<rustc_interface[56c72d32e7a2fc8f]::interface::run_compiler<core[c001877e96235ee8]::result::Result<(), rustc_errors[105efaf57a0dac8a]::ErrorGuaranteed>, rustc_driver[a487d662d491ed7a]::run_compiler::{closure#1}>::{closure#0}, core[c001877e96235ee8]::result::Result<(), rustc_errors[105efaf57a0dac8a]::ErrorGuaranteed>>::{closure#0}, core[c001877e96235ee8]::result::Result<(), rustc_errors[105efaf57a0dac8a]::ErrorGuaranteed>>::{closure#1} as core[c001877e96235ee8]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  20:     0x7f0963839815 - std::sys::unix::thread::Thread::new::thread_start::hed32aff6b46929fd
  21:     0x7f095dd84609 - start_thread
  22:     0x7f0963697133 - clone
  23:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.64.0-nightly (8f81fa257 2022-07-27) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z unpretty=normal -C rpath -C debuginfo=0
query stack during panic:
end of query stack
------------------------------------------

