plain
---- [assembly] assembly/sparc-struct-abi.rs stdout ----

error: compilation failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/assembly/sparc-struct-abi.rs" "-Zthreads=1" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/sparc-struct-abi/sparc-struct-abi.s" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target=sparcv9-sun-solaris" "-Copt-level=3" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/sparc-struct-abi/auxiliary" "--emit=asm"
stdout: none
--- stderr -------------------------------
error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: broken MIR in DefId(0:8 ~ sparc_struct_abi[aab5]::callee) (NoSolution): could not prove Binder(TraitPredicate(<f32 as Copy>, polarity:Positive), [])
  = note: delayed at compiler/rustc_borrowck/src/type_check/canonical.rs:150:13

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1266:13
stack backtrace:
stack backtrace:
   0:     0x7fee322685dc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha34cb1a4e3d1fa48
   1:     0x7fee322cfa0f - core::fmt::write::h720982d4ebe747f4
   2:     0x7fee32257aa1 - std::io::Write::write_fmt::hb29d6f66672a0faf
   3:     0x7fee322683fb - std::sys_common::backtrace::print::hac14395ab32443ba
   4:     0x7fee3226bb34 - std::panicking::default_hook::{{closure}}::hafa68f9833f5ee48
   5:     0x7fee3226b6ed - std::panicking::default_hook::hcc71497c32374ed5
   6:     0x7fee32ce2971 - rustc_driver[b240abd04a67a3ea]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fee3226c21a - std::panicking::rust_panic_with_hook::hd097ab3fa29ee743
   8:     0x7fee35540953 - std[e76133a8c602976]::panicking::begin_panic::<rustc_errors[7ccc56b73cb3eb67]::ExplicitBug>::{closure#0}
   9:     0x7fee35540786 - std[e76133a8c602976]::sys_common::backtrace::__rust_end_short_backtrace::<std[e76133a8c602976]::panicking::begin_panic<rustc_errors[7ccc56b73cb3eb67]::ExplicitBug>::{closure#0}, !>
  10:     0x7fee32ca1acf - std[e76133a8c602976]::panicking::begin_panic::<rustc_errors[7ccc56b73cb3eb67]::ExplicitBug>
  11:     0x7fee35572786 - std[e76133a8c602976]::panic::panic_any::<rustc_errors[7ccc56b73cb3eb67]::ExplicitBug>
  12:     0x7fee35577786 - <rustc_errors[7ccc56b73cb3eb67]::HandlerInner as core[b442dd798ecd1115]::ops::drop::Drop>::drop
  13:     0x7fee32cfa5a2 - core[b442dd798ecd1115]::ptr::drop_in_place::<rustc_session[b13d32ef17fe0d90]::parse::ParseSess>
  14:     0x7fee32cff8fa - <alloc[8d91fa36a5bae629]::rc::Rc<rustc_session[b13d32ef17fe0d90]::session::Session> as core[b442dd798ecd1115]::ops::drop::Drop>::drop
  15:     0x7fee32d5427c - core[b442dd798ecd1115]::ptr::drop_in_place::<rustc_interface[71a089f345d90f4b]::interface::Compiler>
  16:     0x7fee32d41659 - rustc_interface[71a089f345d90f4b]::interface::create_compiler_and_run::<core[b442dd798ecd1115]::result::Result<(), rustc_errors[7ccc56b73cb3eb67]::ErrorGuaranteed>, rustc_driver[b240abd04a67a3ea]::run_compiler::{closure#1}>
  17:     0x7fee32d07f3c - std[e76133a8c602976]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[71a089f345d90f4b]::util::run_in_thread_pool_with_globals<rustc_interface[71a089f345d90f4b]::interface::run_compiler<core[b442dd798ecd1115]::result::Result<(), rustc_errors[7ccc56b73cb3eb67]::ErrorGuaranteed>, rustc_driver[b240abd04a67a3ea]::run_compiler::{closure#1}>::{closure#0}, core[b442dd798ecd1115]::result::Result<(), rustc_errors[7ccc56b73cb3eb67]::ErrorGuaranteed>>::{closure#0}, core[b442dd798ecd1115]::result::Result<(), rustc_errors[7ccc56b73cb3eb67]::ErrorGuaranteed>>
  18:     0x7fee32d5e501 - std[e76133a8c602976]::panic::catch_unwind::<core[b442dd798ecd1115]::panic::unwind_safe::AssertUnwindSafe<<std[e76133a8c602976]::thread::Builder>::spawn_unchecked_<rustc_interface[71a089f345d90f4b]::util::run_in_thread_pool_with_globals<rustc_interface[71a089f345d90f4b]::interface::run_compiler<core[b442dd798ecd1115]::result::Result<(), rustc_errors[7ccc56b73cb3eb67]::ErrorGuaranteed>, rustc_driver[b240abd04a67a3ea]::run_compiler::{closure#1}>::{closure#0}, core[b442dd798ecd1115]::result::Result<(), rustc_errors[7ccc56b73cb3eb67]::ErrorGuaranteed>>::{closure#0}, core[b442dd798ecd1115]::result::Result<(), rustc_errors[7ccc56b73cb3eb67]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[b442dd798ecd1115]::result::Result<(), rustc_errors[7ccc56b73cb3eb67]::ErrorGuaranteed>>
  19:     0x7fee32d09c82 - <<std[e76133a8c602976]::thread::Builder>::spawn_unchecked_<rustc_interface[71a089f345d90f4b]::util::run_in_thread_pool_with_globals<rustc_interface[71a089f345d90f4b]::interface::run_compiler<core[b442dd798ecd1115]::result::Result<(), rustc_errors[7ccc56b73cb3eb67]::ErrorGuaranteed>, rustc_driver[b240abd04a67a3ea]::run_compiler::{closure#1}>::{closure#0}, core[b442dd798ecd1115]::result::Result<(), rustc_errors[7ccc56b73cb3eb67]::ErrorGuaranteed>>::{closure#0}, core[b442dd798ecd1115]::result::Result<(), rustc_errors[7ccc56b73cb3eb67]::ErrorGuaranteed>>::{closure#1} as core[b442dd798ecd1115]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  20:     0x7fee32277a03 - std::sys::unix::thread::Thread::new::thread_start::h1879692b59064a83
  21:     0x7fee2c5e4609 - start_thread
  22:     0x7fee320db293 - clone
  23:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.61.0-nightly (74d98c1ca 2022-03-06) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C prefer-dynamic -C rpath -C debuginfo=0 -C opt-level=3
query stack during panic:
end of query stack
------------------------------------------

