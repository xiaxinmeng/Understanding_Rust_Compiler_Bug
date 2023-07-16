plain
........................................................................................ 4576/13487
........................................................................................ 4664/13487
........................................................................................ 4752/13487
........................................................................................ 4840/13487
.........F.....FF....................................................................... 4928/13487
........................................................................................ 5104/13487
........................................................................................ 5192/13487
............i....................................................................i...... 5280/13487
........................................................................................ 5368/13487
---
....iii................................................................................. 13464/13487
.......................
failures:

---- [ui] src/test/ui/impl-trait/in-ctfe/match-arm-exhaustive.rs stdout ----

1 error: constant pattern depends on a generic parameter
-   --> $DIR/match-arm-exhaustive.rs:19:9
+   --> $DIR/match-arm-exhaustive.rs:22:9
+   --> $DIR/match-arm-exhaustive.rs:22:9
3    |
4 LL |         CT => (),

6 
7 error: constant pattern depends on a generic parameter
-   --> $DIR/match-arm-exhaustive.rs:19:9
-   --> $DIR/match-arm-exhaustive.rs:19:9
+   --> $DIR/match-arm-exhaustive.rs:22:9
9    |
10 LL |         CT => (),


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/in-ctfe/match-arm-exhaustive/match-arm-exhaustive.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args impl-trait/in-ctfe/match-arm-exhaustive.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/in-ctfe/match-arm-exhaustive.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/in-ctfe/match-arm-exhaustive" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/in-ctfe/match-arm-exhaustive/auxiliary"
stdout: none
--- stderr -------------------------------
error: constant pattern depends on a generic parameter
  --> /checkout/src/test/ui/impl-trait/in-ctfe/match-arm-exhaustive.rs:22:9
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
LL |         CT => (),

error: constant pattern depends on a generic parameter
error: constant pattern depends on a generic parameter
  --> /checkout/src/test/ui/impl-trait/in-ctfe/match-arm-exhaustive.rs:22:9
   |
LL |         CT => (),

error: aborting due to 2 previous errors
------------------------------------------



---- [ui] src/test/ui/impl-trait/in-ctfe/array-len.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/in-ctfe/array-len.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/in-ctfe/array-len" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/in-ctfe/array-len/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: no errors encountered even though `delay_span_bug` issued
error: internal compiler error: Missing value for constant, but no error reported?
  --> /checkout/src/test/ui/impl-trait/in-ctfe/array-len.rs:21:19
   |
   |
LL |     let x = [0u8; output(yeet())];
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:243:43


error: internal compiler error: `InferCtxt` incorrectly tainted by errors
   = note: delayed at compiler/rustc_infer/src/infer/mod.rs:1265:27

error: internal compiler error: `ErrorGuaranteed` without an error
  --> /checkout/src/test/ui/impl-trait/in-ctfe/array-len.rs:21:19
  --> /checkout/src/test/ui/impl-trait/in-ctfe/array-len.rs:21:19
   |
LL |     let x = [0u8; output(yeet())];
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1070:31


error: internal compiler error: expected fullfillment errors
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:210:23

error: internal compiler error: Missing value for constant, but no error reported?
  --> /checkout/src/test/ui/impl-trait/in-ctfe/array-len.rs:22:22
  --> /checkout/src/test/ui/impl-trait/in-ctfe/array-len.rs:22:22
   |
LL |     println!("{:?}", x);
   |                      ^
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:243:43
   = note: this error: internal compiler error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)

error: internal compiler error: `InferCtxt` incorrectly tainted by errors
   = note: delayed at compiler/rustc_infer/src/infer/mod.rs:1265:27

error: internal compiler error: `ErrorGuaranteed` without an error
  --> /checkout/src/test/ui/impl-trait/in-ctfe/array-len.rs:22:22
  --> /checkout/src/test/ui/impl-trait/in-ctfe/array-len.rs:22:22
   |
LL |     println!("{:?}", x);
   |                      ^
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1070:31
   = note: this error: internal compiler error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)

error: internal compiler error: expected fullfillment errors
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:210:23

error: internal compiler error: TyKind::Error constructed but no error reported
   |
   |
   = note: delayed at compiler/rustc_mir_build/src/build/mod.rs:759:18

error: internal compiler error: PromoteTemps: MIR had errors
  --> /checkout/src/test/ui/impl-trait/in-ctfe/array-len.rs:20:1
LL | fn main() {
   | ^^^^^^^^^
   |
   = note: delayed at compiler/rustc_const_eval/src/transform/promote_consts.rs:52:22
   = note: delayed at compiler/rustc_const_eval/src/transform/promote_consts.rs:52:22

error: internal compiler error: `InferCtxt` incorrectly tainted by errors
   = note: delayed at compiler/rustc_infer/src/infer/mod.rs:1265:27


error: internal compiler error: broken MIR in DefId(0:11 ~ array_len[fba9]::main) ("return type"): bad type [type error]
  --> /checkout/src/test/ui/impl-trait/in-ctfe/array-len.rs:20:1
LL | fn main() {
   | ^^^^^^^^^
   |
   = note: delayed at compiler/rustc_borrowck/src/type_check/mod.rs:510:13
   = note: delayed at compiler/rustc_borrowck/src/type_check/mod.rs:510:13

error: internal compiler error: TyKind::Error constructed but no error reported
   |
   = note: delayed at compiler/rustc_borrowck/src/type_check/mod.rs:765:20

error: internal compiler error: broken MIR in DefId(0:11 ~ array_len[fba9]::main) (LocalDecl { mutability: Mut, local_info: None, internal: false, is_block_tail: None, ty: [type error], user_ty: None, source_info: SourceInfo { span: /checkout/src/test/ui/impl-trait/in-ctfe/array-len.rs:20:1: 20:10 (#0), scope: scope[0] } }): bad type [type error]
  --> /checkout/src/test/ui/impl-trait/in-ctfe/array-len.rs:20:1
LL | fn main() {
   | ^^^^^^^^^
   |
   = note: delayed at compiler/rustc_borrowck/src/type_check/mod.rs:510:13
---
   = note: delayed at /checkout/compiler/rustc_middle/src/ty/relate.rs:419:59

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1499:13
stack backtrace:
   0:     0x7f0221a2011c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h6154f49ec1b68f6b
   1:     0x7f0221a88cb8 - core::fmt::write::hd9d0343f2cbedb92
   2:     0x7f0221a10951 - std::io::Write::write_fmt::ha549726333f8cdaa
   3:     0x7f0221a230de - std::panicking::default_hook::{{closure}}::h78e4bb572adc4835
   4:     0x7f0221a22da7 - std::panicking::default_hook::h5295d9fa0bc037a5
   5:     0x7f02223dee54 - rustc_driver[7035da461767e574]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f0221a23891 - std::panicking::rust_panic_with_hook::hf7adfa238344656d
   7:     0x7f0225195be3 - std[f51aded392cf5807]::panicking::begin_panic::<rustc_errors[1dc65aa609ac1cb7]::ExplicitBug>::{closure#0}
   8:     0x7f02251950e6 - std[f51aded392cf5807]::sys_common::backtrace::__rust_end_short_backtrace::<std[f51aded392cf5807]::panicking::begin_panic<rustc_errors[1dc65aa609ac1cb7]::ExplicitBug>::{closure#0}, !>
   9:     0x7f02223a15e6 - std[f51aded392cf5807]::panicking::begin_panic::<rustc_errors[1dc65aa609ac1cb7]::ExplicitBug>
  10:     0x7f022519ae46 - std[f51aded392cf5807]::panic::panic_any::<rustc_errors[1dc65aa609ac1cb7]::ExplicitBug>
  11:     0x7f022519f3e0 - <rustc_errors[1dc65aa609ac1cb7]::HandlerInner as core[a0f03c9e75f2c779]::ops::drop::Drop>::drop
  12:     0x7f0222446d32 - core[a0f03c9e75f2c779]::ptr::drop_in_place::<rustc_session[d72b8f68b509f015]::parse::ParseSess>
  13:     0x7f022244c918 - <alloc[71ea6792478043c]::rc::Rc<rustc_session[d72b8f68b509f015]::session::Session> as core[a0f03c9e75f2c779]::ops::drop::Drop>::drop
  14:     0x7f02223f3b2c - core[a0f03c9e75f2c779]::ptr::drop_in_place::<rustc_interface[6c04d4d1816d4b85]::interface::Compiler>
  15:     0x7f02223e3300 - rustc_interface[6c04d4d1816d4b85]::interface::create_compiler_and_run::<core[a0f03c9e75f2c779]::result::Result<(), rustc_errors[1dc65aa609ac1cb7]::ErrorGuaranteed>, rustc_driver[7035da461767e574]::run_compiler::{closure#1}>
  16:     0x7f02223ca3c2 - <scoped_tls[500cab81e62bc929]::ScopedKey<rustc_span[14cd8c142c96546d]::SessionGlobals>>::set::<rustc_interface[6c04d4d1816d4b85]::interface::run_compiler<core[a0f03c9e75f2c779]::result::Result<(), rustc_errors[1dc65aa609ac1cb7]::ErrorGuaranteed>, rustc_driver[7035da461767e574]::run_compiler::{closure#1}>::{closure#0}, core[a0f03c9e75f2c779]::result::Result<(), rustc_errors[1dc65aa609ac1cb7]::ErrorGuaranteed>>
  17:     0x7f02224482bf - std[f51aded392cf5807]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[6c04d4d1816d4b85]::util::run_in_thread_pool_with_globals<rustc_interface[6c04d4d1816d4b85]::interface::run_compiler<core[a0f03c9e75f2c779]::result::Result<(), rustc_errors[1dc65aa609ac1cb7]::ErrorGuaranteed>, rustc_driver[7035da461767e574]::run_compiler::{closure#1}>::{closure#0}, core[a0f03c9e75f2c779]::result::Result<(), rustc_errors[1dc65aa609ac1cb7]::ErrorGuaranteed>>::{closure#0}, core[a0f03c9e75f2c779]::result::Result<(), rustc_errors[1dc65aa609ac1cb7]::ErrorGuaranteed>>
  18:     0x7f02223f76ee - std[f51aded392cf5807]::panic::catch_unwind::<core[a0f03c9e75f2c779]::panic::unwind_safe::AssertUnwindSafe<<std[f51aded392cf5807]::thread::Builder>::spawn_unchecked_<rustc_interface[6c04d4d1816d4b85]::util::run_in_thread_pool_with_globals<rustc_interface[6c04d4d1816d4b85]::interface::run_compiler<core[a0f03c9e75f2c779]::result::Result<(), rustc_errors[1dc65aa609ac1cb7]::ErrorGuaranteed>, rustc_driver[7035da461767e574]::run_compiler::{closure#1}>::{closure#0}, core[a0f03c9e75f2c779]::result::Result<(), rustc_errors[1dc65aa609ac1cb7]::ErrorGuaranteed>>::{closure#0}, core[a0f03c9e75f2c779]::result::Result<(), rustc_errors[1dc65aa609ac1cb7]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[a0f03c9e75f2c779]::result::Result<(), rustc_errors[1dc65aa609ac1cb7]::ErrorGuaranteed>>
  19:     0x7f0222449ae0 - <<std[f51aded392cf5807]::thread::Builder>::spawn_unchecked_<rustc_interface[6c04d4d1816d4b85]::util::run_in_thread_pool_with_globals<rustc_interface[6c04d4d1816d4b85]::interface::run_compiler<core[a0f03c9e75f2c779]::result::Result<(), rustc_errors[1dc65aa609ac1cb7]::ErrorGuaranteed>, rustc_driver[7035da461767e574]::run_compiler::{closure#1}>::{closure#0}, core[a0f03c9e75f2c779]::result::Result<(), rustc_errors[1dc65aa609ac1cb7]::ErrorGuaranteed>>::{closure#0}, core[a0f03c9e75f2c779]::result::Result<(), rustc_errors[1dc65aa609ac1cb7]::ErrorGuaranteed>>::{closure#1} as core[a0f03c9e75f2c779]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  20:     0x7f0221a30635 - std::sys::unix::thread::Thread::new::thread_start::hcc798bf10a89dcd5
  21:     0x7f02217cdb43 - <unknown>
  22:     0x7f022185fa00 - <unknown>
  23:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.65.0-nightly (e7ebcbc2e 2022-09-06) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
------------------------------------------



---- [ui] src/test/ui/impl-trait/in-ctfe/enum-discr.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/in-ctfe/enum-discr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/in-ctfe/enum-discr" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/in-ctfe/enum-discr/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: no errors encountered even though `delay_span_bug` issued
error: internal compiler error: Missing value for constant, but no error reported?
  --> /checkout/src/test/ui/impl-trait/in-ctfe/enum-discr.rs:22:11
   |
   |
LL |     Bar = output(yeet()),
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/const_evaluatable.rs:243:43


error: internal compiler error: `InferCtxt` incorrectly tainted by errors
   = note: delayed at compiler/rustc_infer/src/infer/mod.rs:1265:27

error: internal compiler error: `ErrorGuaranteed` without an error
  --> /checkout/src/test/ui/impl-trait/in-ctfe/enum-discr.rs:22:11
  --> /checkout/src/test/ui/impl-trait/in-ctfe/enum-discr.rs:22:11
   |
LL |     Bar = output(yeet()),
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:1070:31


error: internal compiler error: expected fullfillment errors
   = note: delayed at compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:210:23

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1499:13
stack backtrace:
stack backtrace:
   0:     0x7f60c4c3311c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h6154f49ec1b68f6b
   1:     0x7f60c4c9bcb8 - core::fmt::write::hd9d0343f2cbedb92
   2:     0x7f60c4c23951 - std::io::Write::write_fmt::ha549726333f8cdaa
   3:     0x7f60c4c360de - std::panicking::default_hook::{{closure}}::h78e4bb572adc4835
   4:     0x7f60c4c35da7 - std::panicking::default_hook::h5295d9fa0bc037a5
   5:     0x7f60c55f1e54 - rustc_driver[7035da461767e574]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f60c4c36891 - std::panicking::rust_panic_with_hook::hf7adfa238344656d
   7:     0x7f60c83a8be3 - std[f51aded392cf5807]::panicking::begin_panic::<rustc_errors[1dc65aa609ac1cb7]::ExplicitBug>::{closure#0}
   8:     0x7f60c83a80e6 - std[f51aded392cf5807]::sys_common::backtrace::__rust_end_short_backtrace::<std[f51aded392cf5807]::panicking::begin_panic<rustc_errors[1dc65aa609ac1cb7]::ExplicitBug>::{closure#0}, !>
   9:     0x7f60c55b45e6 - std[f51aded392cf5807]::panicking::begin_panic::<rustc_errors[1dc65aa609ac1cb7]::ExplicitBug>
  10:     0x7f60c83ade46 - std[f51aded392cf5807]::panic::panic_any::<rustc_errors[1dc65aa609ac1cb7]::ExplicitBug>
  11:     0x7f60c83b23e0 - <rustc_errors[1dc65aa609ac1cb7]::HandlerInner as core[a0f03c9e75f2c779]::ops::drop::Drop>::drop
  12:     0x7f60c5659d32 - core[a0f03c9e75f2c779]::ptr::drop_in_place::<rustc_session[d72b8f68b509f015]::parse::ParseSess>
  13:     0x7f60c565f918 - <alloc[71ea6792478043c]::rc::Rc<rustc_session[d72b8f68b509f015]::session::Session> as core[a0f03c9e75f2c779]::ops::drop::Drop>::drop
  14:     0x7f60c5606b2c - core[a0f03c9e75f2c779]::ptr::drop_in_place::<rustc_interface[6c04d4d1816d4b85]::interface::Compiler>
  15:     0x7f60c55f6300 - rustc_interface[6c04d4d1816d4b85]::interface::create_compiler_and_run::<core[a0f03c9e75f2c779]::result::Result<(), rustc_errors[1dc65aa609ac1cb7]::ErrorGuaranteed>, rustc_driver[7035da461767e574]::run_compiler::{closure#1}>
  16:     0x7f60c55dd3c2 - <scoped_tls[500cab81e62bc929]::ScopedKey<rustc_span[14cd8c142c96546d]::SessionGlobals>>::set::<rustc_interface[6c04d4d1816d4b85]::interface::run_compiler<core[a0f03c9e75f2c779]::result::Result<(), rustc_errors[1dc65aa609ac1cb7]::ErrorGuaranteed>, rustc_driver[7035da461767e574]::run_compiler::{closure#1}>::{closure#0}, core[a0f03c9e75f2c779]::result::Result<(), rustc_errors[1dc65aa609ac1cb7]::ErrorGuaranteed>>
  17:     0x7f60c565b2bf - std[f51aded392cf5807]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[6c04d4d1816d4b85]::util::run_in_thread_pool_with_globals<rustc_interface[6c04d4d1816d4b85]::interface::run_compiler<core[a0f03c9e75f2c779]::result::Result<(), rustc_errors[1dc65aa609ac1cb7]::ErrorGuaranteed>, rustc_driver[7035da461767e574]::run_compiler::{closure#1}>::{closure#0}, core[a0f03c9e75f2c779]::result::Result<(), rustc_errors[1dc65aa609ac1cb7]::ErrorGuaranteed>>::{closure#0}, core[a0f03c9e75f2c779]::result::Result<(), rustc_errors[1dc65aa609ac1cb7]::ErrorGuaranteed>>
  18:     0x7f60c560a6ee - std[f51aded392cf5807]::panic::catch_unwind::<core[a0f03c9e75f2c779]::panic::unwind_safe::AssertUnwindSafe<<std[f51aded392cf5807]::thread::Builder>::spawn_unchecked_<rustc_interface[6c04d4d1816d4b85]::util::run_in_thread_pool_with_globals<rustc_interface[6c04d4d1816d4b85]::interface::run_compiler<core[a0f03c9e75f2c779]::result::Result<(), rustc_errors[1dc65aa609ac1cb7]::ErrorGuaranteed>, rustc_driver[7035da461767e574]::run_compiler::{closure#1}>::{closure#0}, core[a0f03c9e75f2c779]::result::Result<(), rustc_errors[1dc65aa609ac1cb7]::ErrorGuaranteed>>::{closure#0}, core[a0f03c9e75f2c779]::result::Result<(), rustc_errors[1dc65aa609ac1cb7]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[a0f03c9e75f2c779]::result::Result<(), rustc_errors[1dc65aa609ac1cb7]::ErrorGuaranteed>>
  19:     0x7f60c565cae0 - <<std[f51aded392cf5807]::thread::Builder>::spawn_unchecked_<rustc_interface[6c04d4d1816d4b85]::util::run_in_thread_pool_with_globals<rustc_interface[6c04d4d1816d4b85]::interface::run_compiler<core[a0f03c9e75f2c779]::result::Result<(), rustc_errors[1dc65aa609ac1cb7]::ErrorGuaranteed>, rustc_driver[7035da461767e574]::run_compiler::{closure#1}>::{closure#0}, core[a0f03c9e75f2c779]::result::Result<(), rustc_errors[1dc65aa609ac1cb7]::ErrorGuaranteed>>::{closure#0}, core[a0f03c9e75f2c779]::result::Result<(), rustc_errors[1dc65aa609ac1cb7]::ErrorGuaranteed>>::{closure#1} as core[a0f03c9e75f2c779]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  20:     0x7f60c4c43635 - std::sys::unix::thread::Thread::new::thread_start::hcc798bf10a89dcd5
  21:     0x7f60c49e0b43 - <unknown>
  22:     0x7f60c4a72a00 - <unknown>
  23:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.65.0-nightly (e7ebcbc2e 2022-09-06) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
------------------------------------------




failures:
    [ui] src/test/ui/impl-trait/in-ctfe/array-len.rs
    [ui] src/test/ui/impl-trait/in-ctfe/enum-discr.rs
    [ui] src/test/ui/impl-trait/in-ctfe/match-arm-exhaustive.rs
test result: FAILED. 13366 passed; 3 failed; 118 ignored; 0 measured; 0 filtered out; finished in 130.23s

Build completed unsuccessfully in 0:12:57
