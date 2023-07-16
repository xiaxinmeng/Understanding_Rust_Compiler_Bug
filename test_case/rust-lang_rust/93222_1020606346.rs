plain
...........................ii....................................................................... 8200/12553
..........iiii...................................................................................... 8300/12553
...............................i......................................i............................. 8400/12553
.................................i.................................................................. 8500/12553
.........FFF..F.........................................................................i........... 8600/12553
.................................................................i.................................. 8800/12553
.................................................................................................... 8900/12553
.................................................................................................... 9000/12553
.................................................................................................... 9100/12553
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
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   |
   = note: `#[warn(const_evaluatable_unchecked)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #76200 <https://github.com/rust-lang/rust/issues/76200>

thread 'rustc' panicked at 'The emitted diagnostic is not an error.', compiler/rustc_errors/src/diagnostic_builder.rs:110:21

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.60.0-nightly (411edcba8 2022-01-24) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `test`
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

note: rustc 1.60.0-nightly (411edcba8 2022-01-24) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `break_ty3`
#1 [typeck_item_bodies] type-checking all item bodies
error: aborting due to 7 previous errors; 1 warning emitted


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

note: rustc 1.60.0-nightly (411edcba8 2022-01-24) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [typeck] type-checking `foo`
#1 [typeck_item_bodies] type-checking all item bodies
warning: 1 warning emitted


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

note: rustc 1.60.0-nightly (411edcba8 2022-01-24) running on x86_64-unknown-linux-gnu

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

note: rustc 1.60.0-nightly (411edcba8 2022-01-24) running on x86_64-unknown-linux-gnu

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

note: rustc 1.60.0-nightly (411edcba8 2022-01-24) running on x86_64-unknown-linux-gnu

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

note: rustc 1.60.0-nightly (411edcba8 2022-01-24) running on x86_64-unknown-linux-gnu

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

note: rustc 1.60.0-nightly (411edcba8 2022-01-24) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [check_mod_item_types] checking item types in top-level module
#1 [analysis] running analysis passes on this crate
error: internal compiler error: TyKind::Error constructed but no error reported
   |
   = note: delayed at compiler/rustc_trait_selection/src/traits/project.rs:1847:35


thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', compiler/rustc_errors/src/lib.rs:1190:13
stack backtrace:
   0:     0x7f7116642e50 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h92236fb36ac4202e
   1:     0x7f71166b107f - core::fmt::write::h2cdfefb35611b05a
   2:     0x7f711662f9f5 - std::io::Write::write_fmt::h89ae33388fd02b02
   3:     0x7f7116647587 - std::panicking::default_hook::{{closure}}::hc7f7aa88d5363d79
   4:     0x7f7116646fb0 - std::panicking::default_hook::ha3c368049283b92b
   5:     0x7f71170e3a61 - rustc_driver[b8ba8c5e6667b42]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f7116647d23 - std::panicking::rust_panic_with_hook::ha7c8c9b674645fc7
   7:     0x7f7116647b77 - std::panicking::begin_panic_handler::{{closure}}::h92bbdf6cd6cfbadb
   8:     0x7f7116643344 - std::sys_common::backtrace::__rust_end_short_backtrace::ha43da447723f4797
   9:     0x7f7116647869 - rust_begin_unwind
  10:     0x7f71165fa8a3 - core::panicking::panic_fmt::h06103882258a30c7
  11:     0x7f7119a6df08 - core[92d31e8e1173dd18]::panicking::panic_display::<&str>
  12:     0x7f7119a6abdf - <rustc_errors[85d10542d29e7dac]::HandlerInner>::flush_delayed
  13:     0x7f7119a67556 - <rustc_errors[85d10542d29e7dac]::HandlerInner as core[92d31e8e1173dd18]::ops::drop::Drop>::drop
  14:     0x7f71170ca202 - core[92d31e8e1173dd18]::ptr::drop_in_place::<rustc_session[1df61b96738c93fa]::parse::ParseSess>
  15:     0x7f71170cfc5a - <alloc[3b7ab78bdcc277a0]::rc::Rc<rustc_session[1df61b96738c93fa]::session::Session> as core[92d31e8e1173dd18]::ops::drop::Drop>::drop
  16:     0x7f71170be9cc - core[92d31e8e1173dd18]::ptr::drop_in_place::<rustc_interface[2133d471a62aeb75]::interface::Compiler>
  17:     0x7f71170c3cce - rustc_span[2116de005453952b]::with_source_map::<core[92d31e8e1173dd18]::result::Result<(), rustc_errors[85d10542d29e7dac]::ErrorReported>, rustc_interface[2133d471a62aeb75]::interface::create_compiler_and_run<core[92d31e8e1173dd18]::result::Result<(), rustc_errors[85d10542d29e7dac]::ErrorReported>, rustc_driver[b8ba8c5e6667b42]::run_compiler::{closure#1}>::{closure#1}>
  18:     0x7f71171456ed - rustc_interface[2133d471a62aeb75]::interface::create_compiler_and_run::<core[92d31e8e1173dd18]::result::Result<(), rustc_errors[85d10542d29e7dac]::ErrorReported>, rustc_driver[b8ba8c5e6667b42]::run_compiler::{closure#1}>
  19:     0x7f71170f4544 - std[fb71fbea334a0009]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[2133d471a62aeb75]::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface[2133d471a62aeb75]::interface::run_compiler<core[92d31e8e1173dd18]::result::Result<(), rustc_errors[85d10542d29e7dac]::ErrorReported>, rustc_driver[b8ba8c5e6667b42]::run_compiler::{closure#1}>::{closure#0}, core[92d31e8e1173dd18]::result::Result<(), rustc_errors[85d10542d29e7dac]::ErrorReported>>::{closure#0}, core[92d31e8e1173dd18]::result::Result<(), rustc_errors[85d10542d29e7dac]::ErrorReported>>
  20:     0x7f7117149ea1 - std[fb71fbea334a0009]::panicking::try::<core[92d31e8e1173dd18]::result::Result<(), rustc_errors[85d10542d29e7dac]::ErrorReported>, core[92d31e8e1173dd18]::panic::unwind_safe::AssertUnwindSafe<<std[fb71fbea334a0009]::thread::Builder>::spawn_unchecked_<rustc_interface[2133d471a62aeb75]::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface[2133d471a62aeb75]::interface::run_compiler<core[92d31e8e1173dd18]::result::Result<(), rustc_errors[85d10542d29e7dac]::ErrorReported>, rustc_driver[b8ba8c5e6667b42]::run_compiler::{closure#1}>::{closure#0}, core[92d31e8e1173dd18]::result::Result<(), rustc_errors[85d10542d29e7dac]::ErrorReported>>::{closure#0}, core[92d31e8e1173dd18]::result::Result<(), rustc_errors[85d10542d29e7dac]::ErrorReported>>::{closure#1}::{closure#0}>>
  21:     0x7f71170eb9ea - <<std[fb71fbea334a0009]::thread::Builder>::spawn_unchecked_<rustc_interface[2133d471a62aeb75]::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface[2133d471a62aeb75]::interface::run_compiler<core[92d31e8e1173dd18]::result::Result<(), rustc_errors[85d10542d29e7dac]::ErrorReported>, rustc_driver[b8ba8c5e6667b42]::run_compiler::{closure#1}>::{closure#0}, core[92d31e8e1173dd18]::result::Result<(), rustc_errors[85d10542d29e7dac]::ErrorReported>>::{closure#0}, core[92d31e8e1173dd18]::result::Result<(), rustc_errors[85d10542d29e7dac]::ErrorReported>>::{closure#1} as core[92d31e8e1173dd18]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  22:     0x7f7116655c13 - std::sys::unix::thread::Thread::new::thread_start::h553976250c6487ea
  23:     0x7f71109c5609 - start_thread
  24:     0x7f71164bc293 - clone
  25:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.60.0-nightly (411edcba8 2022-01-24) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
thread panicked while panicking. aborting.

---
test result: FAILED. 12424 passed; 8 failed; 121 ignored; 0 measured; 0 filtered out; finished in 125.88s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:39
