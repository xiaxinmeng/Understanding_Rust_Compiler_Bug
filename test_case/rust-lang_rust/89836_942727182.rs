plain
.................................................................................................... 10600/12284
.................................................................................................... 10700/12284
.................................................................................................... 10800/12284
.................................................................................................... 10900/12284
...................F.F.....FF....F...F.F..FiiF.F............................i......................F 11000/12284
.................................................................................................... 11200/12284
.................................................................................................... 11300/12284
.................................................................................................... 11400/12284
.................................................................................................... 11500/12284
---
diff of stderr:

2   --> $DIR/tls.rs:12:25
3    |
4 LL |     unsafe { let _val = A; }
-    |                         ^ cannot access thread local static (DefId(0:6 ~ tls[7457]::A))
+    |                         ^ cannot access thread local static (DefId(0:6 ~ tls[aa18]::A))
7 error[E0080]: could not evaluate static initializer
8   --> $DIR/tls.rs:19:26

9    |
9    |
10 LL |     unsafe { let _val = &A; }
-    |                          ^ cannot access thread local static (DefId(0:6 ~ tls[7457]::A))
+    |                          ^ cannot access thread local static (DefId(0:6 ~ tls[aa18]::A))
13 warning: skipping const checks
14    |


---
To only update this specific test, also pass `--test-args consts/miri_unleashed/tls.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/miri_unleashed/tls.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/tls" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zunleash-the-miri-inside-of-you" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/tls/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0080]: could not evaluate static initializer
  --> /checkout/src/test/ui/consts/miri_unleashed/tls.rs:12:25
   |
LL |     unsafe { let _val = A; }
   |                         ^ cannot access thread local static (DefId(0:6 ~ tls[aa18]::A))
error[E0080]: could not evaluate static initializer
  --> /checkout/src/test/ui/consts/miri_unleashed/tls.rs:19:26
   |
   |
LL |     unsafe { let _val = &A; }
   |                          ^ cannot access thread local static (DefId(0:6 ~ tls[aa18]::A))
warning: skipping const checks
   |
help: skipping check that does not even have a feature gate
  --> /checkout/src/test/ui/consts/miri_unleashed/tls.rs:12:25
  --> /checkout/src/test/ui/consts/miri_unleashed/tls.rs:12:25
   |
LL |     unsafe { let _val = A; }
help: skipping check that does not even have a feature gate
  --> /checkout/src/test/ui/consts/miri_unleashed/tls.rs:19:26
   |
   |
LL |     unsafe { let _val = &A; }

error: aborting due to 2 previous errors; 1 warning emitted

For more information about this error, try `rustc --explain E0080`.
---
diff of stderr:

9   --> $DIR/generator-print-verbose-1.rs:35:9
10    |
11 LL |         let _non_send_gen = make_non_send_generator();
-    |             ------------- has type `Opaque(DefId(0:34 ~ generator_print_verbose_1[2906]::make_non_send_generator::{opaque#0}), [])` which is not `Send`
+    |             ------------- has type `Opaque(DefId(0:34 ~ generator_print_verbose_1[762a]::make_non_send_generator::{opaque#0}), [])` which is not `Send`
13 LL |         yield;
14    |         ^^^^^ yield occurs here, with `_non_send_gen` maybe used later


29    = help: the trait `Sync` is not implemented for `RefCell<i32>`
30    = note: required because of the requirements on the impl of `Send` for `Arc<RefCell<i32>>`
31    = note: required because it appears within the type `[make_gen2<Arc<RefCell<i32>>>::{closure#0} upvar_tys=(Arc<RefCell<i32>>) {()}]`
-    = note: required because it appears within the type `Opaque(DefId(0:39 ~ generator_print_verbose_1[2906]::make_gen2::{opaque#0}), [std::sync::Arc<std::cell::RefCell<i32>>])`
-    = note: required because it appears within the type `Opaque(DefId(0:42 ~ generator_print_verbose_1[2906]::make_non_send_generator2::{opaque#0}), [])`
-    = note: required because it appears within the type `{Opaque(DefId(0:42 ~ generator_print_verbose_1[2906]::make_non_send_generator2::{opaque#0}), []), ()}`
-    = note: required because it appears within the type `[test2::{closure#0} upvar_tys=() {Opaque(DefId(0:42 ~ generator_print_verbose_1[2906]::make_non_send_generator2::{opaque#0}), []), ()}]`
+    = note: required because it appears within the type `Opaque(DefId(0:39 ~ generator_print_verbose_1[762a]::make_gen2::{opaque#0}), [std::sync::Arc<std::cell::RefCell<i32>>])`
+    = note: required because it appears within the type `Opaque(DefId(0:42 ~ generator_print_verbose_1[762a]::make_non_send_generator2::{opaque#0}), [])`
+    = note: required because it appears within the type `{Opaque(DefId(0:42 ~ generator_print_verbose_1[762a]::make_non_send_generator2::{opaque#0}), []), ()}`
+    = note: required because it appears within the type `[test2::{closure#0} upvar_tys=() {Opaque(DefId(0:42 ~ generator_print_verbose_1[762a]::make_non_send_generator2::{opaque#0}), []), ()}]`
36 note: required by a bound in `require_send`
38    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/print/generator-print-verbose-1/generator-print-verbose-1.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args generator/print/generator-print-verbose-1.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generator/print/generator-print-verbose-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/print/generator-print-verbose-1" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/print/generator-print-verbose-1/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: generator cannot be sent between threads safely
   |
LL |     require_send(send_gen);
LL |     require_send(send_gen);
   |     ^^^^^^^^^^^^ generator is not `Send`
   |
   = help: the trait `Sync` is not implemented for `RefCell<i32>`
note: generator is not `Send` as this value is used across a yield
   |
   |
LL |         let _non_send_gen = make_non_send_generator();
   |             ------------- has type `Opaque(DefId(0:34 ~ generator_print_verbose_1[762a]::make_non_send_generator::{opaque#0}), [])` which is not `Send`
LL |         yield;
   |         ^^^^^ yield occurs here, with `_non_send_gen` maybe used later
LL |     };
   |     - `_non_send_gen` is later dropped here
note: required by a bound in `require_send`
   |
   |
LL | fn require_send(_: impl Send) {}
   |                         ^^^^ required by this bound in `require_send`

error[E0277]: `RefCell<i32>` cannot be shared between threads safely
   |
LL |     require_send(send_gen);
LL |     require_send(send_gen);
   |     ^^^^^^^^^^^^ `RefCell<i32>` cannot be shared between threads safely
   |
   = help: the trait `Sync` is not implemented for `RefCell<i32>`
   = note: required because of the requirements on the impl of `Send` for `Arc<RefCell<i32>>`
   = note: required because it appears within the type `[make_gen2<Arc<RefCell<i32>>>::{closure#0} upvar_tys=(Arc<RefCell<i32>>) {()}]`
   = note: required because it appears within the type `Opaque(DefId(0:39 ~ generator_print_verbose_1[762a]::make_gen2::{opaque#0}), [std::sync::Arc<std::cell::RefCell<i32>>])`
   = note: required because it appears within the type `Opaque(DefId(0:42 ~ generator_print_verbose_1[762a]::make_non_send_generator2::{opaque#0}), [])`
   = note: required because it appears within the type `{Opaque(DefId(0:42 ~ generator_print_verbose_1[762a]::make_non_send_generator2::{opaque#0}), []), ()}`
   = note: required because it appears within the type `[test2::{closure#0} upvar_tys=() {Opaque(DefId(0:42 ~ generator_print_verbose_1[762a]::make_non_send_generator2::{opaque#0}), []), ()}]`
note: required by a bound in `require_send`
   |
   |
LL | fn require_send(_: impl Send) {}
   |                         ^^^^ required by this bound in `require_send`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0277`.


------------------------------------------


---- [ui] ui/lto-duplicate-symbols.rs stdout ----
diff of stderr:

1 warning: Linking globals named 'foo': symbol multiply defined!
2 
- error: failed to load bc of "lto-duplicate-symbols2.lto_duplicate_symbols2.612ab3b4-cgu.0.rcgu.o": 
+ error: failed to load bc of "lto-duplicate-symbols2.lto_duplicate_symbols2.a575b3ae-cgu.0.rcgu.o": 
5 error: aborting due to previous error; 1 warning emitted
6 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lto-duplicate-symbols/lto-duplicate-symbols.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lto-duplicate-symbols.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lto-duplicate-symbols.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lto-duplicate-symbols" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "lto" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lto-duplicate-symbols/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: Linking globals named 'foo': symbol multiply defined!

error: failed to load bc of "lto-duplicate-symbols2.lto_duplicate_symbols2.a575b3ae-cgu.0.rcgu.o": 
error: aborting due to previous error; 1 warning emitted


------------------------------------------
------------------------------------------


---- [ui] ui/specialization/min_specialization/repeated_projection_type.rs stdout ----
diff of stderr:

- error: cannot specialize on `Binder(ProjectionPredicate(ProjectionTy { substs: [V], item_def_id: DefId(0:6 ~ repeated_projection_type[5448]::Id::This) }, (I,)), [])`
+ error: cannot specialize on `Binder(ProjectionPredicate(ProjectionTy { substs: [V], item_def_id: DefId(0:6 ~ repeated_projection_type[d580]::Id::This) }, (I,)), [])`
3    |
3    |
4 LL | / impl<I, V: Id<This = (I,)>> X for V {

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/min_specialization/repeated_projection_type/repeated_projection_type.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args specialization/min_specialization/repeated_projection_type.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/specialization/min_specialization/repeated_projection_type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/min_specialization/repeated_projection_type" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/min_specialization/repeated_projection_type/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: cannot specialize on `Binder(ProjectionPredicate(ProjectionTy { substs: [V], item_def_id: DefId(0:6 ~ repeated_projection_type[d580]::Id::This) }, (I,)), [])`
   |
   |
LL | / impl<I, V: Id<This = (I,)>> X for V {
LL | |     //~^ ERROR cannot specialize on
LL | |     fn f() {}
LL | | }

error: aborting due to previous error



------------------------------------------


---- [ui] ui/symbol-names/basic.rs#legacy stdout ----
diff of stderr:

- error: symbol-name(_ZN5basic4main17h2f2561ceecf7b669E)
+ error: symbol-name(_ZN5basic4main17hbca64185e7242ca8E)
3    |
4 LL | #[rustc_symbol_name]

5    | ^^^^^^^^^^^^^^^^^^^^
5    | ^^^^^^^^^^^^^^^^^^^^
6 
- error: demangling(basic::main::h2f2561ceecf7b669)
+ error: demangling(basic::main::hbca64185e7242ca8)
9    |
10 LL | #[rustc_symbol_name]



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/basic.legacy/basic.legacy.stderr
To only update this specific test, also pass `--test-args symbol-names/basic.rs`


error in revision `legacy`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/symbol-names/basic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "legacy" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/basic.legacy" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "symbol-mangling-version=legacy" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/basic.legacy/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: symbol-name(_ZN5basic4main17hbca64185e7242ca8E)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling(basic::main::hbca64185e7242ca8)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(basic::main)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: def-path(main)
  --> /checkout/src/test/ui/symbol-names/basic.rs:17:1
   |
LL | #[rustc_def_path]

error: aborting due to 4 previous errors



------------------------------------------


---- [ui] ui/symbol-names/basic.rs#v0 stdout ----

4 LL | #[rustc_symbol_name]
5    | ^^^^^^^^^^^^^^^^^^^^
6 
6 
- error: demangling(basic[cf58ff788efc81d1]::main)
+ error: demangling(basic[befd4d962b760c93]::main)
9    |
10 LL | #[rustc_symbol_name]



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/basic.v0/basic.v0.stderr
To only update this specific test, also pass `--test-args symbol-names/basic.rs`


error in revision `v0`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/symbol-names/basic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "v0" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/basic.v0" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "symbol-mangling-version=v0" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/basic.v0/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: symbol-name(_RNvCsgoDd6tdJD8Z_5basic4main)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling(basic[befd4d962b760c93]::main)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(basic::main)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: def-path(main)
  --> /checkout/src/test/ui/symbol-names/basic.rs:17:1
   |
LL | #[rustc_def_path]

error: aborting due to 4 previous errors


---

4 LL | #[rustc_symbol_name]
5    | ^^^^^^^^^^^^^^^^^^^^
6 
- error: demangling(<c[7b9588b5fcee79b5]::Unsigned<11u8>>)
+ error: demangling(<c[32ebff575b5aeb4]::Unsigned<11u8>>)
9    |
10 LL | #[rustc_symbol_name]

22 LL | #[rustc_symbol_name]
22 LL | #[rustc_symbol_name]
23    | ^^^^^^^^^^^^^^^^^^^^
24 
- error: demangling(<c[7b9588b5fcee79b5]::Signed<-152i16>>)
+ error: demangling(<c[32ebff575b5aeb4]::Signed<-152i16>>)
27    |
28 LL | #[rustc_symbol_name]

40 LL | #[rustc_symbol_name]
40 LL | #[rustc_symbol_name]
41    | ^^^^^^^^^^^^^^^^^^^^
42 
- error: demangling(<c[7b9588b5fcee79b5]::Bool<true>>)
+ error: demangling(<c[32ebff575b5aeb4]::Bool<true>>)
45    |
46 LL | #[rustc_symbol_name]

58 LL | #[rustc_symbol_name]
58 LL | #[rustc_symbol_name]
59    | ^^^^^^^^^^^^^^^^^^^^
60 
- error: demangling(<c[7b9588b5fcee79b5]::Char<'∂'>>)
+ error: demangling(<c[32ebff575b5aeb4]::Char<'∂'>>)
63    |
64 LL | #[rustc_symbol_name]



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/const-generics-demangling/const-generics-demangling.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args symbol-names/const-generics-demangling.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/symbol-names/const-generics-demangling.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/const-generics-demangling" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "symbol-mangling-version=v0" "--crate-name=c" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/const-generics-demangling/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: symbol-name(_RMCsgWlb2EzlZM_1cINtB0_8UnsignedKhb_E)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling(<c[32ebff575b5aeb4]::Unsigned<11u8>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<c::Unsigned<11>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: symbol-name(_RMs_CsgWlb2EzlZM_1cINtB2_6SignedKsn98_E)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling(<c[32ebff575b5aeb4]::Signed<-152i16>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<c::Signed<-152>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: symbol-name(_RMs0_CsgWlb2EzlZM_1cINtB3_4BoolKb1_E)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling(<c[32ebff575b5aeb4]::Bool<true>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<c::Bool<true>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: symbol-name(_RMs1_CsgWlb2EzlZM_1cINtB3_4CharKc2202_E)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling(<c[32ebff575b5aeb4]::Char<'∂'>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<c::Char<'∂'>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^

---

4 LL | #[rustc_symbol_name]
5    | ^^^^^^^^^^^^^^^^^^^^
6 
- error: demangling(<c[7b9588b5fcee79b5]::Str<"abc">>)
+ error: demangling(<c[32ebff575b5aeb4]::Str<"abc">>)
9    |
10 LL | #[rustc_symbol_name]

22 LL | #[rustc_symbol_name]
22 LL | #[rustc_symbol_name]
23    | ^^^^^^^^^^^^^^^^^^^^
24 
- error: demangling(<c[7b9588b5fcee79b5]::Str<"'">>)
+ error: demangling(<c[32ebff575b5aeb4]::Str<"'">>)
27    |
28 LL | #[rustc_symbol_name]

40 LL | #[rustc_symbol_name]
40 LL | #[rustc_symbol_name]
41    | ^^^^^^^^^^^^^^^^^^^^
42 
- error: demangling(<c[7b9588b5fcee79b5]::Str<"\t\n">>)
+ error: demangling(<c[32ebff575b5aeb4]::Str<"\t\n">>)
45    |
46 LL | #[rustc_symbol_name]

58 LL | #[rustc_symbol_name]
58 LL | #[rustc_symbol_name]
59    | ^^^^^^^^^^^^^^^^^^^^
60 
- error: demangling(<c[7b9588b5fcee79b5]::Str<"∂ü">>)
+ error: demangling(<c[32ebff575b5aeb4]::Str<"∂ü">>)
63    |
64 LL | #[rustc_symbol_name]

76 LL | #[rustc_symbol_name]
76 LL | #[rustc_symbol_name]
77    | ^^^^^^^^^^^^^^^^^^^^
78 
- error: demangling(<c[7b9588b5fcee79b5]::Str<"საჭმელად_გემრიელი_სადილი">>)
+ error: demangling(<c[32ebff575b5aeb4]::Str<"საჭმელად_გემრიელი_სადილი">>)
81    |
82 LL | #[rustc_symbol_name]

94 LL | #[rustc_symbol_name]
94 LL | #[rustc_symbol_name]
95    | ^^^^^^^^^^^^^^^^^^^^
96 
- error: demangling(<c[7b9588b5fcee79b5]::Str<"🐊🦈🦆🐮 § 🐶👒☕🔥 § 🧡💛💚💙💜">>)
+ error: demangling(<c[32ebff575b5aeb4]::Str<"🐊🦈🦆🐮 § 🐶👒☕🔥 § 🧡💛💚💙💜">>)
99    |
100 LL | #[rustc_symbol_name]



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/const-generics-str-demangling/const-generics-str-demangling.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args symbol-names/const-generics-str-demangling.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/symbol-names/const-generics-str-demangling.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/const-generics-str-demangling" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "symbol-mangling-version=v0" "--crate-name=c" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/const-generics-str-demangling/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: symbol-name(_RMCsgWlb2EzlZM_1cINtB0_3StrKRe616263_E)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling(<c[32ebff575b5aeb4]::Str<"abc">>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<c::Str<"abc">>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: symbol-name(_RMs_CsgWlb2EzlZM_1cINtB2_3StrKRe27_E)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling(<c[32ebff575b5aeb4]::Str<"'">>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<c::Str<"'">>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: symbol-name(_RMs0_CsgWlb2EzlZM_1cINtB3_3StrKRe090a_E)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling(<c[32ebff575b5aeb4]::Str<"\t\n">>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<c::Str<"\t\n">>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: symbol-name(_RMs1_CsgWlb2EzlZM_1cINtB3_3StrKRee28882c3bc_E)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling(<c[32ebff575b5aeb4]::Str<"∂ü">>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<c::Str<"∂ü">>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: symbol-name(_RMs2_CsgWlb2EzlZM_1cINtB3_3StrKRee183a1e18390e183ade1839be18394e1839ae18390e183935fe18392e18394e1839be183a0e18398e18394e1839ae183985fe183a1e18390e18393e18398e1839ae18398_E)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling(<c[32ebff575b5aeb4]::Str<"საჭმელად_გემრიელი_სადილი">>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<c::Str<"საჭმელად_გემრიელი_სადილი">>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: symbol-name(_RMs3_CsgWlb2EzlZM_1cINtB3_3StrKRef09f908af09fa688f09fa686f09f90ae20c2a720f09f90b6f09f9192e29895f09f94a520c2a720f09fa7a1f09f929bf09f929af09f9299f09f929c_E)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling(<c[32ebff575b5aeb4]::Str<"🐊🦈🦆🐮 § 🐶👒☕🔥 § 🧡💛💚💙💜">>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<c::Str<"🐊🦈🦆🐮 § 🐶👒☕🔥 § 🧡💛💚💙💜">>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: aborting due to 18 previous errors


------------------------------------------


---- [ui] ui/symbol-names/impl1.rs#v0 stdout ----

4 LL |         #[rustc_symbol_name]
5    |         ^^^^^^^^^^^^^^^^^^^^
6 
6 
- error: demangling(<impl1[64a4585493741bc5]::foo::Foo>::bar)
+ error: demangling(<impl1[9e85a5baeb6e4dc2]::foo::Foo>::bar)
9    |
10 LL |         #[rustc_symbol_name]

28 LL |         #[rustc_symbol_name]
28 LL |         #[rustc_symbol_name]
29    |         ^^^^^^^^^^^^^^^^^^^^
30 
- error: demangling(<impl1[64a4585493741bc5]::foo::Foo>::baz)
+ error: demangling(<impl1[9e85a5baeb6e4dc2]::foo::Foo>::baz)
33    |
34 LL |         #[rustc_symbol_name]

52 LL |             #[rustc_symbol_name]
52 LL |             #[rustc_symbol_name]
53    |             ^^^^^^^^^^^^^^^^^^^^
54 
- error: demangling(<[&dyn impl1[64a4585493741bc5]::Foo<Assoc = for<'a> extern "C" fn(&'a u8, ...)> + impl1[64a4585493741bc5]::AutoTrait; 3usize] as impl1[64a4585493741bc5]::main::{closure#1}::Bar>::method)
+ error: demangling(<[&dyn impl1[9e85a5baeb6e4dc2]::Foo<Assoc = for<'a> extern "C" fn(&'a u8, ...)> + impl1[9e85a5baeb6e4dc2]::AutoTrait; 3usize] as impl1[9e85a5baeb6e4dc2]::main::{closure#1}::Bar>::method)
57    |
58 LL |             #[rustc_symbol_name]



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/impl1.v0/impl1.v0.stderr
To only update this specific test, also pass `--test-args symbol-names/impl1.rs`


error in revision `v0`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/symbol-names/impl1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "v0" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/impl1.v0" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "symbol-mangling-version=v0" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/impl1.v0/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: symbol-name(_RNvMNtCsdBOaAtTSw4E_5impl13fooNtB2_3Foo3bar)
   |
LL |         #[rustc_symbol_name]
   |         ^^^^^^^^^^^^^^^^^^^^


error: demangling(<impl1[9e85a5baeb6e4dc2]::foo::Foo>::bar)
   |
LL |         #[rustc_symbol_name]
   |         ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<impl1::foo::Foo>::bar)
   |
LL |         #[rustc_symbol_name]
   |         ^^^^^^^^^^^^^^^^^^^^


error: def-path(foo::Foo::bar)
   |
LL |         #[rustc_def_path]
   |         ^^^^^^^^^^^^^^^^^


error: symbol-name(_RNvMNtCsdBOaAtTSw4E_5impl13barNtNtB4_3foo3Foo3baz)
   |
LL |         #[rustc_symbol_name]
   |         ^^^^^^^^^^^^^^^^^^^^


error: demangling(<impl1[9e85a5baeb6e4dc2]::foo::Foo>::baz)
   |
LL |         #[rustc_symbol_name]
   |         ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<impl1::foo::Foo>::baz)
   |
LL |         #[rustc_symbol_name]
   |         ^^^^^^^^^^^^^^^^^^^^


error: def-path(bar::<impl foo::Foo>::baz)
   |
LL |         #[rustc_def_path]
   |         ^^^^^^^^^^^^^^^^^


error: symbol-name(_RNvXNCNvCsdBOaAtTSw4E_5impl14mains_0ARDNtB6_3Foop5AssocFG_KCRL0_hvEuNtB6_9AutoTraitEL_j3_NtB2_3Bar6method)
   |
LL |             #[rustc_symbol_name]
   |             ^^^^^^^^^^^^^^^^^^^^


error: demangling(<[&dyn impl1[9e85a5baeb6e4dc2]::Foo<Assoc = for<'a> extern "C" fn(&'a u8, ...)> + impl1[9e85a5baeb6e4dc2]::AutoTrait; 3usize] as impl1[9e85a5baeb6e4dc2]::main::{closure#1}::Bar>::method)
   |
LL |             #[rustc_symbol_name]
   |             ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<[&dyn impl1::Foo<Assoc = for<'a> extern "C" fn(&'a u8, ...)> + impl1::AutoTrait; 3] as impl1::main::{closure#1}::Bar>::method)
   |
LL |             #[rustc_symbol_name]
   |             ^^^^^^^^^^^^^^^^^^^^


error: def-path(<[&dyn Foo<Assoc = for<'r> extern "C" fn(&'r u8, ...)> + AutoTrait; 3] as main::{closure#1}::Bar>::method)
   |
LL |             #[rustc_def_path]
   |             ^^^^^^^^^^^^^^^^^

---

4 LL | #[rustc_symbol_name]
5    | ^^^^^^^^^^^^^^^^^^^^
6 
- error: demangling(<c[7b9588b5fcee79b5]::RefByte<{&123u8}>>)
+ error: demangling(<c[32ebff575b5aeb4]::RefByte<{&123u8}>>)
9    |
10 LL | #[rustc_symbol_name]

22 LL | #[rustc_symbol_name]
22 LL | #[rustc_symbol_name]
23    | ^^^^^^^^^^^^^^^^^^^^
24 
- error: demangling(<c[7b9588b5fcee79b5]::RefZst<{&[]}>>)
+ error: demangling(<c[32ebff575b5aeb4]::RefZst<{&[]}>>)
27    |
28 LL | #[rustc_symbol_name]

40 LL | #[rustc_symbol_name]
40 LL | #[rustc_symbol_name]
41    | ^^^^^^^^^^^^^^^^^^^^
42 
- error: demangling(<c[7b9588b5fcee79b5]::Array3Bytes<{[1u8, 2u8, 3u8]}>>)
+ error: demangling(<c[32ebff575b5aeb4]::Array3Bytes<{[1u8, 2u8, 3u8]}>>)
45    |
46 LL | #[rustc_symbol_name]

58 LL | #[rustc_symbol_name]
58 LL | #[rustc_symbol_name]
59    | ^^^^^^^^^^^^^^^^^^^^
60 
- error: demangling(<c[7b9588b5fcee79b5]::TupleByteBool<{(1u8, false)}>>)
+ error: demangling(<c[32ebff575b5aeb4]::TupleByteBool<{(1u8, false)}>>)
63    |
64 LL | #[rustc_symbol_name]

76 LL | #[rustc_symbol_name]
76 LL | #[rustc_symbol_name]
77    | ^^^^^^^^^^^^^^^^^^^^
78 
- error: demangling(<c[7b9588b5fcee79b5]::OptionUsize<{core[$HASH_HEX]::option::Option::<usize>::None}>>)
+ error: demangling(<c[32ebff575b5aeb4]::OptionUsize<{core[$HASH_HEX]::option::Option::<usize>::None}>>)
81    |
82 LL | #[rustc_symbol_name]

94 LL | #[rustc_symbol_name]
94 LL | #[rustc_symbol_name]
95    | ^^^^^^^^^^^^^^^^^^^^
96 
- error: demangling(<c[7b9588b5fcee79b5]::OptionUsize<{core[$HASH_HEX]::option::Option::<usize>::Some(0usize)}>>)
+ error: demangling(<c[32ebff575b5aeb4]::OptionUsize<{core[$HASH_HEX]::option::Option::<usize>::Some(0usize)}>>)
99    |
100 LL | #[rustc_symbol_name]

112 LL | #[rustc_symbol_name]
112 LL | #[rustc_symbol_name]
113    | ^^^^^^^^^^^^^^^^^^^^
114 
- error: demangling(<c[7b9588b5fcee79b5]::Foo_<{c[7b9588b5fcee79b5]::Foo { s: "abc", ch: 'x', slice: &[1u8, 2u8, 3u8] }}>>)
+ error: demangling(<c[32ebff575b5aeb4]::Foo_<{c[32ebff575b5aeb4]::Foo { s: "abc", ch: 'x', slice: &[1u8, 2u8, 3u8] }}>>)
117    |
118 LL | #[rustc_symbol_name]

135    |
135    |
136    = note: this error originates in the macro `duplicate_field_name_test` (in Nightly builds, run with -Z macro-backtrace for more info)
137 
- error: demangling(<c[7b9588b5fcee79b5]::Bar_<{c[7b9588b5fcee79b5]::Bar { x: 123u8, x: 4096u16 }}>>)
+ error: demangling(<c[32ebff575b5aeb4]::Bar_<{c[32ebff575b5aeb4]::Bar { x: 123u8, x: 4096u16 }}>>)
140    |
141 LL |     #[rustc_symbol_name]



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/const-generics-structural-demangling/const-generics-structural-demangling.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args symbol-names/const-generics-structural-demangling.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/symbol-names/const-generics-structural-demangling.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/const-generics-structural-demangling" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "symbol-mangling-version=v0" "--crate-name=c" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/const-generics-structural-demangling/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: symbol-name(_RMCsgWlb2EzlZM_1cINtB0_7RefByteKRh7b_E)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling(<c[32ebff575b5aeb4]::RefByte<{&123u8}>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<c::RefByte<{&123}>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: symbol-name(_RMs_CsgWlb2EzlZM_1cINtB2_6RefZstKRAEE)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling(<c[32ebff575b5aeb4]::RefZst<{&[]}>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<c::RefZst<{&[]}>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: symbol-name(_RMs0_CsgWlb2EzlZM_1cINtB3_11Array3BytesKAh1_h2_h3_EE)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling(<c[32ebff575b5aeb4]::Array3Bytes<{[1u8, 2u8, 3u8]}>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<c::Array3Bytes<{[1, 2, 3]}>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: symbol-name(_RMs1_CsgWlb2EzlZM_1cINtB3_13TupleByteBoolKTh1_b0_EE)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling(<c[32ebff575b5aeb4]::TupleByteBool<{(1u8, false)}>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<c::TupleByteBool<{(1, false)}>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: symbol-name(_RMs2_CsgWlb2EzlZM_1cINtB3_11OptionUsizeKVNtINtNtCs5joVgQWFWp6_4core6option6OptionjE4NoneUE)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling(<c[32ebff575b5aeb4]::OptionUsize<{core[3de20f3aa4c36156]::option::Option::<usize>::None}>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<c::OptionUsize<{core::option::Option::<usize>::None}>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: symbol-name(_RMs3_CsgWlb2EzlZM_1cINtB3_11OptionUsizeKVNtINtNtCs5joVgQWFWp6_4core6option6OptionjE4SomeTj0_EE)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling(<c[32ebff575b5aeb4]::OptionUsize<{core[3de20f3aa4c36156]::option::Option::<usize>::Some(0usize)}>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<c::OptionUsize<{core::option::Option::<usize>::Some(0)}>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: symbol-name(_RMs4_CsgWlb2EzlZM_1cINtB3_4Foo_KVNtB3_3FooS1sRe616263_2chc78_5sliceRAh1_h2_h3_EEE)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling(<c[32ebff575b5aeb4]::Foo_<{c[32ebff575b5aeb4]::Foo { s: "abc", ch: 'x', slice: &[1u8, 2u8, 3u8] }}>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<c::Foo_<{c::Foo { s: "abc", ch: 'x', slice: &[1, 2, 3] }}>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: symbol-name(_RMs9_CsgWlb2EzlZM_1cINtB3_4Bar_KVNtB3_3BarS1xh7b_s_1xt1000_EE)
   |
LL |     #[rustc_symbol_name]
   |     ^^^^^^^^^^^^^^^^^^^^
...
...
LL | duplicate_field_name_test!(x);
   |
   = note: this error originates in the macro `duplicate_field_name_test` (in Nightly builds, run with -Z macro-backtrace for more info)


error: demangling(<c[32ebff575b5aeb4]::Bar_<{c[32ebff575b5aeb4]::Bar { x: 123u8, x: 4096u16 }}>>)
   |
LL |     #[rustc_symbol_name]
   |     ^^^^^^^^^^^^^^^^^^^^
...
...
LL | duplicate_field_name_test!(x);
   |
   = note: this error originates in the macro `duplicate_field_name_test` (in Nightly builds, run with -Z macro-backtrace for more info)


error: demangling-alt(<c::Bar_<{c::Bar { x: 123, x: 4096 }}>>)
   |
LL |     #[rustc_symbol_name]
   |     ^^^^^^^^^^^^^^^^^^^^
...
...
LL | duplicate_field_name_test!(x);
   |
   = note: this error originates in the macro `duplicate_field_name_test` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to 24 previous errors
error: aborting due to 24 previous errors


------------------------------------------


---- [ui] ui/symbol-names/issue-60925.rs#v0 stdout ----

4 LL |         #[rustc_symbol_name]
5    |         ^^^^^^^^^^^^^^^^^^^^
6 
6 
- error: demangling(<issue_60925[6d476aea68d4d114]::foo::Foo<issue_60925[6d476aea68d4d114]::llvm::Foo>>::foo)
+ error: demangling(<issue_60925[ec3187889549501f]::foo::Foo<issue_60925[ec3187889549501f]::llvm::Foo>>::foo)
9    |
10 LL |         #[rustc_symbol_name]



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/issue-60925.v0/issue-60925.v0.stderr
To only update this specific test, also pass `--test-args symbol-names/issue-60925.rs`


error in revision `v0`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/symbol-names/issue-60925.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "v0" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/issue-60925.v0" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "symbol-mangling-version=v0" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/issue-60925.v0/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: symbol-name(_RNvMNtCskhfDVicNTqZ_11issue_609253fooINtB2_3FooNtNtB4_4llvm3FooE3foo)
   |
LL |         #[rustc_symbol_name]
   |         ^^^^^^^^^^^^^^^^^^^^


error: demangling(<issue_60925[ec3187889549501f]::foo::Foo<issue_60925[ec3187889549501f]::llvm::Foo>>::foo)
   |
LL |         #[rustc_symbol_name]
   |         ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<issue_60925::foo::Foo<issue_60925::llvm::Foo>>::foo)
   |
LL |         #[rustc_symbol_name]
   |         ^^^^^^^^^^^^^^^^^^^^


error: aborting due to 3 previous errors


------------------------------------------


---- [ui] ui/symbol-names/issue-75326.rs#v0 stdout ----

4 LL |     #[rustc_symbol_name]
5    |     ^^^^^^^^^^^^^^^^^^^^
6 
6 
- error: demangling(<issue_75326[4bb442b2d6d6d0a7]::Foo<_, _> as issue_75326[4bb442b2d6d6d0a7]::Iterator2>::next)
+ error: demangling(<issue_75326[521a81d3bf6bae49]::Foo<_, _> as issue_75326[521a81d3bf6bae49]::Iterator2>::next)
9    |
10 LL |     #[rustc_symbol_name]



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/issue-75326.v0/issue-75326.v0.stderr
To only update this specific test, also pass `--test-args symbol-names/issue-75326.rs`


error in revision `v0`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/symbol-names/issue-75326.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "v0" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/issue-75326.v0" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "symbol-mangling-version=v0" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/issue-75326.v0/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: symbol-name(_RNvXINICs732brBPyJ7x_11issue_75326s_0pppEINtB5_3FooppENtB5_9Iterator24nextB5_)
   |
LL |     #[rustc_symbol_name]
   |     ^^^^^^^^^^^^^^^^^^^^


error: demangling(<issue_75326[521a81d3bf6bae49]::Foo<_, _> as issue_75326[521a81d3bf6bae49]::Iterator2>::next)
   |
LL |     #[rustc_symbol_name]
   |     ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<issue_75326::Foo<_, _> as issue_75326::Iterator2>::next)
   |
LL |     #[rustc_symbol_name]
   |     ^^^^^^^^^^^^^^^^^^^^

---

---- [ui] ui/symbol-names/issue-60925.rs#legacy stdout ----
diff of stderr:

- error: symbol-name(_ZN11issue_609253foo37Foo$LT$issue_60925..llv$u6d$..Foo$GT$3foo17hf506f7e7e288f83dE)
+ error: symbol-name(_ZN11issue_609253foo37Foo$LT$issue_60925..llv$u6d$..Foo$GT$3foo17h2d133f6360bb67f8E)
3    |
4 LL |         #[rustc_symbol_name]

5    |         ^^^^^^^^^^^^^^^^^^^^
5    |         ^^^^^^^^^^^^^^^^^^^^
6 
- error: demangling(issue_60925::foo::Foo<issue_60925::llvm::Foo>::foo::hf506f7e7e288f83d)
+ error: demangling(issue_60925::foo::Foo<issue_60925::llvm::Foo>::foo::h2d133f6360bb67f8)
9    |
10 LL |         #[rustc_symbol_name]



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/issue-60925.legacy/issue-60925.legacy.stderr
To only update this specific test, also pass `--test-args symbol-names/issue-60925.rs`


error in revision `legacy`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/symbol-names/issue-60925.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "legacy" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/issue-60925.legacy" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "symbol-mangling-version=legacy" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/issue-60925.legacy/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: symbol-name(_ZN11issue_609253foo37Foo$LT$issue_60925..llv$u6d$..Foo$GT$3foo17h2d133f6360bb67f8E)
   |
LL |         #[rustc_symbol_name]
   |         ^^^^^^^^^^^^^^^^^^^^


error: demangling(issue_60925::foo::Foo<issue_60925::llvm::Foo>::foo::h2d133f6360bb67f8)
   |
LL |         #[rustc_symbol_name]
   |         ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(issue_60925::foo::Foo<issue_60925::llvm::Foo>::foo)
   |
LL |         #[rustc_symbol_name]
   |         ^^^^^^^^^^^^^^^^^^^^


error: aborting due to 3 previous errors


------------------------------------------


---- [ui] ui/symbol-names/trait-objects.rs#v0 stdout ----

4 LL |     #[rustc_symbol_name]
5    |     ^^^^^^^^^^^^^^^^^^^^
6 
6 
- error: demangling(<&dyn for<'a> core[HASH]::ops::function::FnMut<(&'a u8,), Output = ()> as trait_objects[970028982d289a87]::Bar>::method)
+ error: demangling(<&dyn for<'a> core[HASH]::ops::function::FnMut<(&'a u8,), Output = ()> as trait_objects[a345f623025f74fd]::Bar>::method)
9    |
10 LL |     #[rustc_symbol_name]

22 LL |     #[rustc_symbol_name]
22 LL |     #[rustc_symbol_name]
23    |     ^^^^^^^^^^^^^^^^^^^^
24 
- error: demangling(<&dyn for<'a> core[HASH]::ops::function::FnMut<(&'a u8,), Output = ()> + core[HASH]::marker::Send as trait_objects[970028982d289a87]::Foo>::method)
+ error: demangling(<&dyn for<'a> core[HASH]::ops::function::FnMut<(&'a u8,), Output = ()> + core[HASH]::marker::Send as trait_objects[a345f623025f74fd]::Foo>::method)
27    |
28 LL |     #[rustc_symbol_name]

40 LL |     #[rustc_symbol_name]
40 LL |     #[rustc_symbol_name]
41    |     ^^^^^^^^^^^^^^^^^^^^
42 
- error: demangling(<&dyn for<'a> core[HASH]::ops::function::FnMut<(&'a u8,), Output = ()> + core[HASH]::marker::Send as trait_objects[970028982d289a87]::Baz>::method)
+ error: demangling(<&dyn for<'a> core[HASH]::ops::function::FnMut<(&'a u8,), Output = ()> + core[HASH]::marker::Send as trait_objects[a345f623025f74fd]::Baz>::method)
45    |
46 LL |     #[rustc_symbol_name]



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/trait-objects.v0/trait-objects.v0.stderr
To only update this specific test, also pass `--test-args symbol-names/trait-objects.rs`


error in revision `v0`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/symbol-names/trait-objects.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "v0" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/trait-objects.v0" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "symbol-mangling-version=v0" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/trait-objects.v0/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: symbol-name(_RNvXCse16bV7JJGVZ_13trait_objectsRDG_INtNtNtCs5joVgQWFWp6_4core3ops8function5FnMutTRL0_hEEp6OutputuEL_NtB2_3Bar6method)
   |
LL |     #[rustc_symbol_name]
   |     ^^^^^^^^^^^^^^^^^^^^


error: demangling(<&dyn for<'a> core[3de20f3aa4c36156]::ops::function::FnMut<(&'a u8,), Output = ()> as trait_objects[a345f623025f74fd]::Bar>::method)
   |
LL |     #[rustc_symbol_name]
   |     ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<&dyn for<'a> core::ops::function::FnMut<(&'a u8,), Output = ()> as trait_objects::Bar>::method)
   |
LL |     #[rustc_symbol_name]
   |     ^^^^^^^^^^^^^^^^^^^^


error: symbol-name(_RNvXs_Cse16bV7JJGVZ_13trait_objectsRDG_INtNtNtCs5joVgQWFWp6_4core3ops8function5FnMutTRL0_hEEp6OutputuNtNtBI_6marker4SendEL_NtB4_3Foo6method)
   |
LL |     #[rustc_symbol_name]
   |     ^^^^^^^^^^^^^^^^^^^^


error: demangling(<&dyn for<'a> core[3de20f3aa4c36156]::ops::function::FnMut<(&'a u8,), Output = ()> + core[3de20f3aa4c36156]::marker::Send as trait_objects[a345f623025f74fd]::Foo>::method)
   |
LL |     #[rustc_symbol_name]
   |     ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<&dyn for<'a> core::ops::function::FnMut<(&'a u8,), Output = ()> + core::marker::Send as trait_objects::Foo>::method)
   |
LL |     #[rustc_symbol_name]
   |     ^^^^^^^^^^^^^^^^^^^^


error: symbol-name(_RNvXs0_Cse16bV7JJGVZ_13trait_objectsRDG_INtNtNtCs5joVgQWFWp6_4core3ops8function5FnMutTRL0_hEEp6OutputuNtNtBJ_6marker4SendEL_NtB5_3Baz6method)
   |
LL |     #[rustc_symbol_name]
   |     ^^^^^^^^^^^^^^^^^^^^


error: demangling(<&dyn for<'a> core[3de20f3aa4c36156]::ops::function::FnMut<(&'a u8,), Output = ()> + core[3de20f3aa4c36156]::marker::Send as trait_objects[a345f623025f74fd]::Baz>::method)
   |
LL |     #[rustc_symbol_name]
   |     ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<&dyn for<'a> core::ops::function::FnMut<(&'a u8,), Output = ()> + core::marker::Send as trait_objects::Baz>::method)
   |
LL |     #[rustc_symbol_name]
   |     ^^^^^^^^^^^^^^^^^^^^

---

---- [ui] ui/thir-tree.rs stdout ----
diff of stdout:

- DefId(0:3 ~ thir_tree[19f3]::main):
+ DefId(0:3 ~ thir_tree[5eaf]::main):
2 Thir {
3     arms: [],
4     exprs: [
30                 region_scope: Node(2),
31                 lint_level: Explicit(
32                     HirId {
32                     HirId {
-                         owner: DefId(0:3 ~ thir_tree[19f3]::main),
+                         owner: DefId(0:3 ~ thir_tree[5eaf]::main),
34                         local_id: 2,
36                 ),


The actual stdout differed from the expected stdout.
The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/thir-tree/thir-tree.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args thir-tree.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/thir-tree.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/thir-tree" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unpretty=thir-tree" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/thir-tree/auxiliary"
------------------------------------------
------------------------------------------
DefId(0:3 ~ thir_tree[5eaf]::main):
Thir {
    arms: [],
    exprs: [
        Expr {
            ty: (),
            temp_lifetime: Some(
            ),
            ),
            span: /checkout/src/test/ui/thir-tree.rs:4:15: 4:17 (#0),
            kind: Block {
                body: Block {
                    targeted_by_break: false,
                    region_scope: Node(1),
                    opt_destruction_scope: None,
                    span: /checkout/src/test/ui/thir-tree.rs:4:15: 4:17 (#0),
                    stmts: [],
                    expr: None,
                    safety_mode: Safe,
            },
        },
        Expr {
            ty: (),
            ty: (),
            temp_lifetime: Some(
                Node(2),
            ),
            span: /checkout/src/test/ui/thir-tree.rs:4:15: 4:17 (#0),
            kind: Scope {
                region_scope: Node(2),
                lint_level: Explicit(
                    HirId {
                        owner: DefId(0:3 ~ thir_tree[5eaf]::main),
                        local_id: 2,
                ),
                value: e0,
            },
        },
        },
        Expr {
            ty: (),
            temp_lifetime: Some(
                Node(2),
            ),
            span: /checkout/src/test/ui/thir-tree.rs:4:15: 4:17 (#0),
            kind: Scope {
                region_scope: Destruction(2),
                lint_level: Inherited,
                value: e1,
        },
    ],
    stmts: [],
}
---
test result: FAILED. 12152 passed; 15 failed; 117 ignored; 0 measured; 0 filtered out; finished in 133.44s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:19
