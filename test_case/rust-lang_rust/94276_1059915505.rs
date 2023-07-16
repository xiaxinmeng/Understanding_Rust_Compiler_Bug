plain

running 328 tests
ii...i........i....i..i.................iii.......iii.......i.................ii.................i.. 100/328
............i................i..iii........i..i..............i.......i..............i...i...i.....ii 200/328
..i.ii.............iiii..F.....................i.ii.i......i.......iii..........i................... 300/328
Some tests failed in compiletest suite=codegen mode=codegen host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
failures:

---- [codegen] codegen/riscv-abi/riscv64-lp64-lp64f-lp64d-abi.rs stdout ----


error: compilation failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/codegen/riscv-abi/riscv64-lp64-lp64f-lp64d-abi.rs" "-Zthreads=1" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/riscv-abi/riscv64-lp64-lp64f-lp64d-abi" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target" "riscv64gc-unknown-linux-gnu" "-O" "-C" "no-prepopulate-passes" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/riscv-abi/riscv64-lp64-lp64f-lp64d-abi/auxiliary" "--emit=llvm-ir"
stdout: none
--- stderr -------------------------------
warning: unused variable: `e`
   |
   |
78 | pub extern "C" fn f_agg_tiny(mut e: Tiny) {
   |                                  ^ help: if this is intentional, prefix it with an underscore: `_e`
   = note: `#[warn(unused_variables)]` on by default

warning: unused variable: `x`
  --> /checkout/src/test/codegen/riscv-abi/riscv64-lp64-lp64f-lp64d-abi.rs:95:35
  --> /checkout/src/test/codegen/riscv-abi/riscv64-lp64-lp64f-lp64d-abi.rs:95:35
   |
95 | pub extern "C" fn f_agg_small(mut x: Small) {
   |                                   ^ help: if this is intentional, prefix it with an underscore: `_x`
warning: unused variable: `x`
   --> /checkout/src/test/codegen/riscv-abi/riscv64-lp64-lp64f-lp64d-abi.rs:111:43
    |
    |
111 | pub extern "C" fn f_agg_small_aligned(mut x: SmallAligned) {
    |                                           ^ help: if this is intentional, prefix it with an underscore: `_x`
warning: unused variable: `x`
   --> /checkout/src/test/codegen/riscv-abi/riscv64-lp64-lp64f-lp64d-abi.rs:124:35
    |
    |
124 | pub extern "C" fn f_agg_large(mut x: Large) {
    |                                   ^ help: if this is intentional, prefix it with an underscore: `_x`
warning: unused variable: `i`
   --> /checkout/src/test/codegen/riscv-abi/riscv64-lp64-lp64f-lp64d-abi.rs:129:35
    |
    |
129 | pub extern "C" fn f_agg_large_ret(i: i32, j: i8) -> Large {
    |                                   ^ help: if this is intentional, prefix it with an underscore: `_i`
warning: unused variable: `j`
   --> /checkout/src/test/codegen/riscv-abi/riscv64-lp64-lp64f-lp64d-abi.rs:129:43
    |
    |
129 | pub extern "C" fn f_agg_large_ret(i: i32, j: i8) -> Large {
    |                                           ^ help: if this is intentional, prefix it with an underscore: `_j`
warning: unused variable: `a`
   --> /checkout/src/test/codegen/riscv-abi/riscv64-lp64-lp64f-lp64d-abi.rs:136:5
    |
    |
136 |     a: Tiny,
    |     ^ help: if this is intentional, prefix it with an underscore: `_a`
warning: unused variable: `b`
   --> /checkout/src/test/codegen/riscv-abi/riscv64-lp64-lp64f-lp64d-abi.rs:137:5
    |
    |
137 |     b: Small,
    |     ^ help: if this is intentional, prefix it with an underscore: `_b`
warning: unused variable: `c`
   --> /checkout/src/test/codegen/riscv-abi/riscv64-lp64-lp64f-lp64d-abi.rs:138:5
    |
    |
138 |     c: SmallAligned,
    |     ^ help: if this is intentional, prefix it with an underscore: `_c`
warning: unused variable: `d`
   --> /checkout/src/test/codegen/riscv-abi/riscv64-lp64-lp64f-lp64d-abi.rs:139:5
    |
139 |     d: Large,
---

warning: unused variable: `b`
   --> /checkout/src/test/codegen/riscv-abi/riscv64-lp64-lp64f-lp64d-abi.rs:151:5
    |
151 |     b: SmallAligned,
    |     ^ help: if this is intentional, prefix it with an underscore: `_b`
warning: unused variable: `c`
   --> /checkout/src/test/codegen/riscv-abi/riscv64-lp64-lp64f-lp64d-abi.rs:152:5
    |
    |
152 |     c: SmallAligned,
    |     ^ help: if this is intentional, prefix it with an underscore: `_c`
warning: unused variable: `d`
   --> /checkout/src/test/codegen/riscv-abi/riscv64-lp64-lp64f-lp64d-abi.rs:153:5
    |
153 |     d: u64,
153 |     d: u64,
    |     ^ help: if this is intentional, prefix it with an underscore: `_d`

warning: variable does not need to be mutable
  --> /checkout/src/test/codegen/riscv-abi/riscv64-lp64-lp64f-lp64d-abi.rs:78:30
   |
78 | pub extern "C" fn f_agg_tiny(mut e: Tiny) {
   |                              |
   |                              help: remove this `mut`
   |
   = note: `#[warn(unused_mut)]` on by default
   = note: `#[warn(unused_mut)]` on by default

warning: variable does not need to be mutable
  --> /checkout/src/test/codegen/riscv-abi/riscv64-lp64-lp64f-lp64d-abi.rs:95:31
   |
95 | pub extern "C" fn f_agg_small(mut x: Small) {
   |                               |
   |                               help: remove this `mut`

warning: variable does not need to be mutable
warning: variable does not need to be mutable
   --> /checkout/src/test/codegen/riscv-abi/riscv64-lp64-lp64f-lp64d-abi.rs:111:39
    |
111 | pub extern "C" fn f_agg_small_aligned(mut x: SmallAligned) {
    |                                       |
    |                                       help: remove this `mut`

warning: variable does not need to be mutable
warning: variable does not need to be mutable
   --> /checkout/src/test/codegen/riscv-abi/riscv64-lp64-lp64f-lp64d-abi.rs:124:31
    |
124 | pub extern "C" fn f_agg_large(mut x: Large) {
    |                               |
    |                               help: remove this `mut`


warning: `extern` fn uses type `Empty`, which is not FFI-safe
   |
   |
64 | pub extern "C" fn f_agg_empty_struct(e: Empty) -> Empty {
   |                                         ^^^^^ not FFI-safe
   = note: `#[warn(improper_ctypes_definitions)]` on by default
   = help: consider adding a member to this struct
   = note: this struct has no fields
note: the type is defined here
note: the type is defined here
  --> /checkout/src/test/codegen/riscv-abi/riscv64-lp64-lp64f-lp64d-abi.rs:60:1
   |
60 | pub struct Empty {}
   | ^^^^^^^^^^^^^^^^^^^

warning: `extern` fn uses type `Empty`, which is not FFI-safe
   |
   |
64 | pub extern "C" fn f_agg_empty_struct(e: Empty) -> Empty {
   |                                                   ^^^^^ not FFI-safe
   = help: consider adding a member to this struct
   = note: this struct has no fields
note: the type is defined here
  --> /checkout/src/test/codegen/riscv-abi/riscv64-lp64-lp64f-lp64d-abi.rs:60:1
  --> /checkout/src/test/codegen/riscv-abi/riscv64-lp64-lp64f-lp64d-abi.rs:60:1
   |
60 | pub struct Empty {}
   | ^^^^^^^^^^^^^^^^^^^

warning: `extern` fn uses type `i128`, which is not FFI-safe
    |
    |
111 | pub extern "C" fn f_agg_small_aligned(mut x: SmallAligned) {
    |                                              ^^^^^^^^^^^^ not FFI-safe
    |
    = note: 128-bit integers don't currently have a known stable ABI

warning: `extern` fn uses type `i128`, which is not FFI-safe
    |
    |
138 |     c: SmallAligned,
    |        ^^^^^^^^^^^^ not FFI-safe
    |
    = note: 128-bit integers don't currently have a known stable ABI

warning: `extern` fn uses type `i128`, which is not FFI-safe
    |
    |
151 |     b: SmallAligned,
    |        ^^^^^^^^^^^^ not FFI-safe
    |
    = note: 128-bit integers don't currently have a known stable ABI

warning: `extern` fn uses type `i128`, which is not FFI-safe
    |
    |
152 |     c: SmallAligned,
    |        ^^^^^^^^^^^^ not FFI-safe
    |
    = note: 128-bit integers don't currently have a known stable ABI
warning: 27 warnings emitted


error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: broken MIR in DefId(0:4 ~ riscv64_lp64_lp64f_lp64d_abi[45a3]::f_scalar_0) (NoSolution): could not prove Binder(TraitPredicate(<bool as Copy>, polarity:Positive), [])
  = note: delayed at compiler/rustc_borrowck/src/type_check/canonical.rs:150:13


error: internal compiler error: broken MIR in DefId(0:5 ~ riscv64_lp64_lp64f_lp64d_abi[45a3]::f_scalar_1) (NoSolution): could not prove Binder(TraitPredicate(<i8 as Copy>, polarity:Positive), [])
  = note: delayed at compiler/rustc_borrowck/src/type_check/canonical.rs:150:13


error: internal compiler error: broken MIR in DefId(0:6 ~ riscv64_lp64_lp64f_lp64d_abi[45a3]::f_scalar_2) (NoSolution): could not prove Binder(TraitPredicate(<u8 as Copy>, polarity:Positive), [])
  = note: delayed at compiler/rustc_borrowck/src/type_check/canonical.rs:150:13


error: internal compiler error: broken MIR in DefId(0:7 ~ riscv64_lp64_lp64f_lp64d_abi[45a3]::f_scalar_3) (NoSolution): could not prove Binder(TraitPredicate(<i32 as Copy>, polarity:Positive), [])
  = note: delayed at compiler/rustc_borrowck/src/type_check/canonical.rs:150:13


error: internal compiler error: broken MIR in DefId(0:8 ~ riscv64_lp64_lp64f_lp64d_abi[45a3]::f_scalar_4) (NoSolution): could not prove Binder(TraitPredicate(<i64 as Copy>, polarity:Positive), [])
  = note: delayed at compiler/rustc_borrowck/src/type_check/canonical.rs:150:13


error: internal compiler error: broken MIR in DefId(0:9 ~ riscv64_lp64_lp64f_lp64d_abi[45a3]::f_fp_scalar_1) (NoSolution): could not prove Binder(TraitPredicate(<f32 as Copy>, polarity:Positive), [])
  = note: delayed at compiler/rustc_borrowck/src/type_check/canonical.rs:150:13


error: internal compiler error: broken MIR in DefId(0:10 ~ riscv64_lp64_lp64f_lp64d_abi[45a3]::f_fp_scalar_2) (NoSolution): could not prove Binder(TraitPredicate(<f64 as Copy>, polarity:Positive), [])
  = note: delayed at compiler/rustc_borrowck/src/type_check/canonical.rs:150:13


error: internal compiler error: broken MIR in DefId(0:36 ~ riscv64_lp64_lp64f_lp64d_abi[45a3]::f_scalar_stack_2) (NoSolution): could not prove Binder(TraitPredicate(<u64 as Copy>, polarity:Positive), [])
  = note: delayed at compiler/rustc_borrowck/src/type_check/canonical.rs:150:13


error: internal compiler error: broken MIR in DefId(0:36 ~ riscv64_lp64_lp64f_lp64d_abi[45a3]::f_scalar_stack_2) (NoSolution): could not prove Binder(TraitPredicate(<u8 as Copy>, polarity:Positive), [])
  = note: delayed at compiler/rustc_borrowck/src/type_check/canonical.rs:150:13


error: internal compiler error: broken MIR in DefId(0:36 ~ riscv64_lp64_lp64f_lp64d_abi[45a3]::f_scalar_stack_2) (NoSolution): could not prove Binder(TraitPredicate(<i8 as Copy>, polarity:Positive), [])
  = note: delayed at compiler/rustc_borrowck/src/type_check/canonical.rs:150:13

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1266:13
stack backtrace:
stack backtrace:
   0:     0x7ff5715405dc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h990bab3a6288f88d
   1:     0x7ff5715a7a0f - core::fmt::write::hfe542457c3d485e7
   2:     0x7ff57152fb41 - std::io::Write::write_fmt::h84baa49a671845ff
   3:     0x7ff5715403fb - std::sys_common::backtrace::print::h820abc9b5ee2eabf
   4:     0x7ff571543b34 - std::panicking::default_hook::{{closure}}::h332b0a151eb60b97
   5:     0x7ff5715436ed - std::panicking::default_hook::h3c3576a310096de7
   6:     0x7ff571fba9f1 - rustc_driver[ae7df348ce3b2059]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7ff57154421a - std::panicking::rust_panic_with_hook::h4533954bffc367f2
   8:     0x7ff5748186e3 - std[313531837bf14f86]::panicking::begin_panic::<rustc_errors[3a3ab70e371313b9]::ExplicitBug>::{closure#0}
   9:     0x7ff574818516 - std[313531837bf14f86]::sys_common::backtrace::__rust_end_short_backtrace::<std[313531837bf14f86]::panicking::begin_panic<rustc_errors[3a3ab70e371313b9]::ExplicitBug>::{closure#0}, !>
  10:     0x7ff571f79b4f - std[313531837bf14f86]::panicking::begin_panic::<rustc_errors[3a3ab70e371313b9]::ExplicitBug>
  11:     0x7ff57484a516 - std[313531837bf14f86]::panic::panic_any::<rustc_errors[3a3ab70e371313b9]::ExplicitBug>
  12:     0x7ff57484f516 - <rustc_errors[3a3ab70e371313b9]::HandlerInner as core[d182f6af4a929a07]::ops::drop::Drop>::drop
  13:     0x7ff571fd2692 - core[d182f6af4a929a07]::ptr::drop_in_place::<rustc_session[ea6f041d0b15ef67]::parse::ParseSess>
  14:     0x7ff571fd7c8a - <alloc[535114e78dca4296]::rc::Rc<rustc_session[ea6f041d0b15ef67]::session::Session> as core[d182f6af4a929a07]::ops::drop::Drop>::drop
  15:     0x7ff57202914c - core[d182f6af4a929a07]::ptr::drop_in_place::<rustc_interface[471c1409757ccb52]::interface::Compiler>
  16:     0x7ff572019799 - rustc_interface[471c1409757ccb52]::interface::create_compiler_and_run::<core[d182f6af4a929a07]::result::Result<(), rustc_errors[3a3ab70e371313b9]::ErrorGuaranteed>, rustc_driver[ae7df348ce3b2059]::run_compiler::{closure#1}>
  17:     0x7ff571fdffac - std[313531837bf14f86]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[471c1409757ccb52]::util::run_in_thread_pool_with_globals<rustc_interface[471c1409757ccb52]::interface::run_compiler<core[d182f6af4a929a07]::result::Result<(), rustc_errors[3a3ab70e371313b9]::ErrorGuaranteed>, rustc_driver[ae7df348ce3b2059]::run_compiler::{closure#1}>::{closure#0}, core[d182f6af4a929a07]::result::Result<(), rustc_errors[3a3ab70e371313b9]::ErrorGuaranteed>>::{closure#0}, core[d182f6af4a929a07]::result::Result<(), rustc_errors[3a3ab70e371313b9]::ErrorGuaranteed>>
  18:     0x7ff572034621 - std[313531837bf14f86]::panic::catch_unwind::<core[d182f6af4a929a07]::panic::unwind_safe::AssertUnwindSafe<<std[313531837bf14f86]::thread::Builder>::spawn_unchecked_<rustc_interface[471c1409757ccb52]::util::run_in_thread_pool_with_globals<rustc_interface[471c1409757ccb52]::interface::run_compiler<core[d182f6af4a929a07]::result::Result<(), rustc_errors[3a3ab70e371313b9]::ErrorGuaranteed>, rustc_driver[ae7df348ce3b2059]::run_compiler::{closure#1}>::{closure#0}, core[d182f6af4a929a07]::result::Result<(), rustc_errors[3a3ab70e371313b9]::ErrorGuaranteed>>::{closure#0}, core[d182f6af4a929a07]::result::Result<(), rustc_errors[3a3ab70e371313b9]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[d182f6af4a929a07]::result::Result<(), rustc_errors[3a3ab70e371313b9]::ErrorGuaranteed>>
  19:     0x7ff571fe1d02 - <<std[313531837bf14f86]::thread::Builder>::spawn_unchecked_<rustc_interface[471c1409757ccb52]::util::run_in_thread_pool_with_globals<rustc_interface[471c1409757ccb52]::interface::run_compiler<core[d182f6af4a929a07]::result::Result<(), rustc_errors[3a3ab70e371313b9]::ErrorGuaranteed>, rustc_driver[ae7df348ce3b2059]::run_compiler::{closure#1}>::{closure#0}, core[d182f6af4a929a07]::result::Result<(), rustc_errors[3a3ab70e371313b9]::ErrorGuaranteed>>::{closure#0}, core[d182f6af4a929a07]::result::Result<(), rustc_errors[3a3ab70e371313b9]::ErrorGuaranteed>>::{closure#1} as core[d182f6af4a929a07]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  20:     0x7ff57154fa03 - std::sys::unix::thread::Thread::new::thread_start::h5a241d8b8c703a44
  21:     0x7ff56b8bc609 - start_thread
  22:     0x7ff5713b3293 - clone
  23:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.61.0-nightly (bcc896214 2022-03-06) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C prefer-dynamic -C rpath -C debuginfo=0 -C no-prepopulate-passes
query stack during panic:
end of query stack
------------------------------------------

