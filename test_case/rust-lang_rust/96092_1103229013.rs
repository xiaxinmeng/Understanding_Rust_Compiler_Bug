plain
........................................................................................ 88/12952
..............................................................iiiiiiiiiiii.............. 176/12952
i.i.................i...i............................................................... 264/12952
........................................................................................ 352/12952
......................................................................................FF 440/12952
......................F..F.............................................................. 528/12952
........................................................................................ 704/12952
..........................i............................................................. 792/12952
......................i................................................................. 880/12952
........................................................................................ 968/12952
---
........................................................................................ 3872/12952
.........i.............................................................................. 3960/12952
....................................................i................................... 4048/12952
........................................................................................ 4136/12952
................................................................F.............F......... 4224/12952
..........................................................................F............. 4312/12952
............................F......F.................................................... 4400/12952
........................................................................................ 4576/12952
........................................................................................ 4664/12952
........................................................................................ 4752/12952
........................................................................................ 4840/12952
---
........................................................................................ 6072/12952
........................................................................................ 6160/12952
..............................................................................i......... 6248/12952
........................................................................................ 6336/12952
...............F.................F................................................i..... 6424/12952
.......................................................................i................ 6600/12952
....................................ii.ii........i...i.................................. 6688/12952
........................................................................................ 6776/12952
................................................................i....i.................. 6864/12952
---
..................................ii.iiiii..iiiiii.i.................................... 10648/12952
........................................................................................ 10736/12952
........................................................................................ 10824/12952
........................................................................................ 10912/12952
...............................F..................F..................................... 11000/12952
........................................................................................ 11176/12952
........................................................................................ 11264/12952
........................................................................................ 11352/12952
........................................................................................ 11440/12952
........................................................................................ 11440/12952
....................................................i........i.....i.................... 11528/12952
........i............................................................................... 11616/12952
........................................................................................ 11704/12952
............F......................................F..F....F.F.......................... 11792/12952
........................................................................................ 11968/12952
........................................................................................ 12056/12952
........................................................................i............... 12144/12952
........................................................................................ 12232/12952
---

---- [ui] src/test/ui/associated-types/defaults-cyclic-fail-2.rs stdout ----
diff of stderr:

- error[E0275]: overflow evaluating the requirement `<bool as Tr>::B == _`
+ error[E0282]: type annotations needed
3    |
3    |
4 LL |     type A = Box<Self::B>;
-    |              ^^^^^^^^^^^^
+    |              ^^^^^^^^^^^^ cannot infer type
6 
6 
- error[E0275]: overflow evaluating the requirement `<usize as Tr>::A == _`
-   --> $DIR/defaults-cyclic-fail-2.rs:33:14
-    |
- LL |     type B = &'static Self::A;
+ error: aborting due to previous error
12 
- error: aborting due to 2 previous errors
- 
- 
- For more information about this error, try `rustc --explain E0275`.
+ For more information about this error, try `rustc --explain E0282`.
16 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/defaults-cyclic-fail-2/defaults-cyclic-fail-2.stderr
To only update this specific test, also pass `--test-args associated-types/defaults-cyclic-fail-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/defaults-cyclic-fail-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/defaults-cyclic-fail-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/defaults-cyclic-fail-2/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/associated-types/defaults-cyclic-fail-2.rs:27:14
   |
   |
LL |     type A = Box<Self::B>;

error: aborting due to previous error

For more information about this error, try `rustc --explain E0282`.
For more information about this error, try `rustc --explain E0282`.
------------------------------------------


---- [ui] src/test/ui/associated-types/defaults-cyclic-fail-1.rs stdout ----
diff of stderr:

- error[E0275]: overflow evaluating the requirement `<bool as Tr>::B == _`
+ error[E0282]: type annotations needed
3    |
3    |
4 LL |     type A = Box<Self::B>;
-    |              ^^^^^^^^^^^^
+    |              ^^^^^^^^^^^^ cannot infer type
6 
6 
- error[E0275]: overflow evaluating the requirement `<usize as Tr>::A == _`
-   --> $DIR/defaults-cyclic-fail-1.rs:32:14
-    |
- LL |     type B = &'static Self::A;
+ error: aborting due to previous error
12 
- error: aborting due to 2 previous errors
- 
- 
- For more information about this error, try `rustc --explain E0275`.
+ For more information about this error, try `rustc --explain E0282`.
16 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/defaults-cyclic-fail-1/defaults-cyclic-fail-1.stderr
To only update this specific test, also pass `--test-args associated-types/defaults-cyclic-fail-1.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/defaults-cyclic-fail-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/defaults-cyclic-fail-1" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/defaults-cyclic-fail-1/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/associated-types/defaults-cyclic-fail-1.rs:26:14
   |
   |
LL |     type A = Box<Self::B>;
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

error: aborting due to previous error

For more information about this error, try `rustc --explain E0282`.
For more information about this error, try `rustc --explain E0282`.
------------------------------------------


---- [ui] src/test/ui/associated-types/impl-wf-cycle-1.rs stdout ----
diff of stderr:

- error[E0275]: overflow evaluating the requirement `<(T,) as Grault>::A == _`
+ error[E0283]: type annotations needed
3    |
3    |
4 LL | / impl<T: Grault> Grault for (T,)
8 ...  |
9 LL | |
10 LL | | }
-    | |_^
-    | |_^
+    | |_^ cannot infer type
12    |
+    = note: cannot satisfy `_: Baz`
13 note: required because of the requirements on the impl of `Grault` for `(T,)`
15    |

18    = note: 1 redundant requirement hidden
18    = note: 1 redundant requirement hidden
19    = note: required because of the requirements on the impl of `Grault` for `(T,)`
20 
- error[E0275]: overflow evaluating the requirement `<(T,) as Grault>::A == _`
+ error[E0283]: type annotations needed
+    |
+    |
+ LL | / impl<T: Grault> Grault for (T,)
+ LL | | where
+ LL | |     Self::A: Baz,
+ LL | |     Self::B: Fiz,
+ LL | |
+ LL | | }
+    | |_^ cannot infer type
+    |
+    |
+    = note: cannot satisfy `_: Baz`
+ note: required because of the requirements on the impl of `Grault` for `(T,)`
+    |
+    |
+ LL | impl<T: Grault> Grault for (T,)
+ 
+ error[E0283]: type annotations needed
22   --> $DIR/impl-wf-cycle-1.rs:20:5
23    |
23    |
24 LL |     type A = ();

-    |     ^^^^^^^^^^^^
+    |     ^^^^^^^^^^^^ cannot infer type
26    |
+    = note: cannot satisfy `_: Baz`
27 note: required because of the requirements on the impl of `Grault` for `(T,)`
29    |

32    = note: 1 redundant requirement hidden
32    = note: 1 redundant requirement hidden
33    = note: required because of the requirements on the impl of `Grault` for `(T,)`
34 
- error[E0275]: overflow evaluating the requirement `<(T,) as Grault>::A == _`
+ error[E0283]: type annotations needed
37    |
38 LL |     type B = bool;

-    |     ^^^^^^^^^^^^^^
-    |     ^^^^^^^^^^^^^^
+    |     ^^^^^^^^^^^^^^ cannot infer type
40    |
+    = note: cannot satisfy `_: Baz`
41 note: required because of the requirements on the impl of `Grault` for `(T,)`
43    |

46    = note: 1 redundant requirement hidden
46    = note: 1 redundant requirement hidden
47    = note: required because of the requirements on the impl of `Grault` for `(T,)`
- error: aborting due to 3 previous errors
+ error: aborting due to 4 previous errors
50 
- For more information about this error, try `rustc --explain E0275`.
---
To only update this specific test, also pass `--test-args associated-types/impl-wf-cycle-1.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/impl-wf-cycle-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/impl-wf-cycle-1" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/impl-wf-cycle-1/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/associated-types/impl-wf-cycle-1.rs:15:1
   |
   |
LL | / impl<T: Grault> Grault for (T,)
LL | | where
LL | |     Self::A: Baz,
LL | |     Self::B: Fiz,
...  |
LL | |     //~^ ERROR overflow evaluating the requirement `<(T,) as Grault>::A == _`
LL | | }
   | |_^ cannot infer type
   = note: cannot satisfy `_: Baz`
   = note: cannot satisfy `_: Baz`
note: required because of the requirements on the impl of `Grault` for `(T,)`
   |
   |
LL | impl<T: Grault> Grault for (T,)
   = note: 1 redundant requirement hidden
   = note: 1 redundant requirement hidden
   = note: required because of the requirements on the impl of `Grault` for `(T,)`
error[E0283]: type annotations needed
  --> /checkout/src/test/ui/associated-types/impl-wf-cycle-1.rs:15:1
   |
   |
LL | / impl<T: Grault> Grault for (T,)
LL | | where
LL | |     Self::A: Baz,
LL | |     Self::B: Fiz,
...  |
LL | |     //~^ ERROR overflow evaluating the requirement `<(T,) as Grault>::A == _`
LL | | }
   | |_^ cannot infer type
   = note: cannot satisfy `_: Baz`
   = note: cannot satisfy `_: Baz`
note: required because of the requirements on the impl of `Grault` for `(T,)`
   |
   |
LL | impl<T: Grault> Grault for (T,)

error[E0283]: type annotations needed
  --> /checkout/src/test/ui/associated-types/impl-wf-cycle-1.rs:20:5
   |
   |
LL |     type A = ();
   |     ^^^^^^^^^^^^ cannot infer type
   |
   = note: cannot satisfy `_: Baz`
note: required because of the requirements on the impl of `Grault` for `(T,)`
   |
   |
LL | impl<T: Grault> Grault for (T,)
   = note: 1 redundant requirement hidden
   = note: 1 redundant requirement hidden
   = note: required because of the requirements on the impl of `Grault` for `(T,)`
error[E0283]: type annotations needed
  --> /checkout/src/test/ui/associated-types/impl-wf-cycle-1.rs:22:5
   |
LL |     type B = bool;
LL |     type B = bool;
   |     ^^^^^^^^^^^^^^ cannot infer type
   |
   = note: cannot satisfy `_: Baz`
note: required because of the requirements on the impl of `Grault` for `(T,)`
   |
   |
LL | impl<T: Grault> Grault for (T,)
   = note: 1 redundant requirement hidden
   = note: 1 redundant requirement hidden
   = note: required because of the requirements on the impl of `Grault` for `(T,)`
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0283`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/associated-types/impl-wf-cycle-2.rs stdout ----
diff of stderr:

- error[E0275]: overflow evaluating the requirement `<(T,) as Grault>::A == _`
+ error[E0283]: type annotations needed
3    |
3    |
4 LL | / impl<T: Grault> Grault for (T,)
8 LL | |     type A = ();
9 LL | |
10 LL | | }
-    | |_^
-    | |_^
+    | |_^ cannot infer type
12    |
+    = note: cannot satisfy `_: Copy`
13 note: required because of the requirements on the impl of `Grault` for `(T,)`
15    |


16 LL | impl<T: Grault> Grault for (T,)
18 
18 
- error[E0275]: overflow evaluating the requirement `<(T,) as Grault>::A == _`
+ error[E0283]: type annotations needed
+    |
+    |
+ LL | / impl<T: Grault> Grault for (T,)
+ LL | | where
+ LL | |     Self::A: Copy,
+ LL | | {
+ LL | |     type A = ();
+ LL | |
+ LL | | }
+    | |_^ cannot infer type
+    = note: cannot satisfy `_: Copy`
+    = note: cannot satisfy `_: Copy`
+ note: required because of the requirements on the impl of `Grault` for `(T,)`
+    |
+    |
+ LL | impl<T: Grault> Grault for (T,)
+ 
+ error[E0283]: type annotations needed
20   --> $DIR/impl-wf-cycle-2.rs:11:5
21    |
21    |
22 LL |     type A = ();

-    |     ^^^^^^^^^^^^
+    |     ^^^^^^^^^^^^ cannot infer type
24    |
+    = note: cannot satisfy `_: Copy`
25 note: required because of the requirements on the impl of `Grault` for `(T,)`
27    |


28 LL | impl<T: Grault> Grault for (T,)
30 
- error: aborting due to 2 previous errors
+ error: aborting due to 3 previous errors
32 
---
To only update this specific test, also pass `--test-args associated-types/impl-wf-cycle-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/impl-wf-cycle-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/impl-wf-cycle-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/impl-wf-cycle-2/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/associated-types/impl-wf-cycle-2.rs:7:1
   |
   |
LL | / impl<T: Grault> Grault for (T,)
LL | | where
LL | |     Self::A: Copy,
LL | | {
LL | |     type A = ();
LL | |     //~^ ERROR overflow evaluating the requirement `<(T,) as Grault>::A == _`
LL | | }
   | |_^ cannot infer type
   = note: cannot satisfy `_: Copy`
   = note: cannot satisfy `_: Copy`
note: required because of the requirements on the impl of `Grault` for `(T,)`
   |
   |
LL | impl<T: Grault> Grault for (T,)

error[E0283]: type annotations needed
  --> /checkout/src/test/ui/associated-types/impl-wf-cycle-2.rs:7:1
   |
   |
LL | / impl<T: Grault> Grault for (T,)
LL | | where
LL | |     Self::A: Copy,
LL | | {
LL | |     type A = ();
LL | |     //~^ ERROR overflow evaluating the requirement `<(T,) as Grault>::A == _`
LL | | }
   | |_^ cannot infer type
   = note: cannot satisfy `_: Copy`
   = note: cannot satisfy `_: Copy`
note: required because of the requirements on the impl of `Grault` for `(T,)`
   |
   |
LL | impl<T: Grault> Grault for (T,)

error[E0283]: type annotations needed
  --> /checkout/src/test/ui/associated-types/impl-wf-cycle-2.rs:11:5
   |
   |
LL |     type A = ();
   |     ^^^^^^^^^^^^ cannot infer type
   |
   = note: cannot satisfy `_: Copy`
note: required because of the requirements on the impl of `Grault` for `(T,)`
   |
   |
LL | impl<T: Grault> Grault for (T,)

error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0283`.
For more information about this error, try `rustc --explain E0283`.
------------------------------------------


---- [ui] src/test/ui/generic-associated-types/bugs/issue-80626.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/bugs/issue-80626.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/bugs/issue-80626" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/bugs/issue-80626/auxiliary"
stdout: none
stderr: none

---- [ui] src/test/ui/generic-associated-types/bugs/issue-87755.rs stdout ----
diff of stderr:


- error[E0275]: overflow evaluating the requirement `<Bar as Foo>::Ass == _`
+ error[E0283]: type annotations needed
3    |
4 LL |     type Ass = Bar;

-    |                ^^^
---
To only update this specific test, also pass `--test-args generic-associated-types/bugs/issue-87755.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/bugs/issue-87755.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/bugs/issue-87755" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/bugs/issue-87755/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/generic-associated-types/bugs/issue-87755.rs:18:16
   |
LL |     type Ass = Bar;
   |                ^^^ cannot infer type
---
---- [ui] src/test/ui/generic-associated-types/issue-87750.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/issue-87750.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-87750" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-87750/auxiliary"
stdout: none
stderr: none

---- [ui] src/test/ui/generic-associated-types/projection-bound-cycle-generic.rs stdout ----
diff of stderr:


- error[E0275]: overflow evaluating the requirement `<T as Foo>::Item: Sized`
-   --> $DIR/projection-bound-cycle-generic.rs:44:18
+ error[E0282]: type annotations needed
3    |
3    |
- LL |     type Assoc = OnlySized<<T as Foo>::Item>;
-    |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^
+ LL |     type Item = [T] where [T]: Sized;
+ 
+ error[E0276]: impl has stricter requirements than trait
+   --> $DIR/projection-bound-cycle-generic.rs:25:5
6    |
6    |
- note: required by a bound in `OnlySized`
-   --> $DIR/projection-bound-cycle-generic.rs:28:18
-    |
- LL | struct OnlySized<T> where T: Sized { f: T }
-    |                  ^ required by this bound in `OnlySized`
+ LL |     type Item: Sized where <Self as Foo>::Item: Sized;
+    |     -------------------------------------------------- definition of `Item` from trait
+ ...
+ LL |     type Item = [T] where [T]: Sized;
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ impl has extra requirement `[T]: Sized`
- error: aborting due to previous error
+ error: aborting due to 2 previous errors
14 
- For more information about this error, try `rustc --explain E0275`.
---
To only update this specific test, also pass `--test-args generic-associated-types/projection-bound-cycle-generic.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/projection-bound-cycle-generic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/projection-bound-cycle-generic" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/projection-bound-cycle-generic/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/generic-associated-types/projection-bound-cycle-generic.rs:25:5
   |
   |
LL |     type Item = [T] where [T]: Sized;

error[E0276]: impl has stricter requirements than trait
  --> /checkout/src/test/ui/generic-associated-types/projection-bound-cycle-generic.rs:25:5
   |
   |
LL |     type Item: Sized where <Self as Foo>::Item: Sized;
   |     -------------------------------------------------- definition of `Item` from trait
...
LL |     type Item = [T] where [T]: Sized;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ impl has extra requirement `[T]: Sized`
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0276, E0282.
For more information about an error, try `rustc --explain E0276`.
For more information about an error, try `rustc --explain E0276`.
------------------------------------------


---- [ui] src/test/ui/generic-associated-types/projection-bound-cycle.rs stdout ----
diff of stderr:

- error[E0275]: overflow evaluating the requirement `<T as Foo>::Item: Sized`
-   --> $DIR/projection-bound-cycle.rs:46:18
+ error[E0282]: type annotations needed
3    |
3    |
- LL |     type Assoc = OnlySized<<T as Foo>::Item>;
-    |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^
+ LL |     type Item = str where str: Sized;
+ 
+ error[E0276]: impl has stricter requirements than trait
+   --> $DIR/projection-bound-cycle.rs:27:5
6    |
6    |
- note: required by a bound in `OnlySized`
-   --> $DIR/projection-bound-cycle.rs:30:18
-    |
- LL | struct OnlySized<T> where T: Sized { f: T }
-    |                  ^ required by this bound in `OnlySized`
+ LL |     type Item: Sized where <Self as Foo>::Item: Sized;
+    |     -------------------------------------------------- definition of `Item` from trait
+ ...
+ LL |     type Item = str where str: Sized;
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ impl has extra requirement `str: Sized`
- error: aborting due to previous error
+ error: aborting due to 2 previous errors
14 
- For more information about this error, try `rustc --explain E0275`.
---
To only update this specific test, also pass `--test-args generic-associated-types/projection-bound-cycle.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/projection-bound-cycle.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/projection-bound-cycle" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/projection-bound-cycle/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/generic-associated-types/projection-bound-cycle.rs:27:5
   |
   |
LL |     type Item = str where str: Sized;

error[E0276]: impl has stricter requirements than trait
  --> /checkout/src/test/ui/generic-associated-types/projection-bound-cycle.rs:27:5
   |
   |
LL |     type Item: Sized where <Self as Foo>::Item: Sized;
   |     -------------------------------------------------- definition of `Item` from trait
...
LL |     type Item = str where str: Sized;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ impl has extra requirement `str: Sized`
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0276, E0282.
For more information about an error, try `rustc --explain E0276`.
For more information about an error, try `rustc --explain E0276`.
------------------------------------------


---- [ui] src/test/ui/issues/issue-21946.rs stdout ----
diff of stderr:

- error[E0275]: overflow evaluating the requirement `<FooStruct as Foo>::A == _`
+ error[E0282]: type annotations needed
3    |
3    |
4 LL |     type A = <FooStruct as Foo>::A;
-    |              ^^^^^^^^^^^^^^^^^^^^^
+    |              ^^^^^^^^^^^^^^^^^^^^^ cannot infer type
6 
7 error: aborting due to previous error
---
To only update this specific test, also pass `--test-args issues/issue-21946.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-21946.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-21946" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-21946/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/issues/issue-21946.rs:8:14
   |
   |
LL |     type A = <FooStruct as Foo>::A;

error: aborting due to previous error

For more information about this error, try `rustc --explain E0282`.
For more information about this error, try `rustc --explain E0282`.
------------------------------------------


---- [ui] src/test/ui/issues/issue-23122-1.rs stdout ----
diff of stderr:

- error[E0275]: overflow evaluating the requirement `<GetNext<T> as Next>::Next == _`
+ error[E0282]: type annotations needed
3    |
3    |
4 LL |     type Next = <GetNext<T> as Next>::Next;
-    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^
+    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot infer type
6 
7 error: aborting due to previous error
---
To only update this specific test, also pass `--test-args issues/issue-23122-1.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-23122-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23122-1" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23122-1/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/issues/issue-23122-1.rs:10:17
   |
   |
LL |     type Next = <GetNext<T> as Next>::Next;

error: aborting due to previous error

For more information about this error, try `rustc --explain E0282`.
For more information about this error, try `rustc --explain E0282`.
------------------------------------------


---- [ui] src/test/ui/issues/issue-54189.rs stdout ----
diff of stderr:

4 LL | fn bug() -> impl for <'r> Fn() -> &'r () { || { &() } }
6 
- error: aborting due to previous error
+ error[E0308]: mismatched types
+   --> $DIR/issue-54189.rs:1:42
+   --> $DIR/issue-54189.rs:1:42
+    |
+ LL | fn bug() -> impl for <'r> Fn() -> &'r () { || { &() } }
+    |                                          ^^^^^^^^^^^^^^ one type is more general than the other
+    = note: expected reference `&()`
+    = note: expected reference `&()`
+               found reference `&'r ()`
- For more information about this error, try `rustc --explain E0582`.
+ error: aborting due to 2 previous errors
+ 
+ Some errors have detailed explanations: E0308, E0582.
---
To only update this specific test, also pass `--test-args issues/issue-54189.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-54189.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-54189" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-54189/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0582]: binding for associated type `Output` references lifetime `'r`, which does not appear in the trait input types
   |
   |
LL | fn bug() -> impl for <'r> Fn() -> &'r () { || { &() } }

error[E0308]: mismatched types
  --> /checkout/src/test/ui/issues/issue-54189.rs:1:42
   |
   |
LL | fn bug() -> impl for <'r> Fn() -> &'r () { || { &() } }
   |                                          ^^^^^^^^^^^^^^ one type is more general than the other
   = note: expected reference `&()`
   = note: expected reference `&()`
              found reference `&'r ()`
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0308, E0582.
For more information about an error, try `rustc --explain E0308`.
For more information about an error, try `rustc --explain E0308`.
------------------------------------------


---- [ui] src/test/ui/issues/issue-54696.rs stdout ----
normalized stderr:
warning: FIXME(skippy) PartialEq on function pointers has been deprecated.
   = note: `#[warn(deprecated)]` on by default

warning: 1 warning emitted

---
To only update this specific test, also pass `--test-args issues/issue-54696.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-54696.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-54696/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-54696/auxiliary"
stdout: none
--- stderr -------------------------------
warning: FIXME(skippy) PartialEq on function pointers has been deprecated.
   = note: `#[warn(deprecated)]` on by default

warning: 1 warning emitted
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/mir/issue-95978-validator-lifetime-comparison.rs stdout ----
normalized stderr:
warning: FIXME(skippy) PartialEq on function pointers has been deprecated.
   |
   |
LL |     let _ = x == foo;
   |
   = note: `#[warn(deprecated)]` on by default

warning: 1 warning emitted
---
To only update this specific test, also pass `--test-args mir/issue-95978-validator-lifetime-comparison.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/mir/issue-95978-validator-lifetime-comparison.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/issue-95978-validator-lifetime-comparison" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zvalidate-mir" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/issue-95978-validator-lifetime-comparison/auxiliary"
stdout: none
--- stderr -------------------------------
warning: FIXME(skippy) PartialEq on function pointers has been deprecated.
   |
   |
LL |     let _ = x == foo;
   |
   = note: `#[warn(deprecated)]` on by default

warning: 1 warning emitted
warning: 1 warning emitted
------------------------------------------


---- [ui] src/test/ui/regions/regions-close-object-into-object-5.rs stdout ----
diff of stderr:

20    |          - help: consider adding an explicit lifetime bound...: `T: 'static`
21 LL |     // oh dear!
22 LL |     Box::new(B(&*v)) as Box<dyn X>
+    |     ^^^^^^^^^^^^^^^^ ...so that the type `T` will meet its required lifetime bounds...
+    |
+ note: ...that is required by this bound
+    |
+    |
+ LL | struct B<'a, T: 'a>(&'a (A<T> + 'a));
+ 
+ error[E0310]: the parameter type `T` may not live long enough
+   --> $DIR/regions-close-object-into-object-5.rs:17:5
+    |
+    |
+ LL | fn f<'a, T, U>(v: Box<A<T> + 'static>) -> Box<X + 'static> {
+    |          - help: consider adding an explicit lifetime bound...: `T: 'static`
+ LL |     // oh dear!
+ LL |     Box::new(B(&*v)) as Box<dyn X>
23    |     ^^^^^^^^^^^^^^^^ ...so that the type `B<'_, T>` will meet its required lifetime bounds
25 error[E0310]: the parameter type `T` may not live long enough


79 LL |     Box::new(B(&*v)) as Box<dyn X>
80    |                ^^^ ...so that the type `(dyn A<T> + 'static)` is not borrowed for too long
- error: aborting due to 7 previous errors
+ error: aborting due to 8 previous errors
83 
84 For more information about this error, try `rustc --explain E0310`.
84 For more information about this error, try `rustc --explain E0310`.
85 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-close-object-into-object-5/regions-close-object-into-object-5.stderr
To only update this specific test, also pass `--test-args regions/regions-close-object-into-object-5.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-close-object-into-object-5.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-close-object-into-object-5" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-close-object-into-object-5/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0310]: the parameter type `T` may not live long enough
   |
   |
LL | fn f<'a, T, U>(v: Box<A<T> + 'static>) -> Box<X + 'static> {
   |          - help: consider adding an explicit lifetime bound...: `T: 'static`
LL |     // oh dear!
LL |     Box::new(B(&*v)) as Box<dyn X>
   |     ^^^^^^^^ ...so that the type `T` will meet its required lifetime bounds...
   |
note: ...that is required by this bound
   |
   |
LL | struct B<'a, T: 'a>(&'a (A<T> + 'a));

error[E0310]: the parameter type `T` may not live long enough
  --> /checkout/src/test/ui/regions/regions-close-object-into-object-5.rs:17:5
   |
   |
LL | fn f<'a, T, U>(v: Box<A<T> + 'static>) -> Box<X + 'static> {
   |          - help: consider adding an explicit lifetime bound...: `T: 'static`
LL |     // oh dear!
LL |     Box::new(B(&*v)) as Box<dyn X>
   |     ^^^^^^^^^^^^^^^^ ...so that the type `T` will meet its required lifetime bounds...
   |
note: ...that is required by this bound
   |
   |
LL | struct B<'a, T: 'a>(&'a (A<T> + 'a));

error[E0310]: the parameter type `T` may not live long enough
  --> /checkout/src/test/ui/regions/regions-close-object-into-object-5.rs:17:5
   |
   |
LL | fn f<'a, T, U>(v: Box<A<T> + 'static>) -> Box<X + 'static> {
   |          - help: consider adding an explicit lifetime bound...: `T: 'static`
LL |     // oh dear!
LL |     Box::new(B(&*v)) as Box<dyn X>
   |     ^^^^^^^^^^^^^^^^ ...so that the type `B<'_, T>` will meet its required lifetime bounds
error[E0310]: the parameter type `T` may not live long enough
  --> /checkout/src/test/ui/regions/regions-close-object-into-object-5.rs:17:14
   |
   |
LL | fn f<'a, T, U>(v: Box<A<T> + 'static>) -> Box<X + 'static> {
   |          - help: consider adding an explicit lifetime bound...: `T: 'static`
LL |     // oh dear!
LL |     Box::new(B(&*v)) as Box<dyn X>
   |              ^ ...so that the type `T` will meet its required lifetime bounds...
   |
note: ...that is required by this bound
   |
   |
LL | struct B<'a, T: 'a>(&'a (A<T> + 'a));

error[E0310]: the parameter type `T` may not live long enough
  --> /checkout/src/test/ui/regions/regions-close-object-into-object-5.rs:17:14
   |
   |
LL | fn f<'a, T, U>(v: Box<A<T> + 'static>) -> Box<X + 'static> {
   |          - help: consider adding an explicit lifetime bound...: `T: 'static`
LL |     // oh dear!
LL |     Box::new(B(&*v)) as Box<dyn X>
   |              ^^^^^^ ...so that the type `T` will meet its required lifetime bounds...
   |
note: ...that is required by this bound
   |
   |
LL | struct B<'a, T: 'a>(&'a (A<T> + 'a));

error[E0310]: the parameter type `T` may not live long enough
  --> /checkout/src/test/ui/regions/regions-close-object-into-object-5.rs:17:16
   |
   |
LL | fn f<'a, T, U>(v: Box<A<T> + 'static>) -> Box<X + 'static> {
   |          - help: consider adding an explicit lifetime bound...: `T: 'static`
LL |     // oh dear!
LL |     Box::new(B(&*v)) as Box<dyn X>
   |                ^^^ ...so that the reference type `&dyn A<T>` does not outlive the data it points at
error[E0310]: the parameter type `T` may not live long enough
  --> /checkout/src/test/ui/regions/regions-close-object-into-object-5.rs:17:16
   |
   |
LL | fn f<'a, T, U>(v: Box<A<T> + 'static>) -> Box<X + 'static> {
   |          - help: consider adding an explicit lifetime bound...: `T: 'static`
LL |     // oh dear!
LL |     Box::new(B(&*v)) as Box<dyn X>
   |                ^^^ ...so that the type `(dyn A<T> + 'static)` is not borrowed for too long
error[E0310]: the parameter type `T` may not live long enough
  --> /checkout/src/test/ui/regions/regions-close-object-into-object-5.rs:17:16
   |
   |
LL | fn f<'a, T, U>(v: Box<A<T> + 'static>) -> Box<X + 'static> {
   |          - help: consider adding an explicit lifetime bound...: `T: 'static`
LL |     // oh dear!
LL |     Box::new(B(&*v)) as Box<dyn X>
   |                ^^^ ...so that the type `(dyn A<T> + 'static)` is not borrowed for too long
error: aborting due to 8 previous errors

For more information about this error, try `rustc --explain E0310`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/specialization/issue-39448.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/specialization/issue-39448.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/issue-39448" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/issue-39448/auxiliary"
stdout: none
--- stderr -------------------------------
warning: the feature `specialization` is incomplete and may not be safe to use and/or cause compiler crashes
   |
   |
LL | #![feature(specialization)] //~ WARN the feature `specialization` is incomplete
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #31844 <https://github.com/rust-lang/rust/issues/31844> for more information
   = help: consider using `min_specialization` instead, which is more stable and complete
---
---- [ui] src/test/ui/specialization/issue-38091-2.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/specialization/issue-38091-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/issue-38091-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/issue-38091-2/auxiliary"
stdout: none
--- stderr -------------------------------
warning: the feature `specialization` is incomplete and may not be safe to use and/or cause compiler crashes
   |
   |
LL | #![feature(specialization)]
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #31844 <https://github.com/rust-lang/rust/issues/31844> for more information
   = help: consider using `min_specialization` instead, which is more stable and complete
---
---- [ui] src/test/ui/traits/cycle-cache-err-60010.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/cycle-cache-err-60010.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/cycle-cache-err-60010" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/cycle-cache-err-60010/auxiliary"
stdout: none
stderr: none

---- [ui] src/test/ui/traits/inductive-overflow/lifetime.rs stdout ----

error: ui test compiled successfully!
error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/inductive-overflow/lifetime.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/inductive-overflow/lifetime" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/inductive-overflow/lifetime/auxiliary"
stdout: none
stderr: none

---- [ui] src/test/ui/traits/inductive-overflow/simultaneous.rs stdout ----

error: ui test compiled successfully!
error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/inductive-overflow/simultaneous.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/inductive-overflow/simultaneous" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/inductive-overflow/simultaneous/auxiliary"
stdout: none
stderr: none

---- [ui] src/test/ui/traits/inductive-overflow/supertrait.rs stdout ----

error: ui test compiled successfully!
error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/inductive-overflow/supertrait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/inductive-overflow/supertrait" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/inductive-overflow/supertrait/auxiliary"
stdout: none
stderr: none

---- [ui] src/test/ui/traits/inductive-overflow/two-traits.rs stdout ----
diff of stderr:


14 LL | impl<T: Magic + std::marker::Sync> Magic for T {
16 
16 
- error[E0275]: overflow evaluating the requirement `*mut (): Magic`
-   --> $DIR/two-traits.rs:20:5
-    |
- LL |     wizard::<*mut ()>();
-    |
-    |
- note: required by a bound in `wizard`
-   --> $DIR/two-traits.rs:17:14
-    |
- LL | fn wizard<T: Magic>() { check::<<T as Magic>::X>(); }
-    |              ^^^^^ required by this bound in `wizard`
+ error: aborting due to previous error
- error: aborting due to 2 previous errors
- 
- Some errors have detailed explanations: E0275, E0277.
- For more information about an error, try `rustc --explain E0275`.
---
To only update this specific test, also pass `--test-args traits/inductive-overflow/two-traits.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/inductive-overflow/two-traits.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/inductive-overflow/two-traits" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/inductive-overflow/two-traits/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: `T` cannot be shared between threads safely
   |
LL |     type X = Self;
LL |     type X = Self;
   |              ^^^^ `T` cannot be shared between threads safely
   |
note: required by a bound in `Magic::X`
   |
   |
LL |     type X: Trait;
   |             ^^^^^ required by this bound in `Magic::X`
   |
   |
LL | impl<T: Magic + std::marker::Sync> Magic for T {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
