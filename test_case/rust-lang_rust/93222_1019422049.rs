plain
.................................................................................................... 1800/12549
....................i............................................................................... 1900/12549
..........................................................................F......................... 2000/12549
...........................................................................................i........ 2100/12549
.................................F.F................................................................ 2200/12549
.................................................................................................... 2400/12549
.................................................................................................... 2500/12549
.................................................................................................... 2600/12549
.................................................................................................... 2700/12549
---
........................ii.......................................................................... 8200/12549
.......iiii......................................................................................... 8300/12549
............................i.......................................i............................... 8400/12549
..............................i..................................................................... 8500/12549
......FF.FF.........................................................................i............... 8600/12549
...............................................................i.................................... 8800/12549
.................................................................................................... 8900/12549
.................................................................................................... 9000/12549
.................................................................................................... 9100/12549
---
---- [ui] ui/const-generics/generic_const_exprs/function-call.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/function-call.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/function-call" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/function-call/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: cannot use constants which depend on generic parameters in types
   |
   |
LL |     let _ = [0; foo::<T>()];
   |
   = note: `#[warn(const_evaluatable_unchecked)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #76200 <https://github.com/rust-lang/rust/issues/76200>
   = note: for more information, see issue #76200 <https://github.com/rust-lang/rust/issues/76200>

thread 'rustc' panicked at 'The emitted diagnostic is not an error.', compiler/rustc_errors/src/diagnostic_builder.rs:110:21

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
error: internal compiler error: unexpected panic


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.60.0-nightly (23e870b1b 2022-01-23) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `test`
#1 [typeck_item_bodies] type-checking all item bodies
warning: 1 warning emitted


------------------------------------------
------------------------------------------


---- [ui] ui/const-generics/min_const_generics/const-evaluatable-unchecked.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/min_const_generics/const-evaluatable-unchecked.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/min_const_generics/const-evaluatable-unchecked" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/min_const_generics/const-evaluatable-unchecked/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: cannot use constants which depend on generic parameters in types
   |
   |
LL |     [0; std::mem::size_of::<*mut T>()];
   |
   = note: `#[warn(const_evaluatable_unchecked)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #76200 <https://github.com/rust-lang/rust/issues/76200>
   = note: for more information, see issue #76200 <https://github.com/rust-lang/rust/issues/76200>

thread 'rustc' panicked at 'The emitted diagnostic is not an error.', compiler/rustc_errors/src/diagnostic_builder.rs:110:21

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.60.0-nightly (23e870b1b 2022-01-23) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `foo`
#1 [typeck_item_bodies] type-checking all item bodies
warning: 1 warning emitted


------------------------------------------
------------------------------------------


---- [ui] ui/const-generics/min_const_generics/complex-expression.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/min_const_generics/complex-expression.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/min_const_generics/complex-expression" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/min_const_generics/complex-expression/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: generic parameters may not be used in const operations
   |
   |
LL | struct Break0<const N: usize>([u8; { N + 1 }]);
   |                                      ^ cannot perform const operation using `N`
   |
   = help: const parameters may only be used as standalone arguments, i.e. `N`
   = help: use `#![feature(generic_const_exprs)]` to allow generic const expressions

error: generic parameters may not be used in const operations
   |
   |
LL | struct Break1<const N: usize>([u8; { { N } }]);
   |                                        ^ cannot perform const operation using `N`
   |
   = help: const parameters may only be used as standalone arguments, i.e. `N`
   = help: use `#![feature(generic_const_exprs)]` to allow generic const expressions

error: generic parameters may not be used in const operations
   |
   |
LL |     let _: [u8; N + 1];
   |                 ^ cannot perform const operation using `N`
   |
   = help: const parameters may only be used as standalone arguments, i.e. `N`
   = help: use `#![feature(generic_const_exprs)]` to allow generic const expressions

error: generic parameters may not be used in const operations
   |
   |
LL |     let _ = [0; N + 1];
   |                 ^ cannot perform const operation using `N`
   |
   = help: const parameters may only be used as standalone arguments, i.e. `N`
   = help: use `#![feature(generic_const_exprs)]` to allow generic const expressions

error: generic parameters may not be used in const operations
   |
   |
LL | struct BreakTy0<T>(T, [u8; { size_of::<*mut T>() }]);
   |                                             ^ cannot perform const operation using `T`
   = note: type parameters may not be used in const expressions
   = note: type parameters may not be used in const expressions
   = help: use `#![feature(generic_const_exprs)]` to allow generic const expressions

error: generic parameters may not be used in const operations
   |
   |
LL | struct BreakTy1<T>(T, [u8; { { size_of::<*mut T>() } }]);
   |                                               ^ cannot perform const operation using `T`
   = note: type parameters may not be used in const expressions
   = note: type parameters may not be used in const expressions
   = help: use `#![feature(generic_const_exprs)]` to allow generic const expressions

error: generic parameters may not be used in const operations
   |
   |
LL |     let _: [u8; size_of::<*mut T>() + 1];
   |                                ^ cannot perform const operation using `T`
   = note: type parameters may not be used in const expressions
   = note: type parameters may not be used in const expressions
   = help: use `#![feature(generic_const_exprs)]` to allow generic const expressions

warning: cannot use constants which depend on generic parameters in types
   |
   |
LL |     let _ = [0; size_of::<*mut T>() + 1];
   |
   = note: `#[warn(const_evaluatable_unchecked)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #76200 <https://github.com/rust-lang/rust/issues/76200>
   = note: for more information, see issue #76200 <https://github.com/rust-lang/rust/issues/76200>

thread 'rustc' panicked at 'The emitted diagnostic is not an error.', compiler/rustc_errors/src/diagnostic_builder.rs:110:21

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.60.0-nightly (23e870b1b 2022-01-23) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `break_ty3`
#1 [typeck_item_bodies] type-checking all item bodies
error: aborting due to 7 previous errors; 1 warning emitted


------------------------------------------
------------------------------------------


---- [ui] ui/parser/import-from-rename.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/import-from-rename.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/import-from-rename" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/import-from-rename/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: expected `;`, found keyword `as`
   |
   |
LL | use foo::{bar} as baz;
   |                ^^ expected `;`
   |
   = note: glob-like brace syntax must be last on the path

thread 'rustc' panicked at 'The emitted diagnostic is not an error.', compiler/rustc_errors/src/diagnostic_builder.rs:110:21

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.60.0-nightly (23e870b1b 2022-01-23) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
error: aborting due to previous error



------------------------------------------


---- [ui] ui/parser/import-from-path.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/import-from-path.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/import-from-path" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/import-from-path/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: expected `;`, found `::`
   |
   |
LL | use foo::{bar}::baz
   |               ^^ expected `;`
   |
   = note: glob-like brace syntax must be last on the path

thread 'rustc' panicked at 'The emitted diagnostic is not an error.', compiler/rustc_errors/src/diagnostic_builder.rs:110:21

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.60.0-nightly (23e870b1b 2022-01-23) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
error: aborting due to previous error



------------------------------------------


---- [ui] ui/parser/import-glob-path.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/import-glob-path.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/import-glob-path" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/import-glob-path/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: expected `;`, found `::`
   |
   |
LL | use foo::*::bar
   |           ^^ expected `;`
   = note: the wildcard token must be last on the path


thread 'rustc' panicked at 'The emitted diagnostic is not an error.', compiler/rustc_errors/src/diagnostic_builder.rs:110:21

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.60.0-nightly (23e870b1b 2022-01-23) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
error: aborting due to previous error



------------------------------------------


---- [ui] ui/parser/import-glob-rename.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/import-glob-rename.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/import-glob-rename" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/import-glob-rename/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: expected `;`, found keyword `as`
   |
   |
LL | use foo::* as baz;
   |            ^^ expected `;`
   = note: the wildcard token must be last on the path


thread 'rustc' panicked at 'The emitted diagnostic is not an error.', compiler/rustc_errors/src/diagnostic_builder.rs:110:21

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.60.0-nightly (23e870b1b 2022-01-23) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
error: aborting due to previous error



------------------------------------------


---- [ui] ui/span/issue-24356.rs stdout ----

error: Error: expected failure status (Some(1)) but received status None.
status: signal: 6 (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/span/issue-24356.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-24356" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-24356/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'The emitted diagnostic is not an error.', compiler/rustc_errors/src/diagnostic_builder.rs:110:21

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.60.0-nightly (23e870b1b 2022-01-23) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [check_mod_item_types] checking item types in top-level module
#1 [analysis] running analysis passes on this crate
error: internal compiler error: TyKind::Error constructed but no error reported
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/project.rs:1847:35


thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', compiler/rustc_errors/src/lib.rs:1202:13
stack backtrace:
   0:     0x7f4913436d40 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hded7bc92c082750b
   1:     0x7f49134a4f6f - core::fmt::write::h89d57f3b082fffc1
   2:     0x7f49134239d5 - std::io::Write::write_fmt::h30f8550be20d15e1
   3:     0x7f491343b477 - std::panicking::default_hook::{{closure}}::h72a89d0efc4cb01b
   4:     0x7f491343aea0 - std::panicking::default_hook::h82776b227d502232
   5:     0x7f4913eee931 - rustc_driver[b2f3a6712a8ae1f8]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f491343bc13 - std::panicking::rust_panic_with_hook::h27a3f4314efdb945
   7:     0x7f491343ba67 - std::panicking::begin_panic_handler::{{closure}}::h063b3a2fb3a14c51
   8:     0x7f4913437234 - std::sys_common::backtrace::__rust_end_short_backtrace::h8900256cd0493087
   9:     0x7f491343b759 - rust_begin_unwind
  10:     0x7f49133ee8a3 - core::panicking::panic_fmt::hf000a75cf7506f9f
  11:     0x7f49168b6288 - core[df2e118c3199f58c]::panicking::panic_display::<&str>
  12:     0x7f49168b301f - <rustc_errors[56bbb2df6e4694eb]::HandlerInner>::flush_delayed
  13:     0x7f49168afa26 - <rustc_errors[56bbb2df6e4694eb]::HandlerInner as core[df2e118c3199f58c]::ops::drop::Drop>::drop
  14:     0x7f4913ed6092 - core[df2e118c3199f58c]::ptr::drop_in_place::<rustc_session[bf61c6c135bf232f]::parse::ParseSess>
  15:     0x7f4913edadaa - <alloc[13b1b9db63e6bfc3]::rc::Rc<rustc_session[bf61c6c135bf232f]::session::Session> as core[df2e118c3199f58c]::ops::drop::Drop>::drop
  16:     0x7f4913edd97c - core[df2e118c3199f58c]::ptr::drop_in_place::<rustc_interface[8188ef922e39633e]::interface::Compiler>
  17:     0x7f4913ee229e - rustc_span[e3bc09a5f67cdc93]::with_source_map::<core[df2e118c3199f58c]::result::Result<(), rustc_errors[56bbb2df6e4694eb]::ErrorReported>, rustc_interface[8188ef922e39633e]::interface::create_compiler_and_run<core[df2e118c3199f58c]::result::Result<(), rustc_errors[56bbb2df6e4694eb]::ErrorReported>, rustc_driver[b2f3a6712a8ae1f8]::run_compiler::{closure#1}>::{closure#1}>
  18:     0x7f4913f51bad - rustc_interface[8188ef922e39633e]::interface::create_compiler_and_run::<core[df2e118c3199f58c]::result::Result<(), rustc_errors[56bbb2df6e4694eb]::ErrorReported>, rustc_driver[b2f3a6712a8ae1f8]::run_compiler::{closure#1}>
  19:     0x7f4913f001d4 - std[7b97bce5625b0034]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[8188ef922e39633e]::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface[8188ef922e39633e]::interface::run_compiler<core[df2e118c3199f58c]::result::Result<(), rustc_errors[56bbb2df6e4694eb]::ErrorReported>, rustc_driver[b2f3a6712a8ae1f8]::run_compiler::{closure#1}>::{closure#0}, core[df2e118c3199f58c]::result::Result<(), rustc_errors[56bbb2df6e4694eb]::ErrorReported>>::{closure#0}, core[df2e118c3199f58c]::result::Result<(), rustc_errors[56bbb2df6e4694eb]::ErrorReported>>
  20:     0x7f4913f56701 - std[7b97bce5625b0034]::panicking::try::<core[df2e118c3199f58c]::result::Result<(), rustc_errors[56bbb2df6e4694eb]::ErrorReported>, core[df2e118c3199f58c]::panic::unwind_safe::AssertUnwindSafe<<std[7b97bce5625b0034]::thread::Builder>::spawn_unchecked<rustc_interface[8188ef922e39633e]::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface[8188ef922e39633e]::interface::run_compiler<core[df2e118c3199f58c]::result::Result<(), rustc_errors[56bbb2df6e4694eb]::ErrorReported>, rustc_driver[b2f3a6712a8ae1f8]::run_compiler::{closure#1}>::{closure#0}, core[df2e118c3199f58c]::result::Result<(), rustc_errors[56bbb2df6e4694eb]::ErrorReported>>::{closure#0}, core[df2e118c3199f58c]::result::Result<(), rustc_errors[56bbb2df6e4694eb]::ErrorReported>>::{closure#1}::{closure#0}>>
  21:     0x7f4913ed752a - <<std[7b97bce5625b0034]::thread::Builder>::spawn_unchecked<rustc_interface[8188ef922e39633e]::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface[8188ef922e39633e]::interface::run_compiler<core[df2e118c3199f58c]::result::Result<(), rustc_errors[56bbb2df6e4694eb]::ErrorReported>, rustc_driver[b2f3a6712a8ae1f8]::run_compiler::{closure#1}>::{closure#0}, core[df2e118c3199f58c]::result::Result<(), rustc_errors[56bbb2df6e4694eb]::ErrorReported>>::{closure#0}, core[df2e118c3199f58c]::result::Result<(), rustc_errors[56bbb2df6e4694eb]::ErrorReported>>::{closure#1} as core[df2e118c3199f58c]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  22:     0x7f4913449b03 - std::sys::unix::thread::Thread::new::thread_start::ha19e2e727e9671eb
  23:     0x7f490d7b9609 - start_thread
  24:     0x7f49132b0293 - clone
  25:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.60.0-nightly (23e870b1b 2022-01-23) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
thread panicked while panicking. aborting.

---
test result: FAILED. 12420 passed; 8 failed; 121 ignored; 0 measured; 0 filtered out; finished in 123.39s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:02
