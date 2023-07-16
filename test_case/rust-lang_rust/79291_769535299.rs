plain
.................................................................................................... 3600/11305
.................................................................................................... 3700/11305
.................................................................................................... 3800/11305
....................................................................................i............... 3900/11305
..............................F.......F............................................................. 4000/11305
.................................................................................................... 4200/11305
......................................ii............................................................ 4300/11305
......................i............................................................................. 4400/11305
.................................................................................................... 4500/11305
---
.................................................................................................... 8100/11305
.......................i............................................................................ 8200/11305
.................................................................................................... 8300/11305
........................................i........................................................... 8400/11305
.....................................................................................FF............. 8500/11305
.....................F.......F.....................................................................i 8600/11305
.................................................................................................... 8800/11305
.................................................................................................... 8900/11305
.................................................................................................... 9000/11305
.................................................................................................... 9100/11305
---
.................................................................................................... 11300/11305
.....
failures:

---- [ui] ui/const-generics/const_evaluatable_checked/eval-privacy.rs stdout ----


- warning: private function `fn(u8) -> u8 {my_const_fn}` in public interface (error E0446)
+ warning: private type `fn(u8) -> u8 {my_const_fn}` in public interface (error E0446)
3    |
3    |
4 LL | / impl<const U: u8> Trait for Const<U>
14    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
15    = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>
16 
16 
- warning: private function `fn(u8) -> u8 {my_const_fn}` in public interface (error E0446)
+ warning: private type `fn(u8) -> u8 {my_const_fn}` in public interface (error E0446)
19    |
19    |
20 LL | / impl<const U: u8> Trait for Const<U>
29    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
30    = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>
31 
31 
- error[E0446]: private function `fn(u8) -> u8 {my_const_fn}` in public interface
+ error[E0446]: private type `fn(u8) -> u8 {my_const_fn}` in public interface
34    |
34    |
35 LL |     type AssocTy = Const<{ my_const_fn(U) }>;
-    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ can't leak private function
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ can't leak private type
37 ...
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
38 LL | const fn my_const_fn(val: u8) -> u8 {
39    | ----------------------------------- `fn(u8) -> u8 {my_const_fn}` declared as private

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const_evaluatable_checked/eval-privacy/eval-privacy.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args const-generics/const_evaluatable_checked/eval-privacy.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/const_evaluatable_checked/eval-privacy.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const_evaluatable_checked/eval-privacy" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const_evaluatable_checked/eval-privacy/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: private type `fn(u8) -> u8 {my_const_fn}` in public interface (error E0446)
  --> /checkout/src/test/ui/const-generics/const_evaluatable_checked/eval-privacy.rs:12:1
   |
LL | / impl<const U: u8> Trait for Const<U>
LL | | //~^ WARN private function
LL | | //~| WARN this was previously
LL | | //~| WARN private function
LL | |     }
LL | | }
   | |_^
   |
   |
   = note: `#[warn(private_in_public)]` on by default
   = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>


warning: private type `fn(u8) -> u8 {my_const_fn}` in public interface (error E0446)
  --> /checkout/src/test/ui/const-generics/const_evaluatable_checked/eval-privacy.rs:12:1
   |
LL | / impl<const U: u8> Trait for Const<U>
LL | | //~^ WARN private function
LL | | //~| WARN this was previously
LL | | //~| WARN private function
LL | |     }
LL | | }
   | |_^
   |
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #34537 <https://github.com/rust-lang/rust/issues/34537>

error[E0446]: private type `fn(u8) -> u8 {my_const_fn}` in public interface
  --> /checkout/src/test/ui/const-generics/const_evaluatable_checked/eval-privacy.rs:21:5
   |
LL |     type AssocTy = Const<{ my_const_fn(U) }>;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ can't leak private type
...
LL | const fn my_const_fn(val: u8) -> u8 {
   | ----------------------------------- `fn(u8) -> u8 {my_const_fn}` declared as private
error: aborting due to previous error; 2 warnings emitted

For more information about this error, try `rustc --explain E0446`.


------------------------------------------


---- [ui] ui/hygiene/impl_items.rs stdout ----
diff of stderr:

- error: function `for<'r> fn(&'r foo::S) {foo::S::f}` is private
+ error: type `for<'r> fn(&'r foo::S) {foo::S::f}` is private
3    |
3    |
4 LL |         let _: () = S.f();
-    |                       ^ private function
+    |                       ^ private type
6 ...
6 ...
7 LL |     foo::m!();


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/impl_items/impl_items.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/impl_items/impl_items.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args hygiene/impl_items.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/hygiene/impl_items.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/impl_items" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/impl_items/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: type `for<'r> fn(&'r foo::S) {foo::S::f}` is private
   |
   |
LL |         let _: () = S.f(); //~ ERROR function `for<'r> fn(&'r foo::S) {foo::S::f}` is private
   |                       ^ private type
...
LL |     foo::m!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to previous error
---

---- [ui] ui/hygiene/intercrate.rs stdout ----
diff of stderr:

- error: function `fn() -> u32 {foo::bar::f}` is private
+ error: type `fn() -> u32 {foo::bar::f}` is private
3    |
3    |
4 LL |     assert_eq!(intercrate::foo::m!(), 1);
-    |                ^^^^^^^^^^^^^^^^^^^^^ private function
+    |                ^^^^^^^^^^^^^^^^^^^^^ private type
6    |
7    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
7    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
8 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/intercrate/intercrate.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args hygiene/intercrate.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/hygiene/intercrate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/intercrate" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/intercrate/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: type `fn() -> u32 {foo::bar::f}` is private
   |
   |
LL |     assert_eq!(intercrate::foo::m!(), 1);
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to previous error
---

---- [ui] ui/privacy/associated-item-privacy-inherent.rs stdout ----
diff of stderr:

- error: function `for<'r> fn(&'r priv_nominal::Pub) {priv_nominal::Pub::method}` is private
+ error: type `for<'r> fn(&'r priv_nominal::Pub) {priv_nominal::Pub::method}` is private
3    |
3    |
4 LL |         let value = Pub::method;
-    |                     ^^^^^^^^^^^ private function
+    |                     ^^^^^^^^^^^ private type
6 ...
6 ...
7 LL |     priv_nominal::mac!();

9    |
10    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
11 
11 
- error: function `for<'r> fn(&'r priv_nominal::Pub) {priv_nominal::Pub::method}` is private
+ error: type `for<'r> fn(&'r priv_nominal::Pub) {priv_nominal::Pub::method}` is private
14    |
15 LL |         value;

-    |         ^^^^^ private function
-    |         ^^^^^ private function
+    |         ^^^^^ private type
17 ...
18 LL |     priv_nominal::mac!();

20    |
21    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
22 
22 
- error: function `for<'r> fn(&'r priv_nominal::Pub) {priv_nominal::Pub::method}` is private
+ error: type `for<'r> fn(&'r priv_nominal::Pub) {priv_nominal::Pub::method}` is private
25    |
25    |
26 LL |         Pub.method();
-    |             ^^^^^^ private function
+    |             ^^^^^^ private type
28 ...
28 ...
29 LL |     priv_nominal::mac!();


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/associated-item-privacy-inherent/associated-item-privacy-inherent.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/associated-item-privacy-inherent/associated-item-privacy-inherent.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args privacy/associated-item-privacy-inherent.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/privacy/associated-item-privacy-inherent.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/associated-item-privacy-inherent" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/associated-item-privacy-inherent/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: type `for<'r> fn(&'r priv_nominal::Pub) {priv_nominal::Pub::method}` is private
   |
   |
LL |         let value = Pub::method;
...
...
LL |     priv_nominal::mac!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: type `for<'r> fn(&'r priv_nominal::Pub) {priv_nominal::Pub::method}` is private
   |
LL |         value;
   |         ^^^^^ private type
...
...
LL |     priv_nominal::mac!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: type `for<'r> fn(&'r priv_nominal::Pub) {priv_nominal::Pub::method}` is private
   |
   |
LL |         Pub.method();
...
...
LL |     priv_nominal::mac!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: associated constant `CONST` is private
error: associated constant `CONST` is private
  --> /checkout/src/test/ui/privacy/associated-item-privacy-inherent.rs:19:9
   |
LL |         Pub::CONST;
   |         ^^^^^^^^^^ private associated constant
...
LL |     priv_nominal::mac!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: type `priv_signature::Priv` is private
   |
   |
LL |         let value = Pub::method;
...
...
LL |     priv_signature::mac!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: type `priv_signature::Priv` is private
   |
LL |         value;
   |         ^^^^^ private type
...
...
LL |     priv_signature::mac!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: type `priv_signature::Priv` is private
   |
   |
LL |         Pub.method(loop {});
...
...
LL |     priv_signature::mac!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: type `priv_substs::Priv` is private
   |
   |
LL |         let value = Pub::method::<Priv>;
...
...
LL |     priv_substs::mac!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: type `priv_substs::Priv` is private
   |
LL |         value;
   |         ^^^^^ private type
...
...
LL |     priv_substs::mac!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: type `priv_substs::Priv` is private
   |
   |
LL |         Pub.method::<Priv>();
...
...
LL |     priv_substs::mac!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: type `priv_parent_substs::Priv` is private
   |
   |
LL |         let value = <Pub>::method;
...
...
LL |     priv_parent_substs::mac!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: type `priv_parent_substs::Priv` is private
   |
LL |         value;
   |         ^^^^^ private type
...
...
LL |     priv_parent_substs::mac!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: type `priv_parent_substs::Priv` is private
   |
   |
LL |         let value = Pub::method;
...
...
LL |     priv_parent_substs::mac!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: type `priv_parent_substs::Priv` is private
   |
LL |         value;
   |         ^^^^^ private type
...
...
LL |     priv_parent_substs::mac!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: type `priv_parent_substs::Priv` is private
   |
   |
LL |         let value = <Pub>::static_method;
...
...
LL |     priv_parent_substs::mac!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: type `priv_parent_substs::Priv` is private
   |
LL |         value;
   |         ^^^^^ private type
...
...
LL |     priv_parent_substs::mac!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: type `priv_parent_substs::Priv` is private
   |
   |
LL |         let value = Pub::static_method;
...
...
LL |     priv_parent_substs::mac!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: type `priv_parent_substs::Priv` is private
   |
LL |         value;
   |         ^^^^^ private type
...
...
LL |     priv_parent_substs::mac!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: type `priv_parent_substs::Priv` is private
   |
   |
LL |         Pub(Priv).method();
...
...
LL |     priv_parent_substs::mac!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: type `priv_parent_substs::Priv` is private
   |
   |
LL |         <Pub>::CONST;
...
...
LL |     priv_parent_substs::mac!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: type `priv_parent_substs::Priv` is private
   |
   |
LL |         Pub::CONST;
...
...
LL |     priv_parent_substs::mac!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to 21 previous errors
---

---- [ui] ui/privacy/associated-item-privacy-trait.rs stdout ----
diff of stderr:

- error: function `for<'r> fn(&'r priv_trait::Pub) {<priv_trait::Pub as PrivTr>::method}` is private
+ error: type `for<'r> fn(&'r priv_trait::Pub) {<priv_trait::Pub as PrivTr>::method}` is private
3    |
3    |
4 LL |         let value = <Pub as PrivTr>::method;
-    |                     ^^^^^^^^^^^^^^^^^^^^^^^ private function
+    |                     ^^^^^^^^^^^^^^^^^^^^^^^ private type
6 ...
6 ...
7 LL |     priv_trait::mac!();

9    |
10    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
11 
11 
- error: function `for<'r> fn(&'r priv_trait::Pub) {<priv_trait::Pub as PrivTr>::method}` is private
+ error: type `for<'r> fn(&'r priv_trait::Pub) {<priv_trait::Pub as PrivTr>::method}` is private
14    |
15 LL |         value;

-    |         ^^^^^ private function
-    |         ^^^^^ private function
+    |         ^^^^^ private type
17 ...
18 LL |     priv_trait::mac!();

20    |
21    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
22 
22 
- error: function `for<'r> fn(&'r Self) {<Self as PrivTr>::method}` is private
+ error: type `for<'r> fn(&'r Self) {<Self as PrivTr>::method}` is private
25    |
25    |
26 LL |         Pub.method();
-    |             ^^^^^^ private function
+    |             ^^^^^^ private type
28 ...
28 ...
29 LL |     priv_trait::mac!();


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/associated-item-privacy-trait/associated-item-privacy-trait.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/associated-item-privacy-trait/associated-item-privacy-trait.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args privacy/associated-item-privacy-trait.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/privacy/associated-item-privacy-trait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/associated-item-privacy-trait" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/associated-item-privacy-trait/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: type `for<'r> fn(&'r priv_trait::Pub) {<priv_trait::Pub as PrivTr>::method}` is private
   |
   |
LL |         let value = <Pub as PrivTr>::method;
...
...
LL |     priv_trait::mac!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: type `for<'r> fn(&'r priv_trait::Pub) {<priv_trait::Pub as PrivTr>::method}` is private
   |
LL |         value;
   |         ^^^^^ private type
...
...
LL |     priv_trait::mac!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: type `for<'r> fn(&'r Self) {<Self as PrivTr>::method}` is private
   |
   |
LL |         Pub.method();
...
...
LL |     priv_trait::mac!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: associated constant `<Pub as PrivTr>::CONST` is private
   |
   |
LL |         <Pub as PrivTr>::CONST;
   |         ^^^^^^^^^^^^^^^^^^^^^^ private associated constant
...
LL |     priv_trait::mac!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: associated type `<Pub as PrivTr>::AssocTy` is private
   |
   |
LL |         let _: <Pub as PrivTr>::AssocTy;
   |                ^^^^^^^^^^^^^^^^^^^^^^^^ private associated type
...
LL |     priv_trait::mac!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: trait `PrivTr` is private
   |
   |
LL |         pub type InSignatureTy = <Pub as PrivTr>::AssocTy;
   |                                  ^^^^^^^^^^^^^^^^^^^^^^^^ private trait
...
LL |     priv_trait::mac!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: trait `PrivTr` is private
   |
   |
LL |         pub trait InSignatureTr: PrivTr {}
   |                                  ^^^^^^ private trait
...
LL |     priv_trait::mac!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: trait `PrivTr` is private
   |
   |
LL |         impl PrivTr for u8 {}
   |              ^^^^^^ private trait
...
LL |     priv_trait::mac!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: type `priv_signature::Priv` is private
   |
   |
LL |         let value = <Pub as PubTr>::method;
...
...
LL |     priv_signature::mac!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: type `priv_signature::Priv` is private
   |
LL |         value;
   |         ^^^^^ private type
...
...
LL |     priv_signature::mac!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: type `priv_signature::Priv` is private
   |
   |
LL |         Pub.method(loop {});
...
...
LL |     priv_signature::mac!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: type `priv_substs::Priv` is private
   |
   |
LL |         let value = <Pub as PubTr>::method::<Priv>;
...
...
LL |     priv_substs::mac!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: type `priv_substs::Priv` is private
   |
LL |         value;
   |         ^^^^^ private type
...
...
LL |     priv_substs::mac!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: type `priv_substs::Priv` is private
   |
   |
LL |         Pub.method::<Priv>();
...
...
LL |     priv_substs::mac!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: type `priv_parent_substs::Priv` is private
   |
   |
LL |         let value = <Pub as PubTr>::method;
...
...
LL |     priv_parent_substs::mac!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: type `priv_parent_substs::Priv` is private
   |
LL |         value;
   |         ^^^^^ private type
...
...
LL |     priv_parent_substs::mac!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: type `priv_parent_substs::Priv` is private
   |
   |
LL |         let value = <Pub as PubTr<_>>::method;
...
...
LL |     priv_parent_substs::mac!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: type `priv_parent_substs::Priv` is private
   |
LL |         value;
   |         ^^^^^ private type
...
...
LL |     priv_parent_substs::mac!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: type `priv_parent_substs::Priv` is private
   |
   |
LL |         Pub.method();
...
...
LL |     priv_parent_substs::mac!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: type `priv_parent_substs::Priv` is private
   |
   |
LL |         let value = <Priv as PubTr<_>>::method;
...
...
LL |     priv_parent_substs::mac!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: type `priv_parent_substs::Priv` is private
   |
LL |         value;
   |         ^^^^^ private type
...
...
LL |     priv_parent_substs::mac!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: type `priv_parent_substs::Priv` is private
   |
   |
LL |         Priv.method();
...
...
LL |     priv_parent_substs::mac!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: type `priv_parent_substs::Priv` is private
   |
   |
LL |         <Pub as PubTr>::CONST;
...
...
LL |     priv_parent_substs::mac!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: type `priv_parent_substs::Priv` is private
   |
   |
LL |         <Pub as PubTr<_>>::CONST;
...
...
LL |     priv_parent_substs::mac!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: type `priv_parent_substs::Priv` is private
   |
   |
LL |         <Priv as PubTr<_>>::CONST;
...
...
LL |     priv_parent_substs::mac!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: type `priv_parent_substs::Priv` is private
   |
   |
LL |         let _: <Pub as PubTr<_>>::AssocTy;
   |                              ^ private type
...
LL |     priv_parent_substs::mac!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: type `priv_parent_substs::Priv` is private
   |
   |
LL |         let _: <Priv as PubTr<_>>::AssocTy;
...
...
LL |     priv_parent_substs::mac!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: type `priv_parent_substs::Priv` is private
   |
   |
LL |         pub type InSignatureTy1 = <Pub as PubTr>::AssocTy;
...
...
LL |     priv_parent_substs::mac!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: type `priv_parent_substs::Priv` is private
   |
   |
LL |         pub type InSignatureTy2 = <Priv as PubTr<Pub>>::AssocTy;
...
...
LL |     priv_parent_substs::mac!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: type `priv_parent_substs::Priv` is private
   |
   |
LL |         impl PubTr for u8 {}
...
...
LL |     priv_parent_substs::mac!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to 30 previous errors
---

---- [ui] ui/privacy/private-inferred-type.rs stdout ----
diff of stderr:

106 LL |     adjust::S1.method_s3();
108 
108 
- error: function `fn() {priv_fn}` is private
+ error: type `fn() {priv_fn}` is private
111    |
112 LL |         priv_fn;

-    |         ^^^^^^^ private function
-    |         ^^^^^^^ private function
+    |         ^^^^^^^ private type
114 ...
115 LL |     m::m!();

128    |
129    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
130 
130 
- error: function `fn() {<u8 as PrivTrait>::method}` is private
+ error: type `fn() {<u8 as PrivTrait>::method}` is private
133    |
133    |
134 LL |         <u8 as PrivTrait>::method;
-    |         ^^^^^^^^^^^^^^^^^^^^^^^^^ private function
+    |         ^^^^^^^^^^^^^^^^^^^^^^^^^ private type
136 ...
136 ...
137 LL |     m::m!();

139    |
140    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
141 
141 
- error: function `fn(u8) -> PrivTupleStruct {PrivTupleStruct}` is private
+ error: type `fn(u8) -> PrivTupleStruct {PrivTupleStruct}` is private
144    |
144    |
145 LL |         PrivTupleStruct;
-    |         ^^^^^^^^^^^^^^^ private function
+    |         ^^^^^^^^^^^^^^^ private type
147 ...
147 ...
148 LL |     m::m!();

150    |
151    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
152 
152 
- error: function `fn(u8) -> PubTupleStruct {PubTupleStruct}` is private
+ error: type `fn(u8) -> PubTupleStruct {PubTupleStruct}` is private
155    |
155    |
156 LL |         PubTupleStruct;
-    |         ^^^^^^^^^^^^^^ private function
+    |         ^^^^^^^^^^^^^^ private type
158 ...
158 ...
159 LL |     m::m!();

161    |
162    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
163 
163 
- error: function `for<'r> fn(&'r Pub<u8>) {Pub::<u8>::priv_method}` is private
+ error: type `for<'r> fn(&'r Pub<u8>) {Pub::<u8>::priv_method}` is private
166    |
166    |
167 LL |         Pub(0u8).priv_method();
-    |                  ^^^^^^^^^^^ private function
+    |                  ^^^^^^^^^^^ private type
169 ...
169 ...
170 LL |     m::m!();


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/private-inferred-type/private-inferred-type.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/private-inferred-type/private-inferred-type.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args privacy/private-inferred-type.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/privacy/private-inferred-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/private-inferred-type" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/private-inferred-type/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0446]: private type `Priv` in public interface
   |
LL |     struct Priv;
LL |     struct Priv;
   |     ------------ `Priv` declared as private
...
LL |     impl TraitWithAssocTy for u8 { type AssocTy = Priv; }
   |                                    ^^^^^^^^^^^^^^^^^^^^ can't leak private type

error[E0446]: private type `S2` in public interface
   |
LL |     struct S2;
LL |     struct S2;
   |     ---------- `S2` declared as private
...
LL |         type Target = S2Alias; //~ ERROR private type `S2` in public interface
   |         ^^^^^^^^^^^^^^^^^^^^^^ can't leak private type

error: type `Priv` is private
   |
   |
LL |     let _: m::Alias; //~ ERROR type `Priv` is private
   |         ^ private type

error: type `Priv` is private
   |
   |
LL |     let _: m::Alias; //~ ERROR type `Priv` is private


error: type `Priv` is private
   |
   |
LL |     let _: <m::Alias as m::TraitWithAssocTy>::AssocTy; //~ ERROR type `Priv` is private


error: type `Priv` is private
   |
   |
LL |     m::Alias {}; //~ ERROR type `Priv` is private


error: type `Priv` is private
   |
   |
LL |     m::Pub { 0: m::Alias {} }; //~ ERROR type `Priv` is private


error: type `Priv` is private
   |
   |
LL |     m::Pub::static_method; //~ ERROR type `Priv` is private


error: type `Priv` is private
   |
   |
LL |     m::Pub::INHERENT_ASSOC_CONST; //~ ERROR type `Priv` is private


error: type `Priv` is private
   |
   |
LL |     m::Pub(0u8).method_with_substs::<m::Alias>(); //~ ERROR type `Priv` is private


error: type `Priv` is private
   |
   |
LL |     m::Pub(0u8).method_with_priv_params(loop{}); //~ ERROR type `Priv` is private


error: type `Priv` is private
   |
   |
LL |     <m::Alias as m::TraitWithAssocConst>::TRAIT_ASSOC_CONST; //~ ERROR type `Priv` is private


error: type `Priv` is private
   |
   |
LL |     <m::Pub<m::Alias>>::INHERENT_ASSOC_CONST; //~ ERROR type `Priv` is private


error: type `Priv` is private
   |
   |
LL |     <m::Pub<m::Alias>>::INHERENT_ASSOC_CONST_GENERIC_SELF; //~ ERROR type `Priv` is private


error: type `Priv` is private
   |
   |
LL |     <m::Pub<m::Alias>>::static_method_generic_self; //~ ERROR type `Priv` is private


error: type `Priv` is private
   |
   |
LL |     u8::pub_method; //~ ERROR type `Priv` is private

error: type `S2` is private
  --> /checkout/src/test/ui/privacy/private-inferred-type.rs:114:5
   |
   |
LL |     adjust::S1.method_s3(); //~ ERROR type `S2` is private


error: type `fn() {priv_fn}` is private
   |
   |
LL |         priv_fn; //~ ERROR function `fn() {priv_fn}` is private
...
...
LL |     m::m!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: type `PrivEnum` is private
   |
   |
LL |         PrivEnum::Variant; //~ ERROR type `PrivEnum` is private
...
...
LL |     m::m!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: type `fn() {<u8 as PrivTrait>::method}` is private
   |
   |
LL |         <u8 as PrivTrait>::method; //~ ERROR function `fn() {<u8 as PrivTrait>::method}` is private
...
...
LL |     m::m!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: type `fn(u8) -> PrivTupleStruct {PrivTupleStruct}` is private
   |
   |
LL |         PrivTupleStruct;
...
...
LL |     m::m!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: type `fn(u8) -> PubTupleStruct {PubTupleStruct}` is private
   |
   |
LL |         PubTupleStruct;
...
...
LL |     m::m!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: type `for<'r> fn(&'r Pub<u8>) {Pub::<u8>::priv_method}` is private
   |
   |
LL |         Pub(0u8).priv_method();
...
...
LL |     m::m!();
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: trait `Trait` is private
   |
   |
LL |     m::leak_anon1(); //~ ERROR trait `Trait` is private
   |     ^^^^^^^^^^^^^^^ private trait

error: type `Priv` is private
   |
   |
LL |     m::leak_anon2(); //~ ERROR type `Priv` is private


error: type `Priv` is private
   |
   |
LL |     m::leak_anon3(); //~ ERROR type `Priv` is private


error: trait `Trait` is private
   |
   |
LL |     m::leak_dyn1(); //~ ERROR trait `Trait` is private
   |     ^^^^^^^^^^^^^^ private trait

error: type `Priv` is private
   |
   |
LL |     m::leak_dyn2(); //~ ERROR type `Priv` is private


error: type `Priv` is private
   |
   |
LL |     m::leak_dyn3(); //~ ERROR type `Priv` is private


error: type `Priv` is private
   |
   |
LL |     let a = m::Alias {}; //~ ERROR type `Priv` is private


error: type `Priv` is private
   |
   |
LL |     let mut b = a; //~ ERROR type `Priv` is private
   |                 ^ private type

error: type `Priv` is private
   |
   |
LL |     b = a; //~ ERROR type `Priv` is private
   |         ^ private type

error: type `Priv` is private
   |
   |
LL |     match a { //~ ERROR type `Priv` is private
   |           ^ private type
error: aborting due to 33 previous errors

For more information about this error, try `rustc --explain E0446`.


------------------------------------------


---- [ui] ui/privacy/private-inferred-type-3.rs stdout ----
diff of stderr:

- error: function `fn() {ext::priv_fn}` is private
+ error: type `fn() {ext::priv_fn}` is private
3    |
4 LL |     ext::m!();

-    |     ^^^^^^^^^^ private function
---

22    |
23    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
24 
- error: function `fn() {<u8 as ext::PrivTrait>::method}` is private
+ error: type `fn() {<u8 as ext::PrivTrait>::method}` is private
27    |
28 LL |     ext::m!();

-    |     ^^^^^^^^^^ private function
-    |     ^^^^^^^^^^ private function
+    |     ^^^^^^^^^^ private type
30    |
31    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
32 

- error: function `fn(u8) -> ext::PrivTupleStruct {ext::PrivTupleStruct}` is private
+ error: type `fn(u8) -> ext::PrivTupleStruct {ext::PrivTupleStruct}` is private
35    |
36 LL |     ext::m!();

-    |     ^^^^^^^^^^ private function
-    |     ^^^^^^^^^^ private function
+    |     ^^^^^^^^^^ private type
38    |
39    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
40 

- error: function `fn(u8) -> PubTupleStruct {PubTupleStruct}` is private
+ error: type `fn(u8) -> PubTupleStruct {PubTupleStruct}` is private
43    |
44 LL |     ext::m!();

-    |     ^^^^^^^^^^ private function
-    |     ^^^^^^^^^^ private function
+    |     ^^^^^^^^^^ private type
46    |
47    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
48 

- error: function `for<'r> fn(&'r Pub<u8>) {Pub::<u8>::priv_method}` is private
+ error: type `for<'r> fn(&'r Pub<u8>) {Pub::<u8>::priv_method}` is private
51    |
52 LL |     ext::m!();

-    |     ^^^^^^^^^^ private function
-    |     ^^^^^^^^^^ private function
+    |     ^^^^^^^^^^ private type
54    |
55    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
56 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/private-inferred-type-3/private-inferred-type-3.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args privacy/private-inferred-type-3.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/privacy/private-inferred-type-3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/private-inferred-type-3" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/private-inferred-type-3/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: type `fn() {ext::priv_fn}` is private
   |
LL |     ext::m!();
   |     ^^^^^^^^^^ private type
   |
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: static `PRIV_STATIC` is private
   |
LL |     ext::m!();
   |     ^^^^^^^^^^ private static
   |
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: type `ext::PrivEnum` is private
   |
LL |     ext::m!();
   |     ^^^^^^^^^^ private type
   |
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: type `fn() {<u8 as ext::PrivTrait>::method}` is private
   |
LL |     ext::m!();
   |     ^^^^^^^^^^ private type
   |
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: type `fn(u8) -> ext::PrivTupleStruct {ext::PrivTupleStruct}` is private
   |
LL |     ext::m!();
   |     ^^^^^^^^^^ private type
   |
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: type `fn(u8) -> PubTupleStruct {PubTupleStruct}` is private
   |
LL |     ext::m!();
   |     ^^^^^^^^^^ private type
   |
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: type `for<'r> fn(&'r Pub<u8>) {Pub::<u8>::priv_method}` is private
   |
LL |     ext::m!();
   |     ^^^^^^^^^^ private type
   |
---
test result: FAILED. 11211 passed; 7 failed; 87 ignored; 0 measured; 0 filtered out; finished in 134.77s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:16:28
