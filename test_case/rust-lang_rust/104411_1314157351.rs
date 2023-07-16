plain
...ii.i................................................................................. 13816/13841
.........................
failures:

---- [ui] src/test/ui/mir/important-higher-ranked-regions.rs stdout ----
error: test compilation failed although it shouldn't!
status: exit status: 101
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/mir/important-higher-ranked-regions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/important-higher-ranked-regions" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/important-higher-ranked-regions/auxiliary" "-Zvalidate-mir"
stdout: none
--- stderr -------------------------------
error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: broken MIR in DefId(0:15 ~ important_higher_ranked_regions[449f]::foo) (_0 = move _1): bad assignment (Foo<fn(&()), std::string::String> = Foo<for<'a> fn(&'a ()), u32>): NoSolution
  --> /checkout/src/test/ui/mir/important-higher-ranked-regions.rs:20:5
LL |     x
   |     ^
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   |
   |
   = note: delayed at compiler/rustc_borrowck/src/type_check/mod.rs:1246:21

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1585:13
stack backtrace:
   0:     0x7f84c58d8e4e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hf48913315401f1fa
   1:     0x7f84c59486e8 - core::fmt::write::hf0487edd1f0daf97
   2:     0x7f84c58cab51 - std::io::Write::write_fmt::h6446acd83ecbec68
   3:     0x7f84c58d8c51 - std::sys_common::backtrace::print::h73ad6e5a976af88e
   4:     0x7f84c58dbfb4 - std::panicking::default_hook::{{closure}}::h8d6265b4fbe8798c
   5:     0x7f84c58dbc79 - std::panicking::default_hook::h01cd0220995411b0
   6:     0x7f84c62e0894 - rustc_driver[5827a64b6883bcf2]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f84c58dc704 - std::panicking::rust_panic_with_hook::h4a7ba4e9f410b137
   8:     0x7f84c922cfc3 - std[6c6d2ca7878e64a5]::panicking::begin_panic::<rustc_errors[ba1b5abbe99caf33]::ExplicitBug>::{closure#0}
   9:     0x7f84c92298f6 - std[6c6d2ca7878e64a5]::sys_common::backtrace::__rust_end_short_backtrace::<std[6c6d2ca7878e64a5]::panicking::begin_panic<rustc_errors[ba1b5abbe99caf33]::ExplicitBug>::{closure#0}, !>
  10:     0x7f84c62aa586 - std[6c6d2ca7878e64a5]::panicking::begin_panic::<rustc_errors[ba1b5abbe99caf33]::ExplicitBug>
  11:     0x7f84c9286bc6 - std[6c6d2ca7878e64a5]::panic::panic_any::<rustc_errors[ba1b5abbe99caf33]::ExplicitBug>
  12:     0x7f84c928fa75 - <rustc_errors[ba1b5abbe99caf33]::HandlerInner>::flush_delayed::<alloc[9d9e7a2183ba6c34]::vec::Vec<rustc_errors[ba1b5abbe99caf33]::diagnostic::Diagnostic>, &str>
  13:     0x7f84c928df61 - <rustc_errors[ba1b5abbe99caf33]::Handler>::flush_delayed
  14:     0x7f84c64384ac - <rustc_interface[ca064b433a0aca7f]::passes::QueryContext>::enter::<<rustc_interface[ca064b433a0aca7f]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[f4e89e42597d90a5]::result::Result<alloc[9d9e7a2183ba6c34]::boxed::Box<dyn core[f4e89e42597d90a5]::any::Any>, rustc_errors[ba1b5abbe99caf33]::ErrorGuaranteed>>
  15:     0x7f84c641e562 - <rustc_interface[ca064b433a0aca7f]::queries::Queries>::ongoing_codegen
  16:     0x7f84c6350aea - <rustc_interface[ca064b433a0aca7f]::interface::Compiler>::enter::<rustc_driver[5827a64b6883bcf2]::run_compiler::{closure#1}::{closure#2}, core[f4e89e42597d90a5]::result::Result<core[f4e89e42597d90a5]::option::Option<rustc_interface[ca064b433a0aca7f]::queries::Linker>, rustc_errors[ba1b5abbe99caf33]::ErrorGuaranteed>>
  17:     0x7f84c62e1fde - rustc_span[bec6c3e9dab5248f]::with_source_map::<core[f4e89e42597d90a5]::result::Result<(), rustc_errors[ba1b5abbe99caf33]::ErrorGuaranteed>, rustc_interface[ca064b433a0aca7f]::interface::run_compiler<core[f4e89e42597d90a5]::result::Result<(), rustc_errors[ba1b5abbe99caf33]::ErrorGuaranteed>, rustc_driver[5827a64b6883bcf2]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  18:     0x7f84c63437fc - <scoped_tls[939839471e13eda0]::ScopedKey<rustc_span[bec6c3e9dab5248f]::SessionGlobals>>::set::<rustc_interface[ca064b433a0aca7f]::interface::run_compiler<core[f4e89e42597d90a5]::result::Result<(), rustc_errors[ba1b5abbe99caf33]::ErrorGuaranteed>, rustc_driver[5827a64b6883bcf2]::run_compiler::{closure#1}>::{closure#0}, core[f4e89e42597d90a5]::result::Result<(), rustc_errors[ba1b5abbe99caf33]::ErrorGuaranteed>>
  19:     0x7f84c633f219 - std[6c6d2ca7878e64a5]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[ca064b433a0aca7f]::util::run_in_thread_pool_with_globals<rustc_interface[ca064b433a0aca7f]::interface::run_compiler<core[f4e89e42597d90a5]::result::Result<(), rustc_errors[ba1b5abbe99caf33]::ErrorGuaranteed>, rustc_driver[5827a64b6883bcf2]::run_compiler::{closure#1}>::{closure#0}, core[f4e89e42597d90a5]::result::Result<(), rustc_errors[ba1b5abbe99caf33]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[f4e89e42597d90a5]::result::Result<(), rustc_errors[ba1b5abbe99caf33]::ErrorGuaranteed>>
  20:     0x7f84c6344906 - std[6c6d2ca7878e64a5]::panic::catch_unwind::<core[f4e89e42597d90a5]::panic::unwind_safe::AssertUnwindSafe<<std[6c6d2ca7878e64a5]::thread::Builder>::spawn_unchecked_<rustc_interface[ca064b433a0aca7f]::util::run_in_thread_pool_with_globals<rustc_interface[ca064b433a0aca7f]::interface::run_compiler<core[f4e89e42597d90a5]::result::Result<(), rustc_errors[ba1b5abbe99caf33]::ErrorGuaranteed>, rustc_driver[5827a64b6883bcf2]::run_compiler::{closure#1}>::{closure#0}, core[f4e89e42597d90a5]::result::Result<(), rustc_errors[ba1b5abbe99caf33]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[f4e89e42597d90a5]::result::Result<(), rustc_errors[ba1b5abbe99caf33]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[f4e89e42597d90a5]::result::Result<(), rustc_errors[ba1b5abbe99caf33]::ErrorGuaranteed>>
  21:     0x7f84c62f10fa - <<std[6c6d2ca7878e64a5]::thread::Builder>::spawn_unchecked_<rustc_interface[ca064b433a0aca7f]::util::run_in_thread_pool_with_globals<rustc_interface[ca064b433a0aca7f]::interface::run_compiler<core[f4e89e42597d90a5]::result::Result<(), rustc_errors[ba1b5abbe99caf33]::ErrorGuaranteed>, rustc_driver[5827a64b6883bcf2]::run_compiler::{closure#1}>::{closure#0}, core[f4e89e42597d90a5]::result::Result<(), rustc_errors[ba1b5abbe99caf33]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[f4e89e42597d90a5]::result::Result<(), rustc_errors[ba1b5abbe99caf33]::ErrorGuaranteed>>::{closure#1} as core[f4e89e42597d90a5]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  22:     0x7f84c58e8e3e - std::sys::unix::thread::Thread::new::thread_start::h250691131591813b
  23:     0x7f84c567fb43 - <unknown>
  24:     0x7f84c5711a00 - <unknown>
  25:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.67.0-nightly (49585e261 2022-11-14) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 -Z validate-mir
query stack during panic:
end of query stack
error: aborting due to 2 previous errors
------------------------------------------
