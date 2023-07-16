plain
.................................................................................................... 1400/11813
................................iiii.ii.i.............i............................................. 1500/11813
.................................................................................................... 1600/11813
................................................i................................................... 1700/11813
..............................................FF.................................................... 1800/11813
...........................................................................F..FF.F..i........F..F... 1900/11813
.................................................................................................... 2100/11813
.................................................................................................... 2200/11813
.................................................................................................... 2300/11813
.................................................................................................... 2400/11813
.................................................................................................... 2400/11813
.................................................................................................... 2500/11813
.................................................................................................... 2600/11813
.................................................................................................... 2700/11813
.i..i............................................................................................... 2800/11813
.................................................................................................... 2900/11813
.............iiiii.................................................................................. 3000/11813
..........................................................................................F......... 3100/11813
.........F...F...................................................................................... 3200/11813
....................................................F............................................... 3300/11813
.................................................................................................... 3500/11813
.................................................................................................... 3600/11813
.................................................................................................... 3700/11813
...............................i.................................................................... 3800/11813
---
.................................................................................................... 5200/11813
...................................................................i...i............................ 5300/11813
.................................................................................................... 5400/11813
.................................................................................................... 5500/11813
..................................................................F..F.............................. 5600/11813
...........F........................................................................................ 5700/11813
..............................................................................i..................... 5900/11813
.................................................................................................... 6000/11813
...................................................................................i................ 6100/11813
........................................................i........................................... 6200/11813
---
.................................................................................................... 9400/11813
.................................................................................................... 9500/11813
.................................................................................................... 9600/11813
...................................i......i......................................................... 9700/11813
.................................................................................iiiiiii..iiiiii.i.. 9800/11813
.................................................................................................F.. 9900/11813
.................................................................................................... 10100/11813
.................................................................................................... 10200/11813
.................................................................................................... 10300/11813
.................................................................................................... 10400/11813
---

---- [ui] ui/const-generics/const-param-type-depends-on-type-param.rs#full stdout ----
diff of stderr:

10 LL | pub struct Dependent<T, const X: T>([(); X]);
11    |                      ^ unused parameter
12    |
+ help: if you intended T to be a const parameter, use const T: usize instead
+    |
+    |
+ LL | pub struct Dependent<T, const X: T>([(); X]);
+    |                      ^
13    = help: consider removing `T`, referring to it in a field, or using a marker such as `PhantomData`
15 error: aborting due to 2 previous errors


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const-param-type-depends-on-type-param.full/const-param-type-depends-on-type-param.full.stderr
To only update this specific test, also pass `--test-args const-generics/const-param-type-depends-on-type-param.rs`


error in revision `full`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/const-param-type-depends-on-type-param.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "full" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const-param-type-depends-on-type-param.full" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const-param-type-depends-on-type-param.full/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0770]: the type of const parameters must not depend on other generic parameters
   |
   |
LL | pub struct Dependent<T, const X: T>([(); X]);
   |                                  ^ the type must not depend on the parameter `T`
error[E0392]: parameter `T` is never used
  --> /checkout/src/test/ui/const-generics/const-param-type-depends-on-type-param.rs:11:22
   |
   |
LL | pub struct Dependent<T, const X: T>([(); X]);
   |                      ^ unused parameter
   |
help: if you intended T to be a const parameter, use const T: usize instead
   |
   |
LL | pub struct Dependent<T, const X: T>([(); X]);
   |                      ^
   = help: consider removing `T`, referring to it in a field, or using a marker such as `PhantomData`
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0392, E0770.
For more information about an error, try `rustc --explain E0392`.
For more information about an error, try `rustc --explain E0392`.

------------------------------------------


---- [ui] ui/const-generics/const-param-type-depends-on-type-param.rs#min stdout ----
diff of stderr:

10 LL | pub struct Dependent<T, const X: T>([(); X]);
11    |                      ^ unused parameter
12    |
+ help: if you intended T to be a const parameter, use const T: usize instead
+    |
+    |
+ LL | pub struct Dependent<T, const X: T>([(); X]);
+    |                      ^
13    = help: consider removing `T`, referring to it in a field, or using a marker such as `PhantomData`
15 error: aborting due to 2 previous errors


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const-param-type-depends-on-type-param.min/const-param-type-depends-on-type-param.min.stderr
To only update this specific test, also pass `--test-args const-generics/const-param-type-depends-on-type-param.rs`


error in revision `min`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/const-param-type-depends-on-type-param.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "min" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const-param-type-depends-on-type-param.min" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const-param-type-depends-on-type-param.min/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
------------------------------------------
error[E0770]: the type of const parameters must not depend on other generic parameters
   |
   |
LL | pub struct Dependent<T, const X: T>([(); X]);
   |                                  ^ the type must not depend on the parameter `T`
error[E0392]: parameter `T` is never used
  --> /checkout/src/test/ui/const-generics/const-param-type-depends-on-type-param.rs:11:22
   |
   |
LL | pub struct Dependent<T, const X: T>([(); X]);
   |                      ^ unused parameter
   |
help: if you intended T to be a const parameter, use const T: usize instead
   |
   |
LL | pub struct Dependent<T, const X: T>([(); X]);
   |                      ^
   = help: consider removing `T`, referring to it in a field, or using a marker such as `PhantomData`
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0392, E0770.
For more information about an error, try `rustc --explain E0392`.
For more information about an error, try `rustc --explain E0392`.

------------------------------------------


---- [ui] ui/const-generics/issue-67375.rs#full stdout ----
diff of stderr:

14 LL | struct Bug<T> {
15    |            ^ unused parameter
16    |
+ help: if you intended T to be a const parameter, use const T: usize instead
+    |
+    |
+ LL | struct Bug<T> {
+    |            ^
17    = help: consider removing `T`, referring to it in a field, or using a marker such as `PhantomData`
19 error: aborting due to previous error; 1 warning emitted


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issue-67375.full/issue-67375.full.stderr
To only update this specific test, also pass `--test-args const-generics/issue-67375.rs`


error in revision `full`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issue-67375.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "full" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issue-67375.full" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issue-67375.full/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: cannot use constants which depend on generic parameters in types
   |
   |
LL |     inner: [(); { [|_: &T| {}; 0].len() }],
   |
   = note: `#[warn(const_evaluatable_unchecked)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #76200 <https://github.com/rust-lang/rust/issues/76200>
   = note: for more information, see issue #76200 <https://github.com/rust-lang/rust/issues/76200>

error[E0392]: parameter `T` is never used
  --> /checkout/src/test/ui/const-generics/issue-67375.rs:6:12
   |
LL | struct Bug<T> {
   |            ^ unused parameter
   |
help: if you intended T to be a const parameter, use const T: usize instead
   |
   |
LL | struct Bug<T> {
   |            ^
   = help: consider removing `T`, referring to it in a field, or using a marker such as `PhantomData`
error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0392`.


------------------------------------------


---- [ui] ui/const-generics/issue-67375.rs#min stdout ----
diff of stderr:

13 LL | struct Bug<T> {
14    |            ^ unused parameter
15    |
+ help: if you intended T to be a const parameter, use const T: usize instead
+    |
+    |
+ LL | struct Bug<T> {
+    |            ^
16    = help: consider removing `T`, referring to it in a field, or using a marker such as `PhantomData`
18 error: aborting due to 2 previous errors


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issue-67375.min/issue-67375.min.stderr
To only update this specific test, also pass `--test-args const-generics/issue-67375.rs`


error in revision `min`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issue-67375.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "min" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issue-67375.min" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issue-67375.min/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: generic parameters may not be used in const operations
   |
   |
LL |     inner: [(); { [|_: &T| {}; 0].len() }],
   |                         ^ cannot perform const operation using `T`
   = note: type parameters may not be used in const expressions
   = note: type parameters may not be used in const expressions
   = help: use `#![feature(const_generics)]` and `#![feature(const_evaluatable_checked)]` to allow generic const expressions
error[E0392]: parameter `T` is never used
  --> /checkout/src/test/ui/const-generics/issue-67375.rs:6:12
   |
   |
LL | struct Bug<T> {
   |            ^ unused parameter
   |
help: if you intended T to be a const parameter, use const T: usize instead
   |
   |
LL | struct Bug<T> {
   |            ^
   = help: consider removing `T`, referring to it in a field, or using a marker such as `PhantomData`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0392`.


------------------------------------------


---- [ui] ui/const-generics/issue-67945-1.rs#min stdout ----
diff of stderr:

22 LL | struct Bug<S> {
23    |            ^ unused parameter
24    |
+ help: if you intended S to be a const parameter, use const S: usize instead
+    |
+    |
+ LL | struct Bug<S> {
+    |            ^
25    = help: consider removing `S`, referring to it in a field, or using a marker such as `PhantomData`
27 error: aborting due to 3 previous errors


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issue-67945-1.min/issue-67945-1.min.stderr
To only update this specific test, also pass `--test-args const-generics/issue-67945-1.rs`


error in revision `min`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issue-67945-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "min" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issue-67945-1.min" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issue-67945-1.min/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: generic parameters may not be used in const operations
   |
   |
LL |         let x: S = MaybeUninit::uninit();
   |                ^ cannot perform const operation using `S`
   = note: type parameters may not be used in const expressions
   = note: type parameters may not be used in const expressions
   = help: use `#![feature(const_generics)]` and `#![feature(const_evaluatable_checked)]` to allow generic const expressions

error: generic parameters may not be used in const operations
   |
   |
LL |         let b = &*(&x as *const _ as *const S);
   |                                             ^ cannot perform const operation using `S`
   = note: type parameters may not be used in const expressions
   = note: type parameters may not be used in const expressions
   = help: use `#![feature(const_generics)]` and `#![feature(const_evaluatable_checked)]` to allow generic const expressions
error[E0392]: parameter `S` is never used
  --> /checkout/src/test/ui/const-generics/issue-67945-1.rs:10:12
   |
   |
LL | struct Bug<S> {
   |            ^ unused parameter
   |
help: if you intended S to be a const parameter, use const S: usize instead
   |
   |
LL | struct Bug<S> {
   |            ^
   = help: consider removing `S`, referring to it in a field, or using a marker such as `PhantomData`
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0392`.


------------------------------------------


---- [ui] ui/const-generics/issue-67945-2.rs#min stdout ----
diff of stderr:

22 LL | struct Bug<S> {
23    |            ^ unused parameter
24    |
+ help: if you intended S to be a const parameter, use const S: usize instead
+    |
+    |
+ LL | struct Bug<S> {
+    |            ^
25    = help: consider removing `S`, referring to it in a field, or using a marker such as `PhantomData`
27 error: aborting due to 3 previous errors


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issue-67945-2.min/issue-67945-2.min.stderr
To only update this specific test, also pass `--test-args const-generics/issue-67945-2.rs`


error in revision `min`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issue-67945-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "min" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issue-67945-2.min" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issue-67945-2.min/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: generic parameters may not be used in const operations
   |
   |
LL |         let x: S = MaybeUninit::uninit();
   |                ^ cannot perform const operation using `S`
   = note: type parameters may not be used in const expressions
   = note: type parameters may not be used in const expressions
   = help: use `#![feature(const_generics)]` and `#![feature(const_evaluatable_checked)]` to allow generic const expressions

error: generic parameters may not be used in const operations
   |
   |
LL |         let b = &*(&x as *const _ as *const S);
   |                                             ^ cannot perform const operation using `S`
   = note: type parameters may not be used in const expressions
   = note: type parameters may not be used in const expressions
   = help: use `#![feature(const_generics)]` and `#![feature(const_evaluatable_checked)]` to allow generic const expressions
error[E0392]: parameter `S` is never used
  --> /checkout/src/test/ui/const-generics/issue-67945-2.rs:8:12
   |
   |
LL | struct Bug<S> {
   |            ^ unused parameter
   |
help: if you intended S to be a const parameter, use const S: usize instead
   |
   |
LL | struct Bug<S> {
   |            ^
   = help: consider removing `S`, referring to it in a field, or using a marker such as `PhantomData`
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0392`.


------------------------------------------


---- [ui] ui/const-generics/issue-67945-2.rs#full stdout ----
diff of stderr:

18 LL | struct Bug<S> {
19    |            ^ unused parameter
20    |
+ help: if you intended S to be a const parameter, use const S: usize instead
+    |
+    |
+ LL | struct Bug<S> {
+    |            ^
21    = help: consider removing `S`, referring to it in a field, or using a marker such as `PhantomData`
23 error: aborting due to 2 previous errors


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issue-67945-2.full/issue-67945-2.full.stderr
To only update this specific test, also pass `--test-args const-generics/issue-67945-2.rs`


error in revision `full`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issue-67945-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "full" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issue-67945-2.full" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issue-67945-2.full/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/issue-67945-2.rs:11:20
   |
LL | struct Bug<S> {
   |            - this type parameter
...
LL |         let x: S = MaybeUninit::uninit();
   |                -   ^^^^^^^^^^^^^^^^^^^^^ expected type parameter `S`, found union `MaybeUninit`
   |                expected due to this
   |
   = note: expected type parameter `S`
   = note: expected type parameter `S`
                       found union `MaybeUninit<_>`
error[E0392]: parameter `S` is never used
  --> /checkout/src/test/ui/const-generics/issue-67945-2.rs:8:12
   |
   |
LL | struct Bug<S> {
   |            ^ unused parameter
   |
help: if you intended S to be a const parameter, use const S: usize instead
   |
   |
LL | struct Bug<S> {
   |            ^
   = help: consider removing `S`, referring to it in a field, or using a marker such as `PhantomData`
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0308, E0392.
For more information about an error, try `rustc --explain E0308`.
For more information about an error, try `rustc --explain E0308`.

------------------------------------------


---- [ui] ui/const-generics/issue-67945-1.rs#full stdout ----
diff of stderr:

18 LL | struct Bug<S> {
19    |            ^ unused parameter
20    |
+ help: if you intended S to be a const parameter, use const S: usize instead
+    |
+    |
+ LL | struct Bug<S> {
+    |            ^
21    = help: consider removing `S`, referring to it in a field, or using a marker such as `PhantomData`
23 error: aborting due to 2 previous errors


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issue-67945-1.full/issue-67945-1.full.stderr
To only update this specific test, also pass `--test-args const-generics/issue-67945-1.rs`


error in revision `full`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issue-67945-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "full" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issue-67945-1.full" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issue-67945-1.full/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/issue-67945-1.rs:13:20
   |
LL | struct Bug<S> {
   |            - this type parameter
...
LL |         let x: S = MaybeUninit::uninit();
   |                -   ^^^^^^^^^^^^^^^^^^^^^ expected type parameter `S`, found union `MaybeUninit`
   |                expected due to this
   |
   = note: expected type parameter `S`
   = note: expected type parameter `S`
                       found union `MaybeUninit<_>`
error[E0392]: parameter `S` is never used
  --> /checkout/src/test/ui/const-generics/issue-67945-1.rs:10:12
   |
   |
LL | struct Bug<S> {
   |            ^ unused parameter
   |
help: if you intended S to be a const parameter, use const S: usize instead
   |
   |
LL | struct Bug<S> {
   |            ^
   = help: consider removing `S`, referring to it in a field, or using a marker such as `PhantomData`
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0308, E0392.
For more information about an error, try `rustc --explain E0308`.
For more information about an error, try `rustc --explain E0308`.

------------------------------------------

---

error: expected identifier, found keyword `Self`
  --> /checkout/src/test/ui/self/self_type_keyword.rs:14:13
   |
LL |         ref Self => (),


error: `mut` must be followed by a named binding
   |
   |
LL |         mut Self => (),
   |         ^^^^^^^^ help: remove the `mut` prefix: `Self`
   |
   = note: `mut` may be followed by `variable` and `variable @ pattern`
error: expected identifier, found keyword `Self`
  --> /checkout/src/test/ui/self/self_type_keyword.rs:19:17
   |
   |
LL |         ref mut Self => (),

error: expected identifier, found keyword `Self`
  --> /checkout/src/test/ui/self/self_type_keyword.rs:23:15
   |
   |
LL |         Foo { Self } => (),

error: expected identifier, found keyword `Self`
  --> /checkout/src/test/ui/self/self_type_keyword.rs:29:26
   |
---

error: lifetimes cannot use keyword names
  --> /checkout/src/test/ui/self/self_type_keyword.rs:6:12
   |
LL | struct Bar<'Self>;


error: cannot find macro `Self` in this scope
   |
   |
LL |         Self!() => (),


error[E0531]: cannot find unit struct, unit variant or constant `Self` in this scope
   |
   |
LL |         mut Self => (),
   |
help: consider importing this unit struct
   |
LL | use foo::Self;
LL | use foo::Self;
   |

error[E0392]: parameter `'Self` is never used
   |
   |
LL | struct Bar<'Self>;
   |            ^^^^^ unused parameter
   |
help: if you intended 'Self to be a const parameter, use const 'Self: usize instead
   |
   |
LL | struct Bar<'Self>;
   |            ^^^^^
   = help: consider removing `'Self`, referring to it in a field, or using a marker such as `PhantomData`
error: aborting due to 12 previous errors

Some errors have detailed explanations: E0392, E0531.
For more information about an error, try `rustc --explain E0392`.
For more information about an error, try `rustc --explain E0392`.

------------------------------------------


---- [ui] ui/variance/variance-regions-unused-direct.rs stdout ----
diff of stderr:

4 LL | struct Bivariant<'a>;
5    |                  ^^ unused parameter
6    |
+ help: if you intended 'a to be a const parameter, use const 'a: usize instead
+    |
+    |
+ LL | struct Bivariant<'a>;
+    |                  ^^
7    = help: consider removing `'a`, referring to it in a field, or using a marker such as `PhantomData`
8 
9 error[E0392]: parameter `'d` is never used

12 LL | struct Struct<'a, 'd> {
13    |                   ^^ unused parameter
14    |
+ help: if you intended 'd to be a const parameter, use const 'd: usize instead
+    |
+    |
+ LL | struct Struct<'a, 'd> {
+    |                   ^^
15    = help: consider removing `'d`, referring to it in a field, or using a marker such as `PhantomData`
17 error: aborting due to 2 previous errors


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-regions-unused-direct/variance-regions-unused-direct.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args variance/variance-regions-unused-direct.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/variance/variance-regions-unused-direct.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-regions-unused-direct" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-regions-unused-direct/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0392]: parameter `'a` is never used
  --> /checkout/src/test/ui/variance/variance-regions-unused-direct.rs:5:18
   |
LL | struct Bivariant<'a>; //~ ERROR parameter `'a` is never used
   |                  ^^ unused parameter
   |
help: if you intended 'a to be a const parameter, use const 'a: usize instead
  --> /checkout/src/test/ui/variance/variance-regions-unused-direct.rs:5:18
   |
LL | struct Bivariant<'a>; //~ ERROR parameter `'a` is never used
   |                  ^^
   = help: consider removing `'a`, referring to it in a field, or using a marker such as `PhantomData`

error[E0392]: parameter `'d` is never used
  --> /checkout/src/test/ui/variance/variance-regions-unused-direct.rs:7:19
   |
LL | struct Struct<'a, 'd> { //~ ERROR parameter `'d` is never used
   |                   ^^ unused parameter
   |
help: if you intended 'd to be a const parameter, use const 'd: usize instead
  --> /checkout/src/test/ui/variance/variance-regions-unused-direct.rs:7:19
   |
LL | struct Struct<'a, 'd> { //~ ERROR parameter `'d` is never used
   |                   ^^
   = help: consider removing `'d`, referring to it in a field, or using a marker such as `PhantomData`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0392`.


------------------------------------------


---- [ui] ui/variance/variance-regions-unused-indirect.rs stdout ----
diff of stderr:

4 LL | enum Foo<'a> {
5    |          ^^ unused parameter
6    |
+ help: if you intended 'a to be a const parameter, use const 'a: usize instead
+    |
+    |
+ LL | enum Foo<'a> {
+    |          ^^
7    = help: consider removing `'a`, referring to it in a field, or using a marker such as `PhantomData`
8 
9 error[E0392]: parameter `'a` is never used

12 LL | enum Bar<'a> {
13    |          ^^ unused parameter
14    |
+ help: if you intended 'a to be a const parameter, use const 'a: usize instead
+    |
+    |
+ LL | enum Bar<'a> {
+    |          ^^
15    = help: consider removing `'a`, referring to it in a field, or using a marker such as `PhantomData`
17 error: aborting due to 2 previous errors


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-regions-unused-indirect/variance-regions-unused-indirect.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args variance/variance-regions-unused-indirect.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/variance/variance-regions-unused-indirect.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-regions-unused-indirect" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-regions-unused-indirect/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0392]: parameter `'a` is never used
  --> /checkout/src/test/ui/variance/variance-regions-unused-indirect.rs:3:10
   |
LL | enum Foo<'a> { //~ ERROR parameter `'a` is never used
   |          ^^ unused parameter
   |
help: if you intended 'a to be a const parameter, use const 'a: usize instead
  --> /checkout/src/test/ui/variance/variance-regions-unused-indirect.rs:3:10
   |
LL | enum Foo<'a> { //~ ERROR parameter `'a` is never used
   |          ^^
   = help: consider removing `'a`, referring to it in a field, or using a marker such as `PhantomData`

error[E0392]: parameter `'a` is never used
  --> /checkout/src/test/ui/variance/variance-regions-unused-indirect.rs:7:10
   |
LL | enum Bar<'a> { //~ ERROR parameter `'a` is never used
   |          ^^ unused parameter
   |
help: if you intended 'a to be a const parameter, use const 'a: usize instead
  --> /checkout/src/test/ui/variance/variance-regions-unused-indirect.rs:7:10
   |
LL | enum Bar<'a> { //~ ERROR parameter `'a` is never used
   |          ^^
   = help: consider removing `'a`, referring to it in a field, or using a marker such as `PhantomData`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0392`.


------------------------------------------


---- [ui] ui/variance/variance-unused-type-param.rs stdout ----
diff of stderr:

4 LL | struct SomeStruct<A> { x: u32 }
5    |                   ^ unused parameter
6    |
+ help: if you intended A to be a const parameter, use const A: usize instead
+    |
+    |
+ LL | struct SomeStruct<A> { x: u32 }
+    |                   ^
7    = help: consider removing `A`, referring to it in a field, or using a marker such as `PhantomData`
8 
9 error[E0392]: parameter `A` is never used

12 LL | enum SomeEnum<A> { Nothing }
13    |               ^ unused parameter
14    |
+ help: if you intended A to be a const parameter, use const A: usize instead
+    |
+    |
+ LL | enum SomeEnum<A> { Nothing }
+    |               ^
15    = help: consider removing `A`, referring to it in a field, or using a marker such as `PhantomData`
17 error[E0392]: parameter `T` is never used


20 LL | enum ListCell<T> {
21    |               ^ unused parameter
22    |
+ help: if you intended T to be a const parameter, use const T: usize instead
+    |
+    |
+ LL | enum ListCell<T> {
+    |               ^
23    = help: consider removing `T`, referring to it in a field, or using a marker such as `PhantomData`
25 error: aborting due to 3 previous errors


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-unused-type-param/variance-unused-type-param.stderr
To only update this specific test, also pass `--test-args variance/variance-unused-type-param.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/variance/variance-unused-type-param.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-unused-type-param" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-unused-type-param/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0392]: parameter `A` is never used
  --> /checkout/src/test/ui/variance/variance-unused-type-param.rs:6:19
   |
LL | struct SomeStruct<A> { x: u32 }
   |                   ^ unused parameter
   |
help: if you intended A to be a const parameter, use const A: usize instead
  --> /checkout/src/test/ui/variance/variance-unused-type-param.rs:6:19
   |
LL | struct SomeStruct<A> { x: u32 }
   |                   ^
   = help: consider removing `A`, referring to it in a field, or using a marker such as `PhantomData`

error[E0392]: parameter `A` is never used
  --> /checkout/src/test/ui/variance/variance-unused-type-param.rs:9:15
   |
LL | enum SomeEnum<A> { Nothing }
   |               ^ unused parameter
   |
help: if you intended A to be a const parameter, use const A: usize instead
  --> /checkout/src/test/ui/variance/variance-unused-type-param.rs:9:15
   |
LL | enum SomeEnum<A> { Nothing }
   |               ^
   = help: consider removing `A`, referring to it in a field, or using a marker such as `PhantomData`
error[E0392]: parameter `T` is never used
  --> /checkout/src/test/ui/variance/variance-unused-type-param.rs:13:15
   |
   |
LL | enum ListCell<T> {
   |               ^ unused parameter
   |
help: if you intended T to be a const parameter, use const T: usize instead
  --> /checkout/src/test/ui/variance/variance-unused-type-param.rs:13:15
   |
LL | enum ListCell<T> {
   |               ^
   = help: consider removing `T`, referring to it in a field, or using a marker such as `PhantomData`
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0392`.


------------------------------------------


---- [ui] ui/variance/variance-unused-region-param.rs stdout ----
diff of stderr:

4 LL | struct SomeStruct<'a> { x: u32 }
5    |                   ^^ unused parameter
6    |
+ help: if you intended 'a to be a const parameter, use const 'a: usize instead
+    |
+    |
+ LL | struct SomeStruct<'a> { x: u32 }
+    |                   ^^
7    = help: consider removing `'a`, referring to it in a field, or using a marker such as `PhantomData`
8 
9 error[E0392]: parameter `'a` is never used

12 LL | enum SomeEnum<'a> { Nothing }
13    |               ^^ unused parameter
14    |
+ help: if you intended 'a to be a const parameter, use const 'a: usize instead
+    |
+    |
+ LL | enum SomeEnum<'a> { Nothing }
+    |               ^^
15    = help: consider removing `'a`, referring to it in a field, or using a marker such as `PhantomData`
17 error: aborting due to 2 previous errors


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-unused-region-param/variance-unused-region-param.stderr
To only update this specific test, also pass `--test-args variance/variance-unused-region-param.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/variance/variance-unused-region-param.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-unused-region-param" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-unused-region-param/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0392]: parameter `'a` is never used
  --> /checkout/src/test/ui/variance/variance-unused-region-param.rs:3:19
   |
LL | struct SomeStruct<'a> { x: u32 } //~ ERROR parameter `'a` is never used
   |                   ^^ unused parameter
   |
help: if you intended 'a to be a const parameter, use const 'a: usize instead
  --> /checkout/src/test/ui/variance/variance-unused-region-param.rs:3:19
   |
LL | struct SomeStruct<'a> { x: u32 } //~ ERROR parameter `'a` is never used
   |                   ^^
   = help: consider removing `'a`, referring to it in a field, or using a marker such as `PhantomData`

error[E0392]: parameter `'a` is never used
  --> /checkout/src/test/ui/variance/variance-unused-region-param.rs:4:15
   |
LL | enum SomeEnum<'a> { Nothing } //~ ERROR parameter `'a` is never used
   |               ^^ unused parameter
   |
help: if you intended 'a to be a const parameter, use const 'a: usize instead
  --> /checkout/src/test/ui/variance/variance-unused-region-param.rs:4:15
   |
LL | enum SomeEnum<'a> { Nothing } //~ ERROR parameter `'a` is never used
   |               ^^
   = help: consider removing `'a`, referring to it in a field, or using a marker such as `PhantomData`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0392`.

---
test result: FAILED. 11692 passed; 24 failed; 97 ignored; 0 measured; 0 filtered out; finished in 119.14s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:12:20
