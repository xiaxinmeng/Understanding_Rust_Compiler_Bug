plain
.................................................................................................... 4100/12341
.................................................................................................... 4200/12341
.................................................................................................... 4300/12341
.................................................................................................... 4400/12341
................................................F................F.F................................ 4500/12341
..........................................F....F...........F......F.........FF........ii..F......... 4600/12341
.................................................................................................... 4800/12341
.................................................................................................... 4900/12341
.................................................................................................... 5000/12341
.................................................................................................... 5100/12341
---
.......i............................................................................................ 6800/12341
........i........................................................................................... 6900/12341
.......................................i.......................................................F.... 7000/12341
............................................................ii...................................... 7100/12341
................i...............................................................F..F................ 7200/12341
.................F.F...F............................................................................ 7300/12341
.................................................................................................... 7500/12341
..............ii................i....i..ii.......................................................... 7600/12341
.................................................................................................... 7700/12341
.................................................................................................... 7800/12341
---
.................................................................................................... 8800/12341
.................................................................................................... 8900/12341
.................................................................................................... 9000/12341
.................................................................................................... 9100/12341
.................................F............................................................iiii.i 9200/12341
iiii..F.............F....................F...........................ii...............i............. 9300/12341
.......F............................................................................................ 9400/12341
.................................................................................................... 9600/12341
.................................................................................................... 9700/12341
.................................................................................................... 9800/12341
.................................................................................................... 9900/12341
---

4 LL |     let v = f;
5    |             ^ ambiguous name
6    |
-    = note: ambiguous because of multiple glob imports of name in the same module
+    = note: ambiguous because of multiple glob imports of a name in the same module
8 note: `f` could refer to the function imported here
9   --> $DIR/ambiguity-item.rs:6:5

24 LL |         f => {}
25    |         ^ ambiguous name
26    |
26    |
-    = note: ambiguous because of multiple glob imports of name in the same module
+    = note: ambiguous because of multiple glob imports of a name in the same module
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
   = note: ambiguous because of multiple glob imports of a name in the same module
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
   = note: ambiguous because of multiple glob imports of a name in the same module
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
-    = note: ambiguous because of multiple glob imports of name in the same module
+    = note: ambiguous because of multiple glob imports of a name in the same module
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
   = note: ambiguous because of multiple glob imports of a name in the same module
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
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

4 LL |     collider::foo();
5    |               ^^^ ambiguous name
6    |
-    = note: ambiguous because of multiple glob imports of name in the same module
+    = note: ambiguous because of multiple glob imports of a name in the same module
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
   = note: ambiguous because of multiple glob imports of a name in the same module
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
-    = note: ambiguous because of multiple glob imports of name in the same module
+    = note: ambiguous because of multiple glob imports of a name in the same module
18 note: `foo` could refer to the module imported here
20    |

34 LL |     f::foo();
35    |        ^^^ ambiguous name
35    |        ^^^ ambiguous name
36    |
-    = note: ambiguous because of multiple glob imports of name in the same module
+    = note: ambiguous because of multiple glob imports of a name in the same module
38 note: `foo` could refer to the function imported here
40    |

54 LL |         foo::bar();
55    |         ^^^ ambiguous name
55    |         ^^^ ambiguous name
56    |
-    = note: ambiguous because of multiple glob imports of name in the same module
+    = note: ambiguous because of multiple glob imports of a name in the same module
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
   = note: ambiguous because of multiple glob imports of a name in the same module
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
   = note: ambiguous because of multiple glob imports of a name in the same module
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
   = note: ambiguous because of multiple glob imports of a name in the same module
note: `foo` could refer to the module imported here
   |
LL |     use self::m1::*;
   |         ^^^^^^^^^^^
   = help: consider adding an explicit import of `foo` to disambiguate
---

---- [ui] ui/imports/glob-shadowing.rs stdout ----
diff of stderr:

4 LL |         let x = env!("PATH");
5    |                 ^^^ ambiguous name
6    |
-    = note: ambiguous because of name conflicts with outer scope during import/macro resolution due to glob import
+    = note: ambiguous because of a name conflict with an outer scope during import/macro resolution due to glob import
8    = note: `env` could refer to a macro from prelude
9 note: `env` could also refer to the macro imported here


20 LL |             let x = env!("PATH");
21    |                     ^^^ ambiguous name
22    |
-    = note: ambiguous because of name conflicts with outer scope during import/macro resolution due to glob import
+    = note: ambiguous because of a name conflict with an outer scope during import/macro resolution due to glob import
24    = note: `env` could refer to a macro from prelude
25 note: `env` could also refer to the macro imported here


35 LL |             let x = fenv!();
36    |                     ^^^^ ambiguous name
37    |
-    = note: ambiguous because of name conflicts with outer scope during import/macro resolution due to glob import
+    = note: ambiguous because of a name conflict with an outer scope during import/macro resolution due to glob import
39 note: `fenv` could refer to the macro imported here
41    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/glob-shadowing/glob-shadowing.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args imports/glob-shadowing.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/imports/glob-shadowing.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/glob-shadowing" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/glob-shadowing/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0659]: `env` is ambiguous
   |
   |
LL |         let x = env!("PATH"); //~ ERROR `env` is ambiguous
   |                 ^^^ ambiguous name
   |
   = note: ambiguous because of a name conflict with an outer scope during import/macro resolution due to glob import
   = note: `env` could refer to a macro from prelude
note: `env` could also refer to the macro imported here
   |
LL |     use m::*;
   |         ^^^^
   = help: consider adding an explicit import of `env` to disambiguate
   = help: consider adding an explicit import of `env` to disambiguate
   = help: or use `self::env` to refer to this macro unambiguously

error[E0659]: `env` is ambiguous
   |
   |
LL |             let x = env!("PATH"); //~ ERROR `env` is ambiguous
   |                     ^^^ ambiguous name
   |
   = note: ambiguous because of a name conflict with an outer scope during import/macro resolution due to glob import
   = note: `env` could refer to a macro from prelude
note: `env` could also refer to the macro imported here
   |
LL |         use m::*;
   |             ^^^^
   = help: consider adding an explicit import of `env` to disambiguate
   = help: consider adding an explicit import of `env` to disambiguate

error[E0659]: `fenv` is ambiguous
   |
   |
LL |             let x = fenv!(); //~ ERROR `fenv` is ambiguous
   |                     ^^^^ ambiguous name
   |
   = note: ambiguous because of a name conflict with an outer scope during import/macro resolution due to glob import
note: `fenv` could refer to the macro imported here
   |
LL |         use m::*;
   |             ^^^^
   |             ^^^^
   = help: consider adding an explicit import of `fenv` to disambiguate
note: `fenv` could also refer to the macro defined here
   |
   |
LL |     pub macro fenv($e: expr) { $e }
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = help: use `self::fenv` to refer to this macro unambiguously
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0659`.

---

15 LL |         Vec::panic!();
16    |         ^^^ ambiguous name
17    |
-    = note: ambiguous because of macro-expanded name conflicts with less macro-expanded name from outer scope during import/macro resolution
+    = note: ambiguous because of a conflict between a macro-expanded name and less macro-expanded name from outer scope during import/macro resolution
19 note: `Vec` could refer to the crate imported here
21    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/extern-prelude-extern-crate-restricted-shadowing/extern-prelude-extern-crate-restricted-shadowing.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args imports/extern-prelude-extern-crate-restricted-shadowing.rs`

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
   = note: ambiguous because of a conflict between a macro-expanded name and less macro-expanded name from outer scope during import/macro resolution
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

10 LL |     mac!();
11    |     ^^^ ambiguous name
12    |
-    = note: ambiguous because of `macro_rules` name conflicts with non-`macro_rules` name from another module
+    = note: ambiguous because of a name conflict between a `macro_rules` and a non-`macro_rules` name from another module
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
   = note: ambiguous because of a name conflict between a `macro_rules` and a non-`macro_rules` name from another module
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
-    = note: ambiguous because of multiple glob imports of name in the same module
+    = note: ambiguous because of multiple glob imports of a name in the same module
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
   = note: ambiguous because of multiple glob imports of a name in the same module
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
-    = note: ambiguous because of conflicts between name from glob import and macro-expanded name in the same module
+    = note: ambiguous because of a conflict between a name from glob import and macro-expanded name in the same module
8 note: `exported` could refer to the macro defined here
10    |


29 LL | exported!();
30    | ^^^^^^^^ ambiguous name
31    |
-    = note: ambiguous because of conflicts between name from glob import and macro-expanded name in the same module
+    = note: ambiguous because of a conflict between a name from glob import and macro-expanded name in the same module
33 note: `exported` could refer to the macro defined here
35    |

54 LL |     panic!();
55    |     ^^^^^ ambiguous name
55    |     ^^^^^ ambiguous name
56    |
-    = note: ambiguous because of macro-expanded name conflicts with less macro-expanded name from outer scope during import/macro resolution
+    = note: ambiguous because of a conflict between a macro-expanded name and less macro-expanded name from outer scope during import/macro resolution
58    = note: `panic` could refer to a macro from prelude
59 note: `panic` could also refer to the macro defined here


75 LL | include!();
76    | ^^^^^^^ ambiguous name
77    |
-    = note: ambiguous because of macro-expanded name conflicts with less macro-expanded name from outer scope during import/macro resolution
+    = note: ambiguous because of a conflict between a macro-expanded name and less macro-expanded name from outer scope during import/macro resolution
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
   = note: ambiguous because of a conflict between a macro-expanded name and less macro-expanded name from outer scope during import/macro resolution
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
   = note: ambiguous because of a conflict between a macro-expanded name and less macro-expanded name from outer scope during import/macro resolution
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
-    = note: ambiguous because of multiple glob imports of name in the same module
+    = note: ambiguous because of multiple glob imports of a name in the same module
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
   = note: ambiguous because of multiple glob imports of a name in the same module
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

4 LL |     m! {
5    |     ^ ambiguous name
6    |
-    = note: ambiguous because of conflicts between name from glob import and macro-expanded name in the same module
+    = note: ambiguous because of a conflict between a name from glob import and macro-expanded name in the same module
8 note: `m` could refer to the macro imported here
10    |

23 LL |     m! {
24    |     ^ ambiguous name
24    |     ^ ambiguous name
25    |
-    = note: ambiguous because of conflicts between name from glob import and macro-expanded name in the same module
+    = note: ambiguous because of a conflict between a name from glob import and macro-expanded name in the same module
27 note: `m` could refer to the macro imported here
29    |

42 LL |         m! {
43    |         ^ ambiguous name
43    |         ^ ambiguous name
44    |
-    = note: ambiguous because of macro-expanded name conflicts with less macro-expanded name from outer scope during import/macro resolution
+    = note: ambiguous because of a conflict between a macro-expanded name and less macro-expanded name from outer scope during import/macro resolution
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
   = note: ambiguous because of a conflict between a macro-expanded name and less macro-expanded name from outer scope during import/macro resolution
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


---- [ui] ui/imports/shadow_builtin_macros.rs stdout ----
diff of stderr:

4 LL |     fn f() { panic!(); }
5    |              ^^^^^ ambiguous name
6    |
-    = note: ambiguous because of name conflicts with outer scope during import/macro resolution due to glob import
+    = note: ambiguous because of a name conflict with an outer scope during import/macro resolution due to glob import
8    = note: `panic` could refer to a macro from prelude
9 note: `panic` could also refer to the macro imported here

20 LL |     panic!();
21    |     ^^^^^ ambiguous name
22    |
22    |
-    = note: ambiguous because of macro-expanded name conflicts with less macro-expanded name from outer scope during import/macro resolution
+    = note: ambiguous because of a conflict between a macro-expanded name and less macro-expanded name from outer scope during import/macro resolution
24    = note: `panic` could refer to a macro from prelude
25 note: `panic` could also refer to the macro defined here

38 LL |     n!();
39    |     ^ ambiguous name
40    |
40    |
-    = note: ambiguous because of name conflicts with outer scope during import/macro resolution due to glob import
+    = note: ambiguous because of a name conflict with an outer scope during import/macro resolution due to glob import
42 note: `n` could refer to the macro imported here
44    |


58 LL |     fn f() { panic!(); }
59    |              ^^^^^ ambiguous name
60    |
-    = note: ambiguous because of macro-expanded name conflicts with less macro-expanded name from outer scope during import/macro resolution
+    = note: ambiguous because of a conflict between a macro-expanded name and less macro-expanded name from outer scope during import/macro resolution
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
   = note: ambiguous because of a conflict between a macro-expanded name and less macro-expanded name from outer scope during import/macro resolution
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
   = note: ambiguous because of a conflict between a macro-expanded name and less macro-expanded name from outer scope during import/macro resolution
   = note: `panic` could refer to a macro from prelude
note: `panic` could also refer to the macro imported here
   |
   |
LL |     ::two_macros::m!(use foo::panic;);
   |                          ^^^^^^^^^^
   = help: use `self::panic` to refer to this macro unambiguously
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0659`.


------------------------------------------


---- [ui] ui/imports/macro-paths.rs stdout ----
diff of stderr:

4 LL |     bar::m! {
5    |     ^^^ ambiguous name
6    |
-    = note: ambiguous because of conflicts between name from glob import and macro-expanded name in the same module
+    = note: ambiguous because of a conflict between a name from glob import and macro-expanded name in the same module
8 note: `bar` could refer to the module defined here
10    |


23 LL |     baz::m! {
24    |     ^^^ ambiguous name
25    |
-    = note: ambiguous because of macro-expanded name conflicts with less macro-expanded name from outer scope during import/macro resolution
+    = note: ambiguous because of a conflict between a macro-expanded name and less macro-expanded name from outer scope during import/macro resolution
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
   = note: ambiguous because of a conflict between a macro-expanded name and less macro-expanded name from outer scope during import/macro resolution
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

4 LL |         m!()
5    |         ^ ambiguous name
6    |
-    = note: ambiguous because of `macro_rules` name conflicts with non-`macro_rules` name from another module
+    = note: ambiguous because of a name conflict between a `macro_rules` and a non-`macro_rules` name from another module
8 note: `m` could refer to the macro defined here
9   --> $DIR/ambiguity-legacy-vs-modern.rs:26:5

22 LL |     m!()
23    |     ^ ambiguous name
24    |
24    |
-    = note: ambiguous because of `macro_rules` name conflicts with non-`macro_rules` name from another module
+    = note: ambiguous because of a name conflict between a `macro_rules` and a non-`macro_rules` name from another module
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
   = note: ambiguous because of a name conflict between a `macro_rules` and a non-`macro_rules` name from another module
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
   = note: ambiguous because of a name conflict between a `macro_rules` and a non-`macro_rules` name from another module
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

4 LL |         std::panic!();
5    |         ^^^ ambiguous name
6    |
-    = note: ambiguous because of name conflicts with outer scope during import/macro resolution due to glob import
+    = note: ambiguous because of a name conflict with an outer scope during import/macro resolution due to glob import
8    = note: `std` could refer to a built-in crate
9 note: `std` could also refer to the module imported here


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-path-prelude-shadowing/macro-path-prelude-shadowing.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-path-prelude-shadowing/macro-path-prelude-shadowing.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args macros/macro-path-prelude-shadowing.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/macro-path-prelude-shadowing.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-path-prelude-shadowing" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-path-prelude-shadowing/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0659]: `std` is ambiguous
  --> /checkout/src/test/ui/macros/macro-path-prelude-shadowing.rs:29:9
   |
LL |         std::panic!(); //~ ERROR `std` is ambiguous
   |         ^^^ ambiguous name
   |
   = note: ambiguous because of a name conflict with an outer scope during import/macro resolution due to glob import
   = note: `std` could refer to a built-in crate
note: `std` could also refer to the module imported here
   |
   |
LL |     use m2::*; // glob-import user-defined `std`
   = help: consider adding an explicit import of `std` to disambiguate
   = help: consider adding an explicit import of `std` to disambiguate
   = help: or use `self::std` to refer to this module unambiguously
error: aborting due to previous error

For more information about this error, try `rustc --explain E0659`.


------------------------------------------


---- [ui] ui/macros/macro-shadowing.rs stdout ----
diff of stderr:

16 LL | foo!();
17    | ^^^ ambiguous name
18    |
-    = note: ambiguous because of macro-expanded name conflicts with less macro-expanded name from outer scope during import/macro resolution
+    = note: ambiguous because of a conflict between a macro-expanded name and less macro-expanded name from outer scope during import/macro resolution
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
   = note: ambiguous because of a conflict between a macro-expanded name and less macro-expanded name from outer scope during import/macro resolution
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


---- [ui] ui/macros/restricted-shadowing-legacy.rs stdout ----
diff of stderr:

7 LL | include!();
9    |
9    |
-    = note: ambiguous because of macro-expanded name conflicts with less macro-expanded name from outer scope during import/macro resolution
+    = note: ambiguous because of a conflict between a macro-expanded name and less macro-expanded name from outer scope during import/macro resolution
11 note: `m` could refer to the macro defined here
13    |


35 LL | include!();
37    |
37    |
-    = note: ambiguous because of macro-expanded name conflicts with less macro-expanded name from outer scope during import/macro resolution
+    = note: ambiguous because of a conflict between a macro-expanded name and less macro-expanded name from outer scope during import/macro resolution
39 note: `m` could refer to the macro defined here
41    |


63 LL | include!();
65    |
65    |
-    = note: ambiguous because of macro-expanded name conflicts with less macro-expanded name from outer scope during import/macro resolution
+    = note: ambiguous because of a conflict between a macro-expanded name and less macro-expanded name from outer scope during import/macro resolution
67 note: `m` could refer to the macro defined here
69    |


91 LL | include!();
93    |
93    |
-    = note: ambiguous because of macro-expanded name conflicts with less macro-expanded name from outer scope during import/macro resolution
+    = note: ambiguous because of a conflict between a macro-expanded name and less macro-expanded name from outer scope during import/macro resolution
95 note: `m` could refer to the macro defined here
97    |


119 LL | include!();
121    |
121    |
-    = note: ambiguous because of macro-expanded name conflicts with less macro-expanded name from outer scope during import/macro resolution
+    = note: ambiguous because of a conflict between a macro-expanded name and less macro-expanded name from outer scope during import/macro resolution
123 note: `m` could refer to the macro defined here
125    |


147 LL | include!();
149    |
149    |
-    = note: ambiguous because of macro-expanded name conflicts with less macro-expanded name from outer scope during import/macro resolution
+    = note: ambiguous because of a conflict between a macro-expanded name and less macro-expanded name from outer scope during import/macro resolution
151 note: `m` could refer to the macro defined here
153    |


175 LL | include!();
177    |
177    |
-    = note: ambiguous because of macro-expanded name conflicts with less macro-expanded name from outer scope during import/macro resolution
+    = note: ambiguous because of a conflict between a macro-expanded name and less macro-expanded name from outer scope during import/macro resolution
179 note: `m` could refer to the macro defined here
181    |


203 LL | include!();
205    |
205    |
-    = note: ambiguous because of macro-expanded name conflicts with less macro-expanded name from outer scope during import/macro resolution
+    = note: ambiguous because of a conflict between a macro-expanded name and less macro-expanded name from outer scope during import/macro resolution
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
   = note: ambiguous because of a conflict between a macro-expanded name and less macro-expanded name from outer scope during import/macro resolution
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
   = note: ambiguous because of a conflict between a macro-expanded name and less macro-expanded name from outer scope during import/macro resolution
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
   = note: ambiguous because of a conflict between a macro-expanded name and less macro-expanded name from outer scope during import/macro resolution
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
   = note: ambiguous because of a conflict between a macro-expanded name and less macro-expanded name from outer scope during import/macro resolution
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
   = note: ambiguous because of a conflict between a macro-expanded name and less macro-expanded name from outer scope during import/macro resolution
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
   = note: ambiguous because of a conflict between a macro-expanded name and less macro-expanded name from outer scope during import/macro resolution
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
   = note: ambiguous because of a conflict between a macro-expanded name and less macro-expanded name from outer scope during import/macro resolution
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
   = note: ambiguous because of a conflict between a macro-expanded name and less macro-expanded name from outer scope during import/macro resolution
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
-    = note: ambiguous because of macro-expanded name conflicts with less macro-expanded name from outer scope during import/macro resolution
+    = note: ambiguous because of a conflict between a macro-expanded name and less macro-expanded name from outer scope during import/macro resolution
11 note: `m` could refer to the macro defined here
13    |


35 LL | include!();
37    |
37    |
-    = note: ambiguous because of macro-expanded name conflicts with less macro-expanded name from outer scope during import/macro resolution
+    = note: ambiguous because of a conflict between a macro-expanded name and less macro-expanded name from outer scope during import/macro resolution
39 note: `m` could refer to the macro defined here
41    |


63 LL | include!();
65    |
65    |
-    = note: ambiguous because of macro-expanded name conflicts with less macro-expanded name from outer scope during import/macro resolution
+    = note: ambiguous because of a conflict between a macro-expanded name and less macro-expanded name from outer scope during import/macro resolution
67 note: `m` could refer to the macro defined here
69    |


91 LL | include!();
93    |
93    |
-    = note: ambiguous because of macro-expanded name conflicts with less macro-expanded name from outer scope during import/macro resolution
+    = note: ambiguous because of a conflict between a macro-expanded name and less macro-expanded name from outer scope during import/macro resolution
95 note: `m` could refer to the macro defined here
97    |


119 LL | include!();
121    |
121    |
-    = note: ambiguous because of macro-expanded name conflicts with less macro-expanded name from outer scope during import/macro resolution
+    = note: ambiguous because of a conflict between a macro-expanded name and less macro-expanded name from outer scope during import/macro resolution
123 note: `m` could refer to the macro defined here
125    |


147 LL | include!();
149    |
149    |
-    = note: ambiguous because of macro-expanded name conflicts with less macro-expanded name from outer scope during import/macro resolution
+    = note: ambiguous because of a conflict between a macro-expanded name and less macro-expanded name from outer scope during import/macro resolution
151 note: `m` could refer to the macro defined here
153    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/restricted-shadowing-modern/restricted-shadowing-modern.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args macros/restricted-shadowing-modern.rs`

---

---- [ui] ui/proc-macro/ambiguous-builtin-attrs.rs stdout ----
diff of stderr:

10 LL | #[repr(C)]
11    |   ^^^^ ambiguous name
12    |
-    = note: ambiguous because of name conflicts with builtin attribute
+    = note: ambiguous because of a name conflict with a builtin attribute
14    = note: `repr` could refer to a built-in attribute
15 note: `repr` could also refer to the attribute macro imported here
16   --> $DIR/ambiguous-builtin-attrs.rs:6:5

25 LL | #[cfg_attr(all(), repr(C))]
26    |                   ^^^^ ambiguous name
27    |
-    = note: ambiguous because of name conflicts with builtin attribute
+    = note: ambiguous because of a name conflict with a builtin attribute
29    = note: `repr` could refer to a built-in attribute
30 note: `repr` could also refer to the attribute macro imported here
31   --> $DIR/ambiguous-builtin-attrs.rs:6:5

40 LL | fn non_macro_expanded_location<#[repr(C)] T>() {
41    |                                  ^^^^ ambiguous name
42    |
-    = note: ambiguous because of name conflicts with builtin attribute
+    = note: ambiguous because of a name conflict with a builtin attribute
44    = note: `repr` could refer to a built-in attribute
45 note: `repr` could also refer to the attribute macro imported here
46   --> $DIR/ambiguous-builtin-attrs.rs:6:5

55 LL |         #[repr(C)]
56    |           ^^^^ ambiguous name
57    |
-    = note: ambiguous because of name conflicts with builtin attribute
+    = note: ambiguous because of a name conflict with a builtin attribute
59    = note: `repr` could refer to a built-in attribute
60 note: `repr` could also refer to the attribute macro imported here
61   --> $DIR/ambiguous-builtin-attrs.rs:6:5

70 LL | #[allow(unused)]
71    |   ^^^^^ ambiguous name
72    |
-    = note: ambiguous because of name conflicts with builtin attribute
+    = note: ambiguous because of a name conflict with a builtin attribute
74    = note: `allow` could refer to a built-in attribute
75 note: `allow` could also refer to the built-in attribute imported here
76   --> $DIR/ambiguous-builtin-attrs.rs:37:5

85 LL | #![feature(decl_macro)]
86    |    ^^^^^^^ ambiguous name
87    |
-    = note: ambiguous because of name conflicts with builtin attribute
+    = note: ambiguous because of a name conflict with a builtin attribute
89    = note: `feature` could refer to a built-in attribute
90 note: `feature` could also refer to the attribute macro imported here
91   --> $DIR/ambiguous-builtin-attrs.rs:6:5

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/ambiguous-builtin-attrs/ambiguous-builtin-attrs.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args proc-macro/ambiguous-builtin-attrs.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/ambiguous-builtin-attrs.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/ambiguous-builtin-attrs" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/ambiguous-builtin-attrs/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0425]: cannot find value `NonExistent` in this scope
  --> /checkout/src/test/ui/proc-macro/ambiguous-builtin-attrs.rs:34:5
   |
LL |     NonExistent; //~ ERROR cannot find value `NonExistent` in this scope


error[E0659]: `repr` is ambiguous
   |
   |
LL | #[repr(C)] //~ ERROR `repr` is ambiguous
   |   ^^^^ ambiguous name
   |
   = note: ambiguous because of a name conflict with a builtin attribute
   = note: `repr` could refer to a built-in attribute
note: `repr` could also refer to the attribute macro imported here
   |
LL | use builtin_attrs::*;
   |     ^^^^^^^^^^^^^^^^
   |     ^^^^^^^^^^^^^^^^
   = help: use `crate::repr` to refer to this attribute macro unambiguously

error[E0659]: `repr` is ambiguous
   |
   |
LL | #[cfg_attr(all(), repr(C))] //~ ERROR `repr` is ambiguous
   |                   ^^^^ ambiguous name
   |
   = note: ambiguous because of a name conflict with a builtin attribute
   = note: `repr` could refer to a built-in attribute
note: `repr` could also refer to the attribute macro imported here
   |
LL | use builtin_attrs::*;
   |     ^^^^^^^^^^^^^^^^
   |     ^^^^^^^^^^^^^^^^
   = help: use `crate::repr` to refer to this attribute macro unambiguously

error[E0659]: `repr` is ambiguous
   |
   |
LL | fn non_macro_expanded_location<#[repr(C)] T>() {
   |                                  ^^^^ ambiguous name
   |
   = note: ambiguous because of a name conflict with a builtin attribute
   = note: `repr` could refer to a built-in attribute
note: `repr` could also refer to the attribute macro imported here
   |
LL | use builtin_attrs::*;
   |     ^^^^^^^^^^^^^^^^
   |     ^^^^^^^^^^^^^^^^
   = help: use `crate::repr` to refer to this attribute macro unambiguously

error[E0659]: `repr` is ambiguous
   |
   |
LL |         #[repr(C)]
   |           ^^^^ ambiguous name
   |
   = note: ambiguous because of a name conflict with a builtin attribute
   = note: `repr` could refer to a built-in attribute
note: `repr` could also refer to the attribute macro imported here
   |
LL | use builtin_attrs::*;
   |     ^^^^^^^^^^^^^^^^
   |     ^^^^^^^^^^^^^^^^
   = help: use `crate::repr` to refer to this attribute macro unambiguously

error[E0659]: `allow` is ambiguous
   |
   |
LL | #[allow(unused)] //~ ERROR `allow` is ambiguous
   |   ^^^^^ ambiguous name
   |
   = note: ambiguous because of a name conflict with a builtin attribute
   = note: `allow` could refer to a built-in attribute
note: `allow` could also refer to the built-in attribute imported here
   |
LL | use deny as allow;
   |     ^^^^^^^^^^^^^
   |     ^^^^^^^^^^^^^
   = help: use `crate::allow` to refer to this built-in attribute unambiguously

error[E0659]: `feature` is ambiguous
   |
   |
LL | #![feature(decl_macro)] //~ ERROR `feature` is ambiguous
   |    ^^^^^^^ ambiguous name
   |
   = note: ambiguous because of a name conflict with a builtin attribute
   = note: `feature` could refer to a built-in attribute
note: `feature` could also refer to the attribute macro imported here
   |
LL | use builtin_attrs::*;
   |     ^^^^^^^^^^^^^^^^
   |     ^^^^^^^^^^^^^^^^
   = help: use `crate::feature` to refer to this attribute macro unambiguously
error[E0517]: attribute should be applied to a struct, enum, or union
  --> /checkout/src/test/ui/proc-macro/ambiguous-builtin-attrs.rs:20:39
   |
   |
LL | fn non_macro_expanded_location<#[repr(C)] T>() {
   |                                       ^   - not a struct, enum, or union
error[E0517]: attribute should be applied to a struct, enum, or union
  --> /checkout/src/test/ui/proc-macro/ambiguous-builtin-attrs.rs:24:16
   |
   |
LL |         #[repr(C)]
...
LL |         _ => {}
   |         ------- not a struct, enum, or union

---

---- [ui] ui/proc-macro/derive-helper-shadowing.rs stdout ----
diff of stderr:

58 LL | #[empty_helper]
59    |   ^^^^^^^^^^^^ ambiguous name
60    |
-    = note: ambiguous because of name conflicts with derive helper attribute
+    = note: ambiguous because of a name conflict with a derive helper attribute
62 note: `empty_helper` could refer to the derive helper attribute defined here
64    |


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
   = note: ambiguous because of a name conflict with a derive helper attribute
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

---- [ui] ui/proc-macro/helper-attr-blocked-by-import-ambig.rs stdout ----
diff of stderr:

4 LL | #[empty_helper]
5    |   ^^^^^^^^^^^^ ambiguous name
6    |
-    = note: ambiguous because of name conflicts with derive helper attribute
+    = note: ambiguous because of a name conflict with a derive helper attribute
8 note: `empty_helper` could refer to the derive helper attribute defined here
10    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/helper-attr-blocked-by-import-ambig/helper-attr-blocked-by-import-ambig.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args proc-macro/helper-attr-blocked-by-import-ambig.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/helper-attr-blocked-by-import-ambig.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/helper-attr-blocked-by-import-ambig" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/helper-attr-blocked-by-import-ambig/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0659]: `empty_helper` is ambiguous
   |
   |
LL | #[empty_helper] //~ ERROR `empty_helper` is ambiguous
   |   ^^^^^^^^^^^^ ambiguous name
   |
   = note: ambiguous because of a name conflict with a derive helper attribute
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
  --> /checkout/src/test/ui/proc-macro/helper-attr-blocked-by-import-ambig.rs:7:3
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

---- [ui] ui/proc-macro/issue-41211.rs stdout ----
diff of stderr:

4 LL | #![identity_attr]
5    |    ^^^^^^^^^^^^^ ambiguous name
6    |
-    = note: ambiguous because of macro-expanded name conflicts with less macro-expanded name from outer scope during import/macro resolution
+    = note: ambiguous because of a conflict between a macro-expanded name and less macro-expanded name from outer scope during import/macro resolution
8 note: `identity_attr` could refer to the attribute macro imported here
10    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/issue-41211/issue-41211.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args proc-macro/issue-41211.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/issue-41211.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/issue-41211" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/issue-41211/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0659]: `identity_attr` is ambiguous
   |
   |
LL | #![identity_attr]
   |    ^^^^^^^^^^^^^ ambiguous name
   |
   = note: ambiguous because of a conflict between a macro-expanded name and less macro-expanded name from outer scope during import/macro resolution
note: `identity_attr` could refer to the attribute macro imported here
   |
LL | use test_macros::identity_attr;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
   = help: use `crate::identity_attr` to refer to this attribute macro unambiguously
note: `identity_attr` could also refer to the explicitly registered attribute defined here
   |
   |
LL | #![register_attr(identity_attr)]

error: aborting due to previous error

For more information about this error, try `rustc --explain E0659`.
---

10 LL | #[B]
11    |   ^ ambiguous name
12    |
-    = note: ambiguous because of name conflicts with derive helper attribute
+    = note: ambiguous because of a name conflict with a derive helper attribute
14 note: `B` could refer to the derive helper attribute defined here
16    |


28 LL | #[B(D)]
29    |   ^ ambiguous name
30    |
-    = note: ambiguous because of name conflicts with derive helper attribute
+    = note: ambiguous because of a name conflict with a derive helper attribute
32 note: `B` could refer to the derive helper attribute defined here
34    |


46 LL | #[B(E = "foo")]
47    |   ^ ambiguous name
48    |
-    = note: ambiguous because of name conflicts with derive helper attribute
+    = note: ambiguous because of a name conflict with a derive helper attribute
50 note: `B` could refer to the derive helper attribute defined here
52    |


64 LL | #[B(arbitrary tokens)]
65    |   ^ ambiguous name
66    |
-    = note: ambiguous because of name conflicts with derive helper attribute
+    = note: ambiguous because of a name conflict with a derive helper attribute
68 note: `B` could refer to the derive helper attribute defined here
70    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/proc-macro-attributes/proc-macro-attributes.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args proc-macro/proc-macro-attributes.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/proc-macro-attributes.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/proc-macro-attributes" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/proc-macro-attributes/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: cannot find attribute `C` in this scope
  --> /checkout/src/test/ui/proc-macro/proc-macro-attributes.rs:9:3
   |
LL | #[C] //~ ERROR cannot find attribute `C` in this scope
   |   ^ help: a derive helper attribute with a similar name exists: `B`

error[E0659]: `B` is ambiguous
   |
   |
LL | #[B] //~ ERROR `B` is ambiguous
   |   ^ ambiguous name
   |
   = note: ambiguous because of a name conflict with a derive helper attribute
note: `B` could refer to the derive helper attribute defined here
   |
LL | #[derive(B)]
   |          ^
   |          ^
note: `B` could also refer to the derive macro imported here
   |
LL | #[macro_use]
   | ^^^^^^^^^^^^


error[E0659]: `B` is ambiguous
   |
   |
LL | #[B(D)] //~ ERROR `B` is ambiguous
   |   ^ ambiguous name
   |
   = note: ambiguous because of a name conflict with a derive helper attribute
note: `B` could refer to the derive helper attribute defined here
   |
LL | #[derive(B)]
   |          ^
   |          ^
note: `B` could also refer to the derive macro imported here
   |
LL | #[macro_use]
   | ^^^^^^^^^^^^


error[E0659]: `B` is ambiguous
   |
   |
LL | #[B(E = "foo")] //~ ERROR `B` is ambiguous
   |   ^ ambiguous name
   |
   = note: ambiguous because of a name conflict with a derive helper attribute
note: `B` could refer to the derive helper attribute defined here
   |
LL | #[derive(B)]
   |          ^
   |          ^
note: `B` could also refer to the derive macro imported here
   |
LL | #[macro_use]
   | ^^^^^^^^^^^^


error[E0659]: `B` is ambiguous
   |
   |
LL | #[B(arbitrary tokens)] //~ ERROR `B` is ambiguous
   |   ^ ambiguous name
   |
   = note: ambiguous because of a name conflict with a derive helper attribute
note: `B` could refer to the derive helper attribute defined here
   |
LL | #[derive(B)]
   |          ^
   |          ^
note: `B` could also refer to the derive macro imported here
   |
LL | #[macro_use]
   | ^^^^^^^^^^^^


warning: derive helper attribute is used before it is introduced
  --> /checkout/src/test/ui/proc-macro/proc-macro-attributes.rs:6:3
   |
LL | #[B] //~ ERROR `B` is ambiguous
...
LL | #[derive(B)]
   |          - the attribute is introduced here
   |
   |
   = note: `#[warn(legacy_derive_helpers)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #79202 <https://github.com/rust-lang/rust/issues/79202>

warning: derive helper attribute is used before it is introduced
  --> /checkout/src/test/ui/proc-macro/proc-macro-attributes.rs:10:3
   |
LL | #[B(D)] //~ ERROR `B` is ambiguous
...
LL | #[derive(B)]
   |          - the attribute is introduced here
   |
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #79202 <https://github.com/rust-lang/rust/issues/79202>

warning: derive helper attribute is used before it is introduced
  --> /checkout/src/test/ui/proc-macro/proc-macro-attributes.rs:13:3
   |
LL | #[B(E = "foo")] //~ ERROR `B` is ambiguous
...
LL | #[derive(B)]
   |          - the attribute is introduced here
   |
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #79202 <https://github.com/rust-lang/rust/issues/79202>

warning: derive helper attribute is used before it is introduced
  --> /checkout/src/test/ui/proc-macro/proc-macro-attributes.rs:16:3
   |
LL | #[B(arbitrary tokens)] //~ ERROR `B` is ambiguous
...
LL | #[derive(B)]
   |          - the attribute is introduced here
   |
---
test result: FAILED. 12207 passed; 24 failed; 110 ignored; 0 measured; 0 filtered out; finished in 137.65s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:16
