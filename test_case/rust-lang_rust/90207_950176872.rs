plain
.................................................................................................... 5800/12328
.................................................................................................... 5900/12328
...................................................................................i................ 6000/12328
.................................................................................................... 6100/12328
.................................................................................i.....F.F.......... 6200/12328
..........................................F......................................................... 6400/12328
.............ii.ii........i...i..................................................................... 6500/12328
..............F.......................................................i....i........................ 6600/12328
.................i................i.............i................................................i.. 6700/12328
---
............................................ii...................................................... 8100/12328
.................................................................................................... 8200/12328
................................................i.....................................i............. 8300/12328
...............................................i.................................................... 8400/12328
....................................F.F...FFF..........................................i............ 8500/12328
.........................................................................i.......................... 8700/12328
.................................................................................................... 8800/12328
.................................................................................................... 8900/12328
.................................................................................................... 9000/12328
---
---- [ui] ui/feature-gates/feature-gate-const_generics_defaults.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-const_generics_defaults.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-const_generics_defaults" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-const_generics_defaults/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
1 error: lifetime parameters must be declared prior to type parameters
-   --> $DIR/issue-59508-1.rs:11:25
+   --> $DIR/issue-59508-1.rs:10:25
3    |
4 LL |     pub fn do_things<T, 'a, 'b: 'a>() {
5    |                     ----^^--^^----- help: reorder the parameters: lifetimes, then consts and types: `<'a, 'b: 'a, T>`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-59508-1/issue-59508-1.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-59508-1.rs`

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-59508-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-59508-1" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-59508-1/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: lifetime parameters must be declared prior to type parameters
  --> /checkout/src/test/ui/issues/issue-59508-1.rs:10:25
   |
LL |     pub fn do_things<T, 'a, 'b: 'a>() {
   |                     ----^^--^^----- help: reorder the parameters: lifetimes, then consts and types: `<'a, 'b: 'a, T>`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/issues/issue-59508.rs stdout ----
diff of stderr:

2   --> $DIR/issue-59508.rs:10:25
3    |
4 LL |     pub fn do_things<T, 'a, 'b: 'a>() {
-    |                     ----^^--^^----- help: reorder the parameters: lifetimes, then types, then consts: `<'a, 'b: 'a, T>`
+    |                     ----^^--^^----- help: reorder the parameters: lifetimes, then consts and types: `<'a, 'b: 'a, T>`
7 error: aborting due to previous error
8 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-59508/issue-59508.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-59508.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-59508.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-59508" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-59508/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: lifetime parameters must be declared prior to type parameters
  --> /checkout/src/test/ui/issues/issue-59508.rs:10:25
   |
LL |     pub fn do_things<T, 'a, 'b: 'a>() {
   |                     ----^^--^^----- help: reorder the parameters: lifetimes, then consts and types: `<'a, 'b: 'a, T>`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/issues/issue-80512-param-reordering-with-defaults.rs stdout ----
diff of stderr:

2   --> $DIR/issue-80512-param-reordering-with-defaults.rs:3:18
3    |
4 LL | struct S<T = (), 'a>(&'a T);
-    |         ---------^^- help: reorder the parameters: lifetimes, then types, then consts: `<'a, T = ()>`
+    |         ---------^^- help: reorder the parameters: lifetimes, then consts and types: `<'a, T = ()>`
7 error: aborting due to previous error
8 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-80512-param-reordering-with-defaults/issue-80512-param-reordering-with-defaults.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-80512-param-reordering-with-defaults.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-80512-param-reordering-with-defaults.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-80512-param-reordering-with-defaults" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-80512-param-reordering-with-defaults/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: lifetime parameters must be declared prior to type parameters
  --> /checkout/src/test/ui/issues/issue-80512-param-reordering-with-defaults.rs:3:18
   |
LL | struct S<T = (), 'a>(&'a T);
   |         ---------^^- help: reorder the parameters: lifetimes, then consts and types: `<'a, T = ()>`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/lifetime-before-type-params.rs stdout ----
diff of stderr:

2   --> $DIR/lifetime-before-type-params.rs:2:13
3    |
4 LL | fn first<T, 'a, 'b>() {}
-    |         ----^^--^^- help: reorder the parameters: lifetimes, then types, then consts: `<'a, 'b, T>`
+    |         ----^^--^^- help: reorder the parameters: lifetimes, then consts and types: `<'a, 'b, T>`
7 error: lifetime parameters must be declared prior to type parameters
8   --> $DIR/lifetime-before-type-params.rs:4:18

9    |
9    |
10 LL | fn second<'a, T, 'b>() {}
-    |          --------^^- help: reorder the parameters: lifetimes, then types, then consts: `<'a, 'b, T>`
+    |          --------^^- help: reorder the parameters: lifetimes, then consts and types: `<'a, 'b, T>`
13 error: lifetime parameters must be declared prior to type parameters
14   --> $DIR/lifetime-before-type-params.rs:6:16

15    |
15    |
16 LL | fn third<T, U, 'a>() {}
-    |         -------^^- help: reorder the parameters: lifetimes, then types, then consts: `<'a, T, U>`
+    |         -------^^- help: reorder the parameters: lifetimes, then consts and types: `<'a, T, U>`
19 error: lifetime parameters must be declared prior to type parameters
20   --> $DIR/lifetime-before-type-params.rs:8:18

21    |
21    |
22 LL | fn fourth<'a, T, 'b, U, 'c, V>() {}
-    |          --------^^-----^^---- help: reorder the parameters: lifetimes, then types, then consts: `<'a, 'b, 'c, T, U, V>`
+    |          --------^^-----^^---- help: reorder the parameters: lifetimes, then consts and types: `<'a, 'b, 'c, T, U, V>`
25 error: aborting due to 4 previous errors
26 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetime-before-type-params/lifetime-before-type-params.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lifetime-before-type-params.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lifetime-before-type-params.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetime-before-type-params" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetime-before-type-params/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: lifetime parameters must be declared prior to type parameters
  --> /checkout/src/test/ui/lifetime-before-type-params.rs:2:13
   |
LL | fn first<T, 'a, 'b>() {}
   |         ----^^--^^- help: reorder the parameters: lifetimes, then consts and types: `<'a, 'b, T>`
error: lifetime parameters must be declared prior to type parameters
  --> /checkout/src/test/ui/lifetime-before-type-params.rs:4:18
   |
   |
LL | fn second<'a, T, 'b>() {}
   |          --------^^- help: reorder the parameters: lifetimes, then consts and types: `<'a, 'b, T>`
error: lifetime parameters must be declared prior to type parameters
  --> /checkout/src/test/ui/lifetime-before-type-params.rs:6:16
   |
   |
LL | fn third<T, U, 'a>() {}
   |         -------^^- help: reorder the parameters: lifetimes, then consts and types: `<'a, T, U>`
error: lifetime parameters must be declared prior to type parameters
  --> /checkout/src/test/ui/lifetime-before-type-params.rs:8:18
   |
   |
LL | fn fourth<'a, T, 'b, U, 'c, V>() {}
   |          --------^^-----^^---- help: reorder the parameters: lifetimes, then consts and types: `<'a, 'b, 'c, T, U, V>`
error: aborting due to 4 previous errors


------------------------------------------
---
1 error[E0412]: cannot find type `N` in this scope
-   --> $DIR/missing-type-parameter2.rs:6:8
+   --> $DIR/missing-type-parameter2.rs:3:8
3    |
4 LL | struct X<const N: u8>();
5    | ------------------------ similarly named struct `X` defined here
17    |     +++
18 
19 error[E0412]: cannot find type `N` in this scope
-   --> $DIR/missing-type-parameter2.rs:9:28
-   --> $DIR/missing-type-parameter2.rs:9:28
+   --> $DIR/missing-type-parameter2.rs:6:28
21    |
22 LL | impl<T, const A: u8 = 2> X<N> {}
23    |      -                     ^
34    |                        +++
35 
36 error[E0412]: cannot find type `T` in this scope
-   --> $DIR/missing-type-parameter2.rs:14:20
-   --> $DIR/missing-type-parameter2.rs:14:20
+   --> $DIR/missing-type-parameter2.rs:11:20
38    |
39 LL | struct X<const N: u8>();
40    | ------------------------ similarly named struct `X` defined here
52    |       +++
53 
54 error[E0412]: cannot find type `T` in this scope
-   --> $DIR/missing-type-parameter2.rs:14:11
-   --> $DIR/missing-type-parameter2.rs:14:11
+   --> $DIR/missing-type-parameter2.rs:11:11
56    |
57 LL | struct X<const N: u8>();
58    | ------------------------ similarly named struct `X` defined here
70    |       +++
71 
72 error[E0412]: cannot find type `A` in this scope
-   --> $DIR/missing-type-parameter2.rs:18:24
-   --> $DIR/missing-type-parameter2.rs:18:24
+   --> $DIR/missing-type-parameter2.rs:15:24
74    |
75 LL | struct X<const N: u8>();
76    | ------------------------ similarly named struct `X` defined here
88    |                   +++
89 
90 error[E0747]: unresolved item provided when a constant was expected
-   --> $DIR/missing-type-parameter2.rs:6:8
-   --> $DIR/missing-type-parameter2.rs:6:8
+   --> $DIR/missing-type-parameter2.rs:3:8
92    |
93 LL | impl X<N> {}

99    |        +   +
100 
100 
101 error: defaults for const parameters are only allowed in `struct`, `enum`, `type`, or `trait` definitions
-   --> $DIR/missing-type-parameter2.rs:9:15
103    |
103    |
104 LL | impl<T, const A: u8 = 2> X<N> {}

106 
107 error[E0747]: unresolved item provided when a constant was expected
-   --> $DIR/missing-type-parameter2.rs:9:28
-   --> $DIR/missing-type-parameter2.rs:9:28
+   --> $DIR/missing-type-parameter2.rs:6:28
109    |
110 LL | impl<T, const A: u8 = 2> X<N> {}


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/missing/missing-items/missing-type-parameter2/missing-type-parameter2.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/missing/missing-items/missing-type-parameter2/missing-type-parameter2.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args missing/missing-items/missing-type-parameter2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/missing/missing-items/missing-type-parameter2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/missing/missing-items/missing-type-parameter2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/missing/missing-items/missing-type-parameter2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0412]: cannot find type `N` in this scope
  --> /checkout/src/test/ui/missing/missing-items/missing-type-parameter2.rs:3:8
   |
LL | struct X<const N: u8>();
   | ------------------------ similarly named struct `X` defined here
LL | 
LL | impl X<N> {}
   |
help: a struct with a similar name exists
   |
   |
LL | impl X<X> {}
help: you might be missing a type parameter
   |
   |
LL | impl<N> X<N> {}

error[E0412]: cannot find type `N` in this scope
  --> /checkout/src/test/ui/missing/missing-items/missing-type-parameter2.rs:6:28
   |
   |
LL | impl<T, const A: u8 = 2> X<N> {}
   |      -                     ^
   |      similarly named type parameter `T` defined here
   |
help: a type parameter with a similar name exists
   |
   |
LL | impl<T, const A: u8 = 2> X<T> {}
help: you might be missing a type parameter
   |
   |
LL | impl<T, const A: u8 = 2, N> X<N> {}

error[E0412]: cannot find type `T` in this scope
  --> /checkout/src/test/ui/missing/missing-items/missing-type-parameter2.rs:11:20
   |
   |
LL | struct X<const N: u8>();
   | ------------------------ similarly named struct `X` defined here
...
LL | fn foo(_: T) where T: Send {}
   |
help: a struct with a similar name exists
   |
   |
LL | fn foo(_: T) where X: Send {}
help: you might be missing a type parameter
   |
   |
LL | fn foo<T>(_: T) where T: Send {}

error[E0412]: cannot find type `T` in this scope
  --> /checkout/src/test/ui/missing/missing-items/missing-type-parameter2.rs:11:11
   |
   |
LL | struct X<const N: u8>();
   | ------------------------ similarly named struct `X` defined here
...
LL | fn foo(_: T) where T: Send {}
   |
help: a struct with a similar name exists
   |
   |
LL | fn foo(_: X) where T: Send {}
help: you might be missing a type parameter
   |
   |
LL | fn foo<T>(_: T) where T: Send {}

error[E0412]: cannot find type `A` in this scope
  --> /checkout/src/test/ui/missing/missing-items/missing-type-parameter2.rs:15:24
   |
   |
LL | struct X<const N: u8>();
   | ------------------------ similarly named struct `X` defined here
...
LL | fn bar<const N: u8>(_: A) {}
   |
help: a struct with a similar name exists
   |
   |
LL | fn bar<const N: u8>(_: X) {}
help: you might be missing a type parameter
   |
   |
LL | fn bar<const N: u8, A>(_: A) {}

error[E0747]: unresolved item provided when a constant was expected
  --> /checkout/src/test/ui/missing/missing-items/missing-type-parameter2.rs:3:8
   |
   |
LL | impl X<N> {}
   |
   |
help: if this generic argument was intended as a const parameter, surround it with braces
   |
LL | impl X<{ N }> {}
   |        +   +

error: defaults for const parameters are only allowed in `struct`, `enum`, `type`, or `trait` definitions
   |
   |
LL | impl<T, const A: u8 = 2> X<N> {}

error[E0747]: unresolved item provided when a constant was expected
  --> /checkout/src/test/ui/missing/missing-items/missing-type-parameter2.rs:6:28
   |
   |
LL | impl<T, const A: u8 = 2> X<N> {}
   |
   |
help: if this generic argument was intended as a const parameter, surround it with braces
   |
LL | impl<T, const A: u8 = 2> X<{ N }> {}
   |                            +   +
error: aborting due to 8 previous errors

Some errors have detailed explanations: E0412, E0747.
For more information about an error, try `rustc --explain E0412`.
---
diff of stderr:

2   --> $DIR/issue-14303-enum.rs:1:15
3    |
4 LL | enum X<'a, T, 'b> {
-    |       --------^^- help: reorder the parameters: lifetimes, then types, then consts: `<'a, 'b, T>`
+    |       --------^^- help: reorder the parameters: lifetimes, then consts and types: `<'a, 'b, T>`
7 error: aborting due to previous error
8 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-14303-enum/issue-14303-enum.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args parser/issue-14303-enum.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/issue-14303-enum.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-14303-enum" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-14303-enum/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: lifetime parameters must be declared prior to type parameters
  --> /checkout/src/test/ui/parser/issue-14303-enum.rs:1:15
   |
LL | enum X<'a, T, 'b> {
   |       --------^^- help: reorder the parameters: lifetimes, then consts and types: `<'a, 'b, T>`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/parser/issue-14303-fn-def.rs stdout ----
diff of stderr:

2   --> $DIR/issue-14303-fn-def.rs:1:15
3    |
4 LL | fn foo<'a, T, 'b>(x: &'a T) {}
-    |       --------^^- help: reorder the parameters: lifetimes, then types, then consts: `<'a, 'b, T>`
+    |       --------^^- help: reorder the parameters: lifetimes, then consts and types: `<'a, 'b, T>`
7 error: aborting due to previous error
8 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-14303-fn-def/issue-14303-fn-def.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args parser/issue-14303-fn-def.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/issue-14303-fn-def.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-14303-fn-def" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-14303-fn-def/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: lifetime parameters must be declared prior to type parameters
  --> /checkout/src/test/ui/parser/issue-14303-fn-def.rs:1:15
   |
LL | fn foo<'a, T, 'b>(x: &'a T) {}
   |       --------^^- help: reorder the parameters: lifetimes, then consts and types: `<'a, 'b, T>`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/parser/issue-14303-impl.rs stdout ----
diff of stderr:

2   --> $DIR/issue-14303-impl.rs:3:13
3    |
4 LL | impl<'a, T, 'b> X<T> {}
-    |     --------^^- help: reorder the parameters: lifetimes, then types, then consts: `<'a, 'b, T>`
+    |     --------^^- help: reorder the parameters: lifetimes, then consts and types: `<'a, 'b, T>`
7 error: aborting due to previous error
8 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-14303-impl/issue-14303-impl.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args parser/issue-14303-impl.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/issue-14303-impl.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-14303-impl" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-14303-impl/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: lifetime parameters must be declared prior to type parameters
  --> /checkout/src/test/ui/parser/issue-14303-impl.rs:3:13
   |
LL | impl<'a, T, 'b> X<T> {}
   |     --------^^- help: reorder the parameters: lifetimes, then consts and types: `<'a, 'b, T>`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/parser/issue-14303-struct.rs stdout ----
diff of stderr:

2   --> $DIR/issue-14303-struct.rs:1:17
3    |
4 LL | struct X<'a, T, 'b> {
-    |         --------^^- help: reorder the parameters: lifetimes, then types, then consts: `<'a, 'b, T>`
+    |         --------^^- help: reorder the parameters: lifetimes, then consts and types: `<'a, 'b, T>`
7 error: aborting due to previous error
8 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-14303-struct/issue-14303-struct.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args parser/issue-14303-struct.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/issue-14303-struct.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-14303-struct" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-14303-struct/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: lifetime parameters must be declared prior to type parameters
  --> /checkout/src/test/ui/parser/issue-14303-struct.rs:1:17
   |
LL | struct X<'a, T, 'b> {
   |         --------^^- help: reorder the parameters: lifetimes, then consts and types: `<'a, 'b, T>`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/parser/issue-14303-trait.rs stdout ----
diff of stderr:

2   --> $DIR/issue-14303-trait.rs:1:18
3    |
4 LL | trait Foo<'a, T, 'b> {}
-    |          --------^^- help: reorder the parameters: lifetimes, then types, then consts: `<'a, 'b, T>`
+    |          --------^^- help: reorder the parameters: lifetimes, then consts and types: `<'a, 'b, T>`
7 error: aborting due to previous error
8 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-14303-trait/issue-14303-trait.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args parser/issue-14303-trait.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/issue-14303-trait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-14303-trait" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-14303-trait/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: lifetime parameters must be declared prior to type parameters
  --> /checkout/src/test/ui/parser/issue-14303-trait.rs:1:18
   |
LL | trait Foo<'a, T, 'b> {}
   |          --------^^- help: reorder the parameters: lifetimes, then consts and types: `<'a, 'b, T>`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/suggestions/suggest-move-lifetimes.rs stdout ----
diff of stderr:

2   --> $DIR/suggest-move-lifetimes.rs:1:13
3    |
4 LL | struct A<T, 'a> {
-    |         ----^^- help: reorder the parameters: lifetimes, then types, then consts: `<'a, T>`
+    |         ----^^- help: reorder the parameters: lifetimes, then consts and types: `<'a, T>`
7 error: lifetime parameters must be declared prior to type parameters
8   --> $DIR/suggest-move-lifetimes.rs:5:13

9    |
9    |
10 LL | struct B<T, 'a, U> {
-    |         ----^^---- help: reorder the parameters: lifetimes, then types, then consts: `<'a, T, U>`
+    |         ----^^---- help: reorder the parameters: lifetimes, then consts and types: `<'a, T, U>`
13 error: lifetime parameters must be declared prior to type parameters
14   --> $DIR/suggest-move-lifetimes.rs:10:16

15    |
15    |
16 LL | struct C<T, U, 'a> {
-    |         -------^^- help: reorder the parameters: lifetimes, then types, then consts: `<'a, T, U>`
+    |         -------^^- help: reorder the parameters: lifetimes, then consts and types: `<'a, T, U>`
19 error: lifetime parameters must be declared prior to type parameters
20   --> $DIR/suggest-move-lifetimes.rs:15:16

21    |
21    |
22 LL | struct D<T, U, 'a, 'b, V, 'c> {
-    |         -------^^--^^-----^^- help: reorder the parameters: lifetimes, then types, then consts: `<'a, 'b, 'c, T, U, V>`
+    |         -------^^--^^-----^^- help: reorder the parameters: lifetimes, then consts and types: `<'a, 'b, 'c, T, U, V>`
25 error: aborting due to 4 previous errors
26 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/suggest-move-lifetimes/suggest-move-lifetimes.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args suggestions/suggest-move-lifetimes.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/suggest-move-lifetimes.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/suggest-move-lifetimes" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/suggest-move-lifetimes/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: lifetime parameters must be declared prior to type parameters
  --> /checkout/src/test/ui/suggestions/suggest-move-lifetimes.rs:1:13
   |
LL | struct A<T, 'a> { //~ ERROR lifetime parameters must be declared
   |         ----^^- help: reorder the parameters: lifetimes, then consts and types: `<'a, T>`
error: lifetime parameters must be declared prior to type parameters
  --> /checkout/src/test/ui/suggestions/suggest-move-lifetimes.rs:5:13
   |
   |
LL | struct B<T, 'a, U> { //~ ERROR lifetime parameters must be declared
   |         ----^^---- help: reorder the parameters: lifetimes, then consts and types: `<'a, T, U>`
error: lifetime parameters must be declared prior to type parameters
  --> /checkout/src/test/ui/suggestions/suggest-move-lifetimes.rs:10:16
   |
   |
LL | struct C<T, U, 'a> { //~ ERROR lifetime parameters must be declared
   |         -------^^- help: reorder the parameters: lifetimes, then consts and types: `<'a, T, U>`
error: lifetime parameters must be declared prior to type parameters
  --> /checkout/src/test/ui/suggestions/suggest-move-lifetimes.rs:15:16
   |
   |
LL | struct D<T, U, 'a, 'b, V, 'c> { //~ ERROR lifetime parameters must be declared
   |         -------^^--^^-----^^- help: reorder the parameters: lifetimes, then consts and types: `<'a, 'b, 'c, T, U, V>`
error: aborting due to 4 previous errors


------------------------------------------
---
test result: FAILED. 12199 passed; 12 failed; 117 ignored; 0 measured; 0 filtered out; finished in 134.46s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:46
