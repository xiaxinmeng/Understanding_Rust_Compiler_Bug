plain
.................................................................................................... 4100/12338
.................................................................................................... 4200/12338
.................................................................................................... 4300/12338
.................................................................................................... 4400/12338
..............................................F..................................................... 4500/12338
.........................................F..........F.....F.....F...........F...F...ii.............. 4600/12338
.................................................................................................... 4800/12338
.................................................................................................... 4900/12338
.................................................................................................... 5000/12338
.................................................................................................... 5100/12338
---

4 LL |     let v = f;
5    |             ^ ambiguous name
6    |
-    = note: ambiguous because of name conflicts in the same module due to glob import
+    = note: ambiguous because of multiple glob imports of name in the same module
8 note: `f` could refer to the function imported here
9   --> $DIR/ambiguity-item.rs:6:5

24 LL |         f => {}
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
25    |         ^ ambiguous name
25    |         ^ ambiguous name
26    |
-    = note: ambiguous because of name conflicts in the same module due to glob import
+    = note: ambiguous because of multiple glob imports of name in the same module
28 note: `f` could refer to the function imported here
29   --> $DIR/ambiguity-item.rs:6:5


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binding/ambiguity-item/ambiguity-item.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binding/ambiguity-item/ambiguity-item.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args binding/ambiguity-item.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/binding/ambiguity-item.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binding/ambiguity-item" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binding/ambiguity-item/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0659]: `f` is ambiguous
   |
   |
LL |     let v = f; //~ ERROR `f` is ambiguous
   |             ^ ambiguous name
   |
   = note: ambiguous because of multiple glob imports of name in the same module
note: `f` could refer to the function imported here
   |
   |
LL | use m::*;
   |     ^^^^
   = help: consider adding an explicit import of `f` to disambiguate
note: `f` could also refer to the function imported here
   |
   |
LL | use n::*; // OK, no conflict with `use m::*;`
   |     ^^^^
   = help: consider adding an explicit import of `f` to disambiguate

error[E0659]: `f` is ambiguous
   |
   |
LL |         f => {} //~ ERROR `f` is ambiguous
   |         ^ ambiguous name
   |
   = note: ambiguous because of multiple glob imports of name in the same module
note: `f` could refer to the function imported here
   |
   |
LL | use m::*;
   |     ^^^^
   = help: consider adding an explicit import of `f` to disambiguate
note: `f` could also refer to the function imported here
   |
   |
LL | use n::*; // OK, no conflict with `use m::*;`
   |     ^^^^
   = help: consider adding an explicit import of `f` to disambiguate
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0659`.

---
diff of stderr:

1 error[E0659]: `main` is ambiguous
2    |
-    = note: ambiguous because of name conflicts in the same module due to glob import
+    = note: ambiguous because of multiple glob imports of name in the same module
4 note: `main` could refer to the function imported here
6    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/entry-point/imported_main_conflict/imported_main_conflict.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args entry-point/imported_main_conflict.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/entry-point/imported_main_conflict.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/entry-point/imported_main_conflict" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/entry-point/imported_main_conflict/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0659]: `main` is ambiguous
   |
   = note: ambiguous because of multiple glob imports of name in the same module
note: `main` could refer to the function imported here
   |
   |
LL | use m1::*;
   = help: consider adding an explicit import of `main` to disambiguate
   = help: consider adding an explicit import of `main` to disambiguate
note: `main` could also refer to the function imported here
   |
   |
LL | use m2::*;
   = help: consider adding an explicit import of `main` to disambiguate

error: aborting due to previous error

---

---- [ui] ui/error-codes/E0659.rs stdout ----
diff of stderr:

4 LL |     collider::foo();
5    |               ^^^ ambiguous name
6    |
-    = note: ambiguous because of name conflicts in the same module due to glob import
+    = note: ambiguous because of multiple glob imports of name in the same module
8 note: `foo` could refer to the function imported here
9   --> $DIR/E0659.rs:10:13


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0659/E0659.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0659/E0659.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args error-codes/E0659.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0659.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0659" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0659/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0659]: `foo` is ambiguous
   |
   |
LL |     collider::foo(); //~ ERROR E0659
   |               ^^^ ambiguous name
   |
   = note: ambiguous because of multiple glob imports of name in the same module
note: `foo` could refer to the function imported here
   |
   |
LL |     pub use moon::*;
   = help: consider adding an explicit import of `foo` to disambiguate
   = help: consider adding an explicit import of `foo` to disambiguate
note: `foo` could also refer to the function imported here
   |
   |
LL |     pub use earth::*;
   = help: consider adding an explicit import of `foo` to disambiguate

error: aborting due to previous error

---

14 LL |     use self::foo::bar;
15    |               ^^^ ambiguous name
16    |
-    = note: ambiguous because of name conflicts in the same module due to glob import
+    = note: ambiguous because of multiple glob imports of name in the same module
18 note: `foo` could refer to the module imported here
20    |

34 LL |     f::foo();
35    |        ^^^ ambiguous name
35    |        ^^^ ambiguous name
36    |
-    = note: ambiguous because of name conflicts in the same module due to glob import
+    = note: ambiguous because of multiple glob imports of name in the same module
38 note: `foo` could refer to the function imported here
40    |

54 LL |         foo::bar();
55    |         ^^^ ambiguous name
55    |         ^^^ ambiguous name
56    |
-    = note: ambiguous because of name conflicts in the same module due to glob import
+    = note: ambiguous because of multiple glob imports of name in the same module
58 note: `foo` could refer to the module imported here
60    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/duplicate/duplicate.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args imports/duplicate.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/imports/duplicate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/duplicate" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/duplicate/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0252]: the name `foo` is defined multiple times
  --> /checkout/src/test/ui/imports/duplicate.rs:15:9
   |
LL |     use a::foo;
   |         ------ previous import of the value `foo` here
LL |     use a::foo; //~ ERROR the name `foo` is defined multiple times
   |         ^^^^^^ `foo` reimported here
   |
   = note: `foo` must be defined only once in the value namespace of this module

error[E0659]: `foo` is ambiguous
   |
   |
LL |     use self::foo::bar; //~ ERROR `foo` is ambiguous
   |               ^^^ ambiguous name
   |
   = note: ambiguous because of multiple glob imports of name in the same module
note: `foo` could refer to the module imported here
   |
LL |     use self::m1::*;
   |         ^^^^^^^^^^^
   = help: consider adding an explicit import of `foo` to disambiguate
   = help: consider adding an explicit import of `foo` to disambiguate
note: `foo` could also refer to the module imported here
  --> /checkout/src/test/ui/imports/duplicate.rs:44:9
   |
LL |     use self::m2::*;
   |         ^^^^^^^^^^^
   = help: consider adding an explicit import of `foo` to disambiguate

error[E0659]: `foo` is ambiguous
   |
   |
LL |     f::foo(); //~ ERROR `foo` is ambiguous
   |        ^^^ ambiguous name
   |
   = note: ambiguous because of multiple glob imports of name in the same module
note: `foo` could refer to the function imported here
   |
   |
LL |     pub use a::*;
   = help: consider adding an explicit import of `foo` to disambiguate
   = help: consider adding an explicit import of `foo` to disambiguate
note: `foo` could also refer to the function imported here
   |
   |
LL |     pub use b::*;
   = help: consider adding an explicit import of `foo` to disambiguate


error[E0659]: `foo` is ambiguous
   |
   |
LL |         foo::bar(); //~ ERROR `foo` is ambiguous
   |         ^^^ ambiguous name
   |
   = note: ambiguous because of multiple glob imports of name in the same module
note: `foo` could refer to the module imported here
   |
LL |     use self::m1::*;
   |         ^^^^^^^^^^^
   = help: consider adding an explicit import of `foo` to disambiguate
---

10 LL |     mac!();
11    |     ^^^ ambiguous name
12    |
-    = note: ambiguous because of macro name conflicts with non-`macro_rules` name from another module
+    = note: ambiguous because of `macro_rules` name conflicts with non-`macro_rules` name from another module
14 note: `mac` could refer to the macro defined here
16    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/issue-53269/issue-53269.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args imports/issue-53269.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/imports/issue-53269.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/issue-53269" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/issue-53269/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0432]: unresolved import `nonexistent_module`
  --> /checkout/src/test/ui/imports/issue-53269.rs:6:9
   |
LL |     use nonexistent_module::mac; //~ ERROR unresolved import `nonexistent_module`
   |         ^^^^^^^^^^^^^^^^^^ maybe a missing crate `nonexistent_module`?

error[E0659]: `mac` is ambiguous
   |
   |
LL |     mac!(); //~ ERROR `mac` is ambiguous
   |     ^^^ ambiguous name
   |
   = note: ambiguous because of `macro_rules` name conflicts with non-`macro_rules` name from another module
note: `mac` could refer to the macro defined here
   |
   |
LL | macro_rules! mac { () => () }
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: `mac` could also refer to the unresolved item imported here
   |
   |
LL |     use nonexistent_module::mac; //~ ERROR unresolved import `nonexistent_module`
   |         ^^^^^^^^^^^^^^^^^^^^^^^
   = help: use `self::mac` to refer to this unresolved item unambiguously
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0432, E0659.
For more information about an error, try `rustc --explain E0432`.
---

4 LL |     use m::S;
5    |            ^ ambiguous name
6    |
-    = note: ambiguous because of name conflicts in the same module due to glob import
+    = note: ambiguous because of multiple glob imports of name in the same module
8 note: `S` could refer to the struct imported here
10    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/issue-55884-1/issue-55884-1.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args imports/issue-55884-1.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/imports/issue-55884-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/issue-55884-1" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/issue-55884-1/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0659]: `S` is ambiguous
   |
   |
LL |     use m::S; //~ ERROR `S` is ambiguous
   |            ^ ambiguous name
   |
   = note: ambiguous because of multiple glob imports of name in the same module
note: `S` could refer to the struct imported here
   |
   |
LL |     pub use self::m1::*;
   |             ^^^^^^^^^^^
   = help: consider adding an explicit import of `S` to disambiguate
note: `S` could also refer to the struct imported here
   |
   |
LL |     pub use self::m2::*;
   |             ^^^^^^^^^^^
   = help: consider adding an explicit import of `S` to disambiguate
error: aborting due to previous error

For more information about this error, try `rustc --explain E0659`.


------------------------------------------


---- [ui] ui/imports/local-modularized-tricky-fail-1.rs stdout ----
diff of stderr:

4 LL | exported!();
5    | ^^^^^^^^ ambiguous name
6    |
-    = note: ambiguous because of name conflicts with macro-expanded name in the same module during import/macro resolution due to glob import
+    = note: ambiguous because of conflicts between name from glob import and macro-expanded name in the same module
8 note: `exported` could refer to the macro defined here
10    |


29 LL | exported!();
30    | ^^^^^^^^ ambiguous name
31    |
-    = note: ambiguous because of name conflicts with macro-expanded name in the same module during import/macro resolution due to glob import
+    = note: ambiguous because of conflicts between name from glob import and macro-expanded name in the same module
33 note: `exported` could refer to the macro defined here
35    |


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
   = note: ambiguous because of conflicts between name from glob import and macro-expanded name in the same module
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
   = note: ambiguous because of conflicts between name from glob import and macro-expanded name in the same module
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
   = note: ambiguous because of macro-expanded name conflicts with less macro-expanded name from outer scope during import/macro resolution
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
   = note: ambiguous because of macro-expanded name conflicts with less macro-expanded name from outer scope during import/macro resolution
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


---- [ui] ui/imports/rfc-1560-warning-cycle.rs stdout ----
diff of stderr:

4 LL |         fn f(_: Foo) {}
5    |                 ^^^ ambiguous name
6    |
-    = note: ambiguous because of name conflicts in the same module due to glob import
+    = note: ambiguous because of multiple glob imports of name in the same module
8 note: `Foo` could refer to the struct imported here
9   --> $DIR/rfc-1560-warning-cycle.rs:7:13


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/rfc-1560-warning-cycle/rfc-1560-warning-cycle.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/rfc-1560-warning-cycle/rfc-1560-warning-cycle.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args imports/rfc-1560-warning-cycle.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/imports/rfc-1560-warning-cycle.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/rfc-1560-warning-cycle" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/rfc-1560-warning-cycle/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0659]: `Foo` is ambiguous
   |
   |
LL |         fn f(_: Foo) {} //~ ERROR `Foo` is ambiguous
   |                 ^^^ ambiguous name
   |
   = note: ambiguous because of multiple glob imports of name in the same module
note: `Foo` could refer to the struct imported here
   |
LL |         use *;
   |             ^
   = help: consider adding an explicit import of `Foo` to disambiguate
   = help: consider adding an explicit import of `Foo` to disambiguate
note: `Foo` could also refer to the struct imported here
  --> /checkout/src/test/ui/imports/rfc-1560-warning-cycle.rs:8:13
   |
LL |         use bar::*;
   = help: consider adding an explicit import of `Foo` to disambiguate

error: aborting due to previous error

---

---- [ui] ui/imports/macro-paths.rs stdout ----
diff of stderr:

4 LL |     bar::m! {
5    |     ^^^ ambiguous name
6    |
-    = note: ambiguous because of name conflicts with macro-expanded name in the same module during import/macro resolution due to glob import
+    = note: ambiguous because of conflicts between name from glob import and macro-expanded name in the same module
8 note: `bar` could refer to the module defined here
10    |


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
   = note: ambiguous because of conflicts between name from glob import and macro-expanded name in the same module
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
   = note: ambiguous because of macro-expanded name conflicts with less macro-expanded name from outer scope during import/macro resolution
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

4 LL |     m! {
5    |     ^ ambiguous name
6    |
-    = note: ambiguous because of name conflicts with macro-expanded name in the same module during import/macro resolution due to glob import
+    = note: ambiguous because of conflicts between name from glob import and macro-expanded name in the same module
8 note: `m` could refer to the macro imported here
10    |

23 LL |     m! {
24    |     ^ ambiguous name
24    |     ^ ambiguous name
25    |
-    = note: ambiguous because of name conflicts with macro-expanded name in the same module during import/macro resolution due to glob import
+    = note: ambiguous because of conflicts between name from glob import and macro-expanded name in the same module
27 note: `m` could refer to the macro imported here
29    |


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
   = note: ambiguous because of conflicts between name from glob import and macro-expanded name in the same module
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
   = note: ambiguous because of conflicts between name from glob import and macro-expanded name in the same module
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
   = note: ambiguous because of macro-expanded name conflicts with less macro-expanded name from outer scope during import/macro resolution
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

---

4 LL |         m!()
5    |         ^ ambiguous name
6    |
-    = note: ambiguous because of macro name conflicts with non-`macro_rules` name from another module
+    = note: ambiguous because of `macro_rules` name conflicts with non-`macro_rules` name from another module
8 note: `m` could refer to the macro defined here
9   --> $DIR/ambiguity-legacy-vs-modern.rs:26:5

22 LL |     m!()
23    |     ^ ambiguous name
24    |
24    |
-    = note: ambiguous because of macro name conflicts with non-`macro_rules` name from another module
+    = note: ambiguous because of `macro_rules` name conflicts with non-`macro_rules` name from another module
26 note: `m` could refer to the macro defined here
27   --> $DIR/ambiguity-legacy-vs-modern.rs:40:9


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/ambiguity-legacy-vs-modern/ambiguity-legacy-vs-modern.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/ambiguity-legacy-vs-modern/ambiguity-legacy-vs-modern.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args macros/ambiguity-legacy-vs-modern.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/ambiguity-legacy-vs-modern.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/ambiguity-legacy-vs-modern" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/ambiguity-legacy-vs-modern/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0659]: `m` is ambiguous
   |
   |
LL |         m!() //~ ERROR `m` is ambiguous
   |         ^ ambiguous name
   |
   = note: ambiguous because of `macro_rules` name conflicts with non-`macro_rules` name from another module
note: `m` could refer to the macro defined here
   |
   |
LL |     macro_rules! m { () => (()) }
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: `m` could also refer to the macro defined here
   |
   |
LL |         macro m() { 0 }


error[E0659]: `m` is ambiguous
   |
   |
LL |     m!() //~ ERROR `m` is ambiguous
   |     ^ ambiguous name
   |
   = note: ambiguous because of `macro_rules` name conflicts with non-`macro_rules` name from another module
note: `m` could refer to the macro defined here
   |
   |
LL |         macro_rules! m { () => (()) }
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: `m` could also refer to the macro defined here
   |
   |
LL |     macro m() { 0 }

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0659`.
---
test result: FAILED. 12217 passed; 11 failed; 110 ignored; 0 measured; 0 filtered out; finished in 139.25s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:30
