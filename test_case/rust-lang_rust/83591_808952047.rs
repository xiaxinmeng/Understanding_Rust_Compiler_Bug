plain
.................................................................................................... 2300/11718
.................................................................................................... 2400/11718
.................................................................................................... 2500/11718
.................................................................................................... 2600/11718
............................................................................................iF.i.... 2700/11718
.......F............................................................................................ 2900/11718
.iiiii.............................................................................................. 3000/11718
.iiiii.............................................................................................. 3000/11718
...........................................................FF.FFF..F................................ 3100/11718
.................................................................................................... 3300/11718
.................................................................................................... 3400/11718
.................................................................................................... 3500/11718
.................................................................................................... 3600/11718
---
.................................................................................................... 5800/11718
............................................i....................................................... 5900/11718
.................................................................................................... 6000/11718
..................................................i................................................. 6100/11718
............................................................................................F....... 6200/11718
................F..................................................................ii.ii.......i...i 6300/11718
...................i....i...............................i..........................i................ 6500/11718
.................................................................................................... 6600/11718
.............i...................................................................................... 6700/11718
.................................................................................................... 6800/11718
.................................................................................................... 6800/11718
.............ii..............................................i.....................................F 6900/11718
.............F...............................F...................................................... 7000/11718
.................................................................F.................................. 7100/11718
..........................................ii.................i.i..ii................................ 7300/11718
..............................................................F..................................... 7400/11718
.................................................................................................... 7500/11718
.................................................................................................... 7600/11718
---
.................................................................................................... 8500/11718
.................................................................................................... 8600/11718
.................................................................................................... 8700/11718
.................................................................................................... 8800/11718
............................................iiii.iiiii............F................................. 8900/11718
..............ii...............i...................F................................................ 9000/11718
..............................F..................................................................... 9100/11718
.................................................................................................... 9300/11718
.................................................................................................... 9300/11718
................................F.............F..................................................... 9400/11718
.......................................F............................................................ 9500/11718
.................................................................................................... 9700/11718
.................................................................................................... 9700/11718
......iiiiiii..iiiiii.i............................................................................. 9800/11718
.................................................................................................... 10000/11718
.................................................................................................... 10100/11718
.................................................................................................... 10200/11718
.................................................................................................... 10300/11718
.................................................................................................... 10300/11718
...F.......F.............................F......F..................................F................ 10400/11718
.........i.......................................................................................... 10600/11718
......................F............................................................................. 10700/11718
....................................................................F............................... 10800/11718
.................................................................................................... 10900/11718
---

---- [ui] ui/async-await/generator-desc.rs stdout ----
diff of stderr:

28    = note: distinct uses of `impl Trait` result in different opaque types
30 error[E0308]: mismatched types
-   --> $DIR/generator-desc.rs:14:26
-    |
-    |
- LL |     fun((async || {})(), (async || {})());
-    |                   --     ^^^^^^^^^^^^^^^ expected `async` closure body, found a different `async` closure body
-    |                   |
-    |                   the expected `async` closure body
37    | 
38   ::: $SRC_DIR/core/src/future/mod.rs:LL:COL

42    |                                           |
43    |                                           the expected opaque type
44    |                                           the found opaque type
44    |                                           the found opaque type
+   --> $DIR/generator-desc.rs:14:26
+    |
+ LL |     fun((async || {})(), (async || {})());
+    |                   --     ^^^^^^^^^^^^^^^ expected `async` closure body, found a different `async` closure body
+    |                   |
+    |                   the expected `async` closure body
45    |
46    = note: expected opaque type `impl Future` (`async` closure body)
47               found opaque type `impl Future` (`async` closure body)

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/generator-desc/generator-desc.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/generator-desc/generator-desc.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args async-await/generator-desc.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/generator-desc.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/generator-desc" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/generator-desc/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: mismatched types
  --> /checkout/src/test/ui/async-await/generator-desc.rs:10:25
   |
LL |     fun(async {}, async {});
   |               --        ^^ expected `async` block, found a different `async` block
   |               |
   |               the expected `async` block
   |
   = note: expected `async` block `[static generator@/checkout/src/test/ui/async-await/generator-desc.rs:10:15: 10:17]`
              found `async` block `[static generator@/checkout/src/test/ui/async-await/generator-desc.rs:10:25: 10:27]`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/async-await/generator-desc.rs:12:16
   |
   |
LL | async fn one() {}
   |                - checked the `Output` of this `async fn`, expected opaque type
LL | async fn two() {}
   |                - checked the `Output` of this `async fn`, found opaque type
...
LL |     fun(one(), two());
   |                ^^^^^ expected opaque type, found a different opaque type
   |
   = note: while checking the return type of the `async fn`
   = note: while checking the return type of the `async fn`
   = note: expected opaque type `impl Future` (opaque type at </checkout/src/test/ui/async-await/generator-desc.rs:5:16>)
              found opaque type `impl Future` (opaque type at </checkout/src/test/ui/async-await/generator-desc.rs:6:16>)
   = help: consider `await`ing on both `Future`s
   = note: distinct uses of `impl Trait` result in different opaque types
error[E0308]: mismatched types
   | 
  ::: /checkout/library/core/src/future/mod.rs:61:43
   |
   |
LL | pub const fn from_generator<T>(gen: T) -> impl Future<Output = T::Return>
   |                                           |
   |                                           the expected opaque type
   |                                           the found opaque type
  --> /checkout/src/test/ui/async-await/generator-desc.rs:14:26
  --> /checkout/src/test/ui/async-await/generator-desc.rs:14:26
   |
LL |     fun((async || {})(), (async || {})());
   |                   --     ^^^^^^^^^^^^^^^ expected `async` closure body, found a different `async` closure body
   |                   |
   |                   the expected `async` closure body
   |
   = note: expected opaque type `impl Future` (`async` closure body)
              found opaque type `impl Future` (`async` closure body)
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0308`.

---

1 error: unconstrained generic constant
-   --> $DIR/cross_crate_predicate.rs:7:13
-    |
- LL |     let _ = const_evaluatable_lib::test1::<T>();
6    | 
7   ::: $DIR/auxiliary/const_evaluatable_lib.rs:6:10
8    |


9 LL |     [u8; std::mem::size_of::<T>() - 1]: Sized,
10    |          ---------------------------- required by this bound in `test1`
-    |
-    = help: try adding a `where` bound using this expression: `where [(); std::mem::size_of::<T>() - 1]:`
- error: unconstrained generic constant
15   --> $DIR/cross_crate_predicate.rs:7:13
16    |
16    |
17 LL |     let _ = const_evaluatable_lib::test1::<T>();
18    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
+    |
+    |
+    = help: try adding a `where` bound using this expression: `where [(); std::mem::size_of::<T>() - 1]:`
+ error: unconstrained generic constant
19    | 
20   ::: $DIR/auxiliary/const_evaluatable_lib.rs:4:27
21    |
21    |

22 LL | pub fn test1<T>() -> [u8; std::mem::size_of::<T>() - 1]
23    |                           ---------------------------- required by this bound in `test1`
24    |
24    |
+ LL |     let _ = const_evaluatable_lib::test1::<T>();
+    |
+    |
25    = help: try adding a `where` bound using this expression: `where [(); std::mem::size_of::<T>() - 1]:`
27 error: unconstrained generic constant

-   --> $DIR/cross_crate_predicate.rs:7:13
-    |
-    |
- LL |     let _ = const_evaluatable_lib::test1::<T>();
32    | 
33   ::: $DIR/auxiliary/const_evaluatable_lib.rs:6:10
34    |


35 LL |     [u8; std::mem::size_of::<T>() - 1]: Sized,
36    |          ---------------------------- required by this bound in `test1`
-    |
-    = help: try adding a `where` bound using this expression: `where [(); std::mem::size_of::<T>() - 1]:`
- error: unconstrained generic constant
41   --> $DIR/cross_crate_predicate.rs:7:13
42    |
42    |
43 LL |     let _ = const_evaluatable_lib::test1::<T>();
44    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
+    |
+    |
+    = help: try adding a `where` bound using this expression: `where [(); std::mem::size_of::<T>() - 1]:`
+ error: unconstrained generic constant
45    | 
46   ::: $DIR/auxiliary/const_evaluatable_lib.rs:4:27
47    |
47    |

48 LL | pub fn test1<T>() -> [u8; std::mem::size_of::<T>() - 1]
49    |                           ---------------------------- required by this bound in `test1`
+    |
+    |
+ LL |     let _ = const_evaluatable_lib::test1::<T>();
50    |
50    |
51    = help: try adding a `where` bound using this expression: `where [(); std::mem::size_of::<T>() - 1]:`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const_evaluatable_checked/cross_crate_predicate/cross_crate_predicate.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const_evaluatable_checked/cross_crate_predicate/cross_crate_predicate.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args const-generics/const_evaluatable_checked/cross_crate_predicate.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/const_evaluatable_checked/cross_crate_predicate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const_evaluatable_checked/cross_crate_predicate" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const_evaluatable_checked/cross_crate_predicate/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: unconstrained generic constant
   | 
  ::: /checkout/src/test/ui/const-generics/const_evaluatable_checked/auxiliary/const_evaluatable_lib.rs:6:10
   |
LL |     [u8; std::mem::size_of::<T>() - 1]: Sized,
   |          ---------------------------- required by this bound in `test1`
   |
   |
LL |     let _ = const_evaluatable_lib::test1::<T>();
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); std::mem::size_of::<T>() - 1]:`
error: unconstrained generic constant
   | 
  ::: /checkout/src/test/ui/const-generics/const_evaluatable_checked/auxiliary/const_evaluatable_lib.rs:4:27
   |
   |
LL | pub fn test1<T>() -> [u8; std::mem::size_of::<T>() - 1]
   |                           ---------------------------- required by this bound in `test1`
   |
   |
LL |     let _ = const_evaluatable_lib::test1::<T>();
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); std::mem::size_of::<T>() - 1]:`
error: unconstrained generic constant
   | 
  ::: /checkout/src/test/ui/const-generics/const_evaluatable_checked/auxiliary/const_evaluatable_lib.rs:6:10
   |
   |
LL |     [u8; std::mem::size_of::<T>() - 1]: Sized,
   |          ---------------------------- required by this bound in `test1`
   |
   |
LL |     let _ = const_evaluatable_lib::test1::<T>();
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); std::mem::size_of::<T>() - 1]:`
error: unconstrained generic constant
   | 
  ::: /checkout/src/test/ui/const-generics/const_evaluatable_checked/auxiliary/const_evaluatable_lib.rs:4:27
   |
   |
LL | pub fn test1<T>() -> [u8; std::mem::size_of::<T>() - 1]
   |                           ---------------------------- required by this bound in `test1`
   |
   |
LL |     let _ = const_evaluatable_lib::test1::<T>();
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); std::mem::size_of::<T>() - 1]:`
error: aborting due to 4 previous errors


------------------------------------------
---
13    = help: add `#![feature(destructuring_assignment)]` to the crate attributes to enable


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cross/cross-file-errors/main/main.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args cross/cross-file-errors/main.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/cross/cross-file-errors/main.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cross/cross-file-errors/main" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cross/cross-file-errors/main/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
   = note: see issue #71126 <https://github.com/rust-lang/rust/issues/71126> for more information
   = help: add `#![feature(destructuring_assignment)]` to the crate attributes to enable
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: in expressions, `_` can only be used on the left-hand side of an assignment
   |
LL |         _
LL |         _
   |         ^ `_` not allowed here
  ::: /checkout/src/test/ui/cross/cross-file-errors/main.rs:5:5
   |
LL |     underscore!();
   |     -------------- in this macro invocation
---

---- [ui] ui/derives/deriving-meta-unknown-trait.rs stdout ----
diff of stderr:

1 error: cannot find derive macro `Eqr` in this scope
-   --> $DIR/deriving-meta-unknown-trait.rs:1:10
-    |
- LL | #[derive(Eqr)]
-    |          ^^^ help: a derive macro with a similar name exists: `Eq`
6    | 
7   ::: $SRC_DIR/core/src/cmp.rs:LL:COL


9 LL | pub macro Eq($item:item) {
10    | ------------------------ similarly named derive macro `Eq` defined here
- 
- error: cannot find derive macro `Eqr` in this scope
14    |
14    |
15 LL | #[derive(Eqr)]

16    |          ^^^ help: a derive macro with a similar name exists: `Eq`
+ 
+ error: cannot find derive macro `Eqr` in this scope
17    | 
18   ::: $SRC_DIR/core/src/cmp.rs:LL:COL


20 LL | pub macro Eq($item:item) {
21    | ------------------------ similarly named derive macro `Eq` defined here
+    |
+    |
+ LL | #[derive(Eqr)]
+    |          ^^^ help: a derive macro with a similar name exists: `Eq`
23 error: aborting due to 2 previous errors
24 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/deriving-meta-unknown-trait/deriving-meta-unknown-trait.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args derives/deriving-meta-unknown-trait.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/derives/deriving-meta-unknown-trait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/deriving-meta-unknown-trait" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/deriving-meta-unknown-trait/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: cannot find derive macro `Eqr` in this scope
  ::: /checkout/library/core/src/cmp.rs:286:1
   |
   |
LL | pub macro Eq($item:item) {
   | ------------------------ similarly named derive macro `Eq` defined here
  --> /checkout/src/test/ui/derives/deriving-meta-unknown-trait.rs:1:10
   |
LL | #[derive(Eqr)]
   |          ^^^ help: a derive macro with a similar name exists: `Eq`

error: cannot find derive macro `Eqr` in this scope
  ::: /checkout/library/core/src/cmp.rs:286:1
   |
   |
LL | pub macro Eq($item:item) {
   | ------------------------ similarly named derive macro `Eq` defined here
  --> /checkout/src/test/ui/derives/deriving-meta-unknown-trait.rs:1:10
   |
LL | #[derive(Eqr)]
   |          ^^^ help: a derive macro with a similar name exists: `Eq`
error: aborting due to 2 previous errors


------------------------------------------
------------------------------------------


---- [ui] ui/empty/empty-struct-braces-pat-2.rs stdout ----
diff of stderr:

22    |         ^^^^^^^
23 
24 error[E0532]: expected tuple struct or tuple variant, found struct `XEmpty1`
-   --> $DIR/empty-struct-braces-pat-2.rs:18:9
-    |
- LL |         XEmpty1() => ()
29    | 
29    | 
30   ::: $DIR/auxiliary/empty-struct.rs:1:1


34 LL | pub struct XEmpty2;
35 LL | pub struct XEmpty6();
36    | --------------------- similarly named tuple struct `XEmpty6` defined here
37    |
37    |
+ LL |         XEmpty1() => ()
+    |
38 help: use struct pattern syntax instead
39    |
39    |
40 LL |         XEmpty1 {} => ()
68    |         ^^^^^^^
69 
69 
70 error[E0532]: expected tuple struct or tuple variant, found struct `XEmpty1`
-   --> $DIR/empty-struct-braces-pat-2.rs:24:9
-    |
- LL |         XEmpty1(..) => ()
75    | 
75    | 
76   ::: $DIR/auxiliary/empty-struct.rs:1:1


80 LL | pub struct XEmpty2;
81 LL | pub struct XEmpty6();
82    | --------------------- similarly named tuple struct `XEmpty6` defined here
+    |
+    |
+ LL |         XEmpty1(..) => ()
83    |
84 help: use struct pattern syntax instead
85    |



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/empty/empty-struct-braces-pat-2/empty-struct-braces-pat-2.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args empty/empty-struct-braces-pat-2.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/empty/empty-struct-braces-pat-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/empty/empty-struct-braces-pat-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/empty/empty-struct-braces-pat-2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0532]: expected tuple struct or tuple variant, found struct `Empty1`
   |
   |
LL | struct Empty1 {}
   | ---------------- `Empty1` defined here
...
LL |         Empty1() => () //~ ERROR expected tuple struct or tuple variant, found struct `Empty1`
   | 
  ::: /checkout/src/test/ui/empty/auxiliary/empty-struct.rs:3:1
   |
   |
LL | pub struct XEmpty6();
   | --------------------- similarly named tuple struct `XEmpty6` defined here
help: use struct pattern syntax instead
   |
   |
LL |         Empty1 {} => () //~ ERROR expected tuple struct or tuple variant, found struct `Empty1`
help: a tuple struct with a similar name exists
   |
   |
LL |         XEmpty6() => () //~ ERROR expected tuple struct or tuple variant, found struct `Empty1`


error[E0532]: expected tuple struct or tuple variant, found struct `XEmpty1`
  ::: /checkout/src/test/ui/empty/auxiliary/empty-struct.rs:1:1
   |
   |
LL | pub struct XEmpty1 {}
   | ------------------ `XEmpty1` defined here
LL | pub struct XEmpty2;
LL | pub struct XEmpty6();
   | --------------------- similarly named tuple struct `XEmpty6` defined here
   |
   |
LL |         XEmpty1() => () //~ ERROR expected tuple struct or tuple variant, found struct `XEmpty1`
   |
help: use struct pattern syntax instead
   |
   |
LL |         XEmpty1 {} => () //~ ERROR expected tuple struct or tuple variant, found struct `XEmpty1`
help: a tuple struct with a similar name exists
   |
   |
LL |         XEmpty6() => () //~ ERROR expected tuple struct or tuple variant, found struct `XEmpty1`


error[E0532]: expected tuple struct or tuple variant, found struct `Empty1`
   |
   |
LL | struct Empty1 {}
   | ---------------- `Empty1` defined here
...
LL |         Empty1(..) => () //~ ERROR expected tuple struct or tuple variant, found struct `Empty1`
   | 
  ::: /checkout/src/test/ui/empty/auxiliary/empty-struct.rs:3:1
   |
   |
LL | pub struct XEmpty6();
   | --------------------- similarly named tuple struct `XEmpty6` defined here
help: use struct pattern syntax instead
   |
   |
LL |         Empty1 {} => () //~ ERROR expected tuple struct or tuple variant, found struct `Empty1`
help: a tuple struct with a similar name exists
   |
   |
LL |         XEmpty6(..) => () //~ ERROR expected tuple struct or tuple variant, found struct `Empty1`


error[E0532]: expected tuple struct or tuple variant, found struct `XEmpty1`
  ::: /checkout/src/test/ui/empty/auxiliary/empty-struct.rs:1:1
   |
   |
LL | pub struct XEmpty1 {}
   | ------------------ `XEmpty1` defined here
LL | pub struct XEmpty2;
LL | pub struct XEmpty6();
   | --------------------- similarly named tuple struct `XEmpty6` defined here
   |
   |
LL |         XEmpty1(..) => () //~ ERROR expected tuple struct or tuple variant, found struct `XEmpty1`
   |
help: use struct pattern syntax instead
   |
   |
LL |         XEmpty1 {} => () //~ ERROR expected tuple struct or tuple variant, found struct `XEmpty1`
help: a tuple struct with a similar name exists
   |
   |
LL |         XEmpty6(..) => () //~ ERROR expected tuple struct or tuple variant, found struct `XEmpty1`

error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0532`.
For more information about this error, try `rustc --explain E0532`.

------------------------------------------


---- [ui] ui/empty/empty-struct-braces-pat-1.rs stdout ----
diff of stderr:

8    |         ^^^^^^^^^ help: use struct pattern syntax instead: `E::Empty3 {}`
9 
10 error[E0532]: expected unit struct, unit variant or constant, found struct variant `XE::XEmpty3`
-   --> $DIR/empty-struct-braces-pat-1.rs:31:9
-    |
- LL |         XE::XEmpty3 => ()
15    | 
15    | 
16   ::: $DIR/auxiliary/empty-struct.rs:6:5


19    |     ------- `XE::XEmpty3` defined here
20 LL |     XEmpty4,
21    |     ------- similarly named unit variant `XEmpty4` defined here
+    |
+    |
+ LL |         XE::XEmpty3 => ()
22    |
23 help: use struct pattern syntax instead
24    |



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/empty/empty-struct-braces-pat-1/empty-struct-braces-pat-1.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args empty/empty-struct-braces-pat-1.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/empty/empty-struct-braces-pat-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/empty/empty-struct-braces-pat-1" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/empty/empty-struct-braces-pat-1/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0532]: expected unit struct, unit variant or constant, found struct variant `E::Empty3`
   |
LL |     Empty3 {}
LL |     Empty3 {}
   |     --------- `E::Empty3` defined here
...
LL |         E::Empty3 => ()
   |         ^^^^^^^^^ help: use struct pattern syntax instead: `E::Empty3 {}`

error[E0532]: expected unit struct, unit variant or constant, found struct variant `XE::XEmpty3`
  ::: /checkout/src/test/ui/empty/auxiliary/empty-struct.rs:6:5
   |
   |
LL |     XEmpty3 {},
   |     ------- `XE::XEmpty3` defined here
LL |     XEmpty4,
   |     ------- similarly named unit variant `XEmpty4` defined here
   |
   |
LL |         XE::XEmpty3 => ()
   |
help: use struct pattern syntax instead
   |
   |
LL |         XE::XEmpty3 { /* fields */ } => ()
help: a unit variant with a similar name exists
   |
   |
LL |         XE::XEmpty4 => ()

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0532`.
For more information about this error, try `rustc --explain E0532`.

------------------------------------------


---- [ui] ui/empty/empty-struct-braces-pat-3.rs stdout ----
diff of stderr:

8    |         ^^^^^^^^^^^ help: use struct pattern syntax instead: `E::Empty3 {}`
9 
10 error[E0532]: expected tuple struct or tuple variant, found struct variant `XE::XEmpty3`
-   --> $DIR/empty-struct-braces-pat-3.rs:21:9
-    |
- LL |         XE::XEmpty3() => ()
15    | 
15    | 
16   ::: $DIR/auxiliary/empty-struct.rs:6:5

20 LL |     XEmpty4,
21 LL |     XEmpty5(),
21 LL |     XEmpty5(),
22    |     --------- similarly named tuple variant `XEmpty5` defined here
23    |
23    |
+ LL |         XE::XEmpty3() => ()
+    |
24 help: use struct pattern syntax instead
25    |
25    |
26 LL |         XE::XEmpty3 { /* fields */ } => ()

40    |         ^^^^^^^^^^^^^ help: use struct pattern syntax instead: `E::Empty3 {}`
41 
42 error[E0532]: expected tuple struct or tuple variant, found struct variant `XE::XEmpty3`
-   --> $DIR/empty-struct-braces-pat-3.rs:29:9
-    |
- LL |         XE::XEmpty3(..) => ()
47    | 
47    | 
48   ::: $DIR/auxiliary/empty-struct.rs:6:5

52 LL |     XEmpty4,
53 LL |     XEmpty5(),
53 LL |     XEmpty5(),
54    |     --------- similarly named tuple variant `XEmpty5` defined here
+    |
+    |
+ LL |         XE::XEmpty3(..) => ()
55    |
56 help: use struct pattern syntax instead
57    |



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/empty/empty-struct-braces-pat-3/empty-struct-braces-pat-3.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args empty/empty-struct-braces-pat-3.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/empty/empty-struct-braces-pat-3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/empty/empty-struct-braces-pat-3" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/empty/empty-struct-braces-pat-3/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0532]: expected tuple struct or tuple variant, found struct variant `E::Empty3`
   |
LL |     Empty3 {}
LL |     Empty3 {}
   |     --------- `E::Empty3` defined here
...
LL |         E::Empty3() => ()
   |         ^^^^^^^^^^^ help: use struct pattern syntax instead: `E::Empty3 {}`

error[E0532]: expected tuple struct or tuple variant, found struct variant `XE::XEmpty3`
  ::: /checkout/src/test/ui/empty/auxiliary/empty-struct.rs:6:5
   |
   |
LL |     XEmpty3 {},
   |     ------- `XE::XEmpty3` defined here
LL |     XEmpty4,
LL |     XEmpty5(),
   |     --------- similarly named tuple variant `XEmpty5` defined here
   |
   |
LL |         XE::XEmpty3() => ()
   |
help: use struct pattern syntax instead
   |
   |
LL |         XE::XEmpty3 { /* fields */ } => ()
help: a tuple variant with a similar name exists
   |
   |
LL |         XE::XEmpty5() => ()


error[E0532]: expected tuple struct or tuple variant, found struct variant `E::Empty3`
   |
LL |     Empty3 {}
LL |     Empty3 {}
   |     --------- `E::Empty3` defined here
...
LL |         E::Empty3(..) => ()
   |         ^^^^^^^^^^^^^ help: use struct pattern syntax instead: `E::Empty3 {}`

error[E0532]: expected tuple struct or tuple variant, found struct variant `XE::XEmpty3`
  ::: /checkout/src/test/ui/empty/auxiliary/empty-struct.rs:6:5
   |
   |
LL |     XEmpty3 {},
   |     ------- `XE::XEmpty3` defined here
LL |     XEmpty4,
LL |     XEmpty5(),
   |     --------- similarly named tuple variant `XEmpty5` defined here
   |
   |
LL |         XE::XEmpty3(..) => ()
   |
help: use struct pattern syntax instead
   |
   |
LL |         XE::XEmpty3 { /* fields */ } => ()
help: a tuple variant with a similar name exists
   |
   |
LL |         XE::XEmpty5(..) => ()

error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0532`.
For more information about this error, try `rustc --explain E0532`.

------------------------------------------


---- [ui] ui/empty/empty-struct-tuple-pat.rs stdout ----
diff of stderr:

26    |         ^^^^^^^^^ help: use the tuple variant pattern syntax instead: `E::Empty4()`
27 
28 error[E0532]: expected unit struct, unit variant or constant, found tuple variant `XE::XEmpty5`
-   --> $DIR/empty-struct-tuple-pat.rs:33:9
-    |
- LL |         XE::XEmpty5 => (),
33    | 
33    | 
34   ::: $DIR/auxiliary/empty-struct.rs:7:5


37    |     ------- similarly named unit variant `XEmpty4` defined here
38 LL |     XEmpty5(),
39    |     --------- `XE::XEmpty5` defined here
+    |
+    |
+ LL |         XE::XEmpty5 => (),
40    |
40    |
41 help: use the tuple variant pattern syntax instead


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/empty/empty-struct-tuple-pat/empty-struct-tuple-pat.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/empty/empty-struct-tuple-pat/empty-struct-tuple-pat.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args empty/empty-struct-tuple-pat.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/empty/empty-struct-tuple-pat.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/empty/empty-struct-tuple-pat" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/empty/empty-struct-tuple-pat/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0530]: match bindings cannot shadow tuple structs
   |
LL | struct Empty2();
LL | struct Empty2();
   | ---------------- the tuple struct `Empty2` is defined here
---

---- [ui] ui/empty/empty-struct-braces-expr.rs stdout ----
diff of stderr:

63    |              ^^^^^^^^^^^ help: use struct literal syntax instead: `E::Empty3 {}`
64 
65 error[E0423]: expected value, found struct `XEmpty1`
-   --> $DIR/empty-struct-braces-expr.rs:22:15
-    |
- LL |     let xe1 = XEmpty1;
70    | 
70    | 
71   ::: $DIR/auxiliary/empty-struct.rs:1:1


74    | ------------------ `XEmpty1` defined here
75 LL | pub struct XEmpty2;
76    | ------------------- similarly named unit struct `XEmpty2` defined here
77    |
77    |
+ LL |     let xe1 = XEmpty1;
+    |
78 help: use struct literal syntax instead
79    |
79    |
80 LL |     let xe1 = XEmpty1 {};
85    |               ^^^^^^^
86 
86 
87 error[E0423]: expected function, tuple struct or tuple variant, found struct `XEmpty1`
-   --> $DIR/empty-struct-braces-expr.rs:23:15
-    |
- LL |     let xe1 = XEmpty1();
92    | 
92    | 
93   ::: $DIR/auxiliary/empty-struct.rs:1:1


96    | ------------------ `XEmpty1` defined here
97 LL | pub struct XEmpty2;
98    | ------------------- similarly named unit struct `XEmpty2` defined here
+    |
+    |
+ LL |     let xe1 = XEmpty1();
99    |
100 help: use struct literal syntax instead
101    |



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/empty/empty-struct-braces-expr/empty-struct-braces-expr.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args empty/empty-struct-braces-expr.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/empty/empty-struct-braces-expr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/empty/empty-struct-braces-expr" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/empty/empty-struct-braces-expr/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0423]: expected value, found struct `Empty1`
   |
   |
LL | struct Empty1 {}
   | ---------------- `Empty1` defined here
...
LL |     let e1 = Empty1; //~ ERROR expected value, found struct `Empty1`
   | 
  ::: /checkout/src/test/ui/empty/auxiliary/empty-struct.rs:2:1
   |
   |
LL | pub struct XEmpty2;
   | ------------------- similarly named unit struct `XEmpty2` defined here
help: use struct literal syntax instead
   |
   |
LL |     let e1 = Empty1 {}; //~ ERROR expected value, found struct `Empty1`
help: a unit struct with a similar name exists
   |
   |
LL |     let e1 = XEmpty2; //~ ERROR expected value, found struct `Empty1`


error[E0423]: expected function, tuple struct or tuple variant, found struct `Empty1`
   |
   |
LL | struct Empty1 {}
   | ---------------- `Empty1` defined here
LL |     let e1 = Empty1();
   |              ^^^^^^^^
   | 
  ::: /checkout/src/test/ui/empty/auxiliary/empty-struct.rs:2:1
  ::: /checkout/src/test/ui/empty/auxiliary/empty-struct.rs:2:1
   |
LL | pub struct XEmpty2;
   | ------------------- similarly named unit struct `XEmpty2` defined here
help: use struct literal syntax instead
   |
   |
LL |     let e1 = Empty1 {};
help: a unit struct with a similar name exists
   |
   |
LL |     let e1 = XEmpty2();


error[E0423]: expected value, found struct variant `E::Empty3`
   |
LL |     Empty3 {}
LL |     Empty3 {}
   |     --------- `E::Empty3` defined here
...
LL |     let e3 = E::Empty3; //~ ERROR expected value, found struct variant `E::Empty3`
   |              ^^^^^^^^^ help: use struct literal syntax instead: `E::Empty3 {}`

error[E0423]: expected function, tuple struct or tuple variant, found struct variant `E::Empty3`
   |
LL |     Empty3 {}
LL |     Empty3 {}
   |     --------- `E::Empty3` defined here
...
LL |     let e3 = E::Empty3();
   |              ^^^^^^^^^^^ help: use struct literal syntax instead: `E::Empty3 {}`

error[E0423]: expected value, found struct `XEmpty1`
  ::: /checkout/src/test/ui/empty/auxiliary/empty-struct.rs:1:1
   |
   |
LL | pub struct XEmpty1 {}
   | ------------------ `XEmpty1` defined here
LL | pub struct XEmpty2;
   | ------------------- similarly named unit struct `XEmpty2` defined here
   |
   |
LL |     let xe1 = XEmpty1; //~ ERROR expected value, found struct `XEmpty1`
   |
help: use struct literal syntax instead
   |
   |
LL |     let xe1 = XEmpty1 {}; //~ ERROR expected value, found struct `XEmpty1`
help: a unit struct with a similar name exists
   |
   |
LL |     let xe1 = XEmpty2; //~ ERROR expected value, found struct `XEmpty1`


error[E0423]: expected function, tuple struct or tuple variant, found struct `XEmpty1`
  ::: /checkout/src/test/ui/empty/auxiliary/empty-struct.rs:1:1
   |
   |
LL | pub struct XEmpty1 {}
   | ------------------ `XEmpty1` defined here
LL | pub struct XEmpty2;
   | ------------------- similarly named unit struct `XEmpty2` defined here
   |
   |
LL |     let xe1 = XEmpty1();
   |
help: use struct literal syntax instead
   |
   |
LL |     let xe1 = XEmpty1 {};
help: a unit struct with a similar name exists
   |
   |
LL |     let xe1 = XEmpty2();


error[E0599]: no variant or associated item named `Empty3` found for enum `empty_struct::XE` in the current scope
   |
   |
LL |     let xe3 = XE::Empty3; //~ ERROR no variant or associated item named `Empty3` found for enum
   |                   |
   |                   |
   |                   variant or associated item not found in `empty_struct::XE`
   |                   help: there is a variant with a similar name: `XEmpty3`

error[E0599]: no variant or associated item named `Empty3` found for enum `empty_struct::XE` in the current scope
   |
   |
LL |     let xe3 = XE::Empty3(); //~ ERROR no variant or associated item named `Empty3` found for enum
   |                   |
   |                   |
   |                   variant or associated item not found in `empty_struct::XE`
   |                   help: there is a variant with a similar name: `XEmpty3`

error[E0599]: no variant named `Empty1` found for enum `empty_struct::XE`
   |
   |
LL |     XE::Empty1 {}; //~ ERROR no variant named `Empty1` found for enum `empty_struct::XE`
   |         ^^^^^^ help: there is a variant with a similar name: `XEmpty3`
error: aborting due to 9 previous errors

Some errors have detailed explanations: E0423, E0599.
For more information about an error, try `rustc --explain E0423`.
For more information about an error, try `rustc --explain E0423`.

------------------------------------------


---- [ui] ui/empty/empty-struct-unit-pat.rs stdout ----
diff of stderr:

1 error[E0532]: expected tuple struct or tuple variant, found unit struct `Empty2`
+    | 
+   ::: $DIR/auxiliary/empty-struct.rs:3:1
+    |
+ LL | pub struct XEmpty6();
+    | --------------------- similarly named tuple struct `XEmpty6` defined here
3    |
3    |
4 LL |         Empty2() => ()

5    |         ^^^^^^ help: a tuple struct with a similar name exists: `XEmpty6`
+ 
+ error[E0532]: expected tuple struct or tuple variant, found unit struct `XEmpty2`
6    | 
7   ::: $DIR/auxiliary/empty-struct.rs:3:1


9 LL | pub struct XEmpty6();
10    | --------------------- similarly named tuple struct `XEmpty6` defined here
- 
- error[E0532]: expected tuple struct or tuple variant, found unit struct `XEmpty2`
14    |
14    |
15 LL |         XEmpty2() => ()

16    |         ^^^^^^^ help: a tuple struct with a similar name exists: `XEmpty6`
+ 
+ error[E0532]: expected tuple struct or tuple variant, found unit struct `Empty2`
17    | 
18   ::: $DIR/auxiliary/empty-struct.rs:3:1


20 LL | pub struct XEmpty6();
21    | --------------------- similarly named tuple struct `XEmpty6` defined here
- 
- error[E0532]: expected tuple struct or tuple variant, found unit struct `Empty2`
25    |
25    |
26 LL |         Empty2(..) => ()

27    |         ^^^^^^ help: a tuple struct with a similar name exists: `XEmpty6`
+ 
+ error[E0532]: expected tuple struct or tuple variant, found unit struct `XEmpty2`
28    | 
29   ::: $DIR/auxiliary/empty-struct.rs:3:1


31 LL | pub struct XEmpty6();
32    | --------------------- similarly named tuple struct `XEmpty6` defined here
- 
- error[E0532]: expected tuple struct or tuple variant, found unit struct `XEmpty2`
36    |
36    |
37 LL |         XEmpty2(..) => ()

38    |         ^^^^^^^ help: a tuple struct with a similar name exists: `XEmpty6`
-    | 
-   ::: $DIR/auxiliary/empty-struct.rs:3:1
-    |
- LL | pub struct XEmpty6();
-    | --------------------- similarly named tuple struct `XEmpty6` defined here
44 
45 error[E0532]: expected tuple struct or tuple variant, found unit variant `E::Empty4`

49    |         ^^^^^^^^^ not a tuple struct or tuple variant
50 
50 
51 error[E0532]: expected tuple struct or tuple variant, found unit variant `XE::XEmpty4`
+    | 
+   ::: $DIR/auxiliary/empty-struct.rs:8:5
+    |
+ LL |     XEmpty5(),
+    |     --------- similarly named tuple variant `XEmpty5` defined here
53    |
53    |
54 LL |         XE::XEmpty4() => (),
55    |         ^^^^-------
56    |             |
56    |             |
57    |             help: a tuple variant with a similar name exists: `XEmpty5`
-    | 
-   ::: $DIR/auxiliary/empty-struct.rs:8:5
-    |
- LL |     XEmpty5(),
-    |     --------- similarly named tuple variant `XEmpty5` defined here
63 
64 error[E0532]: expected tuple struct or tuple variant, found unit variant `E::Empty4`

68    |         ^^^^^^^^^ not a tuple struct or tuple variant
69 
69 
70 error[E0532]: expected tuple struct or tuple variant, found unit variant `XE::XEmpty4`
+    | 
+   ::: $DIR/auxiliary/empty-struct.rs:8:5
+    |
+ LL |     XEmpty5(),
+    |     --------- similarly named tuple variant `XEmpty5` defined here
72    |
72    |
73 LL |         XE::XEmpty4(..) => (),
74    |         ^^^^-------
75    |             |
75    |             |
76    |             help: a tuple variant with a similar name exists: `XEmpty5`
-    | 
-   ::: $DIR/auxiliary/empty-struct.rs:8:5
-    |
- LL |     XEmpty5(),
-    |     --------- similarly named tuple variant `XEmpty5` defined here
83 error: aborting due to 8 previous errors
84 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/empty/empty-struct-unit-pat/empty-struct-unit-pat.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args empty/empty-struct-unit-pat.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/empty/empty-struct-unit-pat.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/empty/empty-struct-unit-pat" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/empty/empty-struct-unit-pat/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0532]: expected tuple struct or tuple variant, found unit struct `Empty2`
  ::: /checkout/src/test/ui/empty/auxiliary/empty-struct.rs:3:1
   |
   |
LL | pub struct XEmpty6();
   | --------------------- similarly named tuple struct `XEmpty6` defined here
   |
   |
LL |         Empty2() => () //~ ERROR expected tuple struct or tuple variant, found unit struct `Empty2`
   |         ^^^^^^ help: a tuple struct with a similar name exists: `XEmpty6`

error[E0532]: expected tuple struct or tuple variant, found unit struct `XEmpty2`
  ::: /checkout/src/test/ui/empty/auxiliary/empty-struct.rs:3:1
   |
   |
LL | pub struct XEmpty6();
   | --------------------- similarly named tuple struct `XEmpty6` defined here
   |
   |
LL |         XEmpty2() => ()
   |         ^^^^^^^ help: a tuple struct with a similar name exists: `XEmpty6`

error[E0532]: expected tuple struct or tuple variant, found unit struct `Empty2`
  ::: /checkout/src/test/ui/empty/auxiliary/empty-struct.rs:3:1
   |
   |
LL | pub struct XEmpty6();
   | --------------------- similarly named tuple struct `XEmpty6` defined here
   |
   |
LL |         Empty2(..) => ()
   |         ^^^^^^ help: a tuple struct with a similar name exists: `XEmpty6`

error[E0532]: expected tuple struct or tuple variant, found unit struct `XEmpty2`
  ::: /checkout/src/test/ui/empty/auxiliary/empty-struct.rs:3:1
   |
   |
LL | pub struct XEmpty6();
   | --------------------- similarly named tuple struct `XEmpty6` defined here
   |
   |
LL |         XEmpty2(..) => ()
   |         ^^^^^^^ help: a tuple struct with a similar name exists: `XEmpty6`

error[E0532]: expected tuple struct or tuple variant, found unit variant `E::Empty4`
   |
   |
LL |         E::Empty4() => ()
   |         ^^^^^^^^^ not a tuple struct or tuple variant

error[E0532]: expected tuple struct or tuple variant, found unit variant `XE::XEmpty4`
  ::: /checkout/src/test/ui/empty/auxiliary/empty-struct.rs:8:5
   |
LL |     XEmpty5(),
LL |     XEmpty5(),
   |     --------- similarly named tuple variant `XEmpty5` defined here
   |
   |
LL |         XE::XEmpty4() => (),
   |             |
   |             |
   |             help: a tuple variant with a similar name exists: `XEmpty5`

error[E0532]: expected tuple struct or tuple variant, found unit variant `E::Empty4`
   |
   |
LL |         E::Empty4(..) => ()
   |         ^^^^^^^^^ not a tuple struct or tuple variant

error[E0532]: expected tuple struct or tuple variant, found unit variant `XE::XEmpty4`
  ::: /checkout/src/test/ui/empty/auxiliary/empty-struct.rs:8:5
   |
LL |     XEmpty5(),
LL |     XEmpty5(),
   |     --------- similarly named tuple variant `XEmpty5` defined here
   |
   |
LL |         XE::XEmpty4(..) => (),
   |             |
   |             |
   |             help: a tuple variant with a similar name exists: `XEmpty5`
error: aborting due to 8 previous errors

For more information about this error, try `rustc --explain E0532`.

---
diff of stderr:

27    |           ^^^^^^^^^^            ^
28 
29 error[E0643]: method `hash` has incompatible signature for trait
-   --> $DIR/impl-generic-mismatch.rs:28:33
-    |
- LL |     fn hash(&self, hasher: &mut impl Hasher) {}
-    |                                 ^^^^^^^^^^^ expected generic parameter, found `impl Trait`
34    | 
35   ::: $SRC_DIR/core/src/hash/mod.rs:LL:COL


37 LL |     fn hash<H: Hasher>(&self, state: &mut H);
38    |             - declaration in trait here
+    |
+    |
+ LL |     fn hash(&self, hasher: &mut impl Hasher) {}
+    |                                 ^^^^^^^^^^^ expected generic parameter, found `impl Trait`
40 error: aborting due to 3 previous errors
41 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/impl-generic-mismatch/impl-generic-mismatch.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args impl-trait/impl-generic-mismatch.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/impl-generic-mismatch.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/impl-generic-mismatch" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/impl-generic-mismatch/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0643]: method `foo` has incompatible signature for trait
  --> /checkout/src/test/ui/impl-trait/impl-generic-mismatch.rs:8:12
   |
LL |     fn foo(&self, _: &impl Debug);
   |                       ---------- declaration in trait here
...
LL |     fn foo<U: Debug>(&self, _: &U) { }
   |            ^ expected `impl Trait`, found generic parameter
   |
help: try removing the generic parameter and using `impl Trait` instead
   |
LL |     fn foo(&self, _: &impl Debug) { }


error[E0643]: method `bar` has incompatible signature for trait
   |
   |
LL |     fn bar<U: Debug>(&self, _: &U);
   |            - declaration in trait here
...
LL |     fn bar(&self, _: &impl Debug) { }
   |                       ^^^^^^^^^^ expected generic parameter, found `impl Trait`
   |
help: try changing the `impl Trait` argument to a generic parameter
   |
LL |     fn bar<U: Debug>(&self, _: &U) { }
   |           ^^^^^^^^^^            ^

error[E0643]: method `hash` has incompatible signature for trait
  ::: /checkout/library/core/src/hash/mod.rs:174:13
   |
   |
LL |     fn hash<H: Hasher>(&self, state: &mut H);
   |             - declaration in trait here
   |
   |
LL |     fn hash(&self, hasher: &mut impl Hasher) {}
   |                                 ^^^^^^^^^^^ expected generic parameter, found `impl Trait`
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0643`.


------------------------------------------


---- [ui] ui/issues/issue-17546.rs stdout ----
diff of stderr:

1 error[E0573]: expected type, found variant `NoResult`
-   --> $DIR/issue-17546.rs:14:17
-    |
- LL |     fn new() -> NoResult<MyEnum, String> {
6    | 
6    | 
7   ::: $SRC_DIR/core/src/result.rs:LL:COL


9 LL | pub enum Result<T, E> {
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
10    | --------------------- similarly named enum `Result` defined here
11    |
11    |
+ LL |     fn new() -> NoResult<MyEnum, String> {
+    |
+    |
12 help: try using the variant's enum
13    |
14 LL |     fn new() -> foo::MyEnum {
53    |
54 
54 
55 error[E0573]: expected type, found variant `NoResult`
-   --> $DIR/issue-17546.rs:35:15
-    |
- LL | fn newer() -> NoResult<foo::MyEnum, String> {
60    | 
60    | 
61   ::: $SRC_DIR/core/src/result.rs:LL:COL


63 LL | pub enum Result<T, E> {
64    | --------------------- similarly named enum `Result` defined here
+    |
+    |
+ LL | fn newer() -> NoResult<foo::MyEnum, String> {
65    |
65    |
66 help: try using the variant's enum


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-17546/issue-17546.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-17546/issue-17546.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-17546.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-17546.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-17546" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-17546/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0573]: expected type, found variant `NoResult`
  ::: /checkout/library/core/src/result.rs:241:1
   |
   |
LL | pub enum Result<T, E> {
   | --------------------- similarly named enum `Result` defined here
   |
   |
LL |     fn new() -> NoResult<MyEnum, String> {
   |
   |
help: try using the variant's enum
   |
LL |     fn new() -> foo::MyEnum {
   |                 ^^^^^^^^^^^
help: an enum with a similar name exists
   |
LL |     fn new() -> Result<MyEnum, String> {

error[E0573]: expected type, found variant `Result`
  --> /checkout/src/test/ui/issues/issue-17546.rs:24:17
   |
   |
LL |     fn new() -> Result<foo::MyEnum, String> {
   |
help: consider importing one of these items instead
   |
LL |     use std::fmt::Result;
---

error[E0573]: expected type, found variant `Result`
  --> /checkout/src/test/ui/issues/issue-17546.rs:30:13
   |
LL | fn new() -> Result<foo::MyEnum, String> {
   |
help: consider importing one of these items instead
   |
LL | use std::fmt::Result;
---
   |
LL | use std::thread::Result;
   |

error[E0573]: expected type, found variant `NoResult`
  ::: /checkout/library/core/src/result.rs:241:1
   |
   |
LL | pub enum Result<T, E> {
   | --------------------- similarly named enum `Result` defined here
   |
   |
LL | fn newer() -> NoResult<foo::MyEnum, String> {
   |
   |
help: try using the variant's enum
   |
LL | fn newer() -> foo::MyEnum {
   |               ^^^^^^^^^^^
help: an enum with a similar name exists
   |
LL | fn newer() -> Result<foo::MyEnum, String> {

error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0573`.
For more information about this error, try `rustc --explain E0573`.

------------------------------------------


---- [ui] ui/issues/issue-7607-1.rs stdout ----
diff of stderr:

1 error[E0412]: cannot find type `Fo` in this scope
-   --> $DIR/issue-7607-1.rs:5:6
-    |
- LL | impl Fo {
-    |      ^^ help: a trait with a similar name exists: `Fn`
6    | 
7   ::: $SRC_DIR/core/src/ops/function.rs:LL:COL


9 LL | pub trait Fn<Args>: FnMut<Args> {
10    | ------------------------------- similarly named trait `Fn` defined here
+    |
+    |
+ LL | impl Fo {
+    |      ^^ help: a trait with a similar name exists: `Fn`
12 error: aborting due to previous error
13 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-7607-1/issue-7607-1.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-7607-1.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-7607-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-7607-1" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-7607-1/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0412]: cannot find type `Fo` in this scope
  ::: /checkout/library/core/src/ops/function.rs:67:1
   |
   |
LL | pub trait Fn<Args>: FnMut<Args> {
   | ------------------------------- similarly named trait `Fn` defined here
   |
   |
LL | impl Fo { //~ ERROR cannot find type `Fo` in this scope
   |      ^^ help: a trait with a similar name exists: `Fn`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0412`.

---
6 
7 error[E0412]: cannot find type `F` in this scope
-   --> $DIR/issue-78720.rs:13:12
-    |
- LL |     _func: F,
12    | 
12    | 
13   ::: $SRC_DIR/core/src/ops/function.rs:LL:COL


15 LL | pub trait Fn<Args>: FnMut<Args> {
16    | ------------------------------- similarly named trait `Fn` defined here
+    |
+    |
+ LL |     _func: F,
17    |
18 help: a trait with a similar name exists
19    |



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-78720/issue-78720.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-78720.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-78720.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-78720" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-78720/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: at least one trait must be specified
  --> /checkout/src/test/ui/issues/issue-78720.rs:1:16
   |
LL | fn server() -> impl {

error[E0412]: cannot find type `F` in this scope
   | 
  ::: /checkout/library/core/src/ops/function.rs:67:1
  ::: /checkout/library/core/src/ops/function.rs:67:1
   |
LL | pub trait Fn<Args>: FnMut<Args> {
   | ------------------------------- similarly named trait `Fn` defined here
   |
   |
LL |     _func: F,
   |
help: a trait with a similar name exists
   |
   |
LL |     _func: Fn,
help: you might be missing a type parameter
   |
   |
LL | struct Map2<Segment2, F> {

error[E0308]: mismatched types
  --> /checkout/src/test/ui/issues/issue-78720.rs:7:36
   |
   |
LL |     fn map2<F>(self, F) -> Map2<F> {}
   |                                    ^^ expected struct `Map2`, found `()`
   |
   = note: expected struct `Map2<F>`


error[E0277]: the size for values of type `Self` cannot be known at compilation time
   |
   |
LL |     fn map2<F>(self, F) -> Map2<F> {}
   |                ^^^^ doesn't have a size known at compile-time
   |
   = help: unsized fn params are gated as an unstable feature
help: consider further restricting `Self`
   |
LL |     fn map2<F>(self, F) -> Map2<F> where Self: Sized {}
   |                                    ^^^^^^^^^^^^^^^^^
help: function arguments must have a statically known size, borrowed types always have a known size
   |
LL |     fn map2<F>(&self, F) -> Map2<F> {}

error: aborting due to 4 previous errors

Some errors have detailed explanations: E0277, E0308, E0412.
---

---- [ui] ui/macros/macro-name-typo.rs stdout ----
diff of stderr:

1 error: cannot find macro `printlx` in this scope
-   --> $DIR/macro-name-typo.rs:2:5
-    |
- LL |     printlx!("oh noes!");
-    |     ^^^^^^^ help: a macro with a similar name exists: `println`
6    | 
7   ::: $SRC_DIR/std/src/macros.rs:LL:COL

9 LL | macro_rules! println {
10    | -------------------- similarly named macro `println` defined here
+   --> $DIR/macro-name-typo.rs:2:5
+   --> $DIR/macro-name-typo.rs:2:5
+    |
+ LL |     printlx!("oh noes!");
+    |     ^^^^^^^ help: a macro with a similar name exists: `println`
12 error: aborting due to previous error
13 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-name-typo/macro-name-typo.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args macros/macro-name-typo.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/macro-name-typo.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-name-typo" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-name-typo/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: cannot find macro `printlx` in this scope
  ::: /checkout/library/std/src/macros.rs:94:1
   |
LL | macro_rules! println {
   | -------------------- similarly named macro `println` defined here
   | -------------------- similarly named macro `println` defined here
  --> /checkout/src/test/ui/macros/macro-name-typo.rs:2:5
   |
LL |     printlx!("oh noes!"); //~ ERROR cannot find
   |     ^^^^^^^ help: a macro with a similar name exists: `println`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/macros/macro-path-prelude-fail-3.rs stdout ----
diff of stderr:

1 error: cannot find macro `inline` in this scope
-   --> $DIR/macro-path-prelude-fail-3.rs:2:5
-    |
- LL |     inline!();
-    |     ^^^^^^ help: a macro with a similar name exists: `line`
6    | 
7   ::: $SRC_DIR/core/src/macros/mod.rs:LL:COL

9 LL |     macro_rules! line {
9 LL |     macro_rules! line {
10    |     ----------------- similarly named macro `line` defined here
+    |
+    |
+ LL |     inline!();
+    |     ^^^^^^ help: a macro with a similar name exists: `line`
12 error: aborting due to previous error
13 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-path-prelude-fail-3/macro-path-prelude-fail-3.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args macros/macro-path-prelude-fail-3.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/macro-path-prelude-fail-3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-path-prelude-fail-3" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-path-prelude-fail-3/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: cannot find macro `inline` in this scope
  ::: /checkout/library/core/src/macros/mod.rs:988:5
   |
LL |     macro_rules! line {
LL |     macro_rules! line {
   |     ----------------- similarly named macro `line` defined here
   |
   |
LL |     inline!(); //~ ERROR cannot find macro `inline` in this scope
   |     ^^^^^^ help: a macro with a similar name exists: `line`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/macros/macro-use-wrong-name.rs stdout ----
diff of stderr:

1 error: cannot find macro `macro_two` in this scope
-   --> $DIR/macro-use-wrong-name.rs:7:5
-    |
- LL |     macro_two!();
-    |     ^^^^^^^^^ help: a macro with a similar name exists: `macro_one`
6    | 
7   ::: $DIR/auxiliary/two_macros.rs:2:1


9 LL | macro_rules! macro_one { () => ("one") }
10    | ---------------------- similarly named macro `macro_one` defined here
+    |
+    |
+ LL |     macro_two!();
+    |     ^^^^^^^^^ help: a macro with a similar name exists: `macro_one`
12    = note: consider importing this macro:
13            two_macros::macro_two



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-use-wrong-name/macro-use-wrong-name.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args macros/macro-use-wrong-name.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/macro-use-wrong-name.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-use-wrong-name" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-use-wrong-name/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: cannot find macro `macro_two` in this scope
  ::: /checkout/src/test/ui/macros/auxiliary/two_macros.rs:2:1
   |
   |
LL | macro_rules! macro_one { () => ("one") }
   | ---------------------- similarly named macro `macro_one` defined here
   |
   |
LL |     macro_two!();
   |     ^^^^^^^^^ help: a macro with a similar name exists: `macro_one`
   = note: consider importing this macro:
           two_macros::macro_two

error: aborting due to previous error
---

---- [ui] ui/methods/method-call-lifetime-args-unresolved.rs stdout ----
diff of stderr:

1 warning: cannot specify lifetime arguments explicitly if late bound lifetime parameters are present
-   --> $DIR/method-call-lifetime-args-unresolved.rs:2:15
-    |
- LL |     0.clone::<'a>();
6    | 
6    | 
7   ::: $SRC_DIR/core/src/clone.rs:LL:COL


9 LL |     fn clone(&self) -> Self;
10    |              - the late bound lifetime parameter is introduced here
+    |
+    |
+ LL |     0.clone::<'a>();
11    |
12    = note: `#[warn(late_bound_lifetime_arguments)]` on by default
13    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/methods/method-call-lifetime-args-unresolved/method-call-lifetime-args-unresolved.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args methods/method-call-lifetime-args-unresolved.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/methods/method-call-lifetime-args-unresolved.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/methods/method-call-lifetime-args-unresolved" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/methods/method-call-lifetime-args-unresolved/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: cannot specify lifetime arguments explicitly if late bound lifetime parameters are present
  ::: /checkout/library/core/src/clone.rs:121:14
   |
   |
LL |     fn clone(&self) -> Self;
   |              - the late bound lifetime parameter is introduced here
   |
   |
LL |     0.clone::<'a>(); //~ ERROR use of undeclared lifetime name `'a`
   |
   = note: `#[warn(late_bound_lifetime_arguments)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #42868 <https://github.com/rust-lang/rust/issues/42868>
   = note: for more information, see issue #42868 <https://github.com/rust-lang/rust/issues/42868>

error[E0261]: use of undeclared lifetime name `'a`
   |
LL | fn main() {
LL | fn main() {
   |        - help: consider introducing lifetime `'a` here: `<'a>`
LL |     0.clone::<'a>(); //~ ERROR use of undeclared lifetime name `'a`
   |               ^^ undeclared lifetime
   |
   = help: if you want to experiment with in-band lifetime bindings, add `#![feature(in_band_lifetimes)]` to the crate attributes
error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0261`.

---
diff of stderr:

20    |
21 
22 error[E0423]: expected value, found type alias `xm1::S`
-   --> $DIR/namespace-mix.rs:40:11
-    |
- LL |     check(xm1::S);
27    | 
27    | 
28   ::: $DIR/auxiliary/namespace-mix.rs:3:5

30 LL |     pub struct TS();
30 LL |     pub struct TS();
31    |     ---------------- similarly named tuple struct `TS` defined here
32    |
32    |
+ LL |     check(xm1::S);
+    |
33    = note: can't use a type alias as a constructor
34 help: a tuple struct with a similar name exists
35    |
35    |

69    |
70 
71 error[E0423]: expected value, found struct variant `xm7::V`
-   --> $DIR/namespace-mix.rs:106:11
-    |
- LL |     check(xm7::V);
76    | 
76    | 
77   ::: $DIR/auxiliary/namespace-mix.rs:6:9


80    |         - `xm7::V` defined here
81 LL |         TV(),
82    |         ---- similarly named tuple variant `TV` defined here
+    |
+    |
+ LL |     check(xm7::V);
83    |
84 help: use struct literal syntax instead
85    |



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/namespace/namespace-mix/namespace-mix.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args namespace/namespace-mix.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/namespace/namespace-mix.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/namespace/namespace-mix" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/namespace/namespace-mix/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0423]: expected value, found type alias `m1::S`
   |
LL |     pub struct TS();
LL |     pub struct TS();
   |     ---------------- similarly named tuple struct `TS` defined here
...
LL |     check(m1::S); //~ ERROR expected value, found type alias `m1::S`
   |
   = note: can't use a type alias as a constructor
help: a tuple struct with a similar name exists
   |
   |
LL |     check(m1::TS); //~ ERROR expected value, found type alias `m1::S`
help: consider importing one of these items instead
   |
   |
LL | use m2::S;
   |
LL | use xm2::S;


error[E0423]: expected value, found type alias `xm1::S`
  ::: /checkout/src/test/ui/namespace/auxiliary/namespace-mix.rs:3:5
   |
LL |     pub struct TS();
LL |     pub struct TS();
   |     ---------------- similarly named tuple struct `TS` defined here
   |
   |
LL |     check(xm1::S); //~ ERROR expected value, found type alias `xm1::S`
   |
   = note: can't use a type alias as a constructor
help: a tuple struct with a similar name exists
   |
   |
LL |     check(xm1::TS); //~ ERROR expected value, found type alias `xm1::S`
help: consider importing one of these items instead
   |
   |
LL | use m2::S;
   |
LL | use xm2::S;


error[E0423]: expected value, found struct variant `m7::V`
   |
LL |         V {},
LL |         V {},
   |         ---- `m7::V` defined here
LL |         TV(),
   |         ---- similarly named tuple variant `TV` defined here
...
LL |     check(m7::V); //~ ERROR expected value, found struct variant `m7::V`
   |
help: use struct literal syntax instead
   |
   |
LL |     check(m7::V {}); //~ ERROR expected value, found struct variant `m7::V`
help: a tuple variant with a similar name exists
   |
   |
LL |     check(m7::TV); //~ ERROR expected value, found struct variant `m7::V`
help: consider importing one of these items instead
   |
   |
LL | use m8::V;
   |
LL | use xm8::V;


error[E0423]: expected value, found struct variant `xm7::V`
  ::: /checkout/src/test/ui/namespace/auxiliary/namespace-mix.rs:6:9
   |
LL |         V {},
LL |         V {},
   |         - `xm7::V` defined here
LL |         TV(),
   |         ---- similarly named tuple variant `TV` defined here
   |
   |
LL |     check(xm7::V); //~ ERROR expected value, found struct variant `xm7::V`
   |
help: use struct literal syntax instead
   |
   |
LL |     check(xm7::V { /* fields */ }); //~ ERROR expected value, found struct variant `xm7::V`
help: a tuple variant with a similar name exists
   |
   |
LL |     check(xm7::TV); //~ ERROR expected value, found struct variant `xm7::V`
help: consider importing one of these items instead
   |
   |
LL | use m8::V;
   |
LL | use xm8::V;


error[E0277]: the trait bound `c::Item: Impossible` is not satisfied
   |
   |
LL | fn check<T: Impossible>(_: T) {}
   |             ---------- required by this bound in `check`
...
LL |     check(m1::S{}); //~ ERROR c::Item
   |           ^^^^^^^ the trait `Impossible` is not implemented for `c::Item`

error[E0277]: the trait bound `c::S: Impossible` is not satisfied
   |
   |
LL | fn check<T: Impossible>(_: T) {}
   |             ---------- required by this bound in `check`
...
LL |     check(m2::S{}); //~ ERROR c::S
   |           ^^^^^^^ the trait `Impossible` is not implemented for `c::S`

error[E0277]: the trait bound `c::Item: Impossible` is not satisfied
   |
   |
LL | fn check<T: Impossible>(_: T) {}
   |             ---------- required by this bound in `check`
...
LL |     check(m2::S); //~ ERROR c::Item
   |           ^^^^^ the trait `Impossible` is not implemented for `c::Item`

error[E0277]: the trait bound `namespace_mix::c::Item: Impossible` is not satisfied
   |
   |
LL | fn check<T: Impossible>(_: T) {}
   |             ---------- required by this bound in `check`
...
LL |     check(xm1::S{}); //~ ERROR c::Item
   |           ^^^^^^^^ the trait `Impossible` is not implemented for `namespace_mix::c::Item`

error[E0277]: the trait bound `namespace_mix::c::S: Impossible` is not satisfied
   |
   |
LL | fn check<T: Impossible>(_: T) {}
   |             ---------- required by this bound in `check`
...
LL |     check(xm2::S{}); //~ ERROR c::S
   |           ^^^^^^^^ the trait `Impossible` is not implemented for `namespace_mix::c::S`

error[E0277]: the trait bound `namespace_mix::c::Item: Impossible` is not satisfied
   |
   |
LL | fn check<T: Impossible>(_: T) {}
   |             ---------- required by this bound in `check`
...
LL |     check(xm2::S); //~ ERROR c::Item
   |           ^^^^^^ the trait `Impossible` is not implemented for `namespace_mix::c::Item`

error[E0277]: the trait bound `c::Item: Impossible` is not satisfied
   |
   |
LL | fn check<T: Impossible>(_: T) {}
   |             ---------- required by this bound in `check`
...
LL |     check(m3::TS{}); //~ ERROR c::Item
   |           ^^^^^^^^ the trait `Impossible` is not implemented for `c::Item`

error[E0277]: the trait bound `fn() -> c::TS {c::TS}: Impossible` is not satisfied
   |
   |
LL | fn check<T: Impossible>(_: T) {}
   |             ---------- required by this bound in `check`
...
LL |     check(m3::TS); //~ ERROR c::TS
   |           ^^^^^^ the trait `Impossible` is not implemented for `fn() -> c::TS {c::TS}`

error[E0277]: the trait bound `c::TS: Impossible` is not satisfied
   |
   |
LL | fn check<T: Impossible>(_: T) {}
   |             ---------- required by this bound in `check`
...
LL |     check(m4::TS{}); //~ ERROR c::TS
   |           ^^^^^^^^ the trait `Impossible` is not implemented for `c::TS`

error[E0277]: the trait bound `c::Item: Impossible` is not satisfied
   |
   |
LL | fn check<T: Impossible>(_: T) {}
   |             ---------- required by this bound in `check`
...
LL |     check(m4::TS); //~ ERROR c::Item
   |           ^^^^^^ the trait `Impossible` is not implemented for `c::Item`

error[E0277]: the trait bound `namespace_mix::c::Item: Impossible` is not satisfied
   |
   |
LL | fn check<T: Impossible>(_: T) {}
   |             ---------- required by this bound in `check`
...
LL |     check(xm3::TS{}); //~ ERROR c::Item
   |           ^^^^^^^^^ the trait `Impossible` is not implemented for `namespace_mix::c::Item`

error[E0277]: the trait bound `fn() -> namespace_mix::c::TS {namespace_mix::c::TS}: Impossible` is not satisfied
   |
   |
LL | fn check<T: Impossible>(_: T) {}
   |             ---------- required by this bound in `check`
...
LL |     check(xm3::TS); //~ ERROR c::TS
   |           ^^^^^^^ the trait `Impossible` is not implemented for `fn() -> namespace_mix::c::TS {namespace_mix::c::TS}`

error[E0277]: the trait bound `namespace_mix::c::TS: Impossible` is not satisfied
   |
   |
LL | fn check<T: Impossible>(_: T) {}
   |             ---------- required by this bound in `check`
...
LL |     check(xm4::TS{}); //~ ERROR c::TS
   |           ^^^^^^^^^ the trait `Impossible` is not implemented for `namespace_mix::c::TS`

error[E0277]: the trait bound `namespace_mix::c::Item: Impossible` is not satisfied
   |
   |
LL | fn check<T: Impossible>(_: T) {}
   |             ---------- required by this bound in `check`
...
LL |     check(xm4::TS); //~ ERROR c::Item
   |           ^^^^^^^ the trait `Impossible` is not implemented for `namespace_mix::c::Item`

error[E0277]: the trait bound `c::Item: Impossible` is not satisfied
   |
   |
LL | fn check<T: Impossible>(_: T) {}
   |             ---------- required by this bound in `check`
...
LL |     check(m5::US{}); //~ ERROR c::Item
   |           ^^^^^^^^ the trait `Impossible` is not implemented for `c::Item`

error[E0277]: the trait bound `c::US: Impossible` is not satisfied
   |
   |
LL | fn check<T: Impossible>(_: T) {}
   |             ---------- required by this bound in `check`
...
LL |     check(m5::US); //~ ERROR c::US
   |           ^^^^^^ the trait `Impossible` is not implemented for `c::US`

error[E0277]: the trait bound `c::US: Impossible` is not satisfied
   |
   |
LL | fn check<T: Impossible>(_: T) {}
   |             ---------- required by this bound in `check`
...
LL |     check(m6::US{}); //~ ERROR c::US
   |           ^^^^^^^^ the trait `Impossible` is not implemented for `c::US`

error[E0277]: the trait bound `c::Item: Impossible` is not satisfied
   |
   |
LL | fn check<T: Impossible>(_: T) {}
   |             ---------- required by this bound in `check`
...
LL |     check(m6::US); //~ ERROR c::Item
   |           ^^^^^^ the trait `Impossible` is not implemented for `c::Item`

error[E0277]: the trait bound `namespace_mix::c::Item: Impossible` is not satisfied
   |
   |
LL | fn check<T: Impossible>(_: T) {}
   |             ---------- required by this bound in `check`
...
LL |     check(xm5::US{}); //~ ERROR c::Item
   |           ^^^^^^^^^ the trait `Impossible` is not implemented for `namespace_mix::c::Item`

error[E0277]: the trait bound `namespace_mix::c::US: Impossible` is not satisfied
   |
   |
LL | fn check<T: Impossible>(_: T) {}
   |             ---------- required by this bound in `check`
...
LL |     check(xm5::US); //~ ERROR c::US
   |           ^^^^^^^ the trait `Impossible` is not implemented for `namespace_mix::c::US`

error[E0277]: the trait bound `namespace_mix::c::US: Impossible` is not satisfied
   |
   |
LL | fn check<T: Impossible>(_: T) {}
   |             ---------- required by this bound in `check`
...
LL |     check(xm6::US{}); //~ ERROR c::US
   |           ^^^^^^^^^ the trait `Impossible` is not implemented for `namespace_mix::c::US`

error[E0277]: the trait bound `namespace_mix::c::Item: Impossible` is not satisfied
   |
   |
LL | fn check<T: Impossible>(_: T) {}
   |             ---------- required by this bound in `check`
...
LL |     check(xm6::US); //~ ERROR c::Item
   |           ^^^^^^^ the trait `Impossible` is not implemented for `namespace_mix::c::Item`

error[E0277]: the trait bound `c::Item: Impossible` is not satisfied
   |
   |
LL | fn check<T: Impossible>(_: T) {}
   |             ---------- required by this bound in `check`
...
LL |     check(m7::V{}); //~ ERROR c::Item
   |           ^^^^^^^ the trait `Impossible` is not implemented for `c::Item`

error[E0277]: the trait bound `c::E: Impossible` is not satisfied
   |
   |
LL | fn check<T: Impossible>(_: T) {}
   |             ---------- required by this bound in `check`
...
LL |     check(m8::V{}); //~ ERROR c::E
   |           ^^^^^^^ the trait `Impossible` is not implemented for `c::E`

error[E0277]: the trait bound `c::Item: Impossible` is not satisfied
   |
   |
LL | fn check<T: Impossible>(_: T) {}
   |             ---------- required by this bound in `check`
...
LL |     check(m8::V); //~ ERROR c::Item
   |           ^^^^^ the trait `Impossible` is not implemented for `c::Item`

error[E0277]: the trait bound `namespace_mix::c::Item: Impossible` is not satisfied
   |
   |
LL | fn check<T: Impossible>(_: T) {}
   |             ---------- required by this bound in `check`
...
LL |     check(xm7::V{}); //~ ERROR c::Item
   |           ^^^^^^^^ the trait `Impossible` is not implemented for `namespace_mix::c::Item`

error[E0277]: the trait bound `namespace_mix::c::E: Impossible` is not satisfied
   |
   |
LL | fn check<T: Impossible>(_: T) {}
   |             ---------- required by this bound in `check`
...
LL |     check(xm8::V{}); //~ ERROR c::E
   |           ^^^^^^^^ the trait `Impossible` is not implemented for `namespace_mix::c::E`

error[E0277]: the trait bound `namespace_mix::c::Item: Impossible` is not satisfied
   |
   |
LL | fn check<T: Impossible>(_: T) {}
   |             ---------- required by this bound in `check`
...
LL |     check(xm8::V); //~ ERROR c::Item
   |           ^^^^^^ the trait `Impossible` is not implemented for `namespace_mix::c::Item`

error[E0277]: the trait bound `c::Item: Impossible` is not satisfied
   |
   |
LL | fn check<T: Impossible>(_: T) {}
   |             ---------- required by this bound in `check`
...
LL |     check(m9::TV{}); //~ ERROR c::Item
   |           ^^^^^^^^ the trait `Impossible` is not implemented for `c::Item`

error[E0277]: the trait bound `fn() -> c::E {c::E::TV}: Impossible` is not satisfied
   |
   |
LL | fn check<T: Impossible>(_: T) {}
   |             ---------- required by this bound in `check`
...
LL |     check(m9::TV); //~ ERROR c::E
   |           ^^^^^^ the trait `Impossible` is not implemented for `fn() -> c::E {c::E::TV}`

error[E0277]: the trait bound `c::E: Impossible` is not satisfied
   |
   |
LL | fn check<T: Impossible>(_: T) {}
   |             ---------- required by this bound in `check`
...
LL |     check(mA::TV{}); //~ ERROR c::E
   |           ^^^^^^^^ the trait `Impossible` is not implemented for `c::E`

error[E0277]: the trait bound `c::Item: Impossible` is not satisfied
   |
   |
LL | fn check<T: Impossible>(_: T) {}
   |             ---------- required by this bound in `check`
...
LL |     check(mA::TV); //~ ERROR c::Item
   |           ^^^^^^ the trait `Impossible` is not implemented for `c::Item`

error[E0277]: the trait bound `namespace_mix::c::Item: Impossible` is not satisfied
   |
   |
LL | fn check<T: Impossible>(_: T) {}
   |             ---------- required by this bound in `check`
...
LL |     check(xm9::TV{}); //~ ERROR c::Item
   |           ^^^^^^^^^ the trait `Impossible` is not implemented for `namespace_mix::c::Item`

error[E0277]: the trait bound `fn() -> namespace_mix::c::E {namespace_mix::xm7::TV}: Impossible` is not satisfied
   |
   |
LL | fn check<T: Impossible>(_: T) {}
   |             ---------- required by this bound in `check`
...
LL |     check(xm9::TV); //~ ERROR c::E
   |           ^^^^^^^ the trait `Impossible` is not implemented for `fn() -> namespace_mix::c::E {namespace_mix::xm7::TV}`

error[E0277]: the trait bound `namespace_mix::c::E: Impossible` is not satisfied
   |
   |
LL | fn check<T: Impossible>(_: T) {}
   |             ---------- required by this bound in `check`
...
LL |     check(xmA::TV{}); //~ ERROR c::E
   |           ^^^^^^^^^ the trait `Impossible` is not implemented for `namespace_mix::c::E`

error[E0277]: the trait bound `namespace_mix::c::Item: Impossible` is not satisfied
   |
   |
LL | fn check<T: Impossible>(_: T) {}
   |             ---------- required by this bound in `check`
...
LL |     check(xmA::TV); //~ ERROR c::Item
   |           ^^^^^^^ the trait `Impossible` is not implemented for `namespace_mix::c::Item`

error[E0277]: the trait bound `c::Item: Impossible` is not satisfied
   |
   |
LL | fn check<T: Impossible>(_: T) {}
   |             ---------- required by this bound in `check`
...
LL |     check(mB::UV{}); //~ ERROR c::Item
   |           ^^^^^^^^ the trait `Impossible` is not implemented for `c::Item`

error[E0277]: the trait bound `c::E: Impossible` is not satisfied
   |
   |
LL | fn check<T: Impossible>(_: T) {}
   |             ---------- required by this bound in `check`
...
LL |     check(mB::UV); //~ ERROR c::E
   |           ^^^^^^ the trait `Impossible` is not implemented for `c::E`
---

---- [ui] ui/proc-macro/resolve-error.rs stdout ----
diff of stderr:

1 error: cannot find macro `bang_proc_macrp` in this scope
-   --> $DIR/resolve-error.rs:60:5
-    |
- LL |     bang_proc_macrp!();
-    |     ^^^^^^^^^^^^^^^ help: a macro with a similar name exists: `bang_proc_macro`
7   ::: $DIR/auxiliary/test-macros.rs:15:1
8    |


9 LL | pub fn empty(_: TokenStream) -> TokenStream {
10    | ------------------------------------------- similarly named macro `bang_proc_macro` defined here
+    |
+    |
+ LL |     bang_proc_macrp!();
+    |     ^^^^^^^^^^^^^^^ help: a macro with a similar name exists: `bang_proc_macro`
11 
12 error: cannot find macro `Dlona` in this scope

46    |          ^^^^^^^^^^^^^^^
47 
47 
48 error: cannot find derive macro `Dlona` in this scope
-   --> $DIR/resolve-error.rs:40:10
-    |
- LL | #[derive(Dlona)]
-    |          ^^^^^ help: a derive macro with a similar name exists: `Clona`
53    | 
54   ::: $DIR/auxiliary/derive-clona.rs:11:1


56 LL | pub fn derive_clonea(input: TokenStream) -> TokenStream {
57    | ------------------------------------------------------- similarly named derive macro `Clona` defined here
- 
- error: cannot find derive macro `Dlona` in this scope
61    |
61    |
62 LL | #[derive(Dlona)]

63    |          ^^^^^ help: a derive macro with a similar name exists: `Clona`
+ 
+ error: cannot find derive macro `Dlona` in this scope
64    | 
65   ::: $DIR/auxiliary/derive-clona.rs:11:1


67 LL | pub fn derive_clonea(input: TokenStream) -> TokenStream {
68    | ------------------------------------------------------- similarly named derive macro `Clona` defined here
+    |
+    |
+ LL | #[derive(Dlona)]
+    |          ^^^^^ help: a derive macro with a similar name exists: `Clona`
69 
70 error: cannot find derive macro `Dlone` in this scope
-   --> $DIR/resolve-error.rs:35:10
-    |
- LL | #[derive(Dlone)]
-    |          ^^^^^ help: a derive macro with a similar name exists: `Clone`
75    | 
76   ::: $SRC_DIR/core/src/clone.rs:LL:COL


78 LL | pub macro Clone($item:item) {
79    | --------------------------- similarly named derive macro `Clone` defined here
- 
- error: cannot find derive macro `Dlone` in this scope
83    |
83    |
84 LL | #[derive(Dlone)]

85    |          ^^^^^ help: a derive macro with a similar name exists: `Clone`
+ 
+ error: cannot find derive macro `Dlone` in this scope
86    | 
87   ::: $SRC_DIR/core/src/clone.rs:LL:COL


89 LL | pub macro Clone($item:item) {
90    | --------------------------- similarly named derive macro `Clone` defined here
+    |
+    |
+ LL | #[derive(Dlone)]
+    |          ^^^^^ help: a derive macro with a similar name exists: `Clone`
91 
92 error: cannot find attribute `FooWithLongNan` in this scope

96    |   ^^^^^^^^^^^^^^
97 
97 
98 error: cannot find attribute `attr_proc_macra` in this scope
-   --> $DIR/resolve-error.rs:28:3
-    |
- LL | #[attr_proc_macra]
-    |   ^^^^^^^^^^^^^^^ help: an attribute macro with a similar name exists: `attr_proc_macro`
104   ::: $DIR/auxiliary/test-macros.rs:20:1
105    |


106 LL | pub fn empty_attr(_: TokenStream, _: TokenStream) -> TokenStream {
107    | ---------------------------------------------------------------- similarly named attribute macro `attr_proc_macro` defined here
+    |
+    |
+ LL | #[attr_proc_macra]
+    |   ^^^^^^^^^^^^^^^ help: an attribute macro with a similar name exists: `attr_proc_macro`
108 
109 error: cannot find derive macro `FooWithLongNan` in this scope
-   --> $DIR/resolve-error.rs:22:10
-    |
- LL | #[derive(FooWithLongNan)]
-    |          ^^^^^^^^^^^^^^ help: a derive macro with a similar name exists: `FooWithLongName`
114    | 
115   ::: $DIR/auxiliary/derive-foo.rs:11:1


117 LL | pub fn derive_foo(input: TokenStream) -> TokenStream {
118    | ---------------------------------------------------- similarly named derive macro `FooWithLongName` defined here
- 
- error: cannot find derive macro `FooWithLongNan` in this scope
122    |
122    |
123 LL | #[derive(FooWithLongNan)]

124    |          ^^^^^^^^^^^^^^ help: a derive macro with a similar name exists: `FooWithLongName`
+ 
+ error: cannot find derive macro `FooWithLongNan` in this scope
125    | 
126   ::: $DIR/auxiliary/derive-foo.rs:11:1


128 LL | pub fn derive_foo(input: TokenStream) -> TokenStream {
129    | ---------------------------------------------------- similarly named derive macro `FooWithLongName` defined here
+    |
+    |
+ LL | #[derive(FooWithLongNan)]
+    |          ^^^^^^^^^^^^^^ help: a derive macro with a similar name exists: `FooWithLongName`
131 error: aborting due to 14 previous errors
132 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/resolve-error/resolve-error.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args proc-macro/resolve-error.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/resolve-error.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/resolve-error" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/resolve-error/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: cannot find macro `bang_proc_macrp` in this scope
  ::: /checkout/src/test/ui/proc-macro/auxiliary/test-macros.rs:15:1
   |
   |
LL | pub fn empty(_: TokenStream) -> TokenStream {
   | ------------------------------------------- similarly named macro `bang_proc_macro` defined here
   |
   |
LL |     bang_proc_macrp!();
   |     ^^^^^^^^^^^^^^^ help: a macro with a similar name exists: `bang_proc_macro`

error: cannot find macro `Dlona` in this scope
   |
   |
LL |     Dlona!();


error: cannot find macro `attr_proc_macra` in this scope
   |
   |
LL | macro_rules! attr_proc_mac {
   | -------------------------- similarly named macro `attr_proc_mac` defined here
...
LL |     attr_proc_macra!();
   |     ^^^^^^^^^^^^^^^ help: a macro with a similar name exists: `attr_proc_mac`

error: cannot find macro `FooWithLongNama` in this scope
   |
   |
LL | macro_rules! FooWithLongNam {
   | --------------------------- similarly named macro `FooWithLongNam` defined here
...
LL |     FooWithLongNama!();
   |     ^^^^^^^^^^^^^^^ help: a macro with a similar name exists: `FooWithLongNam`

error: cannot find derive macro `attr_proc_macra` in this scope
   |
   |
LL | #[derive(attr_proc_macra)]


error: cannot find derive macro `attr_proc_macra` in this scope
   |
   |
LL | #[derive(attr_proc_macra)]


error: cannot find derive macro `Dlona` in this scope
   | 
  ::: /checkout/src/test/ui/proc-macro/auxiliary/derive-clona.rs:11:1
   |
LL | pub fn derive_clonea(input: TokenStream) -> TokenStream {
   | ------------------------------------------------------- similarly named derive macro `Clona` defined here
   |
   |
LL | #[derive(Dlona)]
   |          ^^^^^ help: a derive macro with a similar name exists: `Clona`

error: cannot find derive macro `Dlona` in this scope
   | 
  ::: /checkout/src/test/ui/proc-macro/auxiliary/derive-clona.rs:11:1
   |
LL | pub fn derive_clonea(input: TokenStream) -> TokenStream {
   | ------------------------------------------------------- similarly named derive macro `Clona` defined here
   |
   |
LL | #[derive(Dlona)]
   |          ^^^^^ help: a derive macro with a similar name exists: `Clona`

error: cannot find derive macro `Dlone` in this scope
  ::: /checkout/library/core/src/clone.rs:139:1
   |
   |
LL | pub macro Clone($item:item) {
   | --------------------------- similarly named derive macro `Clone` defined here
   |
   |
LL | #[derive(Dlone)]
   |          ^^^^^ help: a derive macro with a similar name exists: `Clone`

error: cannot find derive macro `Dlone` in this scope
  ::: /checkout/library/core/src/clone.rs:139:1
   |
   |
LL | pub macro Clone($item:item) {
   | --------------------------- similarly named derive macro `Clone` defined here
   |
   |
LL | #[derive(Dlone)]
   |          ^^^^^ help: a derive macro with a similar name exists: `Clone`

error: cannot find attribute `FooWithLongNan` in this scope
   |
   |
LL | #[FooWithLongNan] //~ ERROR cannot find attribute `FooWithLongNan` in this scope


error: cannot find attribute `attr_proc_macra` in this scope
  ::: /checkout/src/test/ui/proc-macro/auxiliary/test-macros.rs:20:1
   |
   |
LL | pub fn empty_attr(_: TokenStream, _: TokenStream) -> TokenStream {
   | ---------------------------------------------------------------- similarly named attribute macro `attr_proc_macro` defined here
   |
   |
LL | #[attr_proc_macra] //~ ERROR cannot find attribute `attr_proc_macra` in this scope
   |   ^^^^^^^^^^^^^^^ help: an attribute macro with a similar name exists: `attr_proc_macro`

error: cannot find derive macro `FooWithLongNan` in this scope
  ::: /checkout/src/test/ui/proc-macro/auxiliary/derive-foo.rs:11:1
   |
   |
LL | pub fn derive_foo(input: TokenStream) -> TokenStream {
   | ---------------------------------------------------- similarly named derive macro `FooWithLongName` defined here
   |
   |
LL | #[derive(FooWithLongNan)]
   |          ^^^^^^^^^^^^^^ help: a derive macro with a similar name exists: `FooWithLongName`

error: cannot find derive macro `FooWithLongNan` in this scope
  ::: /checkout/src/test/ui/proc-macro/auxiliary/derive-foo.rs:11:1
   |
   |
LL | pub fn derive_foo(input: TokenStream) -> TokenStream {
   | ---------------------------------------------------- similarly named derive macro `FooWithLongName` defined here
   |
   |
LL | #[derive(FooWithLongNan)]
   |          ^^^^^^^^^^^^^^ help: a derive macro with a similar name exists: `FooWithLongName`
error: aborting due to 14 previous errors


------------------------------------------
------------------------------------------


---- [ui] ui/resolve/issue-19452.rs stdout ----
diff of stderr:

8    |                  ^^^^^^^^^^^^^^ help: use struct literal syntax instead: `Homura::Madoka { age: val }`
9 
10 error[E0423]: expected value, found struct variant `issue_19452_aux::Homura::Madoka`
-   --> $DIR/issue-19452.rs:13:18
-    |
- LL |     let homura = issue_19452_aux::Homura::Madoka;
-    |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use struct literal syntax instead: `issue_19452_aux::Homura::Madoka { /* fields */ }`
15    | 
16   ::: $DIR/auxiliary/issue-19452-aux.rs:2:5


18 LL |     Madoka { age: u32 }
19    |     ------ `issue_19452_aux::Homura::Madoka` defined here
+    |
+    |
+ LL |     let homura = issue_19452_aux::Homura::Madoka;
+    |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use struct literal syntax instead: `issue_19452_aux::Homura::Madoka { /* fields */ }`
21 error: aborting due to 2 previous errors
22 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/issue-19452/issue-19452.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args resolve/issue-19452.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/resolve/issue-19452.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/issue-19452" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/issue-19452/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0423]: expected value, found struct variant `Homura::Madoka`
   |
   |
LL |     Madoka { age: u32 }
   |     ------------------- `Homura::Madoka` defined here
...
LL |     let homura = Homura::Madoka;
   |                  ^^^^^^^^^^^^^^ help: use struct literal syntax instead: `Homura::Madoka { age: val }`

error[E0423]: expected value, found struct variant `issue_19452_aux::Homura::Madoka`
  ::: /checkout/src/test/ui/resolve/auxiliary/issue-19452-aux.rs:2:5
   |
   |
LL |     Madoka { age: u32 }
   |     ------ `issue_19452_aux::Homura::Madoka` defined here
   |
   |
LL |     let homura = issue_19452_aux::Homura::Madoka;
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use struct literal syntax instead: `issue_19452_aux::Homura::Madoka { /* fields */ }`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0423`.


------------------------------------------


---- [ui] ui/resolve/levenshtein.rs stdout ----
diff of stderr:

14    |          ^^^ help: an enum with a similar name exists: `Bar`
15 
16 error[E0412]: cannot find type `Opiton` in this scope
-   --> $DIR/levenshtein.rs:12:10
-    |
- LL | type B = Opiton<u8>; // Misspelled type name from the prelude.
-    |          ^^^^^^ help: an enum with a similar name exists: `Option`
21    | 
22   ::: $SRC_DIR/core/src/option.rs:LL:COL


24 LL | pub enum Option<T> {
25    | ------------------ similarly named enum `Option` defined here
+   --> $DIR/levenshtein.rs:12:10
+    |
+ LL | type B = Opiton<u8>; // Misspelled type name from the prelude.
+    |          ^^^^^^ help: an enum with a similar name exists: `Option`
26 
27 error[E0412]: cannot find type `Baz` in this scope
28   --> $DIR/levenshtein.rs:16:14

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/levenshtein/levenshtein.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/levenshtein/levenshtein.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args resolve/levenshtein.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/resolve/levenshtein.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/levenshtein" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/levenshtein/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0412]: cannot find type `esize` in this scope
   |
   |
LL | fn foo(c: esize) {} // Misspelled primitive type name.
   |           ^^^^^ help: a builtin type with a similar name exists: `isize`

error[E0412]: cannot find type `Baz` in this scope
   |
   |
LL | enum Bar { }
   | -------- similarly named enum `Bar` defined here
LL | 
LL | type A = Baz; // Misspelled type name.
   |          ^^^ help: an enum with a similar name exists: `Bar`

error[E0412]: cannot find type `Opiton` in this scope
  ::: /checkout/library/core/src/option.rs:161:1
   |
   |
LL | pub enum Option<T> {
   | ------------------ similarly named enum `Option` defined here
   |
   |
LL | type B = Opiton<u8>; // Misspelled type name from the prelude.
   |          ^^^^^^ help: an enum with a similar name exists: `Option`

error[E0412]: cannot find type `Baz` in this scope
   |
   |
LL |     type A = Baz; // No suggestion here, Bar is not visible


error[E0425]: cannot find value `MAXITEM` in this scope
   |
LL | const MAX_ITEM: usize = 10;
LL | const MAX_ITEM: usize = 10;
   | --------------------------- similarly named constant `MAX_ITEM` defined here
...
LL |     let v = [0u32; MAXITEM]; // Misspelled constant name.
   |                    ^^^^^^^ help: a constant with a similar name exists: `MAX_ITEM`

error[E0425]: cannot find function `foobar` in this scope
   |
   |
LL | fn foo_bar() {}
   | ------------ similarly named function `foo_bar` defined here
...
LL |     foobar(); // Misspelled function name.
   |     ^^^^^^ help: a function with a similar name exists: `foo_bar`

error[E0412]: cannot find type `first` in module `m`
   |
LL |     pub struct First;
   |     ----------------- similarly named struct `First` defined here
...
...
LL |     let b: m::first = m::second; // Misspelled item in module.
   |               ^^^^^ help: a struct with a similar name exists (notice the capitalization): `First`

error[E0425]: cannot find value `second` in module `m`
   |
LL |     pub struct Second;
LL |     pub struct Second;
   |     ------------------ similarly named unit struct `Second` defined here
...
LL |     let b: m::first = m::second; // Misspelled item in module.
   |                          ^^^^^^ help: a unit struct with a similar name exists (notice the capitalization): `Second`
error: aborting due to 8 previous errors

Some errors have detailed explanations: E0412, E0425.
For more information about an error, try `rustc --explain E0412`.
For more information about an error, try `rustc --explain E0412`.

------------------------------------------


---- [ui] ui/rfc-1937-termination-trait/termination-trait-test-wrong-type.rs stdout ----
diff of stderr:

1 error[E0277]: `main` has invalid return type `Result<f32, ParseFloatError>`
+    | 
+   ::: $SRC_DIR/test/src/lib.rs:LL:COL
+    |
+ LL |   pub fn assert_test_result<T: Termination>(result: T) {
+    |                                ----------- required by this bound in `assert_test_result`
3    |
3    |
4 LL | / fn can_parse_zero_as_f32() -> Result<f32, ParseFloatError> {

5 LL | |     "0".parse()
6 LL | | }
7    | |_^ `main` can only return types that implement `Termination`
-    | 
-   ::: $SRC_DIR/test/src/lib.rs:LL:COL
-    |
- LL |   pub fn assert_test_result<T: Termination>(result: T) {
-    |                                ----------- required by this bound in `assert_test_result`
13    |
14    = help: the trait `Termination` is not implemented for `Result<f32, ParseFloatError>`
15    = note: this error originates in an attribute macro (in Nightly builds, run with -Z macro-backtrace for more info)

The actual stderr differed from the expected stderr.
---
diff of stderr:

11    |   ^^^^^^^^^ help: a built-in attribute with a similar name exists: `rustc_error`
12 
13 error: cannot find attribute `tests` in this scope
-   --> $DIR/attribute-typos.rs:4:3
-    |
- LL | #[tests]
-    |   ^^^^^ help: an attribute macro with a similar name exists: `test`
18    | 
19   ::: $SRC_DIR/core/src/macros/mod.rs:LL:COL


21 LL |     pub macro test($item:item) {
22    |     -------------------------- similarly named attribute macro `test` defined here
+    |
+    |
+ LL | #[tests]
+    |   ^^^^^ help: an attribute macro with a similar name exists: `test`
23 
24 error: cannot find attribute `deprcated` in this scope


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/attribute-typos/attribute-typos.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/attribute-typos/attribute-typos.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args suggestions/attribute-typos.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/attribute-typos.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/attribute-typos" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/attribute-typos/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: attributes starting with `rustc` are reserved for use by the `rustc` compiler
   |
   |
LL | #[rustc_err]

error: cannot find attribute `rustc_err` in this scope
  --> /checkout/src/test/ui/suggestions/attribute-typos.rs:7:3
   |
   |
LL | #[rustc_err]
   |   ^^^^^^^^^ help: a built-in attribute with a similar name exists: `rustc_error`

error: cannot find attribute `tests` in this scope
  ::: /checkout/library/core/src/macros/mod.rs:1405:5
   |
   |
LL |     pub macro test($item:item) {
   |     -------------------------- similarly named attribute macro `test` defined here
   |
   |
LL | #[tests] //~ ERROR cannot find attribute `tests` in this scope
   |   ^^^^^ help: an attribute macro with a similar name exists: `test`

error: cannot find attribute `deprcated` in this scope
   |
   |
LL | #[deprcated] //~ ERROR cannot find attribute `deprcated` in this scope
   |   ^^^^^^^^^ help: a built-in attribute with a similar name exists: `deprecated`
error: aborting due to 4 previous errors


------------------------------------------
---

1 error[E0573]: expected type, found module `result`
-   --> $DIR/do-not-attempt-to-add-suggestions-with-no-changes.rs:2:6
-    |
- LL | impl result {
-    |      ^^^^^^ help: an enum with a similar name exists: `Result`
6    | 
7   ::: $SRC_DIR/core/src/result.rs:LL:COL


9 LL | pub enum Result<T, E> {
10    | --------------------- similarly named enum `Result` defined here
+    |
+ LL | impl result {
+ LL | impl result {
+    |      ^^^^^^ help: an enum with a similar name exists: `Result`
12 error[E0573]: expected type, found variant `Err`
13   --> $DIR/do-not-attempt-to-add-suggestions-with-no-changes.rs:3:25



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/do-not-attempt-to-add-suggestions-with-no-changes/do-not-attempt-to-add-suggestions-with-no-changes.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args suggestions/do-not-attempt-to-add-suggestions-with-no-changes.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/do-not-attempt-to-add-suggestions-with-no-changes.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/do-not-attempt-to-add-suggestions-with-no-changes" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/do-not-attempt-to-add-suggestions-with-no-changes/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0573]: expected type, found module `result`
   | 
  ::: /checkout/library/core/src/result.rs:241:1
   |
LL | pub enum Result<T, E> {
   | --------------------- similarly named enum `Result` defined here
   |
   |
LL | impl result { //~ ERROR expected type, found module `result`
   |      ^^^^^^ help: an enum with a similar name exists: `Result`
error[E0573]: expected type, found variant `Err`
  --> /checkout/src/test/ui/suggestions/do-not-attempt-to-add-suggestions-with-no-changes.rs:3:25
   |
   |
LL |     fn into_future() -> Err {} //~ ERROR expected type, found variant `Err`

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0573`.
For more information about this error, try `rustc --explain E0573`.

------------------------------------------


---- [ui] ui/suggestions/imm-ref-trait-object.rs stdout ----
diff of stderr:

1 error: the `min` method cannot be invoked on a trait object
-   --> $DIR/imm-ref-trait-object.rs:2:8
-    |
- LL |      t.min().unwrap()
6    | 
6    | 
7   ::: $SRC_DIR/core/src/iter/traits/iterator.rs:LL:COL

9 LL |         Self: Sized,
9 LL |         Self: Sized,
10    |               ----- this has a `Sized` requirement
+   --> $DIR/imm-ref-trait-object.rs:2:8
+    |
+ LL |      t.min().unwrap()
11    |
11    |
12    = note: you need `&mut dyn Iterator<Item = &u64>` instead of `&dyn Iterator<Item = &u64>`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/imm-ref-trait-object/imm-ref-trait-object.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/imm-ref-trait-object/imm-ref-trait-object.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args suggestions/imm-ref-trait-object.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/imm-ref-trait-object.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/imm-ref-trait-object" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/imm-ref-trait-object/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: the `min` method cannot be invoked on a trait object
  ::: /checkout/library/core/src/iter/traits/iterator.rs:2603:15
   |
LL |         Self: Sized,
LL |         Self: Sized,
   |               ----- this has a `Sized` requirement
   |
   |
LL |      t.min().unwrap() //~ ERROR the `min` method cannot be invoked on a trait object
   |
   |
   = note: you need `&mut dyn Iterator<Item = &u64>` instead of `&dyn Iterator<Item = &u64>`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/suggestions/expected-boxed-future-isnt-pinned.rs stdout ----
diff of stderr:

59    = note: required by `Pin::<P>::new`
61 error[E0308]: mismatched types
+    | 
+    | 
+   ::: $SRC_DIR/core/src/future/mod.rs:LL:COL
+    |
+ LL |   pub const fn from_generator<T>(gen: T) -> impl Future<Output = T::Return>
+    |                                             ------------------------------- the found opaque type
63    |
63    |
64 LL |   fn zap() -> BoxFuture<'static, i32> {
67 LL | |         42
68 LL | |     }
68 LL | |     }
69    | |_____^ expected struct `Pin`, found opaque type
-    | 
-   ::: $SRC_DIR/core/src/future/mod.rs:LL:COL
-    |
- LL |   pub const fn from_generator<T>(gen: T) -> impl Future<Output = T::Return>
-    |                                             ------------------------------- the found opaque type
75    |
76    = note:   expected struct `Pin<Box<(dyn Future<Output = i32> + Send + 'static)>>`
77            found opaque type `impl Future`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/expected-boxed-future-isnt-pinned/expected-boxed-future-isnt-pinned.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/expected-boxed-future-isnt-pinned/expected-boxed-future-isnt-pinned.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args suggestions/expected-boxed-future-isnt-pinned.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/expected-boxed-future-isnt-pinned.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/expected-boxed-future-isnt-pinned" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/expected-boxed-future-isnt-pinned/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: mismatched types
  --> /checkout/src/test/ui/suggestions/expected-boxed-future-isnt-pinned.rs:11:5
   |
LL | fn foo<F: Future<Output=i32> + Send + 'static>(x: F) -> BoxFuture<'static, i32> {
   |        - this type parameter                            ----------------------- expected `Pin<Box<(dyn Future<Output = i32> + Send + 'static)>>` because of return type
LL |     // We could instead use an `async` block, but this way we have no std spans.
LL |     x //~ ERROR mismatched types
   |     |
   |     |
   |     expected struct `Pin`, found type parameter `F`
   |     help: you need to pin and box this expression: `Box::pin(x)`
   |
   = note:      expected struct `Pin<Box<(dyn Future<Output = i32> + Send + 'static)>>`
           found type parameter `F`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/suggestions/expected-boxed-future-isnt-pinned.rs:18:5
   |
   |
LL | fn bar<F: Future<Output=i32> + Send + 'static>(x: F) -> BoxFuture<'static, i32> {
   |                                                         ----------------------- expected `Pin<Box<(dyn Future<Output = i32> + Send + 'static)>>` because of return type
LL |     Box::new(x) //~ ERROR mismatched types
   |     ^^^^^^^^^^^ expected struct `Pin`, found struct `Box`
   |
   = note: expected struct `Pin<Box<(dyn Future<Output = i32> + Send + 'static)>>`
              found struct `Box<F>`
   = help: use `Box::pin`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/suggestions/expected-boxed-future-isnt-pinned.rs:22:14
   |
   |
LL | fn baz<F: Future<Output=i32> + Send + 'static>(x: F) -> BoxFuture<'static, i32> {
   |        - this type parameter
LL |     Pin::new(x) //~ ERROR mismatched types
   |              |
   |              |
   |              expected struct `Box`, found type parameter `F`
   |              help: store this in the heap by calling `Box::new`: `Box::new(x)`
   |
   = note:      expected struct `Box<dyn Future<Output = i32> + Send>`
           found type parameter `F`
   = note: for more on the distinction between the stack and the heap, read https://doc.rust-lang.org/book/ch15-01-box.html, https://doc.rust-lang.org/rust-by-example/std/box.html, and https://doc.rust-lang.org/std/boxed/index.html

error[E0277]: `dyn Future<Output = i32> + Send` cannot be unpinned
   |
   |
LL |     Pin::new(x) //~ ERROR mismatched types
   |     ^^^^^^^^ the trait `Unpin` is not implemented for `dyn Future<Output = i32> + Send`
   |
   = note: consider using `Box::pin`
   = note: required by `Pin::<P>::new`

error[E0277]: `dyn Future<Output = i32> + Send` cannot be unpinned
   |
   |
LL |     Pin::new(Box::new(x)) //~ ERROR E0277
   |     ^^^^^^^^ the trait `Unpin` is not implemented for `dyn Future<Output = i32> + Send`
   |
   = note: consider using `Box::pin`
   = note: required by `Pin::<P>::new`
error[E0308]: mismatched types
   | 
  ::: /checkout/library/core/src/future/mod.rs:61:43
   |
   |
LL |   pub const fn from_generator<T>(gen: T) -> impl Future<Output = T::Return>
   |                                             ------------------------------- the found opaque type
   |
   |
LL |   fn zap() -> BoxFuture<'static, i32> {
   |               ----------------------- expected `Pin<Box<(dyn Future<Output = i32> + Send + 'static)>>` because of return type
LL | /     async { //~ ERROR mismatched types
LL | |         42
LL | |     }
   | |_____^ expected struct `Pin`, found opaque type
   |
   = note:   expected struct `Pin<Box<(dyn Future<Output = i32> + Send + 'static)>>`
           found opaque type `impl Future`
help: you need to pin and box this expression
   |
LL |     Box::pin(async { //~ ERROR mismatched types
LL |     })
   |

error: aborting due to 6 previous errors
---

---- [ui] ui/suggestions/issue-81839.rs stdout ----
diff of stderr:

1 error[E0308]: `match` arms have incompatible types
+    | 
+   ::: $DIR/auxiliary/issue-81839.rs:6:49
+    |
+ LL |       pub async fn answer_str(&self, _s: &str) -> Test {
+    |                                                   ---- checked the `Output` of this `async fn`, found opaque type
3    |
3    |
4 LL | /     match num {

13    | |              ^^^^^^^^^^^^^^^^^^^ expected `()`, found opaque type
14 LL | |     }
15    | |_____- `match` arms have incompatible types
-    | 
-   ::: $DIR/auxiliary/issue-81839.rs:6:49
-    |
- LL |       pub async fn answer_str(&self, _s: &str) -> Test {
-    |                                                   ---- checked the `Output` of this `async fn`, found opaque type
21    |
22    = note: while checking the return type of the `async fn`
23    = note:     expected type `()`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-81839/issue-81839.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-81839/issue-81839.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args suggestions/issue-81839.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/issue-81839.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-81839" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-81839/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: `match` arms have incompatible types
  ::: /checkout/src/test/ui/suggestions/auxiliary/issue-81839.rs:6:49
   |
   |
LL |       pub async fn answer_str(&self, _s: &str) -> Test {
   |                                                   ---- checked the `Output` of this `async fn`, found opaque type
   |
   |
LL | /     match num {
LL | |         1 => {
LL | |             cx.answer_str("hi");
   | |             |                  |
   | |             |                  help: consider removing this semicolon
   | |             this is found to be of type `()`
LL | |         }
LL | |         }
LL | |         _ => cx.answer_str("hi"), //~ `match` arms have incompatible types
   | |              ^^^^^^^^^^^^^^^^^^^ expected `()`, found opaque type
LL | |     }
   | |_____- `match` arms have incompatible types
   |
   = note: while checking the return type of the `async fn`
   = note:     expected type `()`
           found opaque type `impl Future`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.

---

1 error[E0404]: expected trait, found struct `String`
-   --> $DIR/assoc_type_bound_with_struct.rs:5:46
-    |
- LL | struct Foo<T> where T: Bar, <T as Bar>::Baz: String {
-    |                                              ^^^^^^ not a trait
6    | 
7   ::: $SRC_DIR/alloc/src/string.rs:LL:COL


9 LL | pub trait ToString {
10    | ------------------ similarly named trait `ToString` defined here
11    |
11    |
+ LL | struct Foo<T> where T: Bar, <T as Bar>::Baz: String {
+    |                                              ^^^^^^ not a trait
12 help: constrain the associated type to `String`
13    |
13    |
14 LL | struct Foo<T> where T: Bar, T: Bar<Baz = String> {
19    |                                              ^^^^^^^^
20 
21 error[E0404]: expected trait, found struct `String`
-   --> $DIR/assoc_type_bound_with_struct.rs:9:54
-   --> $DIR/assoc_type_bound_with_struct.rs:9:54
-    |
- LL | struct Qux<'a, T> where T: Bar, <&'a T as Bar>::Baz: String {
-    |                                                      ^^^^^^ not a trait
26    | 
27   ::: $SRC_DIR/alloc/src/string.rs:LL:COL


29 LL | pub trait ToString {
30    | ------------------ similarly named trait `ToString` defined here
31    |
31    |
+ LL | struct Qux<'a, T> where T: Bar, <&'a T as Bar>::Baz: String {
+    |                                                      ^^^^^^ not a trait
32 help: constrain the associated type to `String`
33    |
33    |
34 LL | struct Qux<'a, T> where T: Bar, &'a T: Bar<Baz = String> {
39    |                                                      ^^^^^^^^
40 
41 error[E0404]: expected trait, found struct `String`
-   --> $DIR/assoc_type_bound_with_struct.rs:13:45
-   --> $DIR/assoc_type_bound_with_struct.rs:13:45
-    |
- LL | fn foo<T: Bar>(_: T) where <T as Bar>::Baz: String {
-    |                                             ^^^^^^ not a trait
46    | 
47   ::: $SRC_DIR/alloc/src/string.rs:LL:COL


49 LL | pub trait ToString {
50    | ------------------ similarly named trait `ToString` defined here
51    |
51    |
+ LL | fn foo<T: Bar>(_: T) where <T as Bar>::Baz: String {
+    |                                             ^^^^^^ not a trait
52 help: constrain the associated type to `String`
53    |
53    |
54 LL | fn foo<T: Bar>(_: T) where T: Bar<Baz = String> {
59    |                                             ^^^^^^^^
60 
61 error[E0404]: expected trait, found struct `String`
-   --> $DIR/assoc_type_bound_with_struct.rs:16:57
-   --> $DIR/assoc_type_bound_with_struct.rs:16:57
-    |
- LL | fn qux<'a, T: Bar>(_: &'a T) where <&'a T as Bar>::Baz: String {
-    |                                                         ^^^^^^ not a trait
66    | 
67   ::: $SRC_DIR/alloc/src/string.rs:LL:COL


69 LL | pub trait ToString {
70    | ------------------ similarly named trait `ToString` defined here
+    |
+    |
+ LL | fn qux<'a, T: Bar>(_: &'a T) where <&'a T as Bar>::Baz: String {
+    |                                                         ^^^^^^ not a trait
72 help: constrain the associated type to `String`
73    |



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/associated_type_bound/assoc_type_bound_with_struct/assoc_type_bound_with_struct.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/associated_type_bound/assoc_type_bound_with_struct.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/associated_type_bound/assoc_type_bound_with_struct.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/associated_type_bound/assoc_type_bound_with_struct" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/associated_type_bound/assoc_type_bound_with_struct/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0404]: expected trait, found struct `String`
   | 
  ::: /checkout/library/alloc/src/string.rs:2243:1
   |
LL | pub trait ToString {
   | ------------------ similarly named trait `ToString` defined here
   |
   |
LL | struct Foo<T> where T: Bar, <T as Bar>::Baz: String { //~ ERROR expected trait, found struct
   |                                              ^^^^^^ not a trait
help: constrain the associated type to `String`
   |
   |
LL | struct Foo<T> where T: Bar, T: Bar<Baz = String> { //~ ERROR expected trait, found struct
help: a trait with a similar name exists
   |
   |
LL | struct Foo<T> where T: Bar, <T as Bar>::Baz: ToString { //~ ERROR expected trait, found struct

error[E0404]: expected trait, found struct `String`
   | 
  ::: /checkout/library/alloc/src/string.rs:2243:1
  ::: /checkout/library/alloc/src/string.rs:2243:1
   |
LL | pub trait ToString {
   | ------------------ similarly named trait `ToString` defined here
   |
   |
LL | struct Qux<'a, T> where T: Bar, <&'a T as Bar>::Baz: String { //~ ERROR expected trait, found struct
   |                                                      ^^^^^^ not a trait
help: constrain the associated type to `String`
   |
   |
LL | struct Qux<'a, T> where T: Bar, &'a T: Bar<Baz = String> { //~ ERROR expected trait, found struct
help: a trait with a similar name exists
   |
   |
LL | struct Qux<'a, T> where T: Bar, <&'a T as Bar>::Baz: ToString { //~ ERROR expected trait, found struct

error[E0404]: expected trait, found struct `String`
   | 
  ::: /checkout/library/alloc/src/string.rs:2243:1
  ::: /checkout/library/alloc/src/string.rs:2243:1
   |
LL | pub trait ToString {
   | ------------------ similarly named trait `ToString` defined here
   |
   |
LL | fn foo<T: Bar>(_: T) where <T as Bar>::Baz: String { //~ ERROR expected trait, found struct
   |                                             ^^^^^^ not a trait
help: constrain the associated type to `String`
   |
   |
LL | fn foo<T: Bar>(_: T) where T: Bar<Baz = String> { //~ ERROR expected trait, found struct
help: a trait with a similar name exists
   |
   |
LL | fn foo<T: Bar>(_: T) where <T as Bar>::Baz: ToString { //~ ERROR expected trait, found struct

error[E0404]: expected trait, found struct `String`
   | 
  ::: /checkout/library/alloc/src/string.rs:2243:1
  ::: /checkout/library/alloc/src/string.rs:2243:1
   |
LL | pub trait ToString {
   | ------------------ similarly named trait `ToString` defined here
   |
   |
LL | fn qux<'a, T: Bar>(_: &'a T) where <&'a T as Bar>::Baz: String { //~ ERROR expected trait, found
   |                                                         ^^^^^^ not a trait
help: constrain the associated type to `String`
   |
   |
LL | fn qux<'a, T: Bar>(_: &'a T) where &'a T: Bar<Baz = String> { //~ ERROR expected trait, found
help: a trait with a similar name exists
   |
   |
LL | fn qux<'a, T: Bar>(_: &'a T) where <&'a T as Bar>::Baz: ToString { //~ ERROR expected trait, found

error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0404`.
For more information about this error, try `rustc --explain E0404`.

------------------------------------------


---- [ui] ui/traits/mutual-recursion-issue-75860.rs stdout ----
diff of stderr:

1 error[E0275]: overflow evaluating the requirement `Option<_>: Sized`
-   --> $DIR/mutual-recursion-issue-75860.rs:11:5
-    |
- LL |     iso(left, right)
6    | 
6    | 
7   ::: $SRC_DIR/core/src/option.rs:LL:COL


9 LL | pub enum Option<T> {
10    |                 - required by this bound in `Option`
+    |
+    |
+ LL |     iso(left, right)
11    |
11    |
12    = help: consider adding a `#![recursion_limit="256"]` attribute to your crate (`mutual_recursion_issue_75860`)


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/mutual-recursion-issue-75860/mutual-recursion-issue-75860.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/mutual-recursion-issue-75860/mutual-recursion-issue-75860.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/mutual-recursion-issue-75860.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/mutual-recursion-issue-75860.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/mutual-recursion-issue-75860" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/mutual-recursion-issue-75860/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0275]: overflow evaluating the requirement `Option<_>: Sized`
  ::: /checkout/library/core/src/option.rs:161:17
   |
   |
LL | pub enum Option<T> {
   |                 - required by this bound in `Option`
   |
LL |     iso(left, right)
   |     ^^^
   |
   |
   = help: consider adding a `#![recursion_limit="256"]` attribute to your crate (`mutual_recursion_issue_75860`)
error: aborting due to previous error

For more information about this error, try `rustc --explain E0275`.

---

1 error[E0423]: expected value, found struct `xcrate_unit_struct::StructWithFields`
-   --> $DIR/xcrate-unit-struct.rs:9:13
-    |
- LL |     let _ = xcrate_unit_struct::StructWithFields;
-    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use struct literal syntax instead: `xcrate_unit_struct::StructWithFields { foo: val }`
6    | 
7   ::: $DIR/auxiliary/xcrate_unit_struct.rs:20:1

9 LL | pub struct StructWithFields {
9 LL | pub struct StructWithFields {
10    | --------------------------- `xcrate_unit_struct::StructWithFields` defined here
+    |
+    |
+ LL |     let _ = xcrate_unit_struct::StructWithFields;
+    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use struct literal syntax instead: `xcrate_unit_struct::StructWithFields { foo: val }`
12 error: aborting due to previous error
13 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/xcrate/xcrate-unit-struct/xcrate-unit-struct.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args xcrate/xcrate-unit-struct.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/xcrate/xcrate-unit-struct.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/xcrate/xcrate-unit-struct" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/xcrate/xcrate-unit-struct/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0423]: expected value, found struct `xcrate_unit_struct::StructWithFields`
   | 
  ::: /checkout/src/test/ui/xcrate/auxiliary/xcrate_unit_struct.rs:20:1
LL | pub struct StructWithFields {
LL | pub struct StructWithFields {
   | --------------------------- `xcrate_unit_struct::StructWithFields` defined here
  --> /checkout/src/test/ui/xcrate/xcrate-unit-struct.rs:9:13
   |
LL |     let _ = xcrate_unit_struct::StructWithFields;
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use struct literal syntax instead: `xcrate_unit_struct::StructWithFields { foo: val }`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0423`.

---
test result: FAILED. 11589 passed; 33 failed; 96 ignored; 0 measured; 0 filtered out; finished in 138.65s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:13:44
