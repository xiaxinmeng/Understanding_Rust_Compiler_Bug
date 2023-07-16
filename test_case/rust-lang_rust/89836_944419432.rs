plain
.................................................................................................... 10600/12296
.................................................................................................... 10700/12296
.................................................................................................... 10800/12296
.................................................................................................... 10900/12296
...............................F..F.....FFF...FF..F.FFFii.....F.........................i........... 11000/12296
.......F............................................................................................ 11100/12296
.................................................................................................... 11300/12296
.................................................................................................... 11400/12296
.................................................................................................... 11500/12296
.....................i.............................................................................. 11600/12296
---
diff of stderr:

2   --> $DIR/tls.rs:12:25
3    |
4 LL |     unsafe { let _val = A; }
-    |                         ^ cannot access thread local static (DefId(0:6 ~ tls[f423]::A))
+    |                         ^ cannot access thread local static (DefId(0:6 ~ tls[bcc9]::A))
7 error[E0080]: could not evaluate static initializer
8   --> $DIR/tls.rs:19:26

9    |
9    |
10 LL |     unsafe { let _val = &A; }
-    |                          ^ cannot access thread local static (DefId(0:6 ~ tls[f423]::A))
+    |                          ^ cannot access thread local static (DefId(0:6 ~ tls[bcc9]::A))
13 warning: skipping const checks
14    |



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/tls/tls.stderr
To update references, rerun the tests and pass the `--bless` flag
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
   |                         ^ cannot access thread local static (DefId(0:6 ~ tls[bcc9]::A))
error[E0080]: could not evaluate static initializer
  --> /checkout/src/test/ui/consts/miri_unleashed/tls.rs:19:26
   |
   |
LL |     unsafe { let _val = &A; }
   |                          ^ cannot access thread local static (DefId(0:6 ~ tls[bcc9]::A))
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
-    |             ------------- has type `Opaque(DefId(0:34 ~ generator_print_verbose_1[70c9]::make_non_send_generator::{opaque#0}), [])` which is not `Send`
+    |             ------------- has type `Opaque(DefId(0:34 ~ generator_print_verbose_1[b356]::make_non_send_generator::{opaque#0}), [])` which is not `Send`
13 LL |         yield;
14    |         ^^^^^ yield occurs here, with `_non_send_gen` maybe used later


29    = help: the trait `Sync` is not implemented for `RefCell<i32>`
30    = note: required because of the requirements on the impl of `Send` for `Arc<RefCell<i32>>`
31    = note: required because it appears within the type `[make_gen2<Arc<RefCell<i32>>>::{closure#0} upvar_tys=(Arc<RefCell<i32>>) {()}]`
-    = note: required because it appears within the type `Opaque(DefId(0:39 ~ generator_print_verbose_1[70c9]::make_gen2::{opaque#0}), [std::sync::Arc<std::cell::RefCell<i32>>])`
-    = note: required because it appears within the type `Opaque(DefId(0:42 ~ generator_print_verbose_1[70c9]::make_non_send_generator2::{opaque#0}), [])`
-    = note: required because it appears within the type `{Opaque(DefId(0:42 ~ generator_print_verbose_1[70c9]::make_non_send_generator2::{opaque#0}), []), ()}`
-    = note: required because it appears within the type `[test2::{closure#0} upvar_tys=() {Opaque(DefId(0:42 ~ generator_print_verbose_1[70c9]::make_non_send_generator2::{opaque#0}), []), ()}]`
+    = note: required because it appears within the type `Opaque(DefId(0:39 ~ generator_print_verbose_1[b356]::make_gen2::{opaque#0}), [std::sync::Arc<std::cell::RefCell<i32>>])`
+    = note: required because it appears within the type `Opaque(DefId(0:42 ~ generator_print_verbose_1[b356]::make_non_send_generator2::{opaque#0}), [])`
+    = note: required because it appears within the type `{Opaque(DefId(0:42 ~ generator_print_verbose_1[b356]::make_non_send_generator2::{opaque#0}), []), ()}`
+    = note: required because it appears within the type `[test2::{closure#0} upvar_tys=() {Opaque(DefId(0:42 ~ generator_print_verbose_1[b356]::make_non_send_generator2::{opaque#0}), []), ()}]`
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
   |             ------------- has type `Opaque(DefId(0:34 ~ generator_print_verbose_1[b356]::make_non_send_generator::{opaque#0}), [])` which is not `Send`
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
   = note: required because it appears within the type `Opaque(DefId(0:39 ~ generator_print_verbose_1[b356]::make_gen2::{opaque#0}), [std::sync::Arc<std::cell::RefCell<i32>>])`
   = note: required because it appears within the type `Opaque(DefId(0:42 ~ generator_print_verbose_1[b356]::make_non_send_generator2::{opaque#0}), [])`
   = note: required because it appears within the type `{Opaque(DefId(0:42 ~ generator_print_verbose_1[b356]::make_non_send_generator2::{opaque#0}), []), ()}`
   = note: required because it appears within the type `[test2::{closure#0} upvar_tys=() {Opaque(DefId(0:42 ~ generator_print_verbose_1[b356]::make_non_send_generator2::{opaque#0}), []), ()}]`
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
- error: failed to load bc of "lto-duplicate-symbols2.lto_duplicate_symbols2.693a75b4-cgu.0.rcgu.o": 
+ error: failed to load bc of "lto-duplicate-symbols2.lto_duplicate_symbols2.4f3a20da-cgu.0.rcgu.o": 
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

error: failed to load bc of "lto-duplicate-symbols2.lto_duplicate_symbols2.4f3a20da-cgu.0.rcgu.o": 
error: aborting due to previous error; 1 warning emitted


------------------------------------------
------------------------------------------


---- [ui] ui/specialization/min_specialization/repeated_projection_type.rs stdout ----
diff of stderr:

- error: cannot specialize on `Binder(ProjectionPredicate(ProjectionTy { substs: [V], item_def_id: DefId(0:6 ~ repeated_projection_type[b09c]::Id::This) }, (I,)), [])`
+ error: cannot specialize on `Binder(ProjectionPredicate(ProjectionTy { substs: [V], item_def_id: DefId(0:6 ~ repeated_projection_type[1d00]::Id::This) }, (I,)), [])`
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
error: cannot specialize on `Binder(ProjectionPredicate(ProjectionTy { substs: [V], item_def_id: DefId(0:6 ~ repeated_projection_type[1d00]::Id::This) }, (I,)), [])`
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

- error: symbol-name(_ZN5basic4main17hd75b915511563828E)
-   --> $DIR/basic.rs:8:1
+ error: symbol-name(_ZN5basic4main17h2e53f9f1d9b17650E)
3    |
4 LL | #[rustc_symbol_name]
5    | ^^^^^^^^^^^^^^^^^^^^


6 
- error: demangling(basic::main::hd75b915511563828)
-   --> $DIR/basic.rs:8:1
+ error: demangling(basic::main::h2e53f9f1d9b17650)
9    |
10 LL | #[rustc_symbol_name]
11    | ^^^^^^^^^^^^^^^^^^^^


12 
13 error: demangling-alt(basic::main)
-   --> $DIR/basic.rs:8:1
15    |
16 LL | #[rustc_symbol_name]
17    | ^^^^^^^^^^^^^^^^^^^^


18 
19 error: def-path(main)
-   --> $DIR/basic.rs:15:1
+   --> $DIR/basic.rs:17:1
21    |
22 LL | #[rustc_def_path]


The actual stderr differed from the expected stderr.
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
error: symbol-name(_ZN5basic4main17h2e53f9f1d9b17650E)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling(basic::main::h2e53f9f1d9b17650)
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


- error: symbol-name(_RNvCsCRATE_HASH5basic4main)
-   --> $DIR/basic.rs:8:1
+ error: symbol-name(_RNvCRATE_HASH5basic4main)
3    |
4 LL | #[rustc_symbol_name]
5    | ^^^^^^^^^^^^^^^^^^^^


6 
- error: demangling(basic[HASH]::main)
-   --> $DIR/basic.rs:8:1
+ error: demangling(basic[37c7dd94fdaa5a7a]::main)
9    |
10 LL | #[rustc_symbol_name]
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
11    | ^^^^^^^^^^^^^^^^^^^^
11    | ^^^^^^^^^^^^^^^^^^^^

12 
13 error: demangling-alt(basic::main)
-   --> $DIR/basic.rs:8:1
15    |
16 LL | #[rustc_symbol_name]
17    | ^^^^^^^^^^^^^^^^^^^^


18 
19 error: def-path(main)
-   --> $DIR/basic.rs:15:1
+   --> $DIR/basic.rs:17:1
21    |
22 LL | #[rustc_def_path]


The actual stderr differed from the expected stderr.
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
error: symbol-name(_RNvCs4MV0w3oCKAo_5basic4main)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling(basic[37c7dd94fdaa5a7a]::main)
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


---- [ui] ui/symbol-names/const-generics-demangling.rs stdout ----
diff of stderr:

- error: symbol-name(_RMCRATE_HASH_1cINtB0_8UnsignedKhb_E)
-   --> $DIR/const-generics-demangling.rs:7:1
+ error: symbol-name(_RMCRATE_HASH1cINtB0_8UnsignedKhb_E)
3    |
4 LL | #[rustc_symbol_name]
5    | ^^^^^^^^^^^^^^^^^^^^


6 
- error: demangling(<c[HASH]::Unsigned<11u8>>)
-   --> $DIR/const-generics-demangling.rs:7:1
+ error: demangling(<c[ead6075fee7cea07]::Unsigned<11u8>>)
9    |
10 LL | #[rustc_symbol_name]
11    | ^^^^^^^^^^^^^^^^^^^^


12 
13 error: demangling-alt(<c::Unsigned<11>>)
-   --> $DIR/const-generics-demangling.rs:7:1
15    |
16 LL | #[rustc_symbol_name]
17    | ^^^^^^^^^^^^^^^^^^^^


18 
- error: symbol-name(_RMs_CRATE_HASH_1cINtB2_6SignedKsn98_E)
-   --> $DIR/const-generics-demangling.rs:15:1
+ error: symbol-name(_RMs_CRATE_HASH1cINtB2_6SignedKsn98_E)
21    |
22 LL | #[rustc_symbol_name]
23    | ^^^^^^^^^^^^^^^^^^^^


24 
- error: demangling(<c[HASH]::Signed<-152i16>>)
-   --> $DIR/const-generics-demangling.rs:15:1
+ error: demangling(<c[ead6075fee7cea07]::Signed<-152i16>>)
27    |
28 LL | #[rustc_symbol_name]
29    | ^^^^^^^^^^^^^^^^^^^^


30 
31 error: demangling-alt(<c::Signed<-152>>)
-   --> $DIR/const-generics-demangling.rs:15:1
33    |
34 LL | #[rustc_symbol_name]
35    | ^^^^^^^^^^^^^^^^^^^^


36 
- error: symbol-name(_RMs0_CRATE_HASH_1cINtB3_4BoolKb1_E)
-   --> $DIR/const-generics-demangling.rs:23:1
+ error: symbol-name(_RMs0_CRATE_HASH1cINtB3_4BoolKb1_E)
39    |
40 LL | #[rustc_symbol_name]
41    | ^^^^^^^^^^^^^^^^^^^^


42 
- error: demangling(<c[HASH]::Bool<true>>)
-   --> $DIR/const-generics-demangling.rs:23:1
+ error: demangling(<c[ead6075fee7cea07]::Bool<true>>)
45    |
46 LL | #[rustc_symbol_name]
47    | ^^^^^^^^^^^^^^^^^^^^


48 
49 error: demangling-alt(<c::Bool<true>>)
-   --> $DIR/const-generics-demangling.rs:23:1
51    |
52 LL | #[rustc_symbol_name]
53    | ^^^^^^^^^^^^^^^^^^^^


54 
- error: symbol-name(_RMs1_CRATE_HASH_1cINtB3_4CharKc2202_E)
-   --> $DIR/const-generics-demangling.rs:31:1
+ error: symbol-name(_RMs1_CRATE_HASH1cINtB3_4CharKc2202_E)
57    |
58 LL | #[rustc_symbol_name]
59    | ^^^^^^^^^^^^^^^^^^^^


60 
- error: demangling(<c[HASH]::Char<'∂'>>)
-   --> $DIR/const-generics-demangling.rs:31:1
+ error: demangling(<c[ead6075fee7cea07]::Char<'∂'>>)
63    |
64 LL | #[rustc_symbol_name]
65    | ^^^^^^^^^^^^^^^^^^^^


66 
67 error: demangling-alt(<c::Char<'∂'>>)
-   --> $DIR/const-generics-demangling.rs:31:1
69    |
70 LL | #[rustc_symbol_name]
71    | ^^^^^^^^^^^^^^^^^^^^

---
To only update this specific test, also pass `--test-args symbol-names/const-generics-demangling.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/symbol-names/const-generics-demangling.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/const-generics-demangling" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "symbol-mangling-version=v0" "--crate-name=c" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/const-generics-demangling/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: symbol-name(_RMCska1EXlzrq0B_1cINtB0_8UnsignedKhb_E)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling(<c[ead6075fee7cea07]::Unsigned<11u8>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<c::Unsigned<11>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: symbol-name(_RMs_Cska1EXlzrq0B_1cINtB2_6SignedKsn98_E)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling(<c[ead6075fee7cea07]::Signed<-152i16>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<c::Signed<-152>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: symbol-name(_RMs0_Cska1EXlzrq0B_1cINtB3_4BoolKb1_E)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling(<c[ead6075fee7cea07]::Bool<true>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<c::Bool<true>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: symbol-name(_RMs1_Cska1EXlzrq0B_1cINtB3_4CharKc2202_E)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling(<c[ead6075fee7cea07]::Char<'∂'>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<c::Char<'∂'>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^

---

---- [ui] ui/symbol-names/impl1.rs#legacy stdout ----
diff of stderr:

1 error: symbol-name(_ZN5impl13foo3Foo3bar17<SYMBOL_HASH>)
-   --> $DIR/impl1.rs:14:9
3    |
4 LL |         #[rustc_symbol_name]
5    |         ^^^^^^^^^^^^^^^^^^^^


6 
7 error: demangling(impl1::foo::Foo::bar::<SYMBOL_HASH>)
-   --> $DIR/impl1.rs:14:9
9    |
10 LL |         #[rustc_symbol_name]
11    |         ^^^^^^^^^^^^^^^^^^^^


12 
13 error: demangling-alt(impl1::foo::Foo::bar)
-   --> $DIR/impl1.rs:14:9
15    |
16 LL |         #[rustc_symbol_name]
17    |         ^^^^^^^^^^^^^^^^^^^^


18 
19 error: def-path(foo::Foo::bar)
-   --> $DIR/impl1.rs:21:9
21    |
22 LL |         #[rustc_def_path]
23    |         ^^^^^^^^^^^^^^^^^


24 
25 error: symbol-name(_ZN5impl13bar33_$LT$impl$u20$impl1..foo..Foo$GT$3baz17<SYMBOL_HASH>)
-   --> $DIR/impl1.rs:32:9
27    |
28 LL |         #[rustc_symbol_name]
29    |         ^^^^^^^^^^^^^^^^^^^^


30 
31 error: demangling(impl1::bar::<impl impl1::foo::Foo>::baz::<SYMBOL_HASH>)
-   --> $DIR/impl1.rs:32:9
33    |
34 LL |         #[rustc_symbol_name]
35    |         ^^^^^^^^^^^^^^^^^^^^


36 
37 error: demangling-alt(impl1::bar::<impl impl1::foo::Foo>::baz)
-   --> $DIR/impl1.rs:32:9
39    |
40 LL |         #[rustc_symbol_name]
41    |         ^^^^^^^^^^^^^^^^^^^^


42 
43 error: def-path(bar::<impl foo::Foo>::baz)
-   --> $DIR/impl1.rs:39:9
45    |
46 LL |         #[rustc_def_path]
47    |         ^^^^^^^^^^^^^^^^^


48 
49 error: symbol-name(_ZN209_$LT$$u5b$$RF$dyn$u20$impl1..Foo$u2b$Assoc$u20$$u3d$$u20$extern$u20$$u22$C$u22$$u20$fn$LP$$RF$u8$C$$u20$...$RP$$u2b$impl1..AutoTrait$u3b$$u20$3$u5d$$u20$as$u20$impl1..main..$u7b$$u7b$closure$u7d$$u7d$..Bar$GT$6method17<SYMBOL_HASH>)
-   --> $DIR/impl1.rs:62:13
51    |
52 LL |             #[rustc_symbol_name]
53    |             ^^^^^^^^^^^^^^^^^^^^


54 
55 error: demangling(<[&dyn impl1::Foo+Assoc = extern "C" fn(&u8, ::.)+impl1::AutoTrait; 3] as impl1::main::{{closure}}::Bar>::method::<SYMBOL_HASH>)
-   --> $DIR/impl1.rs:62:13
57    |
58 LL |             #[rustc_symbol_name]
59    |             ^^^^^^^^^^^^^^^^^^^^


60 
61 error: demangling-alt(<[&dyn impl1::Foo+Assoc = extern "C" fn(&u8, ::.)+impl1::AutoTrait; 3] as impl1::main::{{closure}}::Bar>::method)
-   --> $DIR/impl1.rs:62:13
63    |
64 LL |             #[rustc_symbol_name]
65    |             ^^^^^^^^^^^^^^^^^^^^


66 
67 error: def-path(<[&dyn Foo<Assoc = for<'r> extern "C" fn(&'r u8, ...)> + AutoTrait; 3] as main::{closure#1}::Bar>::method)
-   --> $DIR/impl1.rs:69:13
69    |
70 LL |             #[rustc_def_path]
71    |             ^^^^^^^^^^^^^^^^^



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/impl1.legacy/impl1.legacy.stderr
To only update this specific test, also pass `--test-args symbol-names/impl1.rs`


error in revision `legacy`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/symbol-names/impl1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "legacy" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/impl1.legacy" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "symbol-mangling-version=legacy" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/impl1.legacy/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: symbol-name(_ZN5impl13foo3Foo3bar17h78857374aa014a73E)
   |
LL |         #[rustc_symbol_name]
   |         ^^^^^^^^^^^^^^^^^^^^


error: demangling(impl1::foo::Foo::bar::h78857374aa014a73)
   |
LL |         #[rustc_symbol_name]
   |         ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(impl1::foo::Foo::bar)
   |
LL |         #[rustc_symbol_name]
   |         ^^^^^^^^^^^^^^^^^^^^


error: def-path(foo::Foo::bar)
   |
LL |         #[rustc_def_path]
   |         ^^^^^^^^^^^^^^^^^


error: symbol-name(_ZN5impl13bar33_$LT$impl$u20$impl1..foo..Foo$GT$3baz17hccfbb69ea37b6f43E)
   |
LL |         #[rustc_symbol_name]
   |         ^^^^^^^^^^^^^^^^^^^^


error: demangling(impl1::bar::<impl impl1::foo::Foo>::baz::hccfbb69ea37b6f43)
   |
LL |         #[rustc_symbol_name]
   |         ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(impl1::bar::<impl impl1::foo::Foo>::baz)
   |
LL |         #[rustc_symbol_name]
   |         ^^^^^^^^^^^^^^^^^^^^


error: def-path(bar::<impl foo::Foo>::baz)
   |
LL |         #[rustc_def_path]
   |         ^^^^^^^^^^^^^^^^^


error: symbol-name(_ZN209_$LT$$u5b$$RF$dyn$u20$impl1..Foo$u2b$Assoc$u20$$u3d$$u20$extern$u20$$u22$C$u22$$u20$fn$LP$$RF$u8$C$$u20$...$RP$$u2b$impl1..AutoTrait$u3b$$u20$3$u5d$$u20$as$u20$impl1..main..$u7b$$u7b$closure$u7d$$u7d$..Bar$GT$6method17h2e702ab3a9178dadE)
   |
LL |             #[rustc_symbol_name]
   |             ^^^^^^^^^^^^^^^^^^^^


error: demangling(<[&dyn impl1::Foo+Assoc = extern "C" fn(&u8, ::.)+impl1::AutoTrait; 3] as impl1::main::{{closure}}::Bar>::method::h2e702ab3a9178dad)
   |
LL |             #[rustc_symbol_name]
   |             ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<[&dyn impl1::Foo+Assoc = extern "C" fn(&u8, ::.)+impl1::AutoTrait; 3] as impl1::main::{{closure}}::Bar>::method)
   |
LL |             #[rustc_symbol_name]
   |             ^^^^^^^^^^^^^^^^^^^^


error: def-path(<[&dyn Foo<Assoc = for<'r> extern "C" fn(&'r u8, ...)> + AutoTrait; 3] as main::{closure#1}::Bar>::method)
   |
LL |             #[rustc_def_path]
   |             ^^^^^^^^^^^^^^^^^

---

---- [ui] ui/symbol-names/const-generics-str-demangling.rs stdout ----
diff of stderr:

- error: symbol-name(_RMCRATE_HASH_1cINtB0_3StrKRe616263_E)
-   --> $DIR/const-generics-str-demangling.rs:8:1
+ error: symbol-name(_RMCRATE_HASH1cINtB0_3StrKRe616263_E)
3    |
4 LL | #[rustc_symbol_name]
5    | ^^^^^^^^^^^^^^^^^^^^


6 
- error: demangling(<c[HASH]::Str<"abc">>)
-   --> $DIR/const-generics-str-demangling.rs:8:1
+ error: demangling(<c[ead6075fee7cea07]::Str<"abc">>)
9    |
10 LL | #[rustc_symbol_name]
11    | ^^^^^^^^^^^^^^^^^^^^


12 
13 error: demangling-alt(<c::Str<"abc">>)
-   --> $DIR/const-generics-str-demangling.rs:8:1
15    |
16 LL | #[rustc_symbol_name]
17    | ^^^^^^^^^^^^^^^^^^^^


18 
- error: symbol-name(_RMs_CRATE_HASH_1cINtB2_3StrKRe27_E)
-   --> $DIR/const-generics-str-demangling.rs:14:1
+ error: symbol-name(_RMs_CRATE_HASH1cINtB2_3StrKRe27_E)
21    |
22 LL | #[rustc_symbol_name]
23    | ^^^^^^^^^^^^^^^^^^^^


24 
- error: demangling(<c[HASH]::Str<"'">>)
-   --> $DIR/const-generics-str-demangling.rs:14:1
+ error: demangling(<c[ead6075fee7cea07]::Str<"'">>)
27    |
28 LL | #[rustc_symbol_name]
29    | ^^^^^^^^^^^^^^^^^^^^


30 
31 error: demangling-alt(<c::Str<"'">>)
-   --> $DIR/const-generics-str-demangling.rs:14:1
33    |
34 LL | #[rustc_symbol_name]
35    | ^^^^^^^^^^^^^^^^^^^^


36 
- error: symbol-name(_RMs0_CRATE_HASH_1cINtB3_3StrKRe090a_E)
-   --> $DIR/const-generics-str-demangling.rs:20:1
+ error: symbol-name(_RMs0_CRATE_HASH1cINtB3_3StrKRe090a_E)
39    |
40 LL | #[rustc_symbol_name]
41    | ^^^^^^^^^^^^^^^^^^^^


42 
- error: demangling(<c[HASH]::Str<"\t\n">>)
-   --> $DIR/const-generics-str-demangling.rs:20:1
+ error: demangling(<c[ead6075fee7cea07]::Str<"\t\n">>)
45    |
46 LL | #[rustc_symbol_name]
47    | ^^^^^^^^^^^^^^^^^^^^


48 
49 error: demangling-alt(<c::Str<"\t\n">>)
-   --> $DIR/const-generics-str-demangling.rs:20:1
51    |
52 LL | #[rustc_symbol_name]
53    | ^^^^^^^^^^^^^^^^^^^^


54 
- error: symbol-name(_RMs1_CRATE_HASH_1cINtB3_3StrKRee28882c3bc_E)
-   --> $DIR/const-generics-str-demangling.rs:26:1
+ error: symbol-name(_RMs1_CRATE_HASH1cINtB3_3StrKRee28882c3bc_E)
57    |
58 LL | #[rustc_symbol_name]
59    | ^^^^^^^^^^^^^^^^^^^^


60 
- error: demangling(<c[HASH]::Str<"∂ü">>)
-   --> $DIR/const-generics-str-demangling.rs:26:1
+ error: demangling(<c[ead6075fee7cea07]::Str<"∂ü">>)
63    |
64 LL | #[rustc_symbol_name]
65    | ^^^^^^^^^^^^^^^^^^^^


66 
67 error: demangling-alt(<c::Str<"∂ü">>)
-   --> $DIR/const-generics-str-demangling.rs:26:1
69    |
70 LL | #[rustc_symbol_name]
71    | ^^^^^^^^^^^^^^^^^^^^


72 
- error: symbol-name(_RMs2_CRATE_HASH_1cINtB3_3StrKRee183a1e18390e183ade1839be18394e1839ae18390e183935fe18392e18394e1839be183a0e18398e18394e1839ae183985fe183a1e18390e18393e18398e1839ae18398_E)
-   --> $DIR/const-generics-str-demangling.rs:32:1
+ error: symbol-name(_RMs2_CRATE_HASH1cINtB3_3StrKRee183a1e18390e183ade1839be18394e1839ae18390e183935fe18392e18394e1839be183a0e18398e18394e1839ae183985fe183a1e18390e18393e18398e1839ae18398_E)
75    |
76 LL | #[rustc_symbol_name]
77    | ^^^^^^^^^^^^^^^^^^^^


78 
- error: demangling(<c[HASH]::Str<"საჭმელად_გემრიელი_სადილი">>)
-   --> $DIR/const-generics-str-demangling.rs:32:1
+ error: demangling(<c[ead6075fee7cea07]::Str<"საჭმელად_გემრიელი_სადილი">>)
81    |
82 LL | #[rustc_symbol_name]
83    | ^^^^^^^^^^^^^^^^^^^^


84 
85 error: demangling-alt(<c::Str<"საჭმელად_გემრიელი_სადილი">>)
-   --> $DIR/const-generics-str-demangling.rs:32:1
87    |
88 LL | #[rustc_symbol_name]
89    | ^^^^^^^^^^^^^^^^^^^^


90 
- error: symbol-name(_RMs3_CRATE_HASH_1cINtB3_3StrKRef09f908af09fa688f09fa686f09f90ae20c2a720f09f90b6f09f9192e29895f09f94a520c2a720f09fa7a1f09f929bf09f929af09f9299f09f929c_E)
-   --> $DIR/const-generics-str-demangling.rs:38:1
+ error: symbol-name(_RMs3_CRATE_HASH1cINtB3_3StrKRef09f908af09fa688f09fa686f09f90ae20c2a720f09f90b6f09f9192e29895f09f94a520c2a720f09fa7a1f09f929bf09f929af09f9299f09f929c_E)
93    |
94 LL | #[rustc_symbol_name]
95    | ^^^^^^^^^^^^^^^^^^^^


96 
- error: demangling(<c[HASH]::Str<"🐊🦈🦆🐮 § 🐶👒☕🔥 § 🧡💛💚💙💜">>)
-   --> $DIR/const-generics-str-demangling.rs:38:1
+ error: demangling(<c[ead6075fee7cea07]::Str<"🐊🦈🦆🐮 § 🐶👒☕🔥 § 🧡💛💚💙💜">>)
99    |
100 LL | #[rustc_symbol_name]
101    | ^^^^^^^^^^^^^^^^^^^^


102 
103 error: demangling-alt(<c::Str<"🐊🦈🦆🐮 § 🐶👒☕🔥 § 🧡💛💚💙💜">>)
-   --> $DIR/const-generics-str-demangling.rs:38:1
105    |
106 LL | #[rustc_symbol_name]
107    | ^^^^^^^^^^^^^^^^^^^^

---
To only update this specific test, also pass `--test-args symbol-names/const-generics-str-demangling.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/symbol-names/const-generics-str-demangling.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/const-generics-str-demangling" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "symbol-mangling-version=v0" "--crate-name=c" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/const-generics-str-demangling/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: symbol-name(_RMCska1EXlzrq0B_1cINtB0_3StrKRe616263_E)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling(<c[ead6075fee7cea07]::Str<"abc">>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<c::Str<"abc">>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: symbol-name(_RMs_Cska1EXlzrq0B_1cINtB2_3StrKRe27_E)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling(<c[ead6075fee7cea07]::Str<"'">>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<c::Str<"'">>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: symbol-name(_RMs0_Cska1EXlzrq0B_1cINtB3_3StrKRe090a_E)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling(<c[ead6075fee7cea07]::Str<"\t\n">>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<c::Str<"\t\n">>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: symbol-name(_RMs1_Cska1EXlzrq0B_1cINtB3_3StrKRee28882c3bc_E)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling(<c[ead6075fee7cea07]::Str<"∂ü">>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<c::Str<"∂ü">>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: symbol-name(_RMs2_Cska1EXlzrq0B_1cINtB3_3StrKRee183a1e18390e183ade1839be18394e1839ae18390e183935fe18392e18394e1839be183a0e18398e18394e1839ae183985fe183a1e18390e18393e18398e1839ae18398_E)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling(<c[ead6075fee7cea07]::Str<"საჭმელად_გემრიელი_სადილი">>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<c::Str<"საჭმელად_გემრიელი_სადილი">>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: symbol-name(_RMs3_Cska1EXlzrq0B_1cINtB3_3StrKRef09f908af09fa688f09fa686f09f90ae20c2a720f09f90b6f09f9192e29895f09f94a520c2a720f09fa7a1f09f929bf09f929af09f9299f09f929c_E)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling(<c[ead6075fee7cea07]::Str<"🐊🦈🦆🐮 § 🐶👒☕🔥 § 🧡💛💚💙💜">>)
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


- error: symbol-name(_RNvMNtCRATE_HASH_5impl13fooNtB2_3Foo3bar)
-   --> $DIR/impl1.rs:14:9
+ error: symbol-name(_RNvMNtCRATE_HASH5impl13fooNtB2_3Foo3bar)
3    |
4 LL |         #[rustc_symbol_name]
5    |         ^^^^^^^^^^^^^^^^^^^^


6 
- error: demangling(<impl1[HASH]::foo::Foo>::bar)
-   --> $DIR/impl1.rs:14:9
+ error: demangling(<impl1[61bda61d80c47f0f]::foo::Foo>::bar)
9    |
10 LL |         #[rustc_symbol_name]
11    |         ^^^^^^^^^^^^^^^^^^^^


12 
13 error: demangling-alt(<impl1::foo::Foo>::bar)
-   --> $DIR/impl1.rs:14:9
15    |
16 LL |         #[rustc_symbol_name]
17    |         ^^^^^^^^^^^^^^^^^^^^


18 
19 error: def-path(foo::Foo::bar)
-   --> $DIR/impl1.rs:21:9
21    |
22 LL |         #[rustc_def_path]
23    |         ^^^^^^^^^^^^^^^^^


24 
- error: symbol-name(_RNvMNtCRATE_HASH_5impl13barNtNtB4_3foo3Foo3baz)
-   --> $DIR/impl1.rs:32:9
+ error: symbol-name(_RNvMNtCRATE_HASH5impl13barNtNtB4_3foo3Foo3baz)
27    |
28 LL |         #[rustc_symbol_name]
29    |         ^^^^^^^^^^^^^^^^^^^^


30 
- error: demangling(<impl1[HASH]::foo::Foo>::baz)
-   --> $DIR/impl1.rs:32:9
+ error: demangling(<impl1[61bda61d80c47f0f]::foo::Foo>::baz)
33    |
34 LL |         #[rustc_symbol_name]
35    |         ^^^^^^^^^^^^^^^^^^^^


36 
37 error: demangling-alt(<impl1::foo::Foo>::baz)
-   --> $DIR/impl1.rs:32:9
39    |
40 LL |         #[rustc_symbol_name]
41    |         ^^^^^^^^^^^^^^^^^^^^


42 
43 error: def-path(bar::<impl foo::Foo>::baz)
-   --> $DIR/impl1.rs:39:9
45    |
46 LL |         #[rustc_def_path]
47    |         ^^^^^^^^^^^^^^^^^


48 
- error: symbol-name(_RNvXNCNvCRATE_HASH_5impl14mains_0ARDNtB6_3Foop5AssocFG_KCRL0_hvEuNtB6_9AutoTraitEL_j3_NtB2_3Bar6method)
-   --> $DIR/impl1.rs:62:13
+ error: symbol-name(_RNvXNCNvCRATE_HASH5impl14mains_0ARDNtB6_3Foop5AssocFG_KCRL0_hvEuNtB6_9AutoTraitEL_j3_NtB2_3Bar6method)
51    |
52 LL |             #[rustc_symbol_name]
53    |             ^^^^^^^^^^^^^^^^^^^^


54 
- error: demangling(<[&dyn impl1[HASH]::Foo<Assoc = for<'a> extern "C" fn(&'a u8, ...)> + impl1[HASH]::AutoTrait; 3usize] as impl1[HASH]::main::{closure#1}::Bar>::method)
-   --> $DIR/impl1.rs:62:13
+ error: demangling(<[&dyn impl1[61bda61d80c47f0f]::Foo<Assoc = for<'a> extern "C" fn(&'a u8, ...)> + impl1[61bda61d80c47f0f]::AutoTrait; 3usize] as impl1[61bda61d80c47f0f]::main::{closure#1}::Bar>::method)
57    |
58 LL |             #[rustc_symbol_name]
59    |             ^^^^^^^^^^^^^^^^^^^^


60 
61 error: demangling-alt(<[&dyn impl1::Foo<Assoc = for<'a> extern "C" fn(&'a u8, ...)> + impl1::AutoTrait; 3] as impl1::main::{closure#1}::Bar>::method)
-   --> $DIR/impl1.rs:62:13
63    |
64 LL |             #[rustc_symbol_name]
65    |             ^^^^^^^^^^^^^^^^^^^^


66 
67 error: def-path(<[&dyn Foo<Assoc = for<'r> extern "C" fn(&'r u8, ...)> + AutoTrait; 3] as main::{closure#1}::Bar>::method)
-   --> $DIR/impl1.rs:69:13
69    |
70 LL |             #[rustc_def_path]
71    |             ^^^^^^^^^^^^^^^^^



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
error: symbol-name(_RNvMNtCs8ogRPSlxeW1_5impl13fooNtB2_3Foo3bar)
   |
LL |         #[rustc_symbol_name]
   |         ^^^^^^^^^^^^^^^^^^^^


error: demangling(<impl1[61bda61d80c47f0f]::foo::Foo>::bar)
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


error: symbol-name(_RNvMNtCs8ogRPSlxeW1_5impl13barNtNtB4_3foo3Foo3baz)
   |
LL |         #[rustc_symbol_name]
   |         ^^^^^^^^^^^^^^^^^^^^


error: demangling(<impl1[61bda61d80c47f0f]::foo::Foo>::baz)
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


error: symbol-name(_RNvXNCNvCs8ogRPSlxeW1_5impl14mains_0ARDNtB6_3Foop5AssocFG_KCRL0_hvEuNtB6_9AutoTraitEL_j3_NtB2_3Bar6method)
   |
LL |             #[rustc_symbol_name]
   |             ^^^^^^^^^^^^^^^^^^^^


error: demangling(<[&dyn impl1[61bda61d80c47f0f]::Foo<Assoc = for<'a> extern "C" fn(&'a u8, ...)> + impl1[61bda61d80c47f0f]::AutoTrait; 3usize] as impl1[61bda61d80c47f0f]::main::{closure#1}::Bar>::method)
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

---- [ui] ui/symbol-names/const-generics-structural-demangling.rs stdout ----
diff of stderr:

- error: symbol-name(_RMCsno73SFvQKx_1cINtB0_7RefByteKRh7b_E)
+ error: symbol-name(_RMCska1EXlzrq0B_1cINtB0_7RefByteKRh7b_E)
3    |
4 LL | #[rustc_symbol_name]

5    | ^^^^^^^^^^^^^^^^^^^^
5    | ^^^^^^^^^^^^^^^^^^^^
6 
- error: demangling(<c[464da6a86cb672f]::RefByte<{&123u8}>>)
+ error: demangling(<c[ead6075fee7cea07]::RefByte<{&123u8}>>)
9    |
10 LL | #[rustc_symbol_name]

16 LL | #[rustc_symbol_name]
16 LL | #[rustc_symbol_name]
17    | ^^^^^^^^^^^^^^^^^^^^
18 
- error: symbol-name(_RMs_Csno73SFvQKx_1cINtB2_6RefZstKRAEE)
+ error: symbol-name(_RMs_Cska1EXlzrq0B_1cINtB2_6RefZstKRAEE)
21    |
22 LL | #[rustc_symbol_name]

23    | ^^^^^^^^^^^^^^^^^^^^
23    | ^^^^^^^^^^^^^^^^^^^^
24 
- error: demangling(<c[464da6a86cb672f]::RefZst<{&[]}>>)
+ error: demangling(<c[ead6075fee7cea07]::RefZst<{&[]}>>)
27    |
28 LL | #[rustc_symbol_name]

34 LL | #[rustc_symbol_name]
34 LL | #[rustc_symbol_name]
35    | ^^^^^^^^^^^^^^^^^^^^
36 
- error: symbol-name(_RMs0_Csno73SFvQKx_1cINtB3_11Array3BytesKAh1_h2_h3_EE)
+ error: symbol-name(_RMs0_Cska1EXlzrq0B_1cINtB3_11Array3BytesKAh1_h2_h3_EE)
39    |
40 LL | #[rustc_symbol_name]

41    | ^^^^^^^^^^^^^^^^^^^^
41    | ^^^^^^^^^^^^^^^^^^^^
42 
- error: demangling(<c[464da6a86cb672f]::Array3Bytes<{[1u8, 2u8, 3u8]}>>)
+ error: demangling(<c[ead6075fee7cea07]::Array3Bytes<{[1u8, 2u8, 3u8]}>>)
45    |
46 LL | #[rustc_symbol_name]

52 LL | #[rustc_symbol_name]
52 LL | #[rustc_symbol_name]
53    | ^^^^^^^^^^^^^^^^^^^^
54 
- error: symbol-name(_RMs1_Csno73SFvQKx_1cINtB3_13TupleByteBoolKTh1_b0_EE)
+ error: symbol-name(_RMs1_Cska1EXlzrq0B_1cINtB3_13TupleByteBoolKTh1_b0_EE)
57    |
58 LL | #[rustc_symbol_name]

59    | ^^^^^^^^^^^^^^^^^^^^
59    | ^^^^^^^^^^^^^^^^^^^^
60 
- error: demangling(<c[464da6a86cb672f]::TupleByteBool<{(1u8, false)}>>)
+ error: demangling(<c[ead6075fee7cea07]::TupleByteBool<{(1u8, false)}>>)
63    |
64 LL | #[rustc_symbol_name]

70 LL | #[rustc_symbol_name]
70 LL | #[rustc_symbol_name]
71    | ^^^^^^^^^^^^^^^^^^^^
72 
- error: symbol-name(_RMs2_Csno73SFvQKx_1cINtB3_11OptionUsizeKVNtINtNtCs$HASH_4core6option6OptionjE4NoneUE)
+ error: symbol-name(_RMs2_Cska1EXlzrq0B_1cINtB3_11OptionUsizeKVNtINtNtCs$HASH_4core6option6OptionjE4NoneUE)
75    |
76 LL | #[rustc_symbol_name]

77    | ^^^^^^^^^^^^^^^^^^^^
77    | ^^^^^^^^^^^^^^^^^^^^
78 
- error: demangling(<c[464da6a86cb672f]::OptionUsize<{core[$HASH_HEX]::option::Option::<usize>::None}>>)
+ error: demangling(<c[ead6075fee7cea07]::OptionUsize<{core[$HASH_HEX]::option::Option::<usize>::None}>>)
81    |
82 LL | #[rustc_symbol_name]

88 LL | #[rustc_symbol_name]
88 LL | #[rustc_symbol_name]
89    | ^^^^^^^^^^^^^^^^^^^^
90 
- error: symbol-name(_RMs3_Csno73SFvQKx_1cINtB3_11OptionUsizeKVNtINtNtCs$HASH_4core6option6OptionjE4SomeTj0_EE)
+ error: symbol-name(_RMs3_Cska1EXlzrq0B_1cINtB3_11OptionUsizeKVNtINtNtCs$HASH_4core6option6OptionjE4SomeTj0_EE)
93    |
94 LL | #[rustc_symbol_name]

95    | ^^^^^^^^^^^^^^^^^^^^
95    | ^^^^^^^^^^^^^^^^^^^^
96 
- error: demangling(<c[464da6a86cb672f]::OptionUsize<{core[$HASH_HEX]::option::Option::<usize>::Some(0usize)}>>)
+ error: demangling(<c[ead6075fee7cea07]::OptionUsize<{core[$HASH_HEX]::option::Option::<usize>::Some(0usize)}>>)
99    |
100 LL | #[rustc_symbol_name]

106 LL | #[rustc_symbol_name]
106 LL | #[rustc_symbol_name]
107    | ^^^^^^^^^^^^^^^^^^^^
108 
- error: symbol-name(_RMs4_Csno73SFvQKx_1cINtB3_4Foo_KVNtB3_3FooS1sRe616263_2chc78_5sliceRAh1_h2_h3_EEE)
+ error: symbol-name(_RMs4_Cska1EXlzrq0B_1cINtB3_4Foo_KVNtB3_3FooS1sRe616263_2chc78_5sliceRAh1_h2_h3_EEE)
111    |
112 LL | #[rustc_symbol_name]

113    | ^^^^^^^^^^^^^^^^^^^^
113    | ^^^^^^^^^^^^^^^^^^^^
114 
- error: demangling(<c[464da6a86cb672f]::Foo_<{c[464da6a86cb672f]::Foo { s: "abc", ch: 'x', slice: &[1u8, 2u8, 3u8] }}>>)
+ error: demangling(<c[ead6075fee7cea07]::Foo_<{c[ead6075fee7cea07]::Foo { s: "abc", ch: 'x', slice: &[1u8, 2u8, 3u8] }}>>)
117    |
118 LL | #[rustc_symbol_name]

124 LL | #[rustc_symbol_name]
124 LL | #[rustc_symbol_name]
125    | ^^^^^^^^^^^^^^^^^^^^
126 
- error: symbol-name(_RMs9_Csno73SFvQKx_1cINtB3_4Bar_KVNtB3_3BarS1xh7b_s_1xt1000_EE)
+ error: symbol-name(_RMs9_Cska1EXlzrq0B_1cINtB3_4Bar_KVNtB3_3BarS1xh7b_s_1xt1000_EE)
129    |
130 LL |     #[rustc_symbol_name]

135    |
135    |
136    = note: this error originates in the macro `duplicate_field_name_test` (in Nightly builds, run with -Z macro-backtrace for more info)
137 
- error: demangling(<c[464da6a86cb672f]::Bar_<{c[464da6a86cb672f]::Bar { x: 123u8, x: 4096u16 }}>>)
+ error: demangling(<c[ead6075fee7cea07]::Bar_<{c[ead6075fee7cea07]::Bar { x: 123u8, x: 4096u16 }}>>)
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
error: symbol-name(_RMCska1EXlzrq0B_1cINtB0_7RefByteKRh7b_E)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling(<c[ead6075fee7cea07]::RefByte<{&123u8}>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<c::RefByte<{&123}>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: symbol-name(_RMs_Cska1EXlzrq0B_1cINtB2_6RefZstKRAEE)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling(<c[ead6075fee7cea07]::RefZst<{&[]}>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<c::RefZst<{&[]}>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: symbol-name(_RMs0_Cska1EXlzrq0B_1cINtB3_11Array3BytesKAh1_h2_h3_EE)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling(<c[ead6075fee7cea07]::Array3Bytes<{[1u8, 2u8, 3u8]}>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<c::Array3Bytes<{[1, 2, 3]}>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: symbol-name(_RMs1_Cska1EXlzrq0B_1cINtB3_13TupleByteBoolKTh1_b0_EE)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling(<c[ead6075fee7cea07]::TupleByteBool<{(1u8, false)}>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<c::TupleByteBool<{(1, false)}>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: symbol-name(_RMs2_Cska1EXlzrq0B_1cINtB3_11OptionUsizeKVNtINtNtCs9N55dE0HiYF_4core6option6OptionjE4NoneUE)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling(<c[ead6075fee7cea07]::OptionUsize<{core[720ca211c4a55f63]::option::Option::<usize>::None}>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<c::OptionUsize<{core::option::Option::<usize>::None}>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: symbol-name(_RMs3_Cska1EXlzrq0B_1cINtB3_11OptionUsizeKVNtINtNtCs9N55dE0HiYF_4core6option6OptionjE4SomeTj0_EE)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling(<c[ead6075fee7cea07]::OptionUsize<{core[720ca211c4a55f63]::option::Option::<usize>::Some(0usize)}>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<c::OptionUsize<{core::option::Option::<usize>::Some(0)}>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: symbol-name(_RMs4_Cska1EXlzrq0B_1cINtB3_4Foo_KVNtB3_3FooS1sRe616263_2chc78_5sliceRAh1_h2_h3_EEE)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling(<c[ead6075fee7cea07]::Foo_<{c[ead6075fee7cea07]::Foo { s: "abc", ch: 'x', slice: &[1u8, 2u8, 3u8] }}>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<c::Foo_<{c::Foo { s: "abc", ch: 'x', slice: &[1, 2, 3] }}>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: symbol-name(_RMs9_Cska1EXlzrq0B_1cINtB3_4Bar_KVNtB3_3BarS1xh7b_s_1xt1000_EE)
   |
LL |     #[rustc_symbol_name]
   |     ^^^^^^^^^^^^^^^^^^^^
...
...
LL | duplicate_field_name_test!(x);
   |
   = note: this error originates in the macro `duplicate_field_name_test` (in Nightly builds, run with -Z macro-backtrace for more info)


error: demangling(<c[ead6075fee7cea07]::Bar_<{c[ead6075fee7cea07]::Bar { x: 123u8, x: 4096u16 }}>>)
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


- error: symbol-name(_RNvMNtCs8dUWfuENynB_11issue_609253fooINtB2_3FooNtNtB4_4llvm3FooE3foo)
-   --> $DIR/issue-60925.rs:21:9
+ error: symbol-name(_RNvMNtCRATE_HASH11issue_609253fooINtB2_3FooNtNtB4_4llvm3FooE3foo)
3    |
4 LL |         #[rustc_symbol_name]
5    |         ^^^^^^^^^^^^^^^^^^^^


6 
- error: demangling(<issue_60925[HASH]::foo::Foo<issue_60925[5fcbb46c6fac4139]::llvm::Foo>>::foo)
-   --> $DIR/issue-60925.rs:21:9
+ error: demangling(<issue_60925[6c604637986db0e8]::foo::Foo<issue_60925[6c604637986db0e8]::llvm::Foo>>::foo)
9    |
10 LL |         #[rustc_symbol_name]
11    |         ^^^^^^^^^^^^^^^^^^^^


12 
13 error: demangling-alt(<issue_60925::foo::Foo<issue_60925::llvm::Foo>>::foo)
-   --> $DIR/issue-60925.rs:21:9
15    |
16 LL |         #[rustc_symbol_name]
17    |         ^^^^^^^^^^^^^^^^^^^^



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
error: symbol-name(_RNvMNtCs9iSLlAtk55I_11issue_609253fooINtB2_3FooNtNtB4_4llvm3FooE3foo)
   |
LL |         #[rustc_symbol_name]
   |         ^^^^^^^^^^^^^^^^^^^^


error: demangling(<issue_60925[6c604637986db0e8]::foo::Foo<issue_60925[6c604637986db0e8]::llvm::Foo>>::foo)
   |
LL |         #[rustc_symbol_name]
   |         ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<issue_60925::foo::Foo<issue_60925::llvm::Foo>>::foo)
   |
LL |         #[rustc_symbol_name]
   |         ^^^^^^^^^^^^^^^^^^^^

---

---- [ui] ui/symbol-names/issue-60925.rs#legacy stdout ----
diff of stderr:

- error: symbol-name(_ZN11issue_609253foo37Foo$LT$issue_60925..llv$u6d$..Foo$GT$3foo17h18eaa05e22e59176E)
-   --> $DIR/issue-60925.rs:21:9
+ error: symbol-name(_ZN11issue_609253foo37Foo$LT$issue_60925..llv$u6d$..Foo$GT$3foo17hda3e0b8dbd104ad3E)
3    |
4 LL |         #[rustc_symbol_name]
5    |         ^^^^^^^^^^^^^^^^^^^^


6 
- error: demangling(issue_60925::foo::Foo<issue_60925::llvm::Foo>::foo::h18eaa05e22e59176)
-   --> $DIR/issue-60925.rs:21:9
+ error: demangling(issue_60925::foo::Foo<issue_60925::llvm::Foo>::foo::hda3e0b8dbd104ad3)
9    |
10 LL |         #[rustc_symbol_name]
11    |         ^^^^^^^^^^^^^^^^^^^^


12 
13 error: demangling-alt(issue_60925::foo::Foo<issue_60925::llvm::Foo>::foo)
-   --> $DIR/issue-60925.rs:21:9
15    |
16 LL |         #[rustc_symbol_name]
17    |         ^^^^^^^^^^^^^^^^^^^^



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
error: symbol-name(_ZN11issue_609253foo37Foo$LT$issue_60925..llv$u6d$..Foo$GT$3foo17hda3e0b8dbd104ad3E)
   |
LL |         #[rustc_symbol_name]
   |         ^^^^^^^^^^^^^^^^^^^^


error: demangling(issue_60925::foo::Foo<issue_60925::llvm::Foo>::foo::hda3e0b8dbd104ad3)
   |
LL |         #[rustc_symbol_name]
   |         ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(issue_60925::foo::Foo<issue_60925::llvm::Foo>::foo)
   |
LL |         #[rustc_symbol_name]
   |         ^^^^^^^^^^^^^^^^^^^^

---

---- [ui] ui/symbol-names/issue-75326.rs#legacy stdout ----
diff of stderr:

1 error: symbol-name(_ZN72_$LT$issue_75326..Foo$LT$I$C$E$GT$$u20$as$u20$issue_75326..Iterator2$GT$4next17SYMBOL_HASH)
-   --> $DIR/issue-75326.rs:41:5
3    |
4 LL |     #[rustc_symbol_name]
5    |     ^^^^^^^^^^^^^^^^^^^^


6 
7 error: demangling(<issue_75326::Foo<I,E> as issue_75326::Iterator2>::next::SYMBOL_HASH)
-   --> $DIR/issue-75326.rs:41:5
9    |
10 LL |     #[rustc_symbol_name]
11    |     ^^^^^^^^^^^^^^^^^^^^


12 
13 error: demangling-alt(<issue_75326::Foo<I,E> as issue_75326::Iterator2>::next)
-   --> $DIR/issue-75326.rs:41:5
15    |
16 LL |     #[rustc_symbol_name]
17    |     ^^^^^^^^^^^^^^^^^^^^



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/issue-75326.legacy/issue-75326.legacy.stderr
To only update this specific test, also pass `--test-args symbol-names/issue-75326.rs`


error in revision `legacy`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/symbol-names/issue-75326.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "legacy" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/issue-75326.legacy" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "symbol-mangling-version=legacy" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/issue-75326.legacy/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: symbol-name(_ZN72_$LT$issue_75326..Foo$LT$I$C$E$GT$$u20$as$u20$issue_75326..Iterator2$GT$4next17hab19337632d4eeb4E)
   |
LL |     #[rustc_symbol_name]
   |     ^^^^^^^^^^^^^^^^^^^^


error: demangling(<issue_75326::Foo<I,E> as issue_75326::Iterator2>::next::hab19337632d4eeb4)
   |
LL |     #[rustc_symbol_name]
   |     ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<issue_75326::Foo<I,E> as issue_75326::Iterator2>::next)
   |
LL |     #[rustc_symbol_name]
   |     ^^^^^^^^^^^^^^^^^^^^


error: aborting due to 3 previous errors


------------------------------------------


---- [ui] ui/symbol-names/issue-75326.rs#v0 stdout ----


- error: symbol-name(_RNvXINICRATE_HASH_11issue_75326s_0pppEINtB5_3FooppENtB5_9Iterator24nextB5_)
-   --> $DIR/issue-75326.rs:41:5
+ error: symbol-name(_RNvXINICRATE_HASH11issue_75326s_0pppEINtB5_3FooppENtB5_9Iterator24nextB5_)
3    |
4 LL |     #[rustc_symbol_name]
5    |     ^^^^^^^^^^^^^^^^^^^^


6 
- error: demangling(<issue_75326[HASH]::Foo<_, _> as issue_75326[HASH]::Iterator2>::next)
-   --> $DIR/issue-75326.rs:41:5
+ error: demangling(<issue_75326[1d89cc332eb8b238]::Foo<_, _> as issue_75326[1d89cc332eb8b238]::Iterator2>::next)
9    |
10 LL |     #[rustc_symbol_name]
11    |     ^^^^^^^^^^^^^^^^^^^^


12 
13 error: demangling-alt(<issue_75326::Foo<_, _> as issue_75326::Iterator2>::next)
-   --> $DIR/issue-75326.rs:41:5
15    |
16 LL |     #[rustc_symbol_name]
17    |     ^^^^^^^^^^^^^^^^^^^^



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
error: symbol-name(_RNvXINICs2xelZTPkNVQ_11issue_75326s_0pppEINtB5_3FooppENtB5_9Iterator24nextB5_)
   |
LL |     #[rustc_symbol_name]
   |     ^^^^^^^^^^^^^^^^^^^^


error: demangling(<issue_75326[1d89cc332eb8b238]::Foo<_, _> as issue_75326[1d89cc332eb8b238]::Iterator2>::next)
   |
LL |     #[rustc_symbol_name]
   |     ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<issue_75326::Foo<_, _> as issue_75326::Iterator2>::next)
   |
LL |     #[rustc_symbol_name]
   |     ^^^^^^^^^^^^^^^^^^^^


error: aborting due to 3 previous errors


------------------------------------------


---- [ui] ui/symbol-names/trait-objects.rs#v0 stdout ----

4 LL |     #[rustc_symbol_name]
5    |     ^^^^^^^^^^^^^^^^^^^^
6 
6 
- error: demangling(<&dyn for<'a> core[HASH]::ops::function::FnMut<(&'a u8,), Output = ()> as trait_objects[3f8b57f879016e18]::Bar>::method)
+ error: demangling(<&dyn for<'a> core[HASH]::ops::function::FnMut<(&'a u8,), Output = ()> as trait_objects[44257da5bacecdd0]::Bar>::method)
9    |
10 LL |     #[rustc_symbol_name]

22 LL |     #[rustc_symbol_name]
22 LL |     #[rustc_symbol_name]
23    |     ^^^^^^^^^^^^^^^^^^^^
24 
- error: demangling(<&dyn for<'a> core[HASH]::ops::function::FnMut<(&'a u8,), Output = ()> + core[HASH]::marker::Send as trait_objects[3f8b57f879016e18]::Foo>::method)
+ error: demangling(<&dyn for<'a> core[HASH]::ops::function::FnMut<(&'a u8,), Output = ()> + core[HASH]::marker::Send as trait_objects[44257da5bacecdd0]::Foo>::method)
27    |
28 LL |     #[rustc_symbol_name]

40 LL |     #[rustc_symbol_name]
40 LL |     #[rustc_symbol_name]
41    |     ^^^^^^^^^^^^^^^^^^^^
42 
- error: demangling(<&dyn for<'a> core[HASH]::ops::function::FnMut<(&'a u8,), Output = ()> + core[HASH]::marker::Send as trait_objects[3f8b57f879016e18]::Baz>::method)
+ error: demangling(<&dyn for<'a> core[HASH]::ops::function::FnMut<(&'a u8,), Output = ()> + core[HASH]::marker::Send as trait_objects[44257da5bacecdd0]::Baz>::method)
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
error: symbol-name(_RNvXCs5QK02y8cUay_13trait_objectsRDG_INtNtNtCs9N55dE0HiYF_4core3ops8function5FnMutTRL0_hEEp6OutputuEL_NtB2_3Bar6method)
   |
LL |     #[rustc_symbol_name]
   |     ^^^^^^^^^^^^^^^^^^^^


error: demangling(<&dyn for<'a> core[720ca211c4a55f63]::ops::function::FnMut<(&'a u8,), Output = ()> as trait_objects[44257da5bacecdd0]::Bar>::method)
   |
LL |     #[rustc_symbol_name]
   |     ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<&dyn for<'a> core::ops::function::FnMut<(&'a u8,), Output = ()> as trait_objects::Bar>::method)
   |
LL |     #[rustc_symbol_name]
   |     ^^^^^^^^^^^^^^^^^^^^


error: symbol-name(_RNvXs_Cs5QK02y8cUay_13trait_objectsRDG_INtNtNtCs9N55dE0HiYF_4core3ops8function5FnMutTRL0_hEEp6OutputuNtNtBI_6marker4SendEL_NtB4_3Foo6method)
   |
LL |     #[rustc_symbol_name]
   |     ^^^^^^^^^^^^^^^^^^^^


error: demangling(<&dyn for<'a> core[720ca211c4a55f63]::ops::function::FnMut<(&'a u8,), Output = ()> + core[720ca211c4a55f63]::marker::Send as trait_objects[44257da5bacecdd0]::Foo>::method)
   |
LL |     #[rustc_symbol_name]
   |     ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<&dyn for<'a> core::ops::function::FnMut<(&'a u8,), Output = ()> + core::marker::Send as trait_objects::Foo>::method)
   |
LL |     #[rustc_symbol_name]
   |     ^^^^^^^^^^^^^^^^^^^^


error: symbol-name(_RNvXs0_Cs5QK02y8cUay_13trait_objectsRDG_INtNtNtCs9N55dE0HiYF_4core3ops8function5FnMutTRL0_hEEp6OutputuNtNtBJ_6marker4SendEL_NtB5_3Baz6method)
   |
LL |     #[rustc_symbol_name]
   |     ^^^^^^^^^^^^^^^^^^^^


error: demangling(<&dyn for<'a> core[720ca211c4a55f63]::ops::function::FnMut<(&'a u8,), Output = ()> + core[720ca211c4a55f63]::marker::Send as trait_objects[44257da5bacecdd0]::Baz>::method)
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

- DefId(0:3 ~ thir_tree[348d]::main):
+ DefId(0:3 ~ thir_tree[5d10]::main):
2 Thir {
3     arms: [],
4     exprs: [
30                 region_scope: Node(2),
31                 lint_level: Explicit(
32                     HirId {
32                     HirId {
-                         owner: DefId(0:3 ~ thir_tree[348d]::main),
+                         owner: DefId(0:3 ~ thir_tree[5d10]::main),
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
DefId(0:3 ~ thir_tree[5d10]::main):
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
                        owner: DefId(0:3 ~ thir_tree[5d10]::main),
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
test result: FAILED. 12162 passed; 17 failed; 117 ignored; 0 measured; 0 filtered out; finished in 139.57s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:14:17
