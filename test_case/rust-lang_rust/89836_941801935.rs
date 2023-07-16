plain
.................................................................................................... 10600/12278
.................................................................................................... 10700/12278
.................................................................................................... 10800/12278
.................................................................................................... 10900/12278
..............F.F......FF....F..FF..F.Fii....F..........................i....................F...... 11000/12278
.................................................................................................... 11200/12278
.................................................................................................... 11300/12278
.................................................................................................... 11400/12278
.................................................................................................... 11500/12278
---
diff of stderr:

2   --> $DIR/tls.rs:12:25
3    |
4 LL |     unsafe { let _val = A; }
-    |                         ^ cannot access thread local static (DefId(0:6 ~ tls[f423]::A))
+    |                         ^ cannot access thread local static (DefId(0:6 ~ tls[40b7]::A))
7 error[E0080]: could not evaluate static initializer
8   --> $DIR/tls.rs:19:26

9    |
9    |
10 LL |     unsafe { let _val = &A; }
-    |                          ^ cannot access thread local static (DefId(0:6 ~ tls[f423]::A))
+    |                          ^ cannot access thread local static (DefId(0:6 ~ tls[40b7]::A))
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
   |                         ^ cannot access thread local static (DefId(0:6 ~ tls[40b7]::A))
error[E0080]: could not evaluate static initializer
  --> /checkout/src/test/ui/consts/miri_unleashed/tls.rs:19:26
   |
   |
LL |     unsafe { let _val = &A; }
   |                          ^ cannot access thread local static (DefId(0:6 ~ tls[40b7]::A))
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
+    |             ------------- has type `Opaque(DefId(0:34 ~ generator_print_verbose_1[e574]::make_non_send_generator::{opaque#0}), [])` which is not `Send`
13 LL |         yield;
14    |         ^^^^^ yield occurs here, with `_non_send_gen` maybe used later


29    = help: the trait `Sync` is not implemented for `RefCell<i32>`
30    = note: required because of the requirements on the impl of `Send` for `Arc<RefCell<i32>>`
31    = note: required because it appears within the type `[make_gen2<Arc<RefCell<i32>>>::{closure#0} upvar_tys=(Arc<RefCell<i32>>) {()}]`
-    = note: required because it appears within the type `Opaque(DefId(0:39 ~ generator_print_verbose_1[70c9]::make_gen2::{opaque#0}), [std::sync::Arc<std::cell::RefCell<i32>>])`
-    = note: required because it appears within the type `Opaque(DefId(0:42 ~ generator_print_verbose_1[70c9]::make_non_send_generator2::{opaque#0}), [])`
-    = note: required because it appears within the type `{Opaque(DefId(0:42 ~ generator_print_verbose_1[70c9]::make_non_send_generator2::{opaque#0}), []), ()}`
-    = note: required because it appears within the type `[test2::{closure#0} upvar_tys=() {Opaque(DefId(0:42 ~ generator_print_verbose_1[70c9]::make_non_send_generator2::{opaque#0}), []), ()}]`
+    = note: required because it appears within the type `Opaque(DefId(0:39 ~ generator_print_verbose_1[e574]::make_gen2::{opaque#0}), [std::sync::Arc<std::cell::RefCell<i32>>])`
+    = note: required because it appears within the type `Opaque(DefId(0:42 ~ generator_print_verbose_1[e574]::make_non_send_generator2::{opaque#0}), [])`
+    = note: required because it appears within the type `{Opaque(DefId(0:42 ~ generator_print_verbose_1[e574]::make_non_send_generator2::{opaque#0}), []), ()}`
+    = note: required because it appears within the type `[test2::{closure#0} upvar_tys=() {Opaque(DefId(0:42 ~ generator_print_verbose_1[e574]::make_non_send_generator2::{opaque#0}), []), ()}]`
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
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
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
   |             ------------- has type `Opaque(DefId(0:34 ~ generator_print_verbose_1[e574]::make_non_send_generator::{opaque#0}), [])` which is not `Send`
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
   = note: required because it appears within the type `Opaque(DefId(0:39 ~ generator_print_verbose_1[e574]::make_gen2::{opaque#0}), [std::sync::Arc<std::cell::RefCell<i32>>])`
   = note: required because it appears within the type `Opaque(DefId(0:42 ~ generator_print_verbose_1[e574]::make_non_send_generator2::{opaque#0}), [])`
   = note: required because it appears within the type `{Opaque(DefId(0:42 ~ generator_print_verbose_1[e574]::make_non_send_generator2::{opaque#0}), []), ()}`
   = note: required because it appears within the type `[test2::{closure#0} upvar_tys=() {Opaque(DefId(0:42 ~ generator_print_verbose_1[e574]::make_non_send_generator2::{opaque#0}), []), ()}]`
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
+ error: failed to load bc of "lto-duplicate-symbols2.lto_duplicate_symbols2.e88cfaa5-cgu.0.rcgu.o": 
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

error: failed to load bc of "lto-duplicate-symbols2.lto_duplicate_symbols2.e88cfaa5-cgu.0.rcgu.o": 
error: aborting due to previous error; 1 warning emitted


------------------------------------------
------------------------------------------


---- [ui] ui/single-use-lifetime/one-use-in-fn-argument-in-band.rs stdout ----
diff of stderr:

- error: lifetime parameter `'a` only used once
-   --> $DIR/one-use-in-fn-argument-in-band.rs:11:10
+ error: lifetime parameter `'b` only used once
+   --> $DIR/one-use-in-fn-argument-in-band.rs:11:22
3    |
4 LL | fn a(x: &'a u32, y: &'b u32) {
-    |          ^^-
-    |          this lifetime is only used here
-    |          help: elide the single-use lifetime
+    |                      ^^-
+    |                      |
+    |                      |
+    |                      this lifetime is only used here
+    |                      help: elide the single-use lifetime
9    |
10 note: the lint level is defined here
11   --> $DIR/one-use-in-fn-argument-in-band.rs:4:9

13 LL | #![deny(single_use_lifetimes)]
15 
15 
- error: lifetime parameter `'b` only used once
-   --> $DIR/one-use-in-fn-argument-in-band.rs:11:22
+ error: lifetime parameter `'a` only used once
+   --> $DIR/one-use-in-fn-argument-in-band.rs:11:10
18    |
19 LL | fn a(x: &'a u32, y: &'b u32) {
-    |                      ^^-
-    |                      this lifetime is only used here
-    |                      help: elide the single-use lifetime
+    |          ^^-
+    |          |
---
To only update this specific test, also pass `--test-args single-use-lifetime/one-use-in-fn-argument-in-band.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/single-use-lifetime/one-use-in-fn-argument-in-band.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/single-use-lifetime/one-use-in-fn-argument-in-band" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/single-use-lifetime/one-use-in-fn-argument-in-band/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: lifetime parameter `'b` only used once
   |
   |
LL | fn a(x: &'a u32, y: &'b u32) {
   |                      ^^-
   |                      this lifetime is only used here
   |                      help: elide the single-use lifetime
   |
note: the lint level is defined here
note: the lint level is defined here
  --> /checkout/src/test/ui/single-use-lifetime/one-use-in-fn-argument-in-band.rs:4:9
   |
LL | #![deny(single_use_lifetimes)]


error: lifetime parameter `'a` only used once
   |
   |
LL | fn a(x: &'a u32, y: &'b u32) {
   |          ^^-
   |          this lifetime is only used here
   |          help: elide the single-use lifetime

error: aborting due to 2 previous errors
---

---- [ui] ui/specialization/min_specialization/repeated_projection_type.rs stdout ----
diff of stderr:

- error: cannot specialize on `Binder(ProjectionPredicate(ProjectionTy { substs: [V], item_def_id: DefId(0:6 ~ repeated_projection_type[b09c]::Id::This) }, (I,)), [])`
+ error: cannot specialize on `Binder(ProjectionPredicate(ProjectionTy { substs: [V], item_def_id: DefId(0:6 ~ repeated_projection_type[a430]::Id::This) }, (I,)), [])`
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
error: cannot specialize on `Binder(ProjectionPredicate(ProjectionTy { substs: [V], item_def_id: DefId(0:6 ~ repeated_projection_type[a430]::Id::This) }, (I,)), [])`
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
+ error: symbol-name(_ZN5basic4main17h5b9d58ecfe0016dbE)
3    |
4 LL | #[rustc_symbol_name]

5    | ^^^^^^^^^^^^^^^^^^^^
5    | ^^^^^^^^^^^^^^^^^^^^
6 
- error: demangling(basic::main::hd75b915511563828)
+ error: demangling(basic::main::h5b9d58ecfe0016db)
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
error: symbol-name(_ZN5basic4main17h5b9d58ecfe0016dbE)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling(basic::main::h5b9d58ecfe0016db)
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


- error: symbol-name(_RNvCsj6j3mjPNGKx_5basic4main)
+ error: symbol-name(_RNvCseG8oq8IGSTU_5basic4main)
3    |
4 LL | #[rustc_symbol_name]

5    | ^^^^^^^^^^^^^^^^^^^^
5    | ^^^^^^^^^^^^^^^^^^^^
6 
- error: demangling(basic[de7d5b6b69c71f37]::main)
+ error: demangling(basic[aafb7f9486813d54]::main)
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
error: symbol-name(_RNvCseG8oq8IGSTU_5basic4main)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling(basic[aafb7f9486813d54]::main)
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

- error: symbol-name(_RMCsno73SFvQKx_1cINtB0_8UnsignedKhb_E)
+ error: symbol-name(_RMCs6PneRttvLjJ_1cINtB0_8UnsignedKhb_E)
3    |
4 LL | #[rustc_symbol_name]

5    | ^^^^^^^^^^^^^^^^^^^^
5    | ^^^^^^^^^^^^^^^^^^^^
6 
- error: demangling(<c[464da6a86cb672f]::Unsigned<11u8>>)
+ error: demangling(<c[4f89883736281bfd]::Unsigned<11u8>>)
9    |
10 LL | #[rustc_symbol_name]

16 LL | #[rustc_symbol_name]
16 LL | #[rustc_symbol_name]
17    | ^^^^^^^^^^^^^^^^^^^^
18 
- error: symbol-name(_RMs_Csno73SFvQKx_1cINtB2_6SignedKsn98_E)
+ error: symbol-name(_RMs_Cs6PneRttvLjJ_1cINtB2_6SignedKsn98_E)
21    |
22 LL | #[rustc_symbol_name]

23    | ^^^^^^^^^^^^^^^^^^^^
23    | ^^^^^^^^^^^^^^^^^^^^
24 
- error: demangling(<c[464da6a86cb672f]::Signed<-152i16>>)
+ error: demangling(<c[4f89883736281bfd]::Signed<-152i16>>)
27    |
28 LL | #[rustc_symbol_name]

34 LL | #[rustc_symbol_name]
34 LL | #[rustc_symbol_name]
35    | ^^^^^^^^^^^^^^^^^^^^
36 
- error: symbol-name(_RMs0_Csno73SFvQKx_1cINtB3_4BoolKb1_E)
+ error: symbol-name(_RMs0_Cs6PneRttvLjJ_1cINtB3_4BoolKb1_E)
39    |
40 LL | #[rustc_symbol_name]

41    | ^^^^^^^^^^^^^^^^^^^^
41    | ^^^^^^^^^^^^^^^^^^^^
42 
- error: demangling(<c[464da6a86cb672f]::Bool<true>>)
+ error: demangling(<c[4f89883736281bfd]::Bool<true>>)
45    |
46 LL | #[rustc_symbol_name]

52 LL | #[rustc_symbol_name]
52 LL | #[rustc_symbol_name]
53    | ^^^^^^^^^^^^^^^^^^^^
54 
- error: symbol-name(_RMs1_Csno73SFvQKx_1cINtB3_4CharKc2202_E)
+ error: symbol-name(_RMs1_Cs6PneRttvLjJ_1cINtB3_4CharKc2202_E)
57    |
58 LL | #[rustc_symbol_name]

59    | ^^^^^^^^^^^^^^^^^^^^
59    | ^^^^^^^^^^^^^^^^^^^^
60 
- error: demangling(<c[464da6a86cb672f]::Char<'∂'>>)
+ error: demangling(<c[4f89883736281bfd]::Char<'∂'>>)
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
error: symbol-name(_RMCs6PneRttvLjJ_1cINtB0_8UnsignedKhb_E)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling(<c[4f89883736281bfd]::Unsigned<11u8>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<c::Unsigned<11>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: symbol-name(_RMs_Cs6PneRttvLjJ_1cINtB2_6SignedKsn98_E)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling(<c[4f89883736281bfd]::Signed<-152i16>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<c::Signed<-152>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: symbol-name(_RMs0_Cs6PneRttvLjJ_1cINtB3_4BoolKb1_E)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling(<c[4f89883736281bfd]::Bool<true>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<c::Bool<true>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: symbol-name(_RMs1_Cs6PneRttvLjJ_1cINtB3_4CharKc2202_E)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling(<c[4f89883736281bfd]::Char<'∂'>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<c::Char<'∂'>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^

---

---- [ui] ui/symbol-names/const-generics-str-demangling.rs stdout ----
diff of stderr:

- error: symbol-name(_RMCsno73SFvQKx_1cINtB0_3StrKRe616263_E)
+ error: symbol-name(_RMCs6PneRttvLjJ_1cINtB0_3StrKRe616263_E)
3    |
4 LL | #[rustc_symbol_name]

5    | ^^^^^^^^^^^^^^^^^^^^
5    | ^^^^^^^^^^^^^^^^^^^^
6 
- error: demangling(<c[464da6a86cb672f]::Str<"abc">>)
+ error: demangling(<c[4f89883736281bfd]::Str<"abc">>)
9    |
10 LL | #[rustc_symbol_name]

16 LL | #[rustc_symbol_name]
16 LL | #[rustc_symbol_name]
17    | ^^^^^^^^^^^^^^^^^^^^
18 
- error: symbol-name(_RMs_Csno73SFvQKx_1cINtB2_3StrKRe27_E)
+ error: symbol-name(_RMs_Cs6PneRttvLjJ_1cINtB2_3StrKRe27_E)
21    |
22 LL | #[rustc_symbol_name]

23    | ^^^^^^^^^^^^^^^^^^^^
23    | ^^^^^^^^^^^^^^^^^^^^
24 
- error: demangling(<c[464da6a86cb672f]::Str<"'">>)
+ error: demangling(<c[4f89883736281bfd]::Str<"'">>)
27    |
28 LL | #[rustc_symbol_name]

34 LL | #[rustc_symbol_name]
34 LL | #[rustc_symbol_name]
35    | ^^^^^^^^^^^^^^^^^^^^
36 
- error: symbol-name(_RMs0_Csno73SFvQKx_1cINtB3_3StrKRe090a_E)
+ error: symbol-name(_RMs0_Cs6PneRttvLjJ_1cINtB3_3StrKRe090a_E)
39    |
40 LL | #[rustc_symbol_name]

41    | ^^^^^^^^^^^^^^^^^^^^
41    | ^^^^^^^^^^^^^^^^^^^^
42 
- error: demangling(<c[464da6a86cb672f]::Str<"\t\n">>)
+ error: demangling(<c[4f89883736281bfd]::Str<"\t\n">>)
45    |
46 LL | #[rustc_symbol_name]

52 LL | #[rustc_symbol_name]
52 LL | #[rustc_symbol_name]
53    | ^^^^^^^^^^^^^^^^^^^^
54 
- error: symbol-name(_RMs1_Csno73SFvQKx_1cINtB3_3StrKRee28882c3bc_E)
+ error: symbol-name(_RMs1_Cs6PneRttvLjJ_1cINtB3_3StrKRee28882c3bc_E)
57    |
58 LL | #[rustc_symbol_name]

59    | ^^^^^^^^^^^^^^^^^^^^
59    | ^^^^^^^^^^^^^^^^^^^^
60 
- error: demangling(<c[464da6a86cb672f]::Str<"∂ü">>)
+ error: demangling(<c[4f89883736281bfd]::Str<"∂ü">>)
63    |
64 LL | #[rustc_symbol_name]

70 LL | #[rustc_symbol_name]
70 LL | #[rustc_symbol_name]
71    | ^^^^^^^^^^^^^^^^^^^^
72 
- error: symbol-name(_RMs2_Csno73SFvQKx_1cINtB3_3StrKRee183a1e18390e183ade1839be18394e1839ae18390e183935fe18392e18394e1839be183a0e18398e18394e1839ae183985fe183a1e18390e18393e18398e1839ae18398_E)
+ error: symbol-name(_RMs2_Cs6PneRttvLjJ_1cINtB3_3StrKRee183a1e18390e183ade1839be18394e1839ae18390e183935fe18392e18394e1839be183a0e18398e18394e1839ae183985fe183a1e18390e18393e18398e1839ae18398_E)
75    |
76 LL | #[rustc_symbol_name]

77    | ^^^^^^^^^^^^^^^^^^^^
77    | ^^^^^^^^^^^^^^^^^^^^
78 
- error: demangling(<c[464da6a86cb672f]::Str<"საჭმელად_გემრიელი_სადილი">>)
+ error: demangling(<c[4f89883736281bfd]::Str<"საჭმელად_გემრიელი_სადილი">>)
81    |
82 LL | #[rustc_symbol_name]

88 LL | #[rustc_symbol_name]
88 LL | #[rustc_symbol_name]
89    | ^^^^^^^^^^^^^^^^^^^^
90 
- error: symbol-name(_RMs3_Csno73SFvQKx_1cINtB3_3StrKRef09f908af09fa688f09fa686f09f90ae20c2a720f09f90b6f09f9192e29895f09f94a520c2a720f09fa7a1f09f929bf09f929af09f9299f09f929c_E)
+ error: symbol-name(_RMs3_Cs6PneRttvLjJ_1cINtB3_3StrKRef09f908af09fa688f09fa686f09f90ae20c2a720f09f90b6f09f9192e29895f09f94a520c2a720f09fa7a1f09f929bf09f929af09f9299f09f929c_E)
93    |
94 LL | #[rustc_symbol_name]

95    | ^^^^^^^^^^^^^^^^^^^^
95    | ^^^^^^^^^^^^^^^^^^^^
96 
- error: demangling(<c[464da6a86cb672f]::Str<"🐊🦈🦆🐮 § 🐶👒☕🔥 § 🧡💛💚💙💜">>)
+ error: demangling(<c[4f89883736281bfd]::Str<"🐊🦈🦆🐮 § 🐶👒☕🔥 § 🧡💛💚💙💜">>)
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
error: symbol-name(_RMCs6PneRttvLjJ_1cINtB0_3StrKRe616263_E)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling(<c[4f89883736281bfd]::Str<"abc">>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<c::Str<"abc">>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: symbol-name(_RMs_Cs6PneRttvLjJ_1cINtB2_3StrKRe27_E)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling(<c[4f89883736281bfd]::Str<"'">>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<c::Str<"'">>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: symbol-name(_RMs0_Cs6PneRttvLjJ_1cINtB3_3StrKRe090a_E)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling(<c[4f89883736281bfd]::Str<"\t\n">>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<c::Str<"\t\n">>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: symbol-name(_RMs1_Cs6PneRttvLjJ_1cINtB3_3StrKRee28882c3bc_E)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling(<c[4f89883736281bfd]::Str<"∂ü">>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<c::Str<"∂ü">>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: symbol-name(_RMs2_Cs6PneRttvLjJ_1cINtB3_3StrKRee183a1e18390e183ade1839be18394e1839ae18390e183935fe18392e18394e1839be183a0e18398e18394e1839ae183985fe183a1e18390e18393e18398e1839ae18398_E)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling(<c[4f89883736281bfd]::Str<"საჭმელად_გემრიელი_სადილი">>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<c::Str<"საჭმელად_გემრიელი_სადილი">>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: symbol-name(_RMs3_Cs6PneRttvLjJ_1cINtB3_3StrKRef09f908af09fa688f09fa686f09f90ae20c2a720f09f90b6f09f9192e29895f09f94a520c2a720f09fa7a1f09f929bf09f929af09f9299f09f929c_E)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling(<c[4f89883736281bfd]::Str<"🐊🦈🦆🐮 § 🐶👒☕🔥 § 🧡💛💚💙💜">>)
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


- error: symbol-name(_RNvMNtCs2qSCrjELJET_5impl13fooNtB2_3Foo3bar)
+ error: symbol-name(_RNvMNtCsagfdWIBZnie_5impl13fooNtB2_3Foo3bar)
3    |
4 LL |         #[rustc_symbol_name]

5    |         ^^^^^^^^^^^^^^^^^^^^
5    |         ^^^^^^^^^^^^^^^^^^^^
6 
- error: demangling(<impl1[1c5860ab79c9e305]::foo::Foo>::bar)
+ error: demangling(<impl1[77873528b2aee6b0]::foo::Foo>::bar)
9    |
10 LL |         #[rustc_symbol_name]

22 LL |         #[rustc_def_path]
22 LL |         #[rustc_def_path]
23    |         ^^^^^^^^^^^^^^^^^
24 
- error: symbol-name(_RNvMNtCs2qSCrjELJET_5impl13barNtNtB4_3foo3Foo3baz)
+ error: symbol-name(_RNvMNtCsagfdWIBZnie_5impl13barNtNtB4_3foo3Foo3baz)
27    |
28 LL |         #[rustc_symbol_name]

29    |         ^^^^^^^^^^^^^^^^^^^^
29    |         ^^^^^^^^^^^^^^^^^^^^
30 
- error: demangling(<impl1[1c5860ab79c9e305]::foo::Foo>::baz)
+ error: demangling(<impl1[77873528b2aee6b0]::foo::Foo>::baz)
33    |
34 LL |         #[rustc_symbol_name]

46 LL |         #[rustc_def_path]
46 LL |         #[rustc_def_path]
47    |         ^^^^^^^^^^^^^^^^^
48 
- error: symbol-name(_RNvXNCNvCs2qSCrjELJET_5impl14mains_0ARDNtB6_3Foop5AssocFG_KCRL0_hvEuNtB6_9AutoTraitEL_j3_NtB2_3Bar6method)
+ error: symbol-name(_RNvXNCNvCsagfdWIBZnie_5impl14mains_0ARDNtB6_3Foop5AssocFG_KCRL0_hvEuNtB6_9AutoTraitEL_j3_NtB2_3Bar6method)
51    |
52 LL |             #[rustc_symbol_name]

53    |             ^^^^^^^^^^^^^^^^^^^^
53    |             ^^^^^^^^^^^^^^^^^^^^
54 
- error: demangling(<[&dyn impl1[1c5860ab79c9e305]::Foo<Assoc = for<'a> extern "C" fn(&'a u8, ...)> + impl1[1c5860ab79c9e305]::AutoTrait; 3usize] as impl1[1c5860ab79c9e305]::main::{closure#1}::Bar>::method)
+ error: demangling(<[&dyn impl1[77873528b2aee6b0]::Foo<Assoc = for<'a> extern "C" fn(&'a u8, ...)> + impl1[77873528b2aee6b0]::AutoTrait; 3usize] as impl1[77873528b2aee6b0]::main::{closure#1}::Bar>::method)
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
error: symbol-name(_RNvMNtCsagfdWIBZnie_5impl13fooNtB2_3Foo3bar)
   |
LL |         #[rustc_symbol_name]
   |         ^^^^^^^^^^^^^^^^^^^^


error: demangling(<impl1[77873528b2aee6b0]::foo::Foo>::bar)
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


error: symbol-name(_RNvMNtCsagfdWIBZnie_5impl13barNtNtB4_3foo3Foo3baz)
   |
LL |         #[rustc_symbol_name]
   |         ^^^^^^^^^^^^^^^^^^^^


error: demangling(<impl1[77873528b2aee6b0]::foo::Foo>::baz)
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


error: symbol-name(_RNvXNCNvCsagfdWIBZnie_5impl14mains_0ARDNtB6_3Foop5AssocFG_KCRL0_hvEuNtB6_9AutoTraitEL_j3_NtB2_3Bar6method)
   |
LL |             #[rustc_symbol_name]
   |             ^^^^^^^^^^^^^^^^^^^^


error: demangling(<[&dyn impl1[77873528b2aee6b0]::Foo<Assoc = for<'a> extern "C" fn(&'a u8, ...)> + impl1[77873528b2aee6b0]::AutoTrait; 3usize] as impl1[77873528b2aee6b0]::main::{closure#1}::Bar>::method)
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


- error: symbol-name(_RNvMNtCs8dUWfuENynB_11issue_609253fooINtB2_3FooNtNtB4_4llvm3FooE3foo)
+ error: symbol-name(_RNvMNtCsekGWGG1IaCi_11issue_609253fooINtB2_3FooNtNtB4_4llvm3FooE3foo)
3    |
4 LL |         #[rustc_symbol_name]

5    |         ^^^^^^^^^^^^^^^^^^^^
5    |         ^^^^^^^^^^^^^^^^^^^^
6 
- error: demangling(<issue_60925[5fcbb46c6fac4139]::foo::Foo<issue_60925[5fcbb46c6fac4139]::llvm::Foo>>::foo)
+ error: demangling(<issue_60925[a6f43f09a5740460]::foo::Foo<issue_60925[a6f43f09a5740460]::llvm::Foo>>::foo)
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
error: symbol-name(_RNvMNtCsekGWGG1IaCi_11issue_609253fooINtB2_3FooNtNtB4_4llvm3FooE3foo)
   |
LL |         #[rustc_symbol_name]
   |         ^^^^^^^^^^^^^^^^^^^^


error: demangling(<issue_60925[a6f43f09a5740460]::foo::Foo<issue_60925[a6f43f09a5740460]::llvm::Foo>>::foo)
   |
LL |         #[rustc_symbol_name]
   |         ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<issue_60925::foo::Foo<issue_60925::llvm::Foo>>::foo)
   |
LL |         #[rustc_symbol_name]
   |         ^^^^^^^^^^^^^^^^^^^^

---

---- [ui] ui/symbol-names/const-generics-structural-demangling.rs stdout ----
diff of stderr:

- error: symbol-name(_RMCsno73SFvQKx_1cINtB0_7RefByteKRh7b_E)
+ error: symbol-name(_RMCs6PneRttvLjJ_1cINtB0_7RefByteKRh7b_E)
3    |
4 LL | #[rustc_symbol_name]

5    | ^^^^^^^^^^^^^^^^^^^^
5    | ^^^^^^^^^^^^^^^^^^^^
6 
- error: demangling(<c[464da6a86cb672f]::RefByte<{&123u8}>>)
+ error: demangling(<c[4f89883736281bfd]::RefByte<{&123u8}>>)
9    |
10 LL | #[rustc_symbol_name]

16 LL | #[rustc_symbol_name]
16 LL | #[rustc_symbol_name]
17    | ^^^^^^^^^^^^^^^^^^^^
18 
- error: symbol-name(_RMs_Csno73SFvQKx_1cINtB2_6RefZstKRAEE)
+ error: symbol-name(_RMs_Cs6PneRttvLjJ_1cINtB2_6RefZstKRAEE)
21    |
22 LL | #[rustc_symbol_name]

23    | ^^^^^^^^^^^^^^^^^^^^
23    | ^^^^^^^^^^^^^^^^^^^^
24 
- error: demangling(<c[464da6a86cb672f]::RefZst<{&[]}>>)
+ error: demangling(<c[4f89883736281bfd]::RefZst<{&[]}>>)
27    |
28 LL | #[rustc_symbol_name]

34 LL | #[rustc_symbol_name]
34 LL | #[rustc_symbol_name]
35    | ^^^^^^^^^^^^^^^^^^^^
36 
- error: symbol-name(_RMs0_Csno73SFvQKx_1cINtB3_11Array3BytesKAh1_h2_h3_EE)
+ error: symbol-name(_RMs0_Cs6PneRttvLjJ_1cINtB3_11Array3BytesKAh1_h2_h3_EE)
39    |
40 LL | #[rustc_symbol_name]

41    | ^^^^^^^^^^^^^^^^^^^^
41    | ^^^^^^^^^^^^^^^^^^^^
42 
- error: demangling(<c[464da6a86cb672f]::Array3Bytes<{[1u8, 2u8, 3u8]}>>)
+ error: demangling(<c[4f89883736281bfd]::Array3Bytes<{[1u8, 2u8, 3u8]}>>)
45    |
46 LL | #[rustc_symbol_name]

52 LL | #[rustc_symbol_name]
52 LL | #[rustc_symbol_name]
53    | ^^^^^^^^^^^^^^^^^^^^
54 
- error: symbol-name(_RMs1_Csno73SFvQKx_1cINtB3_13TupleByteBoolKTh1_b0_EE)
+ error: symbol-name(_RMs1_Cs6PneRttvLjJ_1cINtB3_13TupleByteBoolKTh1_b0_EE)
57    |
58 LL | #[rustc_symbol_name]

59    | ^^^^^^^^^^^^^^^^^^^^
59    | ^^^^^^^^^^^^^^^^^^^^
60 
- error: demangling(<c[464da6a86cb672f]::TupleByteBool<{(1u8, false)}>>)
+ error: demangling(<c[4f89883736281bfd]::TupleByteBool<{(1u8, false)}>>)
63    |
64 LL | #[rustc_symbol_name]

70 LL | #[rustc_symbol_name]
70 LL | #[rustc_symbol_name]
71    | ^^^^^^^^^^^^^^^^^^^^
72 
- error: symbol-name(_RMs2_Csno73SFvQKx_1cINtB3_11OptionUsizeKVNtINtNtCs$HASH_4core6option6OptionjE4NoneUE)
+ error: symbol-name(_RMs2_Cs6PneRttvLjJ_1cINtB3_11OptionUsizeKVNtINtNtCs$HASH_4core6option6OptionjE4NoneUE)
75    |
76 LL | #[rustc_symbol_name]

77    | ^^^^^^^^^^^^^^^^^^^^
77    | ^^^^^^^^^^^^^^^^^^^^
78 
- error: demangling(<c[464da6a86cb672f]::OptionUsize<{core[$HASH_HEX]::option::Option::<usize>::None}>>)
+ error: demangling(<c[4f89883736281bfd]::OptionUsize<{core[$HASH_HEX]::option::Option::<usize>::None}>>)
81    |
82 LL | #[rustc_symbol_name]

88 LL | #[rustc_symbol_name]
88 LL | #[rustc_symbol_name]
89    | ^^^^^^^^^^^^^^^^^^^^
90 
- error: symbol-name(_RMs3_Csno73SFvQKx_1cINtB3_11OptionUsizeKVNtINtNtCs$HASH_4core6option6OptionjE4SomeTj0_EE)
+ error: symbol-name(_RMs3_Cs6PneRttvLjJ_1cINtB3_11OptionUsizeKVNtINtNtCs$HASH_4core6option6OptionjE4SomeTj0_EE)
93    |
94 LL | #[rustc_symbol_name]

95    | ^^^^^^^^^^^^^^^^^^^^
95    | ^^^^^^^^^^^^^^^^^^^^
96 
- error: demangling(<c[464da6a86cb672f]::OptionUsize<{core[$HASH_HEX]::option::Option::<usize>::Some(0usize)}>>)
+ error: demangling(<c[4f89883736281bfd]::OptionUsize<{core[$HASH_HEX]::option::Option::<usize>::Some(0usize)}>>)
99    |
100 LL | #[rustc_symbol_name]

106 LL | #[rustc_symbol_name]
106 LL | #[rustc_symbol_name]
107    | ^^^^^^^^^^^^^^^^^^^^
108 
- error: symbol-name(_RMs4_Csno73SFvQKx_1cINtB3_4Foo_KVNtB3_3FooS1sRe616263_2chc78_5sliceRAh1_h2_h3_EEE)
+ error: symbol-name(_RMs4_Cs6PneRttvLjJ_1cINtB3_4Foo_KVNtB3_3FooS1sRe616263_2chc78_5sliceRAh1_h2_h3_EEE)
111    |
112 LL | #[rustc_symbol_name]

113    | ^^^^^^^^^^^^^^^^^^^^
113    | ^^^^^^^^^^^^^^^^^^^^
114 
- error: demangling(<c[464da6a86cb672f]::Foo_<{c[464da6a86cb672f]::Foo { s: "abc", ch: 'x', slice: &[1u8, 2u8, 3u8] }}>>)
+ error: demangling(<c[4f89883736281bfd]::Foo_<{c[4f89883736281bfd]::Foo { s: "abc", ch: 'x', slice: &[1u8, 2u8, 3u8] }}>>)
117    |
118 LL | #[rustc_symbol_name]

124 LL | #[rustc_symbol_name]
124 LL | #[rustc_symbol_name]
125    | ^^^^^^^^^^^^^^^^^^^^
126 
- error: symbol-name(_RMs9_Csno73SFvQKx_1cINtB3_4Bar_KVNtB3_3BarS1xh7b_s_1xt1000_EE)
+ error: symbol-name(_RMs9_Cs6PneRttvLjJ_1cINtB3_4Bar_KVNtB3_3BarS1xh7b_s_1xt1000_EE)
129    |
130 LL |     #[rustc_symbol_name]

135    |
135    |
136    = note: this error originates in the macro `duplicate_field_name_test` (in Nightly builds, run with -Z macro-backtrace for more info)
137 
- error: demangling(<c[464da6a86cb672f]::Bar_<{c[464da6a86cb672f]::Bar { x: 123u8, x: 4096u16 }}>>)
+ error: demangling(<c[4f89883736281bfd]::Bar_<{c[4f89883736281bfd]::Bar { x: 123u8, x: 4096u16 }}>>)
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
error: symbol-name(_RMCs6PneRttvLjJ_1cINtB0_7RefByteKRh7b_E)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling(<c[4f89883736281bfd]::RefByte<{&123u8}>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<c::RefByte<{&123}>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: symbol-name(_RMs_Cs6PneRttvLjJ_1cINtB2_6RefZstKRAEE)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling(<c[4f89883736281bfd]::RefZst<{&[]}>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<c::RefZst<{&[]}>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: symbol-name(_RMs0_Cs6PneRttvLjJ_1cINtB3_11Array3BytesKAh1_h2_h3_EE)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling(<c[4f89883736281bfd]::Array3Bytes<{[1u8, 2u8, 3u8]}>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<c::Array3Bytes<{[1, 2, 3]}>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: symbol-name(_RMs1_Cs6PneRttvLjJ_1cINtB3_13TupleByteBoolKTh1_b0_EE)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling(<c[4f89883736281bfd]::TupleByteBool<{(1u8, false)}>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<c::TupleByteBool<{(1, false)}>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: symbol-name(_RMs2_Cs6PneRttvLjJ_1cINtB3_11OptionUsizeKVNtINtNtCscH748Bqsy8j_4core6option6OptionjE4NoneUE)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling(<c[4f89883736281bfd]::OptionUsize<{core[93defb4af715c5ad]::option::Option::<usize>::None}>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<c::OptionUsize<{core::option::Option::<usize>::None}>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: symbol-name(_RMs3_Cs6PneRttvLjJ_1cINtB3_11OptionUsizeKVNtINtNtCscH748Bqsy8j_4core6option6OptionjE4SomeTj0_EE)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling(<c[4f89883736281bfd]::OptionUsize<{core[93defb4af715c5ad]::option::Option::<usize>::Some(0usize)}>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<c::OptionUsize<{core::option::Option::<usize>::Some(0)}>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: symbol-name(_RMs4_Cs6PneRttvLjJ_1cINtB3_4Foo_KVNtB3_3FooS1sRe616263_2chc78_5sliceRAh1_h2_h3_EEE)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling(<c[4f89883736281bfd]::Foo_<{c[4f89883736281bfd]::Foo { s: "abc", ch: 'x', slice: &[1u8, 2u8, 3u8] }}>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<c::Foo_<{c::Foo { s: "abc", ch: 'x', slice: &[1, 2, 3] }}>>)
   |
LL | #[rustc_symbol_name]
   | ^^^^^^^^^^^^^^^^^^^^


error: symbol-name(_RMs9_Cs6PneRttvLjJ_1cINtB3_4Bar_KVNtB3_3BarS1xh7b_s_1xt1000_EE)
   |
LL |     #[rustc_symbol_name]
   |     ^^^^^^^^^^^^^^^^^^^^
...
...
LL | duplicate_field_name_test!(x);
   |
   = note: this error originates in the macro `duplicate_field_name_test` (in Nightly builds, run with -Z macro-backtrace for more info)


error: demangling(<c[4f89883736281bfd]::Bar_<{c[4f89883736281bfd]::Bar { x: 123u8, x: 4096u16 }}>>)
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


---- [ui] ui/symbol-names/issue-75326.rs#v0 stdout ----


- error: symbol-name(_RNvXINICsiMBouZZ1iuD_11issue_75326s_0pppEINtB5_3FooppENtB5_9Iterator24nextB5_)
+ error: symbol-name(_RNvXINICsbihAvbRTagw_11issue_75326s_0pppEINtB5_3FooppENtB5_9Iterator24nextB5_)
3    |
4 LL |     #[rustc_symbol_name]

5    |     ^^^^^^^^^^^^^^^^^^^^
5    |     ^^^^^^^^^^^^^^^^^^^^
6 
- error: demangling(<issue_75326[dac9b7624645f95d]::Foo<_, _> as issue_75326[dac9b7624645f95d]::Iterator2>::next)
+ error: demangling(<issue_75326[838f04b48393d222]::Foo<_, _> as issue_75326[838f04b48393d222]::Iterator2>::next)
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
error: symbol-name(_RNvXINICsbihAvbRTagw_11issue_75326s_0pppEINtB5_3FooppENtB5_9Iterator24nextB5_)
   |
LL |     #[rustc_symbol_name]
   |     ^^^^^^^^^^^^^^^^^^^^


error: demangling(<issue_75326[838f04b48393d222]::Foo<_, _> as issue_75326[838f04b48393d222]::Iterator2>::next)
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

- error: symbol-name(_ZN11issue_609253foo37Foo$LT$issue_60925..llv$u6d$..Foo$GT$3foo17h18eaa05e22e59176E)
+ error: symbol-name(_ZN11issue_609253foo37Foo$LT$issue_60925..llv$u6d$..Foo$GT$3foo17hcecc1490d9d45e05E)
3    |
4 LL |         #[rustc_symbol_name]

5    |         ^^^^^^^^^^^^^^^^^^^^
5    |         ^^^^^^^^^^^^^^^^^^^^
6 
- error: demangling(issue_60925::foo::Foo<issue_60925::llvm::Foo>::foo::h18eaa05e22e59176)
+ error: demangling(issue_60925::foo::Foo<issue_60925::llvm::Foo>::foo::hcecc1490d9d45e05)
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
error: symbol-name(_ZN11issue_609253foo37Foo$LT$issue_60925..llv$u6d$..Foo$GT$3foo17hcecc1490d9d45e05E)
   |
LL |         #[rustc_symbol_name]
   |         ^^^^^^^^^^^^^^^^^^^^


error: demangling(issue_60925::foo::Foo<issue_60925::llvm::Foo>::foo::hcecc1490d9d45e05)
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
- error: demangling(<&dyn for<'a> core[HASH]::ops::function::FnMut<(&'a u8,), Output = ()> as trait_objects[3f8b57f879016e18]::Bar>::method)
+ error: demangling(<&dyn for<'a> core[HASH]::ops::function::FnMut<(&'a u8,), Output = ()> as trait_objects[f750757cf19b9a63]::Bar>::method)
9    |
10 LL |     #[rustc_symbol_name]

22 LL |     #[rustc_symbol_name]
22 LL |     #[rustc_symbol_name]
23    |     ^^^^^^^^^^^^^^^^^^^^
24 
- error: demangling(<&dyn for<'a> core[HASH]::ops::function::FnMut<(&'a u8,), Output = ()> + core[HASH]::marker::Send as trait_objects[3f8b57f879016e18]::Foo>::method)
+ error: demangling(<&dyn for<'a> core[HASH]::ops::function::FnMut<(&'a u8,), Output = ()> + core[HASH]::marker::Send as trait_objects[f750757cf19b9a63]::Foo>::method)
27    |
28 LL |     #[rustc_symbol_name]

40 LL |     #[rustc_symbol_name]
40 LL |     #[rustc_symbol_name]
41    |     ^^^^^^^^^^^^^^^^^^^^
42 
- error: demangling(<&dyn for<'a> core[HASH]::ops::function::FnMut<(&'a u8,), Output = ()> + core[HASH]::marker::Send as trait_objects[3f8b57f879016e18]::Baz>::method)
+ error: demangling(<&dyn for<'a> core[HASH]::ops::function::FnMut<(&'a u8,), Output = ()> + core[HASH]::marker::Send as trait_objects[f750757cf19b9a63]::Baz>::method)
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
error: symbol-name(_RNvXCslerMN79bChj_13trait_objectsRDG_INtNtNtCscH748Bqsy8j_4core3ops8function5FnMutTRL0_hEEp6OutputuEL_NtB2_3Bar6method)
   |
LL |     #[rustc_symbol_name]
   |     ^^^^^^^^^^^^^^^^^^^^


error: demangling(<&dyn for<'a> core[93defb4af715c5ad]::ops::function::FnMut<(&'a u8,), Output = ()> as trait_objects[f750757cf19b9a63]::Bar>::method)
   |
LL |     #[rustc_symbol_name]
   |     ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<&dyn for<'a> core::ops::function::FnMut<(&'a u8,), Output = ()> as trait_objects::Bar>::method)
   |
LL |     #[rustc_symbol_name]
   |     ^^^^^^^^^^^^^^^^^^^^


error: symbol-name(_RNvXs_CslerMN79bChj_13trait_objectsRDG_INtNtNtCscH748Bqsy8j_4core3ops8function5FnMutTRL0_hEEp6OutputuNtNtBI_6marker4SendEL_NtB4_3Foo6method)
   |
LL |     #[rustc_symbol_name]
   |     ^^^^^^^^^^^^^^^^^^^^


error: demangling(<&dyn for<'a> core[93defb4af715c5ad]::ops::function::FnMut<(&'a u8,), Output = ()> + core[93defb4af715c5ad]::marker::Send as trait_objects[f750757cf19b9a63]::Foo>::method)
   |
LL |     #[rustc_symbol_name]
   |     ^^^^^^^^^^^^^^^^^^^^


error: demangling-alt(<&dyn for<'a> core::ops::function::FnMut<(&'a u8,), Output = ()> + core::marker::Send as trait_objects::Foo>::method)
   |
LL |     #[rustc_symbol_name]
   |     ^^^^^^^^^^^^^^^^^^^^


error: symbol-name(_RNvXs0_CslerMN79bChj_13trait_objectsRDG_INtNtNtCscH748Bqsy8j_4core3ops8function5FnMutTRL0_hEEp6OutputuNtNtBJ_6marker4SendEL_NtB5_3Baz6method)
   |
LL |     #[rustc_symbol_name]
   |     ^^^^^^^^^^^^^^^^^^^^


error: demangling(<&dyn for<'a> core[93defb4af715c5ad]::ops::function::FnMut<(&'a u8,), Output = ()> + core[93defb4af715c5ad]::marker::Send as trait_objects[f750757cf19b9a63]::Baz>::method)
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
+ DefId(0:3 ~ thir_tree[5327]::main):
2 Thir {
3     arms: [],
4     exprs: [
30                 region_scope: Node(2),
31                 lint_level: Explicit(
32                     HirId {
32                     HirId {
-                         owner: DefId(0:3 ~ thir_tree[348d]::main),
+                         owner: DefId(0:3 ~ thir_tree[5327]::main),
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
DefId(0:3 ~ thir_tree[5327]::main):
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
                        owner: DefId(0:3 ~ thir_tree[5327]::main),
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
test result: FAILED. 12145 passed; 16 failed; 117 ignored; 0 measured; 0 filtered out; finished in 133.00s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:24
