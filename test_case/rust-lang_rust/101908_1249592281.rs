plain
+    |     ^
+    |
+ help: maybe you meant to write an assignment here
+    |
+ LL |     let y = 2;
+ help: a local variable with a similar name exists
+    |
+ LL |     x = 2;
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
---
To only update this specific test, also pass `--test-args error-festival.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-festival.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-festival" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-festival/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0425]: cannot find value `y` in this scope
   |
LL |     y = 2;
   |     ^
   |
---
   |
LL |     const FOO: u32 = 0;
   |     ^^^^^^^^^^^^^^^^^^^

error[E0368]: binary assignment operation `+=` cannot be applied to type `&str`
   |
LL |     x += 2;
   |     -^^^^^
   |     |
   |     |
   |     cannot use `+=` on type `&str`

error[E0599]: no method named `z` found for reference `&str` in the current scope
   |
LL |     x.z();
   |       ^ method not found in `&str`


error[E0600]: cannot apply unary operator `!` to type `Question`
  --> /checkout/src/test/ui/error-festival.rs:19:5
   |
LL |     !Question::Yes;
   |
   |
note: an implementation of `Not` might be missing for `Question`
   |
   |
LL | enum Question {
   | ^^^^^^^^^^^^^ must implement `Not`
  --> /checkout/library/core/src/ops/bit.rs:34:1
   |
LL | pub trait Not {
   | ^^^^^^^^^^^^^
   | ^^^^^^^^^^^^^

error[E0604]: only `u8` can be cast as `char`, not `u32`
   |
LL |     0u32 as char;
   |     ^^^^^^^^^^^^
   |     |
   |     |
   |     invalid cast
   |     help: try `char::from_u32` instead: `char::from_u32(0u32)`

error[E0605]: non-primitive cast: `u8` as `Vec<u8>`
   |
LL |     x as Vec<u8>;
LL |     x as Vec<u8>;
   |     ^^^^^^^^^^^^ an `as` expression can only be used to convert between primitive types or to coerce to a specific trait object
error[E0054]: cannot cast as `bool`
  --> /checkout/src/test/ui/error-festival.rs:33:24
   |
LL |     let x_is_nonzero = x as bool;
LL |     let x_is_nonzero = x as bool;
   |                        ^^^^^^^^^ help: compare with zero instead: `x != 0`

error[E0606]: casting `&u8` as `u32` is invalid
   |
   |
LL |     let y: u32 = x as u32;
   |                  -^^^^^^^
   |                  |
   |                  cannot cast `&u8` as `u32`
   |                  help: dereference the expression: `*x`

error[E0607]: cannot cast thin pointer `*const u8` to fat pointer `*const [u8]`
   |
LL |     v as *const [u8];
   |     ^^^^^^^^^^^^^^^^

---


---- [ui] src/test/ui/proc-macro/mixed-site-span.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/mixed-site-span.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/mixed-site-span" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/mixed-site-span/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0426]: use of undeclared label `'label_use`
   |
LL |         proc_macro_rules!();
LL |         proc_macro_rules!();
   |         ^^^^^^^^^^^^^^^^^^^ undeclared label `'label_use`
   = note: this error originates in the macro `proc_macro_rules` (in Nightly builds, run with -Z macro-backtrace for more info)


thread 'rustc' panicked at 'called `Result::unwrap()` on an `Err` value: DistinctSources(DistinctSources { begin: (Real(LocalPath("/checkout/src/test/ui/proc-macro/mixed-site-span.rs")), BytePos(0)), end: (Real(LocalPath("/checkout/src/test/ui/proc-macro/auxiliary/mixed-site-span.rs")), BytePos(298690)) })', compiler/rustc_resolve/src/late/diagnostics.rs:271:67
stack backtrace:
   0:     0x7f09779da52e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h9bd758f4be7dc03e
   1:     0x7f0977a43308 - core::fmt::write::h827ce4d30f6d273d
   2:     0x7f09779cb311 - std::io::Write::write_fmt::h4d2273ff6e1d262c
   3:     0x7f09779dd4fe - std::panicking::default_hook::{{closure}}::h2aa712eab99b82c3
   4:     0x7f09779dd1be - std::panicking::default_hook::ha5bd7896164808c1
   5:     0x7f09783aeb54 - rustc_driver[ffcbba2efc9b7cda]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f09779ddcb1 - std::panicking::rust_panic_with_hook::h61cc83b3b9bd1d67
   7:     0x7f09779ddad7 - std::panicking::begin_panic_handler::{{closure}}::h229add4dc9fc233b
   8:     0x7f09779daa64 - std::sys_common::backtrace::__rust_end_short_backtrace::ha4702affee4002a0
   9:     0x7f09779dd7a2 - rust_begin_unwind
  10:     0x7f097798cf03 - core::panicking::panic_fmt::h0769e80cba2f3d57
  11:     0x7f097798d0a3 - core::result::unwrap_failed::h2722a02ce0c7489f
  12:     0x7f09791055ef - <rustc_resolve[b56d5f8413371fb2]::late::LateResolutionVisitor>::smart_resolve_report_errors
  13:     0x7f097914aec0 - <rustc_resolve[b56d5f8413371fb2]::late::LateResolutionVisitor>::smart_resolve_path_fragment::{closure#0}
  14:     0x7f09791483b3 - <rustc_resolve[b56d5f8413371fb2]::late::LateResolutionVisitor>::smart_resolve_path_fragment
  15:     0x7f0979147309 - <rustc_resolve[b56d5f8413371fb2]::late::LateResolutionVisitor>::smart_resolve_path
  16:     0x7f097910ec0b - <rustc_resolve[b56d5f8413371fb2]::late::LateResolutionVisitor as rustc_ast[118f11ae574b3f4b]::visit::Visitor>::visit_ty
  17:     0x7f09792138c9 - rustc_ast[118f11ae574b3f4b]::visit::walk_item::<rustc_resolve[b56d5f8413371fb2]::late::LateResolutionVisitor>
  18:     0x7f0979131fdb - <rustc_resolve[b56d5f8413371fb2]::late::LateResolutionVisitor>::resolve_item
  19:     0x7f097910c267 - <rustc_resolve[b56d5f8413371fb2]::late::LateResolutionVisitor as rustc_ast[118f11ae574b3f4b]::visit::Visitor>::visit_item
  20:     0x7f09791fa03d - rustc_ast[118f11ae574b3f4b]::visit::walk_crate::<rustc_resolve[b56d5f8413371fb2]::late::LateResolutionVisitor>
  21:     0x7f09791db2ae - <rustc_resolve[b56d5f8413371fb2]::Resolver>::late_resolve_crate
  22:     0x7f09791a42c5 - <rustc_session[792106d857a98eba]::session::Session>::time::<(), <rustc_resolve[b56d5f8413371fb2]::Resolver>::resolve_crate::{closure#0}>
  23:     0x7f097850687a - rustc_interface[9f41295061ef0852]::passes::configure_and_expand
  24:     0x7f09784f4f26 - <rustc_interface[9f41295061ef0852]::passes::boxed_resolver::BoxedResolver>::access::<<rustc_interface[9f41295061ef0852]::queries::Queries>::expansion::{closure#0}::{closure#0}, core[4dccbc9f436e7e63]::result::Result<rustc_ast[118f11ae574b3f4b]::ast::Crate, rustc_errors[7e40ae17dd12cd78]::ErrorGuaranteed>>
  25:     0x7f09784e0136 - <rustc_interface[9f41295061ef0852]::queries::Queries>::expansion
  26:     0x7f09783c5538 - rustc_interface[9f41295061ef0852]::interface::create_compiler_and_run::<core[4dccbc9f436e7e63]::result::Result<(), rustc_errors[7e40ae17dd12cd78]::ErrorGuaranteed>, rustc_driver[ffcbba2efc9b7cda]::run_compiler::{closure#1}>
  27:     0x7f097842949f - <scoped_tls[85ed544e822ae303]::ScopedKey<rustc_span[e4600d84180e9735]::SessionGlobals>>::set::<rustc_interface[9f41295061ef0852]::interface::run_compiler<core[4dccbc9f436e7e63]::result::Result<(), rustc_errors[7e40ae17dd12cd78]::ErrorGuaranteed>, rustc_driver[ffcbba2efc9b7cda]::run_compiler::{closure#1}>::{closure#0}, core[4dccbc9f436e7e63]::result::Result<(), rustc_errors[7e40ae17dd12cd78]::ErrorGuaranteed>>
  28:     0x7f097841372f - std[8958d5a9579685a]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[9f41295061ef0852]::util::run_in_thread_pool_with_globals<rustc_interface[9f41295061ef0852]::interface::run_compiler<core[4dccbc9f436e7e63]::result::Result<(), rustc_errors[7e40ae17dd12cd78]::ErrorGuaranteed>, rustc_driver[ffcbba2efc9b7cda]::run_compiler::{closure#1}>::{closure#0}, core[4dccbc9f436e7e63]::result::Result<(), rustc_errors[7e40ae17dd12cd78]::ErrorGuaranteed>>::{closure#0}, core[4dccbc9f436e7e63]::result::Result<(), rustc_errors[7e40ae17dd12cd78]::ErrorGuaranteed>>
  29:     0x7f09783c3ef1 - std[8958d5a9579685a]::panic::catch_unwind::<core[4dccbc9f436e7e63]::panic::unwind_safe::AssertUnwindSafe<<std[8958d5a9579685a]::thread::Builder>::spawn_unchecked_<rustc_interface[9f41295061ef0852]::util::run_in_thread_pool_with_globals<rustc_interface[9f41295061ef0852]::interface::run_compiler<core[4dccbc9f436e7e63]::result::Result<(), rustc_errors[7e40ae17dd12cd78]::ErrorGuaranteed>, rustc_driver[ffcbba2efc9b7cda]::run_compiler::{closure#1}>::{closure#0}, core[4dccbc9f436e7e63]::result::Result<(), rustc_errors[7e40ae17dd12cd78]::ErrorGuaranteed>>::{closure#0}, core[4dccbc9f436e7e63]::result::Result<(), rustc_errors[7e40ae17dd12cd78]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[4dccbc9f436e7e63]::result::Result<(), rustc_errors[7e40ae17dd12cd78]::ErrorGuaranteed>>
  30:     0x7f0978416f70 - <<std[8958d5a9579685a]::thread::Builder>::spawn_unchecked_<rustc_interface[9f41295061ef0852]::util::run_in_thread_pool_with_globals<rustc_interface[9f41295061ef0852]::interface::run_compiler<core[4dccbc9f436e7e63]::result::Result<(), rustc_errors[7e40ae17dd12cd78]::ErrorGuaranteed>, rustc_driver[ffcbba2efc9b7cda]::run_compiler::{closure#1}>::{closure#0}, core[4dccbc9f436e7e63]::result::Result<(), rustc_errors[7e40ae17dd12cd78]::ErrorGuaranteed>>::{closure#0}, core[4dccbc9f436e7e63]::result::Result<(), rustc_errors[7e40ae17dd12cd78]::ErrorGuaranteed>>::{closure#1} as core[4dccbc9f436e7e63]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  31:     0x7f09779eaaa5 - std::sys::unix::thread::Thread::new::thread_start::hfff75054aa4b0467
  32:     0x7f0977785b43 - <unknown>
  33:     0x7f0977817a00 - <unknown>
  34:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (651c03a06 2022-09-16) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
error: aborting due to previous error

---
diff of stderr:

106   --> $DIR/issue-2356.rs:79:5
107    |
108 LL |     whiskers = 0;
-    |     ^^^^^^^^ help: you might have meant to use the available field: `self.whiskers`
+    |
+ help: maybe you meant to write an assignment here
+    |
+    |
+ LL |     let whiskers = 0;
+    |     ~~~~~~~~~~~~
+ help: you might have meant to use the available field
+    |
+ LL |     self.whiskers = 0;
110 
110 
111 error[E0425]: cannot find value `whiskers` in this scope

113    |
113    |
114 LL |     whiskers = 4;
115    |     ^^^^^^^^ a field by this name exists in `Self`
+ help: maybe you meant to write an assignment here
+    |
+    |
+ LL |     let whiskers = 4;
116 
116 
117 error[E0425]: cannot find function `purr_louder` in this scope


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/issue-2356/issue-2356.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/issue-2356/issue-2356.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args resolve/issue-2356.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/resolve/issue-2356.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/issue-2356" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/issue-2356/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0425]: cannot find function `shave` in this scope
   |
LL |     shave();
   |     ^^^^^ not found in this scope


error[E0425]: cannot find function `clone` in this scope
  --> /checkout/src/test/ui/resolve/issue-2356.rs:24:5
   |
LL |     clone();
   |     ^^^^^ help: you might have meant to call the method: `self.clone`
error[E0425]: cannot find function `default` in this scope
  --> /checkout/src/test/ui/resolve/issue-2356.rs:31:5
   |
LL |     default();
---
   |
LL | use std::default::default;
   |

error[E0425]: cannot find value `whiskers` in this scope
   |
   |
LL |     whiskers -= other;
   |     ^^^^^^^^ a field by this name exists in `Self`

error[E0425]: cannot find function `shave` in this scope
   |
LL |     shave(4);
LL |     shave(4);
   |     ^^^^^ help: you might have meant to call the associated function: `Self::shave`

error[E0425]: cannot find function `purr` in this scope
   |
LL |     purr();
   |     ^^^^ not found in this scope

---
   |
LL |         Self::static_method();
   |         ~~~~~~~~~~~~~~~~~~~

error[E0425]: cannot find function `purr` in this scope
   |
LL |         purr();
   |         ^^^^ not found in this scope


error[E0425]: cannot find function `purr` in this scope
   |
LL |         purr();
   |         ^^^^ not found in this scope


error[E0425]: cannot find function `purr` in this scope
   |
LL |         purr();
   |         ^^^^ not found in this scope


error[E0424]: expected value, found module `self`
  --> /checkout/src/test/ui/resolve/issue-2356.rs:65:8
   |
LL |   fn meow() {
   |      ---- this function doesn't have a `self` parameter
LL |     if self.whiskers > 3 {
   |        ^^^^ `self` value is a keyword only available in methods with a `self` parameter
   |
help: add a `self` receiver parameter to make the associated `fn` a method
   |
LL |   fn meow(&self) {

error[E0425]: cannot find function `grow_older` in this scope
  --> /checkout/src/test/ui/resolve/issue-2356.rs:72:5
   |
---
   |
LL |     Self::grow_older();
   |     ~~~~~~~~~~~~~~~~

error[E0425]: cannot find function `shave` in this scope
   |
LL |     shave();
   |     ^^^^^ not found in this scope


error[E0425]: cannot find value `whiskers` in this scope
   |
   |
LL |     whiskers = 0;
   |
help: maybe you meant to write an assignment here
   |
   |
LL |     let whiskers = 0;
   |     ~~~~~~~~~~~~
help: you might have meant to use the available field
   |
LL |     self.whiskers = 0;


error[E0425]: cannot find value `whiskers` in this scope
   |
   |
LL |     whiskers = 4;
   |     ^^^^^^^^ a field by this name exists in `Self`
help: maybe you meant to write an assignment here
   |
   |
LL |     let whiskers = 4;


error[E0425]: cannot find function `purr_louder` in this scope
   |
   |
LL |     purr_louder();

error[E0424]: expected value, found module `self`
  --> /checkout/src/test/ui/resolve/issue-2356.rs:92:5
   |
   |
LL | fn main() {
   |    ---- this function can't have a `self` parameter
LL |     self += 1;
   |     ^^^^ `self` value is a keyword only available in methods with a `self` parameter
error: aborting due to 17 previous errors

Some errors have detailed explanations: E0424, E0425.
For more information about an error, try `rustc --explain E0424`.
For more information about an error, try `rustc --explain E0424`.
------------------------------------------


---- [ui] src/test/ui/self/self_type_keyword-2.rs stdout ----
diff of stderr:

15    |
16 LL |         Self => (),
+    |
+ help: maybe you meant to write an assignment here
+    |
+    |
+ LL |         let Self => (),
18 
19 error[E0531]: cannot find unit struct, unit variant or constant `Self` in this scope
20   --> $DIR/self_type_keyword-2.rs:10:18



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/self_type_keyword-2/self_type_keyword-2.stderr
To only update this specific test, also pass `--test-args self/self_type_keyword-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/self_type_keyword-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/self_type_keyword-2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/self_type_keyword-2/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0432]: unresolved import `self::Self`
   |
   |
LL | use self::Self as Foo; //~ ERROR unresolved import `self::Self`
   |     ^^^^^^^^^^^^^^^^^ no `Self` in the root
error[E0531]: cannot find unit struct, unit variant or constant `Self` in this scope
  --> /checkout/src/test/ui/self/self_type_keyword-2.rs:4:9
   |
LL |     let Self = 5;
LL |     let Self = 5;
   |         ^^^^ not found in this scope

error[E0531]: cannot find unit struct, unit variant or constant `Self` in this scope
  --> /checkout/src/test/ui/self/self_type_keyword-2.rs:8:9
   |
LL |         Self => (),
   |
help: maybe you meant to write an assignment here
   |
   |
LL |         let Self => (),

error[E0531]: cannot find unit struct, unit variant or constant `Self` in this scope
  --> /checkout/src/test/ui/self/self_type_keyword-2.rs:10:18
   |
   |
LL |         Foo { x: Self } => (),

error: aborting due to 4 previous errors

Some errors have detailed explanations: E0432, E0531.
