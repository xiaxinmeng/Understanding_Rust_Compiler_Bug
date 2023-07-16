plain
.................................................................................................... 10300/11934
.................................................................................................... 10400/11934
.................................................................................................... 10500/11934
.................................................................................................... 10600/11934
...................FF...........F........F..FFiFF....................................i.............. 10700/11934
.................................................................................................... 10900/11934
.................................................................................................... 11000/11934
.................................................................................................... 11100/11934
.................................................................................................... 11200/11934
---
diff of stderr:

2   --> $DIR/tls.rs:12:25
3    |
4 LL |     unsafe { let _val = A; }
-    |                         ^ cannot access thread local static (DefId(0:6 ~ tls[317d]::A))
+    |                         ^ cannot access thread local static (DefId(0:6 ~ tls[f423]::A))
7 error[E0080]: could not evaluate static initializer
8   --> $DIR/tls.rs:19:26

9    |
9    |
10 LL |     unsafe { let _val = &A; }
-    |                          ^ cannot access thread local static (DefId(0:6 ~ tls[317d]::A))
+    |                          ^ cannot access thread local static (DefId(0:6 ~ tls[f423]::A))
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
13 warning: skipping const checks
14    |

---
To only update this specific test, also pass `--test-args consts/miri_unleashed/tls.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/miri_unleashed/tls.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/tls" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zunleash-the-miri-inside-of-you" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/tls/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0080]: could not evaluate static initializer
  --> /checkout/src/test/ui/consts/miri_unleashed/tls.rs:12:25
   |
LL |     unsafe { let _val = A; }
   |                         ^ cannot access thread local static (DefId(0:6 ~ tls[f423]::A))
error[E0080]: could not evaluate static initializer
  --> /checkout/src/test/ui/consts/miri_unleashed/tls.rs:19:26
   |
   |
LL |     unsafe { let _val = &A; }
   |                          ^ cannot access thread local static (DefId(0:6 ~ tls[f423]::A))
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

12   --> $DIR/generator-print-verbose-1.rs:35:9
13    |
14 LL |         let _non_send_gen = make_non_send_generator();
-    |             ------------- has type `Opaque(DefId(0:34 ~ generator_print_verbose_1[317d]::make_non_send_generator::{opaque#0}), [])` which is not `Send`
+    |             ------------- has type `Opaque(DefId(0:34 ~ generator_print_verbose_1[70c9]::make_non_send_generator::{opaque#0}), [])` which is not `Send`
16 LL |         yield;
17    |         ^^^^^ yield occurs here, with `_non_send_gen` maybe used later


30    = help: the trait `Sync` is not implemented for `RefCell<i32>`
31    = note: required because of the requirements on the impl of `Send` for `Arc<RefCell<i32>>`
32    = note: required because it appears within the type `[make_gen2<Arc<RefCell<i32>>>::{closure#0} upvar_tys=(Arc<RefCell<i32>>) {()}]`
-    = note: required because it appears within the type `Opaque(DefId(0:39 ~ generator_print_verbose_1[317d]::make_gen2::{opaque#0}), [std::sync::Arc<std::cell::RefCell<i32>>])`
-    = note: required because it appears within the type `Opaque(DefId(0:42 ~ generator_print_verbose_1[317d]::make_non_send_generator2::{opaque#0}), [])`
-    = note: required because it appears within the type `{Opaque(DefId(0:42 ~ generator_print_verbose_1[317d]::make_non_send_generator2::{opaque#0}), []), ()}`
-    = note: required because it appears within the type `[test2::{closure#0} upvar_tys=() {Opaque(DefId(0:42 ~ generator_print_verbose_1[317d]::make_non_send_generator2::{opaque#0}), []), ()}]`
+    = note: required because it appears within the type `Opaque(DefId(0:39 ~ generator_print_verbose_1[70c9]::make_gen2::{opaque#0}), [std::sync::Arc<std::cell::RefCell<i32>>])`
+    = note: required because it appears within the type `Opaque(DefId(0:42 ~ generator_print_verbose_1[70c9]::make_non_send_generator2::{opaque#0}), [])`
+    = note: required because it appears within the type `{Opaque(DefId(0:42 ~ generator_print_verbose_1[70c9]::make_non_send_generator2::{opaque#0}), []), ()}`
+    = note: required because it appears within the type `[test2::{closure#0} upvar_tys=() {Opaque(DefId(0:42 ~ generator_print_verbose_1[70c9]::make_non_send_generator2::{opaque#0}), []), ()}]`
38 error: aborting due to 2 previous errors
39 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/print/generator-print-verbose-1/generator-print-verbose-1.stderr
To only update this specific test, also pass `--test-args generator/print/generator-print-verbose-1.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generator/print/generator-print-verbose-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/print/generator-print-verbose-1" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zverbose" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/print/generator-print-verbose-1/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: generator cannot be sent between threads safely
   |
   |
LL | fn require_send(_: impl Send) {}
   |                         ---- required by this bound in `require_send`
LL |     require_send(send_gen);
LL |     require_send(send_gen);
   |     ^^^^^^^^^^^^ generator is not `Send`
   |
   = help: the trait `Sync` is not implemented for `RefCell<i32>`
note: generator is not `Send` as this value is used across a yield
   |
   |
LL |         let _non_send_gen = make_non_send_generator();
   |             ------------- has type `Opaque(DefId(0:34 ~ generator_print_verbose_1[70c9]::make_non_send_generator::{opaque#0}), [])` which is not `Send`
LL |         yield;
   |         ^^^^^ yield occurs here, with `_non_send_gen` maybe used later
LL |     };
   |     - `_non_send_gen` is later dropped here

error[E0277]: `RefCell<i32>` cannot be shared between threads safely
   |
   |
LL | fn require_send(_: impl Send) {}
   |                         ---- required by this bound in `require_send`
LL |     require_send(send_gen);
LL |     require_send(send_gen);
   |     ^^^^^^^^^^^^ `RefCell<i32>` cannot be shared between threads safely
   |
   = help: the trait `Sync` is not implemented for `RefCell<i32>`
   = note: required because of the requirements on the impl of `Send` for `Arc<RefCell<i32>>`
   = note: required because it appears within the type `[make_gen2<Arc<RefCell<i32>>>::{closure#0} upvar_tys=(Arc<RefCell<i32>>) {()}]`
   = note: required because it appears within the type `Opaque(DefId(0:39 ~ generator_print_verbose_1[70c9]::make_gen2::{opaque#0}), [std::sync::Arc<std::cell::RefCell<i32>>])`
   = note: required because it appears within the type `Opaque(DefId(0:42 ~ generator_print_verbose_1[70c9]::make_non_send_generator2::{opaque#0}), [])`
   = note: required because it appears within the type `{Opaque(DefId(0:42 ~ generator_print_verbose_1[70c9]::make_non_send_generator2::{opaque#0}), []), ()}`
   = note: required because it appears within the type `[test2::{closure#0} upvar_tys=() {Opaque(DefId(0:42 ~ generator_print_verbose_1[70c9]::make_non_send_generator2::{opaque#0}), []), ()}]`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0277`.


------------------------------------------


---- [ui] ui/lto-duplicate-symbols.rs stdout ----
diff of stderr:

1 warning: Linking globals named 'foo': symbol multiply defined!
2 
- error: failed to load bc of "lto-duplicate-symbols2.lto_duplicate_symbols2.3a1fbbbh-cgu.0.rcgu.o": 
+ error: failed to load bc of "lto-duplicate-symbols2.lto_duplicate_symbols2.288b404e693a75b4-cgu.0.rcgu.o": 
5 error: aborting due to previous error; 1 warning emitted
6 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lto-duplicate-symbols/lto-duplicate-symbols.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lto-duplicate-symbols.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lto-duplicate-symbols.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lto-duplicate-symbols" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "lto" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lto-duplicate-symbols/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: Linking globals named 'foo': symbol multiply defined!

error: failed to load bc of "lto-duplicate-symbols2.lto_duplicate_symbols2.288b404e693a75b4-cgu.0.rcgu.o": 
error: aborting due to previous error; 1 warning emitted


------------------------------------------
------------------------------------------


---- [ui] ui/single-use-lifetime/one-use-in-fn-argument-in-band.rs stdout ----
diff of stderr:

- error: lifetime parameter `'b` only used once
-   --> $DIR/one-use-in-fn-argument-in-band.rs:11:22
+ error: lifetime parameter `'a` only used once
+   --> $DIR/one-use-in-fn-argument-in-band.rs:11:10
3    |
4 LL | fn a(x: &'a u32, y: &'b u32) {
-    |                      ^^-
-    |                      this lifetime is only used here
-    |                      this lifetime is only used here
-    |                      help: elide the single-use lifetime
+    |          ^^-
+    |          this lifetime is only used here
+    |          this lifetime is only used here
+    |          help: elide the single-use lifetime
10 note: the lint level is defined here
11   --> $DIR/one-use-in-fn-argument-in-band.rs:4:9


13 LL | #![deny(single_use_lifetimes)]
15 
15 
- error: lifetime parameter `'a` only used once
-   --> $DIR/one-use-in-fn-argument-in-band.rs:11:10
+ error: lifetime parameter `'b` only used once
+   --> $DIR/one-use-in-fn-argument-in-band.rs:11:22
18    |
19 LL | fn a(x: &'a u32, y: &'b u32) {
-    |          ^^-
-    |          this lifetime is only used here
-    |          this lifetime is only used here
-    |          help: elide the single-use lifetime
+    |                      ^^-
+    |                      this lifetime is only used here
+    |                      this lifetime is only used here
+    |                      help: elide the single-use lifetime
25 error: aborting due to 2 previous errors
26 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/single-use-lifetime/one-use-in-fn-argument-in-band/one-use-in-fn-argument-in-band.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args single-use-lifetime/one-use-in-fn-argument-in-band.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/single-use-lifetime/one-use-in-fn-argument-in-band.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/single-use-lifetime/one-use-in-fn-argument-in-band" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/single-use-lifetime/one-use-in-fn-argument-in-band/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: lifetime parameter `'a` only used once
   |
   |
LL | fn a(x: &'a u32, y: &'b u32) {
   |          ^^-
   |          this lifetime is only used here
   |          this lifetime is only used here
   |          help: elide the single-use lifetime
note: the lint level is defined here
  --> /checkout/src/test/ui/single-use-lifetime/one-use-in-fn-argument-in-band.rs:4:9
   |
   |
LL | #![deny(single_use_lifetimes)]


error: lifetime parameter `'b` only used once
   |
   |
LL | fn a(x: &'a u32, y: &'b u32) {
   |                      ^^-
   |                      this lifetime is only used here
   |                      this lifetime is only used here
   |                      help: elide the single-use lifetime
error: aborting due to 2 previous errors


------------------------------------------
------------------------------------------


---- [ui] ui/specialization/min_specialization/repeated_projection_type.rs stdout ----
diff of stderr:

- error: cannot specialize on `Binder(ProjectionPredicate(ProjectionTy { substs: [V], item_def_id: DefId(0:6 ~ repeated_projection_type[317d]::Id::This) }, (I,)), [])`
+ error: cannot specialize on `Binder(ProjectionPredicate(ProjectionTy { substs: [V], item_def_id: DefId(0:6 ~ repeated_projection_type[b09c]::Id::This) }, (I,)), [])`
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
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/specialization/min_specialization/repeated_projection_type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/min_specialization/repeated_projection_type" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/min_specialization/repeated_projection_type/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: cannot specialize on `Binder(ProjectionPredicate(ProjectionTy { substs: [V], item_def_id: DefId(0:6 ~ repeated_projection_type[b09c]::Id::This) }, (I,)), [])`
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

- error: symbol-name(_ZN5basic4main17h6c535bbea2051f85E)
+ error: symbol-name(_ZN5basic4main17hd75b915511563828E)
3    |
4 LL | #[rustc_symbol_name]

5    | ^^^^^^^^^^^^^^^^^^^^
5    | ^^^^^^^^^^^^^^^^^^^^
6 
- error: demangling(basic::main::h6c535bbea2051f85)
+ error: demangling(basic::main::hd75b915511563828)
9    |
10 LL | #[rustc_symbol_name]



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/basic.legacy/basic.legacy.stderr
To only update this specific test, also pass `--test-args symbol-names/basic.rs`


error in revision `legacy`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/symbol-names/basic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "legacy" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/basic.legacy" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "symbol-mangling-version=legacy" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/basic.legacy/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: symbol-name(_ZN5basic4main17hd75b915511563828E)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling(basic::main::hd75b915511563828)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(basic::main)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: def-path(main)
  --> /checkout/src/test/ui/symbol-names/basic.rs:15:1
   |
LL | #[rustc_def_path]

error: aborting due to 4 previous errors



------------------------------------------


---- [ui] ui/symbol-names/basic.rs#v0 stdout ----


- error: symbol-name(_RNvCs21hi0yVfW1J_5basic4main)
+ error: symbol-name(_RNvCsj6j3mjPNGKx_5basic4main)
3    |
4 LL | #[rustc_symbol_name]

5    | ^^^^^^^^^^^^^^^^^^^^
5    | ^^^^^^^^^^^^^^^^^^^^
6 
- error: demangling(basic[17891616a171812d]::main)
+ error: demangling(basic[de7d5b6b69c71f37]::main)
9    |
10 LL | #[rustc_symbol_name]



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/basic.v0/basic.v0.stderr
To only update this specific test, also pass `--test-args symbol-names/basic.rs`


error in revision `v0`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/symbol-names/basic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "v0" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/basic.v0" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "symbol-mangling-version=v0" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/basic.v0/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: symbol-name(_RNvCsj6j3mjPNGKx_5basic4main)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling(basic[de7d5b6b69c71f37]::main)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(basic::main)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: def-path(main)
  --> /checkout/src/test/ui/symbol-names/basic.rs:15:1
   |
LL | #[rustc_def_path]

error: aborting due to 4 previous errors



------------------------------------------


---- [ui] ui/symbol-names/const-generics-demangling.rs stdout ----
diff of stderr:

- error: symbol-name(_RMCs21hi0yVfW1J_25const_generics_demanglingINtB0_8UnsignedKhb_E)
+ error: symbol-name(_RMCsaP8qXevlYG3_25const_generics_demanglingINtB0_8UnsignedKhb_E)
3    |
4 LL | #[rustc_symbol_name]

5    | ^^^^^^^^^^^^^^^^^^^^
5    | ^^^^^^^^^^^^^^^^^^^^
6 
- error: demangling(<const_generics_demangling[17891616a171812d]::Unsigned<11: u8>>)
+ error: demangling(<const_generics_demangling[7e153590edc26969]::Unsigned<11: u8>>)
9    |
10 LL | #[rustc_symbol_name]

16 LL | #[rustc_symbol_name]
16 LL | #[rustc_symbol_name]
17    | ^^^^^^^^^^^^^^^^^^^^
18 
- error: symbol-name(_RMs_Cs21hi0yVfW1J_25const_generics_demanglingINtB2_6SignedKsn98_E)
+ error: symbol-name(_RMs_CsaP8qXevlYG3_25const_generics_demanglingINtB2_6SignedKsn98_E)
21    |
22 LL | #[rustc_symbol_name]

23    | ^^^^^^^^^^^^^^^^^^^^
23    | ^^^^^^^^^^^^^^^^^^^^
24 
- error: demangling(<const_generics_demangling[17891616a171812d]::Signed<-152: i16>>)
+ error: demangling(<const_generics_demangling[7e153590edc26969]::Signed<-152: i16>>)
27    |
28 LL | #[rustc_symbol_name]

34 LL | #[rustc_symbol_name]
34 LL | #[rustc_symbol_name]
35    | ^^^^^^^^^^^^^^^^^^^^
36 
- error: symbol-name(_RMs0_Cs21hi0yVfW1J_25const_generics_demanglingINtB3_4BoolKb1_E)
+ error: symbol-name(_RMs0_CsaP8qXevlYG3_25const_generics_demanglingINtB3_4BoolKb1_E)
39    |
40 LL | #[rustc_symbol_name]

41    | ^^^^^^^^^^^^^^^^^^^^
41    | ^^^^^^^^^^^^^^^^^^^^
42 
- error: demangling(<const_generics_demangling[17891616a171812d]::Bool<true: bool>>)
+ error: demangling(<const_generics_demangling[7e153590edc26969]::Bool<true: bool>>)
45    |
46 LL | #[rustc_symbol_name]

52 LL | #[rustc_symbol_name]
52 LL | #[rustc_symbol_name]
53    | ^^^^^^^^^^^^^^^^^^^^
54 
- error: symbol-name(_RMs1_Cs21hi0yVfW1J_25const_generics_demanglingINtB3_4CharKc2202_E)
+ error: symbol-name(_RMs1_CsaP8qXevlYG3_25const_generics_demanglingINtB3_4CharKc2202_E)
57    |
58 LL | #[rustc_symbol_name]

59    | ^^^^^^^^^^^^^^^^^^^^
59    | ^^^^^^^^^^^^^^^^^^^^
60 
- error: demangling(<const_generics_demangling[17891616a171812d]::Char<'∂': char>>)
+ error: demangling(<const_generics_demangling[7e153590edc26969]::Char<'∂': char>>)
63    |
64 LL | #[rustc_symbol_name]



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/const-generics-demangling/const-generics-demangling.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args symbol-names/const-generics-demangling.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/symbol-names/const-generics-demangling.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/const-generics-demangling" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "symbol-mangling-version=v0" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/const-generics-demangling/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: symbol-name(_RMCsaP8qXevlYG3_25const_generics_demanglingINtB0_8UnsignedKhb_E)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling(<const_generics_demangling[7e153590edc26969]::Unsigned<11: u8>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<const_generics_demangling::Unsigned<11>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: symbol-name(_RMs_CsaP8qXevlYG3_25const_generics_demanglingINtB2_6SignedKsn98_E)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling(<const_generics_demangling[7e153590edc26969]::Signed<-152: i16>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<const_generics_demangling::Signed<-152>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: symbol-name(_RMs0_CsaP8qXevlYG3_25const_generics_demanglingINtB3_4BoolKb1_E)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling(<const_generics_demangling[7e153590edc26969]::Bool<true: bool>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^
---

---- [ui] ui/symbol-names/issue-60925.rs#legacy stdout ----
diff of stderr:

- error: symbol-name(_ZN11issue_609253foo37Foo$LT$issue_60925..llv$u6d$..Foo$GT$3foo17h6244e5288326926aE)
+ error: symbol-name(_ZN11issue_609253foo37Foo$LT$issue_60925..llv$u6d$..Foo$GT$3foo17h18eaa05e22e59176E)
3    |
4 LL |         #[rustc_symbol_name]

5    |         ^^^^^^^^^^^^^^^^^^^^
5    |         ^^^^^^^^^^^^^^^^^^^^
6 
- error: demangling(issue_60925::foo::Foo<issue_60925::llvm::Foo>::foo::h6244e5288326926a)
+ error: demangling(issue_60925::foo::Foo<issue_60925::llvm::Foo>::foo::h18eaa05e22e59176)
9    |
10 LL |         #[rustc_symbol_name]



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/issue-60925.legacy/issue-60925.legacy.stderr
To only update this specific test, also pass `--test-args symbol-names/issue-60925.rs`


error in revision `legacy`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/symbol-names/issue-60925.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "legacy" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/issue-60925.legacy" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "symbol-mangling-version=legacy" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/issue-60925.legacy/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: symbol-name(_ZN11issue_609253foo37Foo$LT$issue_60925..llv$u6d$..Foo$GT$3foo17h18eaa05e22e59176E)
   |
LL |         #[rustc_symbol_name]
   |         ^^^^^^^^^^^^^^^^^^^^


error: demangling(issue_60925::foo::Foo<issue_60925::llvm::Foo>::foo::h18eaa05e22e59176)
   |
LL |         #[rustc_symbol_name]
   |         ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(issue_60925::foo::Foo<issue_60925::llvm::Foo>::foo)
   |
LL |         #[rustc_symbol_name]
   |         ^^^^^^^^^^^^^^^^^^^^


error: aborting due to 3 previous errors


------------------------------------------


---- [ui] ui/symbol-names/impl1.rs#v0 stdout ----


- error: symbol-name(_RNvMNtCs21hi0yVfW1J_5impl13fooNtB2_3Foo3bar)
+ error: symbol-name(_RNvMNtCs2qSCrjELJET_5impl13fooNtB2_3Foo3bar)
3    |
4 LL |         #[rustc_symbol_name]

5    |         ^^^^^^^^^^^^^^^^^^^^
5    |         ^^^^^^^^^^^^^^^^^^^^
6 
- error: demangling(<impl1[17891616a171812d]::foo::Foo>::bar)
+ error: demangling(<impl1[1c5860ab79c9e305]::foo::Foo>::bar)
9    |
10 LL |         #[rustc_symbol_name]

22 LL |         #[rustc_def_path]
22 LL |         #[rustc_def_path]
23    |         ^^^^^^^^^^^^^^^^^
24 
- error: symbol-name(_RNvMNtCs21hi0yVfW1J_5impl13barNtNtB4_3foo3Foo3baz)
+ error: symbol-name(_RNvMNtCs2qSCrjELJET_5impl13barNtNtB4_3foo3Foo3baz)
27    |
28 LL |         #[rustc_symbol_name]

29    |         ^^^^^^^^^^^^^^^^^^^^
29    |         ^^^^^^^^^^^^^^^^^^^^
30 
- error: demangling(<impl1[17891616a171812d]::foo::Foo>::baz)
+ error: demangling(<impl1[1c5860ab79c9e305]::foo::Foo>::baz)
33    |
34 LL |         #[rustc_symbol_name]

46 LL |         #[rustc_def_path]
46 LL |         #[rustc_def_path]
47    |         ^^^^^^^^^^^^^^^^^
48 
- error: symbol-name(_RNvXNCNvCs21hi0yVfW1J_5impl14mains_0ARDNtB6_3Foop5AssocFG_KCRL0_hvEuNtB6_9AutoTraitEL_j3_NtB2_3Bar6method)
+ error: symbol-name(_RNvXNCNvCs2qSCrjELJET_5impl14mains_0ARDNtB6_3Foop5AssocFG_KCRL0_hvEuNtB6_9AutoTraitEL_j3_NtB2_3Bar6method)
51    |
52 LL |             #[rustc_symbol_name]

53    |             ^^^^^^^^^^^^^^^^^^^^
53    |             ^^^^^^^^^^^^^^^^^^^^
54 
- error: demangling(<[&dyn impl1[17891616a171812d]::Foo<Assoc = for<'a> extern "C" fn(&'a u8, ...)> + impl1[17891616a171812d]::AutoTrait; 3: usize] as impl1[17891616a171812d]::main::{closure#1}::Bar>::method)
+ error: demangling(<[&dyn impl1[1c5860ab79c9e305]::Foo<Assoc = for<'a> extern "C" fn(&'a u8, ...)> + impl1[1c5860ab79c9e305]::AutoTrait; 3: usize] as impl1[1c5860ab79c9e305]::main::{closure#1}::Bar>::method)
57    |
58 LL |             #[rustc_symbol_name]



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/impl1.v0/impl1.v0.stderr
To only update this specific test, also pass `--test-args symbol-names/impl1.rs`


error in revision `v0`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/symbol-names/impl1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "v0" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/impl1.v0" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "symbol-mangling-version=v0" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/impl1.v0/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: symbol-name(_RNvMNtCs2qSCrjELJET_5impl13fooNtB2_3Foo3bar)
   |
LL |         #[rustc_symbol_name]
   |         ^^^^^^^^^^^^^^^^^^^^


error: demangling(<impl1[1c5860ab79c9e305]::foo::Foo>::bar)
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


error: symbol-name(_RNvMNtCs2qSCrjELJET_5impl13barNtNtB4_3foo3Foo3baz)
   |
LL |         #[rustc_symbol_name]
   |         ^^^^^^^^^^^^^^^^^^^^


error: demangling(<impl1[1c5860ab79c9e305]::foo::Foo>::baz)
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


error: symbol-name(_RNvXNCNvCs2qSCrjELJET_5impl14mains_0ARDNtB6_3Foop5AssocFG_KCRL0_hvEuNtB6_9AutoTraitEL_j3_NtB2_3Bar6method)
   |
LL |             #[rustc_symbol_name]
   |             ^^^^^^^^^^^^^^^^^^^^


error: demangling(<[&dyn impl1[1c5860ab79c9e305]::Foo<Assoc = for<'a> extern "C" fn(&'a u8, ...)> + impl1[1c5860ab79c9e305]::AutoTrait; 3: usize] as impl1[1c5860ab79c9e305]::main::{closure#1}::Bar>::method)
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


error: aborting due to 12 previous errors


------------------------------------------


---- [ui] ui/symbol-names/issue-60925.rs#v0 stdout ----


- error: symbol-name(_RNvMNtCs21hi0yVfW1J_11issue_609253fooINtB2_3FooNtNtB4_4llvm3FooE3foo)
+ error: symbol-name(_RNvMNtCs8dUWfuENynB_11issue_609253fooINtB2_3FooNtNtB4_4llvm3FooE3foo)
3    |
4 LL |         #[rustc_symbol_name]

5    |         ^^^^^^^^^^^^^^^^^^^^
5    |         ^^^^^^^^^^^^^^^^^^^^
6 
- error: demangling(<issue_60925[17891616a171812d]::foo::Foo<issue_60925[17891616a171812d]::llvm::Foo>>::foo)
+ error: demangling(<issue_60925[5fcbb46c6fac4139]::foo::Foo<issue_60925[5fcbb46c6fac4139]::llvm::Foo>>::foo)
9    |
10 LL |         #[rustc_symbol_name]



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/issue-60925.v0/issue-60925.v0.stderr
To only update this specific test, also pass `--test-args symbol-names/issue-60925.rs`


error in revision `v0`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/symbol-names/issue-60925.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "v0" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/issue-60925.v0" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "symbol-mangling-version=v0" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/symbol-names/issue-60925.v0/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: symbol-name(_RNvMNtCs8dUWfuENynB_11issue_609253fooINtB2_3FooNtNtB4_4llvm3FooE3foo)
   |
LL |         #[rustc_symbol_name]
   |         ^^^^^^^^^^^^^^^^^^^^


error: demangling(<issue_60925[5fcbb46c6fac4139]::foo::Foo<issue_60925[5fcbb46c6fac4139]::llvm::Foo>>::foo)
   |
LL |         #[rustc_symbol_name]
   |         ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<issue_60925::foo::Foo<issue_60925::llvm::Foo>>::foo)
   |
LL |         #[rustc_symbol_name]
   |         ^^^^^^^^^^^^^^^^^^^^

---
test result: FAILED. 11824 passed; 13 failed; 97 ignored; 0 measured; 0 filtered out; finished in 127.24s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:13:32
