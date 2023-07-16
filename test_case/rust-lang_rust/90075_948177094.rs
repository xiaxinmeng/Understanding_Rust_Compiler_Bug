plain
.................................................................................................... 4200/12310
.................................................................................................... 4300/12310
.................................................................................................... 4400/12310
.................................................................................................... 4500/12310
................................F.........F.........................ii.............................. 4600/12310
.................................................................................................... 4800/12310
.................................................................................................... 4900/12310
.................................................................................................... 5000/12310
.................................................................................................... 5100/12310
---
.................................................................................................... 8800/12310
.................................................................................................... 8900/12310
.................................................................................................... 9000/12310
.................................................................................................... 9100/12310
...................................................................iiii.iiiii..F.................... 9200/12310
.................................................................................................... 9400/12310
.................................................................................................... 9500/12310
.................................................................................................... 9600/12310
.................................................................................................... 9700/12310
.................................................................................................... 9700/12310
.................................................................................................... 9800/12310
.................................................................................................... 9900/12310
...........................i..ii.i.................................................................. 10000/12310
............................................................................F...FFFFF...F.......F... 10100/12310
.................................................................................................... 10300/12310
.................................................................................................... 10400/12310
.................................................................................................... 10500/12310
.................................................................................................... 10600/12310
---

4 LL |     use core;
5    |         ^^^^ ambiguous name
6    |
-    = note: ambiguous because of multiple potential import sources for `core`
+    = note: ambiguous because of multiple potential import sources
8    = note: `core` could refer to a built-in crate
9    = help: use `::core` to refer to this crate unambiguously
10 note: `core` could also refer to the module imported here

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/issue-57539/issue-57539.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args imports/issue-57539.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/imports/issue-57539.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/issue-57539" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/issue-57539/auxiliary"
------------------------------------------

------------------------------------------
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
stderr:
------------------------------------------
error[E0659]: `core` is ambiguous
  --> /checkout/src/test/ui/imports/issue-57539.rs:4:9
   |
LL |     use core; //~ ERROR `core` is ambiguous
   |         ^^^^ ambiguous name
   |
   = note: ambiguous because of multiple potential import sources
   = note: `core` could refer to a built-in crate
   = help: use `::core` to refer to this crate unambiguously
note: `core` could also refer to the module imported here
   |
LL |     use crate::*;
   |         ^^^^^^^^
   |         ^^^^^^^^
   = help: use `self::core` to refer to this module unambiguously
error: aborting due to previous error

For more information about this error, try `rustc --explain E0659`.


------------------------------------------


---- [ui] ui/imports/issue-56125.rs stdout ----
diff of stderr:

10 LL |     use issue_56125::last_segment::*;
11    |         ^^^^^^^^^^^ ambiguous name
12    |
-    = note: ambiguous because of multiple potential import sources for `issue_56125`
+    = note: ambiguous because of multiple potential import sources
14    = note: `issue_56125` could refer to a crate passed with `--extern`
15    = help: use `::issue_56125` to refer to this crate unambiguously
16 note: `issue_56125` could also refer to the module imported here

26 LL |     use issue_56125::non_last_segment::non_last_segment::*;
27    |         ^^^^^^^^^^^ ambiguous name
28    |
-    = note: ambiguous because of multiple potential import sources for `issue_56125`
+    = note: ambiguous because of multiple potential import sources
30    = note: `issue_56125` could refer to a crate passed with `--extern`
31    = help: use `::issue_56125` to refer to this crate unambiguously
32 note: `issue_56125` could also refer to the module imported here
42 LL |     use issue_56125::*;
43    |         ^^^^^^^^^^^ ambiguous name
44    |
44    |
-    = note: ambiguous because of multiple potential import sources for `issue_56125`
+    = note: ambiguous because of multiple potential import sources
46    = note: `issue_56125` could refer to a crate passed with `--extern`
47    = help: use `::issue_56125` to refer to this crate unambiguously
48 note: `issue_56125` could also refer to the module imported here

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/issue-56125/issue-56125.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args imports/issue-56125.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/imports/issue-56125.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/issue-56125" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "--extern" "issue_56125" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/issue-56125/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0432]: unresolved import `empty::issue_56125`
   |
   |
LL |     use empty::issue_56125; //~ ERROR unresolved import `empty::issue_56125`
   |         ^^^^^^^^^^^^^^^^^^ no `issue_56125` in `m3::empty`

error[E0659]: `issue_56125` is ambiguous
   |
   |
LL |     use issue_56125::last_segment::*;
   |         ^^^^^^^^^^^ ambiguous name
   |
   = note: ambiguous because of multiple potential import sources
   = note: `issue_56125` could refer to a crate passed with `--extern`
   = help: use `::issue_56125` to refer to this crate unambiguously
note: `issue_56125` could also refer to the module imported here
   |
   |
LL |     use issue_56125::last_segment::*;
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = help: use `self::issue_56125` to refer to this module unambiguously

error[E0659]: `issue_56125` is ambiguous
   |
   |
LL |     use issue_56125::non_last_segment::non_last_segment::*;
   |         ^^^^^^^^^^^ ambiguous name
   |
   = note: ambiguous because of multiple potential import sources
   = note: `issue_56125` could refer to a crate passed with `--extern`
   = help: use `::issue_56125` to refer to this crate unambiguously
note: `issue_56125` could also refer to the module imported here
   |
   |
LL |     use issue_56125::non_last_segment::non_last_segment::*;
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = help: use `self::issue_56125` to refer to this module unambiguously

error[E0659]: `issue_56125` is ambiguous
   |
   |
LL |     use issue_56125::*; //~ ERROR `issue_56125` is ambiguous
   |         ^^^^^^^^^^^ ambiguous name
   |
   = note: ambiguous because of multiple potential import sources
   = note: `issue_56125` could refer to a crate passed with `--extern`
   = help: use `::issue_56125` to refer to this crate unambiguously
note: `issue_56125` could also refer to the module imported here
   |
   |
LL |     use issue_56125::*; //~ ERROR `issue_56125` is ambiguous
   |         ^^^^^^^^^^^^^^
   = help: use `self::issue_56125` to refer to this module unambiguously
error: aborting due to 4 previous errors

Some errors have detailed explanations: E0432, E0659.
For more information about an error, try `rustc --explain E0432`.
---

39 LL |         use empty_helper;
40    |             ^^^^^^^^^^^^ ambiguous name
41    |
-    = note: ambiguous because of multiple potential import sources for `empty_helper`
+    = note: ambiguous because of multiple potential import sources
43 note: `empty_helper` could refer to the derive helper attribute defined here
45    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/derive-helper-shadowing/derive-helper-shadowing.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args proc-macro/derive-helper-shadowing.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/derive-helper-shadowing.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/derive-helper-shadowing" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/derive-helper-shadowing/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: cannot use a derive helper attribute through an import
  --> /checkout/src/test/ui/proc-macro/derive-helper-shadowing.rs:42:15
   |
LL |             #[renamed] //~ ERROR cannot use a derive helper attribute through an import
   |
note: the derive helper attribute imported here
  --> /checkout/src/test/ui/proc-macro/derive-helper-shadowing.rs:41:17
   |
   |
LL |             use empty_helper as renamed;
   |                 ^^^^^^^^^^^^^^^^^^^^^^^

error: cannot find attribute `empty_helper` in this scope
   |
   |
LL |             #[derive(GenHelperUse)] //~ ERROR cannot find attribute `empty_helper` in this scope
   |
   = note: consider importing this attribute macro:
           empty_helper
           empty_helper
   = note: this error originates in the derive macro `GenHelperUse` (in Nightly builds, run with -Z macro-backtrace for more info)

error: cannot find attribute `empty_helper` in this scope
   |
   |
LL |         #[empty_helper] //~ ERROR cannot find attribute `empty_helper` in this scope
...
...
LL |             gen_helper_use!();
   |
   = note: consider importing this attribute macro:
           crate::empty_helper
           crate::empty_helper
   = note: this error originates in the macro `gen_helper_use` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0659]: `empty_helper` is ambiguous
   |
   |
LL |         use empty_helper; //~ ERROR `empty_helper` is ambiguous
   |             ^^^^^^^^^^^^ ambiguous name
   |
   = note: ambiguous because of multiple potential import sources
note: `empty_helper` could refer to the derive helper attribute defined here
   |
   |
LL | #[derive(Empty)]
   |          ^^^^^
note: `empty_helper` could also refer to the attribute macro imported here
   |
   |
LL | use test_macros::empty_attr as empty_helper;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = help: use `crate::empty_helper` to refer to this attribute macro unambiguously

error[E0659]: `empty_helper` is ambiguous
   |
   |
LL | #[empty_helper] //~ ERROR `empty_helper` is ambiguous
   |   ^^^^^^^^^^^^ ambiguous name
   |
   = note: ambiguous because of name conflicts with derive helper attribute
note: `empty_helper` could refer to the derive helper attribute defined here
   |
   |
LL | #[derive(Empty)]
   |          ^^^^^
note: `empty_helper` could also refer to the attribute macro imported here
   |
   |
LL | use test_macros::empty_attr as empty_helper;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = help: use `crate::empty_helper` to refer to this attribute macro unambiguously
warning: derive helper attribute is used before it is introduced
  --> /checkout/src/test/ui/proc-macro/derive-helper-shadowing.rs:19:3
   |
   |
LL | #[empty_helper] //~ ERROR `empty_helper` is ambiguous
...
...
LL | #[derive(Empty)]
   |
   = note: `#[warn(legacy_derive_helpers)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #79202 <https://github.com/rust-lang/rust/issues/79202>
---

4 LL |     pub use std::io;
5    |             ^^^ ambiguous name
6    |
-    = note: ambiguous because of multiple potential import sources for `std`
+    = note: ambiguous because of multiple potential import sources
8    = note: `std` could refer to a built-in crate
9    = help: use `::std` to refer to this crate unambiguously
10 note: `std` could also refer to the module defined here

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/uniform-paths/ambiguity-macros-nested/ambiguity-macros-nested.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args rust-2018/uniform-paths/ambiguity-macros-nested.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rust-2018/uniform-paths/ambiguity-macros-nested.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/uniform-paths/ambiguity-macros-nested" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/uniform-paths/ambiguity-macros-nested/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0659]: `std` is ambiguous
  --> /checkout/src/test/ui/rust-2018/uniform-paths/ambiguity-macros-nested.rs:8:13
   |
LL |     pub use std::io;
   |             ^^^ ambiguous name
   |
   = note: ambiguous because of multiple potential import sources
   = note: `std` could refer to a built-in crate
   = help: use `::std` to refer to this crate unambiguously
note: `std` could also refer to the module defined here
   |
LL | /             mod std {
LL | |                 pub struct io;
LL | |             }
LL | |             }
   | |_____________^
...
LL |       m!();
   |       ---- in this macro invocation
   = help: use `self::std` to refer to this module unambiguously
   = note: this error originates in the macro `m` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to previous error

For more information about this error, try `rustc --explain E0659`.

---

4 LL | use std::io;
5    |     ^^^ ambiguous name
6    |
-    = note: ambiguous because of multiple potential import sources for `std`
+    = note: ambiguous because of multiple potential import sources
8    = note: `std` could refer to a built-in crate
9    = help: use `::std` to refer to this crate unambiguously
10 note: `std` could also refer to the module defined here

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/uniform-paths/ambiguity-macros/ambiguity-macros.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args rust-2018/uniform-paths/ambiguity-macros.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rust-2018/uniform-paths/ambiguity-macros.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/uniform-paths/ambiguity-macros" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/uniform-paths/ambiguity-macros/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0659]: `std` is ambiguous
  --> /checkout/src/test/ui/rust-2018/uniform-paths/ambiguity-macros.rs:7:5
   |
LL | use std::io;
   |     ^^^ ambiguous name
   |
   = note: ambiguous because of multiple potential import sources
   = note: `std` could refer to a built-in crate
   = help: use `::std` to refer to this crate unambiguously
note: `std` could also refer to the module defined here
   |
LL | /         mod std {
LL | |             pub struct io;
LL | |         }
LL | |         }
   | |_________^
...
LL |   m!();
   |   ---- in this macro invocation
   = help: use `crate::std` to refer to this module unambiguously
   = note: this error originates in the macro `m` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to previous error

For more information about this error, try `rustc --explain E0659`.

---

4 LL | use std::io;
5    |     ^^^ ambiguous name
6    |
-    = note: ambiguous because of multiple potential import sources for `std`
+    = note: ambiguous because of multiple potential import sources
8    = note: `std` could refer to a built-in crate
9    = help: use `::std` to refer to this crate unambiguously
10 note: `std` could also refer to the module defined here

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/uniform-paths/ambiguity/ambiguity.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args rust-2018/uniform-paths/ambiguity.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rust-2018/uniform-paths/ambiguity.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/uniform-paths/ambiguity" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/uniform-paths/ambiguity/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0659]: `std` is ambiguous
  --> /checkout/src/test/ui/rust-2018/uniform-paths/ambiguity.rs:5:5
   |
LL | use std::io;
   |     ^^^ ambiguous name
   |
   = note: ambiguous because of multiple potential import sources
   = note: `std` could refer to a built-in crate
   = help: use `::std` to refer to this crate unambiguously
note: `std` could also refer to the module defined here
   |
LL | / mod std {
LL | |     pub struct io;
LL | | }
LL | | }
   | |_^
   = help: use `crate::std` to refer to this module unambiguously
error: aborting due to previous error

For more information about this error, try `rustc --explain E0659`.

---

4 LL |     pub use std::io;
5    |             ^^^ ambiguous name
6    |
-    = note: ambiguous because of multiple potential import sources for `std`
+    = note: ambiguous because of multiple potential import sources
8    = note: `std` could refer to a built-in crate
9    = help: use `::std` to refer to this crate unambiguously
10 note: `std` could also refer to the module defined here

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/uniform-paths/ambiguity-nested/ambiguity-nested.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args rust-2018/uniform-paths/ambiguity-nested.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rust-2018/uniform-paths/ambiguity-nested.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/uniform-paths/ambiguity-nested" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/uniform-paths/ambiguity-nested/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0659]: `std` is ambiguous
  --> /checkout/src/test/ui/rust-2018/uniform-paths/ambiguity-nested.rs:8:13
   |
LL |     pub use std::io;
   |             ^^^ ambiguous name
   |
   = note: ambiguous because of multiple potential import sources
   = note: `std` could refer to a built-in crate
   = help: use `::std` to refer to this crate unambiguously
note: `std` could also refer to the module defined here
   |
LL | /     mod std {
LL | |         pub struct io;
LL | |     }
LL | |     }
   | |_____^
   = help: use `self::std` to refer to this module unambiguously
error: aborting due to previous error

For more information about this error, try `rustc --explain E0659`.

---

4 LL |         use sub::bar;
5    |             ^^^ ambiguous name
6    |
-    = note: ambiguous because of multiple potential import sources for `sub`
+    = note: ambiguous because of multiple potential import sources
8 note: `sub` could refer to the module imported here
10    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/uniform-paths/block-scoped-shadow-nested/block-scoped-shadow-nested.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args rust-2018/uniform-paths/block-scoped-shadow-nested.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rust-2018/uniform-paths/block-scoped-shadow-nested.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/uniform-paths/block-scoped-shadow-nested" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/uniform-paths/block-scoped-shadow-nested/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0659]: `sub` is ambiguous
   |
   |
LL |         use sub::bar; //~ ERROR `sub` is ambiguous
   |             ^^^ ambiguous name
   |
   = note: ambiguous because of multiple potential import sources
note: `sub` could refer to the module imported here
   |
   |
LL |     use my::sub;
   |         ^^^^^^^
note: `sub` could also refer to the module defined here
   |
LL | / mod sub {
LL | |     pub fn bar() {}
LL | | }
LL | | }
   | |_^
   = help: use `crate::sub` to refer to this module unambiguously
error: aborting due to previous error

For more information about this error, try `rustc --explain E0659`.

---

4 LL |     use Foo::*;
5    |         ^^^ ambiguous name
6    |
-    = note: ambiguous because of multiple potential import sources for `Foo`
+    = note: ambiguous because of multiple potential import sources
8 note: `Foo` could refer to the enum defined here
10    |

23 LL |     use std as foo;
24    |         ^^^ ambiguous name
24    |         ^^^ ambiguous name
25    |
-    = note: ambiguous because of multiple potential import sources for `std`
+    = note: ambiguous because of multiple potential import sources
27 note: `std` could refer to the enum defined here
29    |

42 LL |     use std as foo;
43    |         ^^^ ambiguous name
43    |         ^^^ ambiguous name
44    |
-    = note: ambiguous because of multiple potential import sources for `std`
+    = note: ambiguous because of multiple potential import sources
46 note: `std` could refer to the function defined here
48    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/uniform-paths/block-scoped-shadow/block-scoped-shadow.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args rust-2018/uniform-paths/block-scoped-shadow.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rust-2018/uniform-paths/block-scoped-shadow.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/uniform-paths/block-scoped-shadow" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/uniform-paths/block-scoped-shadow/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0659]: `Foo` is ambiguous
   |
LL |     use Foo::*;
   |         ^^^ ambiguous name
   |
   |
   = note: ambiguous because of multiple potential import sources
note: `Foo` could refer to the enum defined here
   |
   |
LL |     enum Foo { A, B }
   |     ^^^^^^^^^^^^^^^^^
note: `Foo` could also refer to the enum defined here
   |
LL | enum Foo {}
   | ^^^^^^^^^^^
   | ^^^^^^^^^^^
   = help: use `crate::Foo` to refer to this enum unambiguously
error[E0659]: `std` is ambiguous
  --> /checkout/src/test/ui/rust-2018/uniform-paths/block-scoped-shadow.rs:18:9
   |
LL |     use std as foo;
LL |     use std as foo;
   |         ^^^ ambiguous name
   |
   = note: ambiguous because of multiple potential import sources
note: `std` could refer to the enum defined here
   |
LL |     enum std {}
   |     ^^^^^^^^^^^
   |     ^^^^^^^^^^^
note: `std` could also refer to the struct defined here
   |
LL | struct std;
   | ^^^^^^^^^^^
   | ^^^^^^^^^^^
   = help: use `crate::std` to refer to this struct unambiguously
error[E0659]: `std` is ambiguous
  --> /checkout/src/test/ui/rust-2018/uniform-paths/block-scoped-shadow.rs:18:9
   |
LL |     use std as foo;
LL |     use std as foo;
   |         ^^^ ambiguous name
   |
   = note: ambiguous because of multiple potential import sources
note: `std` could refer to the function defined here
   |
LL |     fn std() {}
   |     ^^^^^^^^^^^
   |     ^^^^^^^^^^^
note: `std` could also refer to the unit struct defined here
   |
LL | struct std;
   | ^^^^^^^^^^^
   | ^^^^^^^^^^^
   = help: use `crate::std` to refer to this unit struct unambiguously
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0659`.

---

16 LL |         use legacy_macro as _;
17    |             ^^^^^^^^^^^^ ambiguous name
18    |
-    = note: ambiguous because of multiple potential import sources for `legacy_macro`
+    = note: ambiguous because of multiple potential import sources
20 note: `legacy_macro` could refer to the macro defined here
22    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/uniform-paths/macro-rules/macro-rules.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args rust-2018/uniform-paths/macro-rules.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rust-2018/uniform-paths/macro-rules.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/uniform-paths/macro-rules" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/uniform-paths/macro-rules/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0364]: `legacy_macro` is private, and cannot be re-exported
   |
   |
LL |     pub use legacy_macro as _; //~ ERROR `legacy_macro` is private, and cannot be re-exported
   |
   |
note: consider marking `legacy_macro` as `pub` in the imported module
   |
   |
LL |     pub use legacy_macro as _; //~ ERROR `legacy_macro` is private, and cannot be re-exported


error[E0659]: `legacy_macro` is ambiguous
   |
   |
LL |         use legacy_macro as _; //~ ERROR `legacy_macro` is ambiguous
   |             ^^^^^^^^^^^^ ambiguous name
   |
   = note: ambiguous because of multiple potential import sources
note: `legacy_macro` could refer to the macro defined here
   |
   |
LL |         macro_rules! legacy_macro { () => () }
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: `legacy_macro` could also refer to the macro defined here
   |
   |
LL |     macro legacy_macro() {}
   |     ^^^^^^^^^^^^^^^^^^^^^^^
   = help: use `self::legacy_macro` to refer to this macro unambiguously
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0364, E0659.
For more information about an error, try `rustc --explain E0364`.
---

4 LL | use issue_56596;
5    |     ^^^^^^^^^^^ ambiguous name
6    |
-    = note: ambiguous because of multiple potential import sources for `issue_56596`
+    = note: ambiguous because of multiple potential import sources
8    = note: `issue_56596` could refer to a crate passed with `--extern`
9    = help: use `::issue_56596` to refer to this crate unambiguously
10 note: `issue_56596` could also refer to the module imported here

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/uniform-paths/issue-56596/issue-56596.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args rust-2018/uniform-paths/issue-56596.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rust-2018/uniform-paths/issue-56596.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/uniform-paths/issue-56596" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "--extern" "issue_56596" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/uniform-paths/issue-56596/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0659]: `issue_56596` is ambiguous
   |
   |
LL | use issue_56596; //~ ERROR `issue_56596` is ambiguous
   |     ^^^^^^^^^^^ ambiguous name
   |
   = note: ambiguous because of multiple potential import sources
   = note: `issue_56596` could refer to a crate passed with `--extern`
   = help: use `::issue_56596` to refer to this crate unambiguously
note: `issue_56596` could also refer to the module imported here
   |
   |
LL | use m::*;
   |     ^^^^
   = help: use `crate::issue_56596` to refer to this module unambiguously
error: aborting due to previous error

For more information about this error, try `rustc --explain E0659`.

---
test result: FAILED. 12182 passed; 11 failed; 117 ignored; 0 measured; 0 filtered out; finished in 131.29s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:03
