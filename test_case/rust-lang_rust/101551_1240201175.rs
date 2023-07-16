plain
..........................................................................iiiiiiiiiiiiii 176/13491
.....................i..................i............................................... 264/13491
........................................................................................ 352/13491
........................................................................................ 440/13491
.......................................FF.FF.....FF..................................... 528/13491
........................................................................................ 704/13491
.................................................................i...................... 792/13491
........................................................................i............... 880/13491
........................................................................................ 968/13491
........................................................................................ 968/13491
.........................................F.....F........................................ 1056/13491
........................................................................................ 1232/13491
.................................................................i...................... 1320/13491
........................................................................................ 1408/13491
....................................i................................................... 1496/13491
---
........................................................................................ 4840/13491
........................................................................................ 4928/13491
........................................................................................ 5016/13491
........................................................................................ 5104/13491
............................................................F.F......................... 5192/13491
.......F....i....................................................................i...... 5280/13491
........................................................................................ 5456/13491
........................................................................................ 5544/13491
........................................................................................ 5632/13491
........................................................................................ 5720/13491
---
........................................................................................ 6248/13491
........................................................................................ 6336/13491
......................................................................i................. 6424/13491
........................................................................................ 6512/13491
.F................................................................i................F.... 6600/13491
...........................................i............................................ 6776/13491
...........................................i............................................ 6776/13491
............ii.ii........i....i..................................F.....F................ 6864/13491
......i.....................................................................F........... 6952/13491
...........................i..................i.............i........................... 7128/13491
...............................i........................................................ 7216/13491
....................................................i................................... 7304/13491
........................................................................................ 7392/13491
---
........................i............................................................... 10912/13491
..................................iiiiii.i..iiiiii.i.................................... 11000/13491
........................................................................................ 11088/13491
........................................................................................ 11176/13491
..........................................F......FF..................................... 11264/13491
...F.................................................................................... 11352/13491
...................................F.................................................... 11440/13491
........................................................................................ 11616/13491
........................................................................................ 11616/13491
................F.....F................F...............F...................F............ 11704/13491
F...F..F.............F.................................................................. 11792/13491
.....................................F...........................................F...... 11880/13491
........................................................................................ 12056/13491
........................................................................................ 12056/13491
................................F....F.........F........................................ 12144/13491
..........................F............................................................. 12320/13491
........................................................................................ 12408/13491
........................................................................................ 12496/13491
........................................................................................ 12584/13491
........................................................................................ 12584/13491
...........................................i............................................ 12672/13491
........................................................................................ 12760/13491
.....................................F......F........................................... 12848/13491
........................................................................................ 13024/13491
........................................................................................ 13112/13491
........................................................................................ 13200/13491
........................................................................................ 13288/13491
---
10    |


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/hr-associated-type-bound-1/hr-associated-type-bound-1.stderr
To only update this specific test, also pass `--test-args associated-types/hr-associated-type-bound-1.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/hr-associated-type-bound-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/hr-associated-type-bound-1" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/hr-associated-type-bound-1/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the trait bound `str: Clone` is not satisfied
   |
LL |     type U = str;
   |              ^^^ the trait `Clone` is not implemented for `str`
   |
   |
   = help: the trait `Clone` is implemented for `String<A>`
note: required by a bound in `X`
  --> /checkout/src/test/ui/associated-types/hr-associated-type-bound-1.rs:3:33
   |
LL | trait X<'a>
LL | where
LL | where
LL |     for<'b> <Self as X<'b>>::U: Clone,
   |                                 ^^^^^ required by this bound in `X`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
------------------------------------------
---
10    |


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/hr-associated-type-bound-param-1/hr-associated-type-bound-param-1.stderr
To only update this specific test, also pass `--test-args associated-types/hr-associated-type-bound-param-1.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/hr-associated-type-bound-param-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/hr-associated-type-bound-param-1" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/hr-associated-type-bound-param-1/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the trait bound `str: Clone` is not satisfied
   |
LL |     type V = str;
   |              ^^^ the trait `Clone` is not implemented for `str`
   |
   |
   = help: the trait `Clone` is implemented for `String<A>`
note: required by a bound in `Y`
  --> /checkout/src/test/ui/associated-types/hr-associated-type-bound-param-1.rs:4:36
   |
LL | trait Y<'a, T: ?Sized>
...
...
LL |     for<'b> <Self as Y<'b, T>>::V: Clone,
   |                                    ^^^^^ required by this bound in `Y`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/associated-types/hr-associated-type-bound-param-2.rs stdout ----
diff of stderr:

4 LL |     T: Z<'a, u16>,
5    |        ^^^^^^^^^^ the trait `Clone` is not implemented for `str`
-    = help: the trait `Clone` is implemented for `String`
+    = help: the trait `Clone` is implemented for `String<A>`
8 note: required by a bound in `Z`
9   --> $DIR/hr-associated-type-bound-param-2.rs:6:35
---
24 note: required by a bound in `Z`
25   --> $DIR/hr-associated-type-bound-param-2.rs:6:35
26    |

36 LL |     T: Z<'a, u16>,
37    |        ^^^^^^^^^^ the trait `Clone` is not implemented for `str`
-    = help: the trait `Clone` is implemented for `String`
+    = help: the trait `Clone` is implemented for `String<A>`
40 note: required by a bound in `Z`
41   --> $DIR/hr-associated-type-bound-param-2.rs:6:35
41   --> $DIR/hr-associated-type-bound-param-2.rs:6:35
42    |


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/hr-associated-type-bound-param-2/hr-associated-type-bound-param-2.stderr
To only update this specific test, also pass `--test-args associated-types/hr-associated-type-bound-param-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/hr-associated-type-bound-param-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/hr-associated-type-bound-param-2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/hr-associated-type-bound-param-2/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the trait bound `str: Clone` is not satisfied
   |
   |
LL |     T: Z<'a, u16>,
   |        ^^^^^^^^^^ the trait `Clone` is not implemented for `str`
   = help: the trait `Clone` is implemented for `String<A>`
note: required by a bound in `Z`
  --> /checkout/src/test/ui/associated-types/hr-associated-type-bound-param-2.rs:6:35
   |
   |
LL | trait Z<'a, T: ?Sized>
...
...
LL |     for<'b> <T as Z<'b, u16>>::W: Clone,
   |                                   ^^^^^ required by this bound in `Z`
error[E0277]: the trait bound `str: Clone` is not satisfied
  --> /checkout/src/test/ui/associated-types/hr-associated-type-bound-param-2.rs:15:14
   |
LL |     type W = str;
LL |     type W = str;
   |              ^^^ the trait `Clone` is not implemented for `str`
   |
   = help: the trait `Clone` is implemented for `String<A>`
note: required by a bound in `Z`
  --> /checkout/src/test/ui/associated-types/hr-associated-type-bound-param-2.rs:6:35
   |
LL | trait Z<'a, T: ?Sized>
...
...
LL |     for<'b> <T as Z<'b, u16>>::W: Clone,
   |                                   ^^^^^ required by this bound in `Z`
error[E0277]: the trait bound `str: Clone` is not satisfied
  --> /checkout/src/test/ui/associated-types/hr-associated-type-bound-param-2.rs:3:8
   |
   |
LL |     T: Z<'a, u16>,
   |        ^^^^^^^^^^ the trait `Clone` is not implemented for `str`
   = help: the trait `Clone` is implemented for `String<A>`
note: required by a bound in `Z`
  --> /checkout/src/test/ui/associated-types/hr-associated-type-bound-param-2.rs:6:35
   |
   |
LL | trait Z<'a, T: ?Sized>
...
...
LL |     for<'b> <T as Z<'b, u16>>::W: Clone,
   |                                   ^^^^^ required by this bound in `Z`
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0277`.
------------------------------------------
---
10    |


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/hr-associated-type-bound-param-3/hr-associated-type-bound-param-3.stderr
To only update this specific test, also pass `--test-args associated-types/hr-associated-type-bound-param-3.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/hr-associated-type-bound-param-3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/hr-associated-type-bound-param-3" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/hr-associated-type-bound-param-3/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the trait bound `str: Clone` is not satisfied
   |
LL |     type U = str;
   |              ^^^ the trait `Clone` is not implemented for `str`
   |
   |
   = help: the trait `Clone` is implemented for `String<A>`
note: required by a bound in `X`
  --> /checkout/src/test/ui/associated-types/hr-associated-type-bound-param-3.rs:4:33
   |
LL | trait X<'a, T>
...
...
LL |     for<'b> <T as X<'b, T>>::U: Clone,
   |                                 ^^^^^ required by this bound in `X`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
------------------------------------------
---
26    |


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/hr-associated-type-bound-param-5/hr-associated-type-bound-param-5.stderr
To only update this specific test, also pass `--test-args associated-types/hr-associated-type-bound-param-5.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/hr-associated-type-bound-param-5.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/hr-associated-type-bound-param-5" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/hr-associated-type-bound-param-5/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the trait bound `str: Clone` is not satisfied
   |
LL |     type U = str;
   |              ^^^ the trait `Clone` is not implemented for `str`
   |
   |
   = help: the trait `Clone` is implemented for `String<A>`
note: required by a bound in `X`
  --> /checkout/src/test/ui/associated-types/hr-associated-type-bound-param-5.rs:17:45
   |
LL | trait X<'a, T: Cycle + for<'b> X<'b, T>>
...
...
LL |     for<'b> <T::Next as X<'b, T::Next>>::U: Clone,
   |                                             ^^^^^ required by this bound in `X`
error[E0277]: the trait bound `str: Clone` is not satisfied
  --> /checkout/src/test/ui/associated-types/hr-associated-type-bound-param-5.rs:31:14
   |
LL |     type U = str;
LL |     type U = str;
   |              ^^^ the trait `Clone` is not implemented for `str`
   |
   = help: the trait `Clone` is implemented for `String<A>`
note: required by a bound in `X`
  --> /checkout/src/test/ui/associated-types/hr-associated-type-bound-param-5.rs:17:45
   |
LL | trait X<'a, T: Cycle + for<'b> X<'b, T>>
...
...
LL |     for<'b> <T::Next as X<'b, T::Next>>::U: Clone,
   |                                             ^^^^^ required by this bound in `X`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0277`.
------------------------------------------
---
10    |


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/hr-associated-type-bound-param-4/hr-associated-type-bound-param-4.stderr
To only update this specific test, also pass `--test-args associated-types/hr-associated-type-bound-param-4.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/hr-associated-type-bound-param-4.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/hr-associated-type-bound-param-4" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/hr-associated-type-bound-param-4/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the trait bound `str: Clone` is not satisfied
   |
LL |     type U = str;
   |              ^^^ the trait `Clone` is not implemented for `str`
   |
   |
   = help: the trait `Clone` is implemented for `String<A>`
note: required by a bound in `X`
  --> /checkout/src/test/ui/associated-types/hr-associated-type-bound-param-4.rs:4:36
   |
LL | trait X<'a, T>
...
...
LL |     for<'b> <(T,) as X<'b, T>>::U: Clone,
   |                                    ^^^^^ required by this bound in `X`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/block-result/consider-removing-last-semi.rs stdout ----
diff of stderr:

8 LL |     0u8;
9 LL |     "bla".to_string();
+    |
+    = note: expected struct `String`
+            found unit type `()`
11 
11 
12 error[E0308]: mismatched types
13   --> $DIR/consider-removing-last-semi.rs:8:15

19 LL |     "this won't work".to_string();
20 LL |     "removeme".to_string();
+    |
+    = note: expected struct `String`
+            found unit type `()`
22 
---
To only update this specific test, also pass `--test-args block-result/consider-removing-last-semi.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/block-result/consider-removing-last-semi.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/block-result/consider-removing-last-semi" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/block-result/consider-removing-last-semi/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/block-result/consider-removing-last-semi.rs:3:15
   |
   |
LL | pub fn f() -> String {  //~ ERROR mismatched types
   |        -      ^^^^^^ expected struct `String`, found `()`
   |        |
   |        implicitly returns `()` as its body has no tail or `return` expression
LL |     0u8;
LL |     "bla".to_string();
   |
   = note: expected struct `String`
           found unit type `()`


error[E0308]: mismatched types
  --> /checkout/src/test/ui/block-result/consider-removing-last-semi.rs:8:15
   |
LL | pub fn g() -> String {  //~ ERROR mismatched types
   |        -      ^^^^^^ expected struct `String`, found `()`
   |        |
   |        implicitly returns `()` as its body has no tail or `return` expression
LL |     "this won't work".to_string();
LL |     "removeme".to_string();
   |
   = note: expected struct `String`
           found unit type `()`


error[E0308]: mismatched types
  --> /checkout/src/test/ui/block-result/consider-removing-last-semi.rs:13:25
   |
LL | pub fn macro_tests() -> u32 {  //~ ERROR mismatched types
   |        -----------      ^^^ expected `u32`, found `()`
   |        |
   |        implicitly returns `()` as its body has no tail or `return` expression
LL |     mac!();
   |           - help: remove this semicolon

error: aborting due to 3 previous errors
---
diff of stderr:

5    |    ---      ^^^^^^ expected struct `String`, found `()`
6    |    |
7    |    implicitly returns `()` as its body has no tail or `return` expression
+    = note: expected struct `String`
+            found unit type `()`
8 
9 error[E0308]: mismatched types
9 error[E0308]: mismatched types
10   --> $DIR/issue-13428.rs:11:13

16 LL |     "foobar".to_string()
17 LL |     ;
+    |
+    = note: expected struct `String`
+            found unit type `()`
19 
---
To only update this specific test, also pass `--test-args block-result/issue-13428.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/block-result/issue-13428.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/block-result/issue-13428" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/block-result/issue-13428/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/block-result/issue-13428.rs:3:13
   |
   |
LL | fn foo() -> String {  //~ ERROR mismatched types
   |    ---      ^^^^^^ expected struct `String`, found `()`
   |    |
   |    implicitly returns `()` as its body has no tail or `return` expression
   = note: expected struct `String`
           found unit type `()`

error[E0308]: mismatched types
error[E0308]: mismatched types
  --> /checkout/src/test/ui/block-result/issue-13428.rs:11:13
   |
LL | fn bar() -> String {  //~ ERROR mismatched types
   |    ---      ^^^^^^ expected struct `String`, found `()`
   |    |
   |    implicitly returns `()` as its body has no tail or `return` expression
LL |     "foobar".to_string()
LL |     ;
   |
   = note: expected struct `String`
           found unit type `()`

---
diff of stderr:

7    |                              type must be known at this point
8    |
9    = note: multiple `impl`s satisfying `String: PartialEq<_>` found in the `alloc` crate:
-            - impl PartialEq for String;
-            - impl<'a, 'b> PartialEq<&'a str> for String;
-            - impl<'a, 'b> PartialEq<Cow<'a, str>> for String;
-            - impl<'a, 'b> PartialEq<str> for String;
+            - impl<'a, 'b, A> PartialEq<&'a str> for String<A>
+              where A: Allocator;
+            - impl<'a, 'b, A> PartialEq<Cow<'a, str>> for String<A>
+              where A: Allocator;
+            - impl<'a, 'b, A> PartialEq<str> for String<A>
+              where A: Allocator;
+            - impl<A, B> PartialEq<String<B>> for String<A>
+              where A: Allocator, B: Allocator;
14 help: try using a fully qualified path to specify the expected types
15    |
16 LL |         if String::from("a") == <&str as TryInto<T>>::try_into("a").unwrap() {}

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/issue-72616/issue-72616.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args inference/issue-72616.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/inference/issue-72616.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/issue-72616" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/issue-72616/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/inference/issue-72616.rs:20:37
   |
   |
LL |         if String::from("a") == "a".try_into().unwrap() {}
   |                              |
   |                              type must be known at this point
   |
   |
   = note: multiple `impl`s satisfying `String: PartialEq<_>` found in the `alloc` crate:
           - impl<'a, 'b, A> PartialEq<&'a str> for String<A>
             where A: Allocator;
           - impl<'a, 'b, A> PartialEq<Cow<'a, str>> for String<A>
             where A: Allocator;
           - impl<'a, 'b, A> PartialEq<str> for String<A>
             where A: Allocator;
           - impl<A, B> PartialEq<String<B>> for String<A>
             where A: Allocator, B: Allocator;
help: try using a fully qualified path to specify the expected types
   |
LL |         if String::from("a") == <&str as TryInto<T>>::try_into("a").unwrap() {}
   |                                 +++++++++++++++++++++++++++++++   ~
error: aborting due to previous error

For more information about this error, try `rustc --explain E0283`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/inference/issue-72690.rs stdout ----
diff of stderr:

2   --> $DIR/issue-72690.rs:7:5
3    |
4 LL |     String::from("x".as_ref());
-    |     ^^^^^^^^^^^^ cannot infer type for reference `&_`
+    |     ^^^^^^^^^^^^ cannot infer type for struct `String<_>`
6    |
-    = note: multiple `impl`s satisfying `String: From<&_>` found in the `alloc` crate:
+    = note: multiple `impl`s satisfying `String<_>: From<&_>` found in the `alloc` crate:
8            - impl<> From<&String> for String;
9            - impl<> From<&str> for String;

71   --> $DIR/issue-72690.rs:21:5
72    |
72    |
73 LL |     String::from("x".as_ref());
-    |     ^^^^^^^^^^^^ cannot infer type for reference `&_`
+    |     ^^^^^^^^^^^^ cannot infer type for struct `String<_>`
75    |
-    = note: multiple `impl`s satisfying `String: From<&_>` found in the `alloc` crate:
+    = note: multiple `impl`s satisfying `String<_>: From<&_>` found in the `alloc` crate:
77            - impl<> From<&String> for String;
78            - impl<> From<&str> for String;

97   --> $DIR/issue-72690.rs:28:5
98    |
98    |
99 LL |     String::from("x".as_ref());
-    |     ^^^^^^^^^^^^ cannot infer type for reference `&_`
+    |     ^^^^^^^^^^^^ cannot infer type for struct `String<_>`
101    |
-    = note: multiple `impl`s satisfying `String: From<&_>` found in the `alloc` crate:
+    = note: multiple `impl`s satisfying `String<_>: From<&_>` found in the `alloc` crate:
103            - impl<> From<&String> for String;
104            - impl<> From<&str> for String;

123   --> $DIR/issue-72690.rs:37:5
124    |
124    |
125 LL |     String::from("x".as_ref());
-    |     ^^^^^^^^^^^^ cannot infer type for reference `&_`
+    |     ^^^^^^^^^^^^ cannot infer type for struct `String<_>`
127    |
-    = note: multiple `impl`s satisfying `String: From<&_>` found in the `alloc` crate:
+    = note: multiple `impl`s satisfying `String<_>: From<&_>` found in the `alloc` crate:
129            - impl<> From<&String> for String;
130            - impl<> From<&str> for String;

149   --> $DIR/issue-72690.rs:46:5
150    |
150    |
151 LL |     String::from("x".as_ref());
-    |     ^^^^^^^^^^^^ cannot infer type for reference `&_`
+    |     ^^^^^^^^^^^^ cannot infer type for struct `String<_>`
153    |
-    = note: multiple `impl`s satisfying `String: From<&_>` found in the `alloc` crate:
+    = note: multiple `impl`s satisfying `String<_>: From<&_>` found in the `alloc` crate:
155            - impl<> From<&String> for String;
156            - impl<> From<&str> for String;

175   --> $DIR/issue-72690.rs:53:5
176    |
176    |
177 LL |     String::from("x".as_ref());
-    |     ^^^^^^^^^^^^ cannot infer type for reference `&_`
+    |     ^^^^^^^^^^^^ cannot infer type for struct `String<_>`
179    |
-    = note: multiple `impl`s satisfying `String: From<&_>` found in the `alloc` crate:
+    = note: multiple `impl`s satisfying `String<_>: From<&_>` found in the `alloc` crate:
181            - impl<> From<&String> for String;
182            - impl<> From<&str> for String;

201   --> $DIR/issue-72690.rs:62:5
202    |
202    |
203 LL |     String::from("x".as_ref());
-    |     ^^^^^^^^^^^^ cannot infer type for reference `&_`
+    |     ^^^^^^^^^^^^ cannot infer type for struct `String<_>`
205    |
-    = note: multiple `impl`s satisfying `String: From<&_>` found in the `alloc` crate:
+    = note: multiple `impl`s satisfying `String<_>: From<&_>` found in the `alloc` crate:
207            - impl<> From<&String> for String;
208            - impl<> From<&str> for String;


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/issue-72690/issue-72690.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/issue-72690/issue-72690.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args inference/issue-72690.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/inference/issue-72690.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/issue-72690" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/issue-72690/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/inference/issue-72690.rs:7:5
   |
   |
LL |     String::from("x".as_ref()); //~ ERROR type annotations needed
   |     ^^^^^^^^^^^^ cannot infer type for struct `String<_>`
   |
   = note: multiple `impl`s satisfying `String<_>: From<&_>` found in the `alloc` crate:
           - impl<> From<&String> for String;
           - impl<> From<&str> for String;
error[E0283]: type annotations needed
  --> /checkout/src/test/ui/inference/issue-72690.rs:7:22
   |
   |
LL |     String::from("x".as_ref()); //~ ERROR type annotations needed
   |
   |
   = note: multiple `impl`s satisfying `str: AsRef<_>` found in the following crates: `core`, `std`:
           - impl AsRef<OsStr> for str;
           - impl AsRef<Path> for str;
           - impl AsRef<[u8]> for str;
           - impl AsRef<str> for str;
help: try using a fully qualified path to specify the expected types
   |
LL |     String::from(<str as AsRef<T>>::as_ref("x")); //~ ERROR type annotations needed
   |                  ++++++++++++++++++++++++++   ~
error[E0282]: type annotations needed
  --> /checkout/src/test/ui/inference/issue-72690.rs:12:6
   |
   |
LL |     |x| String::from("x".as_ref()); //~ ERROR type annotations needed
   |
   |
help: consider giving this closure parameter an explicit type
   |
LL |     |x: _| String::from("x".as_ref()); //~ ERROR type annotations needed

error[E0283]: type annotations needed
  --> /checkout/src/test/ui/inference/issue-72690.rs:12:26
   |
   |
LL |     |x| String::from("x".as_ref()); //~ ERROR type annotations needed
   |
   |
   = note: multiple `impl`s satisfying `str: AsRef<_>` found in the following crates: `core`, `std`:
           - impl AsRef<OsStr> for str;
           - impl AsRef<Path> for str;
           - impl AsRef<[u8]> for str;
           - impl AsRef<str> for str;
help: try using a fully qualified path to specify the expected types
   |
LL |     |x| String::from(<str as AsRef<T>>::as_ref("x")); //~ ERROR type annotations needed
   |                      ++++++++++++++++++++++++++   ~
error[E0283]: type annotations needed for `&T`
  --> /checkout/src/test/ui/inference/issue-72690.rs:17:9
   |
   |
LL |     let _ = "x".as_ref(); //~ ERROR type annotations needed
   |         ^       ------ type must be known at this point
   |
   = note: multiple `impl`s satisfying `str: AsRef<_>` found in the following crates: `core`, `std`:
           - impl AsRef<OsStr> for str;
           - impl AsRef<Path> for str;
           - impl AsRef<[u8]> for str;
           - impl AsRef<str> for str;
help: consider giving this pattern a type, where the type for type parameter `T` is specified
   |
LL |     let _: &T = "x".as_ref(); //~ ERROR type annotations needed

error[E0283]: type annotations needed
  --> /checkout/src/test/ui/inference/issue-72690.rs:21:5
   |
   |
LL |     String::from("x".as_ref()); //~ ERROR type annotations needed
   |     ^^^^^^^^^^^^ cannot infer type for struct `String<_>`
   |
   = note: multiple `impl`s satisfying `String<_>: From<&_>` found in the `alloc` crate:
           - impl<> From<&String> for String;
           - impl<> From<&str> for String;
error[E0283]: type annotations needed
  --> /checkout/src/test/ui/inference/issue-72690.rs:21:22
   |
   |
LL |     String::from("x".as_ref()); //~ ERROR type annotations needed
   |
   |
   = note: multiple `impl`s satisfying `str: AsRef<_>` found in the following crates: `core`, `std`:
           - impl AsRef<OsStr> for str;
           - impl AsRef<Path> for str;
           - impl AsRef<[u8]> for str;
           - impl AsRef<str> for str;
help: try using a fully qualified path to specify the expected types
   |
LL |     String::from(<str as AsRef<T>>::as_ref("x")); //~ ERROR type annotations needed
   |                  ++++++++++++++++++++++++++   ~
error[E0283]: type annotations needed
  --> /checkout/src/test/ui/inference/issue-72690.rs:28:5
   |
   |
LL |     String::from("x".as_ref()); //~ ERROR type annotations needed
   |     ^^^^^^^^^^^^ cannot infer type for struct `String<_>`
   |
   = note: multiple `impl`s satisfying `String<_>: From<&_>` found in the `alloc` crate:
           - impl<> From<&String> for String;
           - impl<> From<&str> for String;
error[E0283]: type annotations needed
  --> /checkout/src/test/ui/inference/issue-72690.rs:28:22
   |
   |
LL |     String::from("x".as_ref()); //~ ERROR type annotations needed
   |
   |
   = note: multiple `impl`s satisfying `str: AsRef<_>` found in the following crates: `core`, `std`:
           - impl AsRef<OsStr> for str;
           - impl AsRef<Path> for str;
           - impl AsRef<[u8]> for str;
           - impl AsRef<str> for str;
help: try using a fully qualified path to specify the expected types
   |
LL |     String::from(<str as AsRef<T>>::as_ref("x")); //~ ERROR type annotations needed
   |                  ++++++++++++++++++++++++++   ~
error[E0283]: type annotations needed
  --> /checkout/src/test/ui/inference/issue-72690.rs:37:5
   |
   |
LL |     String::from("x".as_ref()); //~ ERROR type annotations needed
   |     ^^^^^^^^^^^^ cannot infer type for struct `String<_>`
   |
   = note: multiple `impl`s satisfying `String<_>: From<&_>` found in the `alloc` crate:
           - impl<> From<&String> for String;
           - impl<> From<&str> for String;
error[E0283]: type annotations needed
  --> /checkout/src/test/ui/inference/issue-72690.rs:37:22
   |
   |
LL |     String::from("x".as_ref()); //~ ERROR type annotations needed
   |
   |
   = note: multiple `impl`s satisfying `str: AsRef<_>` found in the following crates: `core`, `std`:
           - impl AsRef<OsStr> for str;
           - impl AsRef<Path> for str;
           - impl AsRef<[u8]> for str;
           - impl AsRef<str> for str;
help: try using a fully qualified path to specify the expected types
   |
LL |     String::from(<str as AsRef<T>>::as_ref("x")); //~ ERROR type annotations needed
   |                  ++++++++++++++++++++++++++   ~
error[E0283]: type annotations needed
  --> /checkout/src/test/ui/inference/issue-72690.rs:46:5
   |
   |
LL |     String::from("x".as_ref()); //~ ERROR type annotations needed
   |     ^^^^^^^^^^^^ cannot infer type for struct `String<_>`
   |
   = note: multiple `impl`s satisfying `String<_>: From<&_>` found in the `alloc` crate:
           - impl<> From<&String> for String;
           - impl<> From<&str> for String;
error[E0283]: type annotations needed
  --> /checkout/src/test/ui/inference/issue-72690.rs:46:22
   |
   |
LL |     String::from("x".as_ref()); //~ ERROR type annotations needed
   |
   |
   = note: multiple `impl`s satisfying `str: AsRef<_>` found in the following crates: `core`, `std`:
           - impl AsRef<OsStr> for str;
           - impl AsRef<Path> for str;
           - impl AsRef<[u8]> for str;
           - impl AsRef<str> for str;
help: try using a fully qualified path to specify the expected types
   |
LL |     String::from(<str as AsRef<T>>::as_ref("x")); //~ ERROR type annotations needed
   |                  ++++++++++++++++++++++++++   ~
error[E0283]: type annotations needed
  --> /checkout/src/test/ui/inference/issue-72690.rs:53:5
   |
   |
LL |     String::from("x".as_ref()); //~ ERROR type annotations needed
   |     ^^^^^^^^^^^^ cannot infer type for struct `String<_>`
   |
   = note: multiple `impl`s satisfying `String<_>: From<&_>` found in the `alloc` crate:
           - impl<> From<&String> for String;
           - impl<> From<&str> for String;
error[E0283]: type annotations needed
  --> /checkout/src/test/ui/inference/issue-72690.rs:53:22
   |
   |
LL |     String::from("x".as_ref()); //~ ERROR type annotations needed
   |
   |
   = note: multiple `impl`s satisfying `str: AsRef<_>` found in the following crates: `core`, `std`:
           - impl AsRef<OsStr> for str;
           - impl AsRef<Path> for str;
           - impl AsRef<[u8]> for str;
           - impl AsRef<str> for str;
help: try using a fully qualified path to specify the expected types
   |
LL |     String::from(<str as AsRef<T>>::as_ref("x")); //~ ERROR type annotations needed
   |                  ++++++++++++++++++++++++++   ~
error[E0283]: type annotations needed
  --> /checkout/src/test/ui/inference/issue-72690.rs:62:5
   |
   |
LL |     String::from("x".as_ref()); //~ ERROR type annotations needed
   |     ^^^^^^^^^^^^ cannot infer type for struct `String<_>`
   |
   = note: multiple `impl`s satisfying `String<_>: From<&_>` found in the `alloc` crate:
           - impl<> From<&String> for String;
           - impl<> From<&str> for String;
error[E0283]: type annotations needed
  --> /checkout/src/test/ui/inference/issue-72690.rs:62:22
   |
   |
LL |     String::from("x".as_ref()); //~ ERROR type annotations needed
   |
   |
   = note: multiple `impl`s satisfying `str: AsRef<_>` found in the following crates: `core`, `std`:
           - impl AsRef<OsStr> for str;
           - impl AsRef<Path> for str;
           - impl AsRef<[u8]> for str;
           - impl AsRef<str> for str;
help: try using a fully qualified path to specify the expected types
   |
LL |     String::from(<str as AsRef<T>>::as_ref("x")); //~ ERROR type annotations needed
   |                  ++++++++++++++++++++++++++   ~
error: aborting due to 17 previous errors

Some errors have detailed explanations: E0282, E0283.
For more information about an error, try `rustc --explain E0282`.
---
To only update this specific test, also pass `--test-args inference/deref-suggestion.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/inference/deref-suggestion.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/deref-suggestion" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/deref-suggestion/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/inference/deref-suggestion.rs:8:9
   |
LL |     foo(s);
LL |     foo(s);
   |     --- ^- help: try using a conversion method: `.to_string()`
   |     |   |
   |     |   expected struct `String`, found `&String`
   |
   = note: expected struct `String`
           found reference `&String`
note: function defined here
note: function defined here
  --> /checkout/src/test/ui/inference/deref-suggestion.rs:5:4
   |
LL | fn foo(_: String) {}

error[E0308]: mismatched types
  --> /checkout/src/test/ui/inference/deref-suggestion.rs:14:10
   |
   |
LL |     foo3(u);
   |     ---- ^ expected `u32`, found `&u32`
   |     arguments to this function are incorrect
   |
note: function defined here
  --> /checkout/src/test/ui/inference/deref-suggestion.rs:12:4
  --> /checkout/src/test/ui/inference/deref-suggestion.rs:12:4
   |
LL | fn foo3(_: u32) {}
help: consider dereferencing the borrow
   |
LL |     foo3(*u);
   |          +
   |          +

error[E0308]: mismatched types
  --> /checkout/src/test/ui/inference/deref-suggestion.rs:30:9
   |
LL |     foo(&"aaa".to_owned());
   |     --- ^^^^^^^^^^^^^^^^^ expected struct `String`, found `&String`
   |     arguments to this function are incorrect
   |
   = note: expected struct `String`
           found reference `&String`
           found reference `&String`
note: function defined here
  --> /checkout/src/test/ui/inference/deref-suggestion.rs:5:4
   |
LL | fn foo(_: String) {}
help: consider removing the borrow
   |
   |
LL -     foo(&"aaa".to_owned());
LL +     foo("aaa".to_owned());

error[E0308]: mismatched types
  --> /checkout/src/test/ui/inference/deref-suggestion.rs:32:9
   |
   |
LL |     foo(&mut "aaa".to_owned());
   |     --- ^^^^^^^^^^^^^^^^^^^^^ expected struct `String`, found `&mut String`
   |     arguments to this function are incorrect
   |
   = note:         expected struct `String`
           found mutable reference `&mut String`
           found mutable reference `&mut String`
note: function defined here
  --> /checkout/src/test/ui/inference/deref-suggestion.rs:5:4
   |
LL | fn foo(_: String) {}
help: consider removing the borrow
   |
   |
LL -     foo(&mut "aaa".to_owned());
LL +     foo("aaa".to_owned());

error[E0308]: mismatched types
  --> /checkout/src/test/ui/inference/deref-suggestion.rs:34:10
   |
   |
LL |     foo3(borrow!(0));
   |     ---- ^^^^^^^^^^ expected `u32`, found `&{integer}`
   |     arguments to this function are incorrect
   |
note: function defined here
  --> /checkout/src/test/ui/inference/deref-suggestion.rs:12:4
  --> /checkout/src/test/ui/inference/deref-suggestion.rs:12:4
   |
LL | fn foo3(_: u32) {}

error[E0308]: mismatched types
  --> /checkout/src/test/ui/inference/deref-suggestion.rs:37:5
   |
   |
LL |     assert_eq!(3i32, &3i32);
   |     ^^^^^^^^^^^^^^^^^^^^^^^ expected `i32`, found `&i32`
   |
   = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0308]: mismatched types
  --> /checkout/src/test/ui/inference/deref-suggestion.rs:40:17
   |
   |
LL |     let s = S { u };
   |                 |
   |                 |
   |                 expected `&u32`, found integer
   |                 help: consider borrowing here: `u: &u`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/inference/deref-suggestion.rs:42:20
   |
   |
LL |     let s = S { u: u };
   |                    |
   |                    |
   |                    expected `&u32`, found integer
   |                    help: consider borrowing here: `&u`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/inference/deref-suggestion.rs:45:17
   |
   |
LL |     let r = R { i };
   |                 ^ expected `u32`, found `&{integer}`
help: consider dereferencing the borrow
   |
   |
LL |     let r = R { i: *i };

error[E0308]: mismatched types
  --> /checkout/src/test/ui/inference/deref-suggestion.rs:47:20
   |
   |
LL |     let r = R { i: i };
   |                    ^ expected `u32`, found `&{integer}`
help: consider dereferencing the borrow
   |
   |
LL |     let r = R { i: *i };

error[E0308]: mismatched types
  --> /checkout/src/test/ui/inference/deref-suggestion.rs:56:9
   |
   |
LL |         b
   |         ^ expected `i32`, found `&{integer}`
help: consider dereferencing the borrow
   |
LL |         *b
   |         +
   |         +

error[E0308]: mismatched types
  --> /checkout/src/test/ui/inference/deref-suggestion.rs:64:9
   |
LL |         b
   |         ^ expected `i32`, found `&{integer}`
help: consider dereferencing the borrow
   |
LL |         *b
   |         +
   |         +

error[E0308]: `if` and `else` have incompatible types
   |
LL |        let val = if true {
   |   _______________-
LL |  |         *a
LL |  |         *a
   |  |         -- expected because of this
LL |  |     } else if true {
   |  |____________^
LL | ||     //~^ ERROR incompatible types
LL | ||         b
LL | ||     } else {
LL | ||         &0
LL | ||     };
   | ||_____|
   | ||_____|
   | |______`if` and `else` have incompatible types
   |        expected `i32`, found `&{integer}`
error: aborting due to 13 previous errors

For more information about this error, try `rustc --explain E0308`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/issues/issue-53348.rs stdout ----
diff of stderr:

6 LL |     for i in v {
7 LL |         a = *i.to_string();
8    |             ^^^^^^^^^^^^^^ expected struct `String`, found `str`
+    = note: expected struct `String`
+                 found type `str`
9 
10 error: aborting due to previous error
---
To only update this specific test, also pass `--test-args issues/issue-53348.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-53348.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-53348" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-53348/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/issues/issue-53348.rs:10:13
   |
LL |     let mut a = String::new(); //~ NOTE expected due to this value
   |                 ------------- expected due to this value
   |                 ------------- expected due to this value
LL |     for i in v {
LL |         a = *i.to_string();
   |             ^^^^^^^^^^^^^^ expected struct `String`, found `str`
   = note: expected struct `String`
                found type `str`

error: aborting due to previous error
---
To only update this specific test, also pass `--test-args issues/issue-61106.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-61106.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-61106" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-61106/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/issues/issue-61106.rs:3:9
   |
   |
LL |     foo(x.clone()); //~ ERROR mismatched types
   |     |   |
   |     |   expected `&str`, found struct `String`
   |     |   help: consider borrowing here: `&x`
   |     arguments to this function are incorrect
---

24 expected an expression of a different type. It can occur in several cases, the
25 most common being when calling a function and passing an argument which has a
26 different type than the matching type in the function declaration.
- "},"level":"error","spans":[{"file_name":"$DIR/json-bom-plus-crlf-multifile-aux.rs","byte_start":621,"byte_end":622,"line_start":17,"line_end":17,"column_start":22,"column_end":23,"is_primary":true,"text":[{"text":"    let s : String = 1;  // Error in the middle of line.","highlight_start":22,"highlight_end":23}],"label":"expected struct `String`, found integer","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"$DIR/json-bom-plus-crlf-multifile-aux.rs","byte_start":612,"byte_end":618,"line_start":17,"line_end":17,"column_start":13,"column_end":19,"is_primary":false,"text":[{"text":"    let s : String = 1;  // Error in the middle of line.","highlight_start":13,"highlight_end":19}],"label":"expected due to this","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try using a conversion method","code":null,"level":"help","spans":[{"file_name":"$DIR/json-bom-plus-crlf-multifile-aux.rs","byte_start":622,"byte_end":622,"line_start":17,"line_end":17,"column_start":23,"column_end":23,"is_primary":true,"text":[{"text":"    let s : String = 1;  // Error in the middle of line.","highlight_start":23,"highlight_end":23}],"label":null,"suggested_replacement":".to_string()","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"$DIR/json-bom-plus-crlf-multifile-aux.rs:17:22: error[E0308]: mismatched types
+ "},"level":"error","spans":[{"file_name":"$DIR/json-bom-plus-crlf-multifile-aux.rs","byte_start":621,"byte_end":622,"line_start":17,"line_end":17,"column_start":22,"column_end":23,"is_primary":true,"text":[{"text":"    let s : String = 1;  // Error in the middle of line.","highlight_start":22,"highlight_end":23}],"label":"expected struct `String`, found integer","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"$DIR/json-bom-plus-crlf-multifile-aux.rs","byte_start":612,"byte_end":618,"line_start":17,"line_end":17,"column_start":13,"column_end":19,"is_primary":false,"text":[{"text":"    let s : String = 1;  // Error in the middle of line.","highlight_start":13,"highlight_end":19}],"label":"expected due to this","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"expected struct `String`
+      found type `{integer}`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try using a conversion method","code":null,"level":"help","spans":[{"file_name":"$DIR/json-bom-plus-crlf-multifile-aux.rs","byte_start":622,"byte_end":622,"line_start":17,"line_end":17,"column_start":23,"column_end":23,"is_primary":true,"text":[{"text":"    let s : String = 1;  // Error in the middle of line.","highlight_start":23,"highlight_end":23}],"label":null,"suggested_replacement":".to_string()","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"$DIR/json-bom-plus-crlf-multifile-aux.rs:17:22: error[E0308]: mismatched types
29 {"message":"mismatched types","code":{"code":"E0308","explanation":"Expected type did not match the received type.
30 

52 expected an expression of a different type. It can occur in several cases, the
52 expected an expression of a different type. It can occur in several cases, the
53 most common being when calling a function and passing an argument which has a
54 different type than the matching type in the function declaration.
- "},"level":"error","spans":[{"file_name":"$DIR/json-bom-plus-crlf-multifile-aux.rs","byte_start":681,"byte_end":682,"line_start":19,"line_end":19,"column_start":22,"column_end":23,"is_primary":true,"text":[{"text":"    let s : String = 1","highlight_start":22,"highlight_end":23}],"label":"expected struct `String`, found integer","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"$DIR/json-bom-plus-crlf-multifile-aux.rs","byte_start":672,"byte_end":678,"line_start":19,"line_end":19,"column_start":13,"column_end":19,"is_primary":false,"text":[{"text":"    let s : String = 1","highlight_start":13,"highlight_end":19}],"label":"expected due to this","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try using a conversion method","code":null,"level":"help","spans":[{"file_name":"$DIR/json-bom-plus-crlf-multifile-aux.rs","byte_start":682,"byte_end":682,"line_start":19,"line_end":19,"column_start":23,"column_end":23,"is_primary":true,"text":[{"text":"    let s : String = 1","highlight_start":23,"highlight_end":23}],"label":null,"suggested_replacement":".to_string()","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"$DIR/json-bom-plus-crlf-multifile-aux.rs:19:22: error[E0308]: mismatched types
+ "},"level":"error","spans":[{"file_name":"$DIR/json-bom-plus-crlf-multifile-aux.rs","byte_start":681,"byte_end":682,"line_start":19,"line_end":19,"column_start":22,"column_end":23,"is_primary":true,"text":[{"text":"    let s : String = 1","highlight_start":22,"highlight_end":23}],"label":"expected struct `String`, found integer","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"$DIR/json-bom-plus-crlf-multifile-aux.rs","byte_start":672,"byte_end":678,"line_start":19,"line_end":19,"column_start":13,"column_end":19,"is_primary":false,"text":[{"text":"    let s : String = 1","highlight_start":13,"highlight_end":19}],"label":"expected due to this","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"expected struct `String`
+      found type `{integer}`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try using a conversion method","code":null,"level":"help","spans":[{"file_name":"$DIR/json-bom-plus-crlf-multifile-aux.rs","byte_start":682,"byte_end":682,"line_start":19,"line_end":19,"column_start":23,"column_end":23,"is_primary":true,"text":[{"text":"    let s : String = 1","highlight_start":23,"highlight_end":23}],"label":null,"suggested_replacement":".to_string()","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"$DIR/json-bom-plus-crlf-multifile-aux.rs:19:22: error[E0308]: mismatched types
57 {"message":"mismatched types","code":{"code":"E0308","explanation":"Expected type did not match the received type.
58 

80 expected an expression of a different type. It can occur in several cases, the
80 expected an expression of a different type. It can occur in several cases, the
81 most common being when calling a function and passing an argument which has a
82 different type than the matching type in the function declaration.
- "},"level":"error","spans":[{"file_name":"$DIR/json-bom-plus-crlf-multifile-aux.rs","byte_start":745,"byte_end":746,"line_start":23,"line_end":23,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"1;  // Error after the newline.","highlight_start":1,"highlight_end":2}],"label":"expected struct `String`, found integer","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"$DIR/json-bom-plus-crlf-multifile-aux.rs","byte_start":735,"byte_end":741,"line_start":22,"line_end":22,"column_start":13,"column_end":19,"is_primary":false,"text":[{"text":"    let s : String =","highlight_start":13,"highlight_end":19}],"label":"expected due to this","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try using a conversion method","code":null,"level":"help","spans":[{"file_name":"$DIR/json-bom-plus-crlf-multifile-aux.rs","byte_start":746,"byte_end":746,"line_start":23,"line_end":23,"column_start":2,"column_end":2,"is_primary":true,"text":[{"text":"1;  // Error after the newline.","highlight_start":2,"highlight_end":2}],"label":null,"suggested_replacement":".to_string()","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"$DIR/json-bom-plus-crlf-multifile-aux.rs:23:1: error[E0308]: mismatched types
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+ "},"level":"error","spans":[{"file_name":"$DIR/json-bom-plus-crlf-multifile-aux.rs","byte_start":745,"byte_end":746,"line_start":23,"line_end":23,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"1;  // Error after the newline.","highlight_start":1,"highlight_end":2}],"label":"expected struct `String`, found integer","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"$DIR/json-bom-plus-crlf-multifile-aux.rs","byte_start":735,"byte_end":741,"line_start":22,"line_end":22,"column_start":13,"column_end":19,"is_primary":false,"text":[{"text":"    let s : String =","highlight_start":13,"highlight_end":19}],"label":"expected due to this","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"expected struct `String`
+      found type `{integer}`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try using a conversion method","code":null,"level":"help","spans":[{"file_name":"$DIR/json-bom-plus-crlf-multifile-aux.rs","byte_start":746,"byte_end":746,"line_start":23,"line_end":23,"column_start":2,"column_end":2,"is_primary":true,"text":[{"text":"1;  // Error after the newline.","highlight_start":2,"highlight_end":2}],"label":null,"suggested_replacement":".to_string()","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"$DIR/json-bom-plus-crlf-multifile-aux.rs:23:1: error[E0308]: mismatched types
85 {"message":"mismatched types","code":{"code":"E0308","explanation":"Expected type did not match the received type.
86 

108 expected an expression of a different type. It can occur in several cases, the
108 expected an expression of a different type. It can occur in several cases, the
109 most common being when calling a function and passing an argument which has a
110 different type than the matching type in the function declaration.
- "},"level":"error","spans":[{"file_name":"$DIR/json-bom-plus-crlf-multifile-aux.rs","byte_start":801,"byte_end":809,"line_start":25,"line_end":26,"column_start":22,"column_end":6,"is_primary":true,"text":[{"text":"    let s : String = (","highlight_start":22,"highlight_end":23},{"text":"    );  // Error spanning the newline.","highlight_start":1,"highlight_end":6}],"label":"expected struct `String`, found `()`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"$DIR/json-bom-plus-crlf-multifile-aux.rs","byte_start":792,"byte_end":798,"line_start":25,"line_end":25,"column_start":13,"column_end":19,"is_primary":false,"text":[{"text":"    let s : String = (","highlight_start":13,"highlight_end":19}],"label":"expected due to this","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"$DIR/json-bom-plus-crlf-multifile-aux.rs:25:22: error[E0308]: mismatched types
+ "},"level":"error","spans":[{"file_name":"$DIR/json-bom-plus-crlf-multifile-aux.rs","byte_start":801,"byte_end":809,"line_start":25,"line_end":26,"column_start":22,"column_end":6,"is_primary":true,"text":[{"text":"    let s : String = (","highlight_start":22,"highlight_end":23},{"text":"    );  // Error spanning the newline.","highlight_start":1,"highlight_end":6}],"label":"expected struct `String`, found `()`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"$DIR/json-bom-plus-crlf-multifile-aux.rs","byte_start":792,"byte_end":798,"line_start":25,"line_end":25,"column_start":13,"column_end":19,"is_primary":false,"text":[{"text":"    let s : String = (","highlight_start":13,"highlight_end":19}],"label":"expected due to this","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"expected struct `String`
+ found unit type `()`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"$DIR/json-bom-plus-crlf-multifile-aux.rs:25:22: error[E0308]: mismatched types
113 {"message":"aborting due to 4 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 4 previous errors
114 "}



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/json/json-bom-plus-crlf-multifile/json-bom-plus-crlf-multifile.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args json/json-bom-plus-crlf-multifile.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/json/json-bom-plus-crlf-multifile.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/json/json-bom-plus-crlf-multifile" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--json=diagnostic-short" "--error-format=json" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/json/json-bom-plus-crlf-multifile/auxiliary"
stdout: none
--- stderr -------------------------------
/checkout/src/test/ui/json/json-bom-plus-crlf-multifile-aux.rs:17:22: error[E0308]: mismatched types
/checkout/src/test/ui/json/json-bom-plus-crlf-multifile-aux.rs:19:22: error[E0308]: mismatched types
/checkout/src/test/ui/json/json-bom-plus-crlf-multifile-aux.rs:23:1: error[E0308]: mismatched types
/checkout/src/test/ui/json/json-bom-plus-crlf-multifile-aux.rs:25:22: error[E0308]: mismatched types
------------------------------------------


---- [ui] src/test/ui/json/json-bom-plus-crlf.rs stdout ----
---- [ui] src/test/ui/json/json-bom-plus-crlf.rs stdout ----
diff of stderr:

24 expected an expression of a different type. It can occur in several cases, the
25 most common being when calling a function and passing an argument which has a
26 different type than the matching type in the function declaration.
- "},"level":"error","spans":[{"file_name":"$DIR/json-bom-plus-crlf.rs","byte_start":606,"byte_end":607,"line_start":16,"line_end":16,"column_start":22,"column_end":23,"is_primary":true,"text":[{"text":"    let s : String = 1;  // Error in the middle of line.","highlight_start":22,"highlight_end":23}],"label":"expected struct `String`, found integer","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"$DIR/json-bom-plus-crlf.rs","byte_start":597,"byte_end":603,"line_start":16,"line_end":16,"column_start":13,"column_end":19,"is_primary":false,"text":[{"text":"    let s : String = 1;  // Error in the middle of line.","highlight_start":13,"highlight_end":19}],"label":"expected due to this","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try using a conversion method","code":null,"level":"help","spans":[{"file_name":"$DIR/json-bom-plus-crlf.rs","byte_start":607,"byte_end":607,"line_start":16,"line_end":16,"column_start":23,"column_end":23,"is_primary":true,"text":[{"text":"    let s : String = 1;  // Error in the middle of line.","highlight_start":23,"highlight_end":23}],"label":null,"suggested_replacement":".to_string()","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"$DIR/json-bom-plus-crlf.rs:16:22: error[E0308]: mismatched types
+ "},"level":"error","spans":[{"file_name":"$DIR/json-bom-plus-crlf.rs","byte_start":606,"byte_end":607,"line_start":16,"line_end":16,"column_start":22,"column_end":23,"is_primary":true,"text":[{"text":"    let s : String = 1;  // Error in the middle of line.","highlight_start":22,"highlight_end":23}],"label":"expected struct `String`, found integer","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"$DIR/json-bom-plus-crlf.rs","byte_start":597,"byte_end":603,"line_start":16,"line_end":16,"column_start":13,"column_end":19,"is_primary":false,"text":[{"text":"    let s : String = 1;  // Error in the middle of line.","highlight_start":13,"highlight_end":19}],"label":"expected due to this","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"expected struct `String`
+      found type `{integer}`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try using a conversion method","code":null,"level":"help","spans":[{"file_name":"$DIR/json-bom-plus-crlf.rs","byte_start":607,"byte_end":607,"line_start":16,"line_end":16,"column_start":23,"column_end":23,"is_primary":true,"text":[{"text":"    let s : String = 1;  // Error in the middle of line.","highlight_start":23,"highlight_end":23}],"label":null,"suggested_replacement":".to_string()","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"$DIR/json-bom-plus-crlf.rs:16:22: error[E0308]: mismatched types
29 {"message":"mismatched types","code":{"code":"E0308","explanation":"Expected type did not match the received type.
30 

52 expected an expression of a different type. It can occur in several cases, the
52 expected an expression of a different type. It can occur in several cases, the
53 most common being when calling a function and passing an argument which has a
54 different type than the matching type in the function declaration.
- "},"level":"error","spans":[{"file_name":"$DIR/json-bom-plus-crlf.rs","byte_start":666,"byte_end":667,"line_start":18,"line_end":18,"column_start":22,"column_end":23,"is_primary":true,"text":[{"text":"    let s : String = 1","highlight_start":22,"highlight_end":23}],"label":"expected struct `String`, found integer","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"$DIR/json-bom-plus-crlf.rs","byte_start":657,"byte_end":663,"line_start":18,"line_end":18,"column_start":13,"column_end":19,"is_primary":false,"text":[{"text":"    let s : String = 1","highlight_start":13,"highlight_end":19}],"label":"expected due to this","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try using a conversion method","code":null,"level":"help","spans":[{"file_name":"$DIR/json-bom-plus-crlf.rs","byte_start":667,"byte_end":667,"line_start":18,"line_end":18,"column_start":23,"column_end":23,"is_primary":true,"text":[{"text":"    let s : String = 1","highlight_start":23,"highlight_end":23}],"label":null,"suggested_replacement":".to_string()","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"$DIR/json-bom-plus-crlf.rs:18:22: error[E0308]: mismatched types
+ "},"level":"error","spans":[{"file_name":"$DIR/json-bom-plus-crlf.rs","byte_start":666,"byte_end":667,"line_start":18,"line_end":18,"column_start":22,"column_end":23,"is_primary":true,"text":[{"text":"    let s : String = 1","highlight_start":22,"highlight_end":23}],"label":"expected struct `String`, found integer","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"$DIR/json-bom-plus-crlf.rs","byte_start":657,"byte_end":663,"line_start":18,"line_end":18,"column_start":13,"column_end":19,"is_primary":false,"text":[{"text":"    let s : String = 1","highlight_start":13,"highlight_end":19}],"label":"expected due to this","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"expected struct `String`
+      found type `{integer}`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try using a conversion method","code":null,"level":"help","spans":[{"file_name":"$DIR/json-bom-plus-crlf.rs","byte_start":667,"byte_end":667,"line_start":18,"line_end":18,"column_start":23,"column_end":23,"is_primary":true,"text":[{"text":"    let s : String = 1","highlight_start":23,"highlight_end":23}],"label":null,"suggested_replacement":".to_string()","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"$DIR/json-bom-plus-crlf.rs:18:22: error[E0308]: mismatched types
57 {"message":"mismatched types","code":{"code":"E0308","explanation":"Expected type did not match the received type.
58 

80 expected an expression of a different type. It can occur in several cases, the
80 expected an expression of a different type. It can occur in several cases, the
81 most common being when calling a function and passing an argument which has a
82 different type than the matching type in the function declaration.
- "},"level":"error","spans":[{"file_name":"$DIR/json-bom-plus-crlf.rs","byte_start":730,"byte_end":731,"line_start":22,"line_end":22,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"1;  // Error after the newline.","highlight_start":1,"highlight_end":2}],"label":"expected struct `String`, found integer","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"$DIR/json-bom-plus-crlf.rs","byte_start":720,"byte_end":726,"line_start":21,"line_end":21,"column_start":13,"column_end":19,"is_primary":false,"text":[{"text":"    let s : String =","highlight_start":13,"highlight_end":19}],"label":"expected due to this","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try using a conversion method","code":null,"level":"help","spans":[{"file_name":"$DIR/json-bom-plus-crlf.rs","byte_start":731,"byte_end":731,"line_start":22,"line_end":22,"column_start":2,"column_end":2,"is_primary":true,"text":[{"text":"1;  // Error after the newline.","highlight_start":2,"highlight_end":2}],"label":null,"suggested_replacement":".to_string()","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"$DIR/json-bom-plus-crlf.rs:22:1: error[E0308]: mismatched types
+ "},"level":"error","spans":[{"file_name":"$DIR/json-bom-plus-crlf.rs","byte_start":730,"byte_end":731,"line_start":22,"line_end":22,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"1;  // Error after the newline.","highlight_start":1,"highlight_end":2}],"label":"expected struct `String`, found integer","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"$DIR/json-bom-plus-crlf.rs","byte_start":720,"byte_end":726,"line_start":21,"line_end":21,"column_start":13,"column_end":19,"is_primary":false,"text":[{"text":"    let s : String =","highlight_start":13,"highlight_end":19}],"label":"expected due to this","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"expected struct `String`
+      found type `{integer}`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try using a conversion method","code":null,"level":"help","spans":[{"file_name":"$DIR/json-bom-plus-crlf.rs","byte_start":731,"byte_end":731,"line_start":22,"line_end":22,"column_start":2,"column_end":2,"is_primary":true,"text":[{"text":"1;  // Error after the newline.","highlight_start":2,"highlight_end":2}],"label":null,"suggested_replacement":".to_string()","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"$DIR/json-bom-plus-crlf.rs:22:1: error[E0308]: mismatched types
85 {"message":"mismatched types","code":{"code":"E0308","explanation":"Expected type did not match the received type.
86 

108 expected an expression of a different type. It can occur in several cases, the
108 expected an expression of a different type. It can occur in several cases, the
109 most common being when calling a function and passing an argument which has a
110 different type than the matching type in the function declaration.
- "},"level":"error","spans":[{"file_name":"$DIR/json-bom-plus-crlf.rs","byte_start":786,"byte_end":794,"line_start":24,"line_end":25,"column_start":22,"column_end":6,"is_primary":true,"text":[{"text":"    let s : String = (","highlight_start":22,"highlight_end":23},{"text":"    );  // Error spanning the newline.","highlight_start":1,"highlight_end":6}],"label":"expected struct `String`, found `()`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"$DIR/json-bom-plus-crlf.rs","byte_start":777,"byte_end":783,"line_start":24,"line_end":24,"column_start":13,"column_end":19,"is_primary":false,"text":[{"text":"    let s : String = (","highlight_start":13,"highlight_end":19}],"label":"expected due to this","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"$DIR/json-bom-plus-crlf.rs:24:22: error[E0308]: mismatched types
+ "},"level":"error","spans":[{"file_name":"$DIR/json-bom-plus-crlf.rs","byte_start":786,"byte_end":794,"line_start":24,"line_end":25,"column_start":22,"column_end":6,"is_primary":true,"text":[{"text":"    let s : String = (","highlight_start":22,"highlight_end":23},{"text":"    );  // Error spanning the newline.","highlight_start":1,"highlight_end":6}],"label":"expected struct `String`, found `()`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"$DIR/json-bom-plus-crlf.rs","byte_start":777,"byte_end":783,"line_start":24,"line_end":24,"column_start":13,"column_end":19,"is_primary":false,"text":[{"text":"    let s : String = (","highlight_start":13,"highlight_end":19}],"label":"expected due to this","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"expected struct `String`
+ found unit type `()`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"$DIR/json-bom-plus-crlf.rs:24:22: error[E0308]: mismatched types
113 {"message":"aborting due to 4 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 4 previous errors
114 "}



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/json/json-bom-plus-crlf/json-bom-plus-crlf.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args json/json-bom-plus-crlf.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/json/json-bom-plus-crlf.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/json/json-bom-plus-crlf" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--json=diagnostic-short" "--error-format=json" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/json/json-bom-plus-crlf/auxiliary"
stdout: none
--- stderr -------------------------------
/checkout/src/test/ui/json/json-bom-plus-crlf.rs:16:22: error[E0308]: mismatched types
/checkout/src/test/ui/json/json-bom-plus-crlf.rs:18:22: error[E0308]: mismatched types
/checkout/src/test/ui/json/json-bom-plus-crlf.rs:22:1: error[E0308]: mismatched types
/checkout/src/test/ui/json/json-bom-plus-crlf.rs:24:22: error[E0308]: mismatched types
------------------------------------------


---- [ui] src/test/ui/lexer/lex-bad-char-literals-6.rs stdout ----
---- [ui] src/test/ui/lexer/lex-bad-char-literals-6.rs stdout ----
diff of stderr:

40    = help: the trait `PartialEq<char>` is not implemented for `&str`
41    = help: the following other types implement trait `PartialEq<Rhs>`:
42              <&'a str as PartialEq<OsString>>
-              <&'a str as PartialEq<String>>
+              <&'a str as PartialEq<String<A>>>
44              <&'b str as PartialEq<Cow<'a, str>>>
-              <String as PartialEq<&'a str>>
-              <String as PartialEq<Cow<'a, str>>>
-              <String as PartialEq<str>>
-              <String as PartialEq>
+              <String<A> as PartialEq<&'a str>>
+              <String<A> as PartialEq<Cow<'a, str>>>
+              <String<A> as PartialEq<String<B>>>
+              <String<A> as PartialEq<str>>
49              <str as PartialEq<Cow<'a, str>>>
51 


66    = help: the trait `PartialEq<char>` is not implemented for `&str`
67    = help: the following other types implement trait `PartialEq<Rhs>`:
68              <&'a str as PartialEq<OsString>>
-              <&'a str as PartialEq<String>>
+              <&'a str as PartialEq<String<A>>>
70              <&'b str as PartialEq<Cow<'a, str>>>
-              <String as PartialEq<&'a str>>
-              <String as PartialEq<Cow<'a, str>>>
-              <String as PartialEq<str>>
-              <String as PartialEq>
+              <String<A> as PartialEq<&'a str>>
+              <String<A> as PartialEq<Cow<'a, str>>>
+              <String<A> as PartialEq<String<B>>>
+              <String<A> as PartialEq<str>>
75              <str as PartialEq<Cow<'a, str>>>
77 


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lexer/lex-bad-char-literals-6/lex-bad-char-literals-6.stderr
To only update this specific test, also pass `--test-args lexer/lex-bad-char-literals-6.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lexer/lex-bad-char-literals-6.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lexer/lex-bad-char-literals-6" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lexer/lex-bad-char-literals-6/auxiliary"
stdout: none
--- stderr -------------------------------
error: character literal may only contain one codepoint
   |
   |
LL |     let x: &str = 'ab';
   |
   |
help: if you meant to write a `str` literal, use double quotes
   |
LL |     let x: &str = "ab";

error: character literal may only contain one codepoint
  --> /checkout/src/test/ui/lexer/lex-bad-char-literals-6.rs:4:19
   |
   |
LL |     let y: char = 'cd';
   |
   |
help: if you meant to write a `str` literal, use double quotes
   |
LL |     let y: char = "cd";

error: character literal may only contain one codepoint
  --> /checkout/src/test/ui/lexer/lex-bad-char-literals-6.rs:6:13
   |
   |
LL |     let z = 'ef';
   |
   |
help: if you meant to write a `str` literal, use double quotes
   |
LL |     let z = "ef";


error[E0277]: can't compare `&str` with `char`
   |
   |
LL |     if x == y {}
   |          ^^ no implementation for `&str == char`
   |
   = help: the trait `PartialEq<char>` is not implemented for `&str`
   = help: the following other types implement trait `PartialEq<Rhs>`:
             <&'a str as PartialEq<OsString>>
             <&'a str as PartialEq<String<A>>>
             <&'b str as PartialEq<Cow<'a, str>>>
             <String<A> as PartialEq<&'a str>>
             <String<A> as PartialEq<Cow<'a, str>>>
             <String<A> as PartialEq<String<B>>>
             <String<A> as PartialEq<str>>
             <str as PartialEq<Cow<'a, str>>>

error[E0308]: mismatched types
  --> /checkout/src/test/ui/lexer/lex-bad-char-literals-6.rs:15:20
   |
   |
LL |     let a: usize = "";
   |            -----   ^^ expected `usize`, found `&str`
   |            expected due to this


error[E0277]: can't compare `&str` with `char`
   |
   |
LL |     if x == z {}
   |          ^^ no implementation for `&str == char`
   |
   = help: the trait `PartialEq<char>` is not implemented for `&str`
   = help: the following other types implement trait `PartialEq<Rhs>`:
             <&'a str as PartialEq<OsString>>
             <&'a str as PartialEq<String<A>>>
             <&'b str as PartialEq<Cow<'a, str>>>
             <String<A> as PartialEq<&'a str>>
             <String<A> as PartialEq<Cow<'a, str>>>
             <String<A> as PartialEq<String<B>>>
             <String<A> as PartialEq<str>>
             <str as PartialEq<Cow<'a, str>>>

error: aborting due to 6 previous errors

Some errors have detailed explanations: E0277, E0308.
---
+    = note: expected struct `String`
+                 found type `{integer}`
85 help: try using a conversion method
86    |
87 LL |     (1+2).to_string()
95 LL |     -2
96    |     ^^ expected struct `String`, found integer
97    |
+    = note: expected struct `String`
+    = note: expected struct `String`
+                 found type `{integer}`
98 help: try using a conversion method
99    |
100 LL |     (-2).to_string()

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/abridged/abridged.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args mismatched_types/abridged.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/mismatched_types/abridged.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/abridged" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/abridged/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/mismatched_types/abridged.rs:16:5
   |
   |
LL | fn a() -> Foo {
   |           --- expected `Foo` because of return type
LL |     Some(Foo { bar: 1 }) //~ ERROR mismatched types
   |     ^^^^^^^^^^^^^^^^^^^^ expected struct `Foo`, found enum `Option`
   = note: expected struct `Foo`
                found enum `Option<Foo>`

error[E0308]: mismatched types
error[E0308]: mismatched types
  --> /checkout/src/test/ui/mismatched_types/abridged.rs:20:5
   |
LL | fn a2() -> Foo {
   |            --- expected `Foo` because of return type
LL |     Ok(Foo { bar: 1}) //~ ERROR mismatched types
   |     ^^^^^^^^^^^^^^^^^ expected struct `Foo`, found enum `Result`
   = note: expected struct `Foo`
   = note: expected struct `Foo`
                found enum `Result<Foo, _>`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/mismatched_types/abridged.rs:24:5
   |
   |
LL | fn b() -> Option<Foo> {
   |           ----------- expected `Option<Foo>` because of return type
LL |     Foo { bar: 1 } //~ ERROR mismatched types
   |
   = note: expected enum `Option<Foo>`
            found struct `Foo`
help: try wrapping the expression in `Some`
help: try wrapping the expression in `Some`
   |
LL |     Some(Foo { bar: 1 }) //~ ERROR mismatched types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/mismatched_types/abridged.rs:28:5
   |
   |
LL | fn c() -> Result<Foo, Bar> {
   |           ---------------- expected `Result<Foo, Bar>` because of return type
LL |     Foo { bar: 1 } //~ ERROR mismatched types
   |
   |
   = note: expected enum `Result<Foo, Bar>`
help: try wrapping the expression in `Ok`
   |
   |
LL |     Ok(Foo { bar: 1 }) //~ ERROR mismatched types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/mismatched_types/abridged.rs:39:5
   |
   |
LL | fn d() -> X<X<String, String>, String> {
   |           ---------------------------- expected `X<X<String, String>, String>` because of return type
...
LL |     x //~ ERROR mismatched types
   |     ^ expected struct `String`, found integer
   |
   = note: expected struct `X<X<_, String>, String>`
              found struct `X<X<_, {integer}>, {integer}>`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/mismatched_types/abridged.rs:50:5
   |
   |
LL | fn e() -> X<X<String, String>, String> {
   |           ---------------------------- expected `X<X<String, String>, String>` because of return type
...
LL |     x //~ ERROR mismatched types
   |     ^ expected struct `String`, found integer
   |
   = note: expected struct `X<X<_, String>, _>`
              found struct `X<X<_, {integer}>, _>`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/mismatched_types/abridged.rs:54:5
   |
   |
LL | fn f() -> String {
   |           ------ expected `String` because of return type
LL |     1+2 //~ ERROR mismatched types
   |     ^^^ expected struct `String`, found integer
   = note: expected struct `String`
                found type `{integer}`
help: try using a conversion method
   |
   |
LL |     (1+2).to_string() //~ ERROR mismatched types
   |     +   +++++++++++++
error[E0308]: mismatched types
  --> /checkout/src/test/ui/mismatched_types/abridged.rs:59:5
   |
   |
LL | fn g() -> String {
   |           ------ expected `String` because of return type
LL |     -2 //~ ERROR mismatched types
   |     ^^ expected struct `String`, found integer
   = note: expected struct `String`
                found type `{integer}`
help: try using a conversion method
   |
   |
LL |     (-2).to_string() //~ ERROR mismatched types
   |     +  +++++++++++++
error: aborting due to 8 previous errors

For more information about this error, try `rustc --explain E0308`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/pattern/pat-type-err-formal-param.rs stdout ----
diff of stderr:

5    |        ^^^^^^^^  ------ expected due to this
6    |        |
7    |        expected struct `String`, found struct `Tuple`
+    = note: expected struct `String`
+               found struct `Tuple`
8 
9 error: aborting due to previous error
---
To only update this specific test, also pass `--test-args pattern/pat-type-err-formal-param.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/pattern/pat-type-err-formal-param.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/pat-type-err-formal-param" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/pat-type-err-formal-param/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/pattern/pat-type-err-formal-param.rs:8:8
   |
   |
LL | fn foo(Tuple(_): String) {} //~ ERROR mismatched types
   |        ^^^^^^^^  ------ expected due to this
   |        |
   |        expected struct `String`, found struct `Tuple`
   = note: expected struct `String`
              found struct `Tuple`

error: aborting due to previous error
---
+    |
+    = note: expected reference `&str`
+                  found struct `String`
88 
89 error[E0369]: cannot add `String` to `&String`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-39018/issue-39018.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-39018/issue-39018.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args span/issue-39018.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/span/issue-39018.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-39018" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-39018/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0369]: cannot add `&str` to `&str`
   |
   |
LL |     let x = "Hello " + "World!";
   |             -------- ^ -------- &str
   |             |        |
   |             |        `+` cannot be used to concatenate two `&str` strings
   |             &str
   |
   = note: string concatenation requires an owned `String` on the left
help: create an owned `String` from a string reference
   |
LL |     let x = "Hello ".to_owned() + "World!";


error[E0369]: cannot add `World` to `World`
   |
   |
LL |     let y = World::Hello + World::Goodbye;
   |             ------------ ^ -------------- World
   |             World
   |
   |
note: an implementation of `Add<_>` might be missing for `World`
   |
   |
LL | enum World {
   | ^^^^^^^^^^ must implement `Add<_>`
  --> /checkout/library/core/src/ops/arith.rs:100:1
   |
   |
LL | pub trait Add<Rhs = Self> {


error[E0369]: cannot add `String` to `&str`
   |
   |
LL |     let x = "Hello " + "World!".to_owned();
   |             -------- ^ ------------------- String
   |             |        |
   |             |        `+` cannot be used to concatenate a `&str` with a `String`
   |             &str
   |
help: create an owned `String` on the left and add a borrow on the right
   |
LL |     let x = "Hello ".to_owned() + &"World!".to_owned();
   |                     +++++++++++   +

error[E0369]: cannot add `&String` to `&String`
   |
   |
LL |     let _ = &a + &b; //~ ERROR cannot add
   |             -- ^ -- &String
   |             |  |
   |             |  `+` cannot be used to concatenate two `&str` strings
   |             &String
   |
   = note: string concatenation requires an owned `String` on the left
help: remove the borrow to obtain an owned `String`
   |
LL -     let _ = &a + &b; //~ ERROR cannot add
LL +     let _ = a + &b; //~ ERROR cannot add


error[E0369]: cannot add `String` to `&String`
   |
   |
LL |     let _ = &a + b; //~ ERROR cannot add
   |             -- ^ - String
   |             |  |
   |             |  `+` cannot be used to concatenate a `&str` with a `String`
   |             &String
help: remove the borrow on the left and add one on the right
   |
   |
LL -     let _ = &a + b; //~ ERROR cannot add
LL +     let _ = a + &b; //~ ERROR cannot add

error[E0308]: mismatched types
  --> /checkout/src/test/ui/span/issue-39018.rs:29:17
   |
   |
LL |     let _ = a + b; //~ ERROR mismatched types
   |                 |
   |                 expected `&str`, found struct `String`
   |                 help: consider borrowing here: `&b`
   |
   |
   = note: expected reference `&str`
                 found struct `String`

error[E0369]: cannot add `String` to `&String`
   |
   |
LL |     let _ = e + b; //~ ERROR cannot add
   |             - ^ - String
   |             | |
   |             | `+` cannot be used to concatenate a `&str` with a `String`
   |             &String
   |
help: create an owned `String` on the left and add a borrow on the right
   |
LL |     let _ = e.to_owned() + &b; //~ ERROR cannot add
   |              +++++++++++   +

error[E0369]: cannot add `&String` to `&String`
   |
   |
LL |     let _ = e + &b; //~ ERROR cannot add
   |             - ^ -- &String
   |             | |
   |             | `+` cannot be used to concatenate two `&str` strings
   |             &String
   |
   = note: string concatenation requires an owned `String` on the left
help: create an owned `String` from a string reference
   |
LL |     let _ = e.to_owned() + &b; //~ ERROR cannot add

error[E0369]: cannot add `&str` to `&String`
  --> /checkout/src/test/ui/span/issue-39018.rs:32:15
   |
   |
LL |     let _ = e + d; //~ ERROR cannot add
   |             - ^ - &str
   |             | |
   |             | `+` cannot be used to concatenate two `&str` strings
   |             &String
   |
   = note: string concatenation requires an owned `String` on the left
help: create an owned `String` from a string reference
   |
LL |     let _ = e.to_owned() + d; //~ ERROR cannot add


error[E0369]: cannot add `&&str` to `&String`
   |
   |
LL |     let _ = e + &d; //~ ERROR cannot add
   |             - ^ -- &&str
   |             | |
   |             | `+` cannot be used to concatenate two `&str` strings
   |             &String
   |
   = note: string concatenation requires an owned `String` on the left
help: create an owned `String` from a string reference
   |
LL |     let _ = e.to_owned() + &d; //~ ERROR cannot add


error[E0369]: cannot add `&&str` to `&&str`
   |
   |
LL |     let _ = &c + &d; //~ ERROR cannot add
   |             -- ^ -- &&str
   |             &&str


error[E0369]: cannot add `&str` to `&&str`
   |
   |
LL |     let _ = &c + d; //~ ERROR cannot add
   |             -- ^ - &str
   |             &&str


error[E0369]: cannot add `&&str` to `&str`
   |
   |
LL |     let _ = c + &d; //~ ERROR cannot add
   |             - ^ -- &&str
   |             | |
   |             | `+` cannot be used to concatenate two `&str` strings
   |             &str
   |
   = note: string concatenation requires an owned `String` on the left
help: create an owned `String` from a string reference
   |
LL |     let _ = c.to_owned() + &d; //~ ERROR cannot add

error[E0369]: cannot add `&str` to `&str`
  --> /checkout/src/test/ui/span/issue-39018.rs:37:15
   |
   |
LL |     let _ = c + d; //~ ERROR cannot add
   |             - ^ - &str
   |             | |
   |             | `+` cannot be used to concatenate two `&str` strings
   |             &str
   |
   = note: string concatenation requires an owned `String` on the left
help: create an owned `String` from a string reference
   |
LL |     let _ = c.to_owned() + d; //~ ERROR cannot add

error: aborting due to 14 previous errors

Some errors have detailed explanations: E0308, E0369.
---
To only update this specific test, also pass `--test-args span/coerce-suggestions.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/span/coerce-suggestions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/coerce-suggestions" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/coerce-suggestions/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/span/coerce-suggestions.rs:7:20
   |
LL |     let x: usize = String::new();
   |            -----   ^^^^^^^^^^^^^ expected `usize`, found struct `String`
---
                      found reference `&String`
note: function defined here
  --> /checkout/src/test/ui/span/coerce-suggestions.rs:1:4
   |
LL | fn test(_x: &mut String) {}

error[E0308]: mismatched types
  --> /checkout/src/test/ui/span/coerce-suggestions.rs:14:11
   |
---
                      found reference `&String`
note: function defined here
  --> /checkout/src/test/ui/span/coerce-suggestions.rs:3:4
   |
LL | fn test2(_x: &mut i32) {}

error[E0308]: mismatched types
  --> /checkout/src/test/ui/span/coerce-suggestions.rs:17:9
   |
   |
LL |     f = Box::new(f);
   |         ^^^^^^^^^^^ cyclic type of infinite size
help: consider unboxing the value
   |
   |
LL |     f = *Box::new(f);

error[E0308]: mismatched types
  --> /checkout/src/test/ui/span/coerce-suggestions.rs:21:9
   |
---

---- [ui] src/test/ui/span/issue-33884.rs stdout ----
diff of stderr:

4 LL |     stream.write_fmt(format!("message received"))
5    |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Arguments`, found struct `String`
+    = note: expected struct `Arguments<'_>`
+               found struct `String`
7    = note: this error originates in the macro `format` (in Nightly builds, run with -Z macro-backtrace for more info)
8 
---
To only update this specific test, also pass `--test-args span/issue-33884.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/span/issue-33884.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-33884" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-33884/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/span/issue-33884.rs:6:22
   |
   |
LL |     stream.write_fmt(format!("message received"))
   |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `Arguments`, found struct `String`
   = note: expected struct `Arguments<'_>`
              found struct `String`
   = note: this error originates in the macro `format` (in Nightly builds, run with -Z macro-backtrace for more info)

---

14 LL |     default type U = str;
15    |                      ^^^ the trait `Clone` is not implemented for `str`
16    |
-    = help: the trait `Clone` is implemented for `String`
+    = help: the trait `Clone` is implemented for `String<A>`
18 note: required by a bound in `X::U`
20    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/default-associated-type-bound-1/default-associated-type-bound-1.stderr
To only update this specific test, also pass `--test-args specialization/default-associated-type-bound-1.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/specialization/default-associated-type-bound-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/default-associated-type-bound-1" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/default-associated-type-bound-1/auxiliary"
stdout: none
--- stderr -------------------------------
warning: the feature `specialization` is incomplete and may not be safe to use and/or cause compiler crashes
   |
LL | #![feature(specialization)]
   |            ^^^^^^^^^^^^^^
   |
---
   |
LL |     default type U = str;
   |                      ^^^ the trait `Clone` is not implemented for `str`
   |
   = help: the trait `Clone` is implemented for `String<A>`
note: required by a bound in `X::U`
   |
   |
LL |     type U: Clone;
   |             ^^^^^ required by this bound in `X::U`
error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0277`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/static/bad-const-type.rs stdout ----
diff of stderr:

5    |                    ^^- help: try using a conversion method: `.to_string()`
7    |                    expected struct `String`, found integer
+    |
+    = note: expected struct `String`
+                 found type `{integer}`
---
To only update this specific test, also pass `--test-args static/bad-const-type.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/static/bad-const-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/static/bad-const-type" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/static/bad-const-type/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/static/bad-const-type.rs:1:20
   |
   |
LL | static i: String = 10;
   |                    ^^- help: try using a conversion method: `.to_string()`
   |                    expected struct `String`, found integer
   |
   = note: expected struct `String`
                found type `{integer}`
---

---- [ui] src/test/ui/suggestions/const-in-struct-pat.rs stdout ----
diff of stderr:

10    |                 expected struct `String`, found struct `foo`
11    |                 `foo` is interpreted as a unit struct, not a new binding
+    = note: expected struct `String`
+               found struct `foo`
13 help: bind the struct field to a different name instead
14    |
14    |
15 LL |     let Thing { foo: other_foo } = t;

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/const-in-struct-pat/const-in-struct-pat.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args suggestions/const-in-struct-pat.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/const-in-struct-pat.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/const-in-struct-pat" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/const-in-struct-pat/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/suggestions/const-in-struct-pat.rs:8:17
   |
LL | struct foo;
   | ---------- unit struct defined here
   | ---------- unit struct defined here
...
LL |     let Thing { foo } = t; //~ ERROR mismatched types
   |                 ^^^     - this expression has type `Thing`
   |                 expected struct `String`, found struct `foo`
   |                 expected struct `String`, found struct `foo`
   |                 `foo` is interpreted as a unit struct, not a new binding
   = note: expected struct `String`
              found struct `foo`
help: bind the struct field to a different name instead
   |
   |
LL |     let Thing { foo: other_foo } = t; //~ ERROR mismatched types

error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
For more information about this error, try `rustc --explain E0308`.
------------------------------------------


---- [ui] src/test/ui/suggestions/chain-method-call-mutation-in-place.rs stdout ----
diff of stderr:

6 LL |     s.push_str("asdf")
7    |     ^^^^^^^^^^^^^^^^^^ expected struct `String`, found `()`
+    = note: expected struct `String`
+            found unit type `()`
+            found unit type `()`
9 note: method `push_str` modifies its receiver in-place
11    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/chain-method-call-mutation-in-place/chain-method-call-mutation-in-place.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args suggestions/chain-method-call-mutation-in-place.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/chain-method-call-mutation-in-place.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/chain-method-call-mutation-in-place" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/chain-method-call-mutation-in-place/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/suggestions/chain-method-call-mutation-in-place.rs:3:5
   |
   |
LL | fn foo(mut s: String) -> String {
   |                          ------ expected `String` because of return type
LL |     s.push_str("asdf") //~ ERROR mismatched types
   |     ^^^^^^^^^^^^^^^^^^ expected struct `String`, found `()`
   = note: expected struct `String`
           found unit type `()`
           found unit type `()`
note: method `push_str` modifies its receiver in-place
   |
   |
LL |     s.push_str("asdf") //~ ERROR mismatched types
   |     - ^^^^^^^^ this call modifies `s` in-place
   |     you probably want to use this value after calling the method...
   |     you probably want to use this value after calling the method...
   = note: ...instead of the `()` output of method `push_str`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
------------------------------------------
---
To only update this specific test, also pass `--test-args suggestions/dont-suggest-deref-inside-macro-issue-58298.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/dont-suggest-deref-inside-macro-issue-58298.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/dont-suggest-deref-inside-macro-issue-58298" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/dont-suggest-deref-inside-macro-issue-58298/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/suggestions/dont-suggest-deref-inside-macro-issue-58298.rs:11:5
   |
   |
LL | /     intrinsic_match! {
LL | |         "abc"
LL | |     };
   | |_____^ expected `&str`, found struct `String`
   = note: expected reference `&str`
                 found struct `String`
   = note: this error originates in the macro `format` which comes from the expansion of the macro `intrinsic_match` (in Nightly builds, run with -Z macro-backtrace for more info)

---
+    = note: expected struct `String`
+            found reference `&String`
9 help: consider removing the borrow
10    |
11 LL -     let a: String = &String::from("a");
20    |            |
21    |            expected due to this
22    |
+    = note: expected struct `String`
+    = note: expected struct `String`
+            found reference `&String`
23 help: consider removing the borrow
24    |
25 LL -     let b: String = &format!("b");
34    |            |
35    |            expected due to this
36    |
+    = note:         expected struct `String`
+    = note:         expected struct `String`
+            found mutable reference `&mut String`
37 help: consider removing the borrow
38    |
39 LL -     let c: String = &mut format!("c");
48    |            |
49    |            expected due to this
50    |
+    = note:         expected struct `String`
+    = note:         expected struct `String`
+            found mutable reference `&mut String`
51 help: consider removing the borrow
52    |
53 LL -     let d: String = &mut (format!("d"));

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/format-borrow/format-borrow.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args suggestions/format-borrow.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/format-borrow.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/format-borrow" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/format-borrow/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/suggestions/format-borrow.rs:2:21
   |
   |
LL |     let a: String = &String::from("a");
   |            ------   ^^^^^^^^^^^^^^^^^^ expected struct `String`, found `&String`
   |            expected due to this
   |
   = note: expected struct `String`
           found reference `&String`
           found reference `&String`
help: consider removing the borrow
   |
LL -     let a: String = &String::from("a");
LL +     let a: String = String::from("a");

error[E0308]: mismatched types
  --> /checkout/src/test/ui/suggestions/format-borrow.rs:4:21
   |
   |
LL |     let b: String = &format!("b");
   |            ------   ^^^^^^^^^^^^^ expected struct `String`, found `&String`
   |            expected due to this
   |
   = note: expected struct `String`
           found reference `&String`
           found reference `&String`
help: consider removing the borrow
   |
LL -     let b: String = &format!("b");
LL +     let b: String = format!("b");

error[E0308]: mismatched types
  --> /checkout/src/test/ui/suggestions/format-borrow.rs:6:21
   |
   |
LL |     let c: String = &mut format!("c");
   |            ------   ^^^^^^^^^^^^^^^^^ expected struct `String`, found `&mut String`
   |            expected due to this
   |
   = note:         expected struct `String`
           found mutable reference `&mut String`
           found mutable reference `&mut String`
help: consider removing the borrow
   |
LL -     let c: String = &mut format!("c");
LL +     let c: String = format!("c");

error[E0308]: mismatched types
  --> /checkout/src/test/ui/suggestions/format-borrow.rs:8:21
   |
   |
LL |     let d: String = &mut (format!("d"));
   |            ------   ^^^^^^^^^^^^^^^^^^^ expected struct `String`, found `&mut String`
   |            expected due to this
   |
   = note:         expected struct `String`
           found mutable reference `&mut String`
           found mutable reference `&mut String`
help: consider removing the borrow
   |
LL -     let d: String = &mut (format!("d"));
LL +     let d: String = format!("d"));

error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0308`.
For more information about this error, try `rustc --explain E0308`.
------------------------------------------


---- [ui] src/test/ui/suggestions/into-str.rs stdout ----
diff of stderr:

11              <String as From<&String>>
12              <String as From<&mut str>>
13              <String as From<&str>>
-              <String as From<Box<str>>>
15              <String as From<Cow<'a, str>>>
16              <String as From<char>>
+              <String<A> as From<Box<str, A>>>
17    = note: required for `String` to implement `Into<&str>`
18 note: required by a bound in `foo`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/into-str/into-str.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/into-str/into-str.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args suggestions/into-str.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/into-str.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/into-str" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/into-str/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the trait bound `&str: From<String>` is not satisfied
   |
LL |     foo(String::new());
LL |     foo(String::new());
   |     --- ^^^^^^^^^^^^^ the trait `From<String>` is not implemented for `&str`
   |     required by a bound introduced by this call
   |
   |
   = note: to coerce a `String` into a `&str`, use `&*` as a prefix
   = help: the following other types implement trait `From<T>`:
             <String as From<&String>>
             <String as From<&mut str>>
             <String as From<&str>>
             <String as From<Cow<'a, str>>>
             <String as From<char>>
             <String<A> as From<Box<str, A>>>
   = note: required for `String` to implement `Into<&str>`
note: required by a bound in `foo`
   |
   |
LL | fn foo<'a, T>(_t: T) where T: Into<&'a str> {}
   |                               ^^^^^^^^^^^^^ required by this bound in `foo`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/suggestions/issue-59819.rs stdout ----
diff of stderr:

32    |            |        |
33    |            |        expected struct `String`, found struct `Bar`
+    |
+    = note: expected struct `String`
+               found struct `Bar`
35 
---
To only update this specific test, also pass `--test-args suggestions/issue-59819.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/issue-59819.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-59819" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-59819/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/suggestions/issue-59819.rs:28:18
   |
   |
LL |     let y: i32 = x; //~ ERROR mismatched types
   |            ---   ^ expected `i32`, found struct `Foo`
   |            expected due to this
   |
help: consider dereferencing the type
   |
   |
LL |     let y: i32 = *x; //~ ERROR mismatched types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/suggestions/issue-59819.rs:30:18
   |
   |
LL |     let b: i32 = a; //~ ERROR mismatched types
   |            ---   ^ expected `i32`, found `&{integer}`
   |            expected due to this
   |
help: consider dereferencing the borrow
   |
   |
LL |     let b: i32 = *a; //~ ERROR mismatched types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/suggestions/issue-59819.rs:34:21
   |
   |
LL |     let g: String = f; //~ ERROR mismatched types
   |            ------   ^- help: try using a conversion method: `.to_string()`
   |            |        |
   |            |        expected struct `String`, found struct `Bar`
   |
   = note: expected struct `String`
              found struct `Bar`

---
+    = note: expected struct `String`
+            found reference `&str`
7 help: try using a conversion method
8    |
9 LL |         guts: guts.to_string(),
17    |                 |    |
18    |                 |    help: try using a conversion method: `to_string`
19    |                 expected struct `String`, found `&str`
+    |
---
To only update this specific test, also pass `--test-args suggestions/issue-52820.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/issue-52820.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-52820" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-52820/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/suggestions/issue-52820.rs:12:9
   |
   |
LL |         guts, //~ ERROR mismatched types
   |         ^^^^ expected struct `String`, found `&str`
   = note: expected struct `String`
           found reference `&str`
help: try using a conversion method
   |
   |
LL |         guts: guts.to_string(), //~ ERROR mismatched types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/suggestions/issue-52820.rs:13:17
   |
   |
LL |         brains: guts.clone(), //~ ERROR mismatched types
   |                 |    |
   |                 |    help: try using a conversion method: `to_string`
   |                 expected struct `String`, found `&str`
   |
---

---- [ui] src/test/ui/suggestions/issue-53692.rs stdout ----
diff of stderr:

20    |                 |        | help: try using a conversion method: `to_string`
21    |                 |        expected struct `String`, found `&str`
+    |
+    = note: expected struct `String`
+            found reference `&str`
23 
---
To only update this specific test, also pass `--test-args suggestions/issue-53692.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/issue-53692.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-53692" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-53692/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/suggestions/issue-53692.rs:7:33
   |
   |
LL |     let items_clone: Vec<i32> = ref_items.clone();
   |                      |          |         |
   |                      |          |         help: try using a conversion method: `to_vec`
   |                      |          |         help: try using a conversion method: `to_vec`
   |                      |          expected struct `Vec`, found `&[i32]`
   |
   = note: expected struct `Vec<i32>`
           found reference `&[i32]`


error[E0308]: mismatched types
  --> /checkout/src/test/ui/suggestions/issue-53692.rs:14:26
   |
LL |     let string: String = s.clone();
   |                 ------   ^^-----^^
   |                 |        | |
   |                 |        | help: try using a conversion method: `to_string`
   |                 |        expected struct `String`, found `&str`
   |
   = note: expected struct `String`
           found reference `&str`

---

---- [ui] src/test/ui/suggestions/issue-83943.rs stdout ----
diff of stderr:

11    | |         expected struct `String`, found `&str`
12 LL | |     };
13    | |_____- `if` and `else` have incompatible types
+    = note: expected struct `String`
+            found reference `&str`
14 
15 error: aborting due to previous error
---
To only update this specific test, also pass `--test-args suggestions/issue-83943.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/issue-83943.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-83943" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-83943/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0308]: `if` and `else` have incompatible types
   |
LL | /     if true {
LL | /     if true {
LL | |         "A".to_string()
LL | |     } else {
LL | |     } else {
LL | |         "B" //~ ERROR `if` and `else` have incompatible types
   | |         ^^^- help: try using a conversion method: `.to_string()`
   | |         expected struct `String`, found `&str`
LL | |     };
LL | |     };
   | |_____- `if` and `else` have incompatible types
   = note: expected struct `String`
           found reference `&str`

error: aborting due to previous error
---
+    = note: expected struct `String`
+            found unit type `()`
23 help: consider returning the local binding `s`
24    |
25 LL ~     let s: String = if let Some(s) = opt_str {
52 LL | |     } else {
53    | |_____^ expected struct `String`, found `()`
54    |
+    = note: expected struct `String`
+    = note: expected struct `String`
+            found unit type `()`
55 help: consider returning the local binding `s`
56    |
57 LL ~     let s: String = if let Some(s) = opt_str {
69 LL |           String::new()
70    |           ^^^^^^^^^^^^^ expected `()`, found struct `String`
71    |
+    = note: expected unit type `()`
+    = note: expected unit type `()`
+                  found struct `String`
72 help: consider returning the local binding `s`
73    |
74 LL ~     let s = if let Some(s) = opt_str {
82 LL |         Some(s) => {}
83    |                    ^^ expected struct `String`, found `()`
84    |
+    = note: expected struct `String`
+    = note: expected struct `String`
+            found unit type `()`
85 help: consider returning the local binding `s`
86    |
87 LL |         Some(s) => { s }
100 LL | |     };
101    | |_____- `match` arms have incompatible types
102    |
+    = note: expected unit type `()`
+    = note: expected unit type `()`
+                  found struct `String`
103 help: consider returning the local binding `s`
104    |
105 LL |         Some(s) => { s }

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/return-bindings/return-bindings.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args suggestions/return-bindings.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/return-bindings.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/return-bindings" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/return-bindings/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/suggestions/return-bindings.rs:3:17
   |
   |
LL | fn a(i: i32) -> i32 {}
   |    -            ^^^ expected `i32`, found `()`
   |    |
   |    implicitly returns `()` as its body has no tail or `return` expression
help: consider returning the local binding `i`
   |
   |
LL | fn a(i: i32) -> i32 { i }

error[E0308]: mismatched types
  --> /checkout/src/test/ui/suggestions/return-bindings.rs:7:46
   |
   |
LL |       let s: String = if let Some(s) = opt_str {
   |  ______________________________________________^
LL | |         //~^ ERROR mismatched types
LL | |     } else {
   | |_____^ expected struct `String`, found `()`
   = note: expected struct `String`
           found unit type `()`
help: consider returning the local binding `s`
   |
   |
LL ~     let s: String = if let Some(s) = opt_str {
LL +         s
LL ~     //~^ ERROR mismatched types

error[E0308]: mismatched types
  --> /checkout/src/test/ui/suggestions/return-bindings.rs:14:11
   |
   |
LL | fn c() -> Option<i32> {
   |    |
   |    |
   |    implicitly returns `()` as its body has no tail or `return` expression
   = note:   expected enum `Option<i32>`
           found unit type `()`
help: consider returning the local binding `x`
   |
   |
LL ~     let x = Some(1);
LL +     x
   |

error[E0308]: mismatched types
  --> /checkout/src/test/ui/suggestions/return-bindings.rs:20:46
   |
LL |       let s: String = if let Some(s) = opt_str {
   |  ______________________________________________^
LL | |         //~^ ERROR mismatched types
LL | |     } else {
   | |_____^ expected struct `String`, found `()`
   = note: expected struct `String`
           found unit type `()`
help: consider returning the local binding `s`
   |
   |
LL ~     let s: String = if let Some(s) = opt_str {
LL +         s
LL ~     //~^ ERROR mismatched types


error[E0308]: `if` and `else` have incompatible types
   |
   |
LL |       let s = if let Some(s) = opt_str {
LL | |     } else {
   | |_____- expected because of this
LL |           String::new()
   |           ^^^^^^^^^^^^^ expected `()`, found struct `String`
   |           ^^^^^^^^^^^^^ expected `()`, found struct `String`
   |
   = note: expected unit type `()`
                 found struct `String`
help: consider returning the local binding `s`
   |
LL ~     let s = if let Some(s) = opt_str {
LL +         s
LL ~     } else {

error[E0308]: mismatched types
  --> /checkout/src/test/ui/suggestions/return-bindings.rs:37:20
   |
   |
LL |         Some(s) => {}
   |                    ^^ expected struct `String`, found `()`
   |
   = note: expected struct `String`
           found unit type `()`
help: consider returning the local binding `s`
   |
LL |         Some(s) => { s }

error[E0308]: `match` arms have incompatible types
  --> /checkout/src/test/ui/suggestions/return-bindings.rs:46:17
   |
   |
LL |       let s = match opt_str {
   |  _____________-
LL | |         Some(s) => {}
LL | |         None => String::new(),
   | |                 ^^^^^^^^^^^^^ expected `()`, found struct `String`
   | |                 ^^^^^^^^^^^^^ expected `()`, found struct `String`
LL | |         //~^ ERROR `match` arms have incompatible types
LL | |     };
   |
   = note: expected unit type `()`
                 found struct `String`
help: consider returning the local binding `s`
help: consider returning the local binding `s`
   |
LL |         Some(s) => { s }

error: aborting due to 7 previous errors

For more information about this error, try `rustc --explain E0308`.
---
To only update this specific test, also pass `--test-args switched-expectations.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/switched-expectations.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/switched-expectations" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/switched-expectations/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/switched-expectations.rs:3:30
   |
   |
LL |     let ref string: String = var; //~ ERROR mismatched types [E0308]
   |                              ^^^ expected struct `String`, found `i32`
   = note: expected struct `String`
                found type `i32`

error: aborting due to previous error
---

---- [ui] src/test/ui/traits/associated_type_bound/check-trait-object-bounds-1.rs stdout ----
diff of stderr:

4 LL |     f::<dyn X<Y = str>>();
5    |         ^^^^^^^^^^^^^^ the trait `Clone` is not implemented for `str`
-    = help: the trait `Clone` is implemented for `String`
+    = help: the trait `Clone` is implemented for `String<A>`
8 note: required by a bound in `f`
9   --> $DIR/check-trait-object-bounds-1.rs:7:9
9   --> $DIR/check-trait-object-bounds-1.rs:7:9
10    |


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/associated_type_bound/check-trait-object-bounds-1/check-trait-object-bounds-1.stderr
To only update this specific test, also pass `--test-args traits/associated_type_bound/check-trait-object-bounds-1.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/associated_type_bound/check-trait-object-bounds-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/associated_type_bound/check-trait-object-bounds-1" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/associated_type_bound/check-trait-object-bounds-1/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the trait bound `str: Clone` is not satisfied
   |
   |
LL |     f::<dyn X<Y = str>>();
   |         ^^^^^^^^^^^^^^ the trait `Clone` is not implemented for `str`
   = help: the trait `Clone` is implemented for `String<A>`
note: required by a bound in `f`
  --> /checkout/src/test/ui/traits/associated_type_bound/check-trait-object-bounds-1.rs:7:9
   |
   |
LL | fn f<T: X + ?Sized>() {
   |         ^ required by this bound in `f`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/traits/associated_type_bound/check-trait-object-bounds-4.rs stdout ----
diff of stderr:

4 LL |     f::<dyn X<Y = str>>();
5    |         ^^^^^^^^^^^^^^ the trait `Clone` is not implemented for `str`
-    = help: the trait `Clone` is implemented for `String`
+    = help: the trait `Clone` is implemented for `String<A>`
8 note: required by a bound in `f`
9   --> $DIR/check-trait-object-bounds-4.rs:10:9
9   --> $DIR/check-trait-object-bounds-4.rs:10:9
10    |


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/associated_type_bound/check-trait-object-bounds-4/check-trait-object-bounds-4.stderr
To only update this specific test, also pass `--test-args traits/associated_type_bound/check-trait-object-bounds-4.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/associated_type_bound/check-trait-object-bounds-4.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/associated_type_bound/check-trait-object-bounds-4" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/associated_type_bound/check-trait-object-bounds-4/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the trait bound `str: Clone` is not satisfied
   |
   |
LL |     f::<dyn X<Y = str>>();
   |         ^^^^^^^^^^^^^^ the trait `Clone` is not implemented for `str`
   = help: the trait `Clone` is implemented for `String<A>`
note: required by a bound in `f`
  --> /checkout/src/test/ui/traits/associated_type_bound/check-trait-object-bounds-4.rs:10:9
   |
   |
LL | fn f<T: X + ?Sized>() {
   |         ^ required by this bound in `f`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/traits/bound/assoc-fn-bound-root-obligation.rs stdout ----
diff of stderr:

6    |
7    = help: the trait `FnMut<(char,)>` is not implemented for `u8`
8    = help: the following other types implement trait `Pattern<'a>`:
-              &'b String
+              &'b String<A>
10              &'b [char; N]
11              &'b [char]
12              &'b str

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/bound/assoc-fn-bound-root-obligation/assoc-fn-bound-root-obligation.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/bound/assoc-fn-bound-root-obligation.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/bound/assoc-fn-bound-root-obligation.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/bound/assoc-fn-bound-root-obligation" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/bound/assoc-fn-bound-root-obligation/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: expected a `FnMut<(char,)>` closure, found `u8`
   |
   |
LL |     s.strip_suffix(b'\n').unwrap_or(s)
   |       ^^^^^^^^^^^^ expected an `FnMut<(char,)>` closure, found `u8`
   |
   = help: the trait `FnMut<(char,)>` is not implemented for `u8`
   = help: the following other types implement trait `Pattern<'a>`:
             &'b String<A>
             &'b [char; N]
             &'b [char]
             &'b str
             &'c &'b str
             [char; N]
             char
   = note: required for `u8` to implement `Pattern<'_>`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/traits/issue-77982.rs stdout ----
diff of stderr:

31    = note: multiple `impl`s satisfying `String: AsRef<_>` found in the following crates: `alloc`, `std`:
32            - impl AsRef<OsStr> for String;
33            - impl AsRef<Path> for String;
-            - impl AsRef<[u8]> for String;
-            - impl AsRef<str> for String;
+            - impl<A> AsRef<[u8]> for String<A>
+              where A: Allocator;
+            - impl<A> AsRef<str> for String<A>
+              where A: Allocator;
36 help: consider specifying the generic argument
37    |
38 LL |     opts.get::<Q>(opt.as_ref());

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-77982/issue-77982.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/issue-77982.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/issue-77982.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-77982" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-77982/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/traits/issue-77982.rs:8:10
   |
   |
LL |     opts.get(opt.as_ref()); //~ ERROR type annotations needed
   |          ^^^ ------------ type must be known at this point
   |          |
   |          cannot infer type of the type parameter `Q` declared on the associated function `get`
   |
   = note: multiple `impl`s satisfying `String: Borrow<_>` found in the following crates: `alloc`, `core`:
           - impl Borrow<str> for String;
           - impl<T> Borrow<T> for T
             where T: ?Sized;
note: required by a bound in `HashMap::<K, V, S>::get`
   |
   |
LL |         K: Borrow<Q>,
   |            ^^^^^^^^^ required by this bound in `HashMap::<K, V, S>::get`
help: consider specifying the generic argument
   |
LL |     opts.get::<Q>(opt.as_ref()); //~ ERROR type annotations needed

error[E0283]: type annotations needed
  --> /checkout/src/test/ui/traits/issue-77982.rs:8:10
   |
   |
LL |     opts.get(opt.as_ref()); //~ ERROR type annotations needed
   |          ^^^     ------ type must be known at this point
   |          |
   |          cannot infer type of the type parameter `Q` declared on the associated function `get`
   |
   = note: multiple `impl`s satisfying `String: AsRef<_>` found in the following crates: `alloc`, `std`:
           - impl AsRef<OsStr> for String;
           - impl AsRef<Path> for String;
           - impl<A> AsRef<[u8]> for String<A>
             where A: Allocator;
           - impl<A> AsRef<str> for String<A>
             where A: Allocator;
help: consider specifying the generic argument
   |
LL |     opts.get::<Q>(opt.as_ref()); //~ ERROR type annotations needed

error[E0283]: type annotations needed
  --> /checkout/src/test/ui/traits/issue-77982.rs:13:59
   |
   |
LL |     let ips: Vec<_> = (0..100_000).map(|_| u32::from(0u32.into())).collect();
   |                                            |
   |                                            required by a bound introduced by this call
   |
   |
   = note: multiple `impl`s satisfying `u32: From<_>` found in the following crates: `core`, `std`:
           - impl From<Ipv4Addr> for u32;
           - impl From<NonZeroU32> for u32;
           - impl From<bool> for u32;
           - impl From<char> for u32;
           and 3 more
help: try using a fully qualified path to specify the expected types
   |
LL |     let ips: Vec<_> = (0..100_000).map(|_| u32::from(<u32 as Into<T>>::into(0u32))).collect();

error[E0283]: type annotations needed for `Box<T>`
  --> /checkout/src/test/ui/traits/issue-77982.rs:36:9
   |
   |
LL |     let _ = ().foo(); //~ ERROR type annotations needed
   |         ^      --- type must be known at this point
   |
note: multiple `impl`s satisfying `(): Foo<'_, _>` found
   |
   |
LL | impl Foo<'static, u32> for () {}
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
LL | impl<'a> Foo<'a, i16> for () {}
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: consider giving this pattern a type, where the type for type parameter `T` is specified
   |
LL |     let _: Box<T> = ().foo(); //~ ERROR type annotations needed

error[E0283]: type annotations needed for `Box<T>`
  --> /checkout/src/test/ui/traits/issue-77982.rs:40:9
   |
   |
LL |     let _ = (&()).bar(); //~ ERROR type annotations needed
   |         ^         --- type must be known at this point
   |
note: multiple `impl`s satisfying `&(): Bar<'_, _>` found
   |
   |
LL | impl<'a> Bar<'static, u32> for &'a () {}
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
LL | impl<'a> Bar<'a, i16> for &'a () {}
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: consider giving this pattern a type, where the type for type parameter `T` is specified
   |
LL |     let _: Box<T> = (&()).bar(); //~ ERROR type annotations needed

error: aborting due to 5 previous errors

For more information about this error, try `rustc --explain E0283`.
---
diff of stderr:

11    |    ---                 ^^^^^^ expected struct `String`, found `()`
12    |    |
13    |    implicitly returns `()` as its body has no tail or `return` expression
+    = note: expected struct `String`
+            found unit type `()`
14 
15 error: aborting due to 2 previous errors
---
To only update this specific test, also pass `--test-args typeck/issue-67971.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/typeck/issue-67971.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-67971" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-67971/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0609]: no field `sleep` on type `&mut S`
   |
   |
LL |     ctx.sleep = 0;

error[E0308]: mismatched types
  --> /checkout/src/test/ui/typeck/issue-67971.rs:3:24
   |
   |
LL | fn foo(ctx: &mut S) -> String { //~ ERROR mismatched types
   |    ---                 ^^^^^^ expected struct `String`, found `()`
   |    |
   |    implicitly returns `()` as its body has no tail or `return` expression
   = note: expected struct `String`
           found unit type `()`

error: aborting due to 2 previous errors
---
To only update this specific test, also pass `--test-args typeck/conversion-methods.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/typeck/conversion-methods.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/conversion-methods" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/conversion-methods/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/typeck/conversion-methods.rs:5:41
   |
   |
LL |     let _tis_an_instants_play: String = "'Tis a fond Ambush—"; //~ ERROR mismatched types
   |                                ------   ^^^^^^^^^^^^^^^^^^^^^- help: try using a conversion method: `.to_string()`
   |                                |        expected struct `String`, found `&str`
   |                                expected due to this
   |
   = note: expected struct `String`
   = note: expected struct `String`
           found reference `&'static str`

error[E0308]: mismatched types
  --> /checkout/src/test/ui/typeck/conversion-methods.rs:6:40
   |
LL |     let _just_to_make_bliss: PathBuf = Path::new("/ern/her/own/surprise");
   |                              -------   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^- help: try using a conversion method: `.to_path_buf()`
   |                              |         expected struct `PathBuf`, found `&Path`
   |                              expected due to this

error[E0308]: mismatched types
error[E0308]: mismatched types
  --> /checkout/src/test/ui/typeck/conversion-methods.rs:9:40
   |
LL |     let _but_should_the_play: String = 2; // Perhaps surprisingly, we suggest .to_string() here
   |                               ------   ^- help: try using a conversion method: `.to_string()`
   |                               |        expected struct `String`, found integer
   |                               expected due to this
   |
   = note: expected struct `String`
   = note: expected struct `String`
                found type `{integer}`

error[E0308]: mismatched types
  --> /checkout/src/test/ui/typeck/conversion-methods.rs:12:47
   |
LL |     let _prove_piercing_earnest: Vec<usize> = &[1, 2, 3]; //~ ERROR mismatched types
   |                                  ----------   ^^^^^^^^^^ expected struct `Vec`, found `&[{integer}; 3]`
   |                                  expected due to this
   |
   = note: expected struct `Vec<usize>`
   = note: expected struct `Vec<usize>`
           found reference `&[{integer}; 3]`
help: try using a conversion method
   |
LL |     let _prove_piercing_earnest: Vec<usize> = (&[1, 2, 3]).to_vec(); //~ ERROR mismatched types

error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0308`.
