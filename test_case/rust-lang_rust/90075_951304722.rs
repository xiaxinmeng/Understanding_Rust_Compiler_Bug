plain
.................................................................................................... 4100/12341
.................................................................................................... 4200/12341
.................................................................................................... 4300/12341
.................................................................................................... 4400/12341
....................................................................F............................... 4500/12341
.............................................................F...........F......FF....ii............ 4600/12341
.................................................................................................... 4800/12341
.................................................................................................... 4900/12341
.................................................................................................... 5000/12341
.................................................................................................... 5100/12341
---
.......i............................................................................................ 6800/12341
........i........................................................................................... 6900/12341
.......................................i............................................................ 7000/12341
...........................................................ii....................................... 7100/12341
................i...................................................................F............... 7200/12341
.................F.FF............................................................................... 7300/12341
.................................................................................................... 7500/12341
..............ii................i.....i.ii.......................................................... 7600/12341
.................................................................................................... 7700/12341
.................................................................................................... 7800/12341
---

15 LL |         Vec::panic!();
16    |         ^^^ ambiguous name
17    |
-    = note: ambiguous because of a conflict between a macro-expanded name and less macro-expanded name from outer scope during import/macro resolution
+    = note: ambiguous because of a conflict between a macro-expanded name and a less macro-expanded name from outer scope during import/macro resolution
19 note: `Vec` could refer to the crate imported here
21    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/extern-prelude-extern-crate-restricted-shadowing/extern-prelude-extern-crate-restricted-shadowing.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args imports/extern-prelude-extern-crate-restricted-shadowing.rs`
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/imports/extern-prelude-extern-crate-restricted-shadowing.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/extern-prelude-extern-crate-restricted-shadowing" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/extern-prelude-extern-crate-restricted-shadowing/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: macro-expanded `extern crate` items cannot shadow names passed with `--extern`
   |
LL |         extern crate std as core;
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^
...
...
LL | define_other_core!();
   |
   = note: this error originates in the macro `define_other_core` (in Nightly builds, run with -Z macro-backtrace for more info)


error[E0659]: `Vec` is ambiguous
   |
   |
LL |         Vec::panic!(); //~ ERROR `Vec` is ambiguous
   |         ^^^ ambiguous name
   |
   = note: ambiguous because of a conflict between a macro-expanded name and a less macro-expanded name from outer scope during import/macro resolution
note: `Vec` could refer to the crate imported here
   |
   |
LL |         extern crate std as Vec;
...
LL | define_vec!();
   | ------------- in this macro invocation
   | ------------- in this macro invocation
note: `Vec` could also refer to the struct defined here
   |
   |
LL |     pub use super::v1::*;
   = note: this error originates in the macro `define_vec` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to 2 previous errors

---

54 LL |     panic!();
55    |     ^^^^^ ambiguous name
56    |
-    = note: ambiguous because of a conflict between a macro-expanded name and less macro-expanded name from outer scope during import/macro resolution
+    = note: ambiguous because of a conflict between a macro-expanded name and a less macro-expanded name from outer scope during import/macro resolution
58    = note: `panic` could refer to a macro from prelude
59 note: `panic` could also refer to the macro defined here


75 LL | include!();
76    | ^^^^^^^ ambiguous name
77    |
-    = note: ambiguous because of a conflict between a macro-expanded name and less macro-expanded name from outer scope during import/macro resolution
+    = note: ambiguous because of a conflict between a macro-expanded name and a less macro-expanded name from outer scope during import/macro resolution
79    = note: `include` could refer to a macro from prelude
80 note: `include` could also refer to the macro defined here


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/local-modularized-tricky-fail-1/local-modularized-tricky-fail-1.stderr
To only update this specific test, also pass `--test-args imports/local-modularized-tricky-fail-1.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/imports/local-modularized-tricky-fail-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/local-modularized-tricky-fail-1" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/local-modularized-tricky-fail-1/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0659]: `exported` is ambiguous
   |
   |
LL | exported!(); //~ ERROR `exported` is ambiguous
   | ^^^^^^^^ ambiguous name
   |
   = note: ambiguous because of a conflict between a name from glob import and macro-expanded name in the same module
note: `exported` could refer to the macro defined here
   |
   |
LL | /     macro_rules! exported {
LL | |         () => ()
LL | |     }
...
LL |       define_exported!();
   |       ------------------ in this macro invocation
   |       ------------------ in this macro invocation
note: `exported` could also refer to the macro imported here
   |
LL | use inner1::*;
   |     ^^^^^^^^^
   |     ^^^^^^^^^
   = help: consider adding an explicit import of `exported` to disambiguate
   = note: this error originates in the macro `define_exported` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0659]: `exported` is ambiguous
   |
   |
LL | exported!(); //~ ERROR `exported` is ambiguous
   | ^^^^^^^^ ambiguous name
   |
   = note: ambiguous because of a conflict between a name from glob import and macro-expanded name in the same module
note: `exported` could refer to the macro defined here
   |
   |
LL | /     macro_rules! exported {
LL | |         () => ()
LL | |     }
...
LL |       define_exported!();
   |       ------------------ in this macro invocation
   |       ------------------ in this macro invocation
note: `exported` could also refer to the macro imported here
   |
LL | use inner1::*;
   |     ^^^^^^^^^
   |     ^^^^^^^^^
   = help: consider adding an explicit import of `exported` to disambiguate
   = note: this error originates in the macro `define_exported` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0659]: `panic` is ambiguous
   |
   |
LL |     panic!(); //~ ERROR `panic` is ambiguous
   |     ^^^^^ ambiguous name
   |
   = note: ambiguous because of a conflict between a macro-expanded name and a less macro-expanded name from outer scope during import/macro resolution
   = note: `panic` could refer to a macro from prelude
note: `panic` could also refer to the macro defined here
   |
LL | /     macro_rules! panic {
LL | |         () => ()
LL | |     }
LL | |     }
   | |_____^
...
LL |       define_panic!();
   |       --------------- in this macro invocation
   = help: use `crate::panic` to refer to this macro unambiguously
   = note: this error originates in the macro `define_panic` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0659]: `include` is ambiguous
   |
   |
LL | include!(); //~ ERROR `include` is ambiguous
   | ^^^^^^^ ambiguous name
   |
   = note: ambiguous because of a conflict between a macro-expanded name and a less macro-expanded name from outer scope during import/macro resolution
   = note: `include` could refer to a macro from prelude
note: `include` could also refer to the macro defined here
   |
LL | /     macro_rules! include {
LL | |         () => ()
LL | |     }
LL | |     }
   | |_____^
...
LL |       define_include!();
   |       ----------------- in this macro invocation
   = help: use `crate::include` to refer to this macro unambiguously
   = note: this error originates in the macro `define_include` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0659`.


------------------------------------------


---- [ui] ui/imports/macro-paths.rs stdout ----
diff of stderr:

23 LL |     baz::m! {
24    |     ^^^ ambiguous name
25    |
-    = note: ambiguous because of a conflict between a macro-expanded name and less macro-expanded name from outer scope during import/macro resolution
+    = note: ambiguous because of a conflict between a macro-expanded name and a less macro-expanded name from outer scope during import/macro resolution
27 note: `baz` could refer to the module defined here
29    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/macro-paths/macro-paths.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args imports/macro-paths.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/imports/macro-paths.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/macro-paths" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/macro-paths/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0659]: `bar` is ambiguous
   |
   |
LL |     bar::m! { //~ ERROR ambiguous
   |     ^^^ ambiguous name
   |
   = note: ambiguous because of a conflict between a name from glob import and macro-expanded name in the same module
note: `bar` could refer to the module defined here
   |
   |
LL |         mod bar { pub use two_macros::m; }
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: `bar` could also refer to the module imported here
   |
LL |     use foo::*;
   |         ^^^^^^
   |         ^^^^^^
   = help: consider adding an explicit import of `bar` to disambiguate

error[E0659]: `baz` is ambiguous
   |
   |
LL |     baz::m! { //~ ERROR ambiguous
   |     ^^^ ambiguous name
   |
   = note: ambiguous because of a conflict between a macro-expanded name and a less macro-expanded name from outer scope during import/macro resolution
note: `baz` could refer to the module defined here
   |
   |
LL |         mod baz { pub use two_macros::m; }
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: `baz` could also refer to the module defined here
   |
   |
LL | / pub mod baz {
LL | |     pub use two_macros::m;
LL | | }
   | |_^
   = help: use `crate::baz` to refer to this module unambiguously
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0659`.

---

20 LL |     panic!();
21    |     ^^^^^ ambiguous name
22    |
-    = note: ambiguous because of a conflict between a macro-expanded name and less macro-expanded name from outer scope during import/macro resolution
+    = note: ambiguous because of a conflict between a macro-expanded name and a less macro-expanded name from outer scope during import/macro resolution
24    = note: `panic` could refer to a macro from prelude
25 note: `panic` could also refer to the macro defined here


58 LL |     fn f() { panic!(); }
59    |              ^^^^^ ambiguous name
60    |
-    = note: ambiguous because of a conflict between a macro-expanded name and less macro-expanded name from outer scope during import/macro resolution
+    = note: ambiguous because of a conflict between a macro-expanded name and a less macro-expanded name from outer scope during import/macro resolution
62    = note: `panic` could refer to a macro from prelude
63 note: `panic` could also refer to the macro imported here


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/shadow_builtin_macros/shadow_builtin_macros.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/shadow_builtin_macros/shadow_builtin_macros.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args imports/shadow_builtin_macros.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/imports/shadow_builtin_macros.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/shadow_builtin_macros" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/shadow_builtin_macros/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0659]: `panic` is ambiguous
   |
   |
LL |     fn f() { panic!(); } //~ ERROR ambiguous
   |              ^^^^^ ambiguous name
   |
   = note: ambiguous because of a name conflict with an outer scope during import/macro resolution due to glob import
   = note: `panic` could refer to a macro from prelude
note: `panic` could also refer to the macro imported here
   |
LL |     use foo::*;
   |         ^^^^^^
   = help: consider adding an explicit import of `panic` to disambiguate
   = help: consider adding an explicit import of `panic` to disambiguate
   = help: or use `self::panic` to refer to this macro unambiguously

error[E0659]: `panic` is ambiguous
   |
   |
LL |     panic!(); //~ ERROR `panic` is ambiguous
   |     ^^^^^ ambiguous name
   |
   = note: ambiguous because of a conflict between a macro-expanded name and a less macro-expanded name from outer scope during import/macro resolution
   = note: `panic` could refer to a macro from prelude
note: `panic` could also refer to the macro defined here
   |
   |
LL |         macro_rules! panic { () => {} }
LL |     } }
LL |     m!();
   |     ---- in this macro invocation
   |     ---- in this macro invocation
   = note: this error originates in the macro `m` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0659]: `n` is ambiguous
   |
   |
LL |     n!(); //~ ERROR ambiguous
   |     ^ ambiguous name
   |
   = note: ambiguous because of a name conflict with an outer scope during import/macro resolution due to glob import
note: `n` could refer to the macro imported here
   |
   |
LL |     use bar::*;
   = help: consider adding an explicit import of `n` to disambiguate
   = help: consider adding an explicit import of `n` to disambiguate
   = help: or use `self::n` to refer to this macro unambiguously
note: `n` could also refer to the macro imported here
   |
LL | #[macro_use(n)]
   |             ^


error[E0659]: `panic` is ambiguous
   |
   |
LL |     fn f() { panic!(); } //~ ERROR ambiguous
   |              ^^^^^ ambiguous name
   |
   = note: ambiguous because of a conflict between a macro-expanded name and a less macro-expanded name from outer scope during import/macro resolution
   = note: `panic` could refer to a macro from prelude
note: `panic` could also refer to the macro imported here
   |
   |
LL |     ::two_macros::m!(use foo::panic;);
   |                          ^^^^^^^^^^
   = help: use `self::panic` to refer to this macro unambiguously
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0659`.

---

42 LL |         m! {
43    |         ^ ambiguous name
44    |
-    = note: ambiguous because of a conflict between a macro-expanded name and less macro-expanded name from outer scope during import/macro resolution
+    = note: ambiguous because of a conflict between a macro-expanded name and a less macro-expanded name from outer scope during import/macro resolution
46 note: `m` could refer to the macro imported here
48    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/macros/macros.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args imports/macros.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/imports/macros.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/macros" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/macros/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0659]: `m` is ambiguous
   |
   |
LL |     m! { //~ ERROR ambiguous
   |     ^ ambiguous name
   |
   = note: ambiguous because of a conflict between a name from glob import and macro-expanded name in the same module
note: `m` could refer to the macro imported here
   |
LL |         use foo::m;
   |             ^^^^^^
   |             ^^^^^^
note: `m` could also refer to the macro imported here
   |
LL |     use two_macros::*;
   |         ^^^^^^^^^^^^^
   |         ^^^^^^^^^^^^^
   = help: consider adding an explicit import of `m` to disambiguate

error[E0659]: `m` is ambiguous
   |
   |
LL |     m! { //~ ERROR ambiguous
   |     ^ ambiguous name
   |
   = note: ambiguous because of a conflict between a name from glob import and macro-expanded name in the same module
note: `m` could refer to the macro imported here
   |
LL |         use foo::m;
   |             ^^^^^^
   |             ^^^^^^
note: `m` could also refer to the macro imported here
   |
LL |     use two_macros::*;
   |         ^^^^^^^^^^^^^
   |         ^^^^^^^^^^^^^
   = help: consider adding an explicit import of `m` to disambiguate

error[E0659]: `m` is ambiguous
   |
   |
LL |         m! { //~ ERROR ambiguous
   |         ^ ambiguous name
   |
   = note: ambiguous because of a conflict between a macro-expanded name and a less macro-expanded name from outer scope during import/macro resolution
note: `m` could refer to the macro imported here
   |
   |
LL |             use two_macros::n as m;
   |                 ^^^^^^^^^^^^^^^^^^
note: `m` could also refer to the macro imported here
   |
LL |     use two_macros::m;
   |         ^^^^^^^^^^^^^
   |         ^^^^^^^^^^^^^
   = help: use `self::m` to refer to this macro unambiguously
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0659`.


------------------------------------------


---- [ui] ui/macros/macro-shadowing.rs stdout ----
diff of stderr:

16 LL | foo!();
17    | ^^^ ambiguous name
18    |
-    = note: ambiguous because of a conflict between a macro-expanded name and less macro-expanded name from outer scope during import/macro resolution
+    = note: ambiguous because of a conflict between a macro-expanded name and a less macro-expanded name from outer scope during import/macro resolution
20 note: `foo` could refer to the macro defined here
22    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-shadowing/macro-shadowing.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args macros/macro-shadowing.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/macro-shadowing.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-shadowing" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-shadowing/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: `macro_two` is already in scope
   |
   |
LL |     #[macro_use] //~ ERROR `macro_two` is already in scope
...
...
LL | m1!();
   |
   |
   = note: macro-expanded `#[macro_use]`s may not shadow existing macros (see RFC 1560)
   = note: this error originates in the macro `m1` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0659]: `foo` is ambiguous
   |
   |
LL | foo!(); //~ ERROR `foo` is ambiguous
   | ^^^ ambiguous name
   |
   = note: ambiguous because of a conflict between a macro-expanded name and a less macro-expanded name from outer scope during import/macro resolution
note: `foo` could refer to the macro defined here
   |
   |
LL |     macro_rules! foo { () => {} }
...
...
LL | m1!();
note: `foo` could also refer to the macro defined here
  --> /checkout/src/test/ui/macros/macro-shadowing.rs:5:1
   |
   |
LL | macro_rules! foo { () => {} }
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: this error originates in the macro `m1` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0659`.


------------------------------------------


---- [ui] ui/macros/out-of-order-shadowing.rs stdout ----
diff of stderr:

4 LL | bar!();
5    | ^^^ ambiguous name
6    |
-    = note: ambiguous because of a conflict between a macro-expanded name and less macro-expanded name from outer scope during import/macro resolution
+    = note: ambiguous because of a conflict between a macro-expanded name and a less macro-expanded name from outer scope during import/macro resolution
8 note: `bar` could refer to the macro defined here
10    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/out-of-order-shadowing/out-of-order-shadowing.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args macros/out-of-order-shadowing.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/out-of-order-shadowing.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/out-of-order-shadowing" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/out-of-order-shadowing/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0659]: `bar` is ambiguous
   |
   |
LL | bar!(); //~ ERROR `bar` is ambiguous
   | ^^^ ambiguous name
   |
   = note: ambiguous because of a conflict between a macro-expanded name and a less macro-expanded name from outer scope during import/macro resolution
note: `bar` could refer to the macro defined here
   |
   |
LL | define_macro!(bar);
   | ^^^^^^^^^^^^^^^^^^
note: `bar` could also refer to the macro defined here
   |
   |
LL | macro_rules! bar { () => {} }
   = note: this error originates in the macro `define_macro` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to previous error

---

---- [ui] ui/macros/restricted-shadowing-legacy.rs stdout ----
diff of stderr:

7 LL | include!();
9    |
9    |
-    = note: ambiguous because of a conflict between a macro-expanded name and less macro-expanded name from outer scope during import/macro resolution
+    = note: ambiguous because of a conflict between a macro-expanded name and a less macro-expanded name from outer scope during import/macro resolution
11 note: `m` could refer to the macro defined here
13    |


35 LL | include!();
37    |
37    |
-    = note: ambiguous because of a conflict between a macro-expanded name and less macro-expanded name from outer scope during import/macro resolution
+    = note: ambiguous because of a conflict between a macro-expanded name and a less macro-expanded name from outer scope during import/macro resolution
39 note: `m` could refer to the macro defined here
41    |


63 LL | include!();
65    |
65    |
-    = note: ambiguous because of a conflict between a macro-expanded name and less macro-expanded name from outer scope during import/macro resolution
+    = note: ambiguous because of a conflict between a macro-expanded name and a less macro-expanded name from outer scope during import/macro resolution
67 note: `m` could refer to the macro defined here
69    |


91 LL | include!();
93    |
93    |
-    = note: ambiguous because of a conflict between a macro-expanded name and less macro-expanded name from outer scope during import/macro resolution
+    = note: ambiguous because of a conflict between a macro-expanded name and a less macro-expanded name from outer scope during import/macro resolution
95 note: `m` could refer to the macro defined here
97    |


119 LL | include!();
121    |
121    |
-    = note: ambiguous because of a conflict between a macro-expanded name and less macro-expanded name from outer scope during import/macro resolution
+    = note: ambiguous because of a conflict between a macro-expanded name and a less macro-expanded name from outer scope during import/macro resolution
123 note: `m` could refer to the macro defined here
125    |


147 LL | include!();
149    |
149    |
-    = note: ambiguous because of a conflict between a macro-expanded name and less macro-expanded name from outer scope during import/macro resolution
+    = note: ambiguous because of a conflict between a macro-expanded name and a less macro-expanded name from outer scope during import/macro resolution
151 note: `m` could refer to the macro defined here
153    |


175 LL | include!();
177    |
177    |
-    = note: ambiguous because of a conflict between a macro-expanded name and less macro-expanded name from outer scope during import/macro resolution
+    = note: ambiguous because of a conflict between a macro-expanded name and a less macro-expanded name from outer scope during import/macro resolution
179 note: `m` could refer to the macro defined here
181    |


203 LL | include!();
205    |
205    |
-    = note: ambiguous because of a conflict between a macro-expanded name and less macro-expanded name from outer scope during import/macro resolution
+    = note: ambiguous because of a conflict between a macro-expanded name and a less macro-expanded name from outer scope during import/macro resolution
207 note: `m` could refer to the macro defined here
209    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/restricted-shadowing-legacy/restricted-shadowing-legacy.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args macros/restricted-shadowing-legacy.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/restricted-shadowing-legacy.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/restricted-shadowing-legacy" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/restricted-shadowing-legacy/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0659]: `m` is ambiguous
   |
   |
LL |             m!(); //~ ERROR `m` is ambiguous
   |             ^ ambiguous name
...
LL | include!();
   |
   |
   = note: ambiguous because of a conflict between a macro-expanded name and a less macro-expanded name from outer scope during import/macro resolution
note: `m` could refer to the macro defined here
   |
   |
LL |         macro_rules! m { () => { Right } }
...
...
LL | include!();
   | ---------- in this macro invocation
note: `m` could also refer to the macro defined here
   |
   |
LL |         macro_rules! m { () => {} }
...
...
LL | include!();
   | ---------- in this macro invocation
   = note: this error originates in the macro `gen_gen_inner_invoc` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0659]: `m` is ambiguous
   |
   |
LL |         macro_rules! gen_invoc { () => { m!() } } //~ ERROR `m` is ambiguous
   |                                          ^ ambiguous name
...
LL | include!();
   |
   |
   = note: ambiguous because of a conflict between a macro-expanded name and a less macro-expanded name from outer scope during import/macro resolution
note: `m` could refer to the macro defined here
   |
   |
LL |         macro_rules! m { () => { Right } }
...
...
LL | include!();
   | ---------- in this macro invocation
note: `m` could also refer to the macro defined here
   |
   |
LL |         macro_rules! m { () => {} }
...
...
LL | include!();
   | ---------- in this macro invocation
   = note: this error originates in the macro `gen_invoc` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0659]: `m` is ambiguous
   |
   |
LL |         m!(); //~ ERROR `m` is ambiguous
   |         ^ ambiguous name
...
LL | include!();
   |
   |
   = note: ambiguous because of a conflict between a macro-expanded name and a less macro-expanded name from outer scope during import/macro resolution
note: `m` could refer to the macro defined here
   |
   |
LL |         macro_rules! m { () => { Right } }
...
...
LL | include!();
   | ---------- in this macro invocation
note: `m` could also refer to the macro defined here
   |
   |
LL |         macro_rules! m { () => {} }
...
...
LL | include!();
   | ---------- in this macro invocation
   = note: this error originates in the macro `include` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0659]: `m` is ambiguous
   |
   |
LL |         m!(); //~ ERROR `m` is ambiguous
   |         ^ ambiguous name
...
LL | include!();
   |
   |
   = note: ambiguous because of a conflict between a macro-expanded name and a less macro-expanded name from outer scope during import/macro resolution
note: `m` could refer to the macro defined here
   |
   |
LL |         macro_rules! m { () => { Right } }
...
...
LL | include!();
   | ---------- in this macro invocation
note: `m` could also refer to the macro defined here
   |
   |
LL |         macro_rules! m { () => { Wrong } }
...
...
LL | include!();
   | ---------- in this macro invocation
   = note: this error originates in the macro `include` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0659]: `m` is ambiguous
   |
   |
LL |             m!(); //~ ERROR `m` is ambiguous
   |             ^ ambiguous name
...
LL | include!();
   |
   |
   = note: ambiguous because of a conflict between a macro-expanded name and a less macro-expanded name from outer scope during import/macro resolution
note: `m` could refer to the macro defined here
   |
   |
LL |         macro_rules! m { () => { Right } }
...
...
LL | include!();
   | ---------- in this macro invocation
note: `m` could also refer to the macro defined here
   |
   |
LL |         macro_rules! m { () => { Wrong } }
...
...
LL | include!();
   | ---------- in this macro invocation
   = note: this error originates in the macro `gen_gen_inner_invoc` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0659]: `m` is ambiguous
   |
   |
LL |         macro_rules! gen_invoc { () => { m!() } } //~ ERROR `m` is ambiguous
   |                                          ^ ambiguous name
...
LL | include!();
   |
   |
   = note: ambiguous because of a conflict between a macro-expanded name and a less macro-expanded name from outer scope during import/macro resolution
note: `m` could refer to the macro defined here
   |
   |
LL |         macro_rules! m { () => { Right } }
...
...
LL | include!();
   | ---------- in this macro invocation
note: `m` could also refer to the macro defined here
   |
   |
LL |         macro_rules! m { () => { Wrong } }
...
...
LL | include!();
   | ---------- in this macro invocation
   = note: this error originates in the macro `gen_invoc` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0659]: `m` is ambiguous
   |
   |
LL |         m!(); //~ ERROR `m` is ambiguous
   |         ^ ambiguous name
...
LL | include!();
   |
   |
   = note: ambiguous because of a conflict between a macro-expanded name and a less macro-expanded name from outer scope during import/macro resolution
note: `m` could refer to the macro defined here
   |
   |
LL |         macro_rules! m { () => { Right } }
...
...
LL | include!();
   | ---------- in this macro invocation
note: `m` could also refer to the macro defined here
   |
   |
LL |             macro_rules! m { () => {} }
...
...
LL | include!();
   | ---------- in this macro invocation
   = note: this error originates in the macro `include` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0659]: `m` is ambiguous
   |
   |
LL |         macro_rules! gen_invoc { () => { m!() } } //~ ERROR `m` is ambiguous
   |                                          ^ ambiguous name
...
LL | include!();
   |
   |
   = note: ambiguous because of a conflict between a macro-expanded name and a less macro-expanded name from outer scope during import/macro resolution
note: `m` could refer to the macro defined here
   |
   |
LL |         macro_rules! m { () => { Right } }
...
...
LL | include!();
   | ---------- in this macro invocation
note: `m` could also refer to the macro defined here
   |
   |
LL |             macro_rules! m { () => {} }
...
...
LL | include!();
   | ---------- in this macro invocation
   = note: this error originates in the macro `gen_invoc` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 8 previous errors

For more information about this error, try `rustc --explain E0659`.


------------------------------------------


---- [ui] ui/macros/restricted-shadowing-modern.rs stdout ----
diff of stderr:

7 LL | include!();
9    |
9    |
-    = note: ambiguous because of a conflict between a macro-expanded name and less macro-expanded name from outer scope during import/macro resolution
+    = note: ambiguous because of a conflict between a macro-expanded name and a less macro-expanded name from outer scope during import/macro resolution
11 note: `m` could refer to the macro defined here
13    |


35 LL | include!();
37    |
37    |
-    = note: ambiguous because of a conflict between a macro-expanded name and less macro-expanded name from outer scope during import/macro resolution
+    = note: ambiguous because of a conflict between a macro-expanded name and a less macro-expanded name from outer scope during import/macro resolution
39 note: `m` could refer to the macro defined here
41    |


63 LL | include!();
65    |
65    |
-    = note: ambiguous because of a conflict between a macro-expanded name and less macro-expanded name from outer scope during import/macro resolution
+    = note: ambiguous because of a conflict between a macro-expanded name and a less macro-expanded name from outer scope during import/macro resolution
67 note: `m` could refer to the macro defined here
69    |


91 LL | include!();
93    |
93    |
-    = note: ambiguous because of a conflict between a macro-expanded name and less macro-expanded name from outer scope during import/macro resolution
+    = note: ambiguous because of a conflict between a macro-expanded name and a less macro-expanded name from outer scope during import/macro resolution
95 note: `m` could refer to the macro defined here
97    |


119 LL | include!();
121    |
121    |
-    = note: ambiguous because of a conflict between a macro-expanded name and less macro-expanded name from outer scope during import/macro resolution
+    = note: ambiguous because of a conflict between a macro-expanded name and a less macro-expanded name from outer scope during import/macro resolution
123 note: `m` could refer to the macro defined here
125    |


147 LL | include!();
149    |
149    |
-    = note: ambiguous because of a conflict between a macro-expanded name and less macro-expanded name from outer scope during import/macro resolution
+    = note: ambiguous because of a conflict between a macro-expanded name and a less macro-expanded name from outer scope during import/macro resolution
151 note: `m` could refer to the macro defined here
153    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/restricted-shadowing-modern/restricted-shadowing-modern.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args macros/restricted-shadowing-modern.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/restricted-shadowing-modern.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/restricted-shadowing-modern" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/restricted-shadowing-modern/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0659]: `m` is ambiguous
   |
   |
LL |                 m!(); //~ ERROR `m` is ambiguous
   |                 ^ ambiguous name
...
LL | include!();
   |
   |
   = note: ambiguous because of a conflict between a macro-expanded name and a less macro-expanded name from outer scope during import/macro resolution
note: `m` could refer to the macro defined here
   |
   |
LL |         macro m() { Right }
...
...
LL | include!();
   | ---------- in this macro invocation
note: `m` could also refer to the macro defined here
   |
LL |         macro m() {}
   |         ^^^^^^^^^^^^
...
...
LL | include!();
   | ---------- in this macro invocation
   = note: this error originates in the macro `gen_gen_inner_invoc` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0659]: `m` is ambiguous
   |
   |
LL |             macro gen_invoc() { m!() } //~ ERROR `m` is ambiguous
   |                                 ^ ambiguous name
...
LL | include!();
   |
   |
   = note: ambiguous because of a conflict between a macro-expanded name and a less macro-expanded name from outer scope during import/macro resolution
note: `m` could refer to the macro defined here
   |
   |
LL |         macro m() { Right }
...
...
LL | include!();
   | ---------- in this macro invocation
note: `m` could also refer to the macro defined here
   |
LL |         macro m() {}
   |         ^^^^^^^^^^^^
...
...
LL | include!();
   | ---------- in this macro invocation
   = note: this error originates in the macro `gen_invoc` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0659]: `m` is ambiguous
   |
   |
LL |             m!(); //~ ERROR `m` is ambiguous
   |             ^ ambiguous name
...
LL | include!();
   |
   |
   = note: ambiguous because of a conflict between a macro-expanded name and a less macro-expanded name from outer scope during import/macro resolution
note: `m` could refer to the macro defined here
   |
   |
LL |         macro m() { Right }
...
...
LL | include!();
   | ---------- in this macro invocation
note: `m` could also refer to the macro defined here
   |
LL |         macro m() {}
   |         ^^^^^^^^^^^^
...
...
LL | include!();
   | ---------- in this macro invocation
   = note: this error originates in the macro `include` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0659]: `m` is ambiguous
   |
   |
LL |             m!(); //~ ERROR `m` is ambiguous
   |             ^ ambiguous name
...
LL | include!();
   |
   |
   = note: ambiguous because of a conflict between a macro-expanded name and a less macro-expanded name from outer scope during import/macro resolution
note: `m` could refer to the macro defined here
   |
   |
LL |         macro m() { Right }
...
...
LL | include!();
   | ---------- in this macro invocation
note: `m` could also refer to the macro defined here
   |
   |
LL |         macro m() { Wrong }
---
test result: FAILED. 12221 passed; 10 failed; 110 ignored; 0 measured; 0 filtered out; finished in 117.25s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:11:27
