plain

running 328 tests
ii...i........i....i..i.................iii.......iii.......i.................ii.................i.. 100/328
............i...............i...iii........i..i..............i.......i..............i...i...i.....ii 200/328
..i.ii.............iiii.F......................i.ii.i......i.......iii..........i................... 300/328
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
85 | pub extern "C" fn f_agg_tiny(mut e: Tiny) {
   |                                  ^ help: if this is intentional, prefix it with an underscore: `_e`
   = note: `#[warn(unused_variables)]` on by default

warning: unused variable: `x`
   --> /checkout/src/test/codegen/riscv-abi/riscv64-lp64-lp64f-lp64d-abi.rs:102:35
   --> /checkout/src/test/codegen/riscv-abi/riscv64-lp64-lp64f-lp64d-abi.rs:102:35
    |
102 | pub extern "C" fn f_agg_small(mut x: Small) {
    |                                   ^ help: if this is intentional, prefix it with an underscore: `_x`
warning: unused variable: `x`
   --> /checkout/src/test/codegen/riscv-abi/riscv64-lp64-lp64f-lp64d-abi.rs:118:43
    |
    |
118 | pub extern "C" fn f_agg_small_aligned(mut x: SmallAligned) {
    |                                           ^ help: if this is intentional, prefix it with an underscore: `_x`
warning: unused variable: `x`
   --> /checkout/src/test/codegen/riscv-abi/riscv64-lp64-lp64f-lp64d-abi.rs:131:35
    |
    |
131 | pub extern "C" fn f_agg_large(mut x: Large) {
    |                                   ^ help: if this is intentional, prefix it with an underscore: `_x`
warning: unused variable: `i`
   --> /checkout/src/test/codegen/riscv-abi/riscv64-lp64-lp64f-lp64d-abi.rs:136:35
    |
    |
136 | pub extern "C" fn f_agg_large_ret(i: i32, j: i8) -> Large {
    |                                   ^ help: if this is intentional, prefix it with an underscore: `_i`
warning: unused variable: `j`
   --> /checkout/src/test/codegen/riscv-abi/riscv64-lp64-lp64f-lp64d-abi.rs:136:43
    |
    |
136 | pub extern "C" fn f_agg_large_ret(i: i32, j: i8) -> Large {
    |                                           ^ help: if this is intentional, prefix it with an underscore: `_j`
warning: unused variable: `a`
   --> /checkout/src/test/codegen/riscv-abi/riscv64-lp64-lp64f-lp64d-abi.rs:143:5
    |
    |
143 |     a: Tiny,
    |     ^ help: if this is intentional, prefix it with an underscore: `_a`
warning: unused variable: `b`
   --> /checkout/src/test/codegen/riscv-abi/riscv64-lp64-lp64f-lp64d-abi.rs:144:5
    |
    |
144 |     b: Small,
    |     ^ help: if this is intentional, prefix it with an underscore: `_b`
warning: unused variable: `c`
   --> /checkout/src/test/codegen/riscv-abi/riscv64-lp64-lp64f-lp64d-abi.rs:145:5
    |
    |
145 |     c: SmallAligned,
    |     ^ help: if this is intentional, prefix it with an underscore: `_c`
warning: unused variable: `d`
   --> /checkout/src/test/codegen/riscv-abi/riscv64-lp64-lp64f-lp64d-abi.rs:146:5
    |
146 |     d: Large,
---

warning: unused variable: `b`
   --> /checkout/src/test/codegen/riscv-abi/riscv64-lp64-lp64f-lp64d-abi.rs:158:5
    |
158 |     b: SmallAligned,
    |     ^ help: if this is intentional, prefix it with an underscore: `_b`
warning: unused variable: `c`
   --> /checkout/src/test/codegen/riscv-abi/riscv64-lp64-lp64f-lp64d-abi.rs:159:5
    |
    |
159 |     c: SmallAligned,
    |     ^ help: if this is intentional, prefix it with an underscore: `_c`
warning: unused variable: `d`
   --> /checkout/src/test/codegen/riscv-abi/riscv64-lp64-lp64f-lp64d-abi.rs:160:5
    |
160 |     d: u64,
160 |     d: u64,
    |     ^ help: if this is intentional, prefix it with an underscore: `_d`

warning: variable does not need to be mutable
  --> /checkout/src/test/codegen/riscv-abi/riscv64-lp64-lp64f-lp64d-abi.rs:85:30
   |
85 | pub extern "C" fn f_agg_tiny(mut e: Tiny) {
   |                              |
   |                              help: remove this `mut`
   |
   = note: `#[warn(unused_mut)]` on by default
   = note: `#[warn(unused_mut)]` on by default

warning: variable does not need to be mutable
   --> /checkout/src/test/codegen/riscv-abi/riscv64-lp64-lp64f-lp64d-abi.rs:102:31
    |
102 | pub extern "C" fn f_agg_small(mut x: Small) {
    |                               |
    |                               help: remove this `mut`

warning: variable does not need to be mutable
warning: variable does not need to be mutable
   --> /checkout/src/test/codegen/riscv-abi/riscv64-lp64-lp64f-lp64d-abi.rs:118:39
    |
118 | pub extern "C" fn f_agg_small_aligned(mut x: SmallAligned) {
    |                                       |
    |                                       help: remove this `mut`

warning: variable does not need to be mutable
warning: variable does not need to be mutable
   --> /checkout/src/test/codegen/riscv-abi/riscv64-lp64-lp64f-lp64d-abi.rs:131:31
    |
131 | pub extern "C" fn f_agg_large(mut x: Large) {
    |                               |
    |                               help: remove this `mut`


warning: `extern` fn uses type `Empty`, which is not FFI-safe
   |
   |
71 | pub extern "C" fn f_agg_empty_struct(e: Empty) -> Empty {
   |                                         ^^^^^ not FFI-safe
   = note: `#[warn(improper_ctypes_definitions)]` on by default
   = help: consider adding a member to this struct
   = note: this struct has no fields
note: the type is defined here
note: the type is defined here
  --> /checkout/src/test/codegen/riscv-abi/riscv64-lp64-lp64f-lp64d-abi.rs:67:1
   |
67 | pub struct Empty {}
   | ^^^^^^^^^^^^^^^^^^^

warning: `extern` fn uses type `Empty`, which is not FFI-safe
   |
   |
71 | pub extern "C" fn f_agg_empty_struct(e: Empty) -> Empty {
   |                                                   ^^^^^ not FFI-safe
   = help: consider adding a member to this struct
   = note: this struct has no fields
note: the type is defined here
  --> /checkout/src/test/codegen/riscv-abi/riscv64-lp64-lp64f-lp64d-abi.rs:67:1
  --> /checkout/src/test/codegen/riscv-abi/riscv64-lp64-lp64f-lp64d-abi.rs:67:1
   |
67 | pub struct Empty {}
   | ^^^^^^^^^^^^^^^^^^^

warning: `extern` fn uses type `i128`, which is not FFI-safe
    |
    |
118 | pub extern "C" fn f_agg_small_aligned(mut x: SmallAligned) {
    |                                              ^^^^^^^^^^^^ not FFI-safe
    |
    = note: 128-bit integers don't currently have a known stable ABI

warning: `extern` fn uses type `i128`, which is not FFI-safe
    |
    |
145 |     c: SmallAligned,
    |        ^^^^^^^^^^^^ not FFI-safe
    |
    = note: 128-bit integers don't currently have a known stable ABI

warning: `extern` fn uses type `i128`, which is not FFI-safe
    |
    |
158 |     b: SmallAligned,
    |        ^^^^^^^^^^^^ not FFI-safe
    |
    = note: 128-bit integers don't currently have a known stable ABI

warning: `extern` fn uses type `i128`, which is not FFI-safe
    |
    |
159 |     c: SmallAligned,
    |        ^^^^^^^^^^^^ not FFI-safe
    |
    = note: 128-bit integers don't currently have a known stable ABI
warning: 27 warnings emitted


error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: broken MIR in DefId(0:43 ~ riscv64_lp64_lp64f_lp64d_abi[45a3]::f_scalar_stack_2) (NoSolution): could not prove Binder(TraitPredicate(<u64 as Copy>, polarity:Positive), [])
  = note: delayed at compiler/rustc_borrowck/src/type_check/canonical.rs:150:13

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1266:13
stack backtrace:
stack backtrace:
   0:     0x7f1235b9a5dc - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h37203cb1f5ff9a8a
   1:     0x7f1235c01a0f - core::fmt::write::h25577dfccc9130ff
   2:     0x7f1235b89aa1 - std::io::Write::write_fmt::h59cb695e41729b39
   3:     0x7f1235b9a3fb - std::sys_common::backtrace::print::hf16b3e3b9ab747e2
   4:     0x7f1235b9db34 - std::panicking::default_hook::{{closure}}::hd4bcca3767b3b513
   5:     0x7f1235b9d6ed - std::panicking::default_hook::h7889a6a7d55b6279
   6:     0x7f1236617fb1 - rustc_driver[974d80daca7e2b47]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f1235b9e21a - std::panicking::rust_panic_with_hook::hb69c20f9530f019d
   8:     0x7f1238e77203 - std[cf2297139d6b1d09]::panicking::begin_panic::<rustc_errors[2e19bf40437f7a13]::ExplicitBug>::{closure#0}
   9:     0x7f1238e76f56 - std[cf2297139d6b1d09]::sys_common::backtrace::__rust_end_short_backtrace::<std[cf2297139d6b1d09]::panicking::begin_panic<rustc_errors[2e19bf40437f7a13]::ExplicitBug>::{closure#0}, !>
  10:     0x7f12365d70ff - std[cf2297139d6b1d09]::panicking::begin_panic::<rustc_errors[2e19bf40437f7a13]::ExplicitBug>
  11:     0x7f1238ea9b26 - std[cf2297139d6b1d09]::panic::panic_any::<rustc_errors[2e19bf40437f7a13]::ExplicitBug>
  12:     0x7f1238eade86 - <rustc_errors[2e19bf40437f7a13]::HandlerInner as core[730fa126677d4fd4]::ops::drop::Drop>::drop
  13:     0x7f123662f252 - core[730fa126677d4fd4]::ptr::drop_in_place::<rustc_session[3120a712f73c8717]::parse::ParseSess>
  14:     0x7f1236634c3a - <alloc[a317faea6eaa5d2f]::rc::Rc<rustc_session[3120a712f73c8717]::session::Session> as core[730fa126677d4fd4]::ops::drop::Drop>::drop
  15:     0x7f12366874bc - core[730fa126677d4fd4]::ptr::drop_in_place::<rustc_interface[27406f0f5f5a453]::interface::Compiler>
  16:     0x7f123668ba69 - rustc_interface[27406f0f5f5a453]::interface::create_compiler_and_run::<core[730fa126677d4fd4]::result::Result<(), rustc_errors[2e19bf40437f7a13]::ErrorGuaranteed>, rustc_driver[974d80daca7e2b47]::run_compiler::{closure#1}>
  17:     0x7f123663d4fc - std[cf2297139d6b1d09]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[27406f0f5f5a453]::util::run_in_thread_pool_with_globals<rustc_interface[27406f0f5f5a453]::interface::run_compiler<core[730fa126677d4fd4]::result::Result<(), rustc_errors[2e19bf40437f7a13]::ErrorGuaranteed>, rustc_driver[974d80daca7e2b47]::run_compiler::{closure#1}>::{closure#0}, core[730fa126677d4fd4]::result::Result<(), rustc_errors[2e19bf40437f7a13]::ErrorGuaranteed>>::{closure#0}, core[730fa126677d4fd4]::result::Result<(), rustc_errors[2e19bf40437f7a13]::ErrorGuaranteed>>
  18:     0x7f1236694471 - std[cf2297139d6b1d09]::panic::catch_unwind::<core[730fa126677d4fd4]::panic::unwind_safe::AssertUnwindSafe<<std[cf2297139d6b1d09]::thread::Builder>::spawn_unchecked_<rustc_interface[27406f0f5f5a453]::util::run_in_thread_pool_with_globals<rustc_interface[27406f0f5f5a453]::interface::run_compiler<core[730fa126677d4fd4]::result::Result<(), rustc_errors[2e19bf40437f7a13]::ErrorGuaranteed>, rustc_driver[974d80daca7e2b47]::run_compiler::{closure#1}>::{closure#0}, core[730fa126677d4fd4]::result::Result<(), rustc_errors[2e19bf40437f7a13]::ErrorGuaranteed>>::{closure#0}, core[730fa126677d4fd4]::result::Result<(), rustc_errors[2e19bf40437f7a13]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[730fa126677d4fd4]::result::Result<(), rustc_errors[2e19bf40437f7a13]::ErrorGuaranteed>>
  19:     0x7f123663f252 - <<std[cf2297139d6b1d09]::thread::Builder>::spawn_unchecked_<rustc_interface[27406f0f5f5a453]::util::run_in_thread_pool_with_globals<rustc_interface[27406f0f5f5a453]::interface::run_compiler<core[730fa126677d4fd4]::result::Result<(), rustc_errors[2e19bf40437f7a13]::ErrorGuaranteed>, rustc_driver[974d80daca7e2b47]::run_compiler::{closure#1}>::{closure#0}, core[730fa126677d4fd4]::result::Result<(), rustc_errors[2e19bf40437f7a13]::ErrorGuaranteed>>::{closure#0}, core[730fa126677d4fd4]::result::Result<(), rustc_errors[2e19bf40437f7a13]::ErrorGuaranteed>>::{closure#1} as core[730fa126677d4fd4]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  20:     0x7f1235ba9a03 - std::sys::unix::thread::Thread::new::thread_start::h89e7415d1f5b6f5e
  21:     0x7f122ff16609 - start_thread
  22:     0x7f1235a0d293 - clone
  23:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.61.0-nightly (8295d3117 2022-03-06) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C prefer-dynamic -C rpath -C debuginfo=0 -C no-prepopulate-passes
query stack during panic:
end of query stack
------------------------------------------

